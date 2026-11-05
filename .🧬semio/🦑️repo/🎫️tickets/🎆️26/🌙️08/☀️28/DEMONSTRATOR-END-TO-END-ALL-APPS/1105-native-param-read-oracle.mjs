import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
const root="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/📥️provisional/🎛️input/📚️params/🔎️read/🧪️laws";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const valid=new Ajv({strict:false}).compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")));if(!valid(cases))throw Error("shape");const db=new Database(":memory:");
for(const c of cases){const actual=c.kind==="number"?[c.fallback[0],0,0]:[...c.fallback];for(const[index,field]of(c.kind==="number"?["value"]:["x","y","z"]).entries()){
 const path="$."+c.key+"."+field;const row=db.query("SELECT json_type(?1,?2) AS kind,json_extract(?1,?2) AS value").get(JSON.stringify(c.params),path);if(["integer","real"].includes(row.kind))actual[index]=row.value;
}if(JSON.stringify(actual)!==JSON.stringify(c.expected))throw Error(c.name);}
db.close();console.log("PASS native param read schema+Ajv+SQLite cases="+cases.length+"; original native fallback/physical laws NOTRUN");
