import { readFileSync } from "node:fs";import { resolve } from "node:path";import { pathToFileURL } from "node:url";
const root=process.cwd(),{readRustSourceClosureV1}=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const rows=await readRustSourceClosureV1([resolve(root,"🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/🦀️.rs")],root,["test","debug_assertions","windows","target_os=windows","target_arch=x86_64","target_family=windows","target_pointer_width=64"],async(path:string)=>{try{return readFileSync(path)}catch{return null}});
console.log(JSON.stringify({files:rows.length,bytes:rows.reduce((sum:number,row:{path:string})=>sum+readFileSync(resolve(root,row.path)).length,0)}));
