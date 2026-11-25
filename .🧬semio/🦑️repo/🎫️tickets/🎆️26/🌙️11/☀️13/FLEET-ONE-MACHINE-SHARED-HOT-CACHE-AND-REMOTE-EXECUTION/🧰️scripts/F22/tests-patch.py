B='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/'
E=B+'⚡️caching/💾️storage/🔒️enforcement/📦️budget/'
files={
 E+'🕸️graph/📏️headroom/🧪️tests/🔀️selected-state-recovery/🟦️.ts':('../../../../📥️opening/🚦️admission/🟦️.ts',[
  ('partitionOwners:owners,partitionFrames:new AsyncLocalStorage(),','partitionOwners:owners,partitionFrames:new AsyncLocalStorage(),...admissionPorts,'),
  ('.rejects.toThrow("Exact live repository partition cache owner required")','.rejects.toThrow("not issued by its cache owner")'),
 ]),
 B+'🏗️build/📥️inputs/💾️storage/🧪️tests/♻️scoped/⚠️scoped-retirement-and-bridge/🟦️.ts':('../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🚦️admission/🟦️.ts',[
  ('partitionOwners:owners,partitionFrames:frames,','partitionOwners:owners,partitionFrames:frames,...admissionPorts,'),
 ]),
}
for path,(rel,edits) in files.items():
    t=open(path,encoding='utf8').read()
    for old,new in edits:
        assert t.count(old)>=1,(path[-40:],old[:50],t.count(old))
        t=t.replace(old,new)
    lines=t.split('\n')
    k=max(i for i,l in enumerate(lines) if l.startswith('import'))
    lines.insert(k+1,'import*as admission from"%s";const admissionPorts={partitionOperationSlotsV1:admission.partitionOperationSlotsV1,partitionRefusalV1:admission.partitionRefusalV1,partitionFailureTextV1:admission.partitionFailureTextV1,awaitPartitionSlotV1:admission.awaitPartitionSlotV1};'%rel)
    open(path,'w',encoding='utf8').write('\n'.join(lines))
print("ok")
