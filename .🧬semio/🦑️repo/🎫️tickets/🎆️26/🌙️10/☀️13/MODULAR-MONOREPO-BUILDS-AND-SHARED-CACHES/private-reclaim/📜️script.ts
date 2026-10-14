import {lstat,readdir,unlink,rmdir} from "node:fs/promises";
import {existsSync,readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {acquireQueuedResourceLease} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {acquireCargoProfileMaintenanceLeaseV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
const secondary=process.argv[2]==="secondary",tertiary=process.argv[2]==="tertiary";
if(process.argv[2]&&!secondary&&!tertiary)throw Error("Select secondary or the original inactive private output plan");
const workspace=resolve(import.meta.dir,"../../../../../../../.."),cache=join(workspace,".🧬semio/🦑️repo/⚡️cache"),roots=(tertiary?["e1b"]:secondary?["e4","tc","ort"]:["l1","at"]).map(name=>join(cache,name));
const stores=(tertiary?["e1b/b","e1b/t"]:secondary?["e4/b","e4/t","e4/head-b","e4/head-t","tc/build","tc/target",...(["after","before","check","test"].flatMap(lane=>[`ort/${lane}/build`,`ort/${lane}/target`]))]:["l1/build","l1/target","l1/fgUvgd0/b","l1/fgUvgd0/t","at/b","at/t"]).map(name=>join(cache,name));
const outputs=(tertiary?["e1b/b/debug","e1b/b/wasm32-unknown-unknown","e1b/b/wasm32-wasip2"]:secondary?["e4/b/debug","e4/b/wasm-release","e4/b/wasm32-wasip2","e4/t/debug","e4/t/wasm-release","e4/t/wasm32-wasip2","e4/head-b/debug","e4/head-t/debug","tc/build/debug","tc/target/debug","ort/after/build/host-tool","ort/after/target/host-tool","ort/before/build/host-tool","ort/before/target/host-tool","ort/check/build/debug","ort/check/target/debug","ort/test/build/debug","ort/test/target/debug"]:["l1/build/debug","l1/build/wasm32-wasip2","l1/target/wasm32-unknown-unknown","l1/fgUvgd0/b/release","l1/fgUvgd0/b/wasm-release","l1/fgUvgd0/b/wasm32-wasip2","l1/fgUvgd0/t/release","l1/fgUvgd0/t/wasm-release","l1/fgUvgd0/t/wasm32-wasip2","at/b/debug"]).map(name=>join(cache,name));
await receiveScriptProcessInvocation(process.env,async invocation=>{
 const control=scriptCompilerAdmissionControlV1(invocation,"cache:inactive-private-outputs",600000),leases=join(workspace,".cache/framework/leases"),held=[] as {release():void}[];
 try{
  if(!secondary&&!tertiary&&!readFileSync(join(roots[0]!,"fgUvgd0/r/report.md"),"utf8").includes("| wasm-release-00 | wasm-release | wasm32-wasip2 | 13.5 | 554.4 | 1 |"))throw Error("Completed private generation receipt refused");
  if(!secondary&&!tertiary&&!readFileSync(join(roots[1]!,"results.jsonl"),"utf8").trim().endsWith('"secs": 182}'))throw Error("Private test result receipt refused");
  if(tertiary&&!readFileSync(join(workspace,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/e1b-kernel-seams/🏗️cargo.sh"),"utf8").includes("CARGO_BUILD_BUILD_DIR"))throw Error("Private kernel compiler input custody refused");
  if(secondary)for(const input of ["e4-3d-architect-bim","trusted-catalog-receipts","owned-runtime-crate"]){const retained=join(workspace,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT",input,"🏗️cargo.sh");if(!readFileSync(retained,"utf8").includes("CARGO_TARGET_DIR"))throw Error("Private compiler input custody refused");}
  for(const root of [...roots,...stores,...outputs])if(existsSync(root)&&((await lstat(root)).isSymbolicLink()||Date.now()-(await lstat(root)).mtimeMs<3600000))throw Error("Private output age/identity refused");
  held.push(await acquireQueuedResourceLease({directory:leases,resource:"cache-prune",mode:"exclusive",signal:control.signal,control}));
  for(const store of stores)for(const profile of ["debug","release","prototype","wasm-prototype","wasm-release","host-tool"])held.push(await acquireCargoProfileMaintenanceLeaseV1({directory:leases,buildDirectory:store,args:["--profile",profile],signal:control.signal,control}));
  const verify=()=>{compilerAdmissionBudgetV1(control);const processes=spawnSync("ps",["eww","-axo","pid,command"],{encoding:"utf8"});if(processes.status!==0||processes.stdout.split("\n").some(line=>roots.some(root=>line.includes(root))))throw Error("Private output has a live process/environment reference");};
  verify();
  for(const root of roots){const opened=spawnSync("lsof",["-nP","+D",root],{encoding:"utf8"});if(opened.status!==1||opened.stdout.trim())throw Error("Private output has open descriptors or inspection failed");}
  verify();let files=0,bytes=0,last=0;
  const remove=async(path:string):Promise<void>=>{compilerAdmissionBudgetV1(control);const entry=await lstat(path);if(entry.isDirectory()&&!entry.isSymbolicLink()){for(const name of await readdir(path))await remove(join(path,name));await rmdir(path);}else{bytes+=entry.size;await unlink(path);files++;}if(Date.now()-last>1000){last=Date.now();verify();console.log(`[DEBUG] Reclaimed inactive private outputs files=${files} bytes=${bytes}`);await control.publish({resource:"inactive-private-outputs",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}};
  for(const output of outputs)if(existsSync(output))await remove(output);
  console.log(`[DEBUG] Private output reclamation completed=${JSON.stringify({files,bytes,outputs,retained:["input scripts","configs","reports","logs","provenance","telemetry","compiler resource records"],liveReferences:0,openDescriptors:0,lost:"optional private warm resume intermediates"})}`);
 }finally{for(const lease of held.reverse())lease.release();}
});
