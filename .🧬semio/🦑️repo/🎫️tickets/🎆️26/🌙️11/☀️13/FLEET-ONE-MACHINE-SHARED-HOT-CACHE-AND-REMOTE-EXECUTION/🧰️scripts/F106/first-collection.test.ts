import{test}from"bun:test";
import*as jsc from"bun:jsc";
const MB=1048576,mb=(n:number)=>Math.round(n/MB),managed=()=>{const u=process.memoryUsage();return u.heapUsed+u.external;};
const tick=()=>new Promise<void>(r=>setTimeout(r,0));
const collectors:Record<string,()=>void|Promise<void>>={
 "gc":()=>{Bun.gc(true);},
 "gc,shrink,gc":()=>{Bun.gc(true);Bun.shrink();Bun.gc(true);},
 "fullGC":()=>{jsc.fullGC();},
 "gcAndSweep":()=>{jsc.gcAndSweep();},
 "fullGC,shrink,fullGC":()=>{jsc.fullGC();Bun.shrink();jsc.fullGC();},
 "gc,tick,gc":async()=>{Bun.gc(true);await tick();Bun.gc(true);},
 "tick,gc":async()=>{await tick();Bun.gc(true);},
 "tick,fullGC":async()=>{await tick();jsc.fullGC();},
};
test("first collection of a fresh process",async()=>{
 const name=process.env.F106_COLLECTOR!;
 let g:object[]|undefined=Array.from({length:3_000_000},(_,at)=>({at,text:"graph-record-"+at}));
 const before=mb(managed());let n=0;
 const settle=async(release:()=>void)=>{release();await collectors[name]!();return mb(managed());};
 const right=await settle(()=>{n=g!.length;g=undefined;});await tick();
 console.log("RESULT",name,"before",before,"right after",right,"after tick",mb(managed()));void n;
});
