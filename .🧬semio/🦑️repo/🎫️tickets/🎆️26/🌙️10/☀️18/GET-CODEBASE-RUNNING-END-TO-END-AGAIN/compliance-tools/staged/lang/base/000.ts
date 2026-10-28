import {execFile} from "node:child_process";
import {readdir,readFile} from "node:fs/promises";
import {cpus,freemem,loadavg,platform as osPlatform,totalmem} from "node:os";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {formatHostCapacityV1,hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";
import {hostProfileV1} from "../🪞️profile/🟦️.ts";

const totalEnvironment=schema.definitions.Protocol.const.totalEnvironment;

const OBSERVERS=schema.definitions.Observers.const,OPTIONAL_PROBE_MILLISECONDS=3000,MEGABYTE=1048576,SCALE:Readonly<Record<string,number>>={K:1/1024,M:1,G:1024,T:1048576};

/** 🧮️ Raw platform counters in bytes before the policy turns them into one sample; `null` marks a counter the platform does not offer. */
export type HostCountersV1=Readonly<{availableBytes:number|null;compressedBytes:number|null;constrainedBytes:number|null;swapUsedBytes:number|null;swappedOutBytes:number|null;pressureLevel:"normal"|"warn"|"critical"|null;stallPercent:number|null;openFiles:number|null;openFilesMaximum:number|null;attachedImages:number|null;partial?:true}>;
/** 🔌️ Runs one program and answers its standard output; the only way the sampler reaches a command. */
export type HostCommandRunnerV1=(command:string,args:readonly string[],signal?:AbortSignal)=>Promise<string>;
/** 📄️ Reads one text file, answering null when it does not exist. */
export type HostTextReaderV1=(path:string)=>Promise<string|null>;
/** 🔬️ Everything one platform read needs, injectable so every platform parses from fixtures. */
export type HostProbeV1=Readonly<{platform:HostSampleV1["platform"];run:HostCommandRunnerV1;read:HostTextReaderV1;signal?:AbortSignal;notify?:(line:string)=>void}>;
/** 🗃️ A shared sample record: the sample plus the cumulative swap counter its successor needs for a rate. */
export type HostSampleRecordV1=Readonly<{host:HostSampleV1;swappedOutMegabytes:number|null;declared?:string}>;

/** 🍎️ Parses macOS `vm_stat`, `sysctl vm.swapusage`, `sysctl kern.memorystatus_vm_pressure_level`, `sysctl kern.num_files kern.maxfiles` and `hdiutil info` text; available is what the kernel can hand out without compressing or swapping anything: free, file-backed and purgeable pages (Activity Monitor's "memory used" is everything else, the compressor included), never more than free, inactive and speculative pages; the compressor's physical pages are reported on their own, and the attached images are the `images` count of `hdiutil info`, or its `image-path` records. */
export function parseDarwinCountersV1(vmStat:string,swapUsage:string,pressureLevel:string,filesTable:string="",imageInfo:string|null=null):HostCountersV1{
 const page=Number(/page size of (\d+) bytes/u.exec(vmStat)?.[1]??4096),pages=(name:string)=>Number(new RegExp(`^${name}:\\s+(\\d+)\\.`,"mu").exec(vmStat)?.[1]??0),used=/used = ([\d.]+)([KMGT])/u.exec(swapUsage),level=Number(pressureLevel.trim());
 const legacy=pages("Pages free")+pages("Pages inactive")+pages("Pages speculative"),reclaimable=/^File-backed pages:/mu.test(vmStat)?pages("Pages free")+pages("File-backed pages")+pages("Pages purgeable"):legacy,compressed=/^Pages occupied by compressor:/mu.test(vmStat)?pages("Pages occupied by compressor")*page:null;
 return Object.freeze({availableBytes:Math.min(legacy,reclaimable)*page,compressedBytes:compressed,constrainedBytes:null,swapUsedBytes:used?Math.round(Number(used[1])*SCALE[used[2]!]!*MEGABYTE):null,swappedOutBytes:pages("Swapouts")*page,pressureLevel:level===4?"critical":level===2?"warn":level===1?"normal":null,stallPercent:null,...darwinFilesV1(filesTable),attachedImages:darwinImagesV1(imageInfo)});
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
 return Object.freeze({availableBytes:available===null?null:limit!==null&&current!==null?Math.min(available,Math.max(0,limit-current)):available,compressedBytes:null,constrainedBytes:limit,swapUsedBytes:total===null||free===null?null:total-free,swappedOutBytes:out?Number(out[1])*4096:null,pressureLevel:null,stallPercent:some?Number(some[1]):null,...linuxFilesV1(fileNr),attachedImages:null});
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

/** 🐘️ Picks, over the chain of cgroup levels, the level that leaves the least memory; an unlimited level (`max`, or a v1 limit of 2^62 and beyond) constrains nothing. */
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

/** 🧱️ Names the cgroup v1 CFS files that bound this process's CPU, innermost cgroup first and the controller root last: the `N:cpu,cpuacct:/path` (or `N:cpu:/path`) row of `/proc/self/cgroup` gives the path under `/sys/fs/cgroup/cpu,cpuacct` (also mounted as `/sys/fs/cgroup/cpu`); no such row names nothing. */
export function cgroupV1CpuFilesV1(membership:string|null):readonly Readonly<{quota:string;period:string}>[]{
 const row=(membership??"").split("\n").find(line=>/^[0-9]+:[^:]*\bcpu\b[^:]*:/u.test(line));if(row===undefined)return[];
 const controllers=row.split(":")[1]!,mount="/sys/fs/cgroup/"+controllers,parts=row.split(":").slice(2).join(":").split("/").filter(Boolean);
 return Array.from({length:parts.length+1},(_,index)=>{const depth=parts.length-index,base=[mount,...parts.slice(0,depth)].join("/");return{quota:base+"/cpu.cfs_quota_us",period:base+"/cpu.cfs_period_us"};});
}

/** 🧠️ Turns `cpu.cfs_quota_us` and `cpu.cfs_period_us` rows (cgroup v1; a quota of `-1` is unlimited) into the tightest quota in whole cores, or null when no level limits the CPU. */
export function parseCfsQuotaCoresV1(rows:readonly Readonly<{quota:string|null;period:string|null}>[]):number|null{
 const cpuMax=rows.flatMap(row=>{const quota=/^-?[0-9]+$/u.exec((row.quota??"").trim())?.[0],period=/^[0-9]+$/u.exec((row.period??"").trim())?.[0];return quota===undefined||period===undefined||Number(quota)<0?[]:[`${quota} ${period}`];});
 return parseCpuQuotaCoresV1(cpuMax);
}

/** ⚙️ Counts the cores this process may use: the logical cores, bounded by the tightest CPU quota above the process on Linux (cgroup v2 `cpu.max`, or cgroup v1 `cpu.cfs_quota_us` over `cpu.cfs_period_us`). */
export async function readHostCoresV1(probe:HostProbeV1,logical:number):Promise<number>{
 if(probe.platform!=="linux")return logical;
 const membership=await probe.read("/proc/self/cgroup");
 const v2=(membership??"").split("\n").some(line=>line.startsWith("0::")),v1=cgroupV1CpuFilesV1(membership);
 const quota=v2?parseCpuQuotaCoresV1(await Promise.all(cgroupMemoryFilesV1(membership).map(row=>probe.read(row.maximum.replace(/memory\.max$/u,"cpu.max"))))):v1.length?parseCfsQuotaCoresV1(await Promise.all(v1.map(async row=>({quota:await probe.read(row.quota),period:await probe.read(row.period)})))):null;
 return quota===null?logical:Math.min(logical,quota);
}

/** 🔭️ Turns raw counters, host facts and the previous record into one sample; the rate of swap-out needs the earlier cumulative counter. */
export function reduceHostSampleV1(policy:HostCapacityPolicyV1,facts:Readonly<{platform:HostSampleV1["platform"];cores:number;totalMegabytes:number;atMs:number;load:number|null}>,counters:HostCountersV1,previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>):HostSampleV1{
 const megabytes=(bytes:number|null)=>bytes===null?null:Math.floor(bytes/MEGABYTE),constrained=megabytes(counters.constrainedBytes),total=constrained===null?facts.totalMegabytes:Math.min(facts.totalMegabytes,constrained),available=Math.min(total,megabytes(counters.availableBytes)??total);
 const share=available*10000/total,fromAvailable=share<policy.pressure.availableCriticalBasisPoints?"critical" as const:share<policy.pressure.availableWarnBasisPoints?"warn" as const:"normal" as const;
 const fromStall=counters.stallPercent===null?undefined:counters.stallPercent>=policy.pressure.psiCriticalPercent?"critical" as const:counters.stallPercent>=policy.pressure.psiWarnPercent?"warn" as const:"normal" as const;
 const squeeze=counters.compressedBytes==null?0:counters.compressedBytes*10000/(total*MEGABYTE),fromCompressed=squeeze>=policy.pressure.compressedCriticalBasisPoints?"critical" as const:squeeze>=policy.pressure.compressedWarnBasisPoints?"warn" as const:"normal" as const;
 const rank={normal:0,warn:1,critical:2} as const,pressure=([counters.pressureLevel??fromStall??"normal",fromAvailable,fromCompressed,...counters.partial?["warn" as const]:[]] as const).reduce((worst,level)=>rank[level]>rank[worst]?level:worst);
 const swappedOut=megabytes(counters.swappedOutBytes),seconds=previous===undefined?0:(facts.atMs-previous.atMs)/1000,rate=previous!==undefined&&seconds>0&&swappedOut!==null&&previous.swappedOutMegabytes!==null?Math.max(0,Math.round((swappedOut-previous.swappedOutMegabytes)/seconds*100)/100):null;
 return Object.freeze({version:1,atMs:facts.atMs,platform:facts.platform,cores:facts.cores,totalMegabytes:total,availableMegabytes:Math.floor(available),pressure,swapUsedMegabytes:megabytes(counters.swapUsedBytes),swapOutMegabytesPerSecond:rate,load:facts.load,openFiles:counters.openFiles,openFilesMaximum:counters.openFilesMaximum,attachedImages:counters.attachedImages,...counters.partial?{counters:"partial" as const}:{}});
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

/** 🪟️ The one PowerShell command that reads the Windows memory counters: available bytes (free, zeroed and standby pages, what Task Manager calls available), the cumulative pages written to the page file and the page file in use. */
export const WINDOWS_COUNTERS_COMMAND="$m=Get-CimInstance Win32_PerfRawData_PerfOS_Memory;$p=Get-CimInstance Win32_PageFileUsage|Measure-Object -Property CurrentUsage -Sum;[pscustomobject]@{availableBytes=[int64]$m.AvailableBytes;pagesOutput=[int64]$m.PagesOutputPersec;pageFileUsedMegabytes=[int64]$p.Sum}|ConvertTo-Json -Compress";

/** 🪟️ Parses the JSON of `WINDOWS_COUNTERS_COMMAND`; the page counter is cumulative, so the sampler derives the swap-out rate like on the other platforms, and a missing page file reads as zero swap. A Windows host has no compressor, open file table or attachable image count here, so those stay unknown. */
export function parseWindowsCountersV1(text:string):HostCountersV1{
 const row=JSON.parse(text) as Record<string,unknown>,whole=(name:string):number|null=>typeof row[name]==="number"&&Number.isFinite(row[name])&&(row[name] as number)>=0?Math.floor(row[name] as number):null;
 const available=whole("availableBytes"),pages=whole("pagesOutput"),used=whole("pageFileUsedMegabytes");
 return Object.freeze({availableBytes:available,compressedBytes:null,constrainedBytes:null,swapUsedBytes:used===null?0:used*MEGABYTE,swappedOutBytes:pages===null?null:pages*4096,pressureLevel:null,stallPercent:null,openFiles:null,openFilesMaximum:null,attachedImages:null});
}

const noticedV1=new WeakMap<object,Set<string>>(),standardErrorV1={};
/** 📣️ Says once per process and sink, in every language, that this host is blind where the gate cannot learn or measure: nothing fails, so the line is the only signal. Never throws. */
function noticeOnceV1(key:string,message:"treeUnobserved"|"footprintFallback"|"countersPartial",values:Readonly<Record<string,string|number>>,notify?:(line:string)=>void):void{
 const sink=notify??standardErrorV1,seen=noticedV1.get(sink)??new Set<string>();noticedV1.set(sink,seen);
 if(seen.has(key))return;seen.add(key);
 try{for(const language of["en","de"] as const){const line=formatHostCapacityV1(language,message,values);if(notify)notify(line);else process.stderr.write(line+"\n");}}catch{}
}

/** 🌍️ Reads the raw counters of the probed platform; the open file table and the attached images are optional facts that fall back to `null` instead of failing the sample. */
export async function readHostCountersV1(probe:HostProbeV1):Promise<HostCountersV1>{
 if(probe.platform==="darwin"){
  const optional=(command:string,args:readonly string[]):Promise<string|null>=>probe.run(command,args,boundedSignalV1(probe.signal)).catch(()=>null);
  const[vmStat,swap,level,files,images]=await Promise.all([probe.run("vm_stat",[],probe.signal),probe.run("sysctl",["-n","vm.swapusage"],probe.signal),probe.run("sysctl",["-n","kern.memorystatus_vm_pressure_level"],probe.signal),optional("sysctl",["kern.num_files","kern.maxfiles"]),optional("/usr/bin/hdiutil",["info"])]);
  return parseDarwinCountersV1(vmStat,swap,level,files??"",images);
 }
 if(probe.platform==="linux"){const[meminfo,vmstat,pressure,fileNr,membership]=await Promise.all(["/proc/meminfo","/proc/vmstat","/proc/pressure/memory","/proc/sys/fs/file-nr","/proc/self/cgroup"].map(path=>probe.read(path)));if(meminfo==null||vmstat==null)throw hostCapacityErrorV1("countersUnavailable");const limit=effectiveCgroupMemoryV1(await Promise.all(cgroupMemoryFilesV1(membership??null).map(async row=>({maximum:await probe.read(row.maximum),current:await probe.read(row.current)}))));return parseLinuxCountersV1(meminfo,vmstat,pressure??null,limit.maximum,limit.current,fileNr??null);}
 let failure="the platform probe is unavailable";
 if(probe.platform==="win32"){
  try{return parseWindowsCountersV1(await probe.run("powershell.exe",["-NoProfile","-NonInteractive","-Command",WINDOWS_COUNTERS_COMMAND],boundedSignalV1(probe.signal)));}
  catch(error){failure=String((error as Error)?.message??error).split("\n")[0]!.slice(0,200);}
 }
 noticeOnceV1("countersPartial."+probe.platform,"countersPartial",{platform:probe.platform,reason:failure},probe.notify);
 return Object.freeze({availableBytes:freemem(),compressedBytes:null,constrainedBytes:null,swapUsedBytes:null,swappedOutBytes:null,pressureLevel:null,stallPercent:null,openFiles:null,openFilesMaximum:null,attachedImages:null,partial:true as const});
}

/** 🪞️ Reads the simulated host size from the environment: `SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES` makes a larger host report that many megabytes in total (what is available stays what the real host has free, capped at that total: a busy 32 GB host stays busy), so that a 16 GB machine can be measured on a 32 GB one; an absent or malformed value means the real host. */
export function simulatedHostTotalV1(environment:Readonly<Record<string,string|undefined>>):number|null{
 const text=environment[totalEnvironment];
 return text!==undefined&&/^[1-9][0-9]{3,8}$/u.test(text)?Number(text):null;
}

/** 🖥️ Reads one host sample: counters, core count, total memory and load; a simulated total (see `simulatedHostTotalV1`) shrinks the host. */
export async function readHostSampleV1(policy:HostCapacityPolicyV1,probe:HostProbeV1,previous?:Readonly<{atMs:number;swappedOutMegabytes:number|null}>,now:()=>number=Date.now,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<HostSampleRecordV1>{
 const counters=await readHostCountersV1(probe),realCores=await readHostCoresV1(probe,cpus().length||1),{totalMegabytes:total,cores}=hostProfileV1(environment,{totalMegabytes:Math.floor(totalmem()/MEGABYTE),cores:realCores,declared:false}),average=probe.platform==="win32"?busyLoadV1():loadavg()[0]!,host=reduceHostSampleV1(policy,{platform:probe.platform,cores,totalMegabytes:total,atMs:now(),load:average===null?null:Math.round(average*100)/100},counters,previous);
 return Object.freeze({host,swappedOutMegabytes:counters.swappedOutBytes===null?null:Math.floor(counters.swappedOutBytes/MEGABYTE)});
}

/** 🏭️ Runs a program with a hard timeout and bounded output. */
export const runHostCommandV1:HostCommandRunnerV1=(command,args,signal)=>new Promise((resolve,reject)=>{
 execFile(command,[...args],{timeout:10000,maxBuffer:4194304,windowsHide:true,encoding:"utf8",...signal?{signal}:{}},(error,stdout)=>error?reject(error):resolve(stdout));
});

/** 📖️ Reads a text file bounded to one mebibyte, answering null when it is absent. */
export const readHostTextV1:HostTextReaderV1=async path=>{
 try{return (await readFile(path,"utf8")).slice(0,MEGABYTE);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||(error as NodeJS.ErrnoException).code==="ENOTDIR")return null;throw error;}
};

/** 🧰️ The probe of the running machine. */
export function currentHostProbeV1(signal?:AbortSignal):HostProbeV1{
 const platform=osPlatform();
 if(platform!=="darwin"&&platform!=="linux"&&platform!=="win32")throw hostCapacityErrorV1("platformUnsupported",{platform});
 return Object.freeze({platform,run:runHostCommandV1,read:readHostTextV1,...signal?{signal}:{}});
}

/** 🪟️ The one PowerShell command that lists every Windows process as `pid ppid working-set-kilobytes` lines, the shape of `ps -A -o pid=,ppid=,rss=`, so one parser and one tree sum serve macOS and Windows. */
export const WINDOWS_PROCESS_TABLE_COMMAND=OBSERVERS.platforms.win32.resident.arguments[3]!;

/** 🌳️ Parses `ps -A -o pid=,ppid=,rss=` into rows. */
export function parseProcessTableV1(text:string):readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[]{
 const rows=[];for(const line of text.split("\n")){const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s*$/u.exec(line);if(match)rows.push(Object.freeze({pid:Number(match[1]),parent:Number(match[2]),rssKilobytes:Number(match[3])}));}
 return rows;
}

/** 🌿️ The pids of a process and all of its descendants, root first; empty when the process is not in the table. */
export function treePidsV1(rows:readonly Readonly<{pid:number;parent:number}>[],root:number):readonly number[]{
 if(!rows.some(row=>row.pid===root))return[];
 const children=new Map<number,number[]>();for(const row of rows){const list=children.get(row.parent)??[];list.push(row.pid);children.set(row.parent,list);}
 const seen=new Set<number>(),order:number[]=[],pending=[root];
 for(let pid=pending.pop();pid!==undefined;pid=pending.pop()){if(seen.has(pid))continue;seen.add(pid);order.push(pid);pending.push(...children.get(pid)??[]);}
 return order;
}

/** 🌲️ Sums the kilobyte column (the resident set, or the private working set of the Windows footprint table) of a process and all of its descendants. */
export function treeMegabytesV1(rows:readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[],root:number):number|null{
 const members=treePidsV1(rows,root);if(!members.length)return null;
 const kilobytes=new Map(rows.map(row=>[row.pid,row.rssKilobytes] as const));
 return Math.floor(members.reduce((total,pid)=>total+(kilobytes.get(pid)??0),0)/1024);
}

/** 🚥️ Parses `/proc/<pid>/status` into a process row: the parent from `PPid:` and the resident set from `VmRSS:` (absent for kernel threads, so 0). */
export function parseProcStatusV1(pid:number,status:string):Readonly<{pid:number;parent:number;rssKilobytes:number}>|undefined{
 const parent=/^PPid:\s+([0-9]+)\s*$/mu.exec(status),resident=/^VmRSS:\s+([0-9]+)\s+kB\s*$/mu.exec(status);
 return parent?Object.freeze({pid,parent:Number(parent[1]),rssKilobytes:resident?Number(resident[1]):0}):undefined;
}

/** 🗂️ Reads every live process of a Linux host from `/proc` through the injected ports, skipping the ones that exit while the table is read. */
export async function readLinuxProcessTableV1(read:HostTextReaderV1,list:()=>Promise<readonly string[]>):Promise<readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[]>{
 const rows=await Promise.all((await list()).filter(name=>/^[0-9]+$/u.test(name)).map(async name=>{try{const status=await read("/proc/"+name+"/status");return status===null?undefined:parseProcStatusV1(Number(name),status);}catch{return undefined;}}));
 return rows.filter((row):row is NonNullable<typeof row>=>row!==undefined);
}

/** 🔬️ The memory measure a process tree is observed in: the footprint is what the processes hold privately plus what was swapped or compressed out of them (macOS `phys_footprint`, Linux proportional set size plus swapped proportional set size, Windows private working set), the resident set is what their pages occupy right now; macOS keeps a process at its resident high-water mark after it freed memory and does not count compressed pages at all, so only the footprint prices a process by what it really holds (`Observers` of the schema). */
export type HostMemoryMeasureV1="footprint"|"resident";

/** 🍏️ Parses `footprint -f bytes --noCategories -p …` into bytes by pid: the kernel's `phys_footprint` of a block where it prints one, the footprint of the block header otherwise; the summary, the lines of a process that vanished and the arguments of a refused one carry no pid and are skipped. */
export function parseFootprintV1(text:string):ReadonlyMap<number,number>{
 const found=new Map<number,number>();let pid:number|undefined;
 for(const line of text.split("\n")){
  const head=/^.*\[(\d+)\]:[^\[]*\bFootprint:\s+(\d+)\s+B\b/u.exec(line);
  if(head){pid=Number(head[1]);found.set(pid,Number(head[2]));continue;}
  const kernel=/^\s*phys_footprint:\s+(\d+)\s+B\s*$/u.exec(line);
  if(kernel&&pid!==undefined)found.set(pid,Number(kernel[1]));
 }
 return found;
}

/** 🐾️ Parses `/proc/<pid>/smaps_rollup` into the kilobytes of the footprint fields (proportional set size plus swapped proportional set size); undefined when the first field is absent. */
export function parseSmapsRollupV1(text:string,fields:readonly string[]=OBSERVERS.platforms.linux.footprint.fields):number|undefined{
 let total=0;
 for(const [index,field] of fields.entries()){const match=new RegExp(`^${field}:\\s+(\\d+)\\s+kB\\s*$`,"mu").exec(text);if(match)total+=Number(match[1]);else if(index===0)return undefined;}
 return total;
}

/** 👁️ Observes what the process tree under one pid holds, in the footprint by default (`HostMemoryMeasureV1`): `footprint` on macOS (one call for the whole tree, the process table names its members), `/proc/<pid>/smaps_rollup` on Linux (no external tool), one PowerShell table of private working sets on Windows; a platform whose footprint source cannot be read says so once in every language and reads the resident set instead. null when the process is gone or, on Windows, when PowerShell cannot answer at all, which is announced once in every language (the gate then learns nothing from the run and keeps the declared sizes). */
export async function observeTreeMegabytesV1(pid:number,run:HostCommandRunnerV1=runHostCommandV1,platform:HostSampleV1["platform"]=osPlatform() as HostSampleV1["platform"],proc:Readonly<{read:HostTextReaderV1;list:()=>Promise<readonly string[]>}>={read:readHostTextV1,list:()=>readdir("/proc")},notify?:(line:string)=>void,measure:HostMemoryMeasureV1="footprint"):Promise<number|null>{
 const reason=(error:unknown):string=>String((error as Error)?.message??error).split("\n")[0]!.slice(0,200),fallback=(why:string):void=>noticeOnceV1("footprintFallback."+platform,"footprintFallback",{platform,reason:why},notify);
 if(platform==="win32"){
  const table=async(source:Readonly<{command:string;arguments:readonly string[]}>,strict:boolean):Promise<number|null>=>{const rows=parseProcessTableV1(await run(source.command,source.arguments));if(strict&&!rows.length)throw Error("empty process table");return treeMegabytesV1(rows,pid);};
  if(measure==="footprint")try{return await table(OBSERVERS.platforms.win32.footprint,true);}catch(error){fallback(reason(error));}
  try{return await table(OBSERVERS.platforms.win32.resident,false);}
  catch(error){noticeOnceV1("treeUnobserved.win32","treeUnobserved",{platform,reason:reason(error)},notify);return null;}
 }
 if(platform==="linux"){
  const rows=await readLinuxProcessTableV1(proc.read,proc.list);
  if(measure==="resident")return treeMegabytesV1(rows,pid);
  const members=treePidsV1(rows,pid);if(!members.length)return null;
  const linux=OBSERVERS.platforms.linux,resident=new Map(rows.map(row=>[row.pid,row.rssKilobytes] as const));let unread=0;
  const kilobytes=await Promise.all(members.map(async member=>{try{const text=await proc.read(`${linux.processDirectory}/${member}/${linux.footprint.file}`),value=text===null?undefined:parseSmapsRollupV1(text,linux.footprint.fields);if(value!==undefined)return value;}catch{}unread++;return resident.get(member)??0;}));
  if(unread===members.length)fallback(`${linux.footprint.file} unreadable`);
  return Math.floor(kilobytes.reduce((total,value)=>total+value,0)/1024);
 }
 const table=OBSERVERS.platforms.darwin.table,rows=parseProcessTableV1(await run(table.command,table.arguments));
 if(measure==="resident")return treeMegabytesV1(rows,pid);
 const members=treePidsV1(rows,pid);if(!members.length)return null;
 const tool=OBSERVERS.platforms.darwin.footprint;let failure="footprint printed no process";
 const text=await run(tool.command,[...tool.arguments,...members.flatMap(member=>[tool.pidFlag,String(member)])]).catch((error:unknown)=>{failure=reason(error);const partial=(error as {stdout?:unknown})?.stdout;return typeof partial==="string"?partial:"";}),bytes=parseFootprintV1(text);
 if(!bytes.size){fallback(failure);return treeMegabytesV1(rows,pid);}
 const resident=new Map(rows.map(row=>[row.pid,row.rssKilobytes*1024] as const));
 return Math.floor(members.reduce((total,member)=>total+(bytes.get(member)??resident.get(member)??0),0)/MEGABYTE);
}
