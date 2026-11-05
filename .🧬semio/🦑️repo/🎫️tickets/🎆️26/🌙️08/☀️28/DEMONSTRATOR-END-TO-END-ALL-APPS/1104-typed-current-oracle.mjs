import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {compareCurrent} from "./1105-typed-oracle-lib.mjs";
const root="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained-commands/📥️provisional/🎛️input/⚖️current/🧪️laws";
const cases=JSON.parse(readFileSync(root+"/📁️cases.json","utf8"));const ajv=new Ajv({strict:false});if(!ajv.compile(JSON.parse(readFileSync(root+"/📐️schema.json","utf8")))(cases))throw Error("corpus shape");
for(const c of cases)if(compareCurrent(c.current,c.input)!==c.expected)throw Error(c.name);
console.log("PASS typed current schema+Ajv+lodash+SQLite cases="+cases.length+"; native bounded/original landing equality NOTRUN");
