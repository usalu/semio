import {readFileSync,existsSync} from "node:fs";
import {resolve,join} from "node:path";
import {execFileSync} from "node:child_process";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const dir=REPO+"/🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust";
const cfg=execFileSync("rustc",["--print","cfg","--target","wasm32-wasip2","--cfg",'feature="testkit"'],{encoding:"utf8"}).trim().split("\n");
const inputs=await readRustSourceClosureV1([join(dir,"🦀️.rs")],REPO,cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});
let total=0,rows:any[]=[];
for(const x of inputs){const p=resolve(REPO,x.path);let head=0;try{head=Number(execFileSync("git",["-C",REPO,"cat-file","-s","HEAD:"+x.path],{encoding:"utf8"}))}catch{head=-1}total+=x.bytes;rows.push({p:x.path.slice(-60),now:x.bytes,head});}
console.log(total);for(const r of rows.filter(r=>r.now!==r.head))console.log(JSON.stringify(r));
