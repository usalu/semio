import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const root=process.cwd(), entry="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs";

for(const platform of [{name:"macos",cfg:["target_arch=aarch64","target_os=macos","target_family=unix","unix","target_pointer_width=64"]},{name:"linux",cfg:["target_arch=x86_64","target_os=linux","target_family=unix","unix","target_pointer_width=64"]},{name:"windows",cfg:["target_arch=x86_64","target_os=windows","target_family=windows","windows","target_pointer_width=64"]},{name:"wasm",cfg:["target_arch=wasm32","target_os=unknown","target_family=wasm","target_pointer_width=32"]}])
for(const [name,target,features] of [["engine",entry,[]],["engine-testkit",entry,["feature=testkit"]],["engine-test",entry,["test"]],["scene-tests",entry.replace("/🦀️.rs","/🧪️tests/🎬️scene/🦀️.rs"),["test"]],["flow-tests",entry.replace("/🦀️.rs","/🧪️tests/🧵️flow/🦀️.rs"),["test"]]] as const) {
 const inputs=await readRustSourceClosureV1([target],root,[...platform.cfg,...features],async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 console.log(JSON.stringify({platform:platform.name,name,files:inputs.length,bytes:inputs.reduce((a,b)=>a+b.bytes,0),lines:inputs.reduce((a,b)=>a+b.activeLines,0)}));
}
