import {execFile,spawn} from "node:child_process";
import {readFile,writeFile} from "node:fs/promises";
import {arch,cpus,loadavg,platform,totalmem} from "node:os";
import {join} from "node:path";
import {StringDecoder} from "node:string_decoder";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {incrementalChildEnvironmentV1,incrementalCliArgumentsV1,incrementalCliPortsV1,incrementalInsideV1,type IncrementalHostV1} from "../../@@RUNNER@@/@@TS@@";
import type {IncrementalCommandPortsV1} from "../../@@COMMAND@@/@@TS@@";
import type {ConcurrentReportV1} from "../../@@CONC@@/@@TS@@";
import {runConcurrencyCommandV1} from "../../@@CONC@@/@@COMMAND@@/@@TS@@";
import {benchCompareV1,benchHostV1,benchJudgeV1,benchMarkersV1,benchMutateV1,benchReportV1,benchSeriesV1,type BenchHostV1,type BenchLaneV1,type BenchPairV1,type BenchProbeV1,type BenchReasonV1,type BenchReportV1,type BenchRowV1,type BenchSampleV1,type BenchSeriesV1,type BenchSloSetV1,type BenchSloToolV1,type BenchStartKindV1,type BenchToolV1} from "../@@TS@@";
import {terminateOwnedChildTree} from "@@EIGHT@@/@@MODULES@@/@@PROCESS@@/@@TERMINATION@@/@@TS@@";

const policy=schema.definitions.Policy.const;
const both=(english:string,german:string)=>`${english}\n${german}`;

/** 🧪 What a bench spawn runs: the implementation (or `neutral` for a command without `--tool`), the arguments of `repo` and whether the process tree's memory is sampled. */
export type BenchSpawnSpecV1=Readonly<{tool:BenchSloToolV1;args:readonly string[];memory:boolean}>;
/** 🪀 One finished command: its exit code, wall time, when it started on the clock of the ports, the time to the first output and to the first tool marker line, and the peak resident size of its process tree. */
export type BenchRunV1=Readonly<{exit:number|null;wallMs:number;startedMs:number;firstOutputMs:number|null;markerMs:number|null;peakRssBytes:number|null}>;
/** 🎯 The tasks one cached run touched: those that may be cached, how many of them executed, and the uncached ones that executed. */
export type BenchCacheRunV1=Readonly<{cacheable:number;executedCacheable:number;uncachedExecuted:number;total:number}>;
/** 🤹 The numbers of one concurrent-agent run: the span of the concurrent sessions, the serial reference, the longest wait for an unrelated project and the criteria that failed. */
export type BenchConcurrencyRunV1=Readonly<{makespanMs:number;serialMs:number;maxUnrelatedWaitMs:number;failedRules:number}>;
/** 🛎 The resident service of the copy: whether the checkout has one and how to start and stop it. */
export type BenchServicePortV1=Readonly<{available():Promise<boolean>;start():Promise<void>;stop():Promise<void>}>;

/** 🏟 Everything a bench run needs from the outside: the checkout copy, timed commands, observed cache runs, the concurrency harness, the resident service, the host, the clocks and cooperative cancellation. */
export type BenchPortsV1=Readonly<{
 read(path:string):Promise<string>;
 write(path:string,text:string):Promise<void>;
 spawn(spec:BenchSpawnSpecV1,signal:AbortSignal):Promise<BenchRunV1>;
 cache(tool:BenchToolV1,target:string):Promise<BenchCacheRunV1>;
 concurrency(tool:BenchToolV1,workload:string,sessions:number):Promise<BenchConcurrencyRunV1>;
 service:BenchServicePortV1;
 host():BenchHostV1;
 clock():string;
 fresh:boolean;
 now():number;
 nonce():string;
 check():void;
 progress(stage:string):Promise<void>;
 signal:AbortSignal;
}>;

/** 🛰 What a run of the bench is told: how many samples replace the authored run counts, whether the resident service exists, the busiest load per core to start at and the stored baseline to compare with. */
export type BenchOptionsV1=Readonly<{override:number|null;tools:readonly BenchToolV1[];maxLoad:number|null;baseline:BenchReportV1|null}>;

const sample=(index:number,run:BenchRunV1,values:BenchSampleV1["values"],startKind:BenchStartKindV1|null,load:number|null,ok=run.exit===0):BenchSampleV1=>Object.freeze({index,ok,exit:run.exit,values,startKind,loadPerCore:load});
const unavailable=(probe:BenchProbeV1,tool:BenchSloToolV1,reason:BenchReasonV1):BenchSeriesV1=>benchSeriesV1({probe:probe.id,tool,lane:probe.lane??null,status:"unavailable",reason,samples:[]});

/** 🧭 Builds the arguments of `repo run` for a target: the resident lane insists on the service, the one-shot lane switches it off when the checkout has one so that the two lanes measure different paths. */
export function benchRunArgumentsV1(target:string,lane:BenchLaneV1|null,service:boolean,extra:readonly string[]=[]):string[]{
 return ["run",target,...extra,...(lane==="resident"?[policy.residentFlag]:service?[policy.oneshotFlag]:[])];
}

/** 🧮 Reads the sum of resident sizes of a process and all its descendants from a process table: `ps -A -o pid=,ppid=,rss=` rows (kilobytes) on POSIX systems or `ProcessId,ParentProcessId,WorkingSetSize` CSV rows (bytes) on Windows; shared pages count once per process, so the number is an upper bound. */
export function benchTreeRssV1(table:string,root:number,system:string):number|null{
 const rows=new Map<number,{parent:number;bytes:number}>();
 for(const line of table.split(/\r?\n/u)){
  const cells=(system==="win32"?line.replaceAll('"',"").split(","):line.trim().split(/\s+/u)).map(Number);
  if(cells.length<3||cells.slice(0,3).some(cell=>!Number.isFinite(cell)))continue;
  rows.set(cells[0]!,{parent:cells[1]!,bytes:system==="win32"?cells[2]!:cells[2]!*1024});
 }
 if(!rows.has(root))return null;
 const members=new Set([root]);
 for(let grown=true;grown;){grown=false;for(const[pid,row]of rows)if(!members.has(pid)&&members.has(row.parent)){members.add(pid);grown=true;}}
 return [...members].reduce((sum,pid)=>sum+rows.get(pid)!.bytes,0);
}

const processTable=(system:string):Promise<string>=>new Promise(accept=>{
 const [command,args]:[string,string[]]=system==="win32"?["powershell",["-NoProfile","-NonInteractive","-Command",'Get-CimInstance Win32_Process | ForEach-Object { "$($_.ProcessId),$($_.ParentProcessId),$($_.WorkingSetSize)" }']]:["ps",["-A","-o","pid=,ppid=,rss="]];
 execFile(command,args,{windowsHide:true,maxBuffer:64*1024*1024},(error,stdout)=>accept(error?"":stdout));
});

/** 🏃 Runs one command in its own process tree and measures it on a monotonic clock: wall time, the time to the first output byte and to the first line a tool marker matches, and, when asked, the peak resident size of the tree sampled at a fixed interval. The tree is terminated when the signal aborts. */
export function benchSpawnV1(command:string,args:readonly string[],cwd:string,env:Readonly<Record<string,string>>,markers:readonly RegExp[],memory:boolean,signal:AbortSignal,now:()=>number,system:string=platform()):Promise<BenchRunV1>{
 return new Promise((accept,reject)=>{
  if(signal.aborted){reject(signal.reason);return;}
  const startedMs=now(),child=spawn(command,[...args],{cwd,env:env as NodeJS.ProcessEnv,detached:system!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true}),decoders=[new StringDecoder("utf8"),new StringDecoder("utf8")],pending=["",""];
  let firstOutputMs:number|null=null,markerMs:number|null=null,peak=0,bytes=0,sampling=false,sampled=false,done=false;
  const stop=()=>terminateOwnedChildTree(child);
  signal.addEventListener("abort",stop,{once:true});
  const scan=(at:0|1,chunk:Buffer)=>{
   const elapsed=now()-startedMs;
   if(firstOutputMs===null)firstOutputMs=elapsed;
   bytes+=chunk.length;if(bytes>policy.maximumLogBytes||markerMs!==null||!markers.length)return;
   pending[at]+=decoders[at]!.write(chunk);
   for(let cut=pending[at]!.indexOf("\n");cut>=0&&markerMs===null;cut=pending[at]!.indexOf("\n")){const line=pending[at]!.slice(0,cut).replace(/\r$/u,"");pending[at]=pending[at]!.slice(cut+1);if(markers.some(pattern=>pattern.test(line)))markerMs=elapsed;}
  };
  child.stdout!.on("data",(chunk:Buffer)=>scan(0,chunk));
  child.stderr!.on("data",(chunk:Buffer)=>scan(1,chunk));
  const timer=memory&&child.pid!==undefined?setInterval(()=>{
   if(sampling||done)return;
   sampling=true;
   void processTable(system).then(table=>{const bytesNow=benchTreeRssV1(table,child.pid!,system);if(bytesNow!==null){sampled=true;peak=Math.max(peak,bytesNow);}}).finally(()=>{sampling=false;});
  },policy.memoryIntervalMilliseconds):undefined;
  child.on("error",error=>{done=true;if(timer)clearInterval(timer);signal.removeEventListener("abort",stop);reject(error);});
  child.on("close",code=>{
   done=true;if(timer)clearInterval(timer);signal.removeEventListener("abort",stop);
   if(signal.aborted){reject(signal.reason);return;}
   accept(Object.freeze({exit:code,wallMs:now()-startedMs,startedMs,firstOutputMs,markerMs,peakRssBytes:memory&&sampled?peak:null}));
  });
 });
}

const attempt=async<T>(ports:Pick<BenchPortsV1,"signal">,body:()=>Promise<T>):Promise<T|Error>=>{try{return await body();}catch(error){if(ports.signal.aborted)throw ports.signal.reason??error;return error instanceof Error?error:Error(String(error));}};
const failed=(index:number,load:number|null):BenchSampleV1=>Object.freeze({index,ok:false,exit:null,values:{},startKind:null,loadPerCore:load});
const startKindOf=(run:BenchRunV1):BenchStartKindV1=>run.markerMs!==null?"marker":run.firstOutputMs!==null?"first-output":"exit";
const startOffset=(run:BenchRunV1)=>run.markerMs??run.firstOutputMs??run.wallMs;

/** 🏅 Runs one probe with one implementation and returns its series. Resident probes need the service of the checkout, cold probes a copy prepared by this run. A command that fails counts as a failed sample, a cancellation stops the probe at once; edited files are restored byte for byte and the service is stopped whatever happens. */
export async function benchRunProbeV1(set:BenchSloSetV1,probe:BenchProbeV1,tool:BenchSloToolV1,context:Readonly<{override:number|null;service:boolean}>,ports:BenchPortsV1):Promise<BenchSeriesV1>{
 const lane=probe.lane??null,runs=probe.cold?1:context.override??probe.runs,warmup=probe.cold?0:probe.warmup,total=warmup+runs,load=()=>ports.host().loadPerCore;
 if(lane==="resident"&&!context.service)return unavailable(probe,tool,"service-unavailable");
 if(probe.cold&&!ports.fresh)return unavailable(probe,tool,"not-cold");
 const unit=probe.unit===undefined?undefined:set.units.find(row=>row.id===probe.unit),target=probe.target??unit?.target;
 if(probe.kind==="edit"&&!unit)throw Error(both(`Unknown unit: ${probe.unit}`,`Unbekannte Einheit: ${probe.unit}`));
 const samples:BenchSampleV1[]=[],args=target===undefined?[]:benchRunArgumentsV1(target,lane,context.service,probe.kind==="graph"?["--dry-run"]:[]);
 const step=async(at:number,body:()=>Promise<void>)=>{ports.check();await ports.progress(`${probe.id}:${tool}:${at+1}/${total}`);await body();};
 const series=(reason:BenchReasonV1|null=null)=>benchSeriesV1({probe:probe.id,tool,lane,status:"measured",reason,samples});
 const timed=()=>ports.spawn({tool,args,memory:probe.memory===true},ports.signal);
 if(probe.kind==="cli-help"){
  for(let at=0;at<total;at++)await step(at,async()=>{
   const done=await attempt(ports,()=>ports.spawn({tool:"neutral",args:["--help"],memory:false},ports.signal));
   if(at>=warmup)samples.push(done instanceof Error?failed(samples.length,load()):sample(samples.length,done,{wallMs:done.wallMs},null,load()));
  });
  return series();
 }
 if(probe.kind==="cache"){
  const target=probe.target!;
  await step(0,async()=>{const prime=await attempt(ports,()=>ports.cache(tool as BenchToolV1,target));if(prime instanceof Error)samples.push(failed(0,load()));});
  if(samples.length)return series();
  for(let at=0;at<runs;at++)await step(at,async()=>{
   const done=await attempt(ports,()=>ports.cache(tool as BenchToolV1,target));
   samples.push(done instanceof Error||done.cacheable===0?failed(samples.length,load()):Object.freeze({index:samples.length,ok:true,exit:0,values:{hitRatio:(done.cacheable-done.executedCacheable)/done.cacheable},startKind:null,loadPerCore:load()}));
  });
  return series();
 }
 if(probe.kind==="concurrency"){
  for(let at=0;at<runs;at++)await step(at,async()=>{
   const done=await attempt(ports,()=>ports.concurrency(tool as BenchToolV1,probe.workload!,probe.sessions!));
   samples.push(done instanceof Error?failed(samples.length,load()):Object.freeze({index:samples.length,ok:true,exit:null,values:{makespanMs:done.makespanMs,maxUnrelatedWaitMs:done.maxUnrelatedWaitMs,...(done.makespanMs>0&&done.serialMs>0?{speedup:done.serialMs/done.makespanMs}:{})},startKind:null,loadPerCore:load()}));
  });
  return series();
 }
 const resident=lane==="resident";
 let started=false,missing=0;
 try{
  if(resident){await ports.service.start();started=true;}
  if(!probe.cold){
   ports.check();await ports.progress(`${probe.id}:${tool}:prime`);
   const prime=await attempt(ports,()=>timed());
   if(prime instanceof Error||prime.exit!==0){samples.push(failed(0,load()));return series();}
  }
  const original=unit?await ports.read(unit.file):"";
  for(let at=0;at<total;at++)await step(at,async()=>{
   const here=load();
   let outcome:BenchRunV1|Error,offset=0;
   if(unit){
    const edited=benchMutateV1(original,unit,ports.nonce()),t0=ports.now();
    try{
     await ports.write(unit.file,edited);
     outcome=await attempt(ports,()=>timed());
     offset=outcome instanceof Error?0:outcome.startedMs-t0;
    }finally{await ports.write(unit.file,original);}
   }else outcome=await attempt(ports,()=>timed());
   if(at<warmup)return;
   if(outcome instanceof Error){samples.push(failed(samples.length,here));return;}
   if(probe.memory){if(outcome.peakRssBytes===null){missing++;samples.push(failed(samples.length,here));}else samples.push(sample(samples.length,outcome,{peakRssBytes:outcome.peakRssBytes},null,here));return;}
   if(!unit){samples.push(sample(samples.length,outcome,{wallMs:outcome.wallMs},null,here));return;}
   samples.push(sample(samples.length,outcome,outcome.exit===0?{startMs:offset+startOffset(outcome),greenMs:offset+outcome.wallMs}:{},startKindOf(outcome),here));
  });
  return probe.memory===true&&samples.length>0&&missing===samples.length?benchSeriesV1({probe:probe.id,tool,lane,status:"unavailable",reason:"unsupported",samples:[]}):series();
 }finally{if(started)await ports.service.stop();}
}

/** 🏁 Runs the selected probes in order, judges every selected objective against its series, annotates the host before and after, and compares with the stored baseline when one is given. A run refuses to start on a host busier than the given load per core; progress is published per probe and the run stops at the next check when cancelled. */
export async function benchRunV1(set:BenchSloSetV1,pairs:readonly BenchPairV1[],options:BenchOptionsV1,ports:BenchPortsV1):Promise<BenchReportV1>{
 ports.check();
 const startedAt=ports.clock(),host=ports.host();
 if(options.maxLoad!==null&&host.loadPerCore!==null&&host.loadPerCore>options.maxLoad)throw Error(both(`The host load per core ${host.loadPerCore.toFixed(2)} exceeds the limit ${options.maxLoad}`,`Die Last je Kern ${host.loadPerCore.toFixed(2)} überschreitet die Grenze ${options.maxLoad}`));
 const context={override:options.override,service:await ports.service.available()},series:BenchSeriesV1[]=[];
 for(const[at,pair]of pairs.entries()){ports.check();await ports.progress(`${pair.probe.id}:${pair.tool}:${at+1}/${pairs.length}`);series.push(await benchRunProbeV1(set,pair.probe,pair.tool,context,ports));}
 const position=new Map(set.slos.map((slo,at)=>[slo.id,at])),rows:BenchRowV1[]=[];
 for(const[at,pair]of pairs.entries())for(const slo of pair.slos)rows.push(benchJudgeV1(slo,pair.tool,series[at]));
 rows.sort((left,right)=>position.get(left.slo)!-position.get(right.slo)!||(left.tool<right.tool?-1:left.tool>right.tool?1:0));
 const base={startedAt,finishedAt:ports.clock(),host,hostAfter:ports.host(),tools:options.tools,runsOverride:options.override,series,rows,comparison:null};
 const report=benchReportV1(base);
 return options.baseline?benchReportV1({...base,comparison:benchCompareV1(set,report,options.baseline)}):report;
}

/** 🎚 Condenses the JSON report of one concurrent-agent run to the numbers the bench judges: the span of the concurrent sessions, the serial span, the longest wait that was not for a project of the same group and the criteria that failed. */
export function benchConcurrencyFromReportV1(report:ConcurrentReportV1):BenchConcurrencyRunV1{
 const run=report.runs[0];
 if(!run)throw Error(both("The concurrency report holds no run","Der Gleichzeitigkeitsbericht enthält keinen Lauf"));
 const span=(sessions:readonly Readonly<{startMs:number;endMs:number}>[])=>sessions.length?Math.max(...sessions.map(row=>row.endMs))-Math.min(...sessions.map(row=>row.startMs)):0,waits=run.measurement.sessions.flatMap(session=>session.steps.flatMap(step=>step.waits));
 return Object.freeze({makespanMs:span(run.measurement.sessions),serialMs:(run.measurement.serial??[]).reduce((sum,row)=>sum+(row.endMs-row.startMs),0),maxUnrelatedWaitMs:Math.max(0,...waits.filter(wait=>wait.relation!=="related").map(wait=>wait.waitMs)),failedRules:run.rules.filter(rule=>rule.status==="fail").length});
}

/** 🏛 What the real ports need beyond the incremental host: the process spawner, cache runs, concurrency runs, the service and the host sample, all replaceable in tests, and whether this run prepared the copy. */
export type BenchHostPortsV1=IncrementalHostV1&Readonly<{
 fresh:boolean;
 catalog:string;
 commandPorts?:IncrementalCommandPortsV1;
 spawn?:BenchPortsV1["spawn"];
 cache?:BenchPortsV1["cache"];
 concurrency?:BenchPortsV1["concurrency"];
 service?:BenchServicePortV1;
 sample?:()=>BenchHostV1;
 clock?:()=>string;
}>;

/** 🏢 The real ports: reads and writes inside the checkout copy, commands through the copy's own CLI on a monotonic clock, cache runs observed through the incremental harness, concurrency runs through `repo concurrency`, the service looked up in the copy's operations catalogue. */
export function benchCliPortsV1(host:BenchHostPortsV1,signal:AbortSignal):BenchPortsV1{
 let sequence=0;
 const cli=(tool:BenchSloToolV1,args:readonly string[])=>{
  const call=tool==="neutral"?{command:host.executable,args:[join(host.root,host.script),"repo",...args]}:incrementalCliArgumentsV1(host.executable,host.root,host.script,tool,args);
  return {...call,env:incrementalChildEnvironmentV1(host.environment,{})};
 };
 const incremental=incrementalCliPortsV1({...host,now:()=>Math.floor(Date.now())}),slices=new Map<string,Awaited<ReturnType<typeof incremental.slice>>>();
 const service:BenchServicePortV1=host.service??Object.freeze({
  available:async()=>{try{const catalog=JSON.parse(await readFile(join(host.root,host.catalog),"utf8")) as {operations:readonly {name:string}[]};return catalog.operations.some(row=>row.name===policy.serviceOperation);}catch{return false;}},
  start:async()=>{const call=cli("neutral",[policy.serviceOperation,"start"]);await host.run(call.command,call.args,host.root,call.env,host.signal);},
  stop:async()=>{const call=cli("neutral",[policy.serviceOperation,"stop"]);await host.run(call.command,call.args,host.root,call.env,host.signal);}
 });
 return Object.freeze({
  read:path=>readFile(incrementalInsideV1(host.root,path),"utf8"),
  write:async(path,text)=>{await writeFile(incrementalInsideV1(host.root,path),text);},
  spawn:host.spawn??((spec,stop)=>{const call=cli(spec.tool,spec.args);return benchSpawnV1(call.command,call.args,host.root,call.env,benchMarkersV1(spec.tool),spec.memory,stop,host.now);}),
  cache:host.cache??(async(tool,target)=>{
   const key=`${tool}\0${target}`;
   if(!slices.has(key))slices.set(key,await incremental.slice({id:"bench",target,mutation:{file:""}} as never,tool));
   const slice=slices.get(key)!,observation=await incremental.execute(tool,target,slice),executed=new Set(observation.executed),ids=Object.keys(slice.tasks),cacheable=ids.filter(id=>slice.tasks[id]!.cache);
   return {cacheable:cacheable.length,executedCacheable:cacheable.filter(id=>executed.has(id)).length,uncachedExecuted:ids.filter(id=>!slice.tasks[id]!.cache&&executed.has(id)).length,total:ids.length};
  }),
  concurrency:host.concurrency??(async(tool,workload,sessions)=>{
   if(!host.commandPorts)throw Error(both("The command ports are required for concurrency runs","Für Gleichzeitigkeitsläufe werden die Befehlsanschlüsse benötigt"));
   const out:string[]=[];
   try{await runConcurrencyCommandV1(host.live,["concurrency","run",`--workloads=${workload}`,`--sessions=${sessions}`,`--implementations=${tool}`,"--json",`--root=${host.root}`],{...host.commandPorts,write:line=>{out.push(line);}});}
   catch(error){if(!out.length)throw error;}
   return benchConcurrencyFromReportV1(JSON.parse(out.join("\n")) as ConcurrentReportV1);
  }),
  service,
  host:host.sample??(()=>benchHostV1({platform:platform(),arch:arch(),cpus:Math.max(1,cpus().length),memoryBytes:totalmem(),load1:platform()==="win32"?null:loadavg()[0]!})),
  clock:host.clock??(()=>new Date().toISOString()),
  fresh:host.fresh,
  now:host.now,
  nonce:()=>Math.floor(Date.now()).toString(36).slice(-8).padStart(8,"0")+(++sequence).toString(36),
  check:host.check,
  progress:host.progress,
  signal
 });
}
