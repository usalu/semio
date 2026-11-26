import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {METRICS_CATALOGUE_V1,METRICS_TIERS_V1,closeMetricsRingsV1,openMetricsRingsV1,recordSampleV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/📈️metrics/🟦️.ts";

test("measure write cost per tick for 60 metrics and the file sizes per tier",()=>{
 const ids=METRICS_CATALOGUE_V1.slice(0,60).map(metric=>metric.id),directory=mkdtempSync(join(tmpdir(),"metrics-measure-"));
 const BASE=1_800_000_000_000,handle=openMetricsRingsV1({directory,ids,createdMs:BASE}),times:number[]=[];
 const values=Object.fromEntries(ids.map((id,at)=>[id,at*1.25]));
 for(let tick=0;tick<7300;tick++){
  for(const id of ids)values[id]=(tick*7+ids.indexOf(id))%1000/4;
  const started=performance.now();recordSampleV1(handle,BASE+tick*1000,values);times.push((performance.now()-started)*1000);
 }
 closeMetricsRingsV1(handle);
 const sorted=[...times].sort((a,b)=>a-b),mean=times.reduce((a,b)=>a+b,0)/times.length,at=(q:number)=>sorted[Math.floor(q*(sorted.length-1))]!;
 const plain=times.filter((_,tick)=>tick%10!==0).reduce((a,b)=>a+b,0)/times.filter((_,tick)=>tick%10!==0).length;
 const cascade=times.filter((_,tick)=>tick%60===0&&tick>0);
 const result={metrics:ids.length,ticks:times.length,meanMicroseconds:mean,medianMicroseconds:at(0.5),p99Microseconds:at(0.99),maxMicroseconds:sorted.at(-1),nonBoundaryMeanMicroseconds:plain,minuteBoundaryMeanMicroseconds:cascade.reduce((a,b)=>a+b,0)/cascade.length,cpuShareAt1HzPercent:mean/1e6*100,sizes:Object.fromEntries(METRICS_TIERS_V1.map(tier=>[tier.id,statSync(join(directory,`${tier.id}.ring`)).size])),bytesPerTickAtT0:2*8+ids.length*36+8};
 writeFileSync(process.env["M2_MEASURE_OUT"]!,JSON.stringify(result,null,1));
 rmSync(directory,{recursive:true,force:true});
 expect(mean).toBeGreaterThan(0);
});
