import {openProvenanceV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧾️provenance/🟦️.ts";
let t=performance.now();const p=await openProvenanceV1(process.argv[2]!);console.log("open",(performance.now()-t).toFixed(0),"ms; rss MB",(process.memoryUsage().rss/1048576).toFixed(0));
for(let i=0;i<3;i++){t=performance.now();Bun.gc(true);console.log("full gc",(performance.now()-t).toFixed(0),"ms");}
const gaps:number[]=[];let last=performance.now();const timer=setInterval(()=>{const at=performance.now();gaps.push(at-last-5);last=at;},5);
const origins=Array.from({length:13},(_,i)=>"g1-m"+i);
for(let round=0;round<6;round++){t=performance.now();for(let i=0;i<20000;i++){p.record("main","ac",(i*7919+round).toString(16).padStart(64,"0"),origins[i%13]!,(i*31).toString(16).padStart(64,"0"),true);}
 await new Promise(r=>setTimeout(r,500));await p.flush();}
gaps.sort((a,b)=>a-b);console.log("record lag p99",gaps[Math.floor(gaps.length*.99)]!.toFixed(0),"max",gaps.at(-1)!.toFixed(0));
process.exit(0);
