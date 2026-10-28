import {pathToFileURL,fileURLToPath} from "node:url";
import {readFileSync,existsSync,lstatSync} from "node:fs";import {createRequire} from "node:module";const require=createRequire(import.meta.url);
import {resolve,dirname,relative} from "node:path";
const root=process.cwd(),lib=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const m=await import(pathToFileURL(lib+"🟨️.mjs").href);
// closure re-implemented: roots like nxBuildEngineClosureV1, but skipping selected edges
const skipFile=new RegExp(process.argv[2]??"^$"),skipKind=process.argv[3]??"",skipSpec=new RegExp(process.argv[4]??".");
const pending=[lib+"🟨️.mjs",lib+"📦️packages/🟦️typescript/📋️executors.json",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🏃️execution/🟨️.mjs",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/📜️script.ts",lib+"⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟨️.mjs"];
const files=new Set(),via=new Map();
const admit=(owner,spec)=>{
 if(spec.startsWith(".")){const c=m.nxBuildEngineSourceFileV1(owner,spec);if(!relative(root,c).startsWith(".."))pending.push(c);}
 else if(!spec.includes(":")){try{const w=m.nxBuildEngineWorkspacePackageFileV1(root,spec);if(w)pending.push(w);}catch{}}
};
while(pending.length){const p=pending.pop();if(files.has(p))continue;files.add(p);if(p.endsWith(".json"))continue;let refs;try{refs=m.nxBuildEngineReferencesV1(readFileSync(p,"utf8"));}catch{continue}
 for(const r of refs){ if(skipFile.test(relative(root,p))&&(skipKind===""||r.kind===skipKind)&&skipSpec.test(r.specifier))continue;
  try{ if(r.kind==="module")admit(p,r.specifier); else if(r.kind==="file-url"){const t=resolve(dirname(p),r.specifier);if(!existsSync(t)||!lstatSync(t).isDirectory())admit(p,r.specifier);} }catch{} }}
console.log("closure",files.size);if(process.argv[5]){const out=[...files].map(f=>relative(root,f));require("node:fs").writeFileSync(process.argv[5],out.sort().join("\n")+"\n");}
