// 🧪 Feeds the admission's closure walk the root project exactly as the derived static graph shapes it (every derived pattern and file as a literal input, the worst case of the interned production form).
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {buildSourceContractGroupsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),G=join(import.meta.dir,"..","🗑️generated","input-precision");
const defaults=JSON.parse(readFileSync(join(L,"🏗️build/🔣️policy.json"),"utf8")).namedInputs,graph=JSON.parse(readFileSync(join(G,"graph-merged.json"),"utf8"));
const manifest=JSON.parse(readFileSync(join(W,"📋️project.json"),"utf8"));
for(const project of Object.values<any>(graph.projects)){
 if(!["workspace","@semio-tech/print"].includes(project.name))continue;
 const targets=Object.fromEntries(Object.entries<any>(project.targets).map(([n,t])=>[n,{inputs:t.inputs??["default"]}])),literals=Object.values<any>(targets).reduce((s,t)=>s+t.inputs.length,0);
 try{const groups=buildSourceContractGroupsV1({name:project.name,targets,namedInputs:project.name==="workspace"?manifest.namedInputs:undefined,metadata:project.name==="workspace"?manifest.metadata:undefined} as any,defaults,[]);console.log(JSON.stringify({project:project.name,targets:Object.keys(targets).length,inputEntries:literals,accepted:true,contractGroups:groups.length}));}
 catch(error:any){console.log(JSON.stringify({project:project.name,inputEntries:literals,accepted:false,error:error.message}));}
}
process.exit(0);
