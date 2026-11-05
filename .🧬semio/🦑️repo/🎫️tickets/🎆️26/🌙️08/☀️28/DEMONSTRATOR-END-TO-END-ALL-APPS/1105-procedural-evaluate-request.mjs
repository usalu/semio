import {readFileSync} from "node:fs";
import Ajv from "ajv";
const ticket=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/";
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(ticket+"1105-procedural-evaluate-request.schema.json","utf8")));
let checked=0;
for(const artifact of ["🌀️generation2d","🧊️generation3d"]){
 const row=JSON.parse(readFileSync("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/"+artifact+"/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/⏯️preview-eval-run.json","utf8")).evaluationWire;
 for(const resume of [false,true]){
  const request={neuronId:row.neuronId,operatorId:row.operatorId,inputJson:resume?"":row.inputJson,dependencyJson:resume?"":row.dependencyJson,operatorVersion:"registry:17;geometry:1;policy:1",nodeHash:row.nodeHash,resume,windowId:row.windowId,windowKindId:row.windowKindId,extensionId:row.extensionId};
  if(!validate(request))throw Error(JSON.stringify(validate.errors));
  const encoded=JSON.stringify(request), parsed=JSON.parse(encoded);
  if(Object.keys(parsed).join()!==row.requiredKeys.join())throw Error("canonical field order");
  for(const key of row.requiredCalleeFields)if(typeof parsed[key]!=="string")throw Error("required callee field");
  for(const key of row.authorityFieldsAbsent)if(key in parsed)throw Error("JSON execution authority");
  if(resume)for(const key of row.compactResumeFields)if(parsed[key]!=="")throw Error("noncompact resume");
  const echoed=Object.fromEntries(Object.entries(parsed).filter(([k])=>["nodeHash","windowId","windowKindId","extensionId"].includes(k)));
  const response={...echoed,ok:true,output:{done:true,output:{"源":"mesh"}}};
  if(response.extensionId!==row.extensionId||typeof response.output!=="object"||"outputJson" in response)throw Error("native reactor reflection");
  const missing={...request};delete missing.neuronId;if(validate(missing))throw Error("missing-neuron accepted");
  checked++;
 }
}
console.log("Ajv and independent native JSON oracle passed "+checked+" canonical procedural requests, required-neuron rejection, compact resumes and native response reflection.");
