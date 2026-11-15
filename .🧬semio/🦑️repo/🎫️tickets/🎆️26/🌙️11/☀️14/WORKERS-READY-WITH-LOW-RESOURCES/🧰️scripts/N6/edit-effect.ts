import {execFileSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
const root="/Users/Shared/semio",lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inf=await import(root+"/"+lib+"/⚡️caching/📥️inference/🟨️.mjs");
const {scriptProviderPortsV1}=await import(root+"/"+lib+"/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs").catch(()=>({scriptProviderPortsV1:undefined}));
const h=(v:unknown)=>createHash("sha256").update(JSON.stringify(v)).digest("hex").slice(0,12);
const files=execFileSync("git",["diff","--name-only","-z","HEAD","--",lib],{cwd:root,maxBuffer:1<<28}).toString().split("\0").filter(f=>/\.(ts|mjs|js|cjs)$/.test(f));
const run=(path:string,t:string)=>{const abs=root+"/"+path;const o:any={};
 const t_=(k:string,f:()=>unknown)=>{try{o[k]=h(f())}catch(e){o[k]="err"}};
 t_("cmd",()=>inf.commandSourceImports(abs,t));t_("url",()=>inf.commandUrlReferences(abs,t));t_("rrefs",()=>inf.runtimeModuleSourceReferences(abs,t));
 t_("mod",()=>[inf.moduleSourceImports(abs,t),inf.runtimeModuleSourceImports(abs,t),inf.productionModuleSourceImports(abs,t)]);
 if(scriptProviderPortsV1)t_("ports",()=>scriptProviderPortsV1(t));return o;};
let same=0,diff=0;const changedPaths:string[]=[],analysisPaths:string[]=[];const kinds:Record<string,number>={};const rows:string[]=[];
for(const f of files){let old:string;try{old=execFileSync("git",["show","HEAD:"+f],{cwd:root,maxBuffer:1<<28}).toString()}catch{continue}
 let now:string;try{now=readFileSync(root+"/"+f,"utf8")}catch{continue}
 if(old===now)continue;changedPaths.push(root+"/"+f);const a=run(f,old),b=run(f,now);const d=Object.keys(a).filter(k=>a[k]!==b[k]);
 if(d.length){analysisPaths.push(root+"/"+f);diff++;for(const k of d)kinds[k]=(kinds[k]??0)+1;rows.push(f.slice(lib.length)+" "+d.join(","));}else same++;}
require("node:fs").writeFileSync(process.argv[2]??"/dev/null",JSON.stringify({changed:changedPaths,analysisChanged:analysisPaths}));
console.log("changed files",same+diff,"analysis-identical",same,"analysis-changed",diff,kinds);console.log(rows.slice(0,40).join("\n"));
