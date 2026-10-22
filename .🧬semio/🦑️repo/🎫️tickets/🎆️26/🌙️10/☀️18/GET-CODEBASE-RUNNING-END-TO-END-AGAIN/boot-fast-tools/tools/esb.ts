import {build} from "esbuild";
const r=await build({entryPoints:[process.argv[2]!],bundle:true,format:"esm",platform:"node",write:false,outdir:"/tmp/esb-out",metafile:true,external:["node:*","bun:*"],logLevel:"silent",plugins:[{name:"light",setup(b){b.onResolve({filter:/.*/},args=>args.path.includes("⌨️command")?{path:args.path,external:true}:undefined);}}]});
const inputs=Object.keys(r.metafile!.inputs).filter(p=>!p.endsWith(".json"));
console.log(inputs.length);console.log(inputs.map(p=>p.split("/").slice(-3).join("/")).join("\n"));
