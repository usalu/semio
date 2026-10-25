import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const [,, idxPath, manifestPath, rowId]=process.argv;
const idx=JSON.parse(brotliDecompressSync(readFileSync(idxPath)).toString()),m=JSON.parse(readFileSync(manifestPath,"utf8"));
const owners=new Set();idx.classes.forEach(c=>{if(c.r.includes(+rowId))for(const o of c.o)owners.add(idx.owners[o]);});
const dirty=new Set(m.built.dirty);let both=0;for(const o of owners)if(dirty.has(o))both++;
console.log("row owners",owners.size,"dirty in gen",m.generation,dirty.size,"intersection",both);
console.log([...owners].slice(0,5));
