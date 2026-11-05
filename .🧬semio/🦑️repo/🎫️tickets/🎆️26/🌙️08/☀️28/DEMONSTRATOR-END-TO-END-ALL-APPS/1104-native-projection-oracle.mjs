import {readFileSync} from "node:fs";
import Ajv from "ajv";
const root="🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🪞️projection/🧪️tests";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));
const valid=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));
if(!valid(cases))throw Error(JSON.stringify(valid.errors));
for(const row of cases){const parent=row.wrap==="array"?[null,row.source]:row.source;const before=JSON.stringify(parent);const selected=row.wrap==="array"?parent[1]:parent;const value=structuredClone(selected.value);if(value!==row.expected||JSON.stringify(parent)!==before)throw Error(row.name);}
console.log("PASS original projection Ajv+structuredClone cases="+cases.length+"; native custody NOTRUN");
