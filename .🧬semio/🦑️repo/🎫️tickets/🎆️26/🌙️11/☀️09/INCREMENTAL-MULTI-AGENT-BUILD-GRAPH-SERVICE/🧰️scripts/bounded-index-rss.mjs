import{readFileSync}from"node:fs";
import{unpackFactIndexV1,openPackedFactIndexV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const mode=process.argv[2],file="/home/deck/semio/.cache/b/g/m/generations/x/b0d47698ed98a6115117adb4c57bc5125dd1325ec9f56cd36e906124eb86a782.facts.br";
const mb=n=>Math.round(n/1048576),rss=()=>{Bun.gc?.(true);return mb(process.memoryUsage().rss)};
const base=rss(),bytes=readFileSync(file),t=performance.now();let index;
if(mode==="full")index=unpackFactIndexV1(bytes,1<<30);else index=openPackedFactIndexV1(bytes,1<<30);
let touched=0;if(mode==="open-all"||mode==="full"){for(let i=0;i<index.rows.length;i++){const r=index.rows[i];if(r.kind)touched++;}}
if(mode==="open-all"||mode==="full"){let ids=0;for(const cls of index.classes)ids+=cls.o.length+cls.r.length;touched+=ids*0;}
const out={caches:index.stats?.().caches.map(c=>c.name.split(":").pop()+"="+c.entries+"/"+c.maxEntries+" "+Math.round(c.bytes/1048576)+"MB"),mode,rows:index.rows.length,owners:index.owners.length,touched,baseMb:base,afterMb:rss(),heapMb:mb(process.memoryUsage().heapUsed),externalMb:mb(process.memoryUsage().external),abMb:mb(process.memoryUsage().arrayBuffers),ms:Math.round(performance.now()-t)};
console.log(JSON.stringify(out));
