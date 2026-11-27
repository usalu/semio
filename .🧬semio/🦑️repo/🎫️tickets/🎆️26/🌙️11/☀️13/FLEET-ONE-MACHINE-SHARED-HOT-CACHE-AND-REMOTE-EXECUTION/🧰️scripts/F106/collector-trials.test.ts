import{test}from"bun:test";
import*as jsc from"bun:jsc";
const MB=1048576,mb=(n:number)=>Math.round(n/MB),managed=()=>{const u=process.memoryUsage();return u.heapUsed+u.external;};
const tick=()=>new Promise<void>(r=>setTimeout(r,0)),immediate=()=>new Promise<void>(r=>setImmediate(r));
const mk=()=>Array.from({length:3_000_000},(_,at)=>({at,text:"graph-record-"+at}));
const collectors:Record<string,()=>void|Promise<void>>={
 "Bun.gc(true)":()=>{Bun.gc(true);},
 "Bun.gc(true),shrink,Bun.gc(true)":()=>{Bun.gc(true);Bun.shrink();Bun.gc(true);},
 "fullGC":()=>{jsc.fullGC();},
 "gcAndSweep":()=>{jsc.gcAndSweep();},
 "fullGC,shrink,fullGC":()=>{jsc.fullGC();Bun.shrink();jsc.fullGC();},
 "Bun.gc(true)+tick+Bun.gc(true)":async()=>{Bun.gc(true);await tick();Bun.gc(true);},
 "tick+Bun.gc(true)":async()=>{await tick();Bun.gc(true);},
};
const TRIALS=12;
test("collector trials",async()=>{
 for(const[name,collect]of Object.entries(collectors)){
  const immediateRows:number[]=[],tickRows:number[]=[],ms:number[]=[];
  for(let i=0;i<TRIALS;i++){
   let g:object[]|undefined=mk();let n=0;
   const settle=async()=>{n=g!.length;g=undefined;const s=performance.now();await collect();ms.push(performance.now()-s);immediateRows.push(mb(managed()));};
   await settle();await tick();tickRows.push(mb(managed()));void n;
  }
  const bad=(rows:number[])=>rows.filter(v=>v>50).length;
  console.log(name.padEnd(36),"residual>50MB right after:",bad(immediateRows)+"/"+TRIALS,"after tick:",bad(tickRows)+"/"+TRIALS,"| median right-after MB",immediateRows.sort((a,b)=>a-b)[TRIALS>>1],"| median ms",Math.round(ms.sort((a,b)=>a-b)[TRIALS>>1]));
 }
},120000);
