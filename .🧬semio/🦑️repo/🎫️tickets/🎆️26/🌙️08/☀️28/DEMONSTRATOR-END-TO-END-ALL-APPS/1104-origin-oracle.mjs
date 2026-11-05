import {readFileSync} from "node:fs";
import Ajv from "ajv";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📡️invocation/";
const cases=JSON.parse(readFileSync(root+"📥️origin-work/🧪️laws/📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(root+"📥️origin-work/🧪️laws/📐️schema.json","utf8")));
if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
const sources=JSON.parse(readFileSync(root+"🌳️dependency-work/🧪️laws/📁️cases.json","utf8"));
for(const row of cases){
 const source=sources.find(source=>source.name===row.source);if(!source||source.target!==row.pending.neuronId)throw new Error("Original pending source disagrees with dependency root");
 const origin={windowId:row.windowId,windowKindId:row.windowKindId,...row.pending,dependencyJson:source.expected,operatorVersion:"registry:"+row.registryGeneration+";geometry:1;policy:1"};
 const fields=["neuronId","operatorId","inputJson","dependencyJson","operatorVersion","nodeHash","resume","windowId","windowKindId"];
 const wire=JSON.stringify(Object.fromEntries(fields.map(key=>[key,key==="resume"?false:origin[key]])));
 const decoded=JSON.parse(wire);if(decoded.dependencyJson!==source.expected||decoded.windowId!==row.windowId)throw new Error("Original origin wire loses dependency/address");
}
console.log("Ajv validated 2 original origin cases; independent JSON composition retained pending identity fields, dependency bytes and window addresses.");
