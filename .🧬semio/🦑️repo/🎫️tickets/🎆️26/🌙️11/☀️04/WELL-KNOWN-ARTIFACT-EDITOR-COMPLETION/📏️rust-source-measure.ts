import {resolve,dirname} from "node:path";
const {readRustSourceClosureV1}=await import(resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts"));
const [entry,manifest,cfgJson]=process.argv.slice(2);
if(!entry||!manifest)throw Error("expected entry path, manifest directory, and optional JSON cfg array");
const cfg=cfgJson?JSON.parse(cfgJson):["unix","target_arch=x86_64","target_os=linux","target_family=unix","target_pointer_width=64","target_endian=little","panic=unwind","debug_assertions"];
const root=resolve(".");const generated=[];
const inputs=await readRustSourceClosureV1([resolve(entry)],root,cfg,async p=>{const f=Bun.file(p);return await f.exists()?new Uint8Array(await f.arrayBuffer()):null},{manifestDirectory:resolve(manifest),generated:(file,output)=>generated.push({file,output})});
console.log(JSON.stringify({files:inputs.length,bytes:inputs.reduce((n,x)=>n+x.bytes,0),activeBytes:inputs.reduce((n,x)=>n+x.activeBytes,0),activeLines:inputs.reduce((n,x)=>n+x.activeLines,0),generated:generated.length}));
