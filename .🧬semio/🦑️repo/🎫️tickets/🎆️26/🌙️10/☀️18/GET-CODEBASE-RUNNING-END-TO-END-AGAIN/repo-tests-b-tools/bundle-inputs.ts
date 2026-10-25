import{createRequire}from"node:module";
const owner=process.argv[2];
const b=await createRequire(import.meta.url)("esbuild").build({entryPoints:[owner],bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent"});
const owners=["🧰️framework/🔨️modules/⚙️runtime/"];
for(const p of Object.keys(b.metafile.inputs))if(!(p.startsWith("🧰️framework/🔨️modules/🏃️process/")||owners.some(x=>p.startsWith(x))))console.log("OFFENDER",p);
console.log("total",Object.keys(b.metafile.inputs).length,Object.keys(b.metafile.inputs).includes("🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts"));
