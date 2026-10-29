import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {dirname,resolve} from "node:path";
const REPO="/Users/ueli/Documents/semio";
const {readRustSourceClosureV1}=await import(REPO+"/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts");
const [manifestDir,libRel,...extra]=process.argv.slice(2);
const cfg=[...execFileSync("rustc",["--print","cfg"],{encoding:"utf8"}).split("\n").filter(Boolean),...extra];
const inputs=await readRustSourceClosureV1([resolve(manifestDir,libRel)],REPO,cfg,async(p:string)=>{try{return new Uint8Array(readFileSync(p))}catch{return null}},{manifestDirectory:manifestDir,generated:()=>undefined});
let bytes=0;for(const i of inputs as any[]){bytes+=(i.bytes?.length??i.byteLength??0)}
console.log(JSON.stringify({files:inputs.length}));for(const i of inputs as any[])console.log(Object.keys(i).join(","),JSON.stringify(i).slice(0,260));
