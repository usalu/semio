import assert from "node:assert/strict";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {createHash} from "node:crypto";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2),output=join(ticket,"🗑️generated/canvas-native",command+"-"+epoch);
assert.ok(["assets","native"].includes(command));assert.ok(epoch&&/^\d+$/.test(epoch));assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const owner="🧰️framework/🔨️modules/🖼️canvas",manifest=owner+"/📦️packages/🦀️rust/Cargo.toml",runtime="🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts";
const{readdirSync}=await import("node:fs");
const walk=(directory:string):string[]=>readdirSync(join(root,directory),{withFileTypes:true}).flatMap(entry=>entry.name==="node_modules"||entry.name==="dist"||entry.name==="pkg"?[]:entry.isDirectory()?walk(directory+"/"+entry.name):entry.isFile()&&entry.name!=="📜️script.ts"?[directory+"/"+entry.name]:[]);
const paths=["Cargo.toml","Cargo.lock","🧰️framework/Cargo.toml","🧰️framework/Cargo.lock",...walk(owner),"🧰️framework/🔨️modules/🖼️assets/🔣️icons/🏗️builder/📽️projection/🟦️.ts","🧰️framework/🔨️modules/🖼️assets/🏗️builder/📦️publication/🟦️.ts",runtime].filter(path=>!path.endsWith(".ttf"));
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return{path,source,sha256:createHash("sha256").update(source).digest("hex")};},producer=capture(import.meta.path),sources=paths.map(path=>({...capture(join(root,path)),relativePath:path}));
const policy={maximumUnits:1000000000,maximumOwnedBytes:16777216,maximumCaptureBytes:67108864,budgetMs:600000};
const argv=command==="native"?["test","--offline","--locked","--manifest-path",join(root,manifest),"-p","semio-framework-canvas","--all-targets","--no-fail-fast","--","--nocapture"]:[join(root,"🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts"),"generate","rust"];
const authority={producer,sources,policy,argv,scope:command==="native"?"all General Canvas owning package targets":"General Assets Rust shortcode publication",wholeDependencyClosureAccepted:false,atomicSourceSnapshotClaimed:false};
writeFileSync(join(output,"admission.json"),JSON.stringify(authority));
const controller=new AbortController(),stop=()=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);let result:any,code=1,error:string|undefined;
try{
 const{MutationInventoryProcessWorkspace,runMutationInventoryProcess,readMutationInventoryCapture}=await import(join(root,runtime));let lastProgress=0;
 const operation={signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace:new MutationInventoryProcessWorkspace(),onProgress:(value:any)=>{if(value.stage==="waiting"&&value.completed-lastProgress>=65536){lastProgress=value.completed;console.log("[DEBUG] General Canvas units="+value.completed+" capture="+value.capturedBytes);}},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))};
 for(const source of[producer,...sources])assert.equal(readFileSync(source.path,"utf8"),source.source);
 const target=join(ticket,"🗑️generated/current-native-canonical-6/target");
 result=await runMutationInventoryProcess({command:command==="native"?"cargo":"bun",argv,cwd:root,environment:{...process.env,SEMIO_TEST_LEVEL:"long",SEMIO_COVERAGE:"0",CARGO_PROFILE_DEV_DEBUG:"0",CARGO_INCREMENTAL:"0",CARGO_BUILD_JOBS:"2",RUST_MIN_STACK:"134217728",CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:target},captureDirectory:join(output,"captures"),budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes},operation);
 code=result.reason==="exit"&&result.code===0?0:result.code||1;
 const stdout=await readMutationInventoryCapture(result.stdout,operation),stderr=await readMutationInventoryCapture(result.stderr,operation);writeFileSync(join(output,"stdout.txt"),stdout);writeFileSync(join(output,"stderr.txt"),stderr);
 if(code===0)console.log("[DEBUG] General Canvas declared scope succeeded: "+command);
}catch(failure){code=1;error=String(failure);}finally{
 const postSource=(source:{path:string,source:string})=>{try{const current=readFileSync(source.path,"utf8");return{path:source.path,current,exact:current===source.source};}catch(failure){return{path:source.path,current:null,exact:false,error:String(failure)};}},post=sources.map(postSource),producerPost=postSource(producer),exactSelectedSources=post.every(row=>row.exact),exactProducer=producerPost.exact;if(!exactSelectedSources||!exactProducer)code=1;
 writeFileSync(join(output,"terminal.json"),JSON.stringify({...authority,result,post,producerPost,exactSelectedSources,exactProducer,code,error,allOwningPackageTargetsExecuted:command==="native"&&code===0}));process.off("SIGINT",stop);process.off("SIGTERM",stop);process.exitCode=code;
}
