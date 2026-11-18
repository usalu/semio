import sys
def sub(t,old,new):
    assert t.count(old)==1,(old[:70],t.count(old))
    return t.replace(old,new)
def patch(t):
    t=sub(t,"  const namedInputs = await projectInputs({ ...json, targets: normalized }, root, workspaceRoot, facts, scripts,sourceRepresentation,sourceGroups,inputNames,Object.fromEntries([...commandSourceGroups.values()].map(({name,inputs})=>[name,inputs])));",
"  const providerGroups = json.metadata?.semio?.scriptProviders === undefined ? [] : await closures.sharedClosureGroups(root, json.metadata.semio.scriptProviders);\n  const namedInputs = await projectInputs({ ...json, targets: normalized }, root, workspaceRoot, facts, scripts,sourceRepresentation,sourceGroups,inputNames,{...Object.fromEntries(providerGroups.map(([name])=>[name,[]])),...Object.fromEntries([...commandSourceGroups.values()].map(({name,inputs})=>[name,inputs]))});")
    t=sub(t,"  if (json.metadata?.semio?.scriptProviders !== undefined) for (const [name, inputs] of await closures.sharedClosureGroups(root, json.metadata.semio.scriptProviders)) {","  for (const [name, inputs] of providerGroups) {")
    return t
if __name__=="__main__":
    src,dst=sys.argv[1:3]
    open(dst,"w",encoding="utf8").write(patch(open(src,encoding="utf8").read()))
