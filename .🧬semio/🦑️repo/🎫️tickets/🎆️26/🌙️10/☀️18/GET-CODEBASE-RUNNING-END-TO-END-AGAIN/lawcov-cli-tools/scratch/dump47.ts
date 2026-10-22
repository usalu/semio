import {readFileSync,writeFileSync} from "node:fs";
import {buildTestkitCrate,planTestkitWiring} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧰️testkit/🟦️.ts";
const cases=JSON.parse(readFileSync(new URL("u47_cases.json",import.meta.url),"utf8"));
const out:any={wiring:[]};
for(const row of cases.wiring){
 const crates=await Promise.all(row.crates.map((c:any)=>buildTestkitCrate({name:c.name,ident:c.ident,dependsOn:c.dependsOn,devDependsOn:c.devDependsOn,ownedFiles:c.owned===null?undefined:new Set<string>(c.owned),files:new Map(Object.entries<string>(c.files))},async(f:string)=>new Set<string>(c.featureFiles[f]??[]))));
 const w=planTestkitWiring(crates,row.externals,new Set<string>(row.declared)),shown:any={};
 for(const[n,e]of[...w].sort())shown[n]={features:e.features.map((f:any)=>({name:f.name,entries:[...f.entries]})),optionalDependencies:[...e.optionalDependencies],consumed:Object.fromEntries([...e.consumed].sort().map(([d,f]:any)=>[d,[...f]])),gatedNames:Object.fromEntries([...e.gatedNames].sort().map(([f,n2]:any)=>[f,[...n2].sort()]))};
 out.wiring.push(shown);
}
writeFileSync(new URL("../gen/u47_goldens.json",import.meta.url),JSON.stringify(out,null,1));
