import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert.ok(command&&["source","neutral","native","cold","strict"].includes(command));assert.ok(epoch&&/^\d+$/u.test(epoch));
const output=join(ticket,"🗑️generated/general-command",`${command}-${epoch}`);assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const paths:string[]=[];
const walk=(directory:string):void=>{for(const entry of readdirSync(join(root,directory),{withFileTypes:true})){if(entry.isSymbolicLink())continue;const path=join(directory,entry.name);if(entry.isDirectory()){if(!["🛍️products","node_modules","target","dist","pkg","🗑️generated","🤖️generated",".git"].includes(entry.name))walk(path);}else if(/\.(?:tsx?|json|toml)$/u.test(path)&&!paths.includes(path))paths.push(path);}};
walk("🧰️framework");
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return {path,source,sha256:createHash("sha256").update(source).digest("hex")};},producer=capture(import.meta.path),sources=paths.map(path=>capture(join(root,path))),controls=["launch-additions.json","successor-launch-additions.json","final-launch-additions.json"].map(path=>capture(join(import.meta.dir,path)));
const policy={maximumUnits:1000000000,maximumOwnedBytes:16777216,maximumCaptureBytes:67108864,budgetMs:1800000},controller=new AbortController(),stop=()=>controller.abort();
process.once("SIGINT",stop);process.once("SIGTERM",stop);let result:any,error:string|undefined,code=1;const results:any[]=[];
const owner=join(root,"🧰️framework/🔨️modules/🏃️process"),environment={...process.env,SEMIO_TEST_ARTIFACT_DIR:output,SEMIO_COMMAND_STORE:join(output,"store")};
const argv=command==="strict"?[Bun.resolveSync("typescript/bin/tsc",root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...["🧭️routing/🎛️command/🟦️.ts","🧭️routing/🎛️command/⚙️configuration/🟦️.ts","📦️artifacts/🏗️native-build/🌊️relay/🟦️.ts","🧭️routing/🎛️command/🧪️tests/🟦️.ts","🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts","🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts","📜️script.ts"].map(path=>join(owner,path)),join(root,"🧰️framework/📦️packages/🦀️rust/📜️script.ts")]:command==="source"?["test",join(owner,"🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts")]:command==="neutral"?["test",join(owner,"🧭️routing/🎛️command/🧪️tests/🟦️.ts")]:command==="native"?[join(owner,"📜️script.ts"),"test","native-artifacts"]:["test",join(owner,"🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts")];
const configured=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8")),jobs=[{argv,environment}];
if(command==="source"&&epoch==="5")for(const [name,target]of Object.entries(configured.targets) as [string,any][]){if(!target.options?.command.includes("📜️script.ts command --config"))continue;jobs.push({argv:[join(root,"node_modules/nx/dist/bin/nx.js"),"run",`${configured.name}:${name}`,"--skip-nx-cache"],environment:{...environment,NX_WORKSPACE_ROOT_PATH:root,NX_WORKSPACE_DATA_DIRECTORY:join(output,"configured-nx-data"),NX_CACHE_DIRECTORY:join(output,"configured-nx-cache"),NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false"}});}
writeFileSync(join(output,"admission.json"),JSON.stringify({producer,sources,controls,policy,command,argv,jobs,environmentPolicy:environment.SEMIO_COMMAND_STORE,wholeDependencyClosureAccepted:false,atomicSourceSnapshotClaimed:false}));
try{
  const {MutationInventoryProcessWorkspace,runMutationInventoryProcess,readMutationInventoryCapture}=await import(join(root,"🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts"));
  code=0;let progressAt=0;
  for(const [index,job]of jobs.entries()){
   const operation={signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace:new MutationInventoryProcessWorkspace(),onProgress:()=>{if(Date.now()-progressAt>10000){progressAt=Date.now();console.log(`[DEBUG] General command ${command} job=${index} progress`);}},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))};
   result=await runMutationInventoryProcess({command:process.execPath,argv:job.argv,cwd:root,environment:job.environment,captureDirectory:join(output,"captures",String(index)),budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes},operation);results.push(result);
   const stdout=await readMutationInventoryCapture(result.stdout,operation),stderr=await readMutationInventoryCapture(result.stderr,operation);writeFileSync(join(output,index===0?"stdout.txt":`stdout-${index}.txt`),stdout);writeFileSync(join(output,index===0?"stderr.txt":`stderr-${index}.txt`),stderr);if(result.reason!=="exit"||result.code!==0){code=result.code||1;break;}
  }
}catch(failure){error=String(failure);code=1;}finally{
  const post=sources.map(value=>capture(value.path)),controlPost=controls.map(value=>capture(value.path)),producerPost=capture(import.meta.path),exactSelectedSources=sources.every((value,index)=>value.sha256===post[index].sha256),exactControls=controls.every((value,index)=>value.sha256===controlPost[index].sha256),exactProducer=producer.sha256===producerPost.sha256;
  writeFileSync(join(output,"terminal.json"),JSON.stringify({producer,sources,controls,result,results,post,controlPost,producerPost,exactSelectedSources,exactControls,exactProducer,code,error}));console.log(`[DEBUG] General command ${command} terminal code=${code} selected=${exactSelectedSources} controls=${exactControls} producer=${exactProducer}`);process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=exactSelectedSources&&exactControls&&exactProducer?code:1;
}
