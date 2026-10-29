import {readFileSync} from "node:fs";
import {resolve,join,relative} from "node:path";
import {execFileSync} from "node:child_process";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const cfg=[...execFileSync("rustc",["--print","cfg"],{encoding:"utf8"}).trim().split("\n").map(v=>v.replace(/=(".*")$/u,(_,q)=>"="+JSON.parse(q))),"test","debug_assertions"];
const dir=resolve(process.argv[2]);
const t=Bun.TOML.parse(readFileSync(join(dir,"Cargo.toml"),"utf8")) as any;
const lib=resolve(dir,t.lib.path);
const inputs=await readRustSourceClosureV1([lib],REPO,cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:dir,generated:()=>undefined});
let b=0;for(const x of inputs){b+=x.bytes;console.log(x.bytes,x.activeLines,relative(dir,resolve(REPO,x.path)).slice(-110));}
console.log("TOTAL",inputs.length,b);
