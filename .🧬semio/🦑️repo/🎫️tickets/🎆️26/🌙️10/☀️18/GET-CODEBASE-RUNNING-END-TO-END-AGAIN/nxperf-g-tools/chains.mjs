import {pathToFileURL} from "node:url";
import {readFileSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
const root=process.cwd();
const mod = await import(pathToFileURL(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs").href);
const files=mod.nxBuildEngineClosureV1().files;
const set=new Set(files);
const edges=new Map();
for(const f of files){ if(f.endsWith(".json"))continue; const abs=resolve(root,f); let refs=[]; try{refs=mod.nxBuildEngineReferencesV1(readFileSync(abs,"utf8"));}catch{continue}
 const out=[]; for(const r of refs){ if(r.kind==="file-url"){ try{const t=relative(root,mod.nxBuildEngineSourceFileV1(abs,r.specifier)); if(set.has(t))out.push(t);}catch{} } else if(r.kind==="module"&&r.specifier.startsWith(".")){ try{const t=relative(root,mod.nxBuildEngineSourceFileV1(abs,r.specifier)); if(set.has(t))out.push(t);}catch{} } else if(r.kind==="module"){ try{const w=mod.nxBuildEngineWorkspacePackageFileV1(root,r.specifier); if(w){const t=relative(root,w); if(set.has(t))out.push(t);}}catch{} } }
 edges.set(f,out);}
const lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const roots=(process.env.ROOTS?process.env.ROOTS.split(","):[lib+"🟨️.mjs",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🏃️execution/🟨️.mjs",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/📜️script.ts",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs"]).filter(r=>set.has(r));
const parent=new Map(roots.map(r=>[r,null]));const q=[...roots];
while(q.length){const f=q.shift();for(const t of edges.get(f)??[])if(!parent.has(t)){parent.set(t,f);q.push(t);}}
const pattern=new RegExp(process.argv[2]);
const short=p=>p.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","LIB/").replace("🧰️framework/🔨️modules/","FM/");
let n=0;
for(const f of files){ if(!pattern.test(f))continue; const chain=[];for(let x=f;x;x=parent.get(x))chain.push(short(x)); if(n++<Number(process.argv[3]??5))console.log(chain.reverse().join("\n  -> "),"\n"); }
console.log("matched",n,"reachable-from-start",parent.size,"of",files.length);
