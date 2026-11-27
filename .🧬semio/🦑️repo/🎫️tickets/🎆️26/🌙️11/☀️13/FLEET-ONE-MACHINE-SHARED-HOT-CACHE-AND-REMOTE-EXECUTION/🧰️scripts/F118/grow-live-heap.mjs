import{heapStats}from"bun:jsc";
const cap=4*1024**3,keep=[],step=Number(process.argv[2]??128),limit=Number(process.argv[3]??3600);
const mb=b=>Math.round(b/1048576);
let live=0;
while(live<limit){
 const began=performance.now();
 const rows=[];
 for(let i=0;i<step*1048576/200;i++)rows.push({name:"owner-"+keep.length+"-"+i+"-"+Math.random().toString(36).repeat(3),edges:[i,i+1,"x"+i],n:i});
 keep.push(rows);
 const took=performance.now()-began,s=heapStats(),rss=process.memoryUsage().rss;
 live=mb(s.heapSize);
 console.log(JSON.stringify({live,rss:mb(rss),stepMs:Math.round(took)}));
 if(rss>cap){console.log("CAP");break;}
}
console.log("DONE",keep.length);
