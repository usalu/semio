import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {pathToFileURL} from "node:url";
const lib=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const s=await import(pathToFileURL(lib+"🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts").href),v=s.buildSchedulingV1();
const g=".cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const [project,target]=process.argv.slice(2);
const frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+man.owners[project].fragment+".json.br")).toString("utf8"));
const ni=frag.node.namedInputs,t=frag.node.targets[target];
const seen=new Set();
const expand=(name,depth=0)=>{ if(seen.has(name))return[]; seen.add(name); const l=ni[name]; if(!Array.isArray(l))return[]; return l.flatMap(i=>typeof i==="string"?(i.startsWith("{workspaceRoot}/")||i.startsWith("!")?[i]:expand(i.replace(/^\^/,""),depth+1)):[]);};
for(const i of t.inputs??[]) if(typeof i==="string") { const e=expand(i); const files=e.filter(x=>x.startsWith("{workspaceRoot}/")); let sched=0,doms={}; for(const f of files){const d=s.buildSchedulingDomainOfV1(f.replace("{workspaceRoot}/",""),v);if(d){sched++;doms[d]=(doms[d]??0)+1}} console.log(i.padEnd(26),"entries",e.length,"files",files.length,"scheduling",sched,JSON.stringify(doms)); }
