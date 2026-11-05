import {readFileSync} from "node:fs";
import Ajv from "ajv";
const root="✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/📋️projection/🧪️laws";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const check=new Ajv({strict:true}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!check(cases))throw Error(JSON.stringify(check.errors));for(const c of cases){if(structuredClone(c.input)!==c.expected)throw Error(c.name)}console.log("PASS projected native field schema+Ajv+structuredClone cases="+cases.length+"; native receipts/closure NOTRUN");
