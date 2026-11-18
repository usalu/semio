import sys,json
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
LOAD="🕸️graph/📥️load/🟦️.ts"
OLD1='  for(let attempt=0;attempt<=8;attempt++){\n    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);\n'
NEW1='  for(;;){\n    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);\n'
OLD2='    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;\n    await advanceScriptInvocation(invocation,"build:refresh","running");\n  }\n  throw Error("Build source authority keeps changing during inference");\n}'
NEW2='    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;\n    clearPinnedAuthorityV1();process.stderr.write("[graph-repair] "+JSON.stringify({version:1,registry:"adopted"})+"\\n");\n    await advanceScriptInvocation(invocation,"build:refresh","running");\n  }\n}'
OLD3='/** 🕸️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. */'
NEW3='/** 🕸️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. A moved owner registry is adopted (the authority snapshot is dropped and the registry read once more, its owners are inferred or served by their own records) and never ends the read: under the pin the next registry read is fixed again, so the loop has no count to give up at and ends with the first candidate that holds. */'
def edit(path,pairs,reverse=False):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        if reverse:a,b=b,a
        if s.count(a)!=1:sys.exit("anchor count %d: %s"%(s.count(a),a[:60]))
        s=s.replace(a,b)
    open(path,"w",encoding="utf8").write(s)
if __name__=="__main__":
    mode,target=sys.argv[1],sys.argv[2]
    edit(target,[(OLD1,NEW1),(OLD2,NEW2),(OLD3,NEW3)],mode=="restore")
