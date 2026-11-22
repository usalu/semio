import {test,expect} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {captureGenerationReceiptV1,generationProtocolV1 as protocol,generationSha256V1,sealGenerationReceiptV1,verifyGenerationReceiptV1,type GenerationControlV1,type GenerationReceiptV1,type GenerationRuntimeV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🟦️.ts";
import {manifestProfileSha256V1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🪪️profile/🟦️.ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧬️schema/🔣️.json" with {type:"json"};
import * as glue from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts";
import {testPythonV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {publishBunWorkspaceOwnership,withCurrentBunWorkspaceOwnershipV1,readBunWorkspaceListingProjectionV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";

const runtime:GenerationRuntimeV1={manager:"0.0.0",revision:"fixture",node:"0.0.0",platform:"fixture",arch:"fixture",sha256:"a".repeat(64)};
const oracle=fileURLToPath(new URL("file:///Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧪️tests/🐍️.py")),schemaPath=fileURLToPath(new URL("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/🧬️schema/🔣️.json",import.meta.url)),python=()=>testPythonV1()??"python3";
const control=():GenerationControlV1=>({signal:new AbortController().signal,check:()=>{},tick:async()=>{}});
const manifest=(name:string,extra:object={})=>JSON.stringify({name,version:"1.0.0",exports:{".":"./a.js"},scripts:{build:"x"},...extra});
const scopeManifest=(name:string,members:string[])=>manifest(name,{semio:{workspace:{schemaVersion:1,members,owners:["*/package.json"]}}});
const READY="fingerprint";
/** 🌍️ A root scope over `lib`, scope `sa` over its own package and `lib`, scope `sb` over its own package, `lib` and the packages of `sa`. */
function world(){
 const root=realpathSync(mkdtempSync(join(tmpdir(),"gr-scale-"))),store=join(root,".store");
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 put("package.json",manifest("root",{semio:{workspace:{schemaVersion:1,members:["lib/**"],owners:["*/package.json"]}}}));
 for(let i=0;i<300;i++)put("lib/p"+i+"/package.json",manifest("lib-p"+i));
 put("sa/package.json",scopeManifest("sa",["**","../lib/**"]));for(let i=0;i<150;i++)put("sa/q"+i+"/package.json",manifest("sa-q"+i));
 put("sb/package.json",scopeManifest("sb",["**","../lib/**","../sa/**"]));for(let i=0;i<150;i++)put("sb/q"+i+"/package.json",manifest("sb-q"+i));
 put("bun.lock","{}");mkdirSync(store,{recursive:true});
 return{root,store,put};
}
const REQUIRED:Record<string,string[]>={".":["."],sa:[".","sa"],sb:[".","sa","sb"]};
const DIRECTORIES=[".","sa","sb"];
const run=(...args:string[])=>{const out=spawnSync(python(),["-I",oracle,...args],{encoding:"utf8"});if(out.status!==0)throw Error("oracle failed: "+out.stderr);return JSON.parse(out.stdout);};
async function receiptOf(root:string,store:string,scope:string,walked:Awaited<ReturnType<typeof walk>>){
 const required=REQUIRED[scope]!,ambient={env:{},home:join(root,".ambient/home"),config:join(root,".ambient/xdg")};
 const sources=(owner:string)=>walked.texts.filter(row=>walked.textOwners[row.path]===owner).map(row=>{const f=lstatSync(join(root,row.path));return{path:join(root,row.path),kind:"file" as const,dev:String(f.dev),ino:String(f.ino),mode:f.mode,bytes:readFileSync(join(root,row.path))};});
 const generation="e".repeat(64),observation="[]",scopes=required.map(name=>({scope:name,sha256:"f".repeat(64)})),receipts=required.map(name=>({scope:name,generation,fingerprint:READY}));
 const input={workspace:root,storeRoot:store,scope,startedAtMs:Date.now()+10000,runtime,producer:glue.observeJavascriptDependencyGenerationProducerV1(root,store,required,ambient),recipeSha256:"c".repeat(64),ownershipSha256:"d".repeat(64),required,
  inputs:DIRECTORIES.map(name=>({scope:name,sources:sources(name)})),
  membership:{root:walked.root,paths:walked.paths,texts:walked.texts.map(row=>({...row,owner:walked.textOwners[row.path]!,profileSha256:manifestProfileSha256V1(row.path,readFileSync(join(root,row.path)))}))},
  bindings:required.map(name=>({scope:name,pairs:[],observation})),ready:required.map(name=>({scope:name,generation,path:join(store,"j/g1/bindings.json"),fingerprint:READY})),
  execution:{scopes,runtimeSha256:runtime.sha256,observations:[]},scopeReadSha256:generationSha256V1(JSON.stringify([receipts,required.map(name=>[name,observation]),scopes])),control:control()};
 const result=await captureGenerationReceiptV1(input);expect(result.status).toBe("captured");if(result.status!=="captured")throw Error("deferred");
 return{receipt:result.receipt,files:new Map(result.files.map(file=>[file.name,file.bytes])),ambient};
}
const walk=async(root:string)=>{await publishBunWorkspaceOwnership(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}});return withCurrentBunWorkspaceOwnershipV1(root,{kind:"all"},{signal:new AbortController().signal,advance:async()=>{}},async custody=>custody.observed());};
const verify=(root:string,store:string,receipt:GenerationReceiptV1,files:Map<string,Uint8Array>,ambient:{env:Record<string,string>;home:string;config:string})=>verifyGenerationReceiptV1({
 workspace:root,storeRoot:store,receipt,readEvidence:name=>{const bytes=files.get(name);if(!bytes)throw Error("Sealed generation evidence required");return bytes;},
 producer:()=>glue.observeJavascriptDependencyGenerationProducerV1(root,store,receipt.required,ambient),runtime:()=>runtime,conditionsClosed:()=>true,
 project:path=>readBunWorkspaceListingProjectionV1(root,path),observeBindings:()=>"[]",readReady:()=>READY,control:control(),
});
const oracleVerify=(root:string,store:string,receipt:GenerationReceiptV1)=>{
 const file=root+".receipt.json";writeFileSync(file,JSON.stringify(receipt));
 try{return run("verify",file,root,store,JSON.stringify(runtime),schemaPath,JSON.stringify({env:{},policy:protocol.producer}))as{status:string;class?:string};}finally{rmSync(file,{force:true});}
};
const outcome=(result:{status:string;class?:string})=>result.status==="match"?"match":"mismatch:"+result.class;


const exportsEdit=(text:string)=>JSON.stringify({...JSON.parse(text),exports:{".":"./b.js"}});
const scriptsEdit=(text:string)=>JSON.stringify({...JSON.parse(text),scripts:{build:"y"}});
const ms=()=>Number(process.hrtime.bigint()/1000n)/1000;
test("measure",async()=>{
 const{root,store,put}=world();
 try{
  let t=ms();const walked=await walk(root);const tw=ms()-t;
  const receipts=new Map<string,Awaited<ReturnType<typeof receiptOf>>>();
  t=ms();for(const scope of Object.keys(REQUIRED))receipts.set(scope,await receiptOf(root,store,scope,walked));const tc=ms()-t;
  const rows:string[]=["walk "+tw.toFixed(0)+" ms, capture of 3 receipts "+tc.toFixed(0)+" ms, manifests "+walked.texts.length];
  const cases:[string,string,(t:string)=>string][]=[["unchanged","sa/q0/package.json",t=>t],["inert scripts edit in sa/q0","sa/q0/package.json",scriptsEdit],["exports edit in sb/q0 (sb only)","sb/q0/package.json",exportsEdit],["exports edit in sa/q0","sa/q0/package.json",exportsEdit],["exports edit in lib/p0","lib/p0/package.json",exportsEdit]];
  for(const[name,path,change]of cases){
   const original=readFileSync(join(root,path),"utf8");put(path,change(original));
   const parts:string[]=[];
   for(const scope of Object.keys(REQUIRED)){const row=receipts.get(scope)!;const a=ms();const v=await verify(root,store,row.receipt,row.files,row.ambient);parts.push(scope+"="+(v.status==="match"?"match":"STALE")+" "+(ms()-a).toFixed(0)+"ms");}
   rows.push(name+": "+parts.join(", "));put(path,original);
  }
  console.log("[E58-MEASURE]\n"+rows.join("\n"));
 }finally{rmSync(root,{recursive:true,force:true});}
},600000);
