import fs from "node:fs/promises";
import {resolve,dirname} from "node:path";
import {pathToFileURL} from "node:url";
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
for(const [name,dir] of [["mesh-engine","🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust"],["remodel-editor","✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust"],["value-ordered","🧰️framework/🔨️modules/🌱️value/🗂️ordered/📦️packages/🦀️rust"],["native-owned","🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust"],["native-decoding","🧰️framework/🔨️modules/🌱️value/🛬️decode/📦️packages/🦀️rust"]])for(const cfg of [[],["test"]]){
const files=await readRustSourceClosureV1([resolve(dir,name === "mesh-engine" ? "../../🦀️.rs" : "🦀️.rs")],resolve("."),cfg,async path=>{try{return await fs.readFile(path)}catch(error){if(error.code==="ENOENT")return null;throw error}},{manifestDirectory:dirname(resolve(dir,"Cargo.toml")),generated:()=>undefined});
console.log(JSON.stringify({name,cfg,files:files.length,bytes:files.reduce((a,b)=>a+b.bytes,0),activeLines:files.reduce((a,b)=>a+b.activeLines,0)}));
}
