const mod=process.argv[3];const{startSliceReportV1}=await import(mod);
const MB=1048576,keep=[];while(process.memoryUsage().heapUsed/MB<560){for(let at=0;at<200;at++)keep.push({id:at,name:"p"+Math.random(),rows:["a"+at,"b"+at,{x:at}]});}
const report=startSliceReportV1({root:process.argv[2],snapshot:()=>globalThis.Bun.generateHeapSnapshot(),intervalMilliseconds:250});
let guard=0,rss=0;const probe=setInterval(()=>{const usage=process.memoryUsage();guard=Math.max(guard,(usage.heapUsed+usage.external)/MB);rss=Math.max(rss,usage.rss/MB);},100);
await new Promise(done=>setTimeout(done,24000));clearInterval(probe);report.stop();
const usage=process.memoryUsage();console.log(JSON.stringify({guard:Math.max(guard,(usage.heapUsed+usage.external)/MB),rss:Math.max(rss,usage.rss/MB),kept:keep.length}));
