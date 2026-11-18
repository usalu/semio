import os,sys
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
from lib import *
B=LIB+"/🏗️build"
NODES=B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
POOL=B+"/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs"
COST=B+"/🪪️graph-state/🙋️wanted/💰️cost/🧬️schema/🔣️.json"
SEEDS=eval(os.environ.get("R145_SEEDS","{}")) if "R145_SEEDS" in os.environ else {}

def patch_nodes(t):
    t=sub1(t,'      neverHere: names => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }); return names.filter(name => { const row = cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); return cost.ownerCostPolicyV1.heavySources.includes(row.source) && (row.saturated === true || row.megabytes * 1048576 > here); }); },',
'      neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }); return names.filter(name => { const row = cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); if (row.source !== "recorded") return false; return row.saturated === true || (childFootprintMegabytes === undefined ? row.megabytes * 1048576 > here : row.megabytes * cost.ownerCostPolicyV1.footprintFactor > childFootprintMegabytes); }); },')
    t=sub1(t,'/** 🪙️ What one project may take on this machine and where the cost of every owner is recorded:',
'/** 📏️ The peak a farm child reports (its physical footprint in bytes) in the unit of the cost table, the managed bytes: divided by the footprint factor, so `known` multiplies it back to the footprint once and not twice; undefined stays undefined, no factor leaves the bytes. */\nexport function ownerMeasuredManagedBytesV1(bytes, factor) { return bytes === undefined ? undefined : factor > 0 ? bytes / factor : bytes; }\n\n/** 🪙️ What one project may take on this machine and where the cost of every owner is recorded:')
    t=sub1(t,'enforcement.neverHere?.(jobs.filter(job=>alone.has(job.name)).map(job=>job.name))??[]','enforcement.neverHere?.(jobs.filter(job=>alone.has(job.name)).map(job=>job.name),farm===undefined?undefined:farmAloneCapV1(hostMemoryMegabytesV1()))??[]')
    t=sub1(t,'measured:name=>farm?.peakOf?.(name),','measured:name=>ownerMeasuredManagedBytesV1(farm?.peakOf?.(name),enforcement.cost?.ownerCostPolicyV1.footprintFactor),')
    t=sub1(t,'import { planFarmWorkersV1, planStageLimitBytesV1 } from','import { farmAloneCapV1, planFarmWorkersV1, planStageLimitBytesV1 } from')
    t=sub1(t,'const refuseHeavy=left=>refuseBuildV1("unserved","graph.owner-too-heavy",{named:named(left),limit:Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor)});',
'const refuseHeavy=left=>refuseBuildV1("unserved","graph.owner-too-heavy",{named:named(left),limit:Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor),producers:Object.keys(fleetPort?.producers?.()??{}).length});')
    return t

def patch_pool(t):
    return sub1(t,'if(error?.key==="build.graph.owner-too-heavy"&&!signal?.aborted&&heavyHandoffs<cutPolicy.heavyHandoffs){','if(error?.key==="build.graph.owner-too-heavy"&&!signal?.aborted&&Number(error.values?.producers)!==0&&heavyHandoffs<cutPolicy.heavyHandoffs){')

def patch_cost(t):
    for name,value in SEEDS.items():
        import re
        t,n=re.subn(r'("%s": )\d+'%re.escape(name),r'\g<1>%d'%value,t);assert n==1,(name,n)
    return t
