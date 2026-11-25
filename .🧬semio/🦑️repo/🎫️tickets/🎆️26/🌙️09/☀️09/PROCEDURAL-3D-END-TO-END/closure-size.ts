import {readFile} from "node:fs/promises";
import {resolve,dirname} from "node:path";
import {readRustSourceClosureV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const workspace=process.cwd(),cfg=["target_arch=wasm32","target_os=wasi","target_env=p2","target_pointer_width=32"];
for(const manifest of process.argv.slice(2)){
 const text=await readFile(manifest,"utf8"),lib=/^\[lib\][^\[]*?^path\s*=\s*"([^"]+)"/ms.exec(text)?.[1]??"src/lib.rs",entry=resolve(dirname(manifest),lib);
 const rows=await readRustSourceClosureV1([entry],workspace,cfg,async p=>await readFile(p).catch(e=>e.code==="ENOENT"?null:Promise.reject(e)));
 console.log(JSON.stringify({manifest,files:rows.length,bytes:rows.reduce((s,r)=>s+r.bytes,0),lines:rows.reduce((s,r)=>s+r.activeLines,0)}));
}
