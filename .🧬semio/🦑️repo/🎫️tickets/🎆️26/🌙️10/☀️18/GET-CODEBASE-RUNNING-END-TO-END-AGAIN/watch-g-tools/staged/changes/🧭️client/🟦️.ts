import {randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {
 CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1,changeAffectsV1,changeHeadV1,changeHeartbeatAgeV1,changeRootExcludedV1,lostAffectsV1,mergeChangeRowsV1,readChangeRecordsAfterV1,readChangeStateV1,touchChangeFileV1,
 type ChangeCursorV1,type ChangeJournalPolicyV1,type ChangeRecordV1,type ChangeRefusalV1,type ChangeRefusedV1,type ChangeRowV1,type ChangeStateV1,
} from "../🟦️.ts";

/** 🧭️ The answer to "what changed under these roots since this cursor": the new cursor and the changed paths, or a refusal. */
export type ChangeAnswerV1=Readonly<{exact:true;cursor:ChangeCursorV1;rows:readonly ChangeRowV1[]}>|ChangeRefusedV1;
/** 🎛️ Ports of one client: the journal directory of the workspace and the workspace root (the fence file is written below it), the on-demand starter and the clock. */
export type ChangeJournalClientOptionsV1=Readonly<{
 directory:string;root:string;insensitive?:boolean;policy?:ChangeJournalPolicyV1;environment?:Readonly<Record<string,string|undefined>>;
 start?:()=>Promise<void>;now?:()=>number;sleep?:(milliseconds:number)=>Promise<void>;
}>;
/** 🧭️ Fenced reads of the change journal of one workspace. */
export interface ChangeJournalClientV1{
 /** A predicate, fixed to the service state of this moment, for "the service never reports changes at this workspace-relative path" (false everywhere when no service state exists). */
 excluder():(path:string)=>boolean;
 /** The cursor after every change made before this call. */
 cursor(signal?:AbortSignal):Promise<ChangeCursorV1|ChangeRefusedV1>;
 /** The paths under `roots` ("" is the whole workspace) that may have changed after `from`, and the cursor after every change made before this call. */
 since(from:ChangeCursorV1,roots:readonly string[],signal?:AbortSignal):Promise<ChangeAnswerV1>;
}

/** 🎚️ The switch `SEMIO_REPO_CHANGES=on|off` (default on); anything else is a refusal in both languages. */
export const CHANGE_JOURNAL_VARIABLE_V1="SEMIO_REPO_CHANGES";
export function readChangeJournalSwitchV1(environment:Readonly<Record<string,string|undefined>>=process.env):boolean{
 const raw=environment[CHANGE_JOURNAL_VARIABLE_V1]??"on",value=raw.toLowerCase();
 if(value==="on")return true;
 if(value==="off")return false;
 throw Error(`Expected ${CHANGE_JOURNAL_VARIABLE_V1}=on|off, got "${raw}" / Erwartet ${CHANGE_JOURNAL_VARIABLE_V1}=on|off, erhalten "${raw}"`);
}

const refuse=(reason:ChangeRefusalV1,fields:Partial<Omit<ChangeRefusedV1,"exact"|"reason">>={}):ChangeRefusedV1=>Object.freeze({exact:false as const,reason,...fields});
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));

/** 🧭️ Creates the client of one workspace. Nothing is started and nothing is written until the first call. */
export function createChangeJournalClientV1(options:ChangeJournalClientOptionsV1):ChangeJournalClientV1{
 const {directory,root,policy=CHANGE_JOURNAL_POLICY_V1,environment=process.env,now=Date.now,sleep=wait}=options;
 const insensitive=options.insensitive??(process.platform==="darwin"||process.platform==="win32");
 let counter=0,touched=0;
 const recentlyUnavailable=():boolean=>{
  try{const at=(JSON.parse(readFileSync(join(directory,CHANGE_JOURNAL_FILES_V1.unavailable),"utf8")) as {at?:number}).at;return typeof at==="number"&&now()-at<policy.unavailableMilliseconds;}catch{return false;}
 };
 const live=():ChangeStateV1|undefined=>{const state=readChangeStateV1(directory);return state&&changeHeartbeatAgeV1(directory,now())<policy.heartbeatStaleMilliseconds?state:undefined;};

 async function gate(signal?:AbortSignal):Promise<ChangeStateV1|ChangeRefusedV1>{
  if(!readChangeJournalSwitchV1(environment))return refuse("disabled");
  signal?.throwIfAborted();
  let state=live();
  if(!state){
   if(recentlyUnavailable()||!options.start)return refuse("no-service");
   await options.start();signal?.throwIfAborted();state=live();
   if(!state)return refuse("no-service");
  }
  if(!state.ready)return refuse("starting");
  if(state.trust!=="exact")return refuse("untrusted",{detail:state.reason??state.backend});
  const at=now();
  if(at-touched>policy.activityMilliseconds/2){touched=at;touchChangeFileV1(directory,CHANGE_JOURNAL_FILES_V1.activity,at);}
  return state;
 }

 async function fenced(state:ChangeStateV1,after:number,signal:AbortSignal|undefined):Promise<Readonly<{records:readonly ChangeRecordV1[];fence:number}>|ChangeRefusedV1>{
  const relative=state.fence+"/"+process.pid+"-"+(++counter)+"-"+randomBytes(3).toString("hex"),file=join(state.root,...relative.split("/")),records:ChangeRecordV1[]=[];
  try{mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"",{flag:"wx"});}catch{return refuse("untrusted",{detail:"fence"});}
  const deadline=now()+policy.fenceTimeoutMilliseconds;
  try{
   for(let cursor=after,checked=now();;){
    signal?.throwIfAborted();
    const read=readChangeRecordsAfterV1(directory,state.epoch,cursor);
    if(!read.ok)return refuse(read.reason);
    for(const record of read.records){
     records.push(record);cursor=record.seq;
     if(record.rows?.some(row=>row[2]===relative))return{records,fence:record.seq};
    }
    const at=now();
    if(at>deadline)return refuse("fence-timeout");
    if(at-checked>50){checked=at;const current=readChangeStateV1(directory);if(!current||current.epoch!==state.epoch)return refuse("epoch");if(changeHeartbeatAgeV1(directory,at)>=policy.heartbeatStaleMilliseconds)return refuse("stale");}
    await sleep(policy.fencePollMilliseconds);
   }
  }finally{try{rmSync(file,{force:true});}catch{}}
 }

 const recheck=(state:ChangeStateV1):ChangeRefusedV1|undefined=>{
  const current=readChangeStateV1(directory);
  if(!current||current.epoch!==state.epoch)return refuse("epoch");
  if(current.trust!=="exact")return refuse("untrusted",{detail:current.reason??current.backend});
  return undefined;
 };

 return{
  excluder(){
   const state=readChangeStateV1(directory);
   if(state===undefined)return()=>false;
   const key=(value:string)=>insensitive?value.normalize("NFC").toLowerCase():value,rooted=new Set(state.exclude.rooted.map(key)),parents=new Map<string,boolean>(),names=new Set(state.exclude.names.map(key));
   const parent=(path:string):boolean=>{let known=parents.get(path);if(known===undefined){known=changeRootExcludedV1(state,path,insensitive);parents.set(path,known);}return known;};
   return path=>{
    const clean=path.replace(/^\/+|\/+$/gu,""),cut=clean.lastIndexOf("/"),name=clean.slice(cut+1);
    return(cut>0&&parent(clean.slice(0,cut)))||names.has(key(name))||rooted.has(key(clean));
   };
  },
  async cursor(signal){
   const state=await gate(signal);
   if("exact" in state)return state;
   const from=changeHeadV1(directory,state.epoch,policy.tailChunkBytes);
   if(from===undefined)return refuse("gap");
   const result=await fenced(state,from,signal);
   if("exact" in result)return result;
   return recheck(state)??Object.freeze({epoch:state.epoch,seq:result.fence});
  },
  async since(from,roots,signal){
   const state=await gate(signal);
   if("exact" in state)return state;
   if(from.epoch!==state.epoch)return refuse("epoch");
   const names=roots.map(root=>root.replace(/^\/+|\/+$/gu,""));
   for(const root of names)if(changeRootExcludedV1(state,root,insensitive))return refuse("excluded",{path:root});
   const result=await fenced(state,from.seq,signal);
   if("exact" in result)return result;
   const refused=recheck(state);
   if(refused)return refused;
   const rows:ChangeRowV1[]=[];
   for(const record of result.records){
    if(record.lost){
     if(lostAffectsV1(record.lost[1],names,insensitive))return refuse("lost",{path:record.lost[1]||".",detail:record.lost[0]});
     continue;
    }
    for(const row of record.rows??[]){
     if(row[2]===state.fence||row[2].startsWith(state.fence+"/"))continue;
     if(names.some(root=>changeAffectsV1(row,root,insensitive)))rows.push(row);
    }
   }
   const merged=mergeChangeRowsV1(rows);
   if(merged.length>policy.maximumRowsPerAnswer)return refuse("overflow",{count:policy.maximumRowsPerAnswer});
   return Object.freeze({exact:true as const,cursor:Object.freeze({epoch:state.epoch,seq:result.fence}),rows:merged});
  },
 };
}
