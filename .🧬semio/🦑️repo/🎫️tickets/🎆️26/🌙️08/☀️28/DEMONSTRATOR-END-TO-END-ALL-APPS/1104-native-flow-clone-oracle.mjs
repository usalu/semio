import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const base="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone";
const cases=JSON.parse(readFileSync(base+"/📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(base+"/📐️schema.json","utf8")));
if(!validate(cases))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
db.exec("CREATE TABLE widgets(position INTEGER, id TEXT); CREATE TABLE synapses(position INTEGER, id TEXT)");
for(const row of cases){
 for(const scalar of row.floatWords){
  const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt("0x"+scalar.bits));const value=view.getFloat64(0);
  let json=Object.is(value,-0)?"-0.0":JSON.stringify(value);if(!json.includes(".")&&!/[eE]/.test(json))json+=".0";
  if(json!==scalar.json)throw Error("scalar "+scalar.bits+" "+json+" != "+scalar.json);
 }
 const clone=structuredClone(row.snapshot);
 if(JSON.stringify(clone)!==JSON.stringify(JSON.parse(JSON.stringify(row.snapshot))))throw Error("native fixture equality");
 db.exec("DELETE FROM widgets; DELETE FROM synapses");
 for(const [i,widget]of clone.widgets.entries())db.query("INSERT INTO widgets VALUES (?,?)").run(i,widget.id);
 for(const [i,edge]of clone.synapses.entries())db.query("INSERT INTO synapses VALUES (?,?)").run(i,edge.id);
 if(JSON.stringify(db.query("SELECT id FROM widgets ORDER BY position").all().map(x=>x.id))!==JSON.stringify(row.expectedIds))throw Error(row.name+" widgets");
 if(JSON.stringify(db.query("SELECT id FROM synapses ORDER BY position").all().map(x=>x.id))!==JSON.stringify(row.expectedSynapses))throw Error(row.name+" synapses");
}
db.close();
console.log("PASS Flow native-field fixture "+cases.length+" Ajv+structuredClone+SQLite order; native cloning NOTRUN");
