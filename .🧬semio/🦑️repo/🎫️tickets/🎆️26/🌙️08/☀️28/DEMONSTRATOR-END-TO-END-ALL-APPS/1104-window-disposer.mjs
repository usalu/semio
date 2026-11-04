import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import Ajv from "ajv";

const contract=JSON.parse(await readFile(new URL("./1104-window-disposer.json",import.meta.url),"utf8"));
assert(new Ajv({strict:true}).validate({"type":"object","required":["depth","copyBytes","releaseBytes","owners"],"properties":{"depth":{"const":1},"copyBytes":{"const":0},"releaseBytes":{"const":0},"owners":{"type":"array","minItems":6,"items":{"type":"object","required":["source","owner","concreteOwners"],"properties":{"source":{"type":"string"},"owner":{"type":"string"},"concreteOwners":{"type":"array","minItems":1,"items":{"type":"string"}}}}}}},contract));
const rows=[];
for(const row of contract.owners){
 const source=await readFile(row.source,"utf8"),factory=[...source.matchAll(/fn build_store_disposer\s*\(/g)].length,births=[...source.matchAll(/fn store_disposer_birth\s*\(/g)].length;
 rows.push({source:row.source,owner:row.owner,factory,births});
 assert.equal(factory,1,"Exact bounded owner factory required");
 assert(source.includes("bounded_window_config_store_disposer::<Self>()"),"Real bounded disposer required");
}
console.log(JSON.stringify(rows));
assert(rows.every(row=>row.births===row.factory),"Every declared bounded factory requires its explicit birth");
for(const row of contract.owners){
 const source=await readFile(row.source,"utf8");
 assert(/fn store_disposer_birth\s*\([^)]*\)\s*->\s*Result<semio_framework_value::RetirementDemand,\s*semio_framework_value::ValueError>\s*\{\s*semio_framework_plugin::bounded_window_config_store_disposer_birth::<Self>\(\)\s*\}/.test(source),"Exact matching bounded birth required");
}
console.log(JSON.stringify({sources:rows.length,concreteOwners:contract.owners.reduce((sum,row)=>sum+row.concreteOwners.length,0),status:"passed"}));
