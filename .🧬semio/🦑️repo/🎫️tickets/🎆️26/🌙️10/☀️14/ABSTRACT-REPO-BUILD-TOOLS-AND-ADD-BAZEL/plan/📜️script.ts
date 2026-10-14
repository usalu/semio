import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {readFile,writeFile} from "node:fs/promises";
import {isAbsolute,join} from "node:path";
import {createRequire} from "node:module";
const root=process.env.SEMIO_WORKSPACE_ROOT!,artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR!;
if(!root||!artifacts)throw Error("Workspace and ticket artifacts required");
const path=process.argv[2],expectedDigest=process.argv[3];
if(process.argv.length!==4||!path||!isAbsolute(path)||!expectedDigest||!/^[a-f0-9]{64}$/u.test(expectedDigest))throw Error("Explicit absolute immutable graph pin and SHA-256 digest required");
assert.equal(process.env.SEMIO_BUILD_GRAPH_PATH,path,"Public entry changed the original graph pin");assert.equal(process.env.SEMIO_BUILD_GRAPH_DIGEST,expectedDigest,"Public entry changed the original graph digest");
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {planBazelTasksV1}=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts"));
const {readBuildGraphV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts"));
const Ajv=createRequire(join(root,".nx/installation/package.json"))("ajv");
const schema=JSON.parse(await readFile(join(library,"🏗️build/🔧️tools/🏗️bazel/🧬️schema/🔣️.json"),"utf8")),validate=new Ajv({strict:true}).compile(schema);
let source=await readFile(path,"utf8");const digest=createHash("sha256").update(source).digest("hex");assert.equal(digest,expectedDigest);
const graph=readBuildGraphV1(JSON.parse(source));source="";Bun.gc(true);const began=performance.now(),unique=new Set<string>(),failures:{task:string;message:string}[]=[];
let planned=0,baseTasks=0,configuredTasks=0,expandedEntries=0,maximumClosure=0,cancelled=false;
const cancel=()=>{cancelled=true;};process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
try{
 for(const project of Object.values(graph.projects))for(const[name,target]of Object.entries(project.targets)){
  const base=project.name+":"+name;const selections=[base,...Object.keys(target.configurations??{}).map(configuration=>base+":"+configuration)];baseTasks++;configuredTasks+=selections.length-1;
  for(const task of selections){if(cancelled||performance.now()-began>300000)throw Error("All-task qualification cancelled or exhausted its five-minute deadline");try{const plan=planBazelTasksV1(graph,{tasks:[task]});assert(validate(plan),JSON.stringify(validate.errors));const ids=Object.keys(plan.tasks);expandedEntries+=ids.length;maximumClosure=Math.max(maximumClosure,ids.length);for(const id of ids)unique.add(id);}catch(error){failures.push({task,message:error instanceof Error?error.message:String(error)});}planned++;if(planned%250===0){console.log(`[DEBUG] Bazel plans checked ${planned} failures=${failures.length}`);await new Promise(resolve=>setImmediate(resolve));}}
 }
 const result={graph:{path,digest,projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length},baseTasks,configuredTasks,planned,expandedEntries,uniqueExpandedTasks:unique.size,maximumClosure,failures,elapsedMilliseconds:Math.round(performance.now()-began)};
 await writeFile(join(artifacts,"bazel-all-task-plan.json"),JSON.stringify(result,null,2)+"\n");console.log("[DEBUG] Bazel complete task qualification "+JSON.stringify(result));assert.equal(failures.length,0,"Bazel task plans failed");
}finally{process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
