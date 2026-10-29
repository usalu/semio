import {resolve} from "node:path";
import {readFile} from "node:fs/promises";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const entry="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🦀️.rs",cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix","test"];
const inputs=await readRustSourceClosureV1([entry],process.cwd(),cfg,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)),{manifestDirectory:resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/📦️packages/🦀️rust"),generated:()=>{throw Error("generated tokens need actual compiler evidence")}});
console.log(JSON.stringify({files:inputs.length,bytes:inputs.reduce((n,r)=>n+r.bytes,0),lines:inputs.reduce((n,r)=>n+r.activeLines,0)}));
