import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";const names=process.argv.slice(2);
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
console.log("generation",m.generation);
for(const n of names){const f=JSON.parse(brotliDecompressSync(readFileSync(join(g,"o",m.owners[n].fragment+".json.br"))).toString()).node;
 console.log(n,"total",JSON.stringify(f).length);for(const[k,v]of Object.entries(f.namedInputs??{}))console.log("  ",k,Array.isArray(v)?v.length:"?",JSON.stringify(v).length, JSON.stringify(v).slice(0,110));}
