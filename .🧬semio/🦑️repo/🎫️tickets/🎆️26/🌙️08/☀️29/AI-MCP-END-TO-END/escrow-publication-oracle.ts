import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const fixture=JSON.parse(readFileSync(`${import.meta.dirname}/escrow-publication-cases.json`,"utf8"));
const schema=JSON.parse(readFileSync(`${import.meta.dirname}/escrow-publication-cases.schema.json`,"utf8"));
const valid=new Ajv2020({strict:true}).compile(schema);if(!valid(fixture))throw Error(JSON.stringify(valid.errors));
const db=new Database(":memory:");const query=db.query("select ?1+case when ?3 and ?4 and (?1=?2 or ?5) then 1 else 0 end as publication,case when ?5 then ?1 else ?2 end as nextAck");
for(const row of fixture.cases){const actual=query.get(row.previous,row.ack,+row.frozen,+row.funded,+row.handback) as {publication:number;nextAck:number};if(actual.publication!==row.publication||actual.nextAck!==row.nextAck||row.ack>row.previous||row.handback&&row.previous===0)throw Error(JSON.stringify({row,actual}));}
console.log(`SQLite publication serial oracle passed ${fixture.cases.length} cases`);
