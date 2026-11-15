import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const dir=process.argv[2]+"/l",names=readdirSync(dir).map(f=>[f,statSync(dir+"/"+f).size] as const).sort((a,b)=>b[1]-a[1]);
const seen=new Set<string>();let total=0,bytes=0,distinctBytes=0;const sample=[...names.slice(0,150),...names.filter((_,i)=>i%100===0).slice(0,200)];
const t=performance.now();
for(const[f]of sample){const rows=JSON.parse(brotliDecompressSync(readFileSync(dir+"/"+f)).toString("utf8")) as unknown[];for(const row of rows){const s=JSON.stringify(row);total++;bytes+=s.length;if(!seen.has(s)){seen.add(s);distinctBytes+=s.length;}}}
console.log(JSON.stringify({layers:sample.length,rows:total,distinct:seen.size,dupFraction:+(1-seen.size/total).toFixed(4),MB:Math.round(bytes/1e6),distinctMB:Math.round(distinctBytes/1e6),ms:Math.round(performance.now()-t)}));
