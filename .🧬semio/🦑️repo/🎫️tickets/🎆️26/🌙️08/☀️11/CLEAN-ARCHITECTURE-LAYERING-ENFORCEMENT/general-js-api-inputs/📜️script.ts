import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert.ok(command&&["source","fixture","2d","3d","kernel","kernel-label","job"].includes(command));assert.ok(epoch&&/^\d+$/u.test(epoch));
const output=join(ticket,"🗑️generated/general-js-api",`${command}-${epoch}`);assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const custody=JSON.parse(readFileSync(join(import.meta.dir,"custody.json"),"utf8")) as {paths:string[]};
const paths=custody.paths.filter(path=>command==="source"||!path.startsWith(".vscode/")).map(path=>command==="source"&&epoch==="1"?path:path.replace("🧪️semio-tech-s-2d-js","🧪️semio-tech-framework-2d-js").replace("🧪️semio-tech-geometry-brep-js","🧪️semio-tech-framework-3d-js"));
paths.push("🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts","🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts","🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
const walk=(directory:string):void=>{for(const entry of readdirSync(join(root,directory),{withFileTypes:true})){if(entry.isSymbolicLink())continue;const path=join(directory,entry.name);if(entry.isDirectory()){if(!["node_modules","target","dist","pkg","🗑️generated","🤖️generated"].includes(entry.name))walk(path);}else if(/\.(?:tsx?|json)$/u.test(path)&&!paths.includes(path))paths.push(path);}};
for(const directory of ["🧰️framework/🔨️modules/◻️2d","🧰️framework/🔨️modules/🧊️3d","🧰️framework/🔨️modules/🎠️kernel","🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures"])walk(directory);
paths.push("🧰️framework/🔨️modules/🧵️job/🧪️tests/📏️close-demand/🟦️.ts","🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json","🧰️framework/🔨️modules/🧵️job/🦀️.rs");
paths.push("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts");
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return {path,source,sha256:createHash("sha256").update(source).digest("hex")};},producer=capture(import.meta.path),sources=paths.map(path=>capture(join(root,path))),controls=["schema.json","corpus.json","custody.json","🧪️tests/🟦️.ts","🧪️tests/🎚️fixtures/🟦️.ts"].map(path=>capture(join(import.meta.dir,path)));
const policy={maximumUnits:1000000000,maximumOwnedBytes:16777216,maximumCaptureBytes:67108864,budgetMs:1800000},controller=new AbortController(),stop=()=>controller.abort();
process.once("SIGINT",stop);process.once("SIGTERM",stop);let result:any,error:string|undefined,code=1;
const module=command==="2d"?"◻️2d":command==="3d"?"🧊️3d":command?.startsWith("kernel")?"🎠️kernel":"🧵️job",owner=join(root,`🧰️framework/🔨️modules/${module}/📦️packages/${command==="job"?"🦀️rust":"🟦️typescript"}`),environment={...process.env};
environment.SEMIO_TEST_ARTIFACT_DIR=output;environment.SEMIO_TEST_BUDGET_MS=String(policy.budgetMs);environment.SEMIO_VITEST_POLICY=JSON.stringify({version:1,cwd:owner,toolPath:join(root,"node_modules/vitest/vitest.mjs"),runtime:process.execPath,coverageRuntime:process.execPath,cacheRoot:join(output,"cache"),coverageDirectory:join(output,"coverage"),budgetMs:policy.budgetMs});
const argv=command==="source"||command==="fixture"?["test",join(import.meta.dir,command==="fixture"?"🧪️tests/🎚️fixtures/🟦️.ts":"🧪️tests/🟦️.ts")]:[join(owner,"📜️script.ts"),command==="job"?"test-close-demand-source":"test",command==="kernel-label"?"localized-label":"long"];
writeFileSync(join(output,"admission.json"),JSON.stringify({producer,sources,controls,policy,command,argv,environmentPolicy:environment.SEMIO_VITEST_POLICY,wholeDependencyClosureAccepted:false,atomicSourceSnapshotClaimed:false}));
try{
  const {MutationInventoryProcessWorkspace,runMutationInventoryProcess,readMutationInventoryCapture}=await import(join(root,"🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts"));
  let progressAt=0;const operation={signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace:new MutationInventoryProcessWorkspace(),onProgress:()=>{if(Date.now()-progressAt>10000){progressAt=Date.now();console.log(`[DEBUG] General JS ${command} progress`);}},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))};
  result=await runMutationInventoryProcess({command:process.execPath,argv,cwd:root,environment,captureDirectory:join(output,"captures"),budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes},operation);
  const stdout=await readMutationInventoryCapture(result.stdout,operation),stderr=await readMutationInventoryCapture(result.stderr,operation);writeFileSync(join(output,"stdout.txt"),stdout);writeFileSync(join(output,"stderr.txt"),stderr);code=result.reason==="exit"&&result.code===0?0:result.code||1;
}catch(failure){error=String(failure);}finally{
  const post=sources.map(value=>capture(value.path)),controlPost=controls.map(value=>capture(value.path)),producerPost=capture(import.meta.path),exactSelectedSources=sources.every((value,index)=>value.sha256===post[index].sha256),exactControls=controls.every((value,index)=>value.sha256===controlPost[index].sha256),exactProducer=producer.sha256===producerPost.sha256;
  writeFileSync(join(output,"terminal.json"),JSON.stringify({producer,sources,controls,result,post,controlPost,producerPost,exactSelectedSources,exactControls,exactProducer,code,error}));console.log(`[DEBUG] General JS ${command} terminal code=${code} selected=${exactSelectedSources} controls=${exactControls} producer=${exactProducer}`);process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=exactSelectedSources&&exactControls&&exactProducer?code:1;
}
