import sys,os
base=sys.argv[1]
old='readNativeImageRetirementFileV1(path,storageSchema.definitions.ImageProtocol.const.journalMaximumBytes).stamp!==prior.stamp)throw Error("Original disposable'
new='!readNativeImageRetirementFileV1(path,storageSchema.definitions.ImageProtocol.const.journalMaximumBytes).bytes.equals(prior.bytes))throw Error("Original disposable'
for rel in ("🧹️cleanup/🎛️command/🟦️.ts","🧱️fixed/🟦️.ts"):
    F=base+"/"+rel;t=open(F,encoding="utf8").read()
    assert t.count(old)==1,(rel,t.count(old))
    t=t.replace(old,new)
    tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F)
print("ok")
