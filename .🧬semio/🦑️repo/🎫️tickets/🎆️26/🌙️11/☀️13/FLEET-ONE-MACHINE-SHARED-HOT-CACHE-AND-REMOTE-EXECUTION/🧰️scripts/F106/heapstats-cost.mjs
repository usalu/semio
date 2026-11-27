import*as jsc from"bun:jsc";
console.log(Object.keys(jsc).join(" "));
const t=(n,f)=>{const s=performance.now();for(let i=0;i<200;i++)f();console.log(n,((performance.now()-s)/200).toFixed(3),"ms/call");};
const keep=Array.from({length:2_000_000},(_,i)=>({i,s:"x"+i}));
t("heapStats()",()=>jsc.heapStats());t("memoryUsage() full",()=>process.memoryUsage());t("jsc.memoryUsage()",()=>jsc.memoryUsage());
console.log(JSON.stringify(jsc.memoryUsage()),Object.keys(jsc.heapStats()).join(","));
console.log(typeof jsc.heapSize, jsc.heapSize?.());
const u=process.memoryUsage(),h=jsc.heapStats();console.log("mu",u.heapUsed,u.external,u.heapTotal,"hs",h.heapSize,h.extraMemorySize,h.heapCapacity, keep.length);
