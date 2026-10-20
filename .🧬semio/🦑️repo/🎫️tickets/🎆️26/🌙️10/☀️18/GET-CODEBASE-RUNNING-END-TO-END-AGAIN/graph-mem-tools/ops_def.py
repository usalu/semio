"""Defines the staged operations of graph-mem and writes staged/ops.json (apply.py) and staged/bench.json (the harness patches): every anchor is exact live text that must occur exactly once."""
import json,pathlib,sys
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE
while not (ROOT/".git").exists():ROOT=ROOT.parent
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B=f"{L}/🏗️build"
G=f"{B}/🕸️graph"
OBS=f"{B}/👁️observation/🟨️.mjs"
NODES=f"{G}/📥️inference/📦️project-nodes/🟨️.mjs"
INVOCATION=f"{G}/📥️inference/🚦️invocation/🟨️.mjs"
LOAD=f"{G}/🕰️generations/🔌️load/🟨️.mjs"
ENGINE=f"{G}/🕰️generations/🟨️.mjs"
GRAPH=f"{G}/🟦️.ts"
MEMORY=f"{B}/🧠️memory"
edits=[]
def edit(file,anchor,replacement):edits.append({"file":file,"anchor":anchor,"replacement":replacement})

# ── observation: shared strings, no retained image of a persisted project ─────────────────────────────────────────────
edit(OBS,'import { createHash } from "node:crypto";\n','import { createHash } from "node:crypto";\nimport { GRAPH_MEMORY_POLICY_V1, graphStringInternerV1, internJsonStringsV1 } from "../🧠️memory/🟨️.mjs";\n')
edit(OBS,' if(previous?.identity===identity&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){\n  const value=JSON.parse(previous.image.toString("utf8"));',
' if(previous?.identity===identity&&previous.image!==null&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){\n  const value=internJsonStringsV1(JSON.parse(previous.image.toString("utf8")),graphStringInternerV1);')
edit(OBS," projectImages.set(cachePath,{identity,image:Buffer.from(JSON.stringify(value)),observations:publication.observations,publication,headerDigest:usedInference.get(cachePath)??null});\n return value;",
" retainProjectImageV1(cachePath,{identity,image:Buffer.from(JSON.stringify(value)),observations:publication.observations,publication,headerDigest:usedInference.get(cachePath)??null});\n return internJsonStringsV1(value,graphStringInternerV1);")
edit(OBS,"/** 📊️ Exposes owned project computation counts without influencing source admission. */",
"/** 🧊️ Keeps the serialized image of recently inferred projects up to the declared byte bound, oldest first out: a repeated round in this process reuses a retained image, every other repeat reads the persisted record, so the images of thousands of projects never accumulate. */\nconst retainedProjectImages=new Set();let retainedProjectImageBytes=0;\nfunction retainProjectImageV1(cachePath,entry){\n const previous=projectImages.get(cachePath);if(previous?.image)retainedProjectImageBytes-=previous.image.length;\n retainedProjectImages.delete(cachePath);projectImages.set(cachePath,entry);retainedProjectImages.add(cachePath);retainedProjectImageBytes+=entry.image.length;\n while(retainedProjectImageBytes>GRAPH_MEMORY_POLICY_V1.retainedImageBytes&&retainedProjectImages.size){const oldest=retainedProjectImages.values().next().value,held=projectImages.get(oldest);retainedProjectImages.delete(oldest);if(held?.image){retainedProjectImageBytes-=held.image.length;held.image=null;}}\n}\n\n/** 📊️ Exposes owned project computation counts without influencing source admission. */")

edit(OBS,"let projectObservationRows=0;for(const project of projectImages.values())projectObservationRows+=project.observations.length;return {loadObservations:loadObservations.size,projectImages:projectImages.size,",
"let projectObservationRows=0,projectImageBytes=0;for(const project of projectImages.values()){projectObservationRows+=project.observations.length;projectImageBytes+=project.image?.length??0;}return {loadObservations:loadObservations.size,projectImages:projectImages.size,projectImageBytes,")

# ── project nodes: memory governed dispatch ──────────────────────────────────────────────────────────────────────────
edit(NODES,'import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";',
'import { createGraphPhaseClockV1, writeGraphPhaseRowV1, writeGraphSettleRowV1 } from "../../⏱️phases/🟨️.mjs";\nimport { createMemoryGovernorV1, graphMemoryBudgetBytesV1 } from "../../../🧠️memory/🟨️.mjs";')
edit(NODES,'  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length){await inferProject(jobs[cursor++]);await phases?.advance("inference",++finished);}});',
'  const limit=Math.min(projectConcurrencyV1(options),jobs.length),governor=limit>0?createMemoryGovernorV1({budgetBytes:graphMemoryBudgetBytesV1(process.env),workers:limit,signal:context.phaseControl?.signal,report:line=>process.stderr.write(line+"\\n"),environment:process.env}):undefined;\n  const workers=Array.from({length:limit},async()=>{while(!stopped&&cursor<jobs.length){const job=jobs[cursor++];await governor.run(()=>inferProject(job));await phases?.advance("inference",++finished);}});')
edit(NODES,'  await phases?.end("inference",{projects:jobs.length});','  await phases?.end("inference",{projects:jobs.length,...(governor?{memory:governor.stats()}:{})});')

# ── graph: authority list, shared strings of expanded input lists ────────────────────────────────────────────────────
edit(GRAPH,'import{deriveEnvironmentV1,environmentRowHoldsV1}from"./🕰️generations/🌍️environment/🟨️.mjs";','import{deriveEnvironmentV1,environmentRowHoldsV1}from"./🕰️generations/🌍️environment/🟨️.mjs";\nimport{graphStringInternerV1}from"../🧠️memory/🟨️.mjs";')
edit(GRAPH,'new URL("./🧭️file-owner/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyURLV1])',
'new URL("./🧭️file-owner/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧠️memory/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyURLV1])')
edit(GRAPH,'captureSourceInputInventoryV1(file.value,root).value.files.map(file=>"{workspaceRoot}/"+file)','captureSourceInputInventoryV1(file.value,root).value.files.map(file=>graphStringInternerV1.share("{workspaceRoot}/"+file))')

# ── generations: owner row claims by sorted path ranges, shared strings of stored documents ─────────────────────────
edit(LOAD,' const claimed=new Set(),owners=[];\n const claim=(base,roots)=>{\n  const keys=new Set(base.map(rowKeyV1)),rows=[...base],list=prefixes(roots);\n  if(list.length)for(const row of part.rows)if(list.some(entry=>row.path.startsWith(entry))){const key=rowKeyV1(row);if(!keys.has(key)){keys.add(key);rows.push(row);}}\n  for(const key of keys)claimed.add(key);return rows;\n };',
' const claimed=new Set(),owners=[],ordered=part.rows.map((row,at)=>[row.path,at]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:a[1]-b[1]);\n const under=entry=>{let low=0,high=ordered.length;while(low<high){const middle=(low+high)>>>1;if(ordered[middle][0]<entry)low=middle+1;else high=middle;}const found=[];for(;low<ordered.length&&ordered[low][0].startsWith(entry);low++)found.push(ordered[low][1]);return found;};\n const claim=(base,roots)=>{\n  const keys=new Set(base.map(rowKeyV1)),list=prefixes(roots);let rows=base;\n  if(list.length)for(const at of[...new Set(list.flatMap(under))].sort((a,b)=>a-b)){const row=part.rows[at],key=rowKeyV1(row);if(!keys.has(key)){keys.add(key);if(rows===base)rows=[...base];rows.push(row);}}\n  for(const key of keys)claimed.add(key);return rows;\n };')
edit(ENGINE,'import{createProjectFileIndexV1}from"../🧭️file-owner/🟨️.mjs";','import{createProjectFileIndexV1}from"../🧭️file-owner/🟨️.mjs";\nimport{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";')
edit(ENGINE,'  const index=readFactIndexV1(unpack(bytes,P.maximumIndexBytes));stats.indexLoads++;','  const index=readFactIndexV1(internJsonStringsV1(unpack(bytes,P.maximumIndexBytes),graphStringInternerV1));stats.indexLoads++;')
edit(ENGINE,'  return readGenerationFragmentV1(unpack(bytes,P.maximumFragmentBytes));','  return readGenerationFragmentV1(internJsonStringsV1(unpack(bytes,P.maximumFragmentBytes),graphStringInternerV1));')

only=set(sys.argv[1:])
checked=[]
for op in edits:
    path=ROOT/op["file"]
    if not path.exists():raise SystemExit("missing "+op["file"])
    text=path.read_text(encoding="utf8")
    n=text.count(op["anchor"])
    if n!=1 and op["replacement"] not in text:raise SystemExit(f"anchor occurs {n} times in {op['file'][-50:]}: {op['anchor'][:80]!r}")
newFiles=[
 {"file":f"{MEMORY}/🟨️.mjs","from":"staged/new/memory/module/memory.mjs"},
 {"file":f"{MEMORY}/🟨️.d.mts","from":"staged/new/memory/module/memory.d.mts"},
 {"file":f"{MEMORY}/🧬️schema/🔣️.json","from":"staged/new/memory/module/schema.json"},
 {"file":f"{MEMORY}/🧪️tests/🟦️.ts","from":"staged/new/memory/tests/test.ts"},
 {"file":f"{MEMORY}/🧪️tests/🔣️.json","from":"staged/new/memory/tests/fixture.json"},
 {"file":f"{MEMORY}/🧪️tests/🧬️schema/🔣️.json","from":"staged/new/memory/tests/schema.json"},
 {"file":f"{MEMORY}/🧪️tests/🐍️oracle.py","from":"staged/new/memory/tests/oracle.py"},
]
CLAIMS=f"{B}/🧪️tests/🕸️graph/🧲️owner-claims"
RETENTION=f"{B}/👁️observation/🧪️tests/🧠️retention"
SHARED=f"{G}/🕰️generations/🧪️tests/🧶️shared-strings"
WIRING=f"{G}/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch"
newFiles+=[{"file":f"{SHARED}/🟦️.ts","from":"staged/new/shared/test.ts"},{"file":f"{WIRING}/🟦️.ts","from":"staged/new/wiring/test.ts"}]
for directory,name,parts in[(CLAIMS,"claims",[("🟦️.ts","test.ts"),("🔣️.json","fixture.json"),("🧬️schema/🔣️.json","schema.json"),("🐍️oracle.py","oracle.py")]),(RETENTION,"retention",[("🟦️.ts","test.ts"),("🔣️.json","fixture.json"),("🧬️schema/🔣️.json","schema.json")])]:
    newFiles+=[{"file":f"{directory}/{target}","from":f"staged/new/{name}/{source}"} for target,source in parts]
(HERE/"staged").mkdir(exist_ok=True)
(HERE/"staged/ops.json").write_text(json.dumps({"version":1,"edits":edits,"newFiles":newFiles},ensure_ascii=False,indent=1),encoding="utf8")
bench={"edits":{},"newModules":{}}
for op in edits:bench["edits"].setdefault(op["file"],[]).append({"anchor":op["anchor"],"replacement":op["replacement"]})
for item in newFiles:
    if item["file"].endswith("/🧠️memory/🟨️.mjs"):bench["newModules"][item["file"]]=str(HERE/item["from"])
(HERE/"staged/bench.json").write_text(json.dumps(bench,ensure_ascii=False),encoding="utf8")
print(len(edits),"edits",len(newFiles),"new files")
