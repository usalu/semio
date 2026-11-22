import fs from "node:fs";
const stats=new Map();const orig=fs.readdirSync;
fs.readdirSync=function(p,...a){const t=performance.now();const r=orig.call(this,p,...a);const d=performance.now()-t;const k=String(p).replace("/Users/ueli/Documents/semio/","");const s=stats.get(k)??{n:0,ms:0,len:0};s.n++;s.ms+=d;s.len=r.length;stats.set(k,s);return r;};
process.on("exit",()=>{const rows=[...stats].sort((a,b)=>b[1].ms-a[1].ms);let tot=0,n=0;for(const [,s] of rows){tot+=s.ms;n+=s.n;}console.log("readdir calls",n,"ms",Math.round(tot),"distinct dirs",rows.length);for(const [k,s] of rows.slice(0,12))console.log(Math.round(s.ms)+"ms",s.n+"x",s.len+" entries",k.slice(-110));});
