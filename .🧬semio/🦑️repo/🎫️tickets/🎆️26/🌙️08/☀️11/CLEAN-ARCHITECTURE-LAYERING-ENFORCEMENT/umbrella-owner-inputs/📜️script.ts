import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert.equal(command,"source");assert.ok(epoch&&/^\d+$/.test(epoch));
const output=join(ticket,"🗑️generated/umbrella-owner",`${command}-${epoch}`);assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const paths=["🧰️framework/🧪️tests/🧱️ownership/🟦️.ts","🧰️framework/🧪️tests/🧱️ownership/🧫️fixtures/🔣️.json","🧰️framework/📦️packages/🦀️rust/Cargo.toml","🧰️framework/📦️packages/🦀️rust/🦀️.rs","🧰️framework/🔨️modules/🎠️kernel/🦀️.rs","🧰️framework/Cargo.toml","Cargo.toml"];
const workspacePath=existsSync(join(root,"🧰️framework/Cargo.toml"))?join(root,"🧰️framework/Cargo.toml"):join(root,"Cargo.toml"),workspaceOwner=dirname(workspacePath),workspace=Bun.TOML.parse(readFileSync(workspacePath,"utf8")).workspace as {members:string[]};
for(const member of workspace.members){const owner=resolve(workspaceOwner,member);if(owner.startsWith(join(root,"🧰️framework/"))&&!owner.includes("/🛍️products/"))paths.push(join(owner,"Cargo.toml"));}
const capture=(path:string)=>{const source=existsSync(path)?readFileSync(path,"utf8"):null;return{path,source,sha256:source===null?null:createHash("sha256").update(source).digest("hex")};},producer=capture(import.meta.path),sources=[...new Set(paths.map(path=>resolve(root,path)))].map(capture),policy={maximumUnits:1000000000,maximumOwnedBytes:16777216,maximumCaptureBytes:67108864,budgetMs:600000};
writeFileSync(join(output,"admission.json"),JSON.stringify({producer,sources,policy,scope:"General umbrella defining ownership",wholeDependencyClosureAccepted:false,atomicSourceSnapshotClaimed:false}));
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
