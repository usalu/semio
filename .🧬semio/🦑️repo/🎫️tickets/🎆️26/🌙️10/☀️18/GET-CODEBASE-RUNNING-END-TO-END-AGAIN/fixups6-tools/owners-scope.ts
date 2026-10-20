import {checkBuildOwnerRegistry,readOwnerTree,findOwnerFindings,syncOwnerRegistry,OWNER_TREE_POLICY,BUILD_OWNER_REGISTRY_FILE} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
import {readFileSync} from "node:fs";
const ROOT="/Users/ueli/Documents/semio";
const scope=process.argv.slice(2);
const r=checkBuildOwnerRegistry(ROOT,scope.length?scope:undefined);
console.log("owners",r.owners,"findings",r.findings.length);
for(const f of r.findings)console.log(JSON.stringify(f).slice(0,400));
if(process.env.SYNC){
  const registry=JSON.parse(readFileSync(ROOT+"/"+BUILD_OWNER_REGISTRY_FILE,"utf8"));
  const tree=readOwnerTree(ROOT,OWNER_TREE_POLICY,scope.length?scope:undefined);
  const s=syncOwnerRegistry(registry,tree,OWNER_TREE_POLICY,scope.length?scope:undefined);
  console.log("SYNC",JSON.stringify({added:s.added?.map((x:any)=>x.root??x),removed:s.removed?.length,moved:s.moved?.length,keys:Object.keys(s)}).slice(0,1500));
}
