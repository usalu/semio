import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const base="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎯️completion-and-owned-context/📥️provisional/🧪️laws";
const corpus=JSON.parse(readFileSync(base+"/📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(base+"/📐️schema.json","utf8")));
if(!validate(corpus))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
for(const row of corpus){
 const rows=db.query("SELECT key,value,type FROM json_each(?) ORDER BY CAST(key AS INTEGER)").all(JSON.stringify(row.values));
 const result=rows.map(entry=>entry.type==="null"?null:entry.type==="true"?true:entry.type==="false"?false:["object","array"].includes(entry.type)?JSON.parse(entry.value):entry.value);
 if(JSON.stringify(result)!==JSON.stringify(row.expected)||JSON.stringify(row.values)!==JSON.stringify(row.remaining))throw Error(row.name);
}
db.close();
console.log("PASS provisional original-order Ajv+SQLite cases="+corpus.length+"; native custody NOTRUN");
