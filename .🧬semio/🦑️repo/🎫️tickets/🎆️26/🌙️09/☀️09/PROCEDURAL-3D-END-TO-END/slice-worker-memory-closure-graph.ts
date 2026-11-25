import {heapStats} from "bun:jsc";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const m=await import(G+"/🕸️graph/📥️inference/🧭️script-inputs/🌳️closure-graph/🟨️.mjs");
const line=(n:string)=>{Bun.gc(true);const c=heapStats().objectTypeCounts as any;console.log(n.padEnd(8),"heap",Math.round(process.memoryUsage().heapUsed/1048576),"Array",c.Array,"string",c.string,"Object",c.Object);};
line("start");
const g:any=m.loadClosureGraphSyncV1("/Users/ueli/Documents/semio/.cache/f/closure-graph");
line("loaded");console.log(g.stats());
