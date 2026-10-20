import {readdirSync,statSync} from "node:fs";import {createHash} from "node:crypto";import {join,dirname} from "node:path";import {execFileSync} from "node:child_process";
const W="/Users/ueli/Documents/semio",np=join(W,".cache/b/g/m/inference/np"),strip=s=>s.replace(/️/g,"");
const tracked=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:W,maxBuffer:1<<28}).toString().split("\0");
const pj=new Set(tracked.filter(f=>strip(f.split("/").at(-1))===strip("📋️project.json")).map(f=>dirname(f)||"."));
const cg=new Set(tracked.filter(f=>f.endsWith("/Cargo.toml")).map(f=>dirname(f)));
const byHash=new Map([...new Set([...pj,...cg])].map(r=>[createHash("sha256").update(r).digest("hex").slice(0,24),r]));
const c={};let n=0;
for(const d of readdirSync(np).filter(f=>f.endsWith(".records"))){let fs;try{fs=readdirSync(join(np,d));}catch{continue;}if(!fs.some(f=>f.includes("-88833-")&&f.endsWith(".pack")))continue;n++;const root=byHash.get(d.slice(0,24));const k=root===undefined?"unknown":(pj.has(root)?"pj":"nopj")+(cg.has(root)?"/cargo":"");c[k]=(c[k]??0)+1;}
console.log("stores with a pid-88833 pack",n,JSON.stringify(c));
