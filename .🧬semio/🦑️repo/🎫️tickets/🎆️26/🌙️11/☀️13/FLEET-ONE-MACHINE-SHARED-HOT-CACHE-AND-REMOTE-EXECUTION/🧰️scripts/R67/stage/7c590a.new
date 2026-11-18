import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import type {GraphStateV1} from "../../🟦️.ts";
import type {GraphDocumentV1} from "../../🛰️exchange/🟦️.ts";

/** 📐️ The numbers of the spool of finished records that were not stored.
 * @see ../🧬️schema/🔣️.json */
export const PROJECT_RECORD_UNSENT_POLICY_V1=schema.definitions.Publish.const.unsent;
/** 📮️ One finished record on disk: its description and its documents. */
export type UnsentRecordV1=Readonly<{meta:Readonly<{version:1;state:GraphStateV1;owner:string;createdMs:number;attempts:number;cause:string;documents:readonly Readonly<{name:string;size:number;sha256:string}>[]}>;documents:readonly GraphDocumentV1[]}>;

const sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),home=(directory:string,digest:string)=>join(directory,digest);

/** 📮️ The digests in the spool, oldest first; an unreadable description is skipped. */
export function unsentListV1(directory:string):readonly Readonly<{digest:string;createdMs:number;bytes:number}>[]{
 let names:string[]=[];try{names=readdirSync(directory).filter(name=>/^[0-9a-f]{64}$/u.test(name));}catch{return[];}
 const rows:{digest:string;createdMs:number;bytes:number}[]=[];
 for(const digest of names)try{const meta=JSON.parse(readFileSync(join(home(directory,digest),"meta.json"),"utf8")) as UnsentRecordV1["meta"];rows.push({digest,createdMs:meta.createdMs,bytes:meta.documents.reduce((sum,row)=>sum+row.size,0)});}catch{}
 return rows.sort((a,b)=>a.createdMs-b.createdMs||(a.digest<b.digest?-1:1));
}

/** 📮️ Removes one record from the spool. */
export function unsentDropV1(directory:string,digest:string):void{try{rmSync(home(directory,digest),{recursive:true,force:true});}catch{}}

/** 📮️ Drops the records older than the maximum age, then the oldest ones until the entries and bytes fit the bounds (`reserve` bytes and one entry kept free for the record about to be put). Returns how many were dropped. */
export function unsentPruneV1(directory:string,now:number,reserve:number=0,policy:typeof PROJECT_RECORD_UNSENT_POLICY_V1=PROJECT_RECORD_UNSENT_POLICY_V1):number{
 let rows=[...unsentListV1(directory)],dropped=0;
 for(const row of rows.filter(each=>now-each.createdMs>policy.maximumAgeMilliseconds)){unsentDropV1(directory,row.digest);dropped++;}
 rows=[...unsentListV1(directory)];
 let bytes=rows.reduce((sum,row)=>sum+row.bytes,0);
 while(rows.length>0&&(rows.length+(reserve>0?1:0)>policy.maximumEntries||bytes+reserve>policy.maximumBytes)){const row=rows.shift()!;unsentDropV1(directory,row.digest);bytes-=row.bytes;dropped++;}
 return dropped;
}

/** 📮️ Keeps a finished record on disk until it is stored: written to a directory of its own and moved into place, so a reader sees all of it or none. A record alone above the byte bound is not kept (`false`). */
export function unsentPutV1(directory:string,input:Readonly<{state:GraphStateV1;owner:string;documents:readonly GraphDocumentV1[];cause:string;attempts:number;now:number}>,policy:typeof PROJECT_RECORD_UNSENT_POLICY_V1=PROJECT_RECORD_UNSENT_POLICY_V1):boolean{
 const size=input.documents.reduce((sum,row)=>sum+row.bytes.length,0);
 if(size>policy.maximumBytes)return false;
 try{
  mkdirSync(directory,{recursive:true});
  const kept=existsSync(home(directory,input.state.digest));
  if(!kept)unsentPruneV1(directory,input.now,size,policy);
  const target=home(directory,input.state.digest),temporary=`${target}.${process.pid}.tmp`;
  rmSync(temporary,{recursive:true,force:true});mkdirSync(temporary,{recursive:true});
  input.documents.forEach((row,index)=>writeFileSync(join(temporary,`${index}.bin`),row.bytes));
  const meta={version:1,state:input.state,owner:input.owner,createdMs:kept?readCreated(target,input.now):input.now,attempts:input.attempts,cause:input.cause.slice(0,300),documents:input.documents.map(row=>({name:row.name,size:row.bytes.length,sha256:sha(row.bytes)}))};
  writeFileSync(join(temporary,"meta.json"),JSON.stringify(meta));
  rmSync(target,{recursive:true,force:true});renameSync(temporary,target);
  return true;
 }catch{return false;}
}
const readCreated=(target:string,fallback:number):number=>{try{return(JSON.parse(readFileSync(join(target,"meta.json"),"utf8")) as {createdMs:number}).createdMs;}catch{return fallback;}};

/** 📮️ Reads one record back, every document verified against its size and digest; `undefined` when anything is missing or damaged (the record is then dropped by the caller). */
export function unsentReadV1(directory:string,digest:string):UnsentRecordV1|undefined{
 try{
  const base=home(directory,digest),meta=JSON.parse(readFileSync(join(base,"meta.json"),"utf8")) as UnsentRecordV1["meta"];
  if(meta.version!==1||meta.state.digest!==digest)return undefined;
  const documents=meta.documents.map((row,index)=>{const bytes=new Uint8Array(readFileSync(join(base,`${index}.bin`)));if(bytes.length!==row.size||sha(bytes)!==row.sha256)throw Error("damaged");return{name:row.name,bytes};});
  return{meta,documents};
 }catch{return undefined;}
}

/** 📮️ The bytes the spool holds on disk. */
export function unsentBytesV1(directory:string):number{
 let total=0;
 for(const row of unsentListV1(directory))try{for(const name of readdirSync(home(directory,row.digest)))total+=statSync(join(home(directory,row.digest),name)).size;}catch{}
 return total;
}
