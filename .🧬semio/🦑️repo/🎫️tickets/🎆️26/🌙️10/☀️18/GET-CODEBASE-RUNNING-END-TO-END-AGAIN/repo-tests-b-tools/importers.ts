import{createRequire}from"node:module";import{resolve}from"node:path";
const[entry,target]=process.argv.slice(2);
const b=await createRequire(import.meta.url)("esbuild").build({entryPoints:[entry],bundle:true,format:"esm",platform:"node",write:false,outdir:"/tmp/x-unused",metafile:true,external:["node:*","bun:*"],logLevel:"silent",plugins:[{name:"light",setup(bd:any){bd.onResolve({filter:/.*/},(a:any)=>a.path.includes("⌨️command")?{path:a.path,external:true}:undefined);}}]});
for(const[p,v]of Object.entries<any>(b.metafile.inputs))for(const i of v.imports)if(i.path===target)console.log("IMPORTER",p,i.kind,i.original);
