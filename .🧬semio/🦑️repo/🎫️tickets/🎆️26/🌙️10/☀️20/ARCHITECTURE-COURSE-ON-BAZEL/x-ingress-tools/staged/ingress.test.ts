import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawn,spawnSync} from "node:child_process";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import {confirmGenerationWalkV1,generationAnchorV1,generationProtocolV1 as protocol,generationSha256V1,generationStampV1,publishGenerationBestEffortV1,runGenerationWriterV1} from "../../🟦️.ts";
import * as glue from "../../💾️storage/🟦️.ts";
import * as storage from "../../../💾️storage/🟦️.ts";
import {acquireResourceLease,resourceLeasePathV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../../../🏗️build/📥️caller/🟦️.ts";
import {withCurrentBunWorkspaceOwnershipV1} from "../../../../../../🗂️workspaces/🟦️bun/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import peer from "../🔣️.json" with {type:"json"};

const oracle=(...args:string[])=>{const run=spawnSync("python3",[fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),...args],{encoding:"utf8"});if(run.status!==0)throw Error("oracle failed: "+run.stderr);return run.stdout.trim();};
const scratch=()=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"ingress-")));
const pattern=(size:number)=>{const bytes=Buffer.alloc(size);for(let index=0;index<size;index++)bytes[index]=(index*31+7)&255;return bytes;};
const text=(relative:string)=>readFileSync(new URL(relative,import.meta.url),"utf8");

test("ingress fixture: a third-party validator accepts the scenarios and the ceiling equals the relative-create protocol",()=>{
 const ajv=new Ajv({strict:true}),valid=ajv.validate(schema,fixture);
 expect(ajv.errors).toBeNull();expect(valid).toBe(true);
 const descriptor=JSON.parse(text("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json")).definitions.RelativeCreateProtocol.const;
 expect(fixture.files.chunkCeiling).toBe(descriptor.maximumWriteBytes);
 expect(text("../../💾️storage/🟦️.ts")).toContain("const relativeWriteCeilingBytes="+descriptor.maximumWriteBytes);
 expect(Math.max(...fixture.files.sizes)).toBeLessThanOrEqual(descriptor.maximumFileBytes);
});

test("ingress confirm: a matching replay skips the second walk and any other answer walks exactly once, equal to an independent oracle",async()=>{
 const expected=JSON.parse(oracle("expect",fileURLToPath(new URL("./🔣️.json",import.meta.url)))).confirm;
 for(const row of fixture.confirm){
  let walks=0;
  const confirmation=await confirmGenerationWalkV1({replay:async()=>row.replay as never,walk:async()=>{walks++;return{sha256:row.walk};}});
  expect({via:confirmation.via,sha256:confirmation.sha256,walks},row.id).toEqual(expected[row.id]);
  expect({via:confirmation.via,sha256:confirmation.sha256,walks},row.id).toEqual(row.expect as never);
  if(confirmation.via==="walk")expect(confirmation.walk.sha256).toBe(row.walk);
 }
});

test("ingress publish: every outcome is reported, a failure never decides readiness and only cancellation throws",async()=>{
 const expected=JSON.parse(oracle("expect",fileURLToPath(new URL("./🔣️.json",import.meta.url)))).publish;
 for(const row of fixture.publish){
  const reports:string[]=[];let result:string|null=null,rethrows=false;
  try{result=await publishGenerationBestEffortV1({attempt:async()=>{if("throws"in row.attempt)throw Error(row.attempt.throws);return row.attempt.returns as never;},aborted:()=>row.aborted,report:(outcome,detail)=>reports.push(detail===undefined?outcome:outcome+":"+detail)});}
  catch{rethrows=true;}
  expect({result,reports,rethrows},row.id).toEqual(expected[row.id]);
  expect({result,reports,rethrows},row.id).toEqual(row.expect as never);
 }
});

test("ingress writer: the writer lease never queues, releases on every path and cancellation propagates",async()=>{
 const expected=JSON.parse(oracle("expect",fileURLToPath(new URL("./🔣️.json",import.meta.url)))).writer;
 for(const row of fixture.writer){
  let released=false,bodies=0;
  const outcome=await runGenerationWriterV1({acquire:async()=>{if(row.lease==="timeout")throw Error("Resource lease timed out: javascript-dependency-generation-writer:/x");if(row.lease==="other")throw Error("Resource lease requires rollback journal mode");return{release:()=>{released=true;}};},aborted:()=>row.aborted,body:async()=>{bodies++;if(row.body==="throw")throw Error("body failed");return"published";}}).catch(()=>"rethrow" as const);
  expect({expect:outcome,released},row.id).toEqual(expected[row.id]);
  expect({expect:outcome,released},row.id).toEqual({expect:row.expect,released:row.released} as never);
  expect(bodies,row.id).toBe(row.lease==="free"?1:0);
 }
});

test("ingress writer with the real lease: a foreign SQLite exclusive transaction (Python sqlite3) makes the writer busy without waiting, then it runs",async()=>{
 const directory=scratch(),resource="javascript-dependency-generation-writer:/fixture",abort=new AbortController();
 try{
  const run=()=>runGenerationWriterV1({acquire:()=>acquireResourceLease({directory,resource,mode:"exclusive",signal:abort.signal,timeoutMs:0}),aborted:()=>abort.signal.aborted,body:async()=>"published"});
  expect(await run()).toBe("published");
  const holder=spawn("python3",[fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),"hold",resourceLeasePathV1(directory,resource)],{stdio:["pipe","pipe","inherit"]});
  const line=async()=>{let buffer="";for await(const chunk of holder.stdout){buffer+=chunk;if(buffer.includes("\n"))break;}return buffer.trim();};
  expect(await line()).toBe("held");
  const started=performance.now();expect(await run()).toBe("busy");expect(performance.now()-started).toBeLessThan(2000);
  holder.stdin.write("\n");holder.stdin.end();await new Promise(resolve=>holder.on("close",resolve));
  expect(await run()).toBe("published");
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("ingress files: evidence of every size is written through the real bounded relative-create API byte-identically, a failed write leaves nothing behind and an existing file of another size is refused",async()=>{
 const root=scratch();
 try{
  for(const size of fixture.files.sizes){
   const name=generationSha256V1("size"+size)+".frontier.ndjson";
   await glue.writeJavascriptDependencyGenerationFilesV1(root,[{name,bytes:pattern(size)}],()=>{},async()=>{});
   const path=join(root,protocol.directory,name);
   expect(statSync(path).size,String(size)).toBe(size);
   expect(generationSha256V1(readFileSync(path)),String(size)).toBe(oracle("digest",String(size)));
  }
  const failing=generationSha256V1("fail")+".execution.ndjson";let calls=0;
  await expect(glue.writeJavascriptDependencyGenerationFilesV1(root,[{name:failing,bytes:pattern(300000)}],()=>{},async()=>{if(++calls===3)throw Error("cancelled mid write");})).rejects.toThrow("cancelled mid write");
  expect(readdirSync(join(root,protocol.directory))).not.toContain(failing);
  const name=generationSha256V1("size65536")+".frontier.ndjson";
  await expect(glue.writeJavascriptDependencyGenerationFilesV1(root,[{name,bytes:pattern(10)}],()=>{},async()=>{})).rejects.toThrow("Existing immutable generation file changed");
  expect(statSync(join(root,protocol.directory,name)).size).toBe(65536);
  await glue.writeJavascriptDependencyGenerationFilesV1(root,[{name,bytes:pattern(65536)}],()=>{},async()=>{});
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("ingress retire: superseded receipts and unreferenced evidence younger than the grace stay, older ones go",()=>{
 const root=scratch(),directory=join(root,protocol.directory);
 try{
  mkdirSync(directory,{recursive:true});
  const evidence=generationSha256V1("stray")+".frontier.ndjson";writeFileSync(join(directory,evidence),"");
  glue.retireJavascriptDependencyGenerationFilesV1(root,".",600000);expect(readdirSync(directory)).toContain(evidence);
  glue.retireJavascriptDependencyGenerationFilesV1(root,".");expect(readdirSync(directory)).not.toContain(evidence);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const bootstrap=()=>text("../../../../../../🏗️build/🚀️bootstrap/🟦️.ts");
test("ingress bootstrap laws: a shared walk decides readiness itself, confirms by replay and publishes without any exclusive custody",()=>{
 const source=bootstrap();
 expect(source).not.toMatch(/if\(!publishReceipt&&walk\)\{candidate=\{walk,startedAt:walkStartedAt\};return "receipt-required";\}/u);
 expect(source).toContain("if(!publishReceipt&&walk)candidate={walk,startedAt:walkStartedAt};");
 expect(source).toContain("const confirmWalk=()=>generation.confirmGenerationWalkV1(");
 expect(source.match(/await currentOwner\(\)/gu)?.length).toBe(1);
 expect(source).toContain("generation.publishGenerationBestEffortV1(");
 expect(source).toContain("generation.publishJavascriptDependencyGenerationReceiptSharedV1(");
 const shared=source.slice(source.indexOf("generation.publishGenerationBestEffortV1("),source.indexOf("generation.publishGenerationBestEffortV1(")+700);
 expect(shared).not.toContain("withJavascriptDependencyStorageV1");
 expect(source).toContain('if(!publishReceipt){if(!lstatSync(path,{throwIfNoEntry:false}))return "receipt-required";return buildParserReceiptMatchesV1(JSON.parse(bounded(path,bootstrap.maximumReceiptBytes).toString()),current)?"ready":"receipt-required";}');
 expect(source.match(/catch\(error\)\{if\(publishReceipt\)throw error;return "material-required";\}/gu)?.length).toBe(2);
});

const withOriginal=<T>(body:(original:issued.ScriptInvocation)=>Promise<T>):Promise<T>=>issued.withScriptProcessEnvelope(issued.createScriptProcessEnvelope({version:1,owner:"generation-ingress",maximumElapsedMilliseconds:60000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),body);
const facts=(path:string)=>lstatSync(path,{bigint:true,throwIfNoEntry:false});

test("ingress replay: the walk's own recorded observations confirm an unchanged tree and fall back for input, provider and unsettled changes",()=>withOriginal(async original=>{
 const root=scratch(),store=join(root,"store");
 try{
  for(const[path,body]of Object.entries(peer.workspace.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);}
  for(const[path,target]of Object.entries(peer.workspace.links)){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(target.replaceAll("@STORE@",store),join(root,path));}
  const generation="e".repeat(64),directory=join(store,"j",generation.slice(0,20)),bindings=join(directory,"bindings.json"),target=join(directory,"node_modules");
  mkdirSync(target,{recursive:true});writeFileSync(bindings,"{\"ready\":true}\n");mkdirSync(join(root,"pkgs/a"),{recursive:true});rmSync(join(root,"pkgs/a/node_modules"),{force:true});symlinkSync(target,join(root,"pkgs/a/node_modules"));
  const phase=issued.createIssuedResourceControlV1(original,"dependencies:generation-ingress",0),context:storage.JavascriptDependencyReadContextV1=Object.freeze({root:store,invocation:original,control:phase,check:(actual:storage.JavascriptDependencyReadContextV1["control"])=>issued.requireIssuedResourceControlRefinementV1(phase,actual),recheck:async(actual:storage.JavascriptDependencyReadContextV1["control"])=>{issued.requireIssuedResourceControlRefinementV1(phase,actual);}});
  const pairs=[{sourceDirectory:join(root,"pkgs/a/node_modules"),targetDirectory:target}],observation=storage.observeJavascriptDependencyBindingsReadV1(context,pairs,phase),receipt=await storage.readJavascriptDependencyBindingReceiptReadV1(context,bindings,phase);
  const walked=await withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());
  const providers=[join(store,"j/g1/node_modules/dep/package.json"),join(store,"j/g1/node_modules/dep"),join(root,"node_modules")].map(path=>{const f=facts(path);return{path,stamp:f?JSON.stringify(protocol.stampFields.map(key=>String(f[key as keyof typeof f]))):null,anchor:f?JSON.stringify(protocol.anchorFields.map(key=>String(f[key as keyof typeof f]))):null,kind:f?f.isDirectory()?"directory":"file":"missing",target:null,full:path.endsWith("package.json")};});
  const scopes=[{scope:".",sha256:"f".repeat(64)}],receipts=[{scope:".",generation,fingerprint:receipt.fingerprint}],sha256=generationSha256V1(JSON.stringify([receipts,[[".",observation]],scopes])),f=lstatSync(join(root,"package.json"));
  const walk={version:1 as const,required:["."],projection:{owners:{sha256:"d".repeat(64)},recipeSha256:"c".repeat(64),plans:[{scope:".",plan:{directory,generation,bindings:pairs,sources:[{path:join(root,"package.json"),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,"package.json"))}]}}]},receipts,sha256,evidence:{scope:".",execution:{scopes,runtimeSha256:storage.javascriptDependencySourceOwnerRuntimeV1(),observations:providers},bindings:[[".",observation]]},observed:()=>walked};
  const subject=Object.freeze({root,entries:peer.implementation.entries,loadedAtMs:Date.now()+10000,ambient:Object.freeze({env:{},home:join(root,".ambient/home"),config:join(root,".ambient/xdg")})}),later=Date.now()+10000;
  const replay=(startedAt:number)=>glue.replayJavascriptDependencyGenerationWalkV1(root,context,phase,()=>{},walk as never,startedAt,walked,subject);
  expect(await replay(later)).toEqual({status:"match",sha256});
  expect(await replay(Date.now())).toMatchObject({status:"deferred",reason:"provider-observation-unsettled"});
  writeFileSync(join(root,"package.json"),readFileSync(join(root,"package.json"),"utf8")+" ");
  expect(await replay(later)).toMatchObject({status:"mismatch",class:"input-changed"});
  writeFileSync(join(root,"package.json"),readFileSync(join(root,"package.json"),"utf8").slice(0,-1));
  expect(await replay(later)).toEqual({status:"match",sha256});
 }finally{rmSync(root,{recursive:true,force:true});}
}),60000);

test("ingress stamp formulas stay the ones the walk records",()=>{
 const root=scratch();
 try{writeFileSync(join(root,"x"),"x");const f=lstatSync(join(root,"x"),{bigint:true});expect(generationStampV1(f)).toBe(protocol.stampFields.map(key=>String(f[key as keyof typeof f])).join(":"));expect(generationAnchorV1(f)).toBe(protocol.anchorFields.map(key=>String(f[key as keyof typeof f])).join(":"));}
 finally{rmSync(root,{recursive:true,force:true});}
});
