import {createRequire} from "node:module";
const require=createRequire(import.meta.url);
const b=await require("/Users/ueli/Documents/semio/node_modules/esbuild").build({entryPoints:[process.argv[2]],bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent"});
const needle=process.argv[3];
for(const [p,v] of Object.entries(b.metafile.inputs)) for(const i of v.imports) if(i.path.includes(needle)) console.log(p,"->",i.path,`(${i.original})`);
