import {readdirSync,statSync} from "node:fs";
import {join} from "node:path";
import {ownersOfAdmissionMemoV1,readAdmissionMemoV1} from "../🧠️admission-memo/🟦️.ts";
import {readForgottenPathsV1,rowsForgottenV1} from "../🪦️forgotten-paths/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const limits=schema.$defs.Limits.const;

/** 🌱️ Seeds the owner facts of the newest admission memos of other selections of this format: an owner's facts depend on the owner, its targets and its inputs, never on the selection that asked, and every seeded entry revalidates its own rows before it answers, so a read whose own memo is missing or stale re-observes only the owners whose rows changed instead of every owner of its closure. Own scope, foreign formats, oversized memos and owners the caller already knows are skipped; a corrupt or vanished memo is a miss.
 * @see ../🧠️admission-memo/🟦️.ts
 * @see ./🧪️tests/🟦️.ts */
export async function seedFromSiblingMemosV1(input:{readonly directory:string;readonly scope:string;readonly format:string;readonly known:Set<string>;readonly seed:(suffix:string,facts:unknown,rows:readonly unknown[])=>void;readonly signal?:AbortSignal}):Promise<{memos:number;owners:number}>{
 const candidates:{scope:string;file:string;size:number;at:number}[]=[];
 try{
  for(const name of readdirSync(input.directory)){
   const found=/^([0-9a-f]{64})\.admission\.json$/u.exec(name);
   if(found===null||found[1]===input.scope)continue;
   try{const status=statSync(join(input.directory,name));if(status.isFile()&&status.size<=limits.maximumMemoBytes)candidates.push({scope:found[1]!,file:join(input.directory,name),size:status.size,at:status.mtimeMs});}catch{}
  }
 }catch{return {memos:0,owners:0};}
 candidates.sort((a,b)=>b.at-a.at||(a.scope<b.scope?-1:1));
 let memos=0,owners=0;
 for(const candidate of candidates.slice(0,limits.maximumAttempts)){
  if(memos>=limits.maximumMemos)break;
  input.signal?.throwIfAborted();
  const lent=await readAdmissionMemoV1(candidate.file,candidate.scope,value=>value.format===input.format);
  if(lent===undefined)continue;
  memos++;
  const forgotten=readForgottenPathsV1(join(input.directory,candidate.scope+".forgotten.json"));
  try{
   for await(const owner of ownersOfAdmissionMemoV1(lent,input.known)){
    input.signal?.throwIfAborted();
    if(rowsForgottenV1(forgotten,owner.rows,candidate.at))continue;
    input.seed(owner.suffix,owner.facts,owner.rows);input.known.add(owner.suffix);owners++;
   }
  }finally{await lent.close();}
 }
 return {memos,owners};
}
