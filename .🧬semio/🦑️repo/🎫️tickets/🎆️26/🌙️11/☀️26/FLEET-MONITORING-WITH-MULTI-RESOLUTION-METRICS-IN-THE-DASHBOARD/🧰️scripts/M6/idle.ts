import{cpuUsage}from"node:process";
const s=Number(process.argv[2]??"20"),c0=cpuUsage(),t0=performance.now();
await new Promise<void>(done=>{let n=0;const t=setInterval(()=>{if(++n>=s){clearInterval(t);done();}},1000);});
const c=cpuUsage(c0),w=(performance.now()-t0)/1000;console.log("idle bun with 1 s timer: cpu%",((c.user+c.system)/1e6/w*100).toFixed(3));
