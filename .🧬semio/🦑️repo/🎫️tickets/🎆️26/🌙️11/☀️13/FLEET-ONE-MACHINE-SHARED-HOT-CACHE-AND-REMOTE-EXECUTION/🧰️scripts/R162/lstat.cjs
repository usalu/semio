const fs=require("fs"),path=require("path");const root=fs.mkdtempSync(path.join(process.env.SCR,"t2-"));const ps=[];
for(let a=0;a<20;a++)for(let b=0;b<20;b++){const d=path.join(root,"a"+a,"b"+b);fs.mkdirSync(d,{recursive:true});for(let c=0;c<20;c++){const p=path.join(d,"c"+c);fs.mkdirSync(p);ps.push(p);}}
let t=performance.now();for(const p of ps)fs.lstatSync(p,{bigint:true});const big=performance.now()-t;
t=performance.now();for(const p of ps)fs.lstatSync(p);const num=performance.now()-t;
t=performance.now();for(const p of ps)fs.lstatSync(p,{bigint:true,throwIfNoEntry:false});const nt=performance.now()-t;
const e=fs.lstatSync(ps[0],{bigint:true});t=performance.now();for(const p of ps){JSON.stringify([ "directory",String(e.dev),String(e.ino),Number(e.mode)]);[e.dev,e.ino,e.mode,e.mtimeNs,e.ctimeNs].map(String).join(":");}const js=performance.now()-t;
console.log("n",ps.length,"lstat bigint us/call",(big*1000/ps.length).toFixed(2),"number",(num*1000/ps.length).toFixed(2),"bigint noThrow",(nt*1000/ps.length).toFixed(2),"stringify+stamp us",(js*1000/ps.length).toFixed(2));
fs.rmSync(root,{recursive:true});
