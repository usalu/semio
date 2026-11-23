import {readdirSync,rmSync,statSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout,turn=()=>new Promise<void>(done=>setImmediate(done));
/** 📊️ What one retention pass over the admission memos removed and kept. */
export type AdmissionRetentionV1=Readonly<{removed:number;freedBytes:number;keptBytes:number;keptFiles:number}>;
/** 📋️ The bounds of the memos: how long one may live, how long the temporary file of a writer that died may stay and how many bytes all of them may take together. */
export type AdmissionLimitsV1=Readonly<{retentionMs:number;temporaryMs:number;maximumBytes:number}>;

/** 🧹️ Retires the admission memos nobody needs. A memo only saves a hash (every recall checks the stamps of its probed paths again), so a removed memo is a miss, never a wrong hit: those older than the retention go, temporary files of dead writers after their short bound, then the oldest remaining ones until the rest fits `maximumBytes`. Streams the shards, yields to the event loop and stops at a cancellation. @see https://doc.rust-lang.org/cargo/guide/build-cache.html */
export async function pruneAdmissionsV1(root:string,nowMs:number,limits:AdmissionLimitsV1,signal?:AbortSignal):Promise<AdmissionRetentionV1>{
 const base=join(root,layout.admissions),kept:{path:string;bytes:number;mtimeMs:number}[]=[];let shards:string[];
 try{shards=readdirSync(base);}catch{return{removed:0,freedBytes:0,keptBytes:0,keptFiles:0};}
 let removed=0,freedBytes=0,keptBytes=0,seen=0;
 const drop=(path:string,bytes:number):void=>{try{rmSync(path,{force:true});removed++;freedBytes+=bytes;}catch{}};
 for(const shard of shards){
  let names:string[];try{names=readdirSync(join(base,shard));}catch{continue;}
  for(const name of names){
   signal?.throwIfAborted();if(++seen%4096===0)await turn();
   const path=join(base,shard,name);let stat;try{stat=statSync(path);}catch{continue;}
   if(!stat.isFile())continue;
   const bytes=Math.max(stat.size,(stat.blocks??0)*512);
   if(nowMs-stat.mtimeMs>(name.endsWith(".json")?limits.retentionMs:limits.temporaryMs)){drop(path,bytes);continue;}
   kept.push({path,bytes,mtimeMs:stat.mtimeMs});keptBytes+=bytes;
  }
 }
 let index=0;
 if(keptBytes>limits.maximumBytes){
  kept.sort((left,right)=>left.mtimeMs-right.mtimeMs||(left.path<right.path?-1:1));
  for(;index<kept.length&&keptBytes>limits.maximumBytes;index++){signal?.throwIfAborted();if(index%4096===4095)await turn();drop(kept[index]!.path,kept[index]!.bytes);keptBytes-=kept[index]!.bytes;}
 }
 return Object.freeze({removed,freedBytes,keptBytes,keptFiles:kept.length-index});
}
