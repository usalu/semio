import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const G="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const g=await import(G);
const rss=()=>Math.round(process.memoryUsage().rss/1048576),heap=()=>{Bun.gc(true);return Math.round(process.memoryUsage().heapUsed/1048576)};
const bytes=readFileSync("index.facts.br");
console.log("baseline rss",rss(),"heap",heap());
const raw=brotliDecompressSync(bytes,{maxOutputLength:4e9});
console.log("raw expanded MB",Math.round(raw.length/1048576),"rss",rss());
const mode=process.argv[2];
if(mode==="lazy"){const v=g.openRawFactIndexV1(raw,4e9,"m");console.log("lazy keys",Object.keys(v).slice(0,12),"rss",rss(),"heap",heap());}
if(mode==="full"){const t=performance.now();const v=g.unpackFactIndexV1(bytes,4e9);console.log("full: owners",v.owners.length,"rows",v.rows.length,"classes",v.classes.length,"ms",Math.round(performance.now()-t),"rss",rss(),"heap",heap());}
