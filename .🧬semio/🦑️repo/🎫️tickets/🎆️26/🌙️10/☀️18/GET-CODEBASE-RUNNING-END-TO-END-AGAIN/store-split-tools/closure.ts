import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
const root=process.cwd();const libs=process.argv.slice(2);
for(const lib of libs){
const rows=await readRustSourceClosureV1([lib],root,["debug_assertions"],async p=>{try{return new Uint8Array(await readFile(p));}catch{return null;}});
let b=0,l=0;for(const r of rows){b+=r.bytes;l+=r.activeLines;}
console.log(lib.split("/").slice(-4).join("/"),"files",rows.length,"bytes",b,"lines",l);
if(process.env.V)for(const r of rows)console.log("  ",r.bytes,r.activeLines,r.path.split("/").slice(-4).join("/"));
}
