// [gram-g] revision reload cost: every library edit makes the next inference call load a complete new component graph (and its parsers) that the loader keeps for ever. Simulates edits with GRAM_REV (patched into implementationRevision). usage: bun reload.mjs [count=5]
import{heapStats}from"bun:jsc";
const ROOT="/Users/ueli/Documents/semio",L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",count=Number(process.argv[2]??"5"),mb=n=>Math.round(n/1048576);
Bun.plugin({name:"gram-rev",setup(b){b.onLoad({filter:/inference\/[^/]*revision\/[^/]*\.mjs$/},async args=>{if(!args.path.endsWith("♻️revision/🟨️.mjs"))return undefined;let t=await Bun.file(args.path).text();const a="hash.update(JSON.stringify(buildModesV1()));";if(!t.includes(a))throw Error("anchor");return {contents:t.replace(a,a+'hash.update(process.env.GRAM_REV??"");'),loader:"js"};});}});
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const live=()=>{Bun.gc(true);Bun.gc(true);const h=heapStats();return h.heapSize+h.extraMemorySize;},row=tag=>console.log(JSON.stringify({tag,live:mb(live()),rss:mb(process.memoryUsage().rss)}));
row("base");
for(let k=1;k<=count;k++){process.env.GRAM_REV=String(k);const t=performance.now(),scope=await inference.loadCurrentImplementationV1();await scope.libraryBootstrap;console.log("reload",k,Math.round(performance.now()-t),"ms");row("after reload "+k);}
