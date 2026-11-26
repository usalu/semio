import { strict as assert } from "node:assert";
import { resolve } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";

const workspace=resolve(import.meta.dirname,"../../../../../../..");
const framework=resolve(workspace,"🧰️framework");
const load=(path:string)=>import(pathToFileURL(resolve(framework,path)).href);
const routing=await load("🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const execution=await load("🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts");
const publication=await load("🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts");
const capacity=await load("🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts");
const store=await load("🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/💾️storage/🟦️.ts");
const machine=await load("🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts");
const stamp=await load("🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs");
const base="🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛠️emitter";
const emitter=await load(base+"/🟦️.ts");
const handoff=await load(base+"/🤝️release-handoff/🟦️.ts");
const contract=await load(base+"/📋️contract/🟦️.ts");
const restart=stamp.formatStampProtocolV1().exitCode;

await routing.receiveScriptProcessInvocation(process.env,async invocation=>{
 const control=capacity.scriptCompilerAdmissionControlV1(invocation,"test:descriptor-handoff",0);
 if(process.argv[2]==="producer"){
  const request=handoff.receivedDescriptorEmitterReleaseRequestV1(process.env,invocation);
  assert.ok(request);
  await emitter.withDescriptorEmitterV1(workspace,invocation,control,tool=>emitter.retainDescriptorEmitterReleaseReplyV1(tool,invocation,request));
  process.exitCode=restart;
  return;
 }

 const session=handoff.createDescriptorEmitterReleaseJobV1(invocation,"describe-tool");
 const products=store.repositoryCachePartitionRootDataV1(workspace,"products");
 const verify=async request=>{
  if(!await emitter.descriptorEmitterReleaseReplyExistsV1(workspace,invocation,control,request))return false;
  return emitter.withDescriptorEmitterV1(workspace,invocation,control,tool=>
   emitter.withDescriptorEmitterTaskV1(tool,invocation,async actual=>{
    assert.equal(createHash("sha256").update(await readFile(actual.path)).digest("hex"),actual.sha256);
    const stdout=await execution.captureOwnedCommandV1(actual.path,["capabilities"],workspace,{control:actual.control,output:{maximumBytes:4096,maximumLines:20}});
    contract.requireDescriptorEmitterCapabilitiesV1(stdout);
    return true;
   }),request);
 };
 try{
  const unsealed=session.attempt(control,verify);
  assert.equal(await unsealed.settle(restart),restart);
  assert.throws(()=>session.selected());
  const attempt=session.attempt(control,verify);
  const selected=JSON.parse(attempt.environment[handoff.descriptorEmitterReleaseHandoffPolicyV1.environment]);
  let exit:number|undefined;
  try{
   await execution.runOwnedCommand("repo",[fileURLToPath(import.meta.url),"producer"],workspace,"descriptor-emitter-authentic-restart",0,
    {control,signal:control.signal,env:routing.scriptResourceProcessEnvironmentV1(control,{...process.env,...attempt.environment}),output:{maximumBytes:8388608,maximumLines:100000}});
  }catch(error){
   if(!(error instanceof execution.OwnedCommandFailure))throw error;
   exit=error.outcome.status;
  }
  assert.equal(exit,restart);
  assert.equal(await attempt.settle(exit),0);
  assert.deepEqual(session.selected(),selected);
  assert.equal(await verify(session.selected()),true);
  const another=handoff.createDescriptorEmitterReleaseJobV1(invocation,"describe-tool");
  assert.equal(await another.attempt(control,verify).settle(restart),restart);
  assert.throws(()=>another.selected());
  console.log("Verified an actual compiler executable, retained receipt/index, sealed reply after child restart, and refusal of a new request");
 }finally{
  const paths=session.requests().map(request=>handoff.descriptorEmitterReleaseArtifactRequestV1(products,request).path);
  const retired=await publication.retireArtifactDirectoriesV1(paths,{leaseDirectory:machine.machinePathV1(workspace,".cache/b/l/m/data"),signal:control.signal});
  assert.deepEqual(retired.held,[]);
 }
});
