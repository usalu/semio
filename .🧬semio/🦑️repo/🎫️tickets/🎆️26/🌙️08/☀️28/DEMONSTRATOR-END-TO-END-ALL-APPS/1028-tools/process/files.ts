import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
const {readRustSourceClosureV1}=await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const dir=resolve(process.argv[2]);const lib=resolve(dir,process.argv[3]);
const cfg=['target_arch="wasm32"','target_os="wasi"','target_family="wasm"','panic="abort"'];
const inputs=await readRustSourceClosureV1([lib],"/Users/ueli/Documents/semio",cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});
let t=0;for(const x of inputs.sort((a:any,b:any)=>b.bytes-a.bytes)){t+=x.bytes;console.log(x.bytes,x.activeLines,x.path.replace(/.*🔌️plugin\//,""))}console.log("TOTAL",t,inputs.length);
