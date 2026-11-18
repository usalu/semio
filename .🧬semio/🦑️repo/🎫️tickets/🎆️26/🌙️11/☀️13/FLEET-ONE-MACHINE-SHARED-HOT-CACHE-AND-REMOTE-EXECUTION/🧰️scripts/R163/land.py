import sys,os,shutil
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes"
POOL=B+"/🏊️project-farm/🏊️pool/🟨️.mjs"; PN=B+"/🟨️.mjs"
def edit(path,pairs):
    s=open(path).read()
    for old,new in pairs:
        assert s.count(old)==1,(path,old[:60])
        s=s.replace(old,new)
    open(path,'w').write(s)
pool=[('      if (slot.state !== "idle") continue;\n      if (held && busy > 0','      if (slot.state !== "idle") continue;\n      if (queue[0].alone === true && slot.served > 0) { retire(slot, true); continue; }\n      if (held && busy > 0')]
ins='\n    const stoppedOf = name => { const entry = recorder.table().owners[name], stopped = entry === undefined ? [] : entry.samples.filter(sample => sample.stopped && sample.atMs >= STOPS_ATTRIBUTABLE_FROM_MS_V1); return stopped.length === 0 ? undefined : Math.round(cost.estimateOwnerCostV1({ ...recorder.table(), owners: { [name]: { ...entry, samples: stopped } } }, name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }).megabytes * cost.ownerCostPolicyV1.footprintFactor); };'
s=open(PN).read(); i=s.index('    const soloBytes = () => {'); j=s.index('\n',i)
anchor=s[i:j]
pn=[(anchor,anchor+ins),
 ('if (childFootprintMegabytes !== undefined) { const entry = table.owners[name], stopped = entry === undefined ? [] : entry.samples.filter(sample => sample.stopped); return stopped.length > 0 && estimate({ ...table, owners: { [name]: { ...entry, samples: stopped } } }, name).megabytes * cost.ownerCostPolicyV1.footprintFactor > childFootprintMegabytes; }','if (childFootprintMegabytes !== undefined) { const footprint = stoppedOf(name); return footprint !== undefined && footprint > childFootprintMegabytes; }'),
 ('const RUNNING_TICK_MS = INFERENCE_LIMITS_V1.runningTickMilliseconds;','const RUNNING_TICK_MS = INFERENCE_LIMITS_V1.runningTickMilliseconds;\n\n/** 🧊️ The first moment from which a recorded STOP of an owner in a farm child is its own: before it an alone job could land on a child that kept the memory of earlier jobs, so such a stop proves nothing about the owner and never refuses it (2026-11-18 00:30 UTC, `graph.farm-overrun` now runs alone only in a fresh child). */\nexport const STOPS_ATTRIBUTABLE_FROM_MS_V1 = 1794961800000;'),
 ('      known: name => {','      stopped: stoppedOf,\n      known: name => {'),
 ('megabytes:Math.round((enforcement.known?.(name)??0)/enforcement.cost.ownerCostPolicyV1.footprintFactor),source:"recorded"})),selectGraphMemoryLanguagesV1(process.env)[0]);\n  const refuseHeavy','megabytes:Math.round((enforcement.stopped?.(name)??enforcement.known?.(name)??0)/enforcement.cost.ownerCostPolicyV1.footprintFactor),source:"recorded"})),selectGraphMemoryLanguagesV1(process.env)[0]);\n  const refuseHeavy')]
mode,bk=sys.argv[1],sys.argv[2]
if mode=="apply":
    os.makedirs(bk,exist_ok=True)
    shutil.copy2(POOL,bk+"/pool.mjs"); shutil.copy2(PN,bk+"/pn.mjs")
    edit(POOL,pool); edit(PN,pn); print("applied")
else:
    shutil.copy2(bk+"/pool.mjs",POOL); shutil.copy2(bk+"/pn.mjs",PN); print("restored")
