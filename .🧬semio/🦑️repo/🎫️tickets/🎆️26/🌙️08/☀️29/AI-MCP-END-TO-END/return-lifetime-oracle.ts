import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const base=import.meta.dirname;
const fixture=JSON.parse(readFileSync(`${base}/return-lifetime-cases.json`,"utf8"));
const schema=JSON.parse(readFileSync(`${base}/return-lifetime-cases.schema.json`,"utf8"));
const validate=new Ajv2020({strict:true}).compile(schema);
if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
const query=db.query("select case when ?1 < 3 or ?2 < ?4 then 'ownership' when ?3 < 3 then 'work' when ?2-?7 < ?4 or ?5 < ?6 or ?1-?8<3 or ?3-?9<3 then 'pending' else 'admit' end as result");
for(const row of fixture.cases){const actual=query.get(row.items,row.copy,row.work,row.requiredCopy,row.currentCopy,fixture.rowCopy,row.heldCopy,row.heldItems,row.heldWork) as {result:string};if(actual.result!==row.result)throw Error(JSON.stringify({row,actual}));}
console.log(`SQLite lifetime preflight oracle passed ${fixture.cases.length} cases`);
