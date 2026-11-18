import {dump} from "./fscount.ts";
import {createRequire} from "node:module";
import {writeFileSync,appendFileSync} from "node:fs";
import {join} from "node:path";
import {closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,buildInvocationGraphSelectionV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const require=createRequire(import.meta.url);
const root=process.env.SEMIO_WORKSPACE_ROOT!,out=process.env.R140_OUT!;

{
 const owner:any=await import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
 const registry=readBuildOwnerRegistryV1(JSON.parse(owner.readBuildOwnerRegistrySourceV1(root)));
 const selection=buildInvocationGraphSelectionV1(["run-many","--projects=@semio-tech/norm-din4108,@semio-tech/norm-din4108-rs,repo-mcp","--targets=build"]);
 const nativeCache=new Map(),nativeClosures=new Map(),rows:any[]=[];const t0=performance.now();
 const admission=await closeBuildOwnerAdmissionV1(selection,registry,async(o,targets,inputs)=>{const s=performance.now(),c=process.cpuUsage();const v=await owner.discoverBuildOwnerAdmissionFactsV1(o,targets,registry,{workspaceRoot:root,selection,nativeCache,nativeClosures},inputs);const u=process.cpuUsage(c);const row={owner:o.name,targets:targets.length,inputs:inputs.length,ms:Math.round(performance.now()-s),cpuMs:Math.round((u.user+u.system)/1000),files:v.files.length,deps:v.dependencies.length};rows.push(row);appendFileSync(join(out,"rows.jsonl"),JSON.stringify(row)+"\n");return v;});
 const total=Math.round(performance.now()-t0);
 dump(join(out,"fscount.json"));writeFileSync(join(out,"probe.json"),JSON.stringify({total,registryOwners:registry.owners.length,closureOwners:admission.owners.length,files:admission.files.length,calls:rows.length,rows,owners:admission.owners},null,1));
}
