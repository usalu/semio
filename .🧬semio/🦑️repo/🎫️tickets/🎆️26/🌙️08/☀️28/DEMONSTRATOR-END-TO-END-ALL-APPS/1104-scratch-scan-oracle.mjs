import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/🔎️rows/🧪️tests";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const valid=new Ajv().compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!valid(cases))throw Error(JSON.stringify(valid.errors));const db=new Database(":memory:");
for(const c of cases){const row=db.query("SELECT CAST(key AS INTEGER) AS ordinal FROM json_each(?) WHERE value=? ORDER BY ordinal LIMIT 1").get(JSON.stringify(c.rows),c.key);if((row?.ordinal??null)!==c.expected)throw Error(c.name);
 let turns=0;
 for(const candidate of c.rows.slice(0,!row?c.rows.length:row.ordinal+1)){
  turns+=db.query("WITH RECURSIVE positions(n) AS (SELECT 0 UNION ALL SELECT n+64 FROM positions WHERE n+64<length(CAST(?2 AS BLOB))) SELECT CASE WHEN length(CAST(?1 AS BLOB))!=length(CAST(?2 AS BLOB)) THEN 1 ELSE COALESCE(MIN(CASE WHEN substr(CAST(?1 AS BLOB),n+1,64)!=substr(CAST(?2 AS BLOB),n+1,64) THEN n/64+1 END),MAX(n/64+1)) END AS turns FROM positions").get(candidate,c.key).turns;
 }
 if(!row)turns++;
 if(turns!==c.expectedTurns)throw Error(c.name+" window count "+turns);}
db.close();console.log("PASS scratch identity Ajv+SQLite cases="+cases.length+"; native NOTRUN");
