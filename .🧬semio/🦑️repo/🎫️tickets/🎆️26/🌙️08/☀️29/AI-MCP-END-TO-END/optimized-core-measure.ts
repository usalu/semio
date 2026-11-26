import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, mkdtemp, readFile, unlink, writeFile } from "node:fs/promises";
import { basename, join, resolve } from "node:path";
import { pluginWasmOptToolV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { actorPreparationImageIdentityV1, actorPreparationPolicyV1, assertActorPreparationBindingV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🌱️actor-preparation/🟦️.ts";

const [sourceModel, expected, emitter] = process.argv.slice(2);
const generated = join(import.meta.dir,"🗑️generated");
const run = await mkdtemp(join(generated,"core-opt-"));
const component = await readFile(join(sourceModel,"📦️component.wasm"));
const digest = (bytes:Uint8Array) => createHash("sha256").update(bytes).digest("hex");
assert.equal(digest(component),expected);
const sections = (buffer:Buffer) => {
 const rows:{id:number,start:number,payload:number,end:number}[]=[];
 for(let start=8;start<buffer.length;){
  let cursor=start+1,length=0,complete=false;
  for(let shift=0;shift<=28 && cursor<buffer.length;shift+=7){
   const byte=buffer[cursor++];
   length+=(byte&127)*2**shift;
   if(byte<128){complete=true;break;}
  }
  const end=cursor+length;
  assert(complete && end<=buffer.length);
  rows.push({id:buffer[start],start,payload:cursor,end});
  start=end;
 }
 return rows;
};
function header(size:number){
 const bytes=[1];
 while(size>=128){bytes.push((size%128)|128);size=Math.floor(size/128);}
 return Buffer.from([...bytes,size]);
}
assert.equal(component.subarray(0,8).toString("hex"),"0061736d0d000100");
const actor = sections(component).filter(section => {
 if(section.id!==1)return false;
 const module=component.subarray(section.payload,section.end);
 return sections(module).some(row => row.id===7 && module.subarray(row.payload,row.end).includes("semio_owned_poll_v1"));
});
assert.equal(actor.length,1);
const [core]=actor;
const raw=join(run,"core.wasm"),opt=join(run,"optimized.wasm");
await writeFile(raw,component.subarray(core.payload,core.end));
const tool=pluginWasmOptToolV1({repoRoot:process.cwd()});
async function invoke(command:string,args:string[]){
 const child=Bun.spawn([command,...args],{env:{...process.env,BINARYEN_CORES:"1"},stdout:"inherit",stderr:"inherit"});
 assert.equal(await child.exited,0,command);
}
console.log("[DEBUG] optimizing core",core.end-core.payload,run);
await invoke(tool.command,[raw,...tool.args,"-o",opt]);
await unlink(raw);
const optimized=await readFile(opt);
assert.equal(optimized.subarray(0,8).toString("hex"),"0061736d01000000");
const module=Buffer.concat([component.subarray(0,core.start),header(optimized.length),optimized,component.subarray(core.end)]);
const original=sections(component), replacement=sections(module);
assert.equal(original.length,replacement.length);
for(const [i,s] of original.entries()){
 const r=replacement[i];assert.equal(s.id,r.id);
 if(s!==core && s.start!==core.start)assert.deepEqual(module.subarray(r.start,r.end),component.subarray(s.start,s.end));
}
const model=join(run,"browser","prototype","components",basename(resolve(sourceModel)));
await mkdir(model,{recursive:true});
const destination=join(model,"📦️component.wasm");
await writeFile(destination,module);
await unlink(opt);
console.log("[DEBUG] optimized component",module.length,digest(module),destination);
await invoke(resolve(emitter),["describe",destination,"--out",model,"--prepare-actor"]);
const pair=async(path:string)=>verifySemanticDescriptorPairBytesV1(await readFile(join(path,"🔣️.json")),await readFile(join(path,"🛂️.descriptor.semio")));
const [before,after]=await Promise.all([pair(sourceModel),pair(model)]);
assert.equal(after.semanticSha256,before.semanticSha256);
const image=join(model,actorPreparationPolicyV1.image);
assertActorPreparationBindingV1(new Uint8Array(await Bun.file(image).slice(0,4096).arrayBuffer()),digest(module));
const prepared=await actorPreparationImageIdentityV1(image,AbortSignal.timeout(30000));
console.log("[DEBUG] verified optimized model",JSON.stringify({root:run,model,descriptor:after.semanticSha256,component:digest(module),prepared}));
