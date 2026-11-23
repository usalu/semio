import {readFileSync,existsSync,statSync,readdirSync} from "node:fs";
import {dirname,resolve,join} from "node:path";
const root=process.argv[2],seen=new Set(),missing=new Map();
const norm=p=>p.split(String.fromCharCode(92)).join("/");
const files=d=>statSync(d).isDirectory()?readdirSync(d).flatMap(n=>files(join(d,n))):[d];
const visit=f=>{
 if(seen.has(f))return;seen.add(f);
 if(!/\.(m?[jt]s)$/.test(f))return;
 let t;try{t=readFileSync(f,"utf8");}catch{return;}
 for(const m of t.matchAll(/(?:from|import)\s*\(?\s*["'](\.{1,2}\/[^"']+)["']/g)){
  const p=resolve(dirname(f),m[1]);
  if(!existsSync(p))missing.set(norm(p).replace(norm(root)+"/",""),norm(f).replace(norm(root)+"/",""));else visit(p);
 }
};
for(const s of process.argv.slice(3))for(const f of files(join(root,s)))visit(f);
for(const [p,by] of missing)console.log(p+"\n   <- "+by);
console.error("visited",seen.size,"missing",missing.size);
