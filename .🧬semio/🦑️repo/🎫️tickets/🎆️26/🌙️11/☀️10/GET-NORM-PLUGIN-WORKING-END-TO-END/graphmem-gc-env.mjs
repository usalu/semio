import{spawnSync}from"node:child_process";
if(process.argv[2]==="child"){
 const{heapStats}=await import("bun:jsc");let live=[],peakHeap=0;const ring=[];
 const t0=performance.now();let i=0;
 while(performance.now()-t0<20000){
  const batch=[];for(let k=0;k<20000;k++)batch.push({a:"x".repeat(50)+k,b:[k,k+1,k+2],c:{d:String(k)}});
  ring.push(batch);if(ring.length>40)ring.shift();live=ring;
  i++;
  if(i%10===0){const h=heapStats();peakHeap=Math.max(peakHeap,h.heapSize+h.extraMemorySize);}
  if(i%5===0)await new Promise(r=>setImmediate(r));
 }
 const h=heapStats();console.log(JSON.stringify({peakRssMB:Math.round(process.resourceUsage().maxRSS/1024),peakHeapMB:Math.round(peakHeap/1048576),iterations:i,liveKept:live.length}));
}else{
 const variants=[["default",{}],["forceRAM1G",{BUN_JSC_forceRAMSize:String(1024*1048576)}],["gcMax300M",{BUN_JSC_gcMaxHeapSize:String(300*1048576)}],["gcMax300M+RAM1G",{BUN_JSC_gcMaxHeapSize:String(300*1048576),BUN_JSC_forceRAMSize:String(1024*1048576)}]];
 for(const[name,env]of variants){const r=spawnSync(process.execPath,[import.meta.filename,"child"],{env:{...process.env,...env},encoding:"utf8"});console.log(name,r.stdout.trim()||r.stderr.slice(0,200));}
}
