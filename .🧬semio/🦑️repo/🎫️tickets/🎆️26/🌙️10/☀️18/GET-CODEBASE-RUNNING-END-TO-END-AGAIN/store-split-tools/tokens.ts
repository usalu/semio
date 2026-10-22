import {readRustSourceClosureV1,selectRustSourceV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {rustTokens} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import {readFileSync} from "node:fs";import {readFile} from "node:fs/promises";import {resolve} from "node:path";
const cfg=["target_arch=aarch64","target_os=macos","target_env=","target_pointer_width=64","unix"];
const rows=await readRustSourceClosureV1([resolve("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os","🔨️modules/🏪️store/🧩️composition/👥️space-member/🦀️.rs")],"/Users/ueli/Documents/semio",cfg,async p=>await readFile(p).catch((e:any)=>e.code==="ENOENT"?null:Promise.reject(e)));
for(const r of rows.filter(r=>r.kind==="source")){const t=rustTokens(selectRustSourceV1(readFileSync(resolve("/Users/ueli/Documents/semio",r.path),"utf8"),cfg));const hit=t.filter(x=>["os_store","os_spr","os_directory","semio_framework_os_kernel","semio_framework_plugin"].includes(x.text));if(hit.length)console.log(r.path,hit.map(x=>x.text));}
