import {readFileSync,existsSync} from "node:fs";
import {resolve} from "node:path";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const cfg=["test","debug_assertions","panic=\"unwind\"","target_pointer_width=\"64\"","unix"];
for(const dir of process.argv.slice(2)){
 const toml=Bun.TOML.parse(readFileSync(dir+"/Cargo.toml","utf8")) as any;
 const entry=resolve(dir,toml.lib?.path??"src/lib.rs");
 const files=await readRustSourceClosureV1([entry],REPO,cfg,async(p:string)=>existsSync(p)?new Uint8Array(readFileSync(p)):null,{manifestDirectory:dir,generated:()=>{}});
 const bytes=files.reduce((s:number,f:any)=>s+f.bytes,0),lines=files.reduce((s:number,f:any)=>s+f.activeLines,0);
 console.log(bytes<=262144&&files.length<=32&&lines<=4000?"PASS":"FAIL",toml.package.name,"files",files.length,"bytes",bytes,"lines",lines);
}
