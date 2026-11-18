import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const d="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce/l/";
const names=readdirSync(d).filter(n=>n.endsWith(".br"));const step=Math.max(1,Math.floor(names.length/Number(process.argv[2]||600)));let n=0,tok=0,tokMs:number[]=[];const mt:number[]=[];
for(let i=0;i<names.length;i+=step){try{const t=brotliDecompressSync(readFileSync(d+names[i])).toString("utf8");n++;if(t.includes("⟦analysis-owner⟧")){tok++;mt.push(statSync(d+names[i]).mtimeMs);}}catch{}}
mt.sort((a,b)=>a-b);console.log(JSON.stringify({layers:names.length,sampled:n,withToken:tok,share:tok/n,oldest:mt.length?new Date(mt[0]!).toISOString():null,newest:mt.length?new Date(mt.at(-1)!).toISOString():null}));
