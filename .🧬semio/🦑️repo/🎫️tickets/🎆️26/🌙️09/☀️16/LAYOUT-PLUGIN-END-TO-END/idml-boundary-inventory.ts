import {readFile} from "node:fs/promises";
import {resolve,relative,dirname} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml";
for(const owner of ["","🧬️schema/📇️descriptor","🧪️tests"]){for(const test of [false,true]){
 const manifest=resolve(base,owner,"📦️packages/🦀️rust/Cargo.toml"),text=await readFile(manifest,"utf8"),path=text.split("[lib]")[1].split(/^[[]/m)[0].match(/^path\s*=\s*"([^"]+)"/m)![1];
 const inputs=await readRustSourceClosureV1([relative(process.cwd(),resolve(dirname(manifest),path))],process.cwd(),["target_arch=aarch64","target_os=macos","target_family=unix","unix",...(test?["test"]:[])],async file=>await readFile(file).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)),{manifestDirectory:dirname(manifest),generated:async()=>{throw Error("no generated source without actual compiler evidence")}});
 const record={owner:owner||"idml",test,files:inputs.length,bytes:inputs.reduce((n,r)=>n+r.bytes,0),lines:inputs.reduce((n,r)=>n+r.activeLines,0)};
 console.log(JSON.stringify(record));if(record.files>32||record.bytes>262144||record.lines>4000)throw Error("IDML owned source boundary exceeded");
}}
