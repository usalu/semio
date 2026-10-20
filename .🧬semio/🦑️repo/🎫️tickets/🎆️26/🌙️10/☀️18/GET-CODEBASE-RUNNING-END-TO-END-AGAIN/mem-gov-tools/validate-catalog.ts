import Ajv from "/private/tmp/mem-gov/r/node_modules/ajv";
import {readFileSync} from "node:fs";
import {join} from "node:path";

const B=join(process.argv[2]??"/private/tmp/mem-gov/h","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations");
const catalog=JSON.parse(readFileSync(join(B,"🔣️catalog.json"),"utf8")),schema=JSON.parse(readFileSync(join(B,"🧬️schema/🔣️.json"),"utf8"));
const ajv=new Ajv({strict:true,allowUnionTypes:true}),validate=ajv.compile(schema),valid=validate(catalog);
console.log("schema valid:",valid,valid?"":JSON.stringify(validate.errors?.slice(0,3)));
const names=catalog.operations.map((row:{name:string})=>row.name);
console.log("enum equals names:",JSON.stringify([...names].sort())===JSON.stringify([...schema.definitions.OperationName.enum].sort()));
let mismatches=0;
for(const row of catalog.operations)for(const name of row.flags){const flag=catalog.flags.find((candidate:{name:string})=>candidate.name===name);if(!flag||!flag.operations?.includes(row.name)){console.log("flag mismatch",row.name,name);mismatches++;}}
for(const flag of catalog.flags)for(const operation of flag.operations??[]){const row=catalog.operations.find((candidate:{name:string})=>candidate.name===operation);if(!row||!row.flags.includes(flag.name)){console.log("flag not listed by operation",operation,flag.name);mismatches++;}}
console.log("flag mismatches:",mismatches);
process.exit(valid&&mismatches===0?0:1);
