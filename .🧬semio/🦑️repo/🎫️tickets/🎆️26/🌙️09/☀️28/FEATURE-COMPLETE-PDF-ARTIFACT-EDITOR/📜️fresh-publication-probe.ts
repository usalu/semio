/** 🗝️ Replays timed directory contention and a genuinely held target against the production publisher. */
import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join,resolve} from "node:path";
import {type ResourceLeaseControl} from "@semio-tech/process/leases";
const workspace=process.cwd(), repository="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const load=(part:string)=>import(resolve(part));
const [{createNativeBuildCallerCapabilitiesV1},{issueRepositoryScriptProvidersV1},{createScriptProcessEnvelope,receiveScriptProcessInvocation,scriptProcessEnvironment,createIssuedResourceControlV1},{withFreshArtifactPublicationWritersV1},{acquireResourceLease,resourceLeasePathV1,withLeaseTaskV1,ResourceLeaseTimeoutErrorV1}]=await Promise.all([
  load(repository+"/🏗️build/📥️caller/🟦️.ts"),load(repository+"/🏃️process/🔌️providers/🟦️.ts"),
  import("@semio-tech/process/routing"), load("🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts"), import("@semio-tech/process/leases"),
]);
const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🔣️fresh-publication-admission.json"),"utf8"));
process.env.SEMIO_BUILD_MODE="prototype";
issueRepositoryScriptProvidersV1(process.env);
const cap=createNativeBuildCallerCapabilitiesV1(workspace,["run","workspace:components","--exclude-dependencies","--configuration=prototype"],"nx",process.env);
const envelope=createScriptProcessEnvelope({version:1,owner:"pdf:fresh-publication",maximumElapsedMilliseconds:25000},cap,Date.now());
Object.assign(process.env,scriptProcessEnvironment(envelope,process.env));
const root=join(import.meta.dir,"🗑️generated","fl"), directory=join(root,"l");
mkdirSync(directory,{recursive:true});
const source=join(root,"in"), destination=join(root,"out");
writeFileSync(source,fixture.expected.text);
await receiveScriptProcessInvocation(process.env,async original=>{
  const abort=new AbortController();
  const control=createIssuedResourceControlV1(original,"pdf:test-fresh-publication",fixture.beforeDeadlineMilliseconds,abort.signal) as ResourceLeaseControl;
  const seed=await acquireResourceLease({directory,resource:"fresh-seed",mode:"exclusive",control,signal:control.signal,timeoutMs:1000});
  seed.release();
  const gate=new Database(resourceLeasePathV1(directory,"resource-directory-admission:v1"));
  gate.exec("pragma busy_timeout=0");
  gate.exec("begin exclusive");
  const release=setTimeout(()=>gate.exec("rollback"),fixture.releaseDirectoryGateAfterMilliseconds);
  const started=performance.now();
  try{
    await withLeaseTaskV1(()=>withFreshArtifactPublicationWritersV1(destination,directory,control,(stage:(owner:string,files:ReadonlyMap<string,string>,options:unknown)=>Promise<void>)=>
      stage(fixture.expected.owner,new Map([[fixture.expected.file,source]]),{leaseDirectory:directory,control,signal:control.signal})));
  }finally{
    clearTimeout(release);gate.close();
  }
  assert.ok(performance.now()-started>=fixture.releaseDirectoryGateAfterMilliseconds-100);
  assert.equal(readFileSync(join(destination,fixture.expected.file),"utf8"),fixture.expected.text);
  const resource="artifact:"+destination;
  const actualTarget=new Database(resourceLeasePathV1(directory,resource));
  actualTarget.exec("pragma busy_timeout=0");
  actualTarget.exec("begin exclusive");
  const collisionStarted=performance.now();
  let called=false;
  try{
    await assert.rejects(()=>withLeaseTaskV1(()=>withFreshArtifactPublicationWritersV1(destination,directory,control,async()=>{called=true;})),(error:unknown)=>error instanceof ResourceLeaseTimeoutErrorV1&&error.resource===resource&&error.cause===undefined);
  }finally{
    actualTarget.exec("rollback");actualTarget.close();
  }
  assert.equal(called,false);
  assert.ok(performance.now()-collisionStarted<fixture.collisionMaximumMilliseconds);
  const stopped=new AbortController();
  const cancellable=createIssuedResourceControlV1(original,"pdf:test-archive-cancel",fixture.beforeDeadlineMilliseconds,stopped.signal) as ResourceLeaseControl;
  const cancelGate=new Database(resourceLeasePathV1(directory,"resource-directory-admission:v1"));
  cancelGate.exec("pragma busy_timeout=0");
  cancelGate.exec("begin exclusive");
  const cause=new Error("cancelled during shared directory admission");
  const timer=setTimeout(()=>stopped.abort(cause),40);
  const cancelStarted=performance.now();
  let visited=false;
  try{
    await assert.rejects(()=>withLeaseTaskV1(()=>withFreshArtifactPublicationWritersV1(join(root,"abort"),directory,cancellable,async()=>{visited=true;})),(error:unknown)=>error===cause);
  }finally{
    clearTimeout(timer);cancelGate.exec("rollback");cancelGate.close();
  }
  assert.equal(visited,false);
  assert.ok(performance.now()-cancelStarted<fixture.collisionMaximumMilliseconds);
  console.log("[DEBUG] Published after directory contention, refused a busy target, and cancelled under the original control");
});
