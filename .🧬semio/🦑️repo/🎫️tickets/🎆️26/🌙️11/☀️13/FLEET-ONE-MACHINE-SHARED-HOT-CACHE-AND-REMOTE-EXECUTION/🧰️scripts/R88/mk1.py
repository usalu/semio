import re,json,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R88/st1/"
def rd(n):return open(S+n,encoding="utf8").read()
def wr(n,t):open(S+n,"w",encoding="utf8").write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:60],t.count(old));return t.replace(old,new)
# 1: slice-report module
t=rd("1.new")
a=t.index("/** 🔬️ What a heap snapshot");b=t.index("/** 💾️ Writes the report atomically")
t=t[:a]+t[b:]
t=sub(t,'import{mkdirSync,renameSync,writeFileSync}from"node:fs";','import{execFile}from"node:child_process";\nimport{mkdirSync,renameSync,writeFileSync}from"node:fs";\nimport{basename}from"node:path";\nimport{platform as osPlatform}from"node:os";')
t=sub(t,'let phase={name:"start",done:0,total:0},stage,stop;','let phase={name:"start",done:0,total:0},stage,stop,measured={};')
new_funcs='''
/** 🧮️ The bytes a value holds when every object, array, set and map is charged ONCE however many referrers it has (`seen` carries the ones already charged): an array or set costs 16 plus 8 per element, an object 16 plus 16 per key, a string only the slot that refers to it (the characters belong to the interner and the row dictionary). The JSON estimate of `estimateCollectionBytesV1` charges a structure shared by thousands of entries once per referrer (263 GB for a sidecar of 100 MB).
 * @see ./🧬️schema/🔣️.json */
export function sharedBytesV1(value,seen=new Set()){
 if(value===null||typeof value!=="object")return 0;
 if(seen.has(value))return 0;
 seen.add(value);
 if(ArrayBuffer.isView(value))return value.byteLength+16;
 let bytes=16;
 if(Array.isArray(value)||value instanceof Set){for(const item of value)bytes+=8+sharedBytesV1(item,seen);return bytes;}
 if(value instanceof Map){for(const[key,item]of value)bytes+=16+sharedBytesV1(key,seen)+sharedBytesV1(item,seen);return bytes;}
 for(const item of Object.values(value))bytes+=16+sharedBytesV1(item,seen);
 return bytes;
}

/** 🏷️ The role of a process by its command line: the base name of the first script it runs, else of its executable. */
export function processRoleV1(command){
 const words=String(command).trim().split(/\\s+/u),script=words.find(word=>/\\.(?:ts|mts|mjs|cjs|js)$/u.test(word));
 return basename(script??words[0]??"?").slice(0,60);
}

/** 🌳️ The process tree below `root` from a `ps -A -o pid=,ppid=,rss=,vsz=,command=` table: the root itself and its descendants with their resident and virtual megabytes and role, the largest `top` of them, the count and the summed resident megabytes of the descendants.
 * @see ./🧬️schema/🔣️.json */
export function parseProcessTreeV1(text,root,top=P.censusChildren){
 const rows=new Map(),children=new Map();
 for(const line of String(text).split("\\n")){
  const match=/^\\s*(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(.*)$/u.exec(line);if(match===null)continue;
  const row={pid:Number(match[1]),parent:Number(match[2]),rssMegabytes:Math.round(Number(match[3])/1024*10)/10,vszMegabytes:Math.round(Number(match[4])/1024*10)/10,role:processRoleV1(match[5])};
  if(/^(?:\\S*\\/)?ps -A -o pid=,ppid=,rss=,vsz=,command=/u.test(match[5]))continue;
  rows.set(row.pid,row);if(!children.has(row.parent))children.set(row.parent,[]);children.get(row.parent).push(row.pid);
 }
 const found=[],queue=[...children.get(root)??[]],visited=new Set([root]);
 while(queue.length>0){const pid=queue.shift();if(visited.has(pid))continue;visited.add(pid);const row=rows.get(pid);if(row!==undefined){found.push(row);queue.push(...children.get(pid)??[]);}}
 const self=rows.get(root);
 return{...self===undefined?{}:{self:{rssMegabytes:self.rssMegabytes,vszMegabytes:self.vszMegabytes}},count:found.length,rssMegabytes:Math.round(found.reduce((total,row)=>total+row.rssMegabytes,0)*10)/10,children:found.sort((a,b)=>b.rssMegabytes-a.rssMegabytes).slice(0,top).map(({pid,role,rssMegabytes,vszMegabytes})=>({pid,role,rssMegabytes,vszMegabytes}))};
}

const psTable=()=>new Promise((resolve,reject)=>execFile("ps",["-A","-o","pid=,ppid=,rss=,vsz=,command="],{maxBuffer:1<<25,timeout:P.censusTimeoutMilliseconds,windowsHide:true},(error,stdout)=>error===null?resolve(stdout):reject(error)));

/** 📏️ What the process holds that `process.memoryUsage()` does not show: its physical footprint (macOS `phys_footprint`), the footprint of its whole tree and the process table of its children with their roles. Every reading is optional (a platform or a command that fails leaves its field out), and runs asynchronously so a report never waits for it.
 * @see ./🧬️schema/🔣️.json */
export async function measureSliceProcessesV1({footprint,table=osPlatform()==="win32"?undefined:psTable,pid=process.pid}={}){
 const out={},megabytes=bytes=>Number.isFinite(bytes)?Math.round(bytes/MEGABYTE):undefined;
 if(table!==undefined)try{out.processes=parseProcessTreeV1(await table(),pid);}catch{}
 try{const own=megabytes(await footprint?.self?.());if(own!==undefined)out.footprintMegabytes=own;}catch{}
 try{if((out.processes?.count??0)>0){const tree=megabytes(await footprint?.tree?.());if(tree!==undefined)out.treeFootprintMegabytes=tree;}}catch{}
 return out;
}
'''
t=sub(t,"/** 🧾️ The report of this moment",new_funcs.lstrip("\n")+"\n/** 🧾️ The report of this moment")
t=sub(t,"const usage=process.memoryUsage(),stats=heap?.()??undefined,body={};","const usage=process.memoryUsage(),stats=heap?.()??undefined,body={};")
t=sub(t,"memory:{rss:megabytes(usage.rss),heapUsed:megabytes(usage.heapUsed),heapTotal:megabytes(usage.heapTotal),external:megabytes(usage.external),arrayBuffers:megabytes(usage.arrayBuffers)},","memory:{rss:megabytes(usage.rss),heapUsed:megabytes(usage.heapUsed),heapTotal:megabytes(usage.heapTotal),external:megabytes(usage.external),arrayBuffers:megabytes(usage.arrayBuffers)},guard:{measure:\"heapUsed+external\",megabytes:megabytes(usage.heapUsed+usage.external),heapUsed:megabytes(usage.heapUsed),external:megabytes(usage.external)},...measured,")
# startSliceReportV1
a=t.index("/** ⏱️ Writes the report now");
t=t[:a]+'''/** ⏱️ Writes the report now and every `intervalMilliseconds` until the returned function is called (which writes the last one); the timer never keeps the process alive. A report never walks the heap (a heap snapshot of a 500 MB heap held 4 GB of resident pages for the rest of the process and made the plan guard see 10 GB): it reads the runtime counters, the process table and, through `footprint` (`{self,tree}`), the physical footprint of the process and of its tree.
 * @see ./🧬️schema/🔣️.json */
export function startSliceReportV1({root,heap,footprint,table,intervalMilliseconds=P.intervalMilliseconds}){
 let collected=false,reading=false;
 const refresh=()=>{if(reading)return;reading=true;measureSliceProcessesV1({footprint,...table===undefined?{}:{table}}).then(row=>{measured=row;},()=>undefined).finally(()=>{reading=false;});};
 refresh();
 writeSliceReportV1(root,{reason:"start",heap});
 const collect=()=>{
  if(collected||process.memoryUsage().heapUsed/MEGABYTE<P.collectMinimumHeapMegabytes)return;
  collected=true;const started=performance.now();
  try{globalThis.Bun?.gc?.(true);}catch{}
  const usage=process.memoryUsage(),stats=heap?.();
  sources.set("afterCollect",()=>({heapUsed:megabytes(usage.heapUsed),external:megabytes(usage.external),seconds:Math.round((performance.now()-started)/100)/10,...stats===undefined?{}:{types:Object.entries(stats.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,P.topTypes).map(([type,count])=>({type,count}))}}));
 };
 const timer=setInterval(()=>{refresh();collect();writeSliceReportV1(root,{reason:"interval",heap});},intervalMilliseconds);timer.unref?.();
 return Object.freeze({now:reason=>writeSliceReportV1(root,{reason,heap}),stop:()=>{clearInterval(timer);writeSliceReportV1(root,{reason:"end",heap});}});
}
'''
wr("1.new",t)
