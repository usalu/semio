import {createHash} from "node:crypto";
import {appendFileSync,mkdirSync,readdirSync,readFileSync,realpathSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {homedir} from "node:os";
import {join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {AGENT_SESSION_POLICY_V1,agentSessionSharesV1,agentSessionStateV1,isAgentSessionV1,type AgentSessionAnchorV1,type AgentSessionStateV1,type AgentSessionV1,type DerivedAgentSessionV1} from "../🪪️identity/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const policy=AGENT_SESSION_POLICY_V1,live=policy.liveness,files={record:".json",seen:".seen",token:".token",lock:".lock"};
/** 🗄️ One worktree's directory inside the per-user registry. */
export type AgentSessionStoreV1=Readonly<{root:string;worktree:string;directory:string}>;
/** 🤖️ One session with the facts a status needs. */
export type AgentSessionRowV1=Readonly<{session:AgentSessionV1;state:AgentSessionStateV1;lastSeenMs:number;shareBasisPoints:number}>;
/** 🔌️ The process facts the store depends on, replaceable in tests. */
export type AgentSessionPortsV1=Readonly<{alive?:(pid:number)=>boolean;now?:number}>;

const defaultAlive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
const sleep=(milliseconds:number):void=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};
const zero="0".repeat(policy.store.worktreeCharacters);

/** 🗺️ The registry root: one directory per user shared by every checkout, next to the lease registry. */
export function agentSessionRegistryRootV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment[policy.environment.registry];if(override)return override;
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):environment.XDG_CACHE_HOME??join(home,".cache");
 return join(base,"semio",policy.store.folder);
}

/** 🔑️ Names a worktree by twelve hex characters of the digest of its real path, case-folded where the file system is. */
export function agentSessionWorktreeKeyV1(worktree:string|undefined,platform:NodeJS.Platform=process.platform):string{
 if(worktree===undefined)return zero;
 let path=resolve(worktree);try{path=realpathSync(path);}catch{}
 const folded=(platform==="win32"||platform==="darwin"?path.toLowerCase():path).replaceAll("\\","/");
 return createHash("sha256").update(policy.identity.domain+"\0worktree\0"+folded).digest("hex").slice(0,policy.store.worktreeCharacters);
}

/** 🗃️ Binds a store to a registry root and a worktree key. */
export function agentSessionStoreV1(root:string,worktree:string):AgentSessionStoreV1{return Object.freeze({root,worktree,directory:join(root,worktree)});}

/** 🧭️ The wire form of a session in the environment: worktree key, a dot and the session identifier. */
export const agentSessionAddressV1=(session:Pick<AgentSessionV1,"worktree"|"id">):string=>`${session.worktree}.${session.id}`;
/** ⌨️ Splits an address into its worktree key and identifier, or answers undefined. */
export function parseAgentSessionAddressV1(text:string|undefined):Readonly<{worktree:string;id:string}>|undefined{
 const match=text===undefined?null:new RegExp(policy.store.addressPattern,"u").exec(text);return match?{worktree:match[1]!,id:match[2]!}:undefined;
}

/** 🔤️ The address of the session an environment names, or undefined; a cheap read that touches no file. */
export const agentSessionAddressOfEnvironmentV1=(environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined=>{
 const text=environment[policy.environment.session];return parseAgentSessionAddressV1(text)===undefined?undefined:text;
};

/** 📂️ The directories a session owns on this host below the host-wide sandbox parent: the root of its test sandboxes and its temp root. */
export function agentSessionPathsV1(session:Pick<AgentSessionV1,"resources">,parent:string):Readonly<{sandboxes:string;temp:string}>{
 const sandboxes=join(parent,session.resources.tests.root);return Object.freeze({sandboxes,temp:join(sandboxes,policy.store.testsFolder)});
}

const path=(store:AgentSessionStoreV1,id:string,kind:keyof typeof files)=>join(store.directory,id+files[kind]);
function atomic(target:string,text:string,mode=0o600):void{
 const temporary=`${target}.${process.pid}.tmp`;writeFileSync(temporary,text,{mode});renameSync(temporary,target);
}
function locked<T>(store:AgentSessionStoreV1,id:string,body:()=>T):T{
 const lock=path(store,id,"lock"),until=Date.now()+5000;
 for(;;){
  try{mkdirSync(lock,{mode:0o700});break;}catch(error){
   if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;
   try{if(Date.now()-statSync(lock).mtimeMs>live.lockStaleMilliseconds)rmSync(lock,{recursive:true,force:true});}catch{}
   if(Date.now()>until)throw new Error(`Agent session ${id} is locked`);sleep(10);
  }
 }
 try{return body();}finally{rmSync(lock,{recursive:true,force:true});}
}

/** 📖️ Reads one session; a missing, foreign or malformed record is no session. */
export function readAgentSessionV1(store:AgentSessionStoreV1,id:string):AgentSessionV1|undefined{
 try{const value=JSON.parse(readFileSync(path(store,id,"record"),"utf8"));return isAgentSessionV1(value)&&value.id===id&&value.worktree===store.worktree?Object.freeze(value):undefined;}catch{return undefined;}
}

/** 👀️ The last time anything used the session. */
export function agentSessionLastSeenMsV1(store:AgentSessionStoreV1,session:AgentSessionV1):number{
 try{return Math.floor(statSync(path(store,session.id,"seen")).mtimeMs);}catch{return session.openedAtMs;}
}

/** 👆️ Records a use of the session, at most once per touch interval. */
export function touchAgentSessionV1(store:AgentSessionStoreV1,id:string,now:number=Date.now(),force=false):void{
 const target=path(store,id,"seen");
 try{if(!force&&now-statSync(target).mtimeMs<live.touchSeconds*1000)return;}catch{}
 try{writeFileSync(target,"");utimesSync(target,now/1000,now/1000);}catch{}
}

function merge(anchors:readonly AgentSessionAnchorV1[],anchor:AgentSessionAnchorV1|undefined,alive:(pid:number)=>boolean):readonly AgentSessionAnchorV1[]{
 if(anchor===undefined)return anchors;
 let next=[...anchors.filter(row=>row.pid!==anchor.pid),anchor];
 if(next.length>live.anchorsMaximum)next=next.filter((row,index)=>alive(row.pid)||index===next.length-1).slice(-live.anchorsMaximum);
 return next;
}

/** 📜️ Appends one audit row to the ledger of the worktree. */
export function appendAgentSessionLedgerV1(store:AgentSessionStoreV1,event:Readonly<{event:"open"|"close"|"collect";session:string;agent:string;pid?:number;reason?:string}>,now:number=Date.now()):void{
 const row={version:1,atMs:now,...event};
 if(validateJsonSchemaSubset(schema.definitions.LedgerEvent,row,schema).length>0)return;
 try{mkdirSync(store.directory,{recursive:true,mode:0o700});appendFileSync(join(store.directory,"ledger.jsonl"),JSON.stringify(row)+"\n");}catch{}
}

/** 🚪️ Opens a session or attaches to the existing one of the same identifier: the owner anchor is added, the weight updated when given, the use recorded. The record is written only when something changed. */
export function openAgentSessionV1(store:AgentSessionStoreV1,derived:DerivedAgentSessionV1,options:AgentSessionPortsV1&Readonly<{weight?:number}>={}):Readonly<{session:AgentSessionV1;created:boolean}>{
 const now=options.now??Date.now(),alive=options.alive??defaultAlive;
 mkdirSync(store.directory,{recursive:true,mode:0o700});
 return locked(store,derived.id,()=>{
  const existing=readAgentSessionV1(store,derived.id),{anchor,...rest}=derived;
  if(existing!==undefined){
   const anchors=merge(existing.anchors,anchor,alive),weight=options.weight??existing.budget.weight;
   const unchanged=anchors.length===existing.anchors.length&&anchors.every((row,index)=>row.pid===existing.anchors[index]!.pid&&row.started===existing.anchors[index]!.started)&&weight===existing.budget.weight;
   if(unchanged){touchAgentSessionV1(store,existing.id,now);return {session:existing,created:false};}
   const next=Object.freeze({...existing,anchors:Object.freeze(anchors),budget:Object.freeze({weight})});
   atomic(path(store,existing.id,"record"),JSON.stringify(next));touchAgentSessionV1(store,existing.id,now,true);
   return {session:next,created:false};
  }
  const created:AgentSessionV1=Object.freeze({...rest,worktree:store.worktree,anchors:Object.freeze(merge([],anchor,alive)),openedAtMs:now,budget:Object.freeze({weight:options.weight??derived.budget.weight})});
  atomic(path(store,created.id,"record"),JSON.stringify(created));touchAgentSessionV1(store,created.id,now,true);
  appendAgentSessionLedgerV1(store,{event:"open",session:created.id,agent:created.agent.id,pid:process.pid},now);
  return {session:created,created:true};
 });
}

/** 🎫️ Records the capability a session was issued (scopes, projects, expiry) in its record; the token itself stays in the private token file. */
export function setAgentSessionCapabilityV1(store:AgentSessionStoreV1,id:string,capability:AgentSessionV1["capability"]):AgentSessionV1|undefined{
 return locked(store,id,()=>{
  const existing=readAgentSessionV1(store,id);if(existing===undefined)return undefined;
  const next=Object.freeze({...existing,capability:Object.freeze({...capability})});
  if(!isAgentSessionV1(next))throw new Error("Closed agent session capability required");
  atomic(path(store,id,"record"),JSON.stringify(next));return next;
 });
}

/** 📋️ Lists the sessions of a worktree with their state, last use and share of the budget (live sessions split it by weight). */
export function listAgentSessionsV1(store:AgentSessionStoreV1,ports:AgentSessionPortsV1={}):readonly AgentSessionRowV1[]{
 const now=ports.now??Date.now(),alive=ports.alive??defaultAlive;
 let names:string[];try{names=readdirSync(store.directory);}catch{return [];}
 const found:{session:AgentSessionV1;state:AgentSessionStateV1;lastSeenMs:number}[]=[];
 for(const name of names.filter(row=>row.endsWith(files.record)).sort()){
  const session=readAgentSessionV1(store,name.slice(0,-files.record.length));if(session===undefined)continue;
  const lastSeenMs=agentSessionLastSeenMsV1(store,session),state=agentSessionStateV1({ageSeconds:Math.max(0,(now-lastSeenMs)/1000),anchorAlive:session.anchors.some(row=>alive(row.pid)),isDefault:session.agent.origin==="default"});
  found.push({session,state,lastSeenMs});
 }
 const shares=agentSessionSharesV1(found.filter(row=>row.state==="live").map(row=>({id:row.session.id,weight:row.session.budget.weight})));
 return Object.freeze(found.map(row=>Object.freeze({...row,shareBasisPoints:shares[row.session.id]??0})));
}

/** 🥧️ What one session holds of the machine's admission gate. */
export type AgentSessionCapacityRowV1=Readonly<{address:string;agent:string;grants:number;reservedMegabytes:number;waiters:number;weight:number;shareBasisPoints:number}>;

/** 👥️ Groups the grants and waiters of the admission gate by the session that owns them and adds each session's agent, weight and share of the live budget; rows without a readable session keep their address as the agent. */
export function agentSessionCapacityRowsV1(input:Readonly<{grants:readonly Readonly<{session?:string;reservedMegabytes:number;lentSinceMs?:number}>[];waiters:readonly Readonly<{session?:string;weight?:number}>[]}>,registry:string,ports:AgentSessionPortsV1={}):readonly AgentSessionCapacityRowV1[]{
 const rows=new Map<string,{grants:number;reservedMegabytes:number;waiters:number;weight?:number}>(),row=(address:string)=>{let found=rows.get(address);if(found===undefined){found={grants:0,reservedMegabytes:0,waiters:0};rows.set(address,found);}return found;};
 for(const grant of input.grants)if(grant.session!==undefined){const found=row(grant.session);found.grants++;if(grant.lentSinceMs===undefined)found.reservedMegabytes+=grant.reservedMegabytes;}
 for(const waiter of input.waiters)if(waiter.session!==undefined){const found=row(waiter.session);found.waiters++;if(waiter.weight!==undefined)found.weight=waiter.weight;}
 const lists=new Map<string,readonly AgentSessionRowV1[]>(),result:AgentSessionCapacityRowV1[]=[];
 for(const [address,counts] of [...rows].sort(([left],[right])=>left<right?-1:left>right?1:0)){
  const parsed=parseAgentSessionAddressV1(address);let known:AgentSessionRowV1|undefined;
  if(parsed!==undefined){
   if(!lists.has(parsed.worktree))lists.set(parsed.worktree,listAgentSessionsV1(agentSessionStoreV1(registry,parsed.worktree),ports));
   known=lists.get(parsed.worktree)!.find(candidate=>candidate.session.id===parsed.id);
  }
  result.push(Object.freeze({address,agent:known?.session.agent.id??address,grants:counts.grants,reservedMegabytes:counts.reservedMegabytes,waiters:counts.waiters,weight:known?.session.budget.weight??counts.weight??1,shareBasisPoints:known?.shareBasisPoints??0}));
 }
 return result;
}

/** 🔏️ Stores the capability token of a session in a private file; the record never carries it. */
export function writeAgentSessionTokenV1(store:AgentSessionStoreV1,id:string,token:string):void{mkdirSync(store.directory,{recursive:true,mode:0o700});atomic(path(store,id,"token"),token,0o600);}
/** 🎨️ Reads the capability token of a session. */
export function readAgentSessionTokenV1(store:AgentSessionStoreV1,id:string):string|undefined{try{return readFileSync(path(store,id,"token"),"utf8");}catch{return undefined;}}

/** 🔒️ Removes a session's record, use marker and token. */
export function closeAgentSessionV1(store:AgentSessionStoreV1,id:string,reason:string,now:number=Date.now()):boolean{
 return locked(store,id,()=>{
  const session=readAgentSessionV1(store,id);
  for(const kind of ["record","seen","token"] as const)rmSync(path(store,id,kind),{force:true});
  if(session!==undefined)appendAgentSessionLedgerV1(store,{event:"close",session:id,agent:session.agent.id,pid:process.pid,reason},now);
  return session!==undefined;
 });
}

/** 🧹️ What a reclaimer reports for one session: resources released and resources kept because something still uses them. */
export type AgentSessionReclaimV1=(session:AgentSessionV1)=>Readonly<{released:number;kept:number}>;
/** 📚️ One collected session. */
export type AgentSessionCollectionV1=Readonly<{session:AgentSessionV1;released:number;kept:number;removed:boolean}>;

/** 🗑️ Collects the gone sessions: each one's resources are reclaimed first and the record goes only when nothing was kept; the default session is never collected. */
export function collectAgentSessionsV1(store:AgentSessionStoreV1,reclaim:AgentSessionReclaimV1,options:AgentSessionPortsV1&Readonly<{dryRun?:boolean;only?:readonly string[];reason?:string}>={}):readonly AgentSessionCollectionV1[]{
 const now=options.now??Date.now(),rows:AgentSessionCollectionV1[]=[];
 for(const row of listAgentSessionsV1(store,options)){
  if(row.state!=="gone"&&options.only===undefined||row.session.agent.origin==="default"||options.only!==undefined&&!options.only.includes(row.session.id))continue;
  if(options.dryRun){rows.push({session:row.session,released:0,kept:0,removed:false});continue;}
  const result=reclaim(row.session),removed=result.kept===0&&closeAgentSessionV1(store,row.session.id,options.reason??"gone",now);
  if(removed)appendAgentSessionLedgerV1(store,{event:"collect",session:row.session.id,agent:row.session.agent.id,reason:options.reason??"gone"},now);
  rows.push({session:row.session,released:result.released,kept:result.kept,removed});
 }
 return rows;
}

/** ⏲️ Collects at most once per collection interval, so every command may call it. */
export function collectAgentSessionsIfDueV1(store:AgentSessionStoreV1,reclaim:AgentSessionReclaimV1,options:AgentSessionPortsV1={}):readonly AgentSessionCollectionV1[]{
 const now=options.now??Date.now(),stamp=join(store.directory,"collect.stamp");
 try{if(now-statSync(stamp).mtimeMs<live.collectIntervalSeconds*1000)return [];}catch{}
 try{mkdirSync(store.directory,{recursive:true,mode:0o700});writeFileSync(stamp,String(now));utimesSync(stamp,now/1000,now/1000);}catch{return [];}
 const rows=collectAgentSessionsV1(store,reclaim,options);
 try{
  const ledger=join(store.directory,"ledger.jsonl");
  if(statSync(ledger).size>live.ledgerMaximumBytes)atomic(ledger,readFileSync(ledger,"utf8").split("\n").slice(-live.ledgerKeepLines-1).join("\n"),0o644);
 }catch{}
 return rows;
}
