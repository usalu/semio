import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/📇️catalogue-work/🧪️laws/";
const cases=JSON.parse(readFileSync(root+"📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(root+"📐️schema.json","utf8")));
if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
for(const row of cases){
 const db=new Database(":memory:");db.run("CREATE TABLE operators(id TEXT PRIMARY KEY,metadata TEXT NOT NULL)");
 for(const info of row.operators)db.run("INSERT INTO operators VALUES(?,?)",info.id,JSON.stringify(info));
 const entries=db.query("SELECT id,metadata FROM operators ORDER BY id COLLATE BINARY").all();
 if(JSON.stringify(entries.map(row=>row.id))!==JSON.stringify(row.expectedIds))throw new Error("Independent catalogue key order disagrees");
 for(const entry of entries){const info=row.operators.find(info=>info.id===entry.id);if(JSON.stringify(JSON.parse(entry.metadata))!==JSON.stringify(info))throw new Error("Independent catalogue lost original metadata")}
 db.close();
}
console.log("Ajv validated the catalogue fixture; SQLite independently retained both complete metadata rows and reproduced exact sorted keys.");
