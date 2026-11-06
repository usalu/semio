import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
const schema=JSON.parse(readFileSync(new URL("./📐️job-source-allocation-schema.json",import.meta.url),"utf8"));
const fixture=JSON.parse(readFileSync(new URL("./📐️job-source-allocation-fixture.json",import.meta.url),"utf8"));
if(!new Ajv({strict:true}).validate(schema,fixture))throw Error("fixture schema differs");
for(const c of fixture.cases){
 const db=new Database(":memory:");db.run("CREATE TABLE events(seq INTEGER PRIMARY KEY,kind TEXT,request TEXT,bytes INTEGER,actual INTEGER,allowed INTEGER)");
 c.events.forEach((e,i)=>db.run("INSERT INTO events VALUES(?,?,?,?,?,?)",i+1,e.kind,e.request,e.bytes,e.actual,Number(e.allow)));
 const rows=db.query(`WITH RECURSIVE state(seq,pending,pending_bytes,actual,accepted) AS (
 SELECT 0,NULL,NULL,0,1 UNION ALL
 SELECT e.seq,
 CASE WHEN e.kind='quote' AND e.allowed=1 AND s.pending IS NULL THEN e.request WHEN e.kind IN ('post','cancel') AND s.pending=e.request AND s.pending_bytes=e.bytes THEN NULL ELSE s.pending END,
 CASE WHEN e.kind='quote' AND e.allowed=1 AND s.pending IS NULL THEN e.bytes WHEN e.kind IN ('post','cancel') AND s.pending=e.request AND s.pending_bytes=e.bytes THEN NULL ELSE s.pending_bytes END,
 s.actual+CASE WHEN e.kind='post' THEN e.actual ELSE 0 END,
 CASE WHEN e.kind='quote' THEN e.allowed=1 AND s.pending IS NULL WHEN e.kind='post' THEN e.allowed=1 AND s.pending=e.request AND s.pending_bytes=e.bytes AND e.actual<=e.bytes ELSE s.pending=e.request AND s.pending_bytes=e.bytes END
 FROM state s JOIN events e ON e.seq=s.seq+1) SELECT pending,actual,coalesce(accepted,0) AS accepted FROM state WHERE seq>0 ORDER BY seq`).all();
 const actual=rows.map(r=>({pending:r.pending,actual:r.actual,accepted:!!r.accepted}));if(JSON.stringify(actual)!==JSON.stringify(c.expected))throw Error(JSON.stringify({name:c.name,actual,expected:c.expected}));db.close();
}
console.log("Ajv + SQLite: "+fixture.cases.length+" Job/Source allocation event cases passed");
