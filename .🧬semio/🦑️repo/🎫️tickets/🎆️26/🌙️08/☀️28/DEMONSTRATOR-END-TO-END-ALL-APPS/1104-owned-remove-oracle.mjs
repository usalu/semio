import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="🧰️framework/🔨️modules/🌱️value/🗂️ordered/✍️owned-entry/➖remove/🧪️tests";
const cases=JSON.parse(readFileSync("🧰️framework/🔨️modules/🌱️value/🗂️ordered/🧪️tests/🧫️fixtures/🔣️ordered-map.json","utf8")).ownedRemovalCases;const valid=new Ajv({strict:false}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!valid(cases))throw Error(JSON.stringify(valid.errors));const db=new Database(":memory:");
for(const c of cases){const result=db.query("SELECT json_extract(value,'$[0]') AS name,json_extract(value,'$[1]') AS value FROM json_each(?) WHERE json_extract(value,'$[0]')<>? ORDER BY name").all(JSON.stringify(c.base),c.key).map(row=>[row.name,row.value]);if(JSON.stringify(result)!==JSON.stringify(c.expected))throw Error(c.name);}
db.close();console.log("PASS original owned remove Ajv+SQLite cases="+cases.length+"; native NOTRUN");
