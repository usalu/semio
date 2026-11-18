import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/ce/l";
const names=readdirSync(D).filter(n=>n.endsWith(".json.br"));const N=Number(process.argv[2]),pick=[];for(let i=0;i<names.length&&pick.length<N;i+=Math.floor(names.length/N))pick.push(names[i]);
const seen=new Set<string>(),paths=new Set<string>();let total=0;const kinds:Record<string,number>={};
for(const n of pick){const rows=JSON.parse(brotliDecompressSync(readFileSync(D+"/"+n)).toString("utf8"));total+=rows.length;for(const r of rows){seen.add(r.kind+"\0"+r.path+"\0"+JSON.stringify(r.fact));paths.add(r.path);kinds[r.kind]=(kinds[r.kind]||0)+1;}}
console.log(JSON.stringify({layers:pick.length,total,distinctRows:seen.size,distinctPaths:paths.size,kinds}));
