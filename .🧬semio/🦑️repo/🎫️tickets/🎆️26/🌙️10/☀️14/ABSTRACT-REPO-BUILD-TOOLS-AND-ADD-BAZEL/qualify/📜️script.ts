import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFile,writeFile} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
const root=process.env.SEMIO_WORKSPACE_ROOT??process.cwd(),ticket=resolve(import.meta.dir,".."),output=join(ticket,"🗑️generated"),began=performance.now();
const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const {createScriptProcessEnvelope,withScriptProcessEnvelope,checkScriptInvocation,advanceScriptInvocation}=await import(join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"));
if(process.argv.includes("--quiet")){
 const sources=["🏗️build/🕸️graph/🟦️.ts","🏗️build/🧬️schema/📣️publication.json","🏗️build/🧭️invocation/🟦️.ts","🏗️build/📜️script.ts","⚡️caching/🚀️bootstrap/📥️admission/🟦️.ts"];
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-source-quiet",maximumElapsedMilliseconds:180000},{workspaceRoot:root},Date.now()),async original=>{
  let previous="",since=performance.now();
  for(;;){
   checkScriptInvocation(original);const bytes=await Promise.all(sources.map(path=>readFile(join(library,path),"utf8"))),fingerprints=Object.fromEntries(sources.map((path,index)=>[path,createHash("sha256").update(bytes[index]!).digest("hex")])),image=JSON.stringify(fingerprints),ready=bytes[2]!.includes("export function readBuildDependencyInvocationV1(")&&bytes[4]!.includes("export async function boundedAdmissionWalkV1<");
   if(image!==previous||!ready){previous=image;since=performance.now();console.error("[DEBUG] source-quiet-change "+JSON.stringify({ready,fingerprints}));}
   if(performance.now()-since>=30000){const result={quietMilliseconds:Math.round(performance.now()-since),elapsedMilliseconds:Math.round(performance.now()-began),fingerprints};await writeFile(join(output,"source-quiet.json"),JSON.stringify(result,null,2)+"\n");console.error("[DEBUG] source-quiet-ready "+JSON.stringify(result));return;}
   await advanceScriptInvocation(original,"build:source-quiet","running");await delay(5000,undefined,{signal:original.control.signal});
  }
 });
 process.exit(0);
}
const {createBuildBootstrapCapabilitiesV1,runOwnedBuildInvocationV1}=await import(join(library,"🏗️build/📜️script.ts"));
const observation=createRequire(import.meta.url)(join(library,"🏗️build/👁️observation/🟨️.mjs")),args=["show","targets","--file="+join(output,"qualified-targets.json")],counts:Record<string,number>={};
let phase="start",phaseStart=began,refreshes=0;
const status=(reason:string)=>console.error("[DEBUG] graph-qualification "+JSON.stringify({reason,phase,phaseMilliseconds:Math.round(performance.now()-phaseStart),elapsedMilliseconds:Math.round(performance.now()-began),pid:process.pid,memory:process.memoryUsage(),observations:observation.sourceObservationStatsV1(),counts}));
const envelope=createScriptProcessEnvelope({version:1,owner:"repository:build",maximumElapsedMilliseconds:1200000},await createBuildBootstrapCapabilitiesV1(root,args,process.env),Date.now());
status("start");
await withScriptProcessEnvelope(envelope,async original=>{
 const invocation={...original,control:{...original.control,publish:async (event:any)=>{const key=event.command+":"+event.stage;counts[key]=(counts[key]??0)+1;if(phase!==key){status("phase-complete");phase=key;phaseStart=performance.now();status("phase-start");}else if(counts[key]%32===0)status("progress");await original.control.publish(event);if(event.command==="build:refresh"&&++refreshes>=2)throw Error("Qualification guard refused a third candidate after two real invalidations");}}};
 await runOwnedBuildInvocationV1(root,args,invocation);
status("published");
const {buildGraphPathV1,readBuildGraphV1,buildGraphProducerAuthorityV1}=await import(join(library,"🏗️build/🕸️graph/🟦️.ts")),{withRepositoryBuildGraphArtifactV1}=await import(join(library,"⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts")),canonicalPath=buildGraphPathV1(root),receipt=JSON.parse(await readFile(join(dirname(canonicalPath),"authority.json"),"utf8")),graph=readBuildGraphV1(JSON.parse(await readFile(canonicalPath,"utf8"))),module={implementation:createHash("sha256").update(await readFile(join(library,"🏗️build/🕸️graph/🟦️.ts"))).digest("hex"),publication:createHash("sha256").update(await readFile(join(library,"🏗️build/🧬️schema/📣️publication.json"))).digest("hex")};
if(!buildGraphProducerAuthorityV1(receipt,module))throw Error("Current loaded graph producer authority required");
await withRepositoryBuildGraphArtifactV1(root,graph,invocation,async snapshot=>{
const path=snapshot.path,source=await readFile(path,"utf8");
if(snapshot.digest!==receipt.graph)throw Error("Published canonical graph and source authority differ");
if(createHash("sha256").update(source).digest("hex")!==receipt.graph)throw Error("Published immutable graph bytes differ from its authority digest");
const summary={path,digest:receipt.graph,bytes:Buffer.byteLength(source),projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length,targets:Object.values(graph.projects).reduce((sum:number,project:any)=>sum+Object.keys(project.targets).length,0),observations:receipt.observations.length,membership:receipt.membership,authorityVersion:receipt.version,module:receipt.module,elapsedMilliseconds:Math.round(performance.now()-began),counts};
await writeFile(join(output,"qualified-graph-current.json"),JSON.stringify(summary,null,2)+"\n");console.error("[DEBUG] graph-publication "+JSON.stringify(summary));
});
});
