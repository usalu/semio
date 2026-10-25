import{brotliDecompressSync}from"node:zlib";import{readFileSync}from"node:fs";
const bytes=readFileSync(process.argv[2]);const raw=brotliDecompressSync(bytes,{maxOutputLength:1<<30});
console.log("expanded",raw.length,"packed",bytes.length);
const v=JSON.parse(raw.toString("utf8"));
console.log("owners",v.owners.length,"rows",v.rows.length,"classes",v.classes.length);
const size=x=>Buffer.byteLength(JSON.stringify(x));
console.log("owners bytes",size(v.owners),"rows bytes",size(v.rows),"classes bytes",size(v.classes));
let rb=0,ob=0,pairs=0,rrefs=0;for(const c of v.classes){rb+=size(c.r);ob+=size(c.o);pairs+=c.r.length*c.o.length;rrefs+=c.r.length;}
console.log("class r bytes",rb,"o bytes",ob,"row refs",rrefs,"pairs",pairs);
const kinds=new Map();for(const r of v.rows){const k=r.kind;const e=kinds.get(k)||{n:0,b:0,path:0,fact:0};e.n++;e.b+=size(r);e.path+=size(r.path);e.fact+=size(r.fact);kinds.set(k,e);}
console.log([...kinds].sort((a,b)=>b[1].b-a[1].b).map(([k,e])=>k+" n="+e.n+" bytes="+e.b+" path="+e.path+" fact="+e.fact).join("\n"));
console.log("sample rows",JSON.stringify(v.rows.slice(0,3)),JSON.stringify(v.rows[v.rows.length>>1]));
const hist=new Map();for(const c of v.classes){const k=c.o.length;hist.set(k,(hist.get(k)||0)+1)};
console.log("class owner-count top",[...hist].sort((a,b)=>b[1]-a[1]).slice(0,10));
const big=[...v.classes].sort((a,b)=>b.r.length-a.r.length).slice(0,5).map(c=>[c.r.length,c.o.length]);console.log("biggest classes r,o",big);
