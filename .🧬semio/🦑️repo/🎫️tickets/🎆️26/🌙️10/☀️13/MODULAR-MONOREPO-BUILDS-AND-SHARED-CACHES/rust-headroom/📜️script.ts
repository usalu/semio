import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {withHistoricalCargoImageMaintenanceV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/📜️history/🟦️.ts";
import {withCompilerWriterCustodyV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {retireInactiveCargoStoreV1,CARGO_RETIREMENT_POLICY_V1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🏪️budget/⚙️execution/🟦️.ts";
import {cargoTargetDirectory} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../.."),command=process.argv[2],mode=process.argv[3];
if(process.argv.length!==4||!(command==="preview"||command==="retire")||!(mode==="prototype"||mode==="release"))throw Error("Exact historical Cargo prerequisite invocation required");
const selection=["nx","run","repo-private-rust-headroom:"+command,"--configuration="+mode],policy={version:1 as const,owner:"repository:historical-cargo-headroom",maximumElapsedMilliseconds:120000},capabilities={workspaceRoot:root,buildMode:mode,arguments:selection,selection,launches:[selection],native:{artifactDirectory:cargoTargetDirectory(root),transport:{maximumBytes:16777216,maximumLines:65536},child:{maximumElapsedMilliseconds:120000},network:{offline:true}}};
await withScriptProcessEnvelope(createScriptProcessEnvelope(policy,capabilities,Date.now()),async invocation=>{
 const control=scriptCompilerAdmissionControlV1(invocation,"cache:historical-cargo-headroom",120000),preview=command==="preview";
 const result=await withHistoricalCargoImageMaintenanceV1(root,invocation,control,{version:1,mode,preview},(image,buildDirectory,targetDirectory,heldControl)=>withCompilerWriterCustodyV1(image,join(root,".cache/b/l/m/data"),heldControl,{version:1,wait:false},()=>retireInactiveCargoStoreV1({workspace:root,buildDirectory,relatedDirectories:[targetDirectory],leaseDirectory:join(root,".cache/b/l/m/data"),artifactDirectory:join(import.meta.dir,"../🗑️generated/root-rust-headroom"),invocation,control:heldControl,policy:CARGO_RETIREMENT_POLICY_V1,preview,wait:false})));
 console.log("[DEBUG] Actual authenticated historical Cargo prerequisite completed "+JSON.stringify({command,mode,...result}));
});
