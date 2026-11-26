import{cpuUsage}from"node:process";import{rmSync}from"node:fs";
const F="/Users/ueli/Documents/semio/🧰️framework",B=F+"/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",H=F+"/🔨️modules/🏃️process/🏗️build";
const m=await import(H+"/📈️metrics/🟦️.ts"),col=await import(H+"/📈️metrics/🛰️collector/🟦️.ts"),prod=await import(B+"/🌐️remote-execution/🛰️sidecar/📈️metrics/🟦️.ts");
const dir=process.argv[2]!;rmSync(dir,{recursive:true,force:true});
const h=m.openMetricsRingsV1({directory:dir});
const values:Record<string,number>={};for(const x of m.METRICS_CATALOGUE_V1)if(x.scope!=="fleet")values[x.id]=1.5;
let at=1_800_000_000_000;const N=2000;
for(let i=0;i<50;i++){m.recordSampleV1(h,at,values);at+=1000;}
const c0=cpuUsage(),t0=performance.now();
for(let i=0;i<N;i++){m.recordSampleV1(h,at,values);at+=1000;}
const c=cpuUsage(c0);console.log("recordSample per tick: wall",((performance.now()-t0)/N*1000).toFixed(1),"us cpu",((c.user+c.system)/N).toFixed(1),"us (user",(c.user/N).toFixed(1),"sys",(c.system/N).toFixed(1),")");
const t1=performance.now();for(let i=0;i<N;i++){m.recordSampleV1(h,at,{"host.cpu.busy":1});at+=1000;}console.log("one metric per tick wall",((performance.now()-t1)/N*1000).toFixed(1),"us");
m.closeMetricsRingsV1(h);
