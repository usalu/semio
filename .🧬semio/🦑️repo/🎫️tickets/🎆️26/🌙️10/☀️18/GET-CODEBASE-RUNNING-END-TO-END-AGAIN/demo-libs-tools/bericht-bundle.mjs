import{createRequire}from"node:module";import{join,resolve}from"node:path";import{readFileSync}from"node:fs";
const ownerRoot=resolve(process.argv[2]);const require=createRequire(join(ownerRoot,"📦️packages/🟦️typescript/📜️script.ts"));
const b=JSON.parse(readFileSync(join(ownerRoot,"🧪️tests/🧫️fixtures/🧭️commands.json"),"utf8"));
for(const entry of b.entries){const r=await require("esbuild").build({entryPoints:[join(ownerRoot,entry)],bundle:true,write:false,metafile:true,platform:"node",format:"esm",packages:"external",logLevel:"silent"});
const inputs=Object.keys(r.metafile.inputs);const external=[...new Set(Object.values(r.metafile.outputs).flatMap(o=>o.imports).filter(i=>i.external&&!i.path.startsWith("node:")&&i.path!=="bun").map(i=>i.path.startsWith("@")?i.path.split("/").slice(0,2).join("/"):i.path.split("/")[0]))].sort();
console.log(inputs.join("\n"));console.log(entry,inputs.length,JSON.stringify(external),"expected",JSON.stringify(b.externalDependencies[entry]),"forbidden hits",b.forbidden.filter(f=>inputs.some(p=>p.includes(f))));}
