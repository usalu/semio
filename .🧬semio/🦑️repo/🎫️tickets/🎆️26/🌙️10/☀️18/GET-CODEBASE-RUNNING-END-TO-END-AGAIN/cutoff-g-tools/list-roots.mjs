import{readdirSync,readFileSync,writeFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const every=Number(process.argv[3]??100),roots=[];
for(const n of readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n)).filter((_,i)=>i%every===0)){try{const t=brotliDecompressSync(readFileSync(D+n)).toString("utf8");const v=JSON.parse(t.slice(t.indexOf("\n")+1));if(v.root)roots.push(v.root);}catch{}}
writeFileSync(process.argv[2],roots.join("\n")+"\n");console.log(roots.length,"roots");
