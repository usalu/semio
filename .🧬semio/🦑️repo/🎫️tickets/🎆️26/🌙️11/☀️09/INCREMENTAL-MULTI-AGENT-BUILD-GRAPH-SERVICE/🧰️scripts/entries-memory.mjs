import{readdirSync,readFileSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const root="/home/deck/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",dir=process.argv[2];
const {createClosureStoreV1}=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs");
const store=createClosureStoreV1({directory:dir});
const mb=()=>{Bun.gc(true);const m=process.memoryUsage();return Math.round(m.rss/1048576)+" MB rss, heap "+Math.round(m.heapUsed/1048576)+" MB";};
console.log("[DEBUG] start",mb());
const ids=readdirSync(join(dir,"e")).filter(n=>n.endsWith(".br")).map(n=>JSON.parse(brotliDecompressSync(readFileSync(join(dir,"e",n))).toString()).id);
console.log("[DEBUG] ids",ids.length,mb());
const keep=[];let rows=0,layers=0;const s=performance.now();
for(const id of ids){const e=await store.load(id);if(!e)continue;keep.push(e);rows+=e.rows.length;layers+=e.digests.length;}
console.log("[DEBUG] loaded all",keep.length,"rows",rows,"layer refs",layers,Math.round(performance.now()-s),"ms",mb());
