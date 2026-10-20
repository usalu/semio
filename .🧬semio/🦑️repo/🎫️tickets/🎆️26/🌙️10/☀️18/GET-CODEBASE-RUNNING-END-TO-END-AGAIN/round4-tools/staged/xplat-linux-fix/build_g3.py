import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, read_abs, text_span
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
MG = os.path.join(C, "mem-gov-tools", "staged")
M = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
SAMPLING, GATE = M + "/📡️sampling/🟦️.ts", M + "/🎫️gate/🟦️.ts"
TEST, FIXTURE = M + "/🧪️tests/📡️sampling/🟦️.ts", M + "/🧪️tests/📡️sampling/🧫️fixtures/🔣️.json"
LIVE_ROOT = "/Users/ueli/Documents/semio/"
sampling, gate = read_abs(LIVE_ROOT + SAMPLING), read_abs(LIVE_ROOT + GATE)
test, fixture = read_abs(LIVE_ROOT + TEST), read_abs(LIVE_ROOT + FIXTURE)
edits = []

# --- Linux: no attachable images (partitions are directories), cgroup membership instead of a fixed root path
old = text_span(sampling, "/** 🐧️ Parses Linux", "\nfunction linuxFilesV1", False)
new = old
new = new.replace(", `/proc/sys/fs/file-nr` (allocated, unused, maximum) and `/proc/mounts`, whose loop devices are the attached images.", " and `/proc/sys/fs/file-nr` (allocated, unused, maximum); a Linux host has no attachable images, because its partitions are directories, so the count stays unknown.")
new = new.replace("fileNr:string|null=null,mounts:string|null=null):HostCountersV1{", "fileNr:string|null=null):HostCountersV1{")
new = new.replace("attachedImages:mounts===null?null:mounts.match(/^\\/dev\\/loop\\d+\\s/gmu)?.length??0})", "attachedImages:null})")
assert new != old and "mounts" not in new
edits.append((SAMPLING, old, new))

old = text_span(sampling, ' if(probe.platform==="linux"){', "\n", False)
new = old
for before, after in [
 ("[meminfo,vmstat,pressure,maximum,current,fileNr,mounts]", "[meminfo,vmstat,pressure,fileNr,membership]"),
 ('"/sys/fs/cgroup/memory.max","/sys/fs/cgroup/memory.current","/proc/sys/fs/file-nr","/proc/mounts"', '"/proc/sys/fs/file-nr","/proc/self/cgroup"'),
 ("if(meminfo===null||vmstat===null)", "if(meminfo==null||vmstat==null)"),
 ("return parseLinuxCountersV1(meminfo,vmstat,pressure,maximum,current,fileNr,mounts);", "const limit=effectiveCgroupMemoryV1(await Promise.all(cgroupMemoryFilesV1(membership??null).map(async row=>({maximum:await probe.read(row.maximum),current:await probe.read(row.current)}))));return parseLinuxCountersV1(meminfo,vmstat,pressure??null,limit.maximum,limit.current,fileNr??null);"),
]:
    assert before in new, before
    new = new.replace(before, after)
edits.append((SAMPLING, old, new))

anchor = "/** 🔭️ Turns raw counters, host facts and the previous record into one sample"
added = '''/** 🧬️ Names the cgroup files that bound this process's memory, innermost cgroup first and the root last: `/proc/self/cgroup` rows `N:memory:/path` (v1, also on hybrid hosts, where the memory controller lives) or `0::/path` (v2) give the path; without membership text only the root files are named. */
export function cgroupMemoryFilesV1(membership:string|null):readonly Readonly<{maximum:string;current:string}>[]{
 const lines=(membership??"").split("\\n"),v2=lines.find(line=>line.startsWith("0::")),v1=lines.find(line=>/^[0-9]+:[^:]*\\bmemory\\b[^:]*:/u.test(line));
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
  const match=/^([0-9]+|max)\\s+([0-9]+)\\s*$/u.exec(row??"");if(!match||match[1]==="max"||Number(match[2])<1)continue;
  const cores=Math.max(1,Math.ceil(Number(match[1])/Number(match[2])));if(best===null||cores<best)best=cores;
 }
 return best;
}

/** 🧠️ Counts the cores this process may use: the logical cores, bounded by the tightest cgroup v2 CPU quota above the process on Linux. */
export async function readHostCoresV1(probe:HostProbeV1,logical:number):Promise<number>{
 if(probe.platform!=="linux")return logical;
 const membership=await probe.read("/proc/self/cgroup");if(!(membership??"").split("\\n").some(line=>line.startsWith("0::")))return logical;
 const quota=parseCpuQuotaCoresV1(await Promise.all(cgroupMemoryFilesV1(membership).map(row=>probe.read(row.maximum.replace(/memory\\.max$/u,"cpu.max")))));
 return quota===null?logical:Math.min(logical,quota);
}

'''
edits.append((SAMPLING, anchor, added + anchor))
edits.append((SAMPLING, "const counters=await readHostCountersV1(probe),average=probe.platform===\"win32\"?busyLoadV1():loadavg()[0]!,host=reduceHostSampleV1(policy,{platform:probe.platform,cores:cpus().length||1,",
 "const counters=await readHostCountersV1(probe),cores=await readHostCoresV1(probe,cpus().length||1),average=probe.platform===\"win32\"?busyLoadV1():loadavg()[0]!,host=reduceHostSampleV1(policy,{platform:probe.platform,cores,"))

# --- Linux process tree without the ps binary
old = text_span(sampling, "/** 👁️ Observes what the process tree under one pid holds", "\n}", True)
new = '''/** 🌳️ Parses `/proc/<pid>/status` into a process row: the parent from `PPid:` and the resident set from `VmRSS:` (absent for kernel threads, so 0). */
export function parseProcStatusV1(pid:number,status:string):Readonly<{pid:number;parent:number;rssKilobytes:number}>|undefined{
 const parent=/^PPid:\\s+([0-9]+)\\s*$/mu.exec(status),resident=/^VmRSS:\\s+([0-9]+)\\s+kB\\s*$/mu.exec(status);
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
}'''
edits.append((SAMPLING, old, new))
edits.append((SAMPLING, 'import {readFile} from "node:fs/promises";', 'import {readdir,readFile} from "node:fs/promises";'))

# --- gate directory: XDG_CACHE_HOME counts only when absolute, as the specification says
edits.append((GATE, 'import {join} from "node:path";', 'import {isAbsolute,join} from "node:path";'))
edits.append((GATE, 'environment.XDG_CACHE_HOME??join(home,".cache")', 'isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache")'))

# --- existing sampling test and fixture follow the new Linux signature
edits.append((TEST, "item.fileNr,item.mounts),item.name)", "item.fileNr),item.name)"))
edits.append((TEST, ',["/proc/mounts",fixture.linux[0]!.mounts!]', ''))
for before, after in [
 ('"attachedImages": 2\n   },\n   "fileNr": "20480\\t512\\t1048576\\n",\n   "mounts": ' + text_span(fixture, '"mounts": "/dev/loop3', '\n  },', False)[len('"mounts": '):] + '\n  },', '"attachedImages": null\n   },\n   "fileNr": "20480\\t512\\t1048576\\n"\n  },'),
 ('"attachedImages": 0\n   },\n   "fileNr": "1000 0 2000\\n",\n   "mounts": "proc /proc proc rw 0 0\\n"\n  },', '"attachedImages": null\n   },\n   "fileNr": "1000 0 2000\\n"\n  },'),
 ('"fileNr": null,\n   "mounts": null\n  }', '"fileNr": null\n  }'),
]:
    assert fixture.count(before) == 1, before[:60]
    edits.append((FIXTURE, before, after))
new = [M + "/🧪️tests/📡️sampling/🐧️linux/🟦️.ts", M + "/🧪️tests/📡️sampling/🐧️linux/🧫️fixtures/🔣️.json"]
emit("g3", "host capacity on Linux: no loop-device attach count, cgroup chain for memory and cores, /proc process tree, absolute XDG cache home", [SAMPLING], edits, new, os.path.join(HERE, "groups"))
print("g3 written", len(edits), "edits")
