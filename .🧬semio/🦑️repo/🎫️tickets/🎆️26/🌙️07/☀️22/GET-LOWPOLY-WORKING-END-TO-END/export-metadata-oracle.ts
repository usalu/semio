import Ajv from "ajv";
import {readFileSync} from "node:fs";
const base="✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/fixtures/";
const schema=JSON.parse(readFileSync(base+"export-metadata.schema.json","utf8"));
const fixture=JSON.parse(readFileSync(base+"export-metadata.json","utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const descriptor=JSON.parse(readFileSync("🌎️hub/🧩️compositions/💠️lowpoly/🧩️component/🔣️.json","utf8"));
const rows=[];
function visit(value){if(!value||typeof value!=="object")return;if(value.id==="exportMesh"&&value.semantics)rows.push({id:value.id,kind:value.kind,destructive:value.semantics.effects.destructive,approval:value.semantics.policy.approval});for(const child of Object.values(value))visit(child);}
visit(descriptor.manifest);
const observed={version:1,actions:rows};
const matches=validate(observed);
console.log(JSON.stringify({fixtureValid:true,descriptorMatchesExpected:matches,observed,errors:validate.errors},null,2));
if(!matches)process.exitCode=1;
