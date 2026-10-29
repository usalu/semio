import {readFile} from "node:fs/promises";
import {resolve,relative,dirname} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
for(const owner of ["🧵️instance/📄️docs","🧵️instance","📜️declaration/👁️view"]){
 const manifest=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",owner,"📦️packages/🦀️rust/Cargo.toml"),text=await readFile(manifest,"utf8"),path=text.split("[lib]")[1].split(/^[[]/m)[0].match(/^path\s*=\s*"([^"]+)"/m)![1],entry=relative(process.cwd(),resolve(dirname(manifest),path));
 const inputs=await readRustSourceClosureV1([entry],process.cwd(),["target_arch=aarch64","target_os=macos","target_family=unix","unix","feature=artifact-app-testing"],async file=>await readFile(file).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));
 console.log(JSON.stringify({owner,files:inputs.length,bytes:inputs.reduce((n,r)=>n+r.bytes,0),lines:inputs.reduce((n,r)=>n+r.activeLines,0)}));
}
