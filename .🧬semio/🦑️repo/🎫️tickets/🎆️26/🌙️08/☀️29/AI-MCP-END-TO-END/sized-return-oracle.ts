import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const law=JSON.parse(readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/sized-case.json","utf8"));
const schema=JSON.parse(readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/sized-case.schema.json","utf8"));
const valid=new Ajv2020({strict:true}).compile(schema);if(!valid(law))throw Error(JSON.stringify(valid.errors));
const db=new Database(":memory:");
const calculate=db.query("select (?1 <= ?2) as admitted, case when ?1 <= ?2 then ?1 else 0 end as held, ?2 - case when ?1 <= ?2 then ?1 else 0 end as remaining, case when ?1 <= ?2 and ?3 then ?1 else 0 end as spent");
for(const row of law.cases){const actual=calculate.get(row.requested,law.grantBytes,row.captured?1:0) as {admitted:number,held:number,remaining:number,spent:number};if(Boolean(actual.admitted)!==row.admitted||actual.held!==row.heldDuring||actual.remaining!==row.remainingDuring||actual.spent!==row.spent||row.held!==0)throw Error(JSON.stringify({row,actual}));}
console.log(`SQLite conservation oracle passed ${law.cases.length} cases`);
