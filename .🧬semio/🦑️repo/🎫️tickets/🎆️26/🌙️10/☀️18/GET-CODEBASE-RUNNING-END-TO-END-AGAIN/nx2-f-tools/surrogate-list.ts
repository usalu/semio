import {existsSync,readFileSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {createHash} from "node:crypto";
import {execFileSync} from "node:child_process";
import {dirname,relative,resolve,sep} from "node:path";
import {isBuiltin} from "node:module";
const root=process.cwd();
const lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const {buildTaskInputsV1}=await import(resolve(root,lib+"🏗️build/🧮️task-model/🟦️.ts"));
const parser=await import(resolve(root,lib+"⚡️caching/📥️inference/🟨️.mjs"));
const scripts=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"));
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const projects:any={},dependencies:any[]=[];
for(const [name,row] of Object.entries<any>(man.owners)){if(row.kind!=="project")continue;let frag:any;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}projects[name]={name,root:frag.root,namedInputs:frag.node?.namedInputs,targets:frag.node?.targets??{}};for(const e of frag.edges??[])dependencies.push({source:e.source,target:e.target});}
const policy=JSON.parse(readFileSync(lib+"🏗️build/🔣️policy.json","utf8"));
const graph={version:1,projects,dependencies,namedInputs:policy.namedInputs};
const inventory=execFileSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:root,maxBuffer:1<<30}).toString("utf8").split("\0").filter(Boolean);
const designationPath=resolve(root,lib+"🏃️process/🔌️providers/🔣️.json"),designation=JSON.parse(readFileSync(designationPath,"utf8")).providers as Record<string,string>;
const presentation=new Set<string>((policy.presentationResources??[]).map((x:string)=>resolve(root,x)));
const regular=(p:string)=>existsSync(p)&&statSync(p).isFile(),skip=(p:string)=>p.split(sep).some(x=>x==="node_modules"||x===".cache");
const edgesCache=new Map<string,string[]>();
const edges=(path:string)=>{if(edgesCache.has(path))return edgesCache.get(path)!;const out:string[]=[];if(/\.[cm]?[jt]sx?$/u.test(path)){const text=readFileSync(path,"utf8");
 for(const spec of parser.commandSourceImports(path,text)){const p=resolve(dirname(path),spec);const r=regular(p)?p:(()=>{try{return Bun.resolveSync(spec,dirname(path));}catch{return undefined}})();if(r&&!skip(r)&&r.startsWith(root))out.push(r);}
 for(const spec of parser.runtimeModuleSourceImports(path,text)){if(spec.startsWith(".")||isBuiltin(spec)||/^[a-z][a-z0-9+.-]*:/iu.test(spec))continue;try{const r=Bun.resolveSync(spec,dirname(path));if(!skip(r)&&r.startsWith(root))out.push(r);}catch{}}
 for(const lit of parser.commandUrlReferences(path,text)){const t=resolve(dirname(path),lit);if(regular(t)&&!skip(t))out.push(t);}
 const ports=scripts.scriptProviderPortsV1(text);if(ports.length){out.push(designationPath);for(const n of ports.includes("*")?Object.keys(designation):ports.filter((n:string)=>Object.hasOwn(designation,n)))out.push(resolve(dirname(designationPath),designation[n]!));}}
 edgesCache.set(path,out);return out;};
const closure=(entry:string)=>{const seen=new Set([entry]),q=[entry];while(q.length){const p=q.shift()!;if(presentation.has(p))continue;for(const to of edges(p)){if(seen.has(to)||!regular(to))continue;seen.add(to);q.push(to);}}return seen;};
const out:Record<string,any>={};
for(const spec of process.argv.slice(2)){
  const [project,target]=spec.split("#");const def=projects[project!].targets[target!];
  const entries=buildTaskInputsV1(graph,{project,definition:def});
  const patterns=entries.filter((e:any)=>e.kind==="file");
  const pos=patterns.filter((e:any)=>!e.exclude).map((e:any)=>e.pattern.replace("{workspaceRoot}/","")),neg=patterns.filter((e:any)=>e.exclude).map((e:any)=>e.pattern.replace("{workspaceRoot}/",""));
  const posGlobs=pos.map((p:string)=>new Bun.Glob(p)),negGlobs=neg.map((p:string)=>new Bun.Glob(p));
  const declared=inventory.filter(f=>posGlobs.some(g=>g.match(f))&&!negGlobs.some(g=>g.match(f)));
  const command=String(def.execution?.command??"").replace(/^repo\s+/,"").split(/\s+/)[0]!.replace(/^["']|["']$/g,"");
  const cwd=resolve(root,def.execution?.cwd??projects[project!].root);
  const entry=resolve(cwd,command);
  const files=new Set<string>(declared.map(f=>resolve(root,f)));
  if(regular(entry))for(const f of closure(entry))files.add(f);
  const hash=createHash("sha256");
  for(const f of [...files].sort()){hash.update(relative(root,f)+"\0");hash.update(regular(f)?createHash("sha256").update(readFileSync(f)).digest("hex"):"missing");}
  if(process.env.LIST)require("node:fs").writeFileSync(process.env.LIST+"."+target+".txt",[...files].map(f=>relative(root,f)).sort().join("\n"));out[spec]={files:files.size,declared:declared.length,entry:relative(root,entry).slice(-60),digest:hash.digest("hex")};
}
console.log(JSON.stringify({generation:cur.generation,rows:out},null,1));
