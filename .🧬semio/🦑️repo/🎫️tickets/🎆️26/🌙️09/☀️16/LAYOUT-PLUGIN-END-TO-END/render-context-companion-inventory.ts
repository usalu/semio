import {readFile} from "node:fs/promises";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const base="✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/";
for(const [name,entry,cfg] of [
 ["editor-companion","✏️editor/🧪️tests/🔬️unit/🧪️cases.rs",["target_arch=aarch64","target_os=macos","target_family=unix","unix","feature=testkit","test"]]
] as [string,string,string[]][]){
 const inputs=await readRustSourceClosureV1([base+entry],process.cwd(),cfg,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 console.log(JSON.stringify({name,files:inputs.length,bytes:inputs.reduce((n,row)=>n+row.bytes,0),lines:inputs.reduce((n,row)=>n+row.activeLines,0)}));
}
