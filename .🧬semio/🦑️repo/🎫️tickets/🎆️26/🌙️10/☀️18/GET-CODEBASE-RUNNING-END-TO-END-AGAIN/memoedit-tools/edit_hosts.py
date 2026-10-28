"""🔧 Anchored edits of batch `derived` in two hot host files (re-run on the then-live files when they drifted; idempotent):
1. `🧭️invocation/🕸️graph/🟦️.ts` `loadBuildGraphInChildV1`: the planner child's launch becomes one `child(label, environment)` helper; the memo receives it as its derivation child (`build:graph-derive`).
2. `⌨️command/⚙️runtime/🟦️.ts`: a child started with `SEMIO_BUILD_GRAPH_MEMO_DERIVE` answers the derivation request and returns before any tool or graph work (it also carries the plan marker, so the dependency step stays skipped as for the planner child).
usage: python3 -I edit_hosts.py <root>"""
import os, sys
root = sys.argv[1]
B = os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")
def edit(rel, pairs):
    path = os.path.join(B, rel); text = open(path, encoding="utf8").read()
    for old, new in pairs:
        if new in text: continue
        if text.count(old) != 1: sys.exit("anchor not unique or missing in %s: %s" % (rel, old[:80]))
        text = text.replace(old, new)
    open(path, "w", encoding="utf8").write(text)
edit("🧭️invocation/🕸️graph/🟦️.ts", [
 (""" const memo=await openGraphMemoForCommandV1(root,args,parent.capabilities as {tool?:string;buildMode?:string},phaseControl.signal);
 const directory=mkdtempSync(join(tmpdir(),"semio-graph-plan-")),file=join(directory,"graph.json");
 try{""",
  """ const directory=mkdtempSync(join(tmpdir(),"semio-graph-plan-")),file=join(directory,"graph.json");
 const child=async(label:string,extra:Readonly<Record<string,string>>):Promise<void>=>{
  const environment=await projectOwnedBuildChildEnvironmentV1(root,args,parent,phaseControl);checkResourceLeaseControl(phaseControl);
  const progress=async()=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(parent,label,"running");checkResourceLeaseControl(phaseControl);};await progress();
  await runAdoptedCapturedBuildSourceV1(args,parent,phaseControl,{label,cwd:root,budgetMilliseconds:scriptInvocationBudget(parent,compilerAdmissionBudgetV1(phaseControl)),environment:{...environment,[BUILD_GRAPH_PLAN_OUT_V1]:file,...extra,BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1())},stdout:"stderr",onProgress:progress});
  checkResourceLeaseControl(phaseControl);
 };
 try{
  const memo=await openGraphMemoForCommandV1(root,args,parent.capabilities as {tool?:string;buildMode?:string},phaseControl.signal,process.env,environment=>child("build:graph-derive",environment));"""),
 ("""   plan:async sidecar=>{
    const environment=await projectOwnedBuildChildEnvironmentV1(root,args,parent,phaseControl);checkResourceLeaseControl(phaseControl);
    const progress=async()=>{checkResourceLeaseControl(phaseControl);await advanceScriptInvocation(parent,"build:graph-plan","running");checkResourceLeaseControl(phaseControl);};await progress();
    await runAdoptedCapturedBuildSourceV1(args,parent,phaseControl,{label:"build:graph-plan",cwd:root,budgetMilliseconds:scriptInvocationBudget(parent,compilerAdmissionBudgetV1(phaseControl)),environment:{...environment,[BUILD_GRAPH_PLAN_OUT_V1]:file,...sidecar,BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1())},stdout:"stderr",onProgress:progress});
    checkResourceLeaseControl(phaseControl);
    return""",
  """   plan:async sidecar=>{
    await child("build:graph-plan",sidecar??{});
    return"""),
 ("""loads exactly what this process loaded, writes the slice and exits; nothing it allocated stays resident in the loader.""",
  """loads exactly what this process loaded, writes the slice and exits; nothing it allocated stays resident in the loader. The memo asks the same launch (`build:graph-derive`) to settle the derived facts an edit touched."""),
])
edit("⌨️command/⚙️runtime/🟦️.ts", [
 ("""    const {prepareBuildToolStorageV1}=await profilePhaseV1("tools.storage-load",""",
  """    if(process.env.SEMIO_BUILD_GRAPH_MEMO_DERIVE!==undefined){await profilePhaseV1("graph.memo-derive",async()=>(await import("../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts")).answerGraphMemoDerivationV1(root,process.env,phaseControl.signal));return;}
    const {prepareBuildToolStorageV1}=await profilePhaseV1("tools.storage-load","""),
])
print("[DEBUG] hosts edited")
