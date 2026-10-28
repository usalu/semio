import {join} from "node:path";
import {fenceMaterializedPathsV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {cacheTextV1} from "../../../🧾️ledger/🟦️.ts";
import policy from "../🔣️policy.json" with {type:"json"};

/** 🗺️ The live store of a checkout: every declared partition directory and the plain control directory beside them. @see ../🔣️policy.json */
export function repositoryStoreDirectoriesV1(checkout:string):string[]{return[...Object.values(policy.partitions).map(partition=>partition.directory),".cache/b/c"].map(directory=>join(checkout,directory));}
/** 🚧️ Fences the live store of a checkout for the rest of this process: every later image, lease or partition path inside it is refused in English and German, so a test that reaches it fails at the reach instead of queueing behind peers. */
export function fenceRepositoryStoreV1(checkout:string):void{fenceMaterializedPathsV1(repositoryStoreDirectoriesV1(checkout),cacheTextV1("en","store.fenced",{root:checkout})+"\n"+cacheTextV1("de","store.fenced",{root:checkout}));}
