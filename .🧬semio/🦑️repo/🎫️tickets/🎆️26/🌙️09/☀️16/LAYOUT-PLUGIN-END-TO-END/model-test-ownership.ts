import {readFile,writeFile} from "node:fs/promises";
import {resolve,relative} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const base="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
const rows=[];
for(const [name,path] of [["model",base+"/🦀️.rs"],["validation",base+"/✅️validation/🧪️tests/🦀️.rs"]]){
 const inputs=await readRustSourceClosureV1([path],process.cwd(),["target_arch=aarch64","target_os=macos","target_family=unix","unix","test","feature=testkit"],async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 const row={name,files:inputs.length,bytes:inputs.reduce((a,b)=>a+b.bytes,0),lines:inputs.reduce((a,b)=>a+b.activeLines,0),inputs};
 rows.push(row);console.log(JSON.stringify(row));
}
await writeFile(resolve(import.meta.dirname,"🗑️generated/model-test-ownership/closure.json"),JSON.stringify(rows,null,2));
