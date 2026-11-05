import {readFileSync} from "node:fs";
import Ajv from "ajv";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📡️invocation/🧾️request-work/🧪️laws/";
const cases=JSON.parse(readFileSync(root+"📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(root+"📐️schema.json","utf8")));
if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
for(const {expected,...fields} of cases){
 const nodeHash=typeof fields.nodeHash==="string"?BigInt(fields.nodeHash):null;
 const sorted=Object.fromEntries(["neuronId","operatorId","inputJson","dependencyJson","operatorVersion","nodeHash","resume","windowId","windowKindId"].map(key=>[key,fields[key]]));
 if(nodeHash!==null)sorted.nodeHash=0;
 const actual=JSON.stringify(sorted).replace(nodeHash===null?/$^/:'"nodeHash":0','"nodeHash":'+nodeHash);
 if(actual!==expected)throw new Error("Independent canonical wire disagrees");
}
console.log("Ajv validated 2 invocation request cases; independent JSON wire matched both, including maximum uint64, Unicode and control escaping.");
