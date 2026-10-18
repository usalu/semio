import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
const R="/Users/ueli/Documents/semio";
const PRUNED = new Set(["node_modules", "target", "dist", "🗑️generated", "🤖️generated", ".git"]);
const files=(root)=>readdirSync(root).flatMap((n)=>{ if(PRUNED.has(n))return[]; const p=join(root,n); return statSync(p).isDirectory()?files(p):n==="🔣️.json"?[p]:[]});
const ann=(node,f=[])=>{ if(Array.isArray(node))for(const c of node)ann(c,f); else if(node&&typeof node==="object")for(const[k,c]of Object.entries(node)){if(k.startsWith("x-semio-"))f.push(k);ann(c,f)} return f};
const voc=JSON.parse(readFileSync(R+"/🧰️framework/🔨️modules/🧬️schema/🧬️vendor-annotation-vocabulary/🔣️.json","utf8"));
const declared=Object.keys(voc.keywords??voc);
console.log("declared",declared);
const hits={};
for(const root of [R+"/🧰️framework/🛍️products/💻️os",R+"/🌎️hub"]) for(const p of files(root)){ let d; try{d=JSON.parse(readFileSync(p,"utf8"))}catch{continue} for(const k of ann(d)) if(!declared.includes(k)) (hits[k]??=new Set()).add(p.replace(R+"/",""));}
for(const[k,v]of Object.entries(hits))console.log(k,[...v].slice(0,4),v.size);
