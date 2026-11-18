import {writeFileSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,buildInvocationGraphSelectionV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const budget=Number(process.env.R141_BUDGET_MS??0),root=process.env.SEMIO_WORKSPACE_ROOT!,out=process.env.R141_OUT!,projects=process.env.R141_PROJECTS??"@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp";
const owner:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
const observation:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
const selection=buildInvocationGraphSelectionV1(["run-many","--projects="+projects,"--targets=build"]);
let t0=performance.now();const nativeCache=new Map(),nativeClosures=new Map(),rows:any[]=[],failures:any[]=[];
const context={workspaceRoot:root,get invocation(){if(budget>0&&performance.now()-t0>budget){console.error("[R141] budget");writeFileSync(join(out,"partial.json"),JSON.stringify({rows,failures}));process.exit(0);}return undefined;},selection,nativeCache,nativeClosures};
const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>closeBuildOwnerAdmissionV1(selection,registry,async(o:any,targets:readonly string[],inputs:readonly string[])=>{
 const s=performance.now(),c=process.cpuUsage();let v:any,failed="";
 try{v=await owner.discoverBuildOwnerAdmissionFactsV1(o,targets,registry,context,inputs);}catch(error:any){failed=String(error?.message??error).slice(0,200);failures.push({owner:o.name,targets:targets.length,failed});v=await owner.discoverBuildOwnerAdmissionFactsV1(o,[],registry,context,[]);}
 const u=process.cpuUsage(c),row={owner:o.name,native:o.manifest.endsWith("Cargo.toml"),targets:targets.length,inputs:inputs.length,ms:Math.round(performance.now()-s),cpuMs:Math.round((u.user+u.system)/1000),files:v.files.length,deps:v.dependencies.length,failed};
 rows.push(row);if(targets.length||inputs.length)appendFileSync(join(out,"files.jsonl"),JSON.stringify({owner:o.name,targets,inputs,files:v.files,observed:v.observedFiles??[],deps:v.dependencies})+"\n");appendFileSync(join(out,"rows.jsonl"),JSON.stringify(row)+"\n");return v;
})));
const obsBytes=Buffer.byteLength(JSON.stringify(captured.observations)),obsCount=captured.observations.length,admission=captured.value as any,total=Math.round(performance.now()-t0);
writeFileSync(join(out,"probe.json"),JSON.stringify({obsBytes,obsCount,total,registryOwners:registry.owners.length,closureOwners:admission.owners.length,files:admission.files.length,calls:rows.length,failures,rows,owners:admission.owners},null,1));
