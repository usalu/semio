import {readFileSync} from "node:fs";
import {join} from "node:path";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {openPackedFactIndexV1,unpackFactIndexV1}=await import(G);
const [dir,digest,mode]=process.argv.slice(2);
const bytes=readFileSync(join(dir,"x",digest+".facts.br"));
const t0=performance.now();
const index=(mode==="unpack"?unpackFactIndexV1:openPackedFactIndexV1)(bytes,1<<30);
console.log("opened",Math.round(performance.now()-t0),"ms rss",Math.round(process.memoryUsage().rss/1048576),"MB; rowsRead",typeof index.rowsRead, Object.getOwnPropertyDescriptor(index,"rows")?.get?"getter":"data");
let n=0,sum=0;
for(let i=0;i<index.rows.length;i++){const r=index.rows[i];n++;sum+=r.path.length;}
console.log("iterated",n,Math.round(performance.now()-t0),"ms rss",Math.round(process.memoryUsage().rss/1048576),"MB cpu ms",Math.round((process.cpuUsage().user+process.cpuUsage().system)/1000), "peak", Math.round(process.resourceUsage().maxRSS/1024), "MB");
