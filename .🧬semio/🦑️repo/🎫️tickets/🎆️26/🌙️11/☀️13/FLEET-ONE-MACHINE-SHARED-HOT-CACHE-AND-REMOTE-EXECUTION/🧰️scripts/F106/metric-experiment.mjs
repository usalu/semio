// F106: what does process.memoryUsage().heapUsed+external measure in Bun/JSC? One scenario per process. Usage: bun metric-experiment.mjs <scenario>
import{heapStats,fullGC}from"bun:jsc";
import{writeFileSync,unlinkSync}from"node:fs";
const MB=1048576,mb=n=>String(Math.round(n/MB)).padStart(5),tick=()=>new Promise(r=>setTimeout(r,0));
const row=label=>{const u=process.memoryUsage(),h=heapStats();console.log(label.padEnd(34),"mu.heapUsed",mb(u.heapUsed),"ext",mb(u.external),"ab",mb(u.arrayBuffers),"| managed",mb(u.heapUsed+u.external),"| jsc.heapSize",mb(h.heapSize),"extra",mb(h.extraMemorySize),"live",mb(h.heapSize+h.extraMemorySize),"| rss",mb(process.memoryUsage.rss()));};
const scenario=process.argv[2];
const records=n=>Array.from({length:n},(_,at)=>({at,text:"graph-record-"+at}));
console.log("== "+scenario+" bun "+Bun.version);
row("start");
let keep;
const drop=async()=>{keep=undefined;row("dropped, no gc");Bun.gc(true);row("Bun.gc(true) immediately");await tick();row("after one macrotask");await tick();row("after two macrotasks");fullGC();row("fullGC immediately");await tick();row("fullGC + macrotask");Bun.gc(true);Bun.shrink();Bun.gc(true);await tick();row("settle shape + macrotask");};
if(scenario==="objects"){keep=records(3_000_000);row("allocated 3M records");await tick();row("allocated + macrotask");await drop();}
if(scenario==="typed"){keep=new Uint8Array(300*MB);keep.fill(1);row("300MB Uint8Array");await tick();row("+ macrotask");await drop();}
if(scenario==="file-text"){const p="/tmp/f106-big.bin";writeFileSync(p,Buffer.alloc(200*MB,97));keep=await Bun.file(p).text();row("Bun.file.text 200MB");await tick();row("+ macrotask");await drop();unlinkSync(p);}
if(scenario==="file-buffer"){const p="/tmp/f106-big.bin";writeFileSync(p,Buffer.alloc(200*MB,97));keep=await Bun.file(p).arrayBuffer();row("Bun.file.arrayBuffer 200MB");await tick();row("+ macrotask");await drop();unlinkSync(p);}
if(scenario==="decoder"){const p="/tmp/f106-big.bin";writeFileSync(p,Buffer.alloc(200*MB,97));const b=await Bun.file(p).bytes();keep=new TextDecoder().decode(b);row("bytes+TextDecoder 200MB");await tick();row("+ macrotask");await drop();unlinkSync(p);}
if(scenario==="sliced"){const big="x".repeat(2_000_000);keep=[];for(let i=0;i<1500;i++)keep.push((big+i).slice(100,100+40));row("1500 x 40-char slices of 2MB");await tick();row("+ macrotask");await drop();}
if(scenario==="strings"){keep=Array.from({length:4_000_000},(_,i)=>"row-"+i+"-"+(i*7));row("4M short strings");await tick();row("+ macrotask");await drop();}
