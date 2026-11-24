import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const observation=await import(B+"👁️observation/🟨️.mjs");
const ce=process.argv[2],before=Number(process.argv[3]),count=Number(process.argv[4]??30);
const names=readdirSync(join(ce,"e")).map(n=>[n,statSync(join(ce,"e",n)).mtimeMs]).filter(([,t])=>t<before).sort((a,b)=>b[1]-a[1]).slice(0,count);
const layer=d=>JSON.parse(brotliDecompressSync(readFileSync(join(ce,"l",d+".json.br"))).toString("utf8"));
const kinds={},paths={};let stale=0,total=0;
for(const[n,t]of names){const e=JSON.parse(brotliDecompressSync(readFileSync(join(ce,"e",n))).toString("utf8"));total++;
 const rows=[...e.rows];let moved=0,first;for(const d of new Set(e.layers))try{rows.push(...layer(d));}catch{}
 for(const r of rows){let fresh=true;try{fresh=observation.observationRowFreshV1(r);}catch{}
  if(!fresh){moved++;kinds[r.kind]=(kinds[r.kind]??0)+1;if(first===undefined)first=r;paths[r.path.slice(-110)]=(paths[r.path.slice(-110)]??0)+1;}}
 if(moved>0)stale++;
 console.log(n.slice(0,8),new Date(t).toISOString().slice(11,19),"rows",rows.length,"moved",moved,first?first.kind+" "+first.path.slice(-90):"");}
console.log({total,stale,kinds});console.log(Object.entries(paths).sort((a,b)=>b[1]-a[1]).slice(0,15));
