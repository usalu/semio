import re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R88/st1/"
def rd(n):return open(S+n,encoding="utf8").read()
def wr(n,t):open(S+n,"w",encoding="utf8").write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:70],t.count(old));return t.replace(old,new)
# 3 schema
t=rd("3.base")
t=re.sub(r'        "snapshot[A-Za-z]+": [0-9]+,\n','',t)
t=sub(t,'        "progressMilliseconds": 5000\n','        "progressMilliseconds": 5000,\n        "censusChildren": 8,\n        "censusTimeoutMilliseconds": 5000\n')
t=sub(t,'''        "dropped": { "type": "array", "items": { "type": "string" } },
        "snapshots": { "type": "array", "items": { "type": "object" } }
''','''        "dropped": { "type": "array", "items": { "type": "string" } },
        "guard": { "description": "EN: The number the plan guard compares with its limit, with its parts (the managed bytes of the process: heapUsed plus external; children, resident pages and mapped files are not in it). DE: Die Zahl, die die Plan-Wache mit ihrem Limit vergleicht, mit ihren Teilen (die verwalteten Bytes des Prozesses: heapUsed plus external; Kindprozesse, residente Seiten und abgebildete Dateien gehören nicht dazu).", "type": "object", "additionalProperties": false, "required": ["measure", "megabytes", "heapUsed", "external"], "properties": { "measure": { "const": "heapUsed+external" }, "megabytes": { "type": "number" }, "heapUsed": { "type": "number" }, "external": { "type": "number" } } },
        "footprintMegabytes": { "description": "EN: The physical footprint of this process (macOS phys_footprint, Linux smaps_rollup), measured outside the report tick. DE: Der physische Fußabdruck dieses Prozesses (macOS phys_footprint, Linux smaps_rollup), außerhalb des Berichtstakts gemessen.", "type": "number" },
        "treeFootprintMegabytes": { "description": "EN: The physical footprint of this process and its children together (what the slice ceiling gate judges); only while children exist. DE: Der physische Fußabdruck dieses Prozesses und seiner Kinder zusammen (was die Slice-Obergrenze beurteilt); nur solange Kinder existieren.", "type": "number" },
        "processes": { "description": "EN: The process table of this process and its descendants (ps): the resident and virtual megabytes of the process itself, the count and summed resident megabytes of its children and the largest of them with their roles. DE: Die Prozesstabelle dieses Prozesses und seiner Nachfahren (ps): residente und virtuelle Megabyte des Prozesses selbst, Anzahl und summierte residente Megabyte der Kinder und die größten davon mit ihren Rollen.", "type": "object", "additionalProperties": false, "required": ["count", "rssMegabytes", "children"], "properties": { "self": { "type": "object", "additionalProperties": false, "required": ["rssMegabytes", "vszMegabytes"], "properties": { "rssMegabytes": { "type": "number" }, "vszMegabytes": { "type": "number" } } }, "count": { "type": "integer", "minimum": 0 }, "rssMegabytes": { "type": "number" }, "children": { "type": "array", "items": { "type": "object", "additionalProperties": false, "required": ["pid", "role", "rssMegabytes", "vszMegabytes"], "properties": { "pid": { "type": "integer" }, "role": { "type": "string" }, "rssMegabytes": { "type": "number" }, "vszMegabytes": { "type": "number" } } } } } }
''')
wr("3.new",t)
# 2 d.mts
t=rd("2.base")
t=sub(t,"intervalMilliseconds:number;topTypes:number;stopLines:number;progressMilliseconds:number}>","intervalMilliseconds:number;topTypes:number;stopLines:number;progressMilliseconds:number;censusChildren:number;censusTimeoutMilliseconds:number}>")
t=sub(t,"export function startSliceReportV1(input:{root:string;heap?:()=>unknown;snapshot?:()=>unknown;intervalMilliseconds?:number})","export function startSliceReportV1(input:{root:string;heap?:()=>unknown;footprint?:SliceFootprintV1;table?:()=>Promise<string>;intervalMilliseconds?:number})")
t=sub(t,"export function summarizeHeapSnapshotV1(snapshot:unknown,policy?:unknown):Record<string,unknown>;\n","""export type SliceFootprintV1={self?:()=>Promise<number|undefined>|number|undefined;tree?:()=>Promise<number|undefined>|number|undefined};
export type SliceProcessesV1={self?:{rssMegabytes:number;vszMegabytes:number};count:number;rssMegabytes:number;children:{pid:number;role:string;rssMegabytes:number;vszMegabytes:number}[]};
export function sharedBytesV1(value:unknown,seen?:Set<unknown>):number;
export function processRoleV1(command:string):string;
export function parseProcessTreeV1(text:string,root:number,top?:number):SliceProcessesV1;
export function measureSliceProcessesV1(input?:{footprint?:SliceFootprintV1;table?:()=>Promise<string>;pid?:number}):Promise<{processes?:SliceProcessesV1;footprintMegabytes?:number;treeFootprintMegabytes?:number}>;
""")
wr("2.new",t)
# 5 worker
t=rd("5.new")
t=sub(t,'const reporter=report.startSliceReportV1({root:workerData.root,snapshot:()=>(globalThis as any).Bun.generateHeapSnapshot(),heap:','const{readProcessFootprintBytesV1,readTreeFootprintBytesV1}=await import("../../../../🧠️memory/🦶️footprint/🟨️.mjs");\nconst reporter=report.startSliceReportV1({root:workerData.root,footprint:{self:()=>readProcessFootprintBytesV1(process.pid),tree:()=>readTreeFootprintBytesV1(process.pid)},heap:')
wr("5.new",t)
# 6 script-inputs
t=rd("6.new")
t=sub(t,'import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";\n','import { createBoundedCacheV1 } from "../../../🧊️bounded/🟨️.mjs";\nimport { sharedBytesV1 } from "../../../🧠️memory/📋️slice-report/🟨️.mjs";\n')
old='const part = key => ({ name: "closure:entries." + key, ...spread(closureEntriesV1, entry => key === "answers" ? [...entry.answers ?? []] : entry[key]) }), real = [part("value"), part("rows"), part("digests"), part("sidecar"), part("answers"), part("recipe"), part("layers"), { name: "closure:walkFacts (real)", ...spread(walkFactsByDigestV1, fact => fact) }, { name: "closure:walkMembers (real)", ...spread(walkMembersByDigestV1, list => [...list]) },'
new='const part = key => ({ name: "closure:entries." + key, ...spread(closureEntriesV1, entry => key === "answers" ? [...entry.answers ?? []] : entry[key]) }), once = (name, cache, pick) => { const seen = new Set(); let bytes = 0, entries = 0; for (const key of cache.keys()) { const value = cache.peek(key); if (value === undefined) continue; entries++; bytes += 8 + sharedBytesV1(pick(value), seen); } return { name, entries, bytes }; }, real = [part("value"), part("rows"), once("closure:entries.digests", closureEntriesV1, entry => entry.digests), once("closure:entries.sidecar", closureEntriesV1, entry => entry.sidecar), part("answers"), part("recipe"), part("layers"), { name: "closure:walkFacts (real)", ...spread(walkFactsByDigestV1, fact => fact) }, once("closure:walkMembers (real)", walkMembersByDigestV1, list => list),'
t=sub(t,old,new)
wr("6.new",t)
