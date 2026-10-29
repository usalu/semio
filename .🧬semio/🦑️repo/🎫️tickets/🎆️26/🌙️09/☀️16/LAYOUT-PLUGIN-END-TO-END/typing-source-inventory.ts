/** ⌨️ Canonical source admission of the actual typing dispatch and mounted instance owners. */
import {readFile,writeFile} from "node:fs/promises";
import {dirname,resolve,relative} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const paths=["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📦️packages/🦀️rust/Cargo.toml","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📦️packages/🦀️rust/Cargo.toml"];
const rows=[];
for(const manifest of paths){
 const text=await readFile(manifest,"utf8"),base=dirname(resolve(manifest)),name=text.match(/^name\s*=\s*"([^"]+)"/m)![1],lib=text.split("[lib]")[1]!.split(/^\[/m)[0]!.match(/^path\s*=\s*"([^"]+)"/m)![1]!;
 for(const test of [false,true]){
  const cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix",...(test?["test"]:[])];
  const closure=await readRustSourceClosureV1([relative(process.cwd(),resolve(base,lib))],process.cwd(),cfg,async file=>await readFile(file).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
  const row={name,test,files:closure.length,bytes:closure.reduce((a,b)=>a+b.bytes,0),lines:closure.reduce((a,b)=>a+b.activeLines,0)};
  rows.push({...row,fits:row.files<=32&&row.bytes<=262144&&row.lines<=4000});
 }
}
await writeFile(resolve(import.meta.dirname,"typing-source-inventory.md"),"# Typing Source Admission\n\nCanonical current closures, macOS ARM64/unix. Source admission only; native compilation pending.\n\n```json\n"+JSON.stringify(rows,null,2)+"\n```\n");
console.log(JSON.stringify(rows));
