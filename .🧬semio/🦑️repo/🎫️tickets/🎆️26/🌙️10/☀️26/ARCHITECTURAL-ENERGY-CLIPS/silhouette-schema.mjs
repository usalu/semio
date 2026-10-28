import { readFileSync } from "node:fs";
import Ajv from "ajv/dist/2020.js";
import {resolve} from "node:path";
const base=resolve(process.cwd(),"🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/")+"/";
for(const [profile,schema] of [["🔣️.json","🧬️schema/🔣️.schema.json"],["🫟️silhouettes/🔣️.json","🫟️silhouettes/🧬️schema/🔣️.schema.json"]]){
const validate=new Ajv({strict:true,strictRequired:false}).compile(JSON.parse(readFileSync(base+schema,"utf8")));
if(!validate(JSON.parse(readFileSync(base+profile,"utf8"))))throw new Error(JSON.stringify(validate.errors));
console.log("[DEBUG] AJV validated "+profile);
}
