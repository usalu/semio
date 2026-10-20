import {readdirSync,statSync,existsSync,readFileSync} from "node:fs";import {createHash} from "node:crypto";import {join,dirname} from "node:path";import {execFileSync} from "node:child_process";
const W="/Users/ueli/Documents/semio",np=join(W,".cache/b/g/m/inference/np");
const strip=s=>s.replace(/️/g,"");
const tracked=execFileSync("git",["-c","core.quotepath=false","ls-files","-z"],{cwd:W,maxBuffer:1<<28}).toString().split("\0");
const pj=new Set(tracked.filter(f=>strip(f.split("/").at(-1))===strip("📋️project.json")).map(f=>dirname(f)||"."));
const cg=new Set(tracked.filter(f=>f.endsWith("/Cargo.toml")||f==="Cargo.toml").map(f=>dirname(f)||"."));
const roots=new Set([...pj,...cg]);
const byHash=new Map([...roots].map(r=>[createHash("sha256").update(r).digest("hex").slice(0,24),r]));
const rows=readdirSync(np).filter(f=>f.endsWith(".json")).map(f=>({h:f.slice(0,-5),m:statSync(join(np,f)).mtimeMs}));
const t1=Date.parse("2026-10-20T06:30:00+02:00"),t2=Date.parse("2026-10-20T09:00:00+02:00");
const cls=r=>{const root=byHash.get(r.h);if(!root)return "unknown";const hasCargo=cg.has(root),hasPj=pj.has(root);return (hasPj?"pj":"nopj")+"/"+(hasCargo?"cargo":"nocargo");};
const g=(name,f)=>{const c={};for(const r of rows.filter(f)){c[cls(r)]=(c[cls(r)]??0)+1;}console.log(name,JSON.stringify(c));};
g("burst1 06:38-06:44",r=>r.m>=t1&&r.m<t2);g("burst2 09:49-09:53",r=>r.m>=t2);g("old (before 10-19)",r=>r.m<Date.parse("2026-10-19T00:00:00+02:00"));
const all={};for(const root of roots){const k=(pj.has(root)?"pj":"nopj")+"/"+(cg.has(root)?"cargo":"nocargo");all[k]=(all[k]??0)+1;}console.log("all roots",JSON.stringify(all),"np files",rows.length,"matched",rows.filter(r=>byHash.has(r.h)).length);
// persisted-projects without np
const have=new Set(rows.map(r=>r.h));const missing={};for(const [h,root] of byHash)if(!have.has(h)){const k=(pj.has(root)?"pj":"nopj")+"/"+(cg.has(root)?"cargo":"nocargo");missing[k]=(missing[k]??0)+1;}console.log("roots with no np file",JSON.stringify(missing));
