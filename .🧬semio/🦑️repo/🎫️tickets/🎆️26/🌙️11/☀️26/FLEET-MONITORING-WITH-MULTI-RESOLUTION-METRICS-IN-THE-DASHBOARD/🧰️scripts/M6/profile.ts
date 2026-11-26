const F="/Users/ueli/Documents/semio/🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",H=F+"/🔨️modules/🏃️process/🏗️build";
const col=await import(H+"/📈️metrics/🛰️collector/🟦️.ts"),prod=await import(B+"/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const environment={...process.env,SEMIO_FLEET_STORE_ROOT:"/Users/Shared/semio-store"};
const sources=prod.fleetMetricsSourcesV1({root:"/Users/ueli/Documents/semio",environment,origin:"g1-m01"});
const state=col.createMetricsCollectorStateV1();
const time=(label:string,fn:()=>void,n=200)=>{for(let i=0;i<20;i++)fn();const t=performance.now();for(let i=0;i<n;i++)fn();console.log(label.padEnd(28),((performance.now()-t)/n*1000).toFixed(1),"us");};
for(const c of col.METRICS_COLLECTORS_V1)if(c.scope==="origin")time(c.name+" ("+c.cadence+")",()=>col.runMetricsCollectorV1(c.name,sources,state),c.cadence==="slow"?50:300);
import{cpus,loadavg,freemem}from"node:os";
time("os.cpus()",()=>{cpus();});time("loadavg",()=>{loadavg();});time("process.memoryUsage.rss",()=>{process.memoryUsage.rss();});time("process.cpuUsage",()=>{process.cpuUsage();});
