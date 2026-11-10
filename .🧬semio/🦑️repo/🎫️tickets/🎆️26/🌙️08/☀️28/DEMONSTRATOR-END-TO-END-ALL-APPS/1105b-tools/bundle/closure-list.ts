import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root=process.cwd();
const out=await readRustSourceClosureV1([resolve(root,process.argv[2])],root,[],async(p:string)=>{try{return readFileSync(p)}catch{return null}});
for(const i of out.sort((a:any,b:any)=>b.bytes-a.bytes))console.log(i.bytes,i.activeLines,(i.path??i.file??"").toString().replace(root+"/","").slice(-110));
