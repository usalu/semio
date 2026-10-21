// 🧪 Proves the oversize cause and the fix: a project whose command-source groups hold 100k literal entries is refused by the closure walk when the groups are expanded, and accepted through the real projectInputs path.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {buildSourceContractGroupsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const defaults=JSON.parse(readFileSync(join(L,"🏗️build/🔣️policy.json"),"utf8")).namedInputs;
const groups=Object.fromEntries(Array.from({length:30},(_,g)=>[`commandSources${g}`,Array.from({length:3500},(_,i)=>`{workspaceRoot}/f${g}/${i}.ts`)]));
const project={name:"synthetic",targets:Object.fromEntries(Object.keys(groups).map(n=>[`t-${n}`,{inputs:["default",n]}]))};
try{buildSourceContractGroupsV1({...project,namedInputs:groups} as any,defaults,[]);console.log("expanded: accepted");}catch(error:any){console.log("expanded groups:",error.message);}
const emptied=Object.fromEntries(Object.keys(groups).map(n=>[n,[]]));
console.log("emptied groups:",JSON.stringify(buildSourceContractGroupsV1({...project,namedInputs:emptied} as any,defaults,[])));
process.exit(0);
