"""[walk-g] writes edits.json of the batch `footprint` (each `old` must occur exactly once in the live file). usage: make_edits.py (from anywhere)."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 10)))
H = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
SCHEMA = H + "🧬️schema/🔣️.json"
SAMPLING = H + "📡️sampling/🟦️.ts"
DECISION = H + "📏️decision/🟦️.ts"
ESTIMATES = H + "📐️estimates/🟦️.ts"
WINDOWS = H + "🧪️tests/🪟️windows/🟦️.ts"
LINUX = H + "🧪️tests/📡️sampling/🐧️linux/🟦️.ts"
GUIDE = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"

PS_TABLE = "Get-CimInstance Win32_Process|ForEach-Object{'{0} {1} {2}' -f $_.ProcessId,$_.ParentProcessId,[int64]($_.WorkingSetSize/1024)}"
PRIVATE_TABLE = "Get-CimInstance Win32_PerfFormattedData_PerfProc_Process|ForEach-Object{'{0} {1} {2}' -f $_.IDProcess,$_.CreatingProcessID,[int64]($_.WorkingSetPrivate/1024)}"
PS = ["-NoProfile", "-NonInteractive", "-Command"]

observers = {
    "description": "How each platform reads the memory a process tree holds. The measure is the footprint: what the processes hold privately plus what was swapped or compressed out of them (macOS phys_footprint, Linux proportional set size plus swapped proportional set size, Windows private working set). The resident set stays at the high-water mark of a process after it freed memory on macOS and does not count compressed pages at all, so it is the fallback of a platform whose footprint source cannot be read, never the first choice. The store of learned peaks is stamped with the measure, so peaks of another measure never mix with it.",
    "const": {
        "measure": "footprint",
        "platforms": {
            "darwin": {
                "table": {"command": "ps", "arguments": ["-A", "-o", "pid=,ppid=,rss="]},
                "footprint": {"command": "footprint", "arguments": ["-f", "bytes", "--noCategories"], "pidFlag": "-p"},
            },
            "linux": {
                "processDirectory": "/proc",
                "footprint": {"file": "smaps_rollup", "fields": ["Pss", "SwapPss"]},
                "resident": {"file": "status", "field": "VmRSS"},
            },
            "win32": {
                "footprint": {"command": "powershell.exe", "arguments": PS + [PRIVATE_TABLE]},
                "resident": {"command": "powershell.exe", "arguments": PS + [PS_TABLE]},
            },
        },
    },
}


def block(indent, value):
    text = json.dumps(value, ensure_ascii=False, indent=2)
    return "\n".join((" " * indent + line) if index else line for index, line in enumerate(text.split("\n")))


edits = []


def edit(file, name, old, new):
    edits.append({"file": file, "name": name, "old": old, "new": new})


edit(SCHEMA, "observers", '  "definitions": {\n    "Protocol": {', '  "definitions": {\n    "Observers": ' + block(4, observers) + ',\n    "Protocol": {')
edit(SCHEMA, "message required", '        "treeUnobserved",\n        "countersPartial",\n', '        "treeUnobserved",\n        "footprintFallback",\n        "countersPartial",\n')
edit(SCHEMA, "message property", '        "treeUnobserved": {\n          "type": "string"\n        },\n', '        "treeUnobserved": {\n          "type": "string"\n        },\n        "footprintFallback": {\n          "type": "string"\n        },\n')
edit(SCHEMA, "message en", '          "countersPartial": "[capacity] the {platform} memory counters',
     '          "footprintFallback": "[capacity] the memory footprint of running process trees cannot be read on {platform} ({reason}); the resident set is used instead, which can overstate what a process holds",\n          "countersPartial": "[capacity] the {platform} memory counters')
edit(SCHEMA, "message de", '          "countersPartial": "[capacity] Die Speicherzaehler von {platform}',
     '          "footprintFallback": "[capacity] Der Speicher-Fussabdruck laufender Prozessbaeume kann unter {platform} nicht gelesen werden ({reason}); stattdessen gilt der residente Speicher, der ueberschaetzen kann, was ein Prozess haelt",\n          "countersPartial": "[capacity] Die Speicherzaehler von {platform}')

edit(DECISION, "message names", '|"treeUnobserved"|"countersPartial",values', '|"treeUnobserved"|"footprintFallback"|"countersPartial",values')

edit(SAMPLING, "schema import",
     'import {formatHostCapacityV1,hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";\n',
     'import {formatHostCapacityV1,hostCapacityErrorV1,type HostCapacityPolicyV1,type HostSampleV1} from "../📏️decision/🟦️.ts";\nimport schema from "../🧬️schema/🔣️.json" with {type:"json"};\n')
edit(SAMPLING, "observers constant",
     'const OPTIONAL_PROBE_MILLISECONDS=3000,MEGABYTE=1048576,',
     'const OBSERVERS=schema.definitions.Observers.const,OPTIONAL_PROBE_MILLISECONDS=3000,MEGABYTE=1048576,')
edit(SAMPLING, "notice names", 'message:"treeUnobserved"|"countersPartial",values', 'message:"treeUnobserved"|"footprintFallback"|"countersPartial",values')
edit(SAMPLING, "windows command",
     'export const WINDOWS_PROCESS_TABLE_COMMAND="' + PS_TABLE + '";',
     'export const WINDOWS_PROCESS_TABLE_COMMAND=OBSERVERS.platforms.win32.resident.arguments[3]!;')

OLD_TREE = '''/** 🌲️ Sums the resident megabytes of a process and all of its descendants. */
export function treeMegabytesV1(rows:readonly Readonly<{pid:number;parent:number;rssKilobytes:number}>[],root:number):number|null{
 if(!rows.some(row=>row.pid===root))return null;
 const children=new Map<number,number[]>();for(const row of rows){const list=children.get(row.parent)??[];list.push(row.pid);children.set(row.parent,list);}
 const kilobytes=new Map(rows.map(row=>[row.pid,row.rssKilobytes] as const)),seen=new Set<number>(),pending=[root];let total=0;
 for(let pid=pending.pop();pid!==undefined;pid=pending.pop()){if(seen.has(pid))continue;seen.add(pid);total+=kilobytes.get(pid)??0;pending.push(...children.get(pid)??[]);}
 return Math.floor(total/1024);
}
'''
NEW_TREE = '''/** 🌿️ The pids of a process and all of its descendants, root first; empty when the process is not in the table. */
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
'''
edit(SAMPLING, "tree sum", OLD_TREE, NEW_TREE)

OLD_OBSERVE_HEAD = '''/** 👁️ Observes what the process tree under one pid holds: `/proc` on Linux (no external tool), `ps` on macOS, one PowerShell process table on Windows; null when the process is gone or, on Windows, when PowerShell cannot answer, which is announced once in every language (the gate then learns nothing from the run and keeps the declared sizes). */
export async function observeTreeMegabytesV1(pid:number,run:HostCommandRunnerV1=runHostCommandV1,platform:HostSampleV1["platform"]=osPlatform() as HostSampleV1["platform"],proc:Readonly<{read:HostTextReaderV1;list:()=>Promise<readonly string[]>}>={read:readHostTextV1,list:()=>readdir("/proc")},notify?:(line:string)=>void):Promise<number|null>{
 if(platform==="win32"){
  try{return treeMegabytesV1(parseProcessTableV1(await run("powershell.exe",["-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_TABLE_COMMAND])),pid);}
  catch(error){noticeOnceV1("treeUnobserved.win32","treeUnobserved",{platform,reason:String((error as Error)?.message??error).split("\\n")[0]!.slice(0,200)},notify);return null;}
 }
 if(platform==="linux")return treeMegabytesV1(await readLinuxProcessTableV1(proc.read,proc.list),pid);
 return treeMegabytesV1(parseProcessTableV1(await run("ps",["-A","-o","pid=,ppid=,rss="])),pid);
}'''
NEW_OBSERVE = '''/** 🔬️ The memory measure a process tree is observed in: the footprint is what the processes hold privately plus what was swapped or compressed out of them (macOS `phys_footprint`, Linux proportional set size plus swapped proportional set size, Windows private working set), the resident set is what their pages occupy right now; macOS keeps a process at its resident high-water mark after it freed memory and does not count compressed pages at all, so only the footprint prices a process by what it really holds (`Observers` of the schema). */
export type HostMemoryMeasureV1="footprint"|"resident";

/** 🍏️ Parses `footprint -f bytes --noCategories -p …` into bytes by pid: the kernel's `phys_footprint` of a block where it prints one, the footprint of the block header otherwise; the summary, the lines of a process that vanished and the arguments of a refused one carry no pid and are skipped. */
export function parseFootprintV1(text:string):ReadonlyMap<number,number>{
 const found=new Map<number,number>();let pid:number|undefined;
 for(const line of text.split("\\n")){
  const head=/^.*\\[(\\d+)\\]:[^\\[]*\\bFootprint:\\s+(\\d+)\\s+B\\b/u.exec(line);
  if(head){pid=Number(head[1]);found.set(pid,Number(head[2]));continue;}
  const kernel=/^\\s*phys_footprint:\\s+(\\d+)\\s+B\\s*$/u.exec(line);
  if(kernel&&pid!==undefined)found.set(pid,Number(kernel[1]));
 }
 return found;
}

/** 🐾️ Parses `/proc/<pid>/smaps_rollup` into the kilobytes of the footprint fields (proportional set size plus swapped proportional set size); undefined when the first field is absent. */
export function parseSmapsRollupV1(text:string,fields:readonly string[]=OBSERVERS.platforms.linux.footprint.fields):number|undefined{
 let total=0;
 for(const [index,field] of fields.entries()){const match=new RegExp(`^${field}:\\\\s+(\\\\d+)\\\\s+kB\\\\s*$`,"mu").exec(text);if(match)total+=Number(match[1]);else if(index===0)return undefined;}
 return total;
}

/** 👁️ Observes what the process tree under one pid holds, in the footprint by default (`HostMemoryMeasureV1`): `footprint` on macOS (one call for the whole tree, the process table names its members), `/proc/<pid>/smaps_rollup` on Linux (no external tool), one PowerShell table of private working sets on Windows; a platform whose footprint source cannot be read says so once in every language and reads the resident set instead. null when the process is gone or, on Windows, when PowerShell cannot answer at all, which is announced once in every language (the gate then learns nothing from the run and keeps the declared sizes). */
export async function observeTreeMegabytesV1(pid:number,run:HostCommandRunnerV1=runHostCommandV1,platform:HostSampleV1["platform"]=osPlatform() as HostSampleV1["platform"],proc:Readonly<{read:HostTextReaderV1;list:()=>Promise<readonly string[]>}>={read:readHostTextV1,list:()=>readdir("/proc")},notify?:(line:string)=>void,measure:HostMemoryMeasureV1="footprint"):Promise<number|null>{
 const reason=(error:unknown):string=>String((error as Error)?.message??error).split("\\n")[0]!.slice(0,200),fallback=(why:string):void=>noticeOnceV1("footprintFallback."+platform,"footprintFallback",{platform,reason:why},notify);
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
}'''
edit(SAMPLING, "observe", OLD_OBSERVE_HEAD, NEW_OBSERVE)

edit(ESTIMATES, "read stamp",
     '  return isHostCapacityV1("EstimateStore",value)?value as HostEstimateStoreV1:emptyHostEstimateStoreV1();',
     '  return isHostCapacityV1("EstimateStore",value)&&(value as {measure?:string}).measure===schema.definitions.Observers.const.measure?value as HostEstimateStoreV1:emptyHostEstimateStoreV1();')
edit(ESTIMATES, "write stamp", 'text=JSON.stringify(next),temporary=', 'text=JSON.stringify({...next,measure:schema.definitions.Observers.const.measure}),temporary=')
# ---- existing laws: the resident measure keeps its own tests
edit(WINDOWS, "resident table", 'expect(await observeTreeMegabytesV1(Number(pid),run,"win32",undefined,()=>{throw Error("no notice expected");})).toBe(megabytes);', 'expect(await observeTreeMegabytesV1(Number(pid),run,"win32",undefined,()=>{throw Error("no notice expected");},"resident")).toBe(megabytes);')
edit(WINDOWS, "resident blind", 'expect(await observeTreeMegabytesV1(100,async()=>{throw Error("powershell.exe timed out");},"win32",undefined,line=>{lines.push(line);})).toBeNull();', 'expect(await observeTreeMegabytesV1(100,async()=>{throw Error("powershell.exe timed out");},"win32",undefined,line=>{lines.push(line);},"resident")).toBeNull();')
edit(WINDOWS, "resident once", 'await observeTreeMegabytesV1(100,async()=>{throw Error("again");},"win32");', 'await observeTreeMegabytesV1(100,async()=>{throw Error("again");},"win32",undefined,undefined,"resident");')
edit(LINUX, "resident tree", 'expect(await observeTreeMegabytesV1(fixture.tree.root,run,"linux",{read,list})).toBe(fixture.tree.megabytes);', 'expect(await observeTreeMegabytesV1(fixture.tree.root,run,"linux",{read,list},undefined,"resident")).toBe(fixture.tree.megabytes);')
edit(LINUX, "resident other platform", 'expect(await observeTreeMegabytesV1(fixture.tree.root,run,"win32",{read,list})).toBeNull();', 'expect(await observeTreeMegabytesV1(fixture.tree.root,run,"win32",{read,list},undefined,"resident")).toBeNull();')
edit(LINUX, "resident live", 'const observed=await observeTreeMegabytesV1(process.pid),ps=', 'const observed=await observeTreeMegabytesV1(process.pid,undefined,undefined,undefined,undefined,"resident"),ps=')


edit(WINDOWS, "blind lines", 'expect(blind.length,"one line per language, once").toBe(2);', 'expect(blind.length,"the footprint fallback and the blind process table, each once in every language").toBe(4);expect(blind[0]).toContain("footprint");expect(blind[2]).toContain("cannot be observed");')

edit(GUIDE, "guide en", 'The state is `estimates.json` in the host-capacity cache directory (', 'What a run peaked at is its memory FOOTPRINT (macOS `phys_footprint`, Linux proportional set size plus swapped, Windows private working set), not its resident set, which on macOS stays at the high-water mark after memory was freed; the sample falls back to the resident set once, in both languages, where the footprint cannot be read. The state is `estimates.json`, stamped `measure: footprint` (a store of another measure is empty and rewritten), in the host-capacity cache directory (')
edit(GUIDE, "guide de", 'Der Zustand ist `estimates.json` im Host-Capacity-Cache-Verzeichnis (', 'Was ein Lauf erreicht hat, ist sein Speicher-FUSSABDRUCK (macOS `phys_footprint`, Linux proportionale Belegung plus ausgelagerte, Windows privates Working Set), nicht sein residenter Speicher, der unter macOS nach dem Freigeben auf dem Höchststand bleibt; wo der Fußabdruck nicht gelesen werden kann, fällt die Messung einmal und in beiden Sprachen auf den residenten Speicher zurück. Der Zustand ist `estimates.json`, mit `measure: footprint` gestempelt (ein Speicher anderer Messart ist leer und wird neu geschrieben), im Host-Capacity-Cache-Verzeichnis (')

edit(SCHEMA, "store property", '        "version": {\n          "const": 1\n        },\n        "classes": {\n          "type": "object",\n          "maxProperties": 4096,', '        "version": {\n          "const": 1\n        },\n        "measure": {\n          "const": "footprint"\n        },\n        "classes": {\n          "type": "object",\n          "maxProperties": 4096,')

README = "🧰️framework/🛍️products/🦑️repo/README.md"
edit(README, "readme en", "- **Early cutoff of derived facts and data sections (graph generations).**", "- **The capacity gate learns footprints, not resident sets (host capacity).** What a run peaked at is the memory FOOTPRINT of its process tree: macOS `phys_footprint` (one `footprint -f bytes --noCategories -p …` call for the whole tree, 60-70 ms for a 4.4 GB process), Linux proportional set size plus swapped proportional set size (`/proc/<pid>/smaps_rollup`, no external tool), Windows private working set (one PowerShell table); the observers are data (`Observers` of the host-capacity schema) and a source that cannot be read says so once in every language and falls back to the resident set. macOS keeps a process at its resident high-water mark after it freed memory (measured: a Bun child that dropped 3 million objects holds 411 MB resident and 10 MB of footprint) and does not count compressed pages at all (a graph builder of 4.6 GB footprint showed 2.1 GB resident), so the resident set priced processes wrongly in both directions. The store of learned peaks is stamped `measure: footprint`; a store of another measure reads as empty. Oracles: a Python parser of the recorded and authored tool outputs and, on macOS, the kernel's own `proc_pid_rusage` read by a child through ctypes.\n\n" + "- **Early cutoff of derived facts and data sections (graph generations).**")
edit(README, "readme de", "- **Frühzeitiger Abbruch bei abgeleiteten Fakten und Datenabschnitten (Graph-Generationen).**", '- **Das Kapazitäts-Tor lernt Fußabdrücke, keine residenten Mengen (Host-Kapazität).** Was ein Lauf erreicht hat, ist der Speicher-FUSSABDRUCK seines Prozessbaums: macOS `phys_footprint` (ein Aufruf `footprint -f bytes --noCategories -p …` für den ganzen Baum, 60-70 ms für einen 4,4-GB-Prozess), Linux proportionale Belegung plus ausgelagerte proportionale Belegung (`/proc/<pid>/smaps_rollup`, ohne externes Werkzeug), Windows privates Working Set (eine PowerShell-Tabelle); die Beobachter sind Daten (`Observers` im Host-Capacity-Schema), und eine Quelle, die nicht gelesen werden kann, sagt es einmal in jeder Sprache und fällt auf den residenten Speicher zurück. macOS lässt einen Prozess nach dem Freigeben auf seinem residenten Höchststand (gemessen: ein Bun-Kind, das 3 Millionen Objekte verwarf, hält 411 MB resident und 10 MB Fußabdruck) und zählt komprimierte Seiten gar nicht (ein Graph-Builder mit 4,6 GB Fußabdruck zeigte 2,1 GB resident); der residente Speicher bewertete Prozesse also in beide Richtungen falsch. Der Speicher der gelernten Gipfel ist mit `measure: footprint` gestempelt; ein Speicher anderer Messart liest sich als leer. Orakel: ein Python-Parser der aufgezeichneten und verfassten Werkzeugausgaben und, auf macOS, das eigene `proc_pid_rusage` des Kernels, von einem Kind über ctypes gelesen.\n\n' + "- **Frühzeitiger Abbruch bei abgeleiteten Fakten und Datenabschnitten (Graph-Generationen).**")

json.dump({"edits": edits}, open(os.path.join(HERE, "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
for e in edits:
    text = open(os.path.join(ROOT, e["file"]), encoding="utf8").read()
    n = text.count(e["old"])
    if n != 1:
        sys.exit(f"anchor {e['name']!r}: {n} occurrences in {e['file']}")
print("edits", len(edits))
