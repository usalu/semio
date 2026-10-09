import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const command=process.argv[2]??"source";
const manifest=readFileSync(join(import.meta.dir,"../draw-identity-owner-manifest-current.md"),"utf8");
const inputs=[...new Set([...manifest.matchAll(/`([^`]+\.(?:rs|ts|json|feature))`/g)].map(match=>match[1]!))];
for(const name of ["📜️script.ts","🦀️.rs","🟦️.ts","🔣️.json","Cargo.toml","Cargo.lock"])inputs.push(join(import.meta.dir,name));
const epoch=new Date().toISOString().replace(/[:.]/g,"-");
function capture(phase:string){const files=inputs.filter(path=>existsSync(resolve(root,path))).map(path=>{const absolute=resolve(root,path);return {path:absolute,utf16Units:absolute.length,sha256:createHash("sha256").update(readFileSync(absolute)).digest("hex")};});writeFileSync(join(import.meta.dir,`../🗑️generated/draw-id-${command}-${epoch}-${phase}.json`),JSON.stringify({capturedAt:new Date().toISOString(),runtimeEpoch:epoch,phase,command,files},null,2));if(files.some(file=>file.utf16Units>256))throw Error("Draw identity input exceeds256 UTF16 units");return files;}
capture("before");process.on("exit",()=>capture("after"));
if(command==="native"){
 const {runCargoTestsV1,readCargoTestPolicyV1}=await import("../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts");
 const controller=new AbortController(),stop=()=>controller.abort();process.once("SIGINT",stop);process.once("SIGTERM",stop);
 try{await runCargoTestsV1({manifestPath:join(import.meta.dir,"Cargo.toml"),packages:[],cwd:import.meta.dir,extraArgs:["--lib","--offline","--","--nocapture"],signal:controller.signal},readCargoTestPolicyV1(process.env));}finally{process.off("SIGINT",stop);process.off("SIGTERM",stop);}process.exit(0);
}
const args=command==="source"?["bun","test",join(import.meta.dir,"🟦️.ts"),join(root,"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts")]:command==="whole"?["bun",join(root,"✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts"),"test"]:[];
if(!args.length)throw Error("Unknown Draw identity command");
const child=Bun.spawn(args,{cwd:root,env:{...process.env,CARGO_TARGET_DIR:join(import.meta.dir,"../🗑️generated/dy/t"),CARGO_INCREMENTAL:"0",CARGO_PROFILE_DEV_DEBUG:"0",CARGO_PROFILE_TEST_DEBUG:"0",CARGO_PROFILE_DEV_CODEGEN_UNITS:"1",CARGO_PROFILE_TEST_CODEGEN_UNITS:"1"},stdout:"inherit",stderr:"inherit"});if(await child.exited!==0)throw Error("Draw identity law failed");

if(command==="source"){
 const base=join(root,"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any");
 const check=Bun.spawn(["bun",join(root,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--skipLibCheck",...['🧬️schema/🪪️identity/🟦️.ts','🚪️io/💾️binary/🪪️identity/🟦️.ts','🚪️io/📝️text/🪪️identity/🟦️.ts'].map(path=>join(base,path))],{cwd:root,stdout:"inherit",stderr:"inherit"});if(await check.exited!==0)throw Error("Draw identity strict TypeScript failed");console.log("[DEBUG] Draw identity production strict TypeScript accepted");
}
