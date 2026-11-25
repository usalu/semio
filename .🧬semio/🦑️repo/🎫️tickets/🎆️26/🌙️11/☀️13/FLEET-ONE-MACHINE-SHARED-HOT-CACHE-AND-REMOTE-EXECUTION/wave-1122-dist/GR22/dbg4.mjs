/** 📏️ [DEBUG] probe: the preflight of every tool closure on the real tree in one process: wall, CPU and peak resident size. */
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",root="/Users/ueli/Documents/semio";
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const si=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const t=performance.now(),scripts=inference.createScriptInputCache({workspaceRoot:root});
const answer=await si.preflightToolClosuresV1(root,scripts);
console.log("[DEBUG] preflight",answer.checked,answer.refused.length,Math.round(performance.now()-t),"ms peak",Math.round(process.resourceUsage().maxRSS/1024),"MB graph",scripts.closureGraph?.stats().nodes);
let size=0,wit=0;for(const r of scripts.closureGraph.records()){const t=JSON.stringify(r.extra);size+=t.length;wit+=JSON.stringify(r.extra?.witness??null).length;}
Bun.gc(true);console.log("[DEBUG] extra chars",size,"witness chars",wit,"heap MB",Math.round(process.memoryUsage().heapUsed/1048576),"rss",Math.round(process.memoryUsage().rss/1048576),"walkNodes",scripts.walkNodes?.size);
process.exit(0);
