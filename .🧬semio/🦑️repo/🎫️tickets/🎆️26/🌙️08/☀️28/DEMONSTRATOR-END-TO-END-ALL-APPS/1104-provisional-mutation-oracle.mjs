import {readFileSync,readdirSync,existsSync} from "node:fs";
import Ajv from "ajv";
const prefix="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts";
for(const kind of ["🌀️generation2d","🧊️generation3d"]){
 const base=prefix+"/"+kind+"/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
 const ajv=new Ajv({strict:false,allErrors:true,validateFormats:false});
 for(const row of readdirSync(base,{withFileTypes:true})){if(!row.isDirectory())continue;const p=base+"/"+row.name+"/🧬️schema/🔣️.json";if(existsSync(p))ajv.addSchema(JSON.parse(readFileSync(p,"utf8")));}
 const validate=ajv.compile(JSON.parse(readFileSync(base+"/🔣️.json","utf8")));
 const cases=JSON.parse(readFileSync(base+"/📥️provisional/🧪️tests/📁️cases.json","utf8"));
 for(const row of cases){if(!validate(row.wire))throw Error(kind+" "+row.name+" "+JSON.stringify(validate.errors?.slice(0,3)));if(JSON.stringify(JSON.parse(JSON.stringify(row.wire)))!==JSON.stringify(row.expected))throw Error(row.name);}
 console.log("PASS "+kind+" actual mutation schemas+Ajv+JSON cases="+cases.length+"; native receiving NOTRUN");
}
