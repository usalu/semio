import{readFileSync,readdirSync,statSync}from"node:fs";
import{join,resolve}from"node:path";
import{heapStats}from"bun:jsc";
const B=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),G=join(B,"🕸️graph");
const gen=await import(join(G,"🕰️generations/🟨️.mjs"));
const snap=async(l:string)=>{for(let i=0;i<3;i++){await new Promise(r=>setTimeout(r,30));Bun.gc(true);}const h=heapStats(),m=process.memoryUsage();console.log("[VIEW]",l,JSON.stringify({rssMB:Math.round(m.rss/1048576),heapMB:Math.round(h.heapSize/1048576),extraMB:Math.round(h.extraMemorySize/1048576),objects:h.objectCount}));};
const dir=resolve(".cache/b/g/m/generations/x"),file=readdirSync(dir).filter(n=>n.endsWith(".facts.br")).map(n=>({n,s:statSync(join(dir,n))})).sort((a,b)=>b.s.size-a.s.size)[0];
await snap("start");
const run=async()=>{
 const index:any=gen.openPackedFactIndexV1(readFileSync(join(dir,file.n)),512*1024*1024,"fact-index:probe");
 await snap("view opened (document only)");
 let touched=0;const n=index.rows.length;
 for(let at=0;at<n;at+=4096){const end=Math.min(n,at+4096);for(let id=at;id<end;id++){const row=index.rows[id];touched+=row.path.length>0?1:0;}await new Promise(r=>setImmediate(r));}
 await snap("every row read once through the view ("+touched+" rows)");
 for(let at=0;at<index.classes.length;at+=512){const end=Math.min(index.classes.length,at+512);for(let id=at;id<end;id++)index.classes[id].r.length;await new Promise(r=>setImmediate(r));}
 await snap("every class read once");
 console.log("[VIEW] stats",JSON.stringify(index.stats().caches.map((c:any)=>[c.name,c.entries,Math.round(c.bytes/1048576)])));
 index.drop();
};
await run();await snap("view dropped");
process.exit(0);
