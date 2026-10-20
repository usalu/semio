import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import {confirmGenerationWalkV1,generationProtocolV1 as protocol,generationSha256V1,guardGenerationReplayV1,generationWalkOriginV1} from "../../🟦️.ts";
import * as glue from "../../💾️storage/🟦️.ts";
import * as storage from "../../../💾️storage/🟦️.ts";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../../🏗️build/📥️caller/🟦️.ts";
import {withCurrentBunWorkspaceOwnershipV1} from "../../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import peer from "../🔣️.json" with {type:"json"};

const here=(relative:string)=>fileURLToPath(new URL(relative,import.meta.url));
const oracle=(...args:string[])=>{const run=spawnSync("python3",[here("./🐍️oracle.py"),...args],{encoding:"utf8"});if(run.status!==0)throw Error("oracle failed: "+run.stderr);return run.stdout.trim();};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"followup-")));
const text=(relative:string)=>readFileSync(here(relative),"utf8");
const pattern=(size:number)=>{const bytes=Buffer.alloc(size);for(let index=0;index<size;index++)bytes[index]=(index*31+7)&255;return bytes;};
const descriptorSchema="../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json";

test("followup fixture: a third-party validator accepts the scenarios and the evidence cap equals the relative-create file cap read independently",()=>{
 const ajv=new Ajv({strict:true}),valid=ajv.validate(schema,fixture);
 expect(ajv.errors).toBeNull();expect(valid).toBe(true);
 const independent=Number(oracle("cap",here(descriptorSchema)));
 expect(independent).toBe(fixture.cap.fileCap);
 expect(protocol.maximumEvidenceBytes).toBe(independent);
 expect(fixture.cap.evidenceCap).toBe(protocol.maximumEvidenceBytes);
});

test("followup guard: a replay that cannot capture is a deferral for the complete walk and only cancellation throws, equal to an independent oracle",async()=>{
 const expected=JSON.parse(oracle("expect",here("./🔣️.json"))).guard;
 for(const row of fixture.guard){
  let status:string|null=null,reason:string|null=null,rethrows=false;
  try{const result=await guardGenerationReplayV1({run:async()=>{if("throws"in row.run)throw Error(row.run.throws);return row.run.returns==="match"?{status:"match",sha256:"aaaaaaaa"}:row.run.returns==="deferred"?{status:"deferred",reason:"unsettled"}:{status:"mismatch",class:"input-changed"} as never;},aborted:()=>row.aborted});status=result.status;reason=result.status==="deferred"&&result.reason==="capture-failed"?result.reason:null;if(result.status==="deferred"&&result.reason==="capture-failed"&&"throws"in row.run)expect(result.path).toBe(row.run.throws);}
  catch{rethrows=true;}
  expect({status,reason,rethrows},row.id).toEqual(expected[row.id]);
  expect({status,reason,rethrows},row.id).toEqual(row.expect as never);
 }
});

test("followup origin: the settle rule is bounded by the earliest walk start, equal to an independent oracle",()=>{
 const expected=JSON.parse(oracle("expect",here("./🔣️.json"))).origin;
 for(const row of fixture.origin){
  expect(generationWalkOriginV1(row.shared??undefined,row.pass),row.id).toBe(expected[row.id]);
  expect(generationWalkOriginV1(row.shared??undefined,row.pass),row.id).toBe(row.expect);
 }
});

test("followup decision: whatever the replay does, the shared pass is ready only when the confirmed sha equals the owner's",async()=>{
 const expected=JSON.parse(oracle("expect",here("./🔣️.json"))).decision;
 for(const row of fixture.decision){
  const confirmed=await confirmGenerationWalkV1({replay:()=>guardGenerationReplayV1({run:async()=>{if(row.replay==="throws")throw Error("Generation evidence capacity exceeded");return row.replay==="match"?{status:"match",sha256:row.owner}:row.replay==="deferred"?{status:"deferred",reason:"unsettled"}:{status:"mismatch",class:"input-changed"} as never;},aborted:()=>false}),walk:async()=>({sha256:row.walk})});
  const state=confirmed.sha256===row.owner?"ready":"custody-changed";
  expect(state,row.id).toBe(expected[row.id]);expect(state,row.id).toBe(row.expect);
 }
});

const bootstrap=()=>text("../../../../../../🏗️build/🚀️bootstrap/🟦️.ts");
test("followup laws: the replay is guarded, every walk-related settle start is the shared walk's, leaf removal goes through the descriptor",()=>{
 const source=bootstrap(),storageSource=text("../../💾️storage/🟦️.ts");
 expect(source).toContain("const walkOriginAt=generation.generationWalkOriginV1(cached?.startedAt,walkStartedAt);");
 expect(source).toContain("check,walk!,walkOriginAt),walk:()=>currentOwner()}");
 expect(source).toContain("check,walk,walkOriginAt);return \"ready\";");
 expect(source).not.toContain("walk!,walkStartedAt),walk:()=>currentOwner()");
 expect(storageSource).toContain("async function replayJavascriptDependencyGenerationWalkUnguardedV1(");
 expect(storageSource).toContain("export function replayJavascriptDependencyGenerationWalkV1(");
 expect(storageSource).toContain("guardGenerationReplayV1({run:()=>replayJavascriptDependencyGenerationWalkUnguardedV1(");
 expect(storageSource).toContain("async function removeUnfinishedGenerationLeafV1(");
 expect(storageSource).toContain("await directory.remove(name,leaf)");
 expect(storageSource).not.toContain("unlinkSync(join(storeRoot,protocol.directory,file.name))");
});

const withOriginal=<T>(body:(original:issued.ScriptInvocation)=>Promise<T>):Promise<T>=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-followup",maximumElapsedMilliseconds:60000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),body);
const facts=(path:string)=>lstatSync(path,{bigint:true,throwIfNoEntry:false});

test("followup replay: a capture failure is contained and the walk's start time is what the settle rule sees",()=>withOriginal(async original=>{
 const root=scratch(),store=join(root,"store");
 try{
  for(const[path,body]of Object.entries(peer.workspace.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);}
  for(const[path,target]of Object.entries(peer.workspace.links)){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(target.replaceAll("@STORE@",store),join(root,path));}
  const generation="e".repeat(64),directory=join(store,"j",generation.slice(0,20)),bindings=join(directory,"bindings.json"),target=join(directory,"node_modules");
  mkdirSync(target,{recursive:true});writeFileSync(bindings,"{\"ready\":true}\n");mkdirSync(join(root,"pkgs/a"),{recursive:true});rmSync(join(root,"pkgs/a/node_modules"),{force:true});symlinkSync(target,join(root,"pkgs/a/node_modules"));
  const phase=issued.createIssuedResourceControlV1(original,"dependencies:generation-followup",0),context:storage.JavascriptDependencyReadContextV1=Object.freeze({root:store,invocation:original,control:phase,check:(actual:storage.JavascriptDependencyReadContextV1["control"])=>issued.requireIssuedResourceControlRefinementV1(phase,actual),recheck:async(actual:storage.JavascriptDependencyReadContextV1["control"])=>{issued.requireIssuedResourceControlRefinementV1(phase,actual);}});
  const pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:target}],observation=storage.observeJavascriptDependencyBindingsReadV1(context,pairs,phase),receipt=await storage.readJavascriptDependencyBindingReceiptReadV1(context,bindings,phase);
  const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());
  const providerPath=join(store,"j/g1/node_modules/dep/package.json"),pastWrite=new Date(Date.now()-1000);utimesSync(providerPath,pastWrite,pastWrite);
  const providers=[providerPath,join(store,"j/g1/node_modules/dep"),join(root,"node_modules")].map(path=>{const f=facts(path);return{path,stamp:f?JSON.stringify(protocol.stampFields.map(key=>String(f[key as keyof typeof f]))):null,anchor:f?JSON.stringify(protocol.anchorFields.map(key=>String(f[key as keyof typeof f]))):null,kind:f?f.isDirectory()?"directory":"file":"missing",target:null,full:path.endsWith("package.json")};});
  const scopes=[{scope:".",sha256:"f".repeat(64)}],receipts=[{scope:".",generation,fingerprint:receipt.fingerprint}],sha256=generationSha256V1(JSON.stringify([receipts,[[".",observation]],scopes])),f=lstatSync(join(root,"package.json"));
  const walk={version:1 as const,required:["."],projection:{owners:{sha256:"d".repeat(64)},recipeSha256:"c".repeat(64),plans:[{scope:".",plan:{directory,generation,bindings:pairs,sources:[{path:join(root,"package.json"),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,"package.json"))}]}}]},receipts,sha256,evidence:{scope:".",execution:{scopes,runtimeSha256:storage.javascriptDependencySourceOwnerRuntimeV1(),observations:providers},bindings:[[".",observation]]},observed:()=>walked};
  const subject=Object.freeze({root,entries:peer.implementation.entries,loadedAtMs:Date.now()+10000,ambient:Object.freeze({env:{},home:join(root,".ambient/home"),config:join(root,".ambient/xdg")})});
  const replay=(candidate:unknown,startedAt:number)=>glue.replayJavascriptDependencyGenerationWalkV1(root,context,phase,()=>{},candidate as never,startedAt,walked,subject);
  const sharedStart=Date.now()-60000,passStart=Date.now()+10000;
  expect(generationWalkOriginV1(sharedStart,passStart)).toBe(sharedStart);
  expect(await replay(walk,passStart)).toEqual({status:"match",sha256});
  expect(await replay(walk,generationWalkOriginV1(sharedStart,passStart))).toMatchObject({status:"deferred",reason:"provider-observation-unsettled"});
  const broken={...walk,evidence:{...walk.evidence,bindings:[]}};
  const contained=await replay(broken,passStart);
  expect(contained).toMatchObject({status:"deferred",reason:"capture-failed"});
  const confirmation=await confirmGenerationWalkV1({replay:()=>replay(broken,passStart),walk:async()=>({sha256:"fresh"})});
  expect(confirmation).toMatchObject({via:"walk",sha256:"fresh"});
 }finally{rmSync(root,{recursive:true,force:true});}
}),60000);

test("followup files: a failed write is removed through the descriptor and the same content can be written again byte-identically",async()=>{
 const root=scratch();
 try{
  const name=generationSha256V1("retry")+".frontier.ndjson";let calls=0;
  await expect(glue.writeJavascriptDependencyGenerationFilesV1(root,[{name,bytes:pattern(300000)}],()=>{},async()=>{if(++calls===3)throw Error("cancelled mid write");})).rejects.toThrow("cancelled mid write");
  expect(readdirSync(join(root,protocol.directory))).not.toContain(name);
  await glue.writeJavascriptDependencyGenerationFilesV1(root,[{name,bytes:pattern(300000)}],()=>{},async()=>{});
  expect(statSync(join(root,protocol.directory,name)).size).toBe(300000);
  const digest=spawnSync("shasum",["-a","256",join(root,protocol.directory,name)],{encoding:"utf8"}).stdout.split(" ")[0];
  expect(generationSha256V1(readFileSync(join(root,protocol.directory,name)))).toBe(digest);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
