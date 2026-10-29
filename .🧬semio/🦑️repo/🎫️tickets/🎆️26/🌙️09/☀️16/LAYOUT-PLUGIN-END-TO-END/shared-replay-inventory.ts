import {readFile} from "node:fs/promises";
import {resolve,relative} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root=process.cwd();
for(const entry of [
"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs",
"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🦀️.rs"
]){
const inputs=await readRustSourceClosureV1([entry],root,["target_arch=aarch64","target_os=macos","target_family=unix","unix"],async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
console.log(JSON.stringify({entry,files:inputs.length,bytes:inputs.reduce((a,b)=>a+b.bytes,0),lines:inputs.reduce((a,b)=>a+b.activeLines,0)}));
}
