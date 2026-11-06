import assert from "node:assert/strict";
import {readFileSync,existsSync} from "node:fs";
import {resolve,dirname} from "node:path";
const files=[
 "🧰️framework/🔨️modules/🧵️job/📝️text/🧳️owned/📦️packages/🦀️rust/Cargo.toml",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/📦️packages/🦀️rust/Cargo.toml",
 "🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/📦️packages/🦀️rust/Cargo.toml"
];
let checked=0;
for(const file of files){
 const source=readFileSync(file,"utf8");
 for(const match of source.matchAll(/(?:path|workspace)\s*=\s*"([^"]+)"/g)){
  const target=resolve(dirname(file),match[1]);
  assert.ok(target.length<=256,target);
  assert.ok(existsSync(target),target);
  checked++;
 }
}
const workspace=readFileSync("🧰️framework/Cargo.toml","utf8");
assert.ok(checked>0);
assert.equal(workspace.split('"🔨️modules/🧵️job/📝️text/🧳️owned/📦️packages/🦀️rust"').length-1,1);
console.log(JSON.stringify({manifests:files.length,relativePaths:checked,newOwnerListedOnce:true,cargoResolution:false}));
