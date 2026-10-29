import {statfsSync} from "node:fs";
import {readNativeRepoRecoveryPlanV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli/🟦️.ts";
import {repositoryCacheBudgetV1,repositoryPartitionRegrowDueV1,elasticCapacityV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import policy from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json";
const root="/Users/ueli/Documents/semio";
for(const t of ["repo-cache-reset:graph","repo-cache-reset:nx","repo-cache-reset:rust","repo-cache-reset:products","repo-cache-retire:inactive-preview","repo-cache-retire:inactive","repo-cache-reset:dependencies","repo:cache-reset-rust"]){try{console.log("plan",t,JSON.stringify(readNativeRepoRecoveryPlanV1(root,["run",t])??null));}catch(e){console.log("plan",t,"REFUSED:",(e as Error).message);}}
const b=repositoryCacheBudgetV1(root);
console.log("rust budget (receipt-pinned)",JSON.stringify(b.partitions.rust),"regrowDue",repositoryPartitionRegrowDueV1(root,"rust"));
const s=statfsSync(root);const free=s.bavail*s.bsize;
console.log("hostFreeGiB",free/2**30,"elastic size a new image would get GiB",elasticCapacityV1(policy.elastic.rust,policy.partitions.rust.capacityBytes,free)/2**30);
