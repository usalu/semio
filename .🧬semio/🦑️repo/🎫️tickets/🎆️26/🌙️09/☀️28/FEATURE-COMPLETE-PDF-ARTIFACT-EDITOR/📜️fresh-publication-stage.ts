/** 🚪️ Extends fresh compiled-artifact directory admission while preserving the target try-lock. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
const source="🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts", schema="🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧬️schema/🚪️fresh-admission.json";
const manifest=join(import.meta.dir,"🗑️generated","fresh-publication-admission.json");
const hash=(s:string)=>createHash("sha256").update(s).digest("hex");
const replace=(s:string,a:string,b:string)=>{
  assert.equal(s.split(a).length,2,a);
  return s.replace(a,b);
};
if(process.argv.includes("--apply")){
 const rows=JSON.parse(readFileSync(manifest,"utf8")) as {path:string;before:string|null;after:string}[];
 const check=(row:typeof rows[number])=>assert.equal(existsSync(row.path)?hash(readFileSync(row.path,"utf8")):null,row.before,row.path);
 for(const row of rows)check(row);
 for(const row of rows){
   check(row);
   writeFileSync(row.path,row.after);
   assert.equal(readFileSync(row.path,"utf8"),row.after,row.path);
 }
 console.log("[DEBUG] Applied the guarded fresh artifact gate policy and caller");
}else{
 assert.ok(resolve(schema).length<=256);
 assert.ok(!existsSync(schema));
 const old=readFileSync(source,"utf8");
 let next=replace(old,"import { acquireFreshResourceLeaseV1, acquireResourceLease,","import { acquireResourceLease,");
 next=replace(next,'import {refuseProcessV1} from "../../📖️texts/🟦️.ts";','import {refuseProcessV1} from "../../📖️texts/🟦️.ts";\nimport freshAdmission from "./🧬️schema/🚪️fresh-admission.json" with {type:"json"};');
 next=replace(next,
  'const lease=await acquireFreshResourceLeaseV1({directory:leaseDirectory,resource,mode:"exclusive",control,signal:control.signal});',
  'const lease=await acquireResourceLease({directory:leaseDirectory,resource,mode:"exclusive",control,signal:control.signal,timeoutMs:0,admissionPatienceMs:freshAdmission.const.directoryPatienceMilliseconds});');
 next=replace(next,
  "/** 🗝️ Retains both physical writers for one fresh destination before entering its higher writer scope. */",
  "/** 🗝️ Retains both fresh publication writers with a bounded directory wait and a single destination lock attempt. */");
 const rules={ "$schema":"https://json-schema.org/draft/2020-12/schema", const:{directoryPatienceMilliseconds:60000} };
 writeFileSync(manifest,JSON.stringify([{path:schema,before:null,after:JSON.stringify(rules,null,2)+"\n"},{path:source,before:hash(old),after:next}],null,2)+"\n");
 console.log("[DEBUG] Staged exact fresh artifact gate policy and caller");
}
