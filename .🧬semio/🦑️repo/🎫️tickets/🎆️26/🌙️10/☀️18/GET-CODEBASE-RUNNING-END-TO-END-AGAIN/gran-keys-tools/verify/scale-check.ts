import {readFileSync} from "node:fs";
const SB="/private/tmp/gk/sb", L=SB+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", REAL="/Users/ueli/Documents/semio";
const lock:any=await import(L+"/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs");
const routing:any=await import(SB+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const caller:any=await import(L+"/🏗️build/📥️caller/🟦️.ts");
const caps=caller.createNativeBuildCallerCapabilitiesV1(SB,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"});
const run=<T>(fn:(c:any)=>Promise<T>)=>routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"gk:scale",maximumElapsedMilliseconds:900000},caps,Date.now()),(o:any)=>fn(routing.createIssuedResourceControlV1(o,"source:scale",900000)));
const all=JSON.parse(readFileSync("/private/tmp/gk/authored.json","utf8"));
const keys=Object.keys(all).filter(k=>{if(!k||k===".")return false;try{readFileSync(REAL+"/"+k+"/Cargo.toml");return true}catch{return false}});
for(const n of [1,10,100,400]){const sel=keys.slice(0,n).map(k=>k+"/Cargo.toml");const t=performance.now();const m=await run(c=>lock.selectedCargoResolutionDigests(REAL,sel,c)) as Map<string,string>;console.log(n,"crates",((performance.now()-t)/1000).toFixed(1),"s",m.size);}
