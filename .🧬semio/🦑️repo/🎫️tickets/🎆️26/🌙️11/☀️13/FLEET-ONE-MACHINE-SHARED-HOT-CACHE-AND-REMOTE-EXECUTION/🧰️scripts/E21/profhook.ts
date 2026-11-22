import {startSamplingProfiler,samplingProfilerStackTraces} from "bun:jsc";
import {appendFileSync} from "node:fs";
export function profileStalls(file:string,thresholdMs=300){
 startSamplingProfiler();let seen=Date.now();
 const timer=setInterval(()=>{
  const at=Date.now(),gap=at-seen-100;seen=at;
  const traces=(samplingProfilerStackTraces() as {traces:{frames:{name:string;sourceURL?:string;line:number}[]}[]}).traces,window=new Map<string,number>();
  if(gap<=thresholdMs)return;
  for(const trace of traces){const key=trace.frames.slice(0,7).map(frame=>`${frame.name}@${String(frame.sourceURL??"").split("/").slice(-2).join("/")}:${frame.line}`).join(" < ");window.set(key,(window.get(key)??0)+1);}
  appendFileSync(file,`${new Date().toISOString()} gap ${gap} ms samples ${traces.length}\n${[...window].sort((a,b)=>b[1]-a[1]).slice(0,10).map(([key,count])=>`  ${count} ${key}`).join("\n")}\n`);
 },100);return timer;
}
