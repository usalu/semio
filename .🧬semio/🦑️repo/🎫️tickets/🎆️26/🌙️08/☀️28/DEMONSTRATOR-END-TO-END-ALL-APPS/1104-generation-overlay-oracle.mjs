import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const base="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🧬️generation/🌊️flow-overlay/🧪️laws";
const cases=JSON.parse(readFileSync(base+"/📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(base+"/📐️schema.json","utf8")));
if(!validate(cases))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
const query=db.query(`SELECT CASE
 WHEN json_extract(w.value,'$.kind')='inputSlider' AND v.type IN('integer','real') THEN json_set(w.value,'$.value',v.value)
 WHEN json_extract(w.value,'$.kind')='inputNote' AND v.type='text' THEN json_set(w.value,'$.text',v.value)
 WHEN json_extract(w.value,'$.kind')='inputImage' AND v.type='text' THEN json_set(w.value,'$.src',v.value)
 WHEN json_extract(w.value,'$.kind')='variable' AND v.type='text' THEN json_set(w.value,'$.name',v.value)
 ELSE w.value END AS widget
 FROM json_each(?) AS w LEFT JOIN json_each(?) AS v ON v.key=json_extract(w.value,'$.id') ORDER BY CAST(w.key AS INTEGER)`);
for(const row of cases){
 const independent=query.all(JSON.stringify(row.widgets),JSON.stringify(row.values)).map(x=>JSON.parse(x.widget));
 if(JSON.stringify(independent)!==JSON.stringify(row.expected))throw Error(row.name);
}
db.close();
console.log("PASS generation Flow overlay Ajv+independent SQLite semantics cases="+cases.length+"; funded producer/native NOTRUN");
