import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,selection,epoch]=process.argv.slice(2);
assert.ok(command==="run"&&selection&&epoch&&/^\d+$/u.test(epoch));
const output=join(ticket,"🗑️generated/wfc-child-inventory",selection+"-"+epoch);
assert.ok(!existsSync(output));mkdirSync(output,{recursive:true});
const jobs=JSON.parse(readFileSync(join(import.meta.dir,"execution-jobs.json"),"utf8"));
const chosen=selection==="red"?jobs.filter((job:any)=>job.artifact==="grid2d"&&job.mode==="check"):selection==="all"?jobs:jobs.filter((job:any)=>job.artifact===selection);
assert.ok(chosen.length);
const paths:string[]=[];
const walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true})){const path=join(directory,entry.name);if(entry.isSymbolicLink())throw Error("WFC evidence refuses symbolic source");if(entry.isDirectory()){if(!["target","dist","node_modules","🗑️generated",".git"].includes(entry.name))walk(path);}else if(/\.(?:tsx?|rs|toml|json|lock)$/u.test(path))paths.push(path);}};
walk(join(root,"✏️s/🔌️plugins/🀄️wfc"));
paths.push(import.meta.path,join(import.meta.dir,"execution-jobs.json"),join(import.meta.dir,"launch-additions.json"));
const capture=(path:string)=>{const source=readFileSync(path,"utf8");return {path,source,sha256:createHash("sha256").update(source).digest("hex")};};
const sources=paths.map(capture),controller=new AbortController(),stop=()=>controller.abort(),results:any[]=[];
process.once("SIGINT",stop);process.once("SIGTERM",stop);
const {executeCommandV1}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts"));
try{
 const ajv=new Ajv({strict:false});
 for(const job of chosen){
  const owner=dirname(dirname(dirname(job.request.manifests[0]))),fixture=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🏭️bridge/🧬️schema/🔣️.json"),"utf8"));
  assert.ok(ajv.compile(schema)(fixture));
  const launch=JSON.parse(readFileSync(join(root,".vscode/launch.json"),"utf8")).configurations.find((row:any)=>row.name===job.name);
  assert.ok(launch,"exact child launch row missing: "+job.name);
  assert.equal(launch.command,"bun "+job.request.args.join(" "));
  for(const [key,value]of Object.entries(job.environment))assert.equal(launch.env[key].replaceAll("${workspaceFolder}",root),value);
  writeFileSync(join(output,"admission.json"),JSON.stringify({sources,jobs:chosen,allCoordinates:jobs.filter((row:any)=>row.mode!=="check").length,wholeRootAccepted:false}));
  let at=0;
  const environment={...process.env,...job.environment};delete environment.NX_WORKSPACE_ROOT_PATH;
  const result=await executeCommandV1({...job.request,command:process.execPath},job.policy,{environment,cancelled:()=>controller.signal.aborted,onProgress:event=>{if(Date.now()-at>=10000){at=Date.now();console.log("[DEBUG] WFC child "+job.artifact+" "+job.mode+" "+event.phase);}}});
  results.push({name:job.name,...result});
  writeFileSync(join(output,"results.json"),JSON.stringify(results));
  console.log("[DEBUG] WFC child "+job.artifact+" "+job.mode+" status="+result.status+" reason="+result.reason);
  if(result.status!==0&&selection!=="red")process.exitCode=1;
 }
}finally{
 const post=paths.map(capture),exact=sources.every((source,index)=>source.sha256===post[index].sha256);
 writeFileSync(join(output,"terminal.json"),JSON.stringify({sources,post,results,exact,wholeRootAccepted:false}));
 console.log("[DEBUG] WFC child source custody exact="+exact+" jobs="+results.length);
 if(!exact)process.exitCode=1;
 process.off("SIGINT",stop);process.off("SIGTERM",stop);
}
