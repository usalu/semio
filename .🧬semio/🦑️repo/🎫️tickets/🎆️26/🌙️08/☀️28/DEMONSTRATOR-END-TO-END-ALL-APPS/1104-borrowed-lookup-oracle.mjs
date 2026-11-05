import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔎️lookup/🪞️borrowed/🧪️tests";
const cases=JSON.parse(readFileSync("🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧪️tests/🧫️fixtures/🔣️ordered-map.json","utf8")).borrowedLookupCases;const valid=new Ajv({strict:false}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!valid(cases))throw Error(JSON.stringify(valid.errors));const db=new Database(":memory:");
for(const c of cases){const row=db.query("SELECT output,ordinal FROM (SELECT json_extract(value,'$[0]') AS name,json_extract(value,'$[1]') AS output,ROW_NUMBER() OVER(ORDER BY json_extract(value,'$[0]'))-1 AS ordinal FROM json_each(?)) WHERE name=?").get(JSON.stringify(c.entries),c.query);if((row?.output??null)!==c.expected||(row?.ordinal??null)!==c.expectedRank)throw Error(c.name);}
db.close();console.log("PASS borrowed ordered lookup Ajv+SQLite cases="+cases.length+"; native NOTRUN");
