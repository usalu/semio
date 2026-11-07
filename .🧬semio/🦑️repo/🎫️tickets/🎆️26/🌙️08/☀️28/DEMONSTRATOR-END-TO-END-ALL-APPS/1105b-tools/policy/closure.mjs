import {readFileSync,existsSync,writeFileSync} from "node:fs";
import {resolve,dirname,relative} from "node:path";
import {execFileSync} from "node:child_process";
const R=process.cwd(),cache=new Map();
const read=p=>{if(!cache.has(p)){try{cache.set(p,Bun.TOML.parse(readFileSync(p,"utf8")))}catch{cache.set(p,{})}}return cache.get(p)};
const tomls=execFileSync("git",["ls-files","-z","--","*Cargo.toml"],{maxBuffer:1<<28}).toString().split("\0").filter(p=>p&&!p.startsWith(".🧬semio")).map(p=>resolve(R,p));
const byname=new Map();for(const p of tomls){const n=read(p).package?.name;if(n){if(!byname.has(n))byname.set(n,[]);byname.get(n).push(p)}}
const wsRoot=p=>{let d=dirname(p);for(;;){const c=resolve(d,"Cargo.toml");if(existsSync(c)&&read(c).workspace)return c;if(d===R||d===dirname(d))return null;d=dirname(d)}};
const deps=(p,dev)=>{const v=read(p);return [v,...Object.values(v.target??{})].flatMap(r=>["dependencies","build-dependencies",...(dev?["dev-dependencies"]:[])].flatMap(k=>Object.entries(r[k]??{})))};
const res=(p,n,s)=>{if(s&&typeof s==="object"){if(s.workspace){const w=wsRoot(p);if(!w)return null;const x=read(w).workspace?.dependencies?.[n];return x&&typeof x==="object"&&x.path?resolve(dirname(w),x.path,"Cargo.toml"):null}if(s.path)return resolve(dirname(p),s.path,"Cargo.toml")}return null};
const seen=new Map(),todo=[];const missing=[];
for(const n of process.argv.slice(2)){if(!byname.has(n))missing.push(n);for(const p of byname.get(n)??[])todo.push([p,true])}
while(todo.length){const[p,dev]=todo.pop();if(seen.has(p))continue;seen.set(p,read(p).package?.name);for(const[n,s]of deps(p,dev)){const t=res(p,n,s);if(t&&existsSync(t)&&!seen.has(t))todo.push([t,false])}}
writeFileSync(process.env.OUT,JSON.stringify(Object.fromEntries([...seen].map(([k,v])=>[relative(R,k),v]))));
console.log(seen.size,"missing",missing);
