/** usage (inside an issued envelope, see recreate-run.ts): bun recreate-child.ts <dry|apply> <partition>... ; recreates fixed images sparse through the official quiescent reset; a live holder makes the exclusive try fail and the partition is skipped. */
import {statSync} from "node:fs";
import {join} from "node:path";
import {receiveScriptProcessInvocation} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
import {acquireResourceLease} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {resetInactiveRepositoryCacheNamespaceV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts";
import {repositoryCacheBudgetV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
const [mode,...partitions]=process.argv.slice(2),root="/Users/ueli/Documents/semio";
const size=(path:string)=>{try{const row=statSync(path);return {logicalGiB:+(row.size/2**30).toFixed(2),allocatedGiB:+(row.blocks*512/2**30).toFixed(2)};}catch{return null;}};
await receiveScriptProcessInvocation(process.env,async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"store-g:recreate",3600000),budget=repositoryCacheBudgetV1(root);
 for(const id of partitions){
  const declaration=(budget.partitions as Record<string,{directory:string}>)[id];if(!declaration){console.log(JSON.stringify({id,verdict:"unknown-partition"}));continue;}
  const image=join(declaration.directory,"fixed.dmg"),before=size(image);let verdict="free";
  try{(await acquireResourceLease({directory:repositoryLeaseDirectoryV1(root),resource:"native-fixed-image:"+declaration.directory,mode:"exclusive",signal:control.signal,control,timeoutMs:0})).release();}catch(error){verdict=String(error).includes("Resource lease timed out")?"held":"unavailable: "+String(error).slice(0,120);}
  if(mode!=="apply"||verdict!=="free"){console.log(JSON.stringify({id,mode,verdict,before}));continue;}
  const outcome=await resetInactiveRepositoryCacheNamespaceV1(original,control,id as "graph"|"rust"|"products");
  console.log(JSON.stringify({id,mode,verdict:"reset",outcome,before,after:size(image)}));
 }
});
