import sys,hashlib
R="/Users/ueli/Documents/semio/"
B=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
def edit(path,pairs):
    t=open(path,encoding="utf8").read();h0=hashlib.sha256(t.encode()).hexdigest()[:12]
    for old,new in pairs:
        if t.count(old)!=1: sys.exit(f"anchor {t.count(old)} {path[-40:]}: {old[:90]!r}")
        t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t);print(path[-44:],h0,"->",hashlib.sha256(t.encode()).hexdigest()[:12])
SR=B+"🧠️memory/📋️slice-report/"
edit(SR+"🟨️.mjs",[("function openSliceReadStepV1(id,detail,diagnostics){","/** ⏳️ Opens the step `id` and answers the function that ends it (ending twice is harmless); a caller that cannot wrap the whole step in one body uses it and ends the step in a `finally`.\n * @see ./🧬️schema/🔣️.json */\nexport function openSliceReadStepV1(id,detail={},diagnostics){")])
edit(SR+"🟨️.d.mts",[("export function sliceReadStepV1<T>(","export function openSliceReadStepV1(id:string,detail?:Readonly<Record<string,string|number>>,diagnostics?:()=>unknown):()=>void;\nexport function sliceReadWaitStepV1<T>(id:string,run:(leave:()=>void)=>Promise<T>|T,detail?:Readonly<Record<string,string|number>>):Promise<T>;\nexport function sliceReadStepV1<T>(")])
edit(SR+"🧬️schema/🔣️.json",[('"registry-read":','"worker-boot": { "boundMilliseconds": 300000, "en": "starting the slice worker and loading its modules", "de": "Starte den Slice-Worker und lade seine Module" },\n          "slice-prepare": { "boundMilliseconds": 300000, "en": "preparing the slice read (backend, compiler runtime, native recheck)", "de": "Bereite das Lesen des Ausschnitts vor (Backend, Compiler-Laufzeit, native Prüfung)" },\n          "registry-read":')])
W=B+"🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"
EMIT='report.startSliceProgressV1({emit:(event:{phase:string;done:number;total:number})=>{report.noteSliceReportPhaseV1(event);parentPort!.postMessage({type:"progress",...event});}});\n'
edit(W,[
("useProcessWideImplementationPinV1();\n","useProcessWideImplementationPinV1();\nconst report=await import(\"../../../../🧠️memory/📋️slice-report/🟨️.mjs\"),leaveBoot=report.openSliceReadStepV1(\"worker-boot\");\n"+EMIT),
('const report=await import("../../../../🧠️memory/📋️slice-report/🟨️.mjs"),{inferenceCacheSizesV1,observationCollectionsV1}=await import(','const{inferenceCacheSizesV1,observationCollectionsV1}=await import('),
("\n"+EMIT+'const{onPlanStageV1}','\nconst{onPlanStageV1}'),
("process.channel?.unref?.();\n","process.channel?.unref?.();\nleaveBoot();\n"),
("""    const control=routing.createIssuedResourceControlV1(original,"graph:slice-worker",0),tool=workerData.tool??buildBackendDefaultIdV1(),backend=await loadBuildBackendV1(tool);
    publishBuildBootstrapSourcesV1(workerData.root);
    const graph:any=await withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control,async()=>{
     const externalInputs=await backend.graphExternalInputs?.()??[];
     return withResidentSliceNativeRecheckV1(original,control,()=>loadBuildInvocationGraphV1(workerData.root,workerData.arguments,original,original,control,false,undefined,workerData.graphPolicy,buildBackendCompactGraphV1(tool),externalInputs));
    });
""","""    const control=routing.createIssuedResourceControlV1(original,"graph:slice-worker",0),tool=workerData.tool??buildBackendDefaultIdV1();
    const graph:any=await report.sliceReadWaitStepV1("slice-prepare",async(leave:()=>void)=>{
     const backend=await loadBuildBackendV1(tool);
     publishBuildBootstrapSourcesV1(workerData.root);
     return withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control,async()=>{
      const externalInputs=await backend.graphExternalInputs?.()??[];
      return withResidentSliceNativeRecheckV1(original,control,()=>{leave();return loadBuildInvocationGraphV1(workerData.root,workerData.arguments,original,original,control,false,undefined,workerData.graphPolicy,buildBackendCompactGraphV1(tool),externalInputs);});
     });
    });
"""),
])
T=SR+"🧪️tests/🪜️read-steps/🟦️.ts"
edit(T,[('"../../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"];','"../../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs","../../../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts"];'),
("(?:sliceReadStepV1|sliceReadWaitStepV1|step)\\(\"","(?:sliceReadStepV1|sliceReadWaitStepV1|openSliceReadStepV1|step)\\(\""),
('"registry-read","graph-state","module-load"]','"registry-read","graph-state","module-load","worker-boot","slice-prepare"]')])
