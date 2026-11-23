import {mkdir,readdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {storePathsV1} from "../📒️journal/🟦️.ts";

/** 🔐️ Why the replica of a machine cannot work on its worktree: `denied` (the operating system refuses the read, for example the macOS consent for the folder was declined) or `waiting-for-consent` (the read does not return, because a consent prompt waits for an answer). */
export type AccessStateV1="denied"|"waiting-for-consent";

/** 🔎️ The record `origins/<origin>/access.json` of the store, shaped by `🧬️schema/🔣️.json`: written by the daemon while it cannot read its worktree and removed when it can. */
export type AccessRecordV1=Readonly<{version:1;originId:string;state:AccessStateV1;wallMs:number;worktree:string;platform:string;error:string}>;

/** 🚪️ What one probe of the worktree found: `ok`, `denied` or `waiting` (no answer within the time limit). */
export type AccessProbeV1="ok"|"denied"|"waiting";

const code=(error:unknown):string|undefined=>(error as {code?:string}).code;
const recordFile=(storeRoot:string,originId:string):string=>join(storePathsV1(storeRoot).origins,originId,"access.json");

/** 🚪️ Probes whether the worktree can be listed (`list` replaces the directory listing in tests): a refusal (`EPERM`, `EACCES`) is `denied`, a call that has not returned within `timeoutMilliseconds` is `waiting`, any other outcome (including a missing directory) is `ok`, because that is not an access problem. A call that hangs stays pending in the background and is never started twice by the caller. */
export async function probeWorktreeAccessV1(worktree:string,timeoutMilliseconds:number,list:(path:string)=>Promise<unknown>=readdir):Promise<AccessProbeV1>{
 let timer:ReturnType<typeof setTimeout>|undefined;
 const listing=list(worktree).then((): AccessProbeV1=>"ok",(error:unknown):AccessProbeV1=>code(error)==="EPERM"||code(error)==="EACCES"?"denied":"ok");
 const limit=new Promise<AccessProbeV1>(done=>{timer=setTimeout(()=>{done("waiting");},timeoutMilliseconds);});
 try{return await Promise.race([listing,limit]);}finally{clearTimeout(timer);}
}

/** ✍️ Writes the access record of one origin atomically through the staging folder of the store. */
export async function writeAccessV1(storeRoot:string,record:AccessRecordV1):Promise<void>{
 const file=recordFile(storeRoot,record.originId),staging=storePathsV1(storeRoot).tmp,temporary=join(staging,`access-${record.originId}-${process.pid}`);
 await mkdir(staging,{recursive:true});await mkdir(join(file,".."),{recursive:true});
 await writeFile(temporary,JSON.stringify(record));
 await rename(temporary,file);
}

/** 📖️ Reads the access record of one origin; `undefined` when there is none (the daemon can read its worktree) or it cannot be read. */
export async function readAccessV1(storeRoot:string,originId:string):Promise<AccessRecordV1|undefined>{
 try{
  const value=JSON.parse(await readFile(recordFile(storeRoot,originId),"utf8")) as AccessRecordV1;
  return value.version===1&&(value.state==="denied"||value.state==="waiting-for-consent")&&value.originId===originId?value:undefined;
 }catch{return undefined;}
}

/** 🧹️ Removes the access record of one origin. */
export async function clearAccessV1(storeRoot:string,originId:string):Promise<void>{
 await rm(recordFile(storeRoot,originId),{force:true});
}

/** 📋️ Lists the origins of the store whose daemon currently reports that it cannot read its worktree. */
export async function listAccessV1(storeRoot:string):Promise<readonly AccessRecordV1[]>{
 const found:AccessRecordV1[]=[];
 for(const origin of await readdir(storePathsV1(storeRoot).origins).catch(()=>[] as string[])){
  const record=await readAccessV1(storeRoot,origin);
  if(record!==undefined)found.push(record);
 }
 return found;
}

/** ⏳️ Holds the daemon back until it can read its worktree: while a probe says `denied` or `waiting`, the record is written (and refreshed every 30 s) and `report` is called once per state change; the loop ends when the worktree is readable, which also removes the record, or when the signal aborts. */
export async function awaitWorktreeAccessV1(options:Readonly<{worktree:string;storeRoot:string;originId:string;platform?:string;probeMilliseconds?:number;pollMilliseconds?:number;list?:(path:string)=>Promise<unknown>}>,signal:AbortSignal,report:(state:AccessStateV1,error:string)=>void):Promise<void>{
 const probe=options.probeMilliseconds??8000,poll=options.pollMilliseconds??5000;
 let last:AccessStateV1|undefined,written=0;
 for(;;){
  signal.throwIfAborted();
  const outcome=await probeWorktreeAccessV1(options.worktree,probe,options.list);
  if(outcome==="ok"){if(last!==undefined)await clearAccessV1(options.storeRoot,options.originId).catch(()=>undefined);return;}
  const state:AccessStateV1=outcome==="denied"?"denied":"waiting-for-consent",error=outcome==="denied"?"EPERM":"no answer";
  if(state!==last)report(state,error);
  if(state!==last||Date.now()-written>30_000){
   await writeAccessV1(options.storeRoot,{version:1,originId:options.originId,state,wallMs:Date.now(),worktree:options.worktree,platform:options.platform??process.platform,error}).catch(()=>undefined);
   written=Date.now();
  }
  last=state;
  await new Promise<void>(done=>{const finish=():void=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();},timer=setTimeout(finish,poll);signal.addEventListener("abort",finish,{once:true});});
 }
}
