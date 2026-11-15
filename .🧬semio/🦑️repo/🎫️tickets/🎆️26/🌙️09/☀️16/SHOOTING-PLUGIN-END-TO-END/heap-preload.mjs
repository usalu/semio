import {heapStats} from "bun:jsc";
import {appendFileSync} from "node:fs";
const out=process.env.SEMIO_DEBUG_HEAP_FILE;
if(out&&!process.env.SEMIO_DEBUG_HEAP_SKIP){
 const MiB=1048576;let last=0,lastAt=0;
 const tick=()=>{const u=process.memoryUsage(),managed=(u.heapUsed+u.external)/MiB,now=Date.now();
  if(Math.abs(managed-last)>150||now-lastAt>20000){const s=heapStats();last=managed;lastAt=now;
   appendFileSync(out,JSON.stringify({t:Math.round(now/1000),pid:process.pid,argv:process.argv.slice(1,3).map(a=>a.slice(-60)),rss:Math.round(u.rss/MiB),heap:Math.round(u.heapUsed/MiB),ext:Math.round(u.external/MiB),ab:Math.round(u.arrayBuffers/MiB),size:Math.round(s.heapSize/MiB),extra:Math.round(s.extraMemorySize/MiB),objs:s.objectCount,types:Object.entries(s.objectTypeCounts).sort((a,b)=>b[1]-a[1]).slice(0,10)})+"\n");}};
 setInterval(tick,1000).unref();
}
