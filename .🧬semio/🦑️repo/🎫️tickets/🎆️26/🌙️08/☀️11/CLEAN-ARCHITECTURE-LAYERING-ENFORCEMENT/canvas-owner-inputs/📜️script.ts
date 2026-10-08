import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert.equal(command,"source");assert.ok(epoch&&/^\d+$/.test(epoch));
const output=join(ticket,"🗑️generated/canvas-owner",`${command}-${epoch}`);assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const paths=["🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🧱️ownership/🟦️.ts","🧰️framework/🔨️modules/🖼️canvas/🧬️schema/🎬️draw-list/🔣️.json","Cargo.toml","🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/🖼️canvas/🧬️schema/🎨️theme-color-fields/🔣️.json","🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/🎨️theme-color-fields/🔣️.json","🧰️framework/🔨️modules/🖼️canvas/🧪️tests/🎨️theme-color-fields/🦀️.rs","🧰️framework/🔨️modules/🖼️canvas/🦀️.rs","🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🔨️modules/🖼️canvas/🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json","🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts","🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs"];
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return{path,source,sha256:createHash("sha256").update(source).digest("hex")};},producer=capture(import.meta.path),sources=paths.map(path=>capture(join(root,path))),policy={maximumUnits:1000000000,maximumOwnedBytes:16777216,maximumCaptureBytes:67108864,budgetMs:600000};
writeFileSync(join(output,"admission.json"),JSON.stringify({producer,sources,policy,scope:"Canvas defining ownership, variable command contract and first-party theme fields with neutral schema/oracle rows",wholeDependencyClosureAccepted:false,atomicSourceSnapshotClaimed:false}));
const controller=new AbortController(),stop=()=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);let result:any,error:string|undefined,code=1;
try{
 const{MutationInventoryProcessWorkspace,runMutationInventoryProcess,readMutationInventoryCapture}=await import(join(root,"🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts"));
 const operation={signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace:new MutationInventoryProcessWorkspace(),onProgress:()=>{},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))};
 result=await runMutationInventoryProcess({command:process.execPath,argv:["test",join(root,paths[0])],cwd:root,environment:process.env,captureDirectory:join(output,"captures"),budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes},operation);
 const stdout=await readMutationInventoryCapture(result.stdout,operation),stderr=await readMutationInventoryCapture(result.stderr,operation);writeFileSync(join(output,"stdout.txt"),stdout);writeFileSync(join(output,"stderr.txt"),stderr);code=result.reason==="exit"&&result.code===0?0:result.code||1;
}catch(failure){error=String(failure);}finally{
 const post=sources.map(value=>capture(value.path)),producerPost=capture(import.meta.path),exactSelectedSources=sources.every((value,index)=>value.sha256===post[index].sha256),exactProducer=producer.sha256===producerPost.sha256;
 writeFileSync(join(output,"terminal.json"),JSON.stringify({producer,sources,result,post,producerPost,exactSelectedSources,exactProducer,code,error}));
 process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=exactSelectedSources&&exactProducer?code:1;
}
