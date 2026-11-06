import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.cwd();
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const cfg=["test","debug_assertions","windows","target_os=windows","target_arch=x86_64","target_family=windows","target_pointer_width=64"];
for(const entry of process.argv.slice(2).length?process.argv.slice(2):["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs"]){
 const files=await readRustSourceClosureV1([resolve(root,entry)],root,cfg,async(path:string)=>{try{return readFileSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}});
 console.log(JSON.stringify(files));
}
