import { fileURLToPath, pathToFileURL } from "node:url";
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
const R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any";
const T=R+"/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/";
const vector=JSON.parse(readFileSync(T+"🧫️fixtures/🌳️typed/🔣️.json","utf8"));
const ajv=new Ajv({allErrors:true,strict:false});
for(const u of vector.schemaDocuments) ajv.addSchema(JSON.parse(readFileSync(fileURLToPath(new URL(u,pathToFileURL(T+"x"))),"utf8")));
ajv.addSchema(JSON.parse(readFileSync(R+"/🧬️schema/🔣️.json","utf8")));
ajv.addSchema(JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/🔣️.json","utf8")));
const validate=ajv.compile(JSON.parse(readFileSync(R+"/🧬️schema/🔺️diff/🔣️.json","utf8")));
const root=R+"/🧪️tests/🧫️fixtures/🧬️mutations/";
for(const p of readdirSync(root,{recursive:true}).map(String).filter(p=>p.endsWith("/🔺️diff/🔣️.json"))){const d=JSON.parse(readFileSync(root+p,"utf8"));if(!validate(d))console.log(p,JSON.stringify(validate.errors?.slice(0,3)).slice(0,500));}
