"""P8: anchored edit of project-nodes/🟨️.mjs applied to the LIVE text at landing time (package P7 edits the same loop line concurrently, so a fixed three-way merge would conflict)."""
import re
def sub(t,pattern,repl,label):
    out,n=re.subn(pattern,lambda m:repl(m) if callable(repl) else repl,t,count=1,flags=re.S)
    if n!=1:raise SystemExit("pn_transform: anchor not found: "+label)
    return out
def transform(t):
    if "graphOwnerEnforcementV1" in t:raise SystemExit("pn_transform: already applied")
    t=sub(t,r'import \{ createGraphOwnerPeaksFileV1, createGraphParentMemoryGovernorV1, createMemoryPortsV1, createParentCollectorV1, graphMemoryAllocationV1, graphParentManagedBytesV1 \} from "\.\./\.\./\.\./🧠️memory/🟨️\.mjs";',
      'import { createGraphOwnerPeaksFileV1, createGraphParentMemoryGovernorV1, createMemoryPortsV1, createParentCollectorV1, graphMemoryAllocationV1, graphParentManagedBytesV1, hostMemoryMegabytesV1, selectGraphMemoryLanguagesV1 } from "../../../🧠️memory/🟨️.mjs";\nimport { ownerTooHeavyOfV1, runInOwnerScopeV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";'.replace('\\','\\\\'),'import')
    helper='''/** 🪙️ What one project may take on this machine and where the cost of every owner is recorded: the limit is the grant of the machine for one project (`ownerGrantV1` of half its memory, never below the growth budget of the parent), none on a serving machine or with `SEMIO_GRAPH_OWNER_LIMIT=off`; a cost module that cannot be loaded enforces nothing and records nothing (a read never depends on it). */
export async function graphOwnerEnforcementV1(workspaceRoot, environment, budgetBytes, rowsOf) {
  if (environment.SEMIO_GRAPH_OWNER_LIMIT === "off") return {};
  try {
    const [cost, wanted] = await Promise.all([import("../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts"), import("../../../🪪️graph-state/🙋️wanted/🟦️.ts")]);
    const grant = cost.ownerGrantV1(Math.floor(hostMemoryMegabytesV1() / 2), wanted.wantedPolicyV1), recorder = cost.createOwnerCostRecorderV1({ path: cost.ownerCostPathV1(workspaceRoot), rowsOf });
    return { cost, recorder, limitBytes: Number.isFinite(grant.managedMegabytes) ? Math.max(grant.managedMegabytes * 1048576, budgetBytes) : undefined };
  } catch { return {}; }
}

'''
    t=sub(t,r'(/\*\* 🛡️ Infers the project nodes of every authored emoji-named project config\.)',lambda m:helper+m.group(1),'doc anchor')
    t=sub(t,r'(const memoryPorts=createMemoryPortsV1\(\),parentBytes=graphParentManagedBytesV1\(process\.env\),parentBaseline=memoryPorts\.sample\(\),parentCollector=createParentCollectorV1\(\{ports:memoryPorts,budgetBytes:parentBytes,baselineBytes:parentBaseline\}\);)',
      lambda m:m.group(1)+'\n  const rowCounts=new Map(),tooHeavy=new Map(),enforcement=await graphOwnerEnforcementV1(workspaceRoot,process.env,parentBytes,key=>rowCounts.get(key)??0);','memory ports')
    t=sub(t,r'(peaks:createGraphOwnerPeaksFileV1\(join\(workspaceDataDirectory\(workspaceRoot\),"np","owner-peaks\.json"\)\),)(signal:context\.phaseControl\?\.signal,)',
      lambda m:m.group(1)+'...enforcement.limitBytes===undefined?{}:{ownerLimitBytes:enforcement.limitBytes},...enforcement.recorder===undefined?{}:{costs:enforcement.recorder},'+m.group(2),'governor options')
    t=sub(t,r'await governor\.run\(async\(\{away\}\)=>\{decision=undefined;','await governor.run(async({away,scope})=>{decision=undefined;','governor.run')
    t=sub(t,r'try\{if\(collector\)await collector\.guard\(\{project:job\.name,root:job\.json\.root,phase:"project"\},\(\)=>inferProject\(job,away(,queuedAt)?\)\);else await inferProject\(job,away(?:,queuedAt)?\);\}finally\{forgetObservedInferenceRowsV1\(',
      lambda m:'try{try{await runInOwnerScopeV1(scope,()=>collector?collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job,away'+(m.group(1) or '')+')):inferProject(job,away'+(m.group(1) or '')+'));}catch(error){if(ownerTooHeavyOfV1(error)===undefined)throw error;}if(scope.exceeded!==undefined)tooHeavy.set(job.name,{...scope.exceeded,name:job.name});}finally{rowCounts.set(job.name,observedInferenceRowsV1(ownerCachePathV1(workspaceRoot,job.json.root)));forgetObservedInferenceRowsV1(','owner try')
    t=sub(t,r'(,\{kind:ownerKindV1\(job\.json,workspaceRoot\))(\}\);\}await publishRunning\(\);)',lambda m:m.group(1)+',key:job.name'+m.group(2),'owner kind')
    t=sub(t,r'(graphWaveMeterV1\.parent\(parentCollector\.finish\(\)\);\n)',lambda m:m.group(1)+'  enforcement.recorder?.flush();\n','wave meter')
    t=sub(t,r'(  await phases\?\.end\("inference",\{projects:jobs\.length,\.\.\.\(governor\?\{memory:governor\.stats\(\)\}:\{\}\)\}\);\n)',
      lambda m:m.group(1)+'  if(tooHeavy.size>0){const heavy=[...tooHeavy.values()].sort((a,b)=>b.takenBytes-a.takenBytes).map(row=>({name:row.name,megabytes:Math.round(row.takenBytes/1048576),source:"recorded"}));throw refuseBuildV1("unserved","graph.owner-too-heavy",{named:enforcement.cost.describeHeavyV1(heavy,selectGraphMemoryLanguagesV1(process.env)[0]),limit:Math.round(enforcement.limitBytes/1048576)});}\n','phase end')
    return t
if __name__=="__main__":
    import sys
    sys.stdout.write(transform(open(sys.argv[1],encoding="utf8").read()))
