/** 🧾 Check each catalog document against the normative schema and immediately record its digest. */
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {join,resolve} from "node:path";

const staffage=resolve(import.meta.dir,"../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const catalog=join(staffage,"🗂️catalog");
const schemaBytes=readFileSync(join(staffage,"🧬️schema/🔣️.json"));
const schemaSha256=createHash("sha256").update(schemaBytes).digest("hex");
const schema=JSON.parse(schemaBytes.toString("utf8"));
const ajv=new Ajv({strict:true,inlineRefs:false,allErrors:false});
ajv.addSchema(schema);
const match=ajv.compile({$ref:schema.$id+"#/$defs/AssetDocument"});
const selection=new Set(process.argv.slice(2));
function check(path:string,id:string){
  const data=readFileSync(path);
  const valid=match(JSON.parse(data.toString("utf8")));
  console.log(JSON.stringify({id,schemaSha256,sha256:createHash("sha256").update(data).digest("hex"),valid,...(!valid?{issues:match.errors}:{})}));
  if(!valid)process.exitCode=1;
}
for(const category of readdirSync(catalog,{withFileTypes:true})){
  if(!category.isDirectory())continue;
  const dir=join(catalog,category.name);
  for(const entry of readdirSync(dir,{withFileTypes:true})){
    if(!entry.isDirectory()||(selection.size&&!selection.has(entry.name)))continue;
    const path=join(dir,entry.name,"🔣️.json");
    if(!existsSync(path))continue;
    check(path,entry.name);
    Bun.gc(true);
  }
}
