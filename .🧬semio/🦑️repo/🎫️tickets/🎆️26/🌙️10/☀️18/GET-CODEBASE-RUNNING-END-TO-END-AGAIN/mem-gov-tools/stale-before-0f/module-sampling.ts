import {execFile} from "node:child_process";
import {readFile} from "node:fs/promises";
import {cpus,freemem,loadavg,platform as osPlatform,totalmem} from "node:os";
import type {HostCapacityPolicyV1,HostSampleV1} from "../📏️decision/🟦️.ts";

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

/** 🐧️ Parses Linux `/proc/meminfo`, `/proc/vmstat`, `/proc/pressure/memory`, the optional cgroup v2 limit files, `/proc/sys/fs/file-nr` (allocated, unused, maximum) and `/proc/mounts`, whose loop devices are the attached images. */
export function parseLinuxCountersV1(meminfo:string,vmstat:string,pressure:string|null,cgroupMaximum:string|null,cgroupCurrent:string|null,fileNr:string|null=null,mounts:string|null=null):HostCountersV1{
 const kib=(name:string):number|null=>{const match=new RegExp(`^${name}:\\s+(\\d+) kB`,"mu").exec(meminfo);return match?Number(match[1])*1024:null;},total=kib("SwapTotal"),free=kib("SwapFree"),out=/^pswpout (\d+)$/mu.exec(vmstat),some=/^some avg10=([\d.]+)/mu.exec(pressure??""),maximum=cgroupMaximum?.trim(),limit=maximum!==undefined&&/^\d+$/u.test(maximum)?Number(maximum):null,current=cgroupCurrent!==null&&/^\d+\s*$/u.test(cgroupCurrent)?Number(cgroupCurrent.trim()):null;
 const available=kib("MemAvailable");
 return Object.freeze({availableBytes:available===null?null:limit!==null&&current!==null?Math.min(available,Math.max(0,limit-current)):available,constrainedBytes:limit,swapUsedBytes:total===null||free===null?null:total-free,swappedOutBytes:out?Number(out[1])*4096:null,pressureLevel:null,stallPercent:some?Number(some[1]):null,...linuxFilesV1(fileNr),attachedImages:mounts===null?null:mounts.match(/^\/dev\/loop\d+\s/gmu)?.length??0});
}

function linuxFilesV1(fileNr:string|null):Pick<HostCountersV1,"openFiles"|"openFilesMaximum">{
 const row=/^(\d+)\s+(\d+)\s+(\d+)\s*$/mu.exec(fileNr??"");
 return row?{openFiles:Math.max(0,Number(row[1])-Number(row[2])),openFilesMaximum:Number(row[3])}:{openFiles:null,openFilesMaximum:null};
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
 if(probe.platform==="linux"){const[meminfo,vmstat,pressure,maximum,current,fileNr,mounts]=await Promise.all(["/proc/meminfo","/proc/vmstat","/proc/pressure/memory","/sys/fs/cgroup/memory.max","/sys/fs/cgroup/memory.current","/proc/sys/fs/file-nr","/proc/mounts"].map(path=>probe.read(path)));if(meminfo===null||vmstat===null)throw Error("Linux memory counters unavailable");return parseLinuxCountersV1(meminfo,vmstat,pressure,maximum,current,fileNr,mounts);}
 return Object.freeze({availableBytes:freemem(),constrainedBytes:null,swapUsedBytes:null,swappedOutBytes:null,pressureLevel:null,stallPercent:null,openFiles:null,openFilesMaximum:null,attachedImages:null});
}

/** 🖥️ Reads one host sample: counters, core count, total memory and load. */
export async function readHostSampleV1(policy:HostCapacityPolicyV1,probe:HostProbeV1,previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>,now:()=>number=Date.now):Promise<HostSampleRecordV1>{
 const counters=await readHostCountersV1(probe),average=probe.platform==="win32"?busyLoadV1():loadavg()[0]!,host=reduceHostSampleV1(policy,{platform:probe.platform,cores:cpus().length||1,totalMegabytes:Math.floor(totalmem()/MEGABYTE),atMs:now(),load:average===null?null:Math.round(average*100)/100},counters,previous);
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
 if(platform!=="darwin"&&platform!=="linux"&&platform!=="win32")throw Error(`Host capacity does not support ${platform}`);
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

/** 👁️ Observes what the process tree under one pid holds; null where the platform has no cheap process table. */
export async function observeTreeMegabytesV1(pid:number,run:HostCommandRunnerV1=runHostCommandV1,platform:HostSampleV1["platform"]=osPlatform() as HostSampleV1["platform"]):Promise<number|null>{
 if(platform==="win32")return null;
 return treeMegabytesV1(parseProcessTableV1(await run("ps",["-A","-o","pid=,ppid=,rss="])),pid);
}
