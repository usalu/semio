import {existsSync,readdirSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";

const fixturePath="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/💾️memory/🧪️tests/🧫️fixtures/🔣️.json";
const imageTest=process.argv.includes("--test-image");
const preparationTest=process.argv.includes("--test-preparation")||imageTest;
const sourceCheck=process.argv.includes("--check-source")||preparationTest;
const ticket=import.meta.dir, scratch=resolve(ticket,"🗑️generated"), build=join(scratch,"native-mcp/debug/build");
const input=join(ticket,"mcp-core-memory-law.rs"), program=join(scratch,"mcp-core-memory-law.wasm");
const zero=join(scratch,"mcp-core-memory-zero.wasm");
const output=join(scratch,process.platform==="win32"?"mcp-core-memory-law.exe":"mcp-core-memory-law");
const fixture=JSON.parse(readFileSync(fixturePath,"utf8"));
function writeProgram(prefix:string,path:string){
 const row=fixture.cases.find((entry:{name:string})=>entry.name.startsWith(prefix));
 if(!row)throw Error(`Missing Wasm fixture ${prefix}`);
 const wasm=[0,97,115,109,1,0,0,0];
 const section=(id:number,data:number[])=>wasm.push(id,data.length,...data);
 section(1,[1,96,1,127,1,127]);
 section(3,[1,0]);
 section(5,[1,1,1,row.maximumPages??2]);
 section(7,[1,3,114,117,110,0,0]);
 section(10,[1,row.body.length,...row.body]);
 section(11,[1,0,65,0,11,6,97,98,99,100,101,102]);
 writeFileSync(path,Uint8Array.from(wasm));
}
writeProgram("repeated growth",program);
writeProgram("zero delta",zero);
const names=readdirSync(build,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>entry.name);
const variantCache=new Map<string,string[]>();
const variants=(pkg:string)=>{
 const existing=variantCache.get(pkg);
 if(existing)return existing;
 if(!names.includes(pkg))return [];
 const values=readdirSync(join(build,pkg)).filter(name=>existsSync(join(build,pkg,name,"out")));
 variantCache.set(pkg,values);
 return values;
};
const cratePath=(pkg:string,identity:string,crate:string)=>join(pkg,identity,"out",`lib${crate}-${identity}`);
const rootPkg="semio-plugin-interpreter", rootCrate="semio_plugin_interpreter";
const rootIdentity=process.env.SEMIO_MEMORY_CORE_VARIANT??variants(rootPkg).filter(variant=>existsSync(join(build,cratePath(rootPkg,variant,rootCrate)+".rlib")))
 .sort((left,right)=>statSync(join(build,cratePath(rootPkg,right,rootCrate)+".rlib")).mtimeMs-statSync(join(build,cratePath(rootPkg,left,rootCrate)+".rlib")).mtimeMs)[0];
if(!rootIdentity)throw Error("Missing linked Core interpreter");
const externs=new Map<string,[string,string]>([[rootCrate,[rootPkg,rootIdentity]]]);
const required=new Set(["semio_framework_actor","semio_framework_job","semio_framework_trace","semio_framework_value"]);
const targets=[{pkg:rootPkg,identity:rootIdentity,crate:rootCrate}];
const visited=new Set<string>();
while(targets.length&&required.size){
 const {pkg,identity,crate}=targets.shift()!;
 const path=join(build,pkg,identity,`fingerprint/lib-${crate}.json`);
 if(visited.has(path)||!existsSync(path))continue;
 visited.add(path);
 const json=readFileSync(path,"utf8");
 for(const match of json.matchAll(/\[\d+,"([a-z0-9_]+)",false,(\d+)\]/gu)){
  const [,dep,digest]=match;
  const dependencyPkg=dep.startsWith("semio_")?dep.replaceAll("_","-"):dep;
  const expected=Buffer.from(BigInt(digest).toString(16).padStart(16,"0"),"hex").reverse().toString("hex");
  const dependency=variants(dependencyPkg).find(variant=>{
   const fingerprint=join(build,dependencyPkg,variant,`fingerprint/lib-${dep}`);
   return existsSync(fingerprint)&&readFileSync(fingerprint,"utf8").trim()===expected;
  });
  if(!dependency)continue;
  if(required.delete(dep))externs.set(dep,[dependencyPkg,dependency]);
  if(dep.startsWith("semio_"))targets.push({pkg:dependencyPkg,identity:dependency,crate:dep});
 }
}
if(required.size)throw Error(`Missing exact crate identities: ${[...required].join(", ")}`);
const allocatorPkg="semio-framework-value-testing-allocator",allocatorCrate="semio_framework_value_testing_allocator";
const [tracePkg,traceIdentity]=externs.get("semio_framework_trace")!;
const traceHash=readFileSync(join(build,tracePkg,traceIdentity,"fingerprint/lib-semio_framework_trace"),"utf8").trim();
const allocator=variants(allocatorPkg).find(variant=>{
 const fingerprint=join(build,allocatorPkg,variant,`fingerprint/lib-${allocatorCrate}.json`);
 if(!existsSync(fingerprint))return false;
 const digest=readFileSync(fingerprint,"utf8").match(/"semio_framework_trace",false,(\d+)/u)?.[1];
 return digest&&Buffer.from(BigInt(digest).toString(16).padStart(16,"0"),"hex").reverse().toString("hex")===traceHash;
});
if(!allocator)throw Error("Missing compatible workspace testing allocator");
externs.set(allocatorCrate,[allocatorPkg,allocator]);
const dependencies=[...externs.entries()].filter(([crate])=>!sourceCheck||["semio_framework_actor","semio_framework_job","semio_framework_value"].includes(crate)).flatMap(([crate,[pkg,identity]])=>{
 const artifact=cratePath(pkg,identity,crate);
 return ["--extern",`${crate}=${artifact}.rmeta`,"--extern",`${crate}=${artifact}.rlib`];
});
const searchPaths=names.flatMap(pkg=>variants(pkg).flatMap(variant=>{
 const path=join(pkg,variant,"out");
 return ["-L",`dependency=${path}`,...(readdirSync(join(build,path)).some(file=>/\.(a|lib)$/.test(file))?["-L",`native=${path}`]:[])];
}));
const installed=Bun.spawnSync(["repo","tools","list","--json"],{stderr:"inherit"});
if(installed.exitCode)process.exit(installed.exitCode);
const rustup=JSON.parse(installed.stdout.toString()).find((entry:{id:string})=>entry.id==="rustup")?.command;
const toolchain=readFileSync("rust-toolchain.toml","utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if(!rustup||!toolchain)throw Error("Pinned Rust unavailable");
console.error(`[DEBUG] linked Core interpreter variant ${rootIdentity}`);
if(sourceCheck){
 const source=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs");
 const target=join(scratch,imageTest?"libsemio_plugin_interpreter-image.rlib":preparationTest?"libsemio_plugin_interpreter.rlib":"mcp-core-preparation-check.rmeta");
 const check=Bun.spawnSync([rustup,"run",toolchain,"rustc","--edition=2021","--crate-name","semio_plugin_interpreter","--crate-type","rlib",...(preparationTest?["-Copt-level=3"]:["--emit=metadata"]),source,
  "-o",target,...dependencies,...searchPaths],{cwd:build,stdout:"inherit",stderr:"inherit"});
 if(check.exitCode||!preparationTest)process.exit(check.exitCode??1);
 const json=variants("serde_json").find(identity=>existsSync(join(build,cratePath("serde_json",identity,"serde_json")+".rlib")));
 if(!json)throw Error("Missing fixture JSON reader");
 const jsonArtifact=cratePath("serde_json",json,"serde_json");
 const name=imageTest?"mcp-core-image-public-law":"mcp-core-preparation-law";
 const law=join(scratch,process.platform==="win32"?name+".exe":name);
 const fixture=imageTest?resolve(source,"..","🏃️runtime","🏗️startup","🧪️tests","🧫️fixtures","prepared.json"):resolve(source,"..","🔌️owned-semio-boundary","🌱️preparation","🧪️tests","🧫️fixtures","🔣️.json");
 const coreImageFixture=resolve(source,"..","🏗️instance","🧊️prepared","🧵️image-wire","🧪️tests","prepared-image.json");
 const hashCrate="semio_framework_hash",hashPkg="semio-framework-hash";
 const hashId=variants(hashPkg).find(identity=>existsSync(join(build,cratePath(hashPkg,identity,hashCrate)+".rlib")));
 if(!hashId)throw Error("Missing prepared wire SHA-256 oracle");
 const imageDependencies=hashId?["--extern",`${hashCrate}=${cratePath(hashPkg,hashId,hashCrate)}.rlib`,"--extern",`${hashCrate}=${cratePath(hashPkg,hashId,hashCrate)}.rmeta`]:[];
 const link=Bun.spawnSync([rustup,"run",toolchain,"rustc","--edition=2024","--test",join(ticket,name+".rs"),
  "-o",law,"--extern",`semio_plugin_interpreter=${target}`,"--extern",`serde_json=${jsonArtifact}.rlib`,"--extern",`serde_json=${jsonArtifact}.rmeta`,...imageDependencies,...searchPaths],
  {cwd:build,env:{...process.env,SEMIO_PREPARATION_FIXTURE:fixture,SEMIO_CORE_IMAGE_FIXTURE:coreImageFixture},stdout:"inherit",stderr:"inherit"});
 if(link.exitCode)process.exit(link.exitCode);
 const tested=Bun.spawnSync([law,"--nocapture",...(process.env.SEMIO_PREPARATION_COMPONENT?["--include-ignored"]:[])],{stdout:"inherit",stderr:"inherit"});
 process.exit(tested.exitCode??1);
}
const compile=Bun.spawnSync([rustup,"run",toolchain,"rustc","--edition=2024","--test",input,"-o",output,...dependencies,...searchPaths],{
 cwd:build,env:{...process.env,SEMIO_LINKED_MEMORY_FIXTURE:program,SEMIO_LINKED_MEMORY_ZERO:zero},stdout:"inherit",stderr:"inherit",
});
if(compile.exitCode!==0)process.exit(compile.exitCode??1);
process.exit(Bun.spawnSync([output,"--nocapture"],{stdout:"inherit",stderr:"inherit"}).exitCode??1);
