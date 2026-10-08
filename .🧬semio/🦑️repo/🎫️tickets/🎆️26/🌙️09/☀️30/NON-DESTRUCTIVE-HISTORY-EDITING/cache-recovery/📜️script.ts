/** 🔎️ Read-only canonical cache policy inventory under the actual debug compiler lease. */
import {writeFile} from "node:fs/promises";
import {resolve} from "node:path";
import {acquireCargoBuildLeaseV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {cargoDirectories} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import {scanCacheAreas} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🌐️workspace/🟦️.ts";
import {planCachePrune} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧹️pruning/🟦️.ts";
import {CACHE_POLICY} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔍️discovery/📂️source/🟦️.ts";
const workspace=resolve(import.meta.dir,"../../../../../../../..");
if(import.meta.main){
 const controller=new AbortController(),cancel=()=>controller.abort();process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
 const lease=await acquireCargoBuildLeaseV1({directory:resolve(workspace,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),buildDirectory:cargoDirectories(workspace).build,args:["--profile","dev"],signal:controller.signal,onWait:wait=>console.log(`[DEBUG] cache audit compiler lease wait=${wait.elapsedMs}`)});
 try{
  const areas=scanCacheAreas(workspace,controller.signal),plan=planCachePrune(areas,Date.now(),CACHE_POLICY.storage.guardAgeMs);
  await writeFile(resolve(import.meta.dir,"../🗑️generated/ui-execution/oct8-full-cache-plan.json"),JSON.stringify(plan,null,2)+"\n");
  for(const area of plan.areas)console.log(`[DEBUG] cache audit area=${area.name} bytes=${area.totalBytes} units=${area.unitCount} ageCandidates=${area.ageDeletions.length} budgetCandidates=${area.budgetDeletions.length} candidateBytes=${[...area.ageDeletions,...area.budgetDeletions].reduce((sum,unit)=>sum+unit.bytes,0)} guardedBytes=${area.guardedOverBudgetBytes}`);
 }finally{lease.release();process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
}
