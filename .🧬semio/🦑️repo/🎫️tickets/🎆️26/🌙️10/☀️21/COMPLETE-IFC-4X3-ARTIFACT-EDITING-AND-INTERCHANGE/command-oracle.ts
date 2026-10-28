/** 📨️ Independent JSON Schema validation of the exact closed native edit-command envelope. */
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {strict as assert} from "node:assert";
interface CommandOracle{valid(value:unknown):boolean}
function oracle(event:Record<string,unknown>):CommandOracle{const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(event);const valid=ajv.compile({type:"object",required:["EditSnapshot"],properties:{EditSnapshot:{type:"object",required:["event"],properties:{event:{$ref:event.$id}},additionalProperties:false}},additionalProperties:false});return{valid:value=>Boolean(valid(value))};}
const root=resolve(import.meta.dirname,"../../../../../../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const cases=read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/command-wire.json");
const reference=oracle(read("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧬️schema/🔣️snapshot-edit-event.json"));
for(const value of cases.valid)assert(reference.valid(value));
for(const value of cases.invalid)assert(!reference.valid(value));
assert.equal(cases.nativeExecution,false);
assert.throws(()=>JSON.parse(cases.invalidWire[1]));
console.log(JSON.stringify({oracle:"Ajv2020",validClosedCommands:cases.valid.length,invalidClosedCommands:cases.invalid.length,trailingDataRefused:true,duplicateWireNativeTest:"authored, not executed",nativeCursorExecution:false}));
