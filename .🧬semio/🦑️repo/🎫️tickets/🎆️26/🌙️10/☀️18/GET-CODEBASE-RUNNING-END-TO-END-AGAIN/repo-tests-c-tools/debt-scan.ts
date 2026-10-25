import {loadOracleRegistry,oracleLinkedPackages} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import {readFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio";
const reg=loadOracleRegistry(root);
const base=JSON.parse(readFileSync(root+"/🔒️dependencies.json","utf8")).entries as any[];
const out:any[]=[];
for(const o of reg.oracles){
  for(const l of oracleLinkedPackages(o)){
    const e=base.find(c=>c.name===l.package);
    if(!e){out.push({id:o.id,pkg:l.package,problem:"absent"});continue}
    if(o.productionDebt===undefined&&(e.productionReachable||JSON.stringify(e.kinds)!=='["test-oracle"]'))out.push({id:o.id,pkg:l.package,kinds:e.kinds,reach:e.productionReachable,users:e.users.slice(0,6)});
  }
}
console.log("[prof]",JSON.stringify(out,null,1));
