import*as jsc from"bun:jsc";import{writeFileSync,unlinkSync}from"node:fs";
const MB=1048576,mb=n=>Math.round(n/MB),tick=()=>new Promise(r=>setTimeout(r,0));
const row=l=>{const u=process.memoryUsage();console.log(l.padEnd(26),"heapUsed",mb(u.heapUsed),"external",mb(u.external),"arrayBuffers",mb(u.arrayBuffers),"| jsc.heapSize()",mb(jsc.heapSize()),"| rss",mb(u.rss));};
const p="/tmp/f106-b.bin";writeFileSync(p,Buffer.alloc(200*MB,97));
row("start");
let a=await Bun.file(p).arrayBuffer();row("200MB arrayBuffer");await tick();row("+tick");
let b=new Uint8Array(a.byteLength);b.set(new Uint8Array(a));row("+ 200MB copy Uint8Array");
let s=new TextDecoder().decode(new Uint8Array(a));row("+ 200MB decoded string");await tick();row("+tick");
a=b=s=undefined;await tick();jsc.fullGC();row("dropped + fullGC");unlinkSync(p);
