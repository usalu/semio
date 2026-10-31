import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import {execFileSync} from "node:child_process";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const base=execFileSync("rustc",["--print","cfg"],{encoding:"utf8"}).trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q)));
const mp=resolve(process.argv[2]);const dir=dirname(mp);
const t=Bun.TOML.parse(readFileSync(mp,"utf8")) as any;
const lib=resolve(dir,t.lib.path);
const rd=async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}};
const run=(c:string[])=>readRustSourceClosureV1([lib],REPO,c,rd,{manifestDirectory:dir,generated:()=>undefined});
const a=await run([...base,"debug_assertions"]);const b=await run([...base,"test","debug_assertions"]);
const key=(x:any)=>x.path??x.file;
const an=new Set(a.map(key));
let tb=0,tl=0,n=0;
for(const x of b){if(!an.has(key(x))){console.log("T",x.bytes,x.activeLines,String(key(x)).replace(dir+"/","").replace(REPO+"/",""));tb+=x.bytes;tl+=x.activeLines;n++}}
const ab=a.reduce((s:number,x:any)=>s+x.bytes,0),al=a.reduce((s:number,x:any)=>s+x.activeLines,0);
const bb=b.reduce((s:number,x:any)=>s+x.bytes,0),bl=b.reduce((s:number,x:any)=>s+x.activeLines,0);
console.log("nontest",ab,a.length,al,"test",bb,b.length,bl,"| test-only files",n,tb,tl,"| in-file test bytes",bb-ab-tb,"lines",bl-al-tl);
