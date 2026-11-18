import sys
path = sys.argv[1]
text = open(path, encoding="utf-8", newline="").read()
old_start = text.index("/** 🗄️ Runs one synchronization as side-by-side phases")
old_end = text.index("\n}\n", old_start) + 3
new = '''/** 🔐️ Runs one body holding the exclusive bindings gate and the writer frame's gate marker, releases both afterwards and surfaces the first failure. */
async function underJavascriptDependencyBindingsGateV1(frame:JavascriptDependencyStoreFrameV1,body:()=>Promise<void>):Promise<void>{
 const gate=await acquireConcurrentLeaseV1("bindings",frame.workspace,"exclusive",concurrentLeasePortsV1(repositoryLeaseDirectoryV1(frame.workspace),frame.control,frame.control.signal,undefined,javascriptDependencyConcurrencyReportV1(frame.control)),concurrentProtocolV1.wait.maximumPublishMilliseconds);
 if(!gate)throw Error("Dependency bindings gate required");
 frame.gate=gate;let failed=false,failure:unknown;
 try{await body();}catch(error){failed=true;failure=error;}
 frame.gate=null;
 try{gate.release();}catch(error){retainedJavascriptDependencyStoreCustodyV1.add(gate);if(!failed){failed=true;failure=error;}}
 if(failed)throw failure;
}
/** 🫙️ Retires the completed preservation capsules of earlier synchronizations before any build starts when the image cannot hold the largest generation plus margin: a capsule keeps a whole replaced Source copy until a publication retires it, so interrupted synchronizations leave them behind, and without this step the image they fill refuses the very build whose publication would free them. */
async function reclaimJavascriptDependencyPreservationsUnderPressureV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl):Promise<void>{
 const frame=requireJavascriptDependencyStoreFrameV1(workspace,store,control),verdict=concurrentAdmissionV1({scope:"",pointers:readConcurrentPointersV1(store.root),freeBytes:await concurrentFreeBytesV1(store.root)});
 if(verdict.admitted)return;
 await javascriptDependencyConcurrencyReportV1(control)(concurrentEventV1({event:"reclaiming",neededBytes:verdict.requiredBytes,freeBytes:verdict.freeBytes}));
 await underJavascriptDependencyBindingsGateV1(frame,async()=>{await retireJavascriptDependencyPreservationsInStoreV1(workspace,store,control);});
}
/** 🗄️ Runs one synchronization as side-by-side phases: reclaim preservation capsules while the image is too full for one more generation, build immutable generations holding only the shared image and the build lease, switch the bindings under a short exclusive gate over already built generations, then collect what no binding references, no reader pins and the retention policy no longer holds. */
async function runJavascriptDependencyStoreGenerationsV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,selection:JavascriptDependencyStoreSelectionV1):Promise<void>{
 await reclaimJavascriptDependencyPreservationsUnderPressureV1(workspace,store,control);
 const frame=requireJavascriptDependencyStoreFrameV1(workspace,store,control),built=await runJavascriptDependencyStoreProducerV1(workspace,store,control,selection,"build");
 if(built.state==="current")return;
 await underJavascriptDependencyBindingsGateV1(frame,async()=>{await runJavascriptDependencyStoreProducerV1(workspace,store,control,selection,"publish");await retireJavascriptDependencyPreservationsInStoreV1(workspace,store,control);});
 await collectJavascriptDependencyGenerationsV1(workspace,store,control,{});
}
'''
text = text[:old_start] + new + text[old_end:]
open(path, "w", encoding="utf-8", newline="").write(text)
