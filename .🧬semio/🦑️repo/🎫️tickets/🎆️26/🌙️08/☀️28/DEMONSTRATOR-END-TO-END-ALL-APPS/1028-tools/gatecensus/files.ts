const {readRustSourceClosureV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
const [lib,dir,...feat]=process.argv.slice(2);
const cfg=["debug_assertions","panic=\"unwind\"","target_arch=\"aarch64\"","target_os=\"macos\"","unix","target_pointer_width=\"64\"",...feat.map(f=>`feature="${f}"`)];
const inputs=await readRustSourceClosureV1([resolve(lib)],"/Users/ueli/Documents/semio",cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});
for(const x of inputs)console.log(x.bytes,x.path);
