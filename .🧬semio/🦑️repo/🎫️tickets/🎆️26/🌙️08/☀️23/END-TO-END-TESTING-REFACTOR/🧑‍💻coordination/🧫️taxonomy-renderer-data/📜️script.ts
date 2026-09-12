import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { startVitest } from "vitest/node";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/renderer-data"),phase=process.argv[2],report=join(ticket,"📓️renderer-data-taxonomy-2026-09-12.md");
mkdirSync(output,{recursive:true});const temp=join(output,phase,"temp");mkdirSync(temp,{recursive:true});process.env.TMPDIR=temp;process.env.TMP=temp;process.env.TEMP=temp;
const append=(value:unknown)=>{let prior="";try{prior=readFileSync(report,"utf8")}catch{}writeFileSync(report,prior+"\n## "+phase+" Runtime\n\n```json\n"+JSON.stringify(value,null,2)+"\n```\n");};
if(phase==="node-oracle"){
 const fixture=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔢️wgpu-u64-seam/🔣️.json"),code='const assert=require("node:assert/strict"),fs=require("node:fs"),laws=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));assert.equal(TypeError.name,laws.numberLoweringFaultType);assert.throws(()=>BigInt.asUintN(64,laws.firstBatch.generation),TypeError);assert.equal(BigInt.asUintN(64,BigInt(laws.firstBatch.generation)),BigInt(laws.firstBatch.generation));console.log("[DEBUG] Node V8 independently confirmed the u64 error class and successful BigInt lowering");';
 const result=Bun.spawnSync(["node","-e",code,fixture],{cwd:root,stdout:"pipe",stderr:"pipe",timeout:10000});append({exitCode:result.exitCode,stdout:result.stdout.toString(),stderr:result.stderr.toString()});assert.equal(result.exitCode,0,result.stderr.toString());console.log(result.stdout.toString());process.exit(0);
}
await import(pathToFileURL(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js")).href);append({flowDrawList:"pass"});
const engine=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"),names=["🌳️wgpu-document-reconcile","🔢️wgpu-u64-seam","🔬️wgpu-extension-dispatch","🖌️wgpu-document-owner-move","🧩️wgpu-module-routes"];
const context=await startVitest("test",names.map(name=>join(engine,"🧪️tests",name,"🟦️.ts")),{config:join(engine,"🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts"),root:engine,run:true,maxWorkers:1,fileParallelism:false},{cacheDir:join(output,"vite-cache")});
try{
 const tasks=(nodes:any[]):any[]=>nodes.flatMap(n=>n.type==="test"?[n]:tasks(n.tasks??[])),tests=tasks(context.state.getFiles()),receipt={passed:tests.filter(t=>t.result?.state==="pass").length,failed:tests.filter(t=>t.result?.state==="fail").length,files:context.state.getFiles().map((f:any)=>({path:f.filepath,state:f.result?.state,errors:f.result?.errors?.map((e:any)=>e.message)})),errors:context.state.getUnhandledErrors().map(String)};
 append(receipt);assert.equal(receipt.failed,0);assert.equal(receipt.errors.length,0);assert.equal(receipt.files.length,5);assert(receipt.files.every(f=>f.state==="pass"));assert(receipt.passed>0);console.log("[DEBUG] renderer data "+phase+" passed="+receipt.passed);
}finally{await context.close();}
