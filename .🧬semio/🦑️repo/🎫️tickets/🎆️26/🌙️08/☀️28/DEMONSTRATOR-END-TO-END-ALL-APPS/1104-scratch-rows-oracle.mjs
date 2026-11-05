import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/📋️rows/🧪️tests";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const valid=new Ajv().compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!valid(cases))throw Error(JSON.stringify(valid.errors));
const db=new Database(":memory:");
for(const c of cases){const at=c.kind==="insert"?Math.min(c.index,c.rows.length):c.index;const result=db.query("SELECT value FROM (SELECT CASE WHEN ?='insert' AND CAST(key AS INTEGER)>=? THEN CAST(key AS INTEGER)+1 ELSE CAST(key AS INTEGER) END AS ordinal, CASE WHEN ?='replace' AND CAST(key AS INTEGER)=? THEN ? ELSE value END AS value FROM json_each(?) WHERE NOT (?='remove' AND CAST(key AS INTEGER)=?) UNION ALL SELECT ?,? WHERE ?='insert') ORDER BY ordinal").all(c.kind,at,c.kind,at,c.value,JSON.stringify(c.rows),c.kind,at,at,c.value,c.kind).map(r=>r.value);if(JSON.stringify(result)!==JSON.stringify(c.expected))throw Error(c.name);}
db.close();console.log("PASS scratch rows Ajv+SQLite cases="+cases.length+"; native NOTRUN");
