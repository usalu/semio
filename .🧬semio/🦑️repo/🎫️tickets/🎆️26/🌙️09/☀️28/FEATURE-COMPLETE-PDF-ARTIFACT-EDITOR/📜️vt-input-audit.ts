/** 🔎 Checks bilingual PDF action annotations with the actual input resolver and Ajv. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import Ajv from "ajv";
const {mutationInputAudit,mutationInputDefs}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🛂️manifest/📥️mutation-input-schema/🟦️.ts")).href);
const {valueSchema}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/🧬️schema/🟦️.ts")).href);
const [base,out]=process.argv.slice(2);
const raw=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
const fixture=raw(join(import.meta.dirname,"🔣️vt-input-cases.json"));
const overlays=raw(join(out,"receipt.json"));
const live=(x:string)=>raw(join(base,x));
const staged=(x:string)=>overlays[x]?raw(join(out,overlays[x].overlay)):live(x);
const snapshotPath="📸️snapshot/🔣️.json";

function annotationsOnly(doc:unknown):unknown {
  if(!doc||typeof doc!=="object")return doc;
  if(Array.isArray(doc))return doc.map(annotationsOnly);
  return Object.fromEntries(Object.entries(doc).filter(([key])=>key!=="x-semio-ui").map(([key,node])=>[key,annotationsOnly(node)]));
}
assert.deepEqual(annotationsOnly(live(snapshotPath)),annotationsOnly(staged(snapshotPath)));
for(const sample of fixture){
  const path="🧬️mutations/"+sample.name+"/🧬️schema/🔣️.json";
  const previous=live(path),after=staged(path);
  const lookup=(snapshot:any)=>(id:string)=>snapshot.$id===id?snapshot:undefined;
  const malformed=structuredClone(sample.payload);
  const selector=sample.invalidPointer.slice(0,-1);
  const target=selector.reduce((node:any,key:string)=>node[key],malformed);
  target[sample.invalidPointer.at(-1)]=sample.invalid;
  assert.deepEqual(annotationsOnly(previous),annotationsOnly(after));
  const observations=mutationInputAudit(previous,lookup(live(snapshotPath))).findings;
  assert(observations.every((f:any)=>f.code==="labelMissing"));
  assert.deepEqual(observations.map((f:any)=>f.pointer),sample.missing);
  assert(mutationInputDefs(after,lookup(staged(snapshotPath))).length>0);
  for(const candidate of [live,staged]){
    const validator=new Ajv({strict:false,allErrors:true});
    validator.addSchema(valueSchema);
    validator.addSchema(candidate(snapshotPath));
    const check=validator.compile(candidate(path));
    assert.equal(check(sample.payload),true,JSON.stringify(check.errors));
    assert.equal(check(malformed),false);
  }
  console.log("[DEBUG]",sample.mutation,"labels resolved in both locales, payload constraints preserved");
}
