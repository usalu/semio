import {readFileSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const [dir,lib]=process.argv.slice(2);
const cfg=["target_arch=\"wasm32\"","target_os=\"wasi\"","target_family=\"wasm\"","debug_assertions","panic=\"abort\"","target_pointer_width=\"32\"","target_endian=\"little\""];
const inputs=await readRustSourceClosureV1([resolve(dir,lib)],REPO,cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});
for(const x of [...inputs].sort((a,b)=>b.bytes-a.bytes))console.log(x.bytes,x.activeLines,relative(REPO,resolve(REPO,x.path)));
