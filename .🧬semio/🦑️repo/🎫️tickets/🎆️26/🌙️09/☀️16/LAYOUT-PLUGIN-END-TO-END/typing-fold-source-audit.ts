/** 🧭️ Inspect the actual mounted typing fold owners and their canonical source budgets. */
import {readFile,writeFile} from "node:fs/promises";
import {dirname,relative,resolve} from "node:path";
import assert from "node:assert/strict";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const roots=[
 "🧰️framework/🔨️modules/🛠️tool-machine",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🧱️ownership",
 "🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit",
 "🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/⌨️typing/✍️edit",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface",
 "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold",
 "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit",
];
const owners=JSON.parse(await readFile("📋️build-owners.json","utf8")).owners;
const rows=[];
for(const root of roots){
 const manifest=resolve(root,"📦️packages/🦀️rust/Cargo.toml"),base=dirname(manifest),doc=Bun.TOML.parse(await readFile(manifest,"utf8")) as any;
 const workspace=resolve(base,doc.package.workspace,"Cargo.toml"),work=Bun.TOML.parse(await readFile(workspace,"utf8")) as any;
 assert(work.workspace.members.includes(relative(dirname(workspace),base)),`${doc.package.name}: workspace member`);
 assert.equal(owners.filter((row:any)=>row.root===relative(process.cwd(),base)).length,1,`${doc.package.name}: owner`);
 const paths=Object.values({...doc.dependencies,...doc["dev-dependencies"]}).filter((dep:any)=>dep.path).map((dep:any)=>resolve(base,dep.path,"Cargo.toml"));
 await Promise.all(paths.map(path=>readFile(path)));
 for(const test of [false,true]){
  const cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix",...(test?["test"]:[])];
  const closure=await readRustSourceClosureV1([relative(process.cwd(),resolve(base,doc.lib.path))],process.cwd(),cfg,async file=>await readFile(file).catch(error=>error.code==="ENOENT"?null:Promise.reject(error))).catch(error=>{throw new Error(`${doc.package.name} (test=${test}): ${error.message}`,{cause:error})});
  const row={name:doc.package.name,test,files:closure.length,bytes:closure.reduce((a,b)=>a+b.bytes,0),lines:closure.reduce((a,b)=>a+b.activeLines,0)};
  rows.push({...row,fits:row.files<=32&&row.bytes<=262144&&row.lines<=4000});
 }
}
await writeFile(resolve(import.meta.dirname,"typing-fold-source-audit.md"),"# Typing Fold Source Admission\n\nActual workspace membership, unique build ownership, TOML parsing, dependency paths and canonical source closure. This is source admission only; native behavior remains unverified.\n\n```json\n"+JSON.stringify(rows,null,2)+"\n```\n");
console.log(JSON.stringify(rows));
assert(rows.every(row=>row.fits),"typing fold source budget exceeded");
