import {mkdirSync,readFileSync,readdirSync,rmSync,statSync,existsSync} from "node:fs";
import {basename,dirname,join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {CHANGE_JOURNAL_FILES_V1,CHANGE_JOURNAL_POLICY_V1,changeHeartbeatAgeV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";

/** 🧺️ The declared retention of the journal state directories; the schema file is its single source. */
export const JOURNAL_ABANDONED_POLICY_V1=Object.freeze(JSON.parse(JSON.stringify(schema.definitions.Policy.const)) as Readonly<{version:1;abandonedAfterMilliseconds:number;rootGoneQuietMilliseconds:number;identifier:string;containers:string;journalName:string}>);
/** 📊️ What one state directory shows: whether a service holds its lock, whether the worktree it was made for still exists (undefined: unknown) and the newest change inside. */
export type JournalStateFactsV1=Readonly<{held:boolean;rootExists:boolean|undefined;newestMs:number}>;
/** 📈️ The state directories a sweep removed. */
export type JournalSweepV1=Readonly<{removed:readonly string[];bytes:number}>;

/** ⚖️ Whether a state directory is abandoned: no service holds it and nothing in it changed for the abandoned age (the quiet time when its worktree is gone). */
export function journalStateAbandonedV1(facts:JournalStateFactsV1,now:number,policy:Pick<typeof JOURNAL_ABANDONED_POLICY_V1,"abandonedAfterMilliseconds"|"rootGoneQuietMilliseconds">=JOURNAL_ABANDONED_POLICY_V1):boolean{
 if(facts.held)return false;
 return now-facts.newestMs>=(facts.rootExists===false?policy.rootGoneQuietMilliseconds:policy.abandonedAfterMilliseconds);
}

const size=(path:string):number=>{try{const facts=statSync(path);if(!facts.isDirectory())return facts.size;let sum=0;for(const name of readdirSync(path))sum+=size(join(path,name));return sum;}catch{return 0;}};
const newest=(path:string,depth:number):number=>{try{const facts=statSync(path);let at=facts.mtimeMs;if(facts.isDirectory()&&depth>0)for(const name of readdirSync(path))at=Math.max(at,newest(join(path,name),depth-1));return at;}catch{return 0;}};
const aliveProcess=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(failure){return(failure as NodeJS.ErrnoException).code==="EPERM";}};

/** 🔒️ Whether a service holds the journal lock of a state directory, by the rule the service itself starts under (a live owner with a fresh heartbeat or a young lock). */
function lockHeldV1(journal:string,now:number):boolean{
 const lock=join(journal,CHANGE_JOURNAL_FILES_V1.lock);
 if(!existsSync(lock))return false;
 let owner:{pid?:unknown;at?:unknown}={};
 try{owner=JSON.parse(readFileSync(join(lock,"owner.json"),"utf8"));}catch{}
 const pid=owner.pid,valid=Number.isSafeInteger(pid)&&(pid as number)>0,at=typeof owner.at==="number"?owner.at:newest(lock,0);
 if(!valid)return now-at<CHANGE_JOURNAL_POLICY_V1.startTimeoutMilliseconds;
 return aliveProcess(pid as number)&&(changeHeartbeatAgeV1(journal,now)<CHANGE_JOURNAL_POLICY_V1.heartbeatStaleMilliseconds||now-at<CHANGE_JOURNAL_POLICY_V1.startTimeoutMilliseconds);
}

/** 🧹️ Removes the abandoned sibling state directories of the running service `own` (`<container>/<id>/journal`). It touches nothing when `own` is not laid out that way (tests, custom directories), never its own id, and takes the journal lock of a candidate before it removes the tree, so a service cannot start into a directory that is being removed. */
export function sweepAbandonedJournalStatesV1(own:string,now:number=Date.now(),policy:typeof JOURNAL_ABANDONED_POLICY_V1=JOURNAL_ABANDONED_POLICY_V1):JournalSweepV1{
 const identifier=new RegExp(policy.identifier,"u"),container=dirname(dirname(own)),removed:string[]=[];let bytes=0;
 if(basename(own)!==policy.journalName||!identifier.test(basename(dirname(own)))||!new RegExp(policy.containers,"u").test(basename(container)))return{removed,bytes};
 let names:string[]=[];
 try{names=readdirSync(container);}catch{return{removed,bytes};}
 for(const name of names.sort()){
  if(!identifier.test(name)||name===basename(dirname(own)))continue;
  const state=join(container,name),journal=join(state,policy.journalName);
  try{
   if(!statSync(journal).isDirectory())continue;
   let root:string|undefined;
   try{root=(JSON.parse(readFileSync(join(journal,CHANGE_JOURNAL_FILES_V1.state),"utf8")) as {root?:unknown}).root as string|undefined;}catch{}
   const facts:JournalStateFactsV1={held:lockHeldV1(journal,now),rootExists:typeof root==="string"?existsSync(root):undefined,newestMs:Math.max(newest(state,2),0)};
   if(!journalStateAbandonedV1(facts,now,policy))continue;
   const lock=join(journal,CHANGE_JOURNAL_FILES_V1.lock);
   try{mkdirSync(lock,{mode:0o700});}catch(failure){if((failure as NodeJS.ErrnoException).code!=="EEXIST")continue;if(lockHeldV1(journal,now))continue;}
   const freed=size(state);rmSync(state,{recursive:true,force:true});bytes+=freed;removed.push(name);
  }catch{}
 }
 return{removed,bytes};
}

const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{(\w+)\}/gu,(_all,key:string)=>String(values[key]));

/** 📣️ The report line of a sweep in one language of the schema. */
export function formatJournalSweepV1(sweep:JournalSweepV1,language:"en"|"de"):string{
 return fill(schema.definitions.Messages.const.swept[language],{count:sweep.removed.length,freedMib:Math.round(sweep.bytes/1048576*10)/10});
}

