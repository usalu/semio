/** 📏️ Canonical source admission for the three migrated domain declarations and focused native targets. */
import {readFile,writeFile} from "node:fs/promises";
import {dirname,resolve,relative} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const roots=[
"✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",
"✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",
"✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml",
"✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml"
];
const root=process.cwd(),rows=[];
for(const manifest of roots){
 const text=await readFile(manifest,"utf8"),base=dirname(resolve(manifest)),name=text.match(/^name\s*=\s*"([^"]+)"/m)![1];
 const lib=text.split("[lib]")[1]?.split(/^\[/m)[0].match(/^path\s*=\s*"([^"]+)"/m)?.[1];
 const target=text.split("[[test]]").find(block=>block.includes('name = "original_host_configuration"'))?.match(/^path\s*=\s*"([^"]+)"/m)?.[1];
 for(const [kind,path] of [["normal",lib],["host-test",target]] as const){
  if(!path)continue;
  const cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix",...(kind==="host-test"?["test"]:[])];
  const closure=await readRustSourceClosureV1([relative(root,resolve(base,path))],root,cfg,async name=>await readFile(name).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
  const row={name,kind,files:closure.length,bytes:closure.reduce((a,b)=>a+b.bytes,0),lines:closure.reduce((a,b)=>a+b.activeLines,0)};
  rows.push({...row,fits:row.files<=32&&row.bytes<=262144&&row.lines<=4000});
 }
}
await writeFile(resolve(import.meta.dirname,"host-policy-inventory.md"),"# Host Policy Source Inventory\n\nCanonical readRustSourceClosureV1 with macOS ARM64/unix flags, cfg(test) only for actual focused standalone targets. Normal libraries are measured without cfg(test); their existing broad unit suites are not silently selected. The Sourcing implementation lives in its declared current first-party delegate package, which is included here. This is source admission only, not compiler or native execution.\n\n~~~json\n"+JSON.stringify(rows,null,2)+"\n~~~\n");
console.log(JSON.stringify(rows));
