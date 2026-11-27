import{heapStats}from"bun:jsc";
const keep=[],mb=b=>Math.round(b/1048576);
for(let s=0;s<16;s++){const rows=[];for(let i=0;i<128*1048576/200;i++)rows.push({name:"o-"+s+"-"+i+"-"+Math.random().toString(36).repeat(3),e:[i,i+1],n:i});keep.push(rows);}
for(let k=0;k<6;k++){const t=performance.now();Bun.gc(true);Bun.shrink();Bun.gc(true);console.log(JSON.stringify({round:k,live:mb(heapStats().heapSize),rss:mb(process.memoryUsage().rss),releaseMs:Math.round(performance.now()-t)}));}
console.log("DONE");
