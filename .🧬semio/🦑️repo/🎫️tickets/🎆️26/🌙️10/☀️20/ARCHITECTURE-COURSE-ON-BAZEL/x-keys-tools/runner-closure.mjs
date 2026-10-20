import {build} from "esbuild";import {readFileSync} from "node:fs";
const entry=process.argv[2],list=new Set(readFileSync(process.argv[3],"utf8").split("\n").filter(Boolean));
const r=await build({absWorkingDir:process.cwd(),entryPoints:[entry],bundle:true,write:false,metafile:true,platform:"node",format:"esm",packages:"external",logLevel:"silent"});
const inputs=Object.keys(r.metafile.inputs).map(p=>p.replace(/ with \{.*\}$/u,""));
console.log(JSON.stringify({closureFiles:inputs.length,noArgCallersInClosure:inputs.filter(p=>list.has(p))},null,1));
