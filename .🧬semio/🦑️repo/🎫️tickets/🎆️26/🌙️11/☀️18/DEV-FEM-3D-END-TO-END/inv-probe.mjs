import {readdir,lstat} from "node:fs/promises";import {join} from "node:path";
const root=process.argv[2],limit=Number(process.argv[3]);
const skip=new Set(["node_modules","target",".git","🗑️generated"]);
const t0=performance.now();let dirs=0,files=[];const pending=[""];
while(pending.length&&files.length<limit){const batch=pending.splice(-32);const ls=await Promise.all(batch.map(d=>readdir(join(root,d),{withFileTypes:true}).then(e=>({d,e}),()=>({d,e:[]}))));for(const{d,e}of ls){dirs++;for(const x of e){if(x.isSymbolicLink())continue;const p=join(d,x.name);if(x.isDirectory()){if(!skip.has(x.name))pending.push(p);}else files.push(p);}}}
const t1=performance.now();
for(let i=0;i<files.length;i+=64)await Promise.all(files.slice(i,i+64).map(f=>lstat(join(root,f)).catch(()=>0)));
const t2=performance.now();
console.log(JSON.stringify({dirs,files:files.length,walkMs:Math.round(t1-t0),statMs:Math.round(t2-t1),statPerSec:Math.round(files.length/((t2-t1)/1000))}));
