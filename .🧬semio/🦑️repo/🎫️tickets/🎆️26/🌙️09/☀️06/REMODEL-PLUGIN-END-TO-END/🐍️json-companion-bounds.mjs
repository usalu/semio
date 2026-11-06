import fs from "node:fs/promises";
import {dirname,resolve} from "node:path";
import {pathToFileURL} from "node:url";
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const root=resolve("."),owner=resolve("🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🥣️companion"),entry=resolve(owner,"🦀️.rs"),manifest=resolve(owner,"📦️packages/🦀️rust/Cargo.toml");
for(const cfg of [[],["test"]]){const inputs=await readRustSourceClosureV1([entry],root,cfg,async p=>{try{return await fs.readFile(p)}catch(e){if(e.code==="ENOENT")return null;throw e}},{manifestDirectory:dirname(manifest),generated:()=>undefined});const result={cfg,files:inputs.length,bytes:inputs.reduce((n,x)=>n+x.bytes,0),activeLines:inputs.reduce((n,x)=>n+x.activeLines,0)};console.log(JSON.stringify(result));if(result.files>32||result.bytes>262144||result.activeLines>4000)throw Error("companion source closure exceeds admission limits")}
