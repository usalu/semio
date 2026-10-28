import{readFileSync,readdirSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const R="/Users/ueli/Documents/semio/";
const names=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n)).filter((_,i)=>i%Number(process.argv[2]??150)===0);
const agg={};let n=0;const lib={};
for(const name of names){let text;try{text=brotliDecompressSync(readFileSync(D+name)).toString("utf8");}catch{continue;}
 const header=JSON.parse(text.slice(0,text.indexOf("\n")));let rows=[...header.observations];
 for(const b of header.blocks??[]){try{rows.push(...JSON.parse(brotliDecompressSync(readFileSync(D+"ob/"+b+".br")).toString("utf8")));}catch{}}
 n++;const k={};for(const r of rows){k[r.kind]=(k[r.kind]??0)+1;if(r.kind==="analysis"&&r.path.includes("🏗️build/"))lib.buildAnalysis=(lib.buildAnalysis??0)+1;}
 for(const[a,b]of Object.entries(k))agg[a]=(agg[a]??0)+b;}
console.log("records",n,JSON.stringify(Object.fromEntries(Object.entries(agg).map(([a,b])=>[a,Math.round(b/n)]))),JSON.stringify(lib));
