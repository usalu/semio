/** 🎟️ Independent event schema and UTF-8 encoding oracle for retained admission vectors. */
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {strict as assert} from "node:assert";
interface EventOracle {valid(value:unknown):boolean;bytes(value:unknown):number}
function createOracle(schema:object):EventOracle {
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
 return {valid:value=>Boolean(validate(value)),bytes:value=>new TextEncoder().encode(JSON.stringify(value)).length};
}
const root=resolve(import.meta.dirname,"../../../../../../..");
const read=(path:string)=>JSON.parse(readFileSync(resolve(root,path),"utf8"));
const fixture=read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/admission.json");
const oracle=createOracle(read("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧬️schema/🔣️snapshot-edit-event.json"));
for(const event of fixture.events)assert(oracle.valid(event));
assert(!oracle.valid({...fixture.events[0],unknown:true}));
assert(oracle.bytes({...fixture.events[0],value:"x".repeat(fixture.refusals.oversizedStringBytes)})>fixture.maximumRawBytes);
console.log(JSON.stringify({oracle:"Ajv2020 and ECMAScript UTF-8 JSON",events:fixture.events.length,exactBytes:fixture.events.map(oracle.bytes),oversizedEventRefused:true,nativeCursorExecution:"not executed"}));
