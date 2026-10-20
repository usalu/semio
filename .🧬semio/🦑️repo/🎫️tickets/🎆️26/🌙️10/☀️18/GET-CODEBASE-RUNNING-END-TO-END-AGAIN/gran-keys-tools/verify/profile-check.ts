import {readFileSync} from "node:fs";
const SB="/private/tmp/gk/sb", L=SB+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", REAL="/Users/ueli/Documents/semio";
const lock:any=await import(L+"/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs");
const read:any=await import(L+"/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs");
const engine:any=await import(L+"/🏗️build/📥️inputs/🦀️resolution/🟦️.ts");
const routing:any=await import(SB+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const caller:any=await import(L+"/🏗️build/📥️caller/🟦️.ts");
const caps=caller.createNativeBuildCallerCapabilitiesV1(SB,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"});
const run=<T>(fn:(c:any)=>Promise<T>)=>routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"gk:prof",maximumElapsedMilliseconds:900000},caps,Date.now()),(o:any)=>fn(routing.createIssuedResourceControlV1(o,"source:prof",900000)));
const all=JSON.parse(readFileSync("/private/tmp/gk/authored.json","utf8"));
const keys=Object.keys(all).filter(k=>{if(!k||k===".")return false;try{readFileSync(REAL+"/"+k+"/Cargo.toml");return true}catch{return false}}).slice(0,400).map(k=>k+"/Cargo.toml");
const policy=engine.readCargoResolutionPolicyV1(JSON.parse(readFileSync(L+"/🏗️build/📥️inputs/🦀️resolution/🔣️.json","utf8")));
await run(async c=>{
  await read.withExactSourceReadBatchV1(REAL,c,async(batch:any)=>{
    let t=performance.now(),n=0;
    for(const m of keys){try{await lock.cargoResolutionDigestInBatchV1 ? 0:0; await (await import(L+"/🏗️build/📥️inputs/🔒️lock/📥️read/🟨️.mjs")).selectedCargoLockDigestInBatchV1(batch,REAL,m);}catch{} n++;}
    console.log("lock-only 400:",((performance.now()-t)/1000).toFixed(1),"s");
  });
  await read.withExactSourceReadBatchV1(REAL,c,async(batch:any)=>{
    let reads=0,rt=0;
    const ctx=engine.createCargoResolutionContextV1({read:async(p:string)=>{const t=performance.now();const v=await read.readExactSourceTomlV1(batch,p);rt+=performance.now()-t;reads++;return v?.value??null;}},policy);
    const t=performance.now();
    for(const m of keys){const d=m.slice(0,-"/Cargo.toml".length);await engine.crateIdentityV1(ctx,d,true);}
    console.log("engine-only 400:",((performance.now()-t)/1000).toFixed(1),"s; manifest reads",reads,"read time",(rt/1000).toFixed(1),"s");
    // digest-only speed
    const files=[...ctx.manifests.keys()].slice(0,500);let dt=performance.now();
    await read.withExactSourceReadBatchV1(REAL,c,async(b2:any)=>{for(const f of files){await read.readExactSourceFileV1(b2,f);}});
    console.log("digest 500 files via batch:",((performance.now()-dt)/1000).toFixed(1),"s");
  });
});
