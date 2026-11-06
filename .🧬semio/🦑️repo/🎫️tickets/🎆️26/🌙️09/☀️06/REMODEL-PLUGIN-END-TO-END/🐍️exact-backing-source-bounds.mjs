import fs from "node:fs/promises";
import {resolve,dirname} from "node:path";
import {pathToFileURL} from "node:url";
const {readRustSourceClosureV1}=await import(pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts")).href);
const owners=[
 ["native-encoding","🧰️framework/🔨️modules/🌱️value/🛫️encode","📦️packages/🦀️rust/🦀️.rs"],
 ["native-encoding-typed-laws","🧰️framework/🔨️modules/🌱️value/🛫️encode/♻️retirement/🧪️tests"],
 ["native-page-parent","🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🎟️reservation"],
 ["paged-list","🧰️framework/🔨️modules/🌱️value/📋️list","📦️packages/🦀️rust/🦀️.rs"],
 ["job-owned-text","🧰️framework/🔨️modules/🧵️job/📝️text/🧳️owned"],
 ["control-contract","🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract"],
 ["flow-host-read","🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read"],
 ["flow-host","🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host"],
 ["remodel-diff","✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff"]
];
for(const [name,directory,relativeEntry="🦀️.rs"] of owners)for(const cfg of [[],["test"]]){
 const entry=resolve(directory,relativeEntry);
 const files=await readRustSourceClosureV1([entry],resolve("."),cfg,async path=>{try{return await fs.readFile(path)}catch(error){if(error.code==="ENOENT")return null;throw error}},{manifestDirectory:resolve(directory,"📦️packages/🦀️rust"),generated:()=>undefined});
 console.log(JSON.stringify({name,cfg,files:files.length,bytes:files.reduce((n,r)=>n+r.bytes,0),activeLines:files.reduce((n,r)=>n+r.activeLines,0),longestPath:Math.max(...files.map(r=>r.path.length))}));
}
