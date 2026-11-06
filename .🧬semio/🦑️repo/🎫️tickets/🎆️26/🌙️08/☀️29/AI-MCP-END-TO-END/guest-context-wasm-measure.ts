import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {pathToFileURL} from "node:url";
const root=process.cwd();
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const cfg=process.argv.includes("--test")?["test","debug_assertions","windows","target_os=windows","target_arch=x86_64","target_family=windows","target_pointer_width=64"]:["debug_assertions","target_os=wasi","target_env=p2","target_arch=wasm32","target_family=wasm","target_pointer_width=32"];
const entry=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🦀️.rs");
const files=await readRustSourceClosureV1([entry],root,cfg,async(path:string)=>{try{return readFileSync(path)}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error}});
console.log(JSON.stringify({files:files.length,bytes:files.reduce((n:number,row:any)=>n+row.bytes,0),activeLines:files.reduce((n:number,row:any)=>n+row.activeLines,0),paths:files.map((row:any)=>row.path)}));
