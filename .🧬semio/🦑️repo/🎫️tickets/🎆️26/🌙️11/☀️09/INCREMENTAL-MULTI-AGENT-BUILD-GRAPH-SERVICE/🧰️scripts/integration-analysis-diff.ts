import{join,resolve}from"node:path";
import{readFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
const G=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"),O=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const producer=await import(join(G,"📥️inference/🟨️.mjs"));await producer.libraryBootstrap;const providers=producer.prepareSourceAnalysesV1();
const obs=await import(O);
console.log(Object.keys(providers));
for(const rel of process.argv.slice(2)){
 const file=resolve(rel),now=readFileSync(file),head=spawnSync("git",["show","HEAD:"+rel],{encoding:"buffer",maxBuffer:1<<28}).stdout;
 for(const[name,provider]of Object.entries(providers)){
  let a,b;try{const run=(bytes:Buffer)=>{ return obs.observedSourceAnalysisV1&&undefined;};}catch{}
  void a;void b;
 }
 console.log(rel,"now",now.length,"head",head.length);
}
