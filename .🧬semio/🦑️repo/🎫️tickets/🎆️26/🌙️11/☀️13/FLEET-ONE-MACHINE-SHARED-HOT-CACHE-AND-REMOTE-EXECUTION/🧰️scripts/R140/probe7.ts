import {writeFileSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,buildInvocationGraphSelectionV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const budget=Number(process.env.R140_BUDGET_MS??0),root=process.env.SEMIO_WORKSPACE_ROOT!,out=process.env.R140_OUT!,projects=process.env.R140_PROJECTS??"@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp";
const owner:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
const observation:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs");
const registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
const selection=buildInvocationGraphSelectionV1(["run-many","--projects="+projects,"--targets=build"]);
let t0=performance.now();const nativeCache=new Map(),nativeClosures=new Map(),rows:any[]=[],failures:any[]=[];
const context={workspaceRoot:root,get invocation(){if(budget>0&&performance.now()-t0>budget){console.error("[R140] budget");writeFileSync(join(out,"partial.json"),JSON.stringify({rows,failures}));process.exit(0);}return undefined;},selection,nativeCache,nativeClosures};
const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>closeBuildOwnerAdmissionV1(selection,registry,async(o:any,targets:readonly string[],inputs:readonly string[])=>{
 const s=performance.now(),c=process.cpuUsage();let v:any,failed="";
 try{v=await owner.discoverBuildOwnerAdmissionFactsV1(o,targets,registry,context,inputs);}catch(error:any){failed=String(error?.message??error).slice(0,200);failures.push({owner:o.name,targets:targets.length,failed});v=await owner.discoverBuildOwnerAdmissionFactsV1(o,[],registry,context,[]);}
 const u=process.cpuUsage(c),row={owner:o.name,native:o.manifest.endsWith("Cargo.toml"),targets:targets.length,inputs:inputs.length,ms:Math.round(performance.now()-s),cpuMs:Math.round((u.user+u.system)/1000),files:v.files.length,deps:v.dependencies.length,failed};
 rows.push(row);appendFileSync(join(out,"rows.jsonl"),JSON.stringify(row)+"\n");return v;
})));
const memo:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧠️admission-memo/🟦️.ts"),workspace:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts");
const memoPath=join(out,"probe.admission.json"),scope="s".repeat(8),w0=performance.now();
const wrote=await memo.writeAdmissionMemoV1(memoPath,{version:1,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},scope,admission:captured.value,observations:captured.observations},1<<30);
const writeMs=Math.round(performance.now()-w0),cpu1=process.cpuUsage(),r0=performance.now();
const hit=await memo.readAdmissionMemoV1(memoPath,scope,()=>true,(rows:any)=>workspace.buildGraphSourceFactsHoldV1(rows));
const used=process.cpuUsage(cpu1),warm={wrote,writeMs,hit:hit!==undefined,readHoldMs:Math.round(performance.now()-r0),readHoldCpuMs:Math.round((used.user+used.system)/1000)};
const obsBytes=Buffer.byteLength(JSON.stringify(captured.observations)),obsCount=captured.observations.length,admission=captured.value as any,total=Math.round(performance.now()-t0);
writeFileSync(join(out,"probe.json"),JSON.stringify({warm,obsBytes,obsCount,total,registryOwners:registry.owners.length,closureOwners:admission.owners.length,files:admission.files.length,calls:rows.length,failures,rows,owners:admission.owners},null,1));
