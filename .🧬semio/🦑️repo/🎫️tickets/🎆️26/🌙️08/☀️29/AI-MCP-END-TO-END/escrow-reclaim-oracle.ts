import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const law=JSON.parse(readFileSync(`${import.meta.dirname}/escrow-reclaim-cases.json`,"utf8"));const schema=JSON.parse(readFileSync(`${import.meta.dirname}/escrow-reclaim-cases.schema.json`,"utf8"));const valid=new Ajv2020({strict:true}).compile(schema);if(!valid(law))throw Error(JSON.stringify(valid.errors));
const query=new Database(":memory:").query("select ?1 and not ?2 and not ?3 as reclaim");for(const row of law.cases){const actual=query.get(+row.ack,+row.borrowed,+row.staged) as {reclaim:number};if(Boolean(actual.reclaim)!==row.reclaim)throw Error(JSON.stringify({row,actual}));}console.log(`SQLite borrowed tombstone oracle passed ${law.cases.length} cases`);
