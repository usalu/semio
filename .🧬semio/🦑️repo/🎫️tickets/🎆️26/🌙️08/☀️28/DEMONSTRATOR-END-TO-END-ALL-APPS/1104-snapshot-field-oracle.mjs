import {readFileSync} from "node:fs";
import Ajv from "ajv";
const base="🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/📋️field-clone";
const fixture=JSON.parse(readFileSync(base+"/🧪️laws/📁️cases.json","utf8"));
const schema=JSON.parse(readFileSync(base+"/🧪️laws/📐️schema.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(schema);
if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
for(const row of fixture){
 const independent=structuredClone({field:row.field,other:73});
 const json=JSON.parse(JSON.stringify({field:row.field,other:73}));
 if(JSON.stringify(independent)!==JSON.stringify(json))throw Error(row.name);
 if(independent.field===row.field||independent.field.some((x,i)=>x!==row.field[i]))throw Error("clone equality/independence");
}
console.log("PASS snapshot-field neutral cases="+fixture.length+" Ajv+structuredClone/JSON equality; native custody NOTRUN");
