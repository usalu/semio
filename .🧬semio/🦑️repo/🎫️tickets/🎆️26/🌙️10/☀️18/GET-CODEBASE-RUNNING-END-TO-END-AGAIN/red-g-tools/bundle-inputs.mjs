import {createRequire} from "node:module";
const require=createRequire(import.meta.url);
const owner=process.argv[2];
const b=await require("/Users/ueli/Documents/semio/node_modules/esbuild").build({entryPoints:[owner],bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent"});
const own=["🧰️framework/🔨️modules/🏃️process/","🧰️framework/🔨️modules/🧬️schema/✅️validator/","🧰️framework/🔨️modules/⚙️runtime/","🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/","🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/","🧰️framework/📦️packages/🟦️typescript/📁️filesystem/🧾️observation/📁️descriptor/"];
for(const p of Object.keys(b.metafile.inputs)) if(!own.some(x=>p.startsWith(x))) console.log(p);
