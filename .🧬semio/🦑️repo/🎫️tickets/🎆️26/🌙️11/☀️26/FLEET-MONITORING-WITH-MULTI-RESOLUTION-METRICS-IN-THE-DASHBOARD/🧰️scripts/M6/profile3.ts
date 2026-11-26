import{cpuUsage}from"node:process";import{rmSync,mkdirSync}from"node:fs";import{join}from"node:path";
const F="/Users/ueli/Documents/semio/🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",H=F+"/🔨️modules/🏃️process/🏗️build";
const col=await import(H+"/📈️metrics/🛰️collector/🟦️.ts"),prod=await import(B+"/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const scratch=process.argv[2]!;rmSync(scratch,{recursive:true,force:true});mkdirSync(join(scratch,"store"),{recursive:true});
const environment={...process.env,SEMIO_FLEET_STORE_ROOT:join(scratch,"store")};
const sources=prod.fleetMetricsSourcesV1({root:"/Users/ueli/Documents/semio",environment,origin:"g1-m01"});
const collector=col.createFleetMetricsCollectorV1({directory:join(scratch,"rings"),sources,store:{root:join(scratch,"store"),origin:"g1-m01"}});
let at=Date.now();
for(let i=0;i<30;i++){collector.step(at);at+=1000;}
const N=600,c0=cpuUsage(),t0=performance.now();
for(let i=0;i<N;i++){collector.step(at);at+=1000;}
const c=cpuUsage(c0);console.log("collector step (consecutive, includes slow steps every 10th and publish every 5th): wall",((performance.now()-t0)/N*1000).toFixed(1),"us cpu",((c.user+c.system)/N).toFixed(1),"us ->",((c.user+c.system)/N/10000).toFixed(3),"% of a core at 1 Hz");
collector.close();
