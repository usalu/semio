import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
const apply=process.argv.includes("--apply");
const pat=/^((?:import|export)\s[^;\n]*?from\s*"([^"]+\.json)")\s*;/gmu;
let files=0,edits=0;
const walk=(d:string):void=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(!["node_modules","🧪️tests","dist","🗑️generated","target",".cache"].includes(e.name))walk(join(d,e.name));}else if(/\.(ts|tsx)$/u.test(e.name)){const p=join(d,e.name);let s:string;try{s=readFileSync(p,"utf8");}catch{continue;}let changed=false;const n=s.replace(pat,(m,head:string,spec:string)=>{if(spec.startsWith(".")||spec.startsWith("/"))return m;let r:string;try{r=Bun.resolveSync(spec,dirname(p));}catch{return m;}if(!r.endsWith(".json"))return m;changed=true;edits++;return head+' with { type: "json" };';});if(changed){files++;console.log(apply?"EDIT":"WOULD",p.slice(-90));if(apply)writeFileSync(p,n);}}}};
for(const root of ["🧰️framework/🔨️modules","🧰️framework/📦️packages","🧰️framework/🛍️products"])walk(root);
console.log(files,edits);
