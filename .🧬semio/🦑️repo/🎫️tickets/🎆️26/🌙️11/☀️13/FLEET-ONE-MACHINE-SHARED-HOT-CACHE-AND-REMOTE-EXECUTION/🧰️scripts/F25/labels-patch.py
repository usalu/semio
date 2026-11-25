import sys,hashlib
R="/Users/ueli/Documents/semio/"
B=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    h0=hashlib.sha256(t.encode()).hexdigest()[:12]
    for old,new in pairs:
        n=t.count(old)
        if n!=1: sys.exit(f"anchor count {n} in {path}: {old[:90]!r}")
        t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t)
    print(path[len(R):][-60:],h0,"->",hashlib.sha256(t.encode()).hexdigest()[:12])

SR=B+"🧠️memory/📋️slice-report/"
edit(SR+"🧬️schema/🔣️.json",[
('"inventory": { "boundMilliseconds": 600000, "en": "checking the file inventory of the tree", "de": "Prüfe das Dateiverzeichnis des Baums" },',
 '"registry-read": { "boundMilliseconds": 120000, "en": "loading the graph modules and reading the owner registry", "de": "Lade die Graphmodule und lese das Projektregister" },\n          "graph-state": { "boundMilliseconds": 600000, "en": "waiting for the graph state lease and checking the graph store", "de": "Warte auf die Graphstatus-Leihe und prüfe den Graphspeicher" },\n          "module-load": { "boundMilliseconds": 120000, "en": "loading the graph inference modules", "de": "Lade die Graph-Ableitungsmodule" },\n          "inventory": { "boundMilliseconds": 600000, "en": "checking the file inventory of the tree (attempt {round})", "de": "Prüfe das Dateiverzeichnis des Baums (Versuch {round})" },'),
('"takeVerified":',
 '"inventoryPhases": {\n          "registry-check": { "en": "checking the owner registry", "de": "Prüfe das Projektregister" },\n          "memo-read": { "en": "reading the admission memo", "de": "Lese das Zulassungsmemo" },\n          "memo-seed": { "en": "seeding owner facts from the memo", "de": "Übernehme Projektfakten aus dem Memo" },\n          "memo-check": { "en": "checking the memo against the tree", "de": "Prüfe das Memo gegen den Baum" },\n          "scoped-discovery": { "en": "scanning the owners", "de": "Durchsuche die Projekte" },\n          "memo-publish": { "en": "publishing the admission memo", "de": "Veröffentliche das Zulassungsmemo" },\n          "catalog": { "en": "listing the workspace files", "de": "Liste die Arbeitsbereichsdateien auf" },\n          "membership-hash": { "en": "hashing the file membership", "de": "Bilde den Hash der Dateimitgliedschaft" }\n        },\n        "takeVerified":'),
])
edit(SR+"🟨️.mjs",[
("""export async function sliceReadStepV1(id,run,detail={},diagnostics){
 const interned""","""export async function sliceReadStepV1(id,run,detail={},diagnostics){
 const end=openSliceReadStepV1(id,detail,diagnostics);
 try{return await run();}finally{end();}
}

/** ⏳️ Runs `run(leave)` as the step `id`, which ends when `run` calls `leave()` or ends, so a wait that is only the first part of a longer body is named while it lasts and no longer.
 * @see ./🧬️schema/🔣️.json */
export async function sliceReadWaitStepV1(id,run,detail={}){
 const end=openSliceReadStepV1(id,detail);
 try{return await run(end);}finally{end();}
}

function openSliceReadStepV1(id,detail,diagnostics){
 const interned"""),
(""" steps.push(row);movedAt=performance.now();
 try{return await run();}finally{const at=steps.lastIndexOf(row);if(at>=0)steps.splice(at,1);movedAt=performance.now();}
}""",""" steps.push(row);movedAt=performance.now();
 return()=>{const at=steps.lastIndexOf(row);if(at>=0){steps.splice(at,1);movedAt=performance.now();}};
}"""),
("""   const note=sample?.note;""","""   const named=sample?.active?.phase;if(typeof named==="string"&&STEPS.inventoryPhases[named]!==undefined)row.phaseName=named;
   const note=sample?.note;"""),
(""" const followed=language=>row.note===undefined?"":" ["+row.note[language]+"]";
 const text=language=>fill(spec[language],fields)+observations(language)""",""" const followed=language=>row.note===undefined?"":" ["+row.note[language]+"]";
 const named=language=>row.phaseName===undefined?"":": "+STEPS.inventoryPhases[row.phaseName][language];
 const text=language=>fill(spec[language],fields)+named(language)+observations(language)"""),
])
L=B+"🕸️graph/📥️load/🟦️.ts"
edit(L,[
("import {describeCollectionV1,registerSliceCollectionsV1,sliceReadStepV1} from","import {describeCollectionV1,registerSliceCollectionsV1,sliceReadStepV1,sliceReadWaitStepV1} from"),
('const observation=observationV1,[producer]=await Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]);',
 'const observation=observationV1,[producer]=await sliceReadStepV1("module-load",()=>Promise.all([import("../📥️inference/🟨️.mjs"),import("../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("../📥️inference/🛡️policy/🟨️.mjs")]));'),
('    const producer=await import("../📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),scope=',
 '    const registry=await sliceReadStepV1("registry-read",async()=>{const producer=await import("../📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);return readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));}),scope='),
('    const graph=await withOwnerRegistryCustodyV1<BuildGraphV1|OwnerRegistryInvalidationV1|undefined>(phaseControl.signal,contain=>withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl,critical)=>{\n',
 '    const graph=await sliceReadWaitStepV1("graph-state",leave=>withOwnerRegistryCustodyV1<BuildGraphV1|OwnerRegistryInvalidationV1|undefined>(phaseControl.signal,contain=>withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl,critical)=>{\n'),
('      checkResourceLeaseControl(actualControl);await store.recheck();const attempt=()=>loadBuildGraphAttemptV1(',
 '      checkResourceLeaseControl(actualControl);await store.recheck();leave();const attempt=()=>loadBuildGraphAttemptV1('),
('    },scope));\n    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);\n    if(graph&&',
 '    },scope)));\n    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);\n    if(graph&&'),
('  const settledInventory=async(orNothing:boolean)','  let inventoryAttempts=0;\n  const settledInventory=async(orNothing:boolean)'),
('lenient?path=>{(volatile as Set<string>).add(join(root,path));}:undefined),{},()=>({...report.read(),...(note&&{note})}));',
 'lenient?path=>{(volatile as Set<string>).add(join(root,path));}:undefined),{round:++inventoryAttempts},()=>({...report.read(),...(note&&{note})}));'),
])
