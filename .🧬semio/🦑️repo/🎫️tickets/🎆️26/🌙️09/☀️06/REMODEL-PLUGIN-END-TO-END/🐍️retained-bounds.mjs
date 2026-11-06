import fs from "node:fs/promises";
import {resolve,dirname} from "node:path";
import {pathToFileURL} from "node:url";
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const dir=resolve("🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/📦️packages/🦀️rust");
for(const cfg of [[],["test"]]){const files=await readRustSourceClosureV1([resolve(dir,"🦀️.rs")],resolve("."),cfg,async path=>{try{return await fs.readFile(path)}catch(error){if(error.code==="ENOENT")return null;throw error}},{manifestDirectory:dirname(resolve(dir,"Cargo.toml")),generated:()=>undefined});console.log(JSON.stringify({cfg,files:files.length,bytes:files.reduce((a,b)=>a+b.bytes,0),activeLines:files.reduce((a,b)=>a+b.activeLines,0)}));}
