import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { startVitest } from "vitest/node";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir)),output=join(ticket,"🗑️generated/testing-taxonomy/renderer");
const engine="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine",owner=engine+"/🧱️elements/📃️UiDocumentStore/📥️intake",config=engine+"/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts";
const command=process.argv[2],report=join(ticket,"📓️renderer-testing-taxonomy-2026-09-12.md");
mkdirSync(output,{recursive:true});process.env.SEMIO_TEST_LEVEL="long";
const suite=command==="corpus"?engine+"/🧱️elements/🗣️Interpreter/🟦️.tsx":command==="baseline"?owner+"/🟦️.ts":owner+"/🧪️tests/📏️step-ceiling/🟦️.ts";
const context=await startVitest("test",[join(root,suite)],{config:join(root,config),run:true,maxWorkers:1,fileParallelism:false,testNamePattern:command==="corpus"?"conformance corpus":"paged scene-lane intake ceiling"},{cacheDir:join(output,"vite-cache")});
try{
 const tasks=(nodes:any[]):any[]=>nodes.flatMap(node=>node.type==="test"?[node]:tasks(node.tasks??[]));
 const tests=tasks(context.state.getFiles()),counts={passed:tests.filter(t=>t.result?.state==="pass").length,failed:tests.filter(t=>t.result?.state==="fail").length,skipped:tests.filter(t=>!t.result||t.result.state==="skip").length};
 const receipt={command,suite,counts,files:context.state.getFiles().map((f:any)=>({path:f.filepath,state:f.result?.state})),errors:context.state.getUnhandledErrors().map(String)};
 let previous="";try{previous=readFileSync(report,"utf8")}catch{}
 writeFileSync(report,previous+"\n## "+command+" Runtime\n\n```json\n"+JSON.stringify(receipt,null,2)+"\n```\n");
 assert.equal(counts.failed,0);assert.equal(receipt.errors.length,0);assert.equal(counts.passed,command==="corpus"?63:3);assert(receipt.files.every((f:any)=>f.state==="pass"));console.log("[DEBUG] "+command+" renderer taxonomy runtime "+JSON.stringify(counts));
}finally{await context.close();}

