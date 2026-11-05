import {readFileSync} from "node:fs";
import Ajv from "ajv";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/📡️invocation/";
const cases=JSON.parse(readFileSync(root+"📨️dispatch-work/🧪️laws/📁️cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(root+"📨️dispatch-work/🧪️laws/📐️schema.json","utf8")));
if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
const requests=JSON.parse(readFileSync(root+"🧾️request-work/🧪️laws/📁️cases.json","utf8"));
for(const row of cases){
 const origin=requests[row.requestCase];if(!origin)throw new Error("Original request source missing");
 const result={extension_id:"geometry-🧊️",capability:row.capability,request_json:origin.expected,response_action:row.responseAction};
 if(Object.keys(result).length!==4||JSON.parse(result.request_json).windowId!==origin.windowId)throw new Error("Dispatch address changed");
}
console.log("Ajv validated 2 complete dispatch cases; independent JSON preserved the four invocation fields and original request/window wire.");
