import {randomBytes} from "node:crypto";
import {closeSync,linkSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
/** ⏳️ Cancellable pause. */
export const sleepV1=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((done,fail)=>{if(signal?.aborted){fail(signal.reason);return;}const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);fail(signal.reason);},{once:true});});

/** 🫀️ Whether a process exists; a permission refusal still means it exists. */
export function processAliveV1(pid:number):boolean{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}}

type Holder=Readonly<{pid:number;token:string;atMs:number}>;
function holderOf(path:string):Holder|undefined{try{const row=JSON.parse(readFileSync(path,"utf8"));return typeof row?.pid==="number"?row:undefined;}catch{return undefined;}}
function ageOf(path:string,now:number):number|undefined{try{return now-statSync(path).mtimeMs;}catch{return undefined;}}

/** 🪪️ One held claim: a file created exclusively and kept fresh by its holder, released by removal. */
export type ClaimV1=Readonly<{path:string;waitedMs:number;release():void}>;

function stale(path:string,staleMs:number,now:number):boolean{
 const age=ageOf(path,now);if(age===undefined)return false;
 if(age>staleMs)return true;
 const holder=holderOf(path);return holder!==undefined&&age>1000&&!processAliveV1(holder.pid);
}

function removeStale(path:string,staleMs:number):void{
 const grave=path+".dead-"+randomBytes(4).toString("hex");
 try{renameSync(path,grave);}catch{return;}
 if(!stale(grave,staleMs,Date.now())){try{linkSync(grave,path);}catch{}}
 rmSync(grave,{force:true});
}

function tryCreate(path:string,heartbeatMs:number):ClaimV1|undefined{
 const token=randomBytes(8).toString("hex");let fd:number;
 try{fd=openSync(path,"wx");}catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return undefined;throw error;}
 try{writeSync(fd,JSON.stringify({pid:process.pid,token,atMs:Date.now()}));}finally{closeSync(fd);}
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(path,now,now);}catch{}},heartbeatMs);beat.unref?.();
 let released=false;
 return Object.freeze({path,waitedMs:0,release(){if(released)return;released=true;clearInterval(beat);if(holderOf(path)?.token===token)rmSync(path,{force:true});}});
}

/** 🚪️ Takes the claim only if it is free right now, replacing a dead or silent holder; never waits. */
export function tryClaimV1(directory:string,name:string):ClaimV1|undefined{
 mkdirSync(directory,{recursive:true});
 const path=join(directory,name+".lock");
 for(let attempt=0;attempt<2;attempt++){
  const claim=tryCreate(path,policy.lock.heartbeatMilliseconds);if(claim)return claim;
  if(!stale(path,policy.lock.staleMilliseconds,Date.now()))return undefined;
  removeStale(path,policy.lock.staleMilliseconds);
 }
 return undefined;
}

/** 🔐️ Takes the per-unit claim named `name` inside `directory`. Waiting is bounded: `undefined` after the maximum wait means "compile anyway", so no wait chain can become a deadlock. A holder that died or stopped beating is replaced atomically. */
export async function acquireUnitClaimV1(directory:string,name:string,options:Readonly<{maximumWaitMs?:number;signal?:AbortSignal;onWait?:(waitedMs:number)=>void}>={}):Promise<ClaimV1|undefined>{
 mkdirSync(directory,{recursive:true});
 const path=join(directory,name+".lock"),started=Date.now(),limit=options.maximumWaitMs??policy.lock.maximumWaitMilliseconds;
 let poll=policy.lock.pollMinimumMilliseconds,reported=false;
 for(;;){
  const claim=tryCreate(path,policy.lock.heartbeatMilliseconds);
  if(claim)return Object.freeze({...claim,waitedMs:Date.now()-started});
  if(stale(path,policy.lock.staleMilliseconds,Date.now())){removeStale(path,policy.lock.staleMilliseconds);continue;}
  const waited=Date.now()-started;
  if(waited>=limit)return undefined;
  if(!reported){reported=true;options.onWait?.(0);}
  await sleepV1(Math.min(poll,Math.max(1,limit-waited)),options.signal);
  poll=Math.min(policy.lock.pollMaximumMilliseconds,Math.ceil(poll*1.5));
 }
}

/** 🎰️ Host-wide compile slots. Waiters take a numbered ticket and only the oldest live ticket may claim a free slot, so grants are first-in-first-out and no waiter starves; tickets and slots of dead processes are reaped. */
export async function acquireSlotV1(root:string,count:number,options:Readonly<{signal?:AbortSignal;onWait?:(waitedMs:number)=>void}>={}):Promise<ClaimV1>{
 if(!Number.isSafeInteger(count)||count<1)throw Error("Positive compile slot count required");
 const slots=join(root,policy.layout.slots),tickets=join(root,policy.layout.tickets);mkdirSync(slots,{recursive:true});mkdirSync(tickets,{recursive:true});
 const started=Date.now(),name=String(started).padStart(15,"0")+"-"+randomBytes(4).toString("hex")+"-"+process.pid,ticket=join(tickets,name);
 const writeTicket=()=>{const fd=openSync(ticket,"w");try{writeSync(fd,JSON.stringify({pid:process.pid,token:name,atMs:started}));}finally{closeSync(fd);}};
 writeTicket();
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(ticket,now,now);}catch{}},policy.slot.heartbeatMilliseconds);beat.unref?.();
 let reported=false;
 try{
  for(;;){
   const now=Date.now(),live:string[]=[];
   for(const entry of readdirSync(tickets).sort()){const path=join(tickets,entry);if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);else live.push(entry);}
   if(live[0]===name){
    for(let index=0;index<count;index++){
     const path=join(slots,String(index)+".lock");
     if(stale(path,policy.slot.staleMilliseconds,now))removeStale(path,policy.slot.staleMilliseconds);
     const claim=tryCreate(path,policy.slot.heartbeatMilliseconds);
     if(claim){clearInterval(beat);rmSync(ticket,{force:true});return Object.freeze({...claim,waitedMs:Date.now()-started});}
    }
   }
   if(!reported&&Date.now()-started>200){reported=true;options.onWait?.(Date.now()-started);}
   await sleepV1(policy.slot.pollMilliseconds,options.signal);
  }
 }catch(error){clearInterval(beat);rmSync(ticket,{force:true});throw error;}
}
