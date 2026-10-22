import {readFileSync} from "node:fs";import {join} from "node:path";import Ajv from "ajv";
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations";
for(const root of process.argv.slice(2)){
 const read=(p:string)=>JSON.parse(readFileSync(join(root,B,p),"utf8")),catalog=read("🔣️catalog.json"),schema=read("🧬️schema/🔣️.json");
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),v=ajv.compile(schema),ok=v(catalog);
 console.log(root.slice(-12),"valid:",ok,"errors:",(v.errors??[]).map(e=>`${e.instancePath} ${e.message}`));
}
