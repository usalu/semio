/** 🧬 Validate the current native BIM data against its authored schema with existing Ajv. */
import fs from "node:fs";
import path from "node:path";
import {createRequire} from "node:module";
import {fileURLToPath} from "node:url";
const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020");
const ticket=path.dirname(fileURLToPath(import.meta.url));
let root=ticket;
while(!fs.existsSync(path.join(root,"🧰️framework"))){const parent=path.dirname(root);if(parent===root)throw Error("Workspace root missing");root=parent;}
const model=path.join(root,"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model");
const schema=JSON.parse(fs.readFileSync(path.join(model,"🧬️schema/🔣️.schema.json"),"utf8"));
const data=JSON.parse(fs.readFileSync(path.join(model,"🔣️.json"),"utf8"));
const validate=new Ajv({allErrors:true,strict:false}).compile(schema);
const valid=validate(data);
const result={valid,errors:validate.errors??[],validator:require("ajv/package.json").version};
const output=path.join(ticket,"🗑️generated/ip");
fs.mkdirSync(output,{recursive:true});fs.writeFileSync(path.join(output,"schema-validation.json"),JSON.stringify(result,null,2));
console.log(JSON.stringify(result));process.exitCode=valid?0:1;
