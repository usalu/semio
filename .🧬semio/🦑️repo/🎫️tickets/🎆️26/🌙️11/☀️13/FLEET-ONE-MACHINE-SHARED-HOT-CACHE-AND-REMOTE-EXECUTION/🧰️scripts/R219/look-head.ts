import {join} from "node:path";
import {createNodeFilePortV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
import {localGenerationIdentityV1,remoteGenerationExchangeV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import {repositoryCacheBudgetV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";

const root=process.argv[2]??"/Users/ueli/Documents/semio",given=process.argv[3],files=createNodeFilePortV1(),store=join(repositoryCacheBudgetV1(root).partitions.graph.directory,"m","generations");
let identity=given;
if(identity===undefined)try{identity=await localGenerationIdentityV1(store,files);}catch(error){console.log(JSON.stringify({state:"no-local-generation",hint:"pass the registry identity printed by the sealing machine",reason:String((error as Error).message)}));process.exit(0);}
const exchange=remoteGenerationExchangeV1({environment:process.env,workspace:root,identity:async()=>identity!,platform:process.argv[4]});
if(!exchange){console.log(JSON.stringify({state:"no-fleet-cache"}));process.exit(0);}
const look=await exchange.look({root:store,fs:files});
console.log(JSON.stringify({identity,...look}));
