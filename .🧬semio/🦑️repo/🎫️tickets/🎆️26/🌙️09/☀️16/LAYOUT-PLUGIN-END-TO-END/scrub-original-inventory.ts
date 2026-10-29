import {readFile,writeFile} from "node:fs/promises";
import {resolve,relative,dirname} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const manifests=[
"🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/Cargo.toml",
"🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml",
"🧰️framework/🔨️modules/📡️replication/🎮️mutation/✏️edit/📦️packages/🦀️rust/Cargo.toml"
];
const root=process.cwd(),results=[];
for(const manifest of manifests){
 const text=await readFile(manifest,"utf8");const section=text.split("[lib]")[1]?.split(/^\[/m)[0],source=section?.match(/^path\s*=\s*"([^"]+)"/m)?.[1];
 if(!source)throw Error(manifest);
 for(const test of [false,true]){
 const cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix",...(test?["test"]:[])];
 const inputs=await readRustSourceClosureV1([relative(root,resolve(dirname(manifest),source))],root,cfg,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 results.push({manifest,test,files:inputs.length,bytes:inputs.reduce((n,row)=>n+row.bytes,0),lines:inputs.reduce((n,row)=>n+row.activeLines,0)});
 }
}
await writeFile(resolve(import.meta.dirname,"scrub-original-inventory.md"),"# Original Scrub Source Inventory\n\nCanonical macOS ARM64/unix normal and cfg(test); global bounds 32 inputs / 262144 bytes / 4000 active lines. No compiler or graph launched.\n\n```json\n"+JSON.stringify(results,null,2)+"\n```\n");
for(const row of results)console.log(JSON.stringify(row));
