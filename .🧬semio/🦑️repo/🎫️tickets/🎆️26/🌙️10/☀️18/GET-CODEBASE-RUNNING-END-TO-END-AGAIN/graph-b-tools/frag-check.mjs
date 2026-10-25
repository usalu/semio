import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations";
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>b[1]-a[1]);const m=JSON.parse(readFileSync(join(g,"m",ms[0][0])));
console.log("generation",m.generation,"built",m.built.kind,m.built.dirty.length);
const names=Object.keys(m.owners).filter(n=>m.owners[n].kind==="project");let bad=0,tok=0,n=0;
for(let i=0;i<names.length;i+=Math.ceil(names.length/300)){const f=JSON.parse(brotliDecompressSync(readFileSync(join(g,"o",m.owners[names[i]].fragment+".json.br"))).toString());const s=JSON.stringify(f);n++;if(s.includes("closureRef")||s.includes("closureRecipes"))bad++;if(/SEMIO_SOURCE_CLOSURE_[0-9a-f]{64}/.test(s))tok++;}
console.log("sampled",n,"fragments with leftover markers",bad,"with concrete tokens",tok);
