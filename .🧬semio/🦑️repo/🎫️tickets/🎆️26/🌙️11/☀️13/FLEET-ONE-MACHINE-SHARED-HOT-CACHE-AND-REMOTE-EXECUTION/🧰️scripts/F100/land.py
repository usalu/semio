import os,sys,tempfile
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
G=B+"🕸️graph/"
dry="--dry" in sys.argv
only=[a for a in sys.argv[1:] if not a.startswith("--")]
def edit(path,pairs,group):
    if only and group not in only: return
    text=open(path,encoding="utf8").read()
    for old,new in pairs:
        if text.count(old)!=1: sys.exit("anchor count %d in %s: %r"%(text.count(old),path,old[:80]))
        text=text.replace(old,new)
    if dry: print("ok",group,path[-60:]);return
    fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".f100-");os.write(fd,text.encode("utf8"));os.close(fd);os.replace(tmp,path);print("wrote",group,path[-60:])

RL=G+"🕰️generations/🗃️row-log/🟨️.mjs"
edit(RL,[
('const P=ROW_LOG_POLICY_V1,EXTENSION=".rows",NEWLINE="\\n";',
 'const P=ROW_LOG_POLICY_V1,EXTENSION=".rows",NEWLINE="\\n",liveLogsV1=new Set();'),
('const descriptor=openSync(path,"w+"),table=createKeyTableV1(),exit=()=>{try{rmSync(path,{force:true});}catch{}};',
 'const descriptor=openSync(path,"w+"),exit=()=>{try{rmSync(path,{force:true});}catch{}};let table=createKeyTableV1(),ref;'),
(' const intern=row=>{\n  const key=rowKeyV1(row)',
 ' const intern=row=>{\n  if(table===undefined)throw Error("Writable row log required: the log is sealed");\n  const key=rowKeyV1(row)'),
('  settle(){require();flush();if(offsets.length>count){offsets=offsets.slice(0,count);lengths=lengths.slice(0,count);}scratch=Buffer.allocUnsafe(0);},',
 '  settle(){require();flush();if(offsets.length>count){offsets=offsets.slice(0,count);lengths=lengths.slice(0,count);}scratch=Buffer.allocUnsafe(0);},\n  /** 🔒️ Ends the writes of the log once its part is shaped: the key table (20 bytes for each slot of an open addressed table at half load, 40 to 80 bytes for each distinct row) that only deduplicates rows while they arrive is dropped, and the lists keep reading their rows by id; a later `id`, `append` or `add` of a row is refused. A read of many waves that keeps every wave\'s log until the seal therefore holds only the offsets, the lengths and the ids of a finished wave. */\n  seal(){require();api.settle();if(table!==undefined){table.clear();table=undefined;}},'),
('  stats:()=>({rows:count,bytes:size,reads,readBytes,pendingBytes}),',
 '  stats:()=>({rows:count,bytes:size,reads,readBytes,pendingBytes,tableBytes:table===undefined?0:table.bytes(),indexBytes:offsets.byteLength+lengths.byteLength,sealed:table===undefined}),'),
('process.removeListener("exit",exit);try{closeSync(descriptor);}catch{}if(existsSync(path))rmSync(path,{force:true});table.clear();count=0;},',
 'process.removeListener("exit",exit);liveLogsV1.delete(ref);try{closeSync(descriptor);}catch{}if(existsSync(path))rmSync(path,{force:true});table?.clear();table=undefined;count=0;},'),
(' });\n return api;\n}',
 ' });\n ref=new WeakRef(api);liveLogsV1.add(ref);\n return api;\n}\n\n/** 🧮️ The row logs this process holds open, for the report of retained memory: the key tables of the logs not sealed yet and the offsets, lengths and unwritten text of all of them, with the distinct rows they name. */\nexport function rowLogCollectionsV1(){\n let logs=0,sealed=0,rows=0,tableBytes=0,indexBytes=0;\n for(const held of liveLogsV1){const log=held.deref();if(log===undefined){liveLogsV1.delete(held);continue;}const stats=log.stats();logs++;if(stats.sealed)sealed++;rows+=stats.rows;tableBytes+=stats.tableBytes;indexBytes+=stats.indexBytes+stats.pendingBytes;}\n return[{name:"graph:rowLogs.keyTables ("+(logs-sealed)+" open of "+logs+")",entries:rows,bytes:tableBytes},{name:"graph:rowLogs.offsets ("+logs+" logs, "+sealed+" sealed)",entries:rows,bytes:indexBytes}];\n}'),
],"graph")
edit(RL[:-len("🟨️.mjs")]+"🟨️.d.mts",[
('stats():{rows:number;bytes:number;reads:number;readBytes:number;pendingBytes:number};settle():void;close():void}',
 'stats():{rows:number;bytes:number;reads:number;readBytes:number;pendingBytes:number;tableBytes:number;indexBytes:number;sealed:boolean};settle():void;seal():void;close():void}'),
('/** 🏷️ The file name of the row log one process opens. */',
 '/** 🧮️ The row logs this process holds open as report rows (entries and bytes). */\nexport function rowLogCollectionsV1():{name:string;entries:number;bytes:number}[];\n/** 🏷️ The file name of the row log one process opens. */'),
],"graph")

LOAD=G+"🕰️generations/🔌️load/🟨️.mjs"
edit(LOAD,[
(' const environment=claims.environment(part.env.rows,[...implementation.all,...implementation.environment]);\n',
 ' const environment=claims.environment(part.env.rows,[...implementation.all,...implementation.environment]);\n part.rows?.log?.seal?.();\n'),
],"graph")

SI=G+"📥️inference/🧭️script-inputs/🟨️.mjs"
edit(SI,[
('  return [...real, { name: "typescript:sharedSourceFiles',
 '  return [...real, { name: "closure:verdictSeen (verdict files of finished rounds)", entries: verdictSeenV1.size, bytes: verdictSeenV1.size * 240 }, { name: "typescript:sharedSourceFiles'),
('const forgetVerdictV1 = file =>',
 '/** 🌊️ Forgets the verdicts this process saw in the rounds of the waves that are over: a round has its own directory, so a later wave never asks for them again and each is read from its file when it is. */\nexport function forgetRoundVerdictsV1() { verdictSeenV1.clear(); }\nconst forgetVerdictV1 = file =>'),
],"graph")
edit(SI[:-len("🟨️.mjs")]+"🟨️.d.mts",[
('export function closureCollectionsV1(): { name: string; entries: number; bytes: number }[];',
 'export function closureCollectionsV1(): { name: string; entries: number; bytes: number }[];\n/** 🌊️ Forgets the verdicts of the rounds of finished waves. */\nexport function forgetRoundVerdictsV1(): void;'),
],"graph")

REL=G+"🕊️release/🟦️.ts"
edit(REL,[
('/** 🔨️ Refuses a graph whose content was released. */',
 '/** 🌊️ Drops what one wave of a long read filled and its owners persisted: the interned and verified rows, the project and analysis images, the settled reads, the input memos, the interned strings, the parsed shared source files and the verdicts of the rounds of the wave. The closure entries and the generation carry stay: the next wave needs them.\n * @see ../🧪️tests/🌊️wave-retention */\nexport async function releaseWaveStateV1():Promise<void>{\n  (await import("../../👁️observation/🟨️.mjs")).releaseInferenceMemoryV1();(await import("../../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();graphStringInternerV1.clear();\n  (await import("../📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs")).sharedSourceFilesV1.clear();\n  (await import("../📥️inference/🧭️script-inputs/🟨️.mjs")).forgetRoundVerdictsV1();\n}\n\n\n/** 🔨️ Refuses a graph whose content was released. */'),
],"graph")

HOST=G+"🕰️host/🟦️.ts"
edit(HOST,[
('import {inventoryDirectoriesV1} from "../🕊️release/🟦️.ts";','import {inventoryDirectoriesV1,releaseWaveStateV1} from "../🕊️release/🟦️.ts";'),
('(await import("../📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs")).sharedSourceFilesV1.clear();if(!state.carry&&options?.more!==true){',
 '(await import("../📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs")).sharedSourceFilesV1.clear();(await import("../📥️inference/🧭️script-inputs/🟨️.mjs")).forgetRoundVerdictsV1();if(!state.carry&&options?.more!==true){'),
('const generation=info.generation;await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:async()=>{observationV1.releaseInferenceMemoryV1();graphStringInternerV1.clear();(await import("../📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs")).sharedSourceFilesV1.clear();}});',
 'const generation=info.generation;await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:releaseWaveStateV1});'),
('seams.published(generation);await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:async()=>{observationV1.releaseInferenceMemoryV1();graphStringInternerV1.clear();(await import("../📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs")).sharedSourceFilesV1.clear();}});',
 'seams.published(generation);await announceGraphWaveV1({generation,info});await settleWaveMemoryV1({generation,release:releaseWaveStateV1});'),
],"graph")

GEN=G+"🕰️generations/🟨️.mjs"
edit(GEN,[
('import{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";',
 'import{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";\nimport{registerSliceCollectionsV1}from"../../🧠️memory/📋️slice-report/🟨️.mjs";'),
('export const carriedStagesV1={};',
 'export const carriedStagesV1={};\n\n/** 🧮️ The state a build carries from wave to wave, as rows of the report of retained memory: the owner table with the strings of the edge reads of its owners, the rows each owner is given (ids into the row logs: their bytes are the ids and the rows held beyond them), the names of the returned, dropped and derived owners and the frontier of the membership the waves reuse.\n * @see ./🧪️tests/🌊️wave-retention */\nfunction carryCollectionsV1(carry,wave){\n const owners=carry.owners??{};let reads=0;\n for(const row of Object.values(owners))if(row.reads)for(const list of Object.values(row.reads))if(Array.isArray(list))reads+=list.length;\n const lists=map=>{let bytes=0;for(const list of map.values()){bytes+=list.ids?.byteLength??list.length*160;for(const tail of list.tails??[])bytes+=tail.length*160;}return bytes;};\n const frontier=wave?.live?.frontier,held=(frontier?.members?.length??0)+(frontier?.directories?.length??0),names=(carry.names?.length??0)+carry.derived.size+carry.dropped.size+carry.returned.size;\n return[\n  {name:"generation:carry.owners",entries:Object.keys(owners).length,sample:()=>Object.values(owners).slice(0,16)},\n  {name:"generation:carry.owners.reads (strings)",entries:reads,bytes:reads*120},\n  {name:"generation:carry.upserts (row lists)",entries:carry.upserts.size,bytes:lists(carry.upserts)},\n  {name:"generation:carry.append (row lists)",entries:carry.append.size,bytes:lists(carry.append)},\n  {name:"generation:carry.names",entries:names,bytes:names*120},\n  {name:"generation:wave.frontier (members and directories)",entries:held,bytes:held*320},\n ];\n}'),
('files:[],names:undefined,resumedFrom:false};\n  try{',
 'files:[],names:undefined,resumedFrom:false},forgetCarry=registerSliceCollectionsV1("generation:carry",()=>carryCollectionsV1(carry,wave));\n  try{'),
('  finally{await disposeSources(carry.disposers);}',
 '  finally{forgetCarry();await disposeSources(carry.disposers);}'),
],"graph")

OBS=B+"👁️observation/🟨️.mjs"
edit(OBS,[
('add("verifiedBlocks",verifiedBlocks);',
 'add("verifiedBlocks",verifiedBlocks);add("witnessStrings",witnessStringsV1);add("fileShaMemo",fileShaMemoV1);add("loadedImplementationFacts",loadedImplementationFactsV1);'),
],"observation")

INP=B+"📥️inputs/🟨️.mjs"
edit(INP,[
('CHUNK_CUTS_LIMIT_V1 = limitsSchema.definitions.Limits.const.chunkCutsLimit;',
 'CHUNK_CUTS_LIMIT_V1 = limitsSchema.definitions.Limits.const.chunkCutsLimit, VERIFIED_CLOSURES_LIMIT_V1 = limitsSchema.definitions.Limits.const.verifiedClosureLimit;'),
('VERIFIED_CLOSURES_V1.set(path,{ino:row.ino,size:row.size,mtimeMs:row.mtimeMs,ctimeMs:row.ctimeMs});return true;});',
 'if(VERIFIED_CLOSURES_V1.size>=VERIFIED_CLOSURES_LIMIT_V1)VERIFIED_CLOSURES_V1.clear();VERIFIED_CLOSURES_V1.set(path,{ino:row.ino,size:row.size,mtimeMs:row.mtimeMs,ctimeMs:row.ctimeMs});return true;});'),
('export function releaseInputMemoryV1() { INVENTORY_MEMO_V1.clear(); VERIFIED_CLOSURES_V1.clear(); CHUNK_CUTS_V1.clear(); }',
 'export function releaseInputMemoryV1() { INVENTORY_MEMO_V1.clear(); VERIFIED_CLOSURES_V1.clear(); CHUNK_CUTS_V1.clear(); }\n/** 🧮️ The memos of this module as rows of the report of retained memory. */\nexport function inputCollectionsV1() { return [{ name: "inputs:inventoryMemo", entries: INVENTORY_MEMO_V1.size, sample: () => [...INVENTORY_MEMO_V1.values()].slice(0, 16) }, { name: "inputs:verifiedClosures", entries: VERIFIED_CLOSURES_V1.size, bytes: VERIFIED_CLOSURES_V1.size * 200 }, { name: "inputs:chunkCuts", entries: CHUNK_CUTS_V1.size, sample: () => [...CHUNK_CUTS_V1.values()].slice(0, 16) }]; }'),
],"inputs")
edit(INP[:-len("🟨️.mjs")]+"🟨️.d.mts",[
('export function releaseInputMemoryV1',
 '/** 🧮️ The memos of this module as report rows. */\nexport function inputCollectionsV1():{name:string;entries:number;bytes?:number;sample?:()=>unknown[]}[];\nexport function releaseInputMemoryV1'),
],"inputs")
edit(B+"📥️inputs/🧬️schema/🔣️.json",[
('        "chunkCutsLimit": 131072,\n','        "chunkCutsLimit": 131072,\n        "verifiedClosureLimit": 131072,\n'),
('          "chunkCutsLimit": "The most verified closure stamps the process remembers at once.",\n','          "chunkCutsLimit": "The most verified closure stamps the process remembers at once.",\n          "verifiedClosureLimit": "The most verified closure leaf stamps the process remembers at once; at the maximum the memo starts again.",\n'),
('          "chunkCutsLimit": "Die meisten verifizierten Abschluss-Stempel, die der Prozess gleichzeitig merkt.",\n','          "chunkCutsLimit": "Die meisten verifizierten Abschluss-Stempel, die der Prozess gleichzeitig merkt.",\n          "verifiedClosureLimit": "Die meisten verifizierten Stempel von Abschluss-Blättern, die der Prozess gleichzeitig merkt; am Maximum beginnt der Speicher neu.",\n'),
],"inputs")

SET=B+"🧠️memory/🧽️settle/🟨️.mjs"
edit(SET,[
('  const[{observationCollectionsV1},{graphStringInternerV1},{closureCollectionsV1}]=await Promise.all([import("../../👁️observation/🟨️.mjs"),import("../🟨️.mjs"),import("../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")]);',
 '  const[{observationCollectionsV1},{graphStringInternerV1},{closureCollectionsV1},{rowLogCollectionsV1},{inputCollectionsV1}]=await Promise.all([import("../../👁️observation/🟨️.mjs"),import("../🟨️.mjs"),import("../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"),import("../../🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs"),import("../../📥️inputs/🟨️.mjs")]);'),
('  registerSliceCollectionsV1("closure",()=>closureCollectionsV1());\n',
 '  registerSliceCollectionsV1("closure",()=>closureCollectionsV1());\n  registerSliceCollectionsV1("rowlogs",()=>rowLogCollectionsV1());\n  registerSliceCollectionsV1("inputs",()=>inputCollectionsV1());\n'),
],"memory")
