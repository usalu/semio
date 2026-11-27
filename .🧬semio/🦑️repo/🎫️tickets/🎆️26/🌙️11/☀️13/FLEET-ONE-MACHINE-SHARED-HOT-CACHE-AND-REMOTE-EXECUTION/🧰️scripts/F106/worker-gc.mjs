import*as jsc from"bun:jsc";import{Worker}from"node:worker_threads";
const MB=1048576,mb=n=>Math.round(n/MB),tick=()=>new Promise(r=>setTimeout(r,0));
const row=l=>{const u=process.memoryUsage(),h=jsc.heapStats();console.log(l.padEnd(30),"mu.heapUsed",mb(u.heapUsed),"ext",mb(u.external),"| jsc",mb(h.heapSize),"extra",mb(h.extraMemorySize));};
const mode=process.argv[2];
if(mode!=="noworker"){const w=new Worker("setInterval(()=>{},50)",{eval:true});w.unref();await tick();await tick();}
const alloc=()=>Array.from({length:3_000_000},(_,at)=>({at,text:"graph-record-"+at}));
let g=alloc();row(mode+": alloc");g=undefined;
for(const step of(process.argv[3]??"gc").split(",")){
 if(step==="gc")Bun.gc(true);if(step==="shrink")Bun.shrink();if(step==="full")jsc.fullGC();if(step==="tick")await tick();if(step==="sleep")await new Promise(r=>setTimeout(r,300));if(step==="gcf")Bun.gc(false);
 row("  after "+step);
}
