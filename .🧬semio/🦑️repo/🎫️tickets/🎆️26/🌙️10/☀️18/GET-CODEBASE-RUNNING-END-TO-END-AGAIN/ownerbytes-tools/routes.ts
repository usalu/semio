/** 🛣️ Reads the native recovery plan of every authored route (frontend build and install, maintenance preview and retire, reset graph, nx, rust and products) from a checkout through the real revision reader and prints one JSON row per route. usage: bun routes.ts <root> */
import {join,resolve} from "node:path";
import {statSync} from "node:fs";

const root=resolve(process.argv[2]??"");
const cli=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/⌨️cli");
const {readNativeRepoRecoveryPlanV1}=await import(join(cli,"🟦️.ts")),{default:schema}=await import(join(cli,"🧬️schema/🔣️.json"),{with:{type:"json"}}),protocol=schema.$defs.Protocol.const;
const routes=[...protocol.targets.map((target:string)=>"repo-mcp:"+target),...Object.keys(protocol.maintenance.targets).map(target=>"repo:"+target),...Object.keys(protocol.reset.targets).map(target=>"repo:"+target)];
let failed=0;
for(const route of routes){
 const project=route.startsWith("repo-mcp:")?protocol.owner:protocol.reset.owner;
 try{const plan=readNativeRepoRecoveryPlanV1(root,["run",route]);console.log(JSON.stringify({route,kind:plan?.kind,target:plan?.target,ownerBytes:statSync(join(root,project,"📋️project.json")).size,ceiling:protocol.maximumOwnerBytes}));if(!plan)failed++;}
 catch(error){failed++;console.log(JSON.stringify({route,error:error instanceof Error?error.message:String(error)}));}
}
process.exit(failed?1:0);
