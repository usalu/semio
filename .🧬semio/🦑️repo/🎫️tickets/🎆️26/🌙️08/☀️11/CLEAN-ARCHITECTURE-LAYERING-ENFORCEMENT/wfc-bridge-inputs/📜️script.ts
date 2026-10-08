import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {createRequire} from "node:module";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,selection,epoch]=process.argv.slice(2);
assert.ok(["run","admit"].includes(command)&&selection&&epoch&&/^\d+$/u.test(epoch));
const output=join(ticket,"🗑️generated/wfc-child-inventory",(command==="admit"?"admit-":"")+selection+"-"+epoch);
assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const jobs=JSON.parse(readFileSync(join(import.meta.dir,"execution-jobs.json"),"utf8"));
const chosen=selection==="red"?jobs.filter((job:any)=>job.artifact==="grid2d"&&job.mode==="check"):selection==="all"?jobs:jobs.filter((job:any)=>job.artifact===selection);
assert.ok(chosen.length);
const sourcePaths=()=>{
 const paths:string[]=[];
 const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){if(["target","dist","node_modules","🗑️generated",".git",".nx",".venv","__pycache__"].includes(entry.name))continue;const path=join(directory,entry.name);if(entry.isSymbolicLink())throw Error("WFC evidence refuses symbolic source: "+path);if(entry.isDirectory())walk(path);else if(/\.(?:[cm]?[jt]sx?|rs|toml|json|lock|wgsl|glsl|c|cpp|h|hpp|patch)$/u.test(path))paths.push(path);}};
 for(const directory of command==="run"?["✏️s/🔌️plugins/🀄️wfc","✏️s/🔌️plugins/🗄️stdio","🧰️framework"]:["✏️s/🔌️plugins/🀄️wfc"])walk(join(root,directory));
 paths.push(join(root,"🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json"));
 for(const path of ["Cargo.toml","Cargo.lock","package.json","bun.lock","nx.json","rust-toolchain.toml",".cargo/config.toml",".config/nextest.toml",".vscode/launch.json"])if(existsSync(join(root,path)))paths.push(join(root,path));
 paths.push(import.meta.path,join(import.meta.dir,"execution-jobs.json"),join(import.meta.dir,"launch-additions-nx-shared-successor.json"),join(import.meta.dir,"launch-additions-source-admission.json"));
 return [...new Set(paths)].sort();
};
const paths=sourcePaths();
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return {path,source,sha256:createHash("sha256").update(source).digest("hex")};};
const sources=paths.map(capture),controller=new AbortController(),stop=()=>controller.abort(),results:any[]=[],sourceChecks:any[]=[];
process.once("SIGINT",stop);process.once("SIGTERM",stop);
const {executeCommandV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts"));
try{
 const ajv=new Ajv({strict:false}),wire=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json"),"utf8")),validateInventory=new Ajv2020({strict:true}).compile({$schema:"https://json-schema.org/draft/2020-12/schema",$defs:wire.$defs,$ref:"#/$defs/RuntimeMutationInventory"});
 const launchRows=JSON.parse(readFileSync(join(root,".vscode/launch.json"),"utf8")).configurations;
 for(const job of chosen){
  const launch=launchRows.find((row:any)=>row.name===job.name);
  assert.ok(launch,"exact child launch row missing: "+job.name);assert.equal(launch.command,"bun "+job.request.args.join(" "));
  for(const [key,value]of Object.entries(job.environment))assert.equal(launch.env[key].replaceAll("${workspaceFolder}",root),value);
 }
 const require=createRequire(join(root,"package.json")),toml=require("@iarna/toml"),original=join(import.meta.dir,"original"),plugin="✏️s/🔌️plugins/🀄️wfc",retained=readFileSync(join(original,plugin,"🏭️bridge/🦀️.rs"),"utf8"),coordinates:any[]=[],aggregatePairs:string[]=[];
 for(const job of jobs.filter((row:any)=>row.mode==="check")){
  const owner=dirname(dirname(dirname(job.request.manifests[0]))),fixture=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧬️schema/🔣️.json"),"utf8")),source=readFileSync(join(owner,"🏭️bridge/🦀️.rs"),"utf8");
  assert.ok(ajv.compile(schema)(fixture));coordinates.push(...fixture.coordinates);
  assert.equal(fixture.refusedArtifacts.length,4);assert.ok(!fixture.refusedArtifacts.includes(fixture.artifact));assert.deepEqual([...fixture.refusedArtifacts,fixture.artifact].sort(),["s.wfc.wfc2d","s.wfc.wfc3d","s.wfc.bitmap","s.wfc.grid2d","s.wfc.grid3d"].sort());
  for(const path of [join(owner,"Cargo.toml"),job.request.manifests[0]]){const text=readFileSync(path,"utf8");assert.deepEqual(Bun.TOML.parse(text),toml.parse(text));}
  const workspace:any=Bun.TOML.parse(readFileSync(join(owner,"Cargo.toml"),"utf8")),pkg:any=Bun.TOML.parse(readFileSync(job.request.manifests[0],"utf8"));assert.deepEqual(workspace.workspace.metadata.semio["mutation-inventory"],{script:"🏭️bridge/📜️script.ts",roots:["."]});assert.deepEqual(pkg.bin[0]["required-features"],["mutation-inventory"]);
  assert.ok(pkg.features["mutation-inventory"].includes("dep:semio-framework-test-mutation-inventory"));assert.ok(!Object.keys(pkg.dependencies).some(name=>name.startsWith("semio-s-artifact-wfc-")&&name!==pkg.package.name));
  for(const match of source.matchAll(/<([^<>]+) as Mutation<([^<>]+)>>::DESCRIPTORS/gu))aggregatePairs.push(match[2]+","+match[1]);
  const lockPath=join(owner,"Cargo.lock"),before=readFileSync(join(original,lockPath.slice(root.length+1)),"utf8"),current=readFileSync(lockPath,"utf8"),foreign=(text:string)=>text.split(/(?=^\[\[package\]\])/mu).filter(block=>/^source\s*=/mu.test(block));
  const oldForeign=foreign(before),newForeign=foreign(current);assert.ok(oldForeign.length>0);assert.deepEqual(newForeign,oldForeign);assert.deepEqual((Bun.TOML.parse(before) as any).package.filter((pkg:any)=>pkg.source),(Bun.TOML.parse(current) as any).package.filter((pkg:any)=>pkg.source));
  sourceChecks.push({artifact:fixture.artifact,coordinates:fixture.coordinates.length,aggregates:fixture.aggregates,fixtureAjvValidated:true,tomlOracleExact:true,foreignCount:oldForeign.length,foreignEntriesByteIdentical:true});
 }
 const originalPairs=[...retained.matchAll(/descriptors::<([^<>]+)>/gu)].map(match=>match[1].split(",").map(value=>value.trim()).join(",")).sort();assert.equal(originalPairs.length,10);assert.deepEqual([...new Set(aggregatePairs)].sort(),originalPairs);assert.equal(coordinates.length,10);
 const retainedCoordinates=[...retained.matchAll(/\("(s\.wfc\.[^"]+)", "([^"]+)", "([^"]+)", "([^"]*)", "([^"]+)", "[^"]*"\)/gu)].map(match=>({artifact:match[1],standard:match[2],subset:match[3],surface:match[4],owner:match[5]}));assert.equal(retainedCoordinates.length,10);const sortedCoordinates=(values:any[])=>values.map(value=>JSON.stringify([value.artifact,value.standard,value.subset,value.surface,value.owner])).sort();assert.deepEqual(sortedCoordinates(coordinates),sortedCoordinates(retainedCoordinates));
 const decode=(text:string)=>{const value=JSON.parse(text);assert.ok(validateInventory(value),JSON.stringify(validateInventory.errors));return value;};
 for(const text of ["noise\n{}","{}\n{}",'{"schema":"unrelated"}'])assert.throws(()=>decode(text));
 writeFileSync(join(output,"source-admission.json"),JSON.stringify({sourceChecks,sourceCount:sources.length,coordinates:coordinates.length,aggregatePairs:originalPairs,launchPreflight:true,producerNoiseRefused:true,wholeRootAccepted:false}));
 if(command==="run")writeFileSync(join(output,"admission.json"),JSON.stringify({sources,jobs:chosen,allCoordinates:jobs.filter((row:any)=>row.mode!=="check").length,wholeRootAccepted:false}));
 if(command==="admit")console.log("[DEBUG] WFC portable admission children="+sourceChecks.length+" coordinates="+coordinates.length+" sourceFiles="+sources.length);
 else
 for(const job of chosen){
  const owner=dirname(dirname(dirname(job.request.manifests[0]))),fixture=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧬️schema/🔣️.json"),"utf8"));
  assert.ok(ajv.compile(schema)(fixture));
  let at=0;
  const environment={...process.env,...job.environment};delete environment.NX_WORKSPACE_ROOT_PATH;
  const captureDirectory=JSON.parse(job.environment.SEMIO_MUTATION_INVENTORY_POLICY).compilerStorage.captureDirectory,beforeCaptures=new Set(existsSync(captureDirectory)?readdirSync(captureDirectory):[]);
  const result=await executeCommandV1({...job.request,command:process.execPath},job.policy,{environment,cancelled:()=>controller.signal.aborted,onProgress:event=>{if(Date.now()-at>=10000){at=Date.now();console.log("[DEBUG] WFC child "+job.artifact+" "+job.mode+" "+event.phase);}}});
  let inventory:any=null;
  if(result.status===0&&job.mode!=="check"){
   const created=readdirSync(captureDirectory).filter(name=>!beforeCaptures.has(name)&&name.startsWith("mutation-inventory-"));assert.equal(created.length,1,"One actual producer capture must be owned by this request");
   const receipt=JSON.parse(readFileSync(join(captureDirectory,created[0],"receipt.json"),"utf8"));assert.equal(receipt.reason,"exit");assert.equal(receipt.code,0);for(const key of ["processClosed","leaseReleased","stdoutClosed","stderrClosed"])assert.equal(receipt[key],true);
   const text=readFileSync(receipt.stdout.path,"utf8");assert.equal(Buffer.byteLength(text),receipt.stdout.bytes);inventory=decode(text);assert.ok(result.stdout.includes(text.trim()),"Nx envelope must retain the exact producer payload");
   const start=job.request.args.indexOf("--")+1,[artifact,standard,subset]=job.request.args.slice(start,start+3),surface=job.request.args[start+3].startsWith("--")?null:job.request.args[start+3];
   assert.equal(inventory.artifact,artifact);assert.equal(inventory.standard,standard);assert.equal(inventory.subset,subset);assert.equal(inventory.surface??null,surface);assert.equal(inventory.producedBy,fixture.producedBy);assert.ok(inventory.mutations.length);assert.equal(new Set(inventory.mutations.map((row:any)=>row.id)).size,inventory.mutations.length);
  }
  results.push({name:job.name,...result,inventory});
  writeFileSync(join(output,"results.json"),JSON.stringify(results));
  console.log("[DEBUG] WFC child "+job.artifact+" "+job.mode+" status="+result.status+" reason="+result.reason);
  if(result.status!==0&&selection!=="red")process.exitCode=1;
 }
}finally{
 const post=sourcePaths().map(capture),exact=sources.length===post.length&&sources.every((source,index)=>source.path===post[index].path&&source.sha256===post[index].sha256);
 const initial=new Map(sources.map(row=>[row.path,row.sha256])),terminal=new Map(post.map(row=>[row.path,row.sha256])),changed=post.filter(row=>initial.has(row.path)&&initial.get(row.path)!==row.sha256).map(row=>row.path),added=post.filter(row=>!initial.has(row.path)).map(row=>row.path),removed=sources.filter(row=>!terminal.has(row.path)).map(row=>row.path);
 writeFileSync(join(output,"terminal.json"),JSON.stringify({sources:sources.map(({path,sha256})=>({path,sha256})),post:post.map(({path,sha256})=>({path,sha256})),results,sourceChecks,exact,changed,added,removed,wholeRootAccepted:false}));
 console.log("[DEBUG] WFC child source custody exact="+exact+" jobs="+results.length);
 if(!exact)process.exitCode=1;
 process.off("SIGINT",stop);process.off("SIGTERM",stop);
}
