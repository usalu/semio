import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const cfg=[...(process.env.TESTCFG?["test"]:[]),"target_arch=\"wasm32\"","target_os=\"wasi\"","target_family=\"wasm\"","debug_assertions","panic=\"unwind\"","target_pointer_width=\"32\"","unix"].filter(c=>!/unix/.test(c));
for(const dir of process.argv.slice(2)){
 const toml=Bun.TOML.parse(readFileSync(dir+"/Cargo.toml","utf8")) as any;
 const entry=resolve(dir,toml.lib?.path??"src/lib.rs");
 const files=await readRustSourceClosureV1([entry],REPO,cfg,async(p:string)=>existsSync(p)?new Uint8Array(readFileSync(p)):null,{manifestDirectory:dir,generated:()=>{}});
 const total=files.reduce((s:number,f:any)=>s+f.bytes,0);
 console.log(toml.package.name,"files",files.length,"bytes",total,"limit 262144/32");
 if(process.env.V)for(const f of files.sort((a:any,b:any)=>b.bytes-a.bytes).slice(0,Number(process.env.V)))console.log("  ",f.bytes,f.kind,f.path.slice(-90));
}
