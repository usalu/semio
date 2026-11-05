import {readFileSync} from "node:fs";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
const base="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️transforms/📥️source/🧪️laws";
const rows=JSON.parse(readFileSync(base+"/cases.json","utf8"));
const valid=new Ajv({strict:true}).compile(JSON.parse(readFileSync(base+"/schema.json","utf8")));
if(!valid(rows))throw Error(JSON.stringify(valid.errors));
for(const c of rows){
 const db=new Database(":memory:");db.run("CREATE TABLE live(window TEXT PRIMARY KEY, ids TEXT);CREATE TABLE captured(window TEXT PRIMARY KEY, ids TEXT)");
 const live=new Map(Object.entries(c.initial));let revision=0;
 for(const [window,ids]of live)db.query("INSERT INTO live VALUES(?,?)").run(window,JSON.stringify(ids));
 db.run("INSERT INTO captured SELECT * FROM live");const captured=[...live].map(([window,ids])=>[window,[...ids]]);const sort=rows=>rows.sort((a,b)=>Buffer.compare(Buffer.from(a[0]),Buffer.from(b[0])));
 for(const {window,ids}of c.writes){
  if(ids===null){if(live.delete(window))revision++;db.query("DELETE FROM live WHERE window=?").run(window)}else{live.set(window,ids);revision++;db.query("INSERT INTO live VALUES(?,?) ON CONFLICT(window) DO UPDATE SET ids=excluded.ids").run(window,JSON.stringify(ids))}
 }
 const sql=table=>db.query("SELECT window,ids FROM "+table+" ORDER BY window COLLATE BINARY").all().map(row=>[row.window,JSON.parse(row.ids)]);
 if(JSON.stringify(sort(captured))!==JSON.stringify(c.expected)||JSON.stringify(sql("captured"))!==JSON.stringify(c.expected)||JSON.stringify(sort([...live]))!==JSON.stringify(c.live)||JSON.stringify(sql("live"))!==JSON.stringify(c.live)||(revision===0)!==c.matches)throw Error(c.name);
 db.close();
}
console.log("PASS gesture authority schema+Ajv2020+independent SQLite immutable capture cases="+rows.length+"; actual native held-root/pointer/work laws NOTRUN");
