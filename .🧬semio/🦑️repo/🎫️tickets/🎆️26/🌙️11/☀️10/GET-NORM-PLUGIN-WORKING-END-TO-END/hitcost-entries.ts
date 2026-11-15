import{readdirSync,readFileSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const dir=process.argv[2],names=readdirSync(dir).filter(n=>n.endsWith(".json.br"));
const sizes=names.map(n=>[n,statSync(dir+"/"+n).size]).sort((a,b)=>b[1]-a[1]);
const pick=[sizes[0],sizes[Math.floor(sizes.length/2)],sizes[sizes.length-1]];
for(const[n,size]of pick){const text=brotliDecompressSync(readFileSync(dir+"/"+n)).toString("utf8"),r=JSON.parse(text);const kinds:Record<string,number>={};for(const row of r.rows)kinds[row.kind]=(kinds[row.kind]??0)+1;console.log(n.slice(0,8),"packed",size,"text",text.length,"rows",r.rows.length,"layers",r.layers.length,JSON.stringify(kinds),"recipe",JSON.stringify(r.recipe).slice(0,100));}
const total=sizes.reduce((s,x)=>s+x[1],0);console.log("entries",names.length,"packed total MB",Math.round(total/1e6));
