import fs from "node:fs/promises";
import {dirname,resolve} from "node:path";
import {pathToFileURL} from "node:url";
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const root=resolve("."),owner=process.argv[2]??"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app",manifest=resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),entry=resolve(owner,"🦀️.rs");
for(const cfg of [[],["test"],["test","feature=artifact-app-testing"],["feature=artifact-app-testing"],["feature=reserved-producer-testing"],["test","feature=artifact-app-testing","feature=reserved-producer-testing"]]){
 const reads=new Map();
 try{const inputs=await readRustSourceClosureV1([entry],root,cfg,async p=>{try{const b=await fs.readFile(p);reads.set(p,b.length);return b;}catch(error){if(error.code==="ENOENT")return null;throw error;}},{manifestDirectory:dirname(manifest),generated:()=>undefined});console.log(JSON.stringify({cfg,files:inputs.length,bytes:inputs.reduce((n,x)=>n+x.bytes,0),activeLines:inputs.reduce((n,x)=>n+x.activeLines,0),physical:reads.size}));}catch(error){console.log(JSON.stringify({cfg,error:error.message,physical:reads.size}));}
}
