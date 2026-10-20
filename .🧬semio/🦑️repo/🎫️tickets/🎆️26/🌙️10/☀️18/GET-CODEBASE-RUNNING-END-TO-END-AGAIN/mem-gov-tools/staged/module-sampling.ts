import {execFile} from "node:child_process";
import {readdir,readFile} from "node:fs/promises";
import {cpus,freemem,loadavg,platform as osPlatform,totalmem} from "node:os";
import {hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";

const OPTIONAL_PROBE_MILLISECONDS=3000,MEGABYTE=1048576,SCALE:Readonly<Record<string,number>>={K:1/1024,M:1,G:1024,T:1048576};

/** 🧮️ Raw platform counters in bytes before the policy turns them into one sample; `null` marks a counter the platform does not offer. */
export type HostCountersV1=Readonly<{availableBytes:number|null;constrainedBytes:number|null;swapUsedBytes:number|null;swappedOutBytes:number|null;pressureLevel:"normal"|"warn"|"critical"|null;stallPercent:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null}>;
/** 🔌️ Runs one program and answers its standard output; the only way the sampler reaches a command. */
export type HostCommandRunnerV1=(command:string,args:readonly string[],signal?:AbortSignal)=>Promise<string>;
/** 📄️ Reads one text file, answering null when it does not exist. */
export type HostTextReaderV1=(path:string)=>Promise<string|null>;
/** 🔬️ Everything one platform read needs, injectable so every platform parses from fixtures. */
export type HostProbeV1=Readonly<{platform:HostSampleV1["platform"];run:HostCommandRunnerV1;read:HostTextReaderV1;signal?:AbortSignal}>;
/** 🗃️ A shared sample record: the sample plus the cumulative swap counter its successor needs for a rate. */
export type HostSampleRecordV1=Readonly<{host:HostSampleV1;swappedOutMegabytes:number|null}>;

/** 🍎️ Parses macOS `vm_stat`, `sysctl vm.swapusage`, `sysctl kern.memorystatus_vm_pressure_level`, `sysctl kern.num_files kern.maxfiles` and `hdiutil info` text; available is free, inactive and speculative pages, and the attached images are the `images` count of `hdiutil info`, or its `image-path` records. */
export function parseDarwinCountersV1(vmStat:string,swapUsage:string,pressureLevel:string,filesTable:string="",imageInfo:string|null=null):HostCountersV1{
 const page=Number(/page size of (\d+) bytes/u.exec(vmStat)?.[1]??4096),pages=(name:string)=>Number(new RegExp(`^${name}:\\s+(\\d+)\\.`,"mu").exec(vmStat)?.[1]??0),used=/used = ([\d.]+)([KMGT])/u.exec(swapUsage),level=Number(pressureLevel.trim());
 return Object.freeze({availableBytes:(pages("Pages free")+pages("Pages inactive")+pages("Pages speculative"))*page,constrainedBytes:null,swapUsedBytes:used?Math.round(Number(used[1])*SCALE[used[2]!]!*MEGABYTE):null,swappedOutBytes:pages("Swapouts")*page,pressureLevel:level===4?"critical":level===2?"warn":level===1?"normal":null,stallPercent:null,...darwinFilesV1(filesTable),attachedImages:darwinImagesV1(imageInfo)});
}

const wholeV1=(match:RegExpExecArray|null):number|null=>match?Number(match[1]):null;
function darwinFilesV1(table:string):Pick<HostCountersV1,"openFiles"|"openFilesMaximum">{
 return {openFiles:wholeV1(/^kern\.num_files:\s+(\d+)/mu.exec(table)),openFilesMaximum:wholeV1(/^kern\.maxfiles:\s+(\d+)/mu.exec(table))};
}
function darwinImagesV1(info:string|null):number|null{
 if(info===null)return null;
 return wholeV1(/^images\s*:\s*(\d+)/mu.exec(info))??(info.match(/^image-path\s*:/gmu)?.length??0);
}

/** 🐧️ Parses Linux `/proc/meminfo`, `/proc/vmstat`, `/proc/pressure/memory`, the optional cgroup v2 limit files and `/proc/sys/fs/file-nr` (allocated, unused, maximum); a Linux host has no attachable images, because its partitions are directories, so the count stays unknown. */
export function parseLinuxCountersV1(meminfo:string,vmstat:string,pressure:string|null,cgroupMaximum:string|null,cgroupCurrent:string|null,fileNr:string|null=null):HostCountersV1{
 const kib=(name:string):number|null=>{const match=new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(meminfo);return match?Number(match[1])*1024:null;},total=kib("SwapTotal"),free=kib("SwapFree"),out=/^pswpout (\d+)$/mu.exec(vmstat),some=/^some avg10=([\d.]+)/mu.exec(pressure??""),maximum=cgroupMaximum?.trim(),limit=maximum!==undefined&&/^\d+$/u.test(maximum)?Number(maximum):null,current=cgroupCurrent!==null&&/^\d+\s*$/u.test(cgroupCurrent)?Number(cgroupCurrent.trim()):null;
 const available=kib("MemAvailable");
 return Object.freeze({availableBytes:available===null?null:limit!==null&&current!==null?Math.min(available,Math.max(0,limit-current)):available,constrainedBytes:limit,swapUsedBytes:total===null||free===null?null:total-free,swappedOutBytes:out?Number(out[1])*4096:null,pressureLevel:null,stallPercent:some?Number(some[1]):null,...linuxFilesV1(fileNr),attachedImages:null});
}

function linuxFilesV1(fileNr:string|null):Pick<HostCountersV1,"openFiles"|"openFilesMaximum">{
 const row=/^(\d+)\s+(\d+)\s+(\d+)\s*$/mu.exec(fileNr??"");
 return row?{openFiles:Math.max(0,Number(row[1])-Number(row[2])),openFilesMaximum:Number(row[3])}:{openFiles:null,openFilesMaximum:null};
}

/** 🧬️ Names the cgroup files that bound this process's memory, innermost cgroup first and the root last: `/proc/self/cgroup` rows `N:memory:/path` (v1, also on hybrid hosts, where the memory controller lives) or `0::/path` (v2) give the path; without membership text only the root files are named. */
export function cgroupMemoryFilesV1(membership:string|null):readonly Readonly<{maximum:string;current:string}>[]{
 const lines=(membership??"").split("\n"),v2=lines.find(line=>line.startsWith("0::")),v1=lines.find(line=>/^[0-9]+:[^:]*\bmemory\b[^:]*:/u.test(line));
 const chain=(base:string,path:string,maximum:string,current:string)=>{const parts=path.split("/").filter(Boolean);return Array.from({length:parts.length+1},(_,index)=>{const depth=parts.length-index;return{maximum:[base,...parts.slice(0,depth),maximum].join("/"),current:[base,...parts.slice(0,depth),current].join("/")};});};
 if(v1!==undefined)return chain("/sys/fs/cgroup/memory",v1.split(":").slice(2).join(":"),"memory.limit_in_bytes","memory.usage_in_bytes");
 if(v2!==undefined)return chain("/sys/fs/cgroup",v2.slice(3),"memory.max","memory.current");
 return[{maximum:"/sys/fs/cgroup/memory.max",current:"/sys/fs/cgroup/memory.current"}];
}

/** 🧮️ Picks, over the chain of cgroup levels, the level that leaves the least memory; an unlimited level (`max`, or a v1 limit of 2^62 and beyond) constrains nothing. */
export function effectiveCgroupMemoryV1(levels:readonly Readonly<{maximum:string|null;current:string|null}>[]):Readonly<{maximum:string|null;current:string|null}>{
 let best:Readonly<{maximum:string|null;current:string|null}>|undefined,room=Infinity;
 for(const level of levels){
  const limit=level.maximum?.trim();if(limit===undefined||!/^[0-9]+$/u.test(limit))continue;
  const bytes=Number(limit);if(!Number.isFinite(bytes)||bytes>=2**62)continue;
  const used=level.current!==null&&/^[0-9]+$/u.test(level.current.trim())?Number(level.current.trim()):0;
  if(bytes-used<room){room=bytes-used;best=level;}
 }
 return best??{maximum:null,current:null};
}

/** 🧠️ Reads `cpu.max` rows (`max 100000` or `200000 100000`) along the cgroup chain and answers the tightest quota in whole cores, or null when no level limits the CPU. */
export function parseCpuQuotaCoresV1(rows:readonly(string|null)[]):number|null{
 let best:number|null=null;
 for(const row of rows){
  const match=/^([0-9]+|max)\s+([0-9]+)\s*$/u.exec(row??"");if(!match||match[1]==="max"||Number(match[2])<1)continue;
  const cores=Math.max(1,Math.ceil(Number(match[1])/Number(match[2])));if(best===null||cores<best)best=cores;
 }
 return best;
}

/** 🧠️ Counts the cores this process may use: the logical cores, bounded by the tightest cgroup v2 CPU quota above the process on Linux. */
export async function readHostCoresV1(probe:HostProbeV1,logical:number):Promise<number>{
 if(probe.platform!=="linux")return logical;
 const membership=await probe.read("/proc/self/cgroup");if(!(membership??"").split("\n").some(line=>line.startsWith("0::")))return logical;
 const quota=parseCpuQuotaCoresV1(await Promise.all(cgroupMemoryFilesV1(membership).map(row=>probe.read(row.maximum.replace(/memory\.max$/u,"cpu.max")))));
 return quota===null?logical:Math.min(logical,quota);
}

/** 🔭️ Turns raw counters, host facts and the previous record into one sample; the rate of swap-out needs the earlier cumulative counter. */
export function reduceHostSampleV1(policy:HostCapacityPolicyV1,facts:Readonly<{platform:HostSampleV1["platform"];cores:number;totalMegabytes:number;atMs:number;load:number|null}>,counters:HostCountersV1,previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>):HostSampleV1{
 const megabytes=(bytes:number|null)=>bytes===null?null:Math.floor(bytes/MEGABYTE),constrained=megabytes(counters.constrainedBytes),total=constrained===null?facts.totalMegabytes:Math.min(facts.totalMegabytes,constrained),available=Math.min(total,megabytes(counters.availableBytes)??total);
 const share=available*10000/total,fromAvailable=share<policy.pressure.availableCriticalBasisPoints?"critical" as const:share<policy.pressure.availableWarnBasisPoints?"warn" as const:"normal" as const;
 const fromStall=counters.stallPercent===null?undefined:counters.stallPercent>=policy.pressure.psiCriticalPercent?"critical" as const:counters.stallPercent>=policy.pressure.psiWarnPercent?"warn" as const:"normal" as const;
 const rank={normal:0,warn:1,critical:2} as const,levels=[counters.pressureLevel??fromStall??"normal",fromAvailable] as const,pressure=rank[levels[0]]>=rank[levels[1]]?levels[0]:levels[1];
 const swappedOut=megabytes(counters.swappedOutBytes),seconds=previous===undefined?0:(facts.atMs-previous.atMs)/1000,rate=previous!==undefined&&seconds>0&&swappedOut!==null&&previous.swappedOutMegabytes!==null?Math.max(0,Math.round((swappedOut-previous.swappedOutMegabytes)/seconds*100)/100):null;
 return Object.freeze({version:1,atMs:facts.atMs,platform:facts.platform,cores:facts.cores,totalMegabytes:total,availableMegabytes:Math.floor(available),pressure,swapUsedMegabytes:megabytes(counters.swapUsedBytes),swapOutMegabytesPerSecond:rate,load:facts.load,openFiles:counters.openFiles,openFilesMaximum:counters.openFilesMaximum,attachedImages:counters.attachedImages});
}

let previousCpu:Readonly<{idle:number;total:number}>|undefined;
/** 🪟️ Estimates the runnable load on a platform without a load average from the busy share of two CPU time readings. */
function busyLoadV1():number|null{
 let idle=0,total=0;for(const core of cpus()){idle+=core.times.idle;total+=core.times.idle+core.times.user+core.times.nice+core.times.sys+core.times.irq;}
 const before=previousCpu;previousCpu={idle,total};
 return before===undefined||total<=before.total?null:Math.round((1-(idle-before.idle)/(total-before.total))*cpus().length*100)/100;
}

/** ⏱️ Bounds an optional probe to a few seconds on top of the caller's cancellation, so a stalled `hdiutil` can never hold the admission loop. */
function boundedSignalV1(signal?:AbortSignal):AbortSignal{
 const bound=AbortSignal.timeout(OPTIONAL_PROBE_MILLISECONDS);
 return signal===undefined?bound:AbortSignal.any([signal,bound]);
}

/** 🌍️ Reads the raw counters of the probed platform; the open file table and the attached images are optional facts that fall back to `null` instead of failing the sample. */
export async function readHostCountersV1(probe:HostProbeV1):Promise<HostCountersV1>{
 if(probe.platform==="darwin"){
  const optional=(command:string,args:readonly string[]):Promise<string|null>=>probe.run(command,args,boundedSignalV1(probe.signal)).catch(()=>null);
  const[vmStat,swap,level,files,images]=await Promise.all([probe.run("vm_stat",[],probe.signal),probe.run("sysctl",["-n","vm.swapusage"],probe.signal),probe.run("sysctl",["-n","kern.memorystatus_vm_pressure_level"],probe.signal),optional("sysctl",["kern.num_files","kern.maxfiles"]),optional("/usr/bin/hdiutil",["info"])]);
  return parseDarwinCountersV1(vmStat,swap,level,files??"",images);
 }
 if(probe.platform==="linux"){const[meminfo,vmstat,pressure,fileNr,membership]=await Promise.all(["/proc/meminfo","/proc/vmstat","/proc/pressure/memory","/proc/sys/fs/file-nr","/proc/self/cgroup"].map(path=>probe.read(path)));if(meminfo==null||vmstat==null)throw hostCapacityErrorV1("countersUnavailable");const limit=effectiveCgroupMemoryV1(await Promise.all(cgroupMemoryFilesV1(membership??null).map(async row=>({maximum:await probe.read(row.maximum),current:await probe.read(row.current)}))));return parseLinuxCountersV1(meminfo,vmstat,pressure??null,limit.maximum,limit.current,fileNr??null);}
 return Object.freeze({availableBytes:freemem(),constrainedBytes:null,swapUsedBytes:null,swappedOutBytes:null,pressureLevel:null,stallPercent:null,openFiles:null,openFilesMaximum:null,attachedImages:null});
}

/** 🖥️ Reads one host sample: counters, core count, total memory and load. */
export async function readHostSampleV1(policy:HostCapacityPolicyV1,probe:HostProbeV1,previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>,now:()=>number=Date.now):Promise<HostSampleRecordV1>{
 const counters=await readHostCountersV1(probe),cores=await readHostCoresV1(probe,cpus().length||1),average=probe.platform==="win32"?busyLoadV1():loadavg()[0]!,host=reduceHostSampleV1(policy,{platform:probe.platform,cores,totalMegabytes:Math.floor(totalmem()/MEGABYTE),atMs:now(),load:average===null?null:Math.round(average*100)/100},counters,previous);
 return Object.freeze({host,swappedOutMegabytes:counters.swappedOutBytes===null?null:Math.floor(counters.swappedOutBytes/MEGABYTE)});
}

/** 🏭️ Runs a program with a hard timeout and bounded output. */
export const runHostCommandV1:HostCommandRunnerV1=(command,args,signal)=>new Promise((resolve,reject)=>{
 execFile(command,[...args],{timeout:10000,maxBuffer:4194304,windowsHide:true,encoding:"utf8",...signal?{signal}:{}},(error,stdout)=>error?reject(error):resolve(stdout));
});

/** 📄️ Reads a text file bounded to one mebibyte, answering null when it is absent. */
export const readHostTextV1:HostTextReaderV1=async path=>{
 try{return (await readFile(path,"utf8")).slice(0,MEGABYTE);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR")return null;throw error;}
};

/** 🧰️ The probe of the running machine. */
export function currentHostProbeV1(signal?:AbortSignal):HostProbeV1{
 const platform=osPlatform();
 if(platform!=="darwin"&&platform!=="linux"&&platform!=="win32")throw hostCapacityErrorV1("platformUnsupported",{platform});
 return Object.freeze({platform,run:runHostCommandV1,read:readHostTextV1,...signal?{signal}:{}});
}

/** 🌳️ Parses `ps -A -o pid=,ppid=,rss=` into rows. */
export function parseProcessTableV1(text:string):readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[]{
 const rows=[];for(const line of text.split("\n")){const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s*$/u.exec(line);if(match)rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),rssKilobytes:Number(match[3])}));}
 return rows;
}

/** 🌲️ Sums the resident megabytes of a process and all of its descendants. */
export function treeMegabytesV1(rows:readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[],root:number):number|null{
 if(!rows.some(row=>row.pid===root))return null;
 const children=new Map<number,number[]>();for(const row of rows){const list=children.get(row.parent)??[];list.push(row.pid);children.set(row.parent,list);}
 const kilobytes=new Map(rows.map(row=>[row.pid,row.rssKilobytes] as const)),seen=new Set<number>(),pending=[root];let total=0;
 for(let pid=pending.pop();pid!==undefined;pid=pending.pop()){if(seen.has(pid))continue;seen.add(pid);total+=kilobytes.get(pid)??0;pending.push(...children.get(pid)??[]);}
 return Math.floor(total/1024);
}

/** 🌳️ Parses `/proc/<pid>/status` into a process row: the parent from `PPid:` and the resident set from `VmRSS:` (absent for kernel threads, so 0). */
export function parseProcStatusV1(pid:number,status:string):Readonly<{pid:number;parent:number;rssKilobytes:number}>|undefined{
 const parent=/^PPid:\s+([0-9]+)\s*$/mu.exec(status),resident=/^VmRSS:\s+([0-9]+)\s+kB\s*$/mu.exec(status);
 return parent?Object.freeze({pid,parent:Number(parent[1]),rssKilobytes:resident?Number(resident[1]):0}):undefined;
}

/** 🗂️ Reads every live process of a Linux host from `/proc` through the injected ports, skipping the ones that exit while the table is read. */
export async function readLinuxProcessTableV1(read:HostTextReaderV1,list:()=>Promise<readonly string[]>):Promise<readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[]>{
 const rows=await Promise.all((await list()).filter(name=>/^[0-9]+$/u.test(name)).map(async name=>{try{const status=await read("/proc/"+name+"/status");return status===null?undefined:parseProcStatusV1(Number(name),status);}catch{return undefined;}}));
 return rows.filter((row):row is NonNullable<typeof row>=>row!==undefined);
}

/** 👁️ Observes what the process tree under one pid holds: `/proc` on Linux (no external tool), `ps` on macOS, null where the platform has no cheap process table. */
export async function observeTreeMegabytesV1(pid:number,run:HostCommandRunnerV1=runHostCommandV1,platform:HostSampleV1["platform"]=osPlatform() as HostSampleV1["platform"],proc:Readonly<{read:HostTextReaderV1;list:()=>Promise<readonly string[]>}>={read:readHostTextV1,list:()=>readdir("/proc")}):Promise<number|null>{
 if(platform==="win32")return null;
 if(platform==="linux")return treeMegabytesV1(await readLinuxProcessTableV1(proc.read,proc.list),pid);
 return treeMegabytesV1(parseProcessTableV1(await run("ps",["-A","-o","pid=,ppid=,rss="])),pid);
}
