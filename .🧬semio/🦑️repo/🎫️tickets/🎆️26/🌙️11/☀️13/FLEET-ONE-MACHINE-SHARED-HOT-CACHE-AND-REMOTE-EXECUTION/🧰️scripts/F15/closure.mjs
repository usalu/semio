import fs from "fs"; import path from "path";
const R="/Users/ueli/Documents/semio";
const parse=f=>Bun.TOML.parse(fs.readFileSync(f,"utf8"));
const wdc=new Map();const wdOf=r=>{if(!wdc.has(r)){let x={};try{x=parse(r+"/Cargo.toml").workspace?.dependencies||{}}catch{}wdc.set(r,x)}return wdc.get(r)};
const cache=new Map();
const load=d=>{if(!cache.has(d)){let t=null;try{t=parse(d+"/Cargo.toml")}catch{}cache.set(d,t)}return cache.get(d)};
const secs=(t,k)=>[t[k]||{},...Object.values(t.target||{}).map(x=>x[k]||{})];
const deps=(d,t,kinds)=>{const out=[];for(const k of kinds)for(const s of secs(t,k))for(const [n,v] of Object.entries(s)){if(typeof v!=="object")continue;let p,base=d;if(v.workspace){const wr=path.normalize(path.join(d,t.package?.workspace||"../.."));const w=wdOf(wr)[n];p=w&&w.path;base=wr}else p=v.path;if(p)out.push({dir:path.normalize(path.join(base,p)),opt:!!v.optional,name:n,feats:v.features||[]})}return out};
export const closure=(start,kinds=["dependencies"])=>{const seen=new Map();const st=[[start,null]];while(st.length){const [d,par]=st.pop();if(seen.has(d))continue;const t=load(d);if(!t)continue;seen.set(d,par);for(const e of deps(d,t,kinds))if(!e.opt)st.push([e.dir,d])}return seen};
export const name=d=>load(d)?.package?.name;
export {load,deps};
if(import.meta.main){const s=closure(process.argv[2]);for(const [d,p] of s)console.log(name(d)+"\t"+(p?name(p):"")+"\t"+d)}
