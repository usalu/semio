import sys,os,json,hashlib
W="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/"
F=W+"🟨️.cjs"; TF=W+"🧪️tests/🧾️receipt/🟦️.ts"
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
L1_START="    for (const record of observations.paths.values()) {\n      control.signal.throwIfAborted();\n      const facts = read(record.path);"
L1_END="    if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return ownershipMoved(owner, owner.ownership.ownershipPath, \"content digest\", \"bytes\");\n    for (const [path, text] of owner.texts) {\n      control.signal.throwIfAborted();\n      if (readManifestText(bounded(owner.root, path), owner.physical) !== text)"
NEW1='''    for (const record of observations.paths.values()) {
      control.signal.throwIfAborted();
      const completedOperations = ++progress.completedOperations;
      if (++visited === observations.paths.size || completedOperations % (control.frontierBatchPaths ?? 1) === 0) try { await control.advance({ operation: "frontier", path: record.path, completedOperations }); } catch (error) { continuationFailed = true; throw error; }
      control.signal.throwIfAborted();
    }
'''
OLD2='''        const before = proof.get(record.path); if (!facts?.isDirectory() || !before) return ownershipMoved(owner, record.path, "identity", "identity");
        const stamp = directoryStamp(facts);
        if ((before.exact || before.stamp !== stamp) && (JSON.stringify(projectListing(owner.root, record.path, operations.list(record.path))) !== JSON.stringify(record.list) || directoryStamp(read(record.path)) !== stamp)) return ownershipMoved(owner, record.path, "listing", "roster");
        proof.set(record.path, { stamp, settled: settledDirectory(facts, Date.now()) });'''
NEW2='''        if (!facts?.isDirectory()) return ownershipMoved(owner, record.path, "identity", "identity");
        const before = observations.frontier.get(record.path), stamp = directoryStamp(facts), decision = ownershipFrontierDecisionV1(before?.stamp, stamp, before?.settled && settledDirectory(facts, Date.now()));
        if (decision === "derive") return ownershipMoved(owner, record.path, "identity", "identity");
        if (decision === "relist" && (JSON.stringify(projectListing(owner.root, record.path, operations.list(record.path))) !== JSON.stringify(record.list) || directoryStamp(read(record.path)) !== stamp)) return ownershipMoved(owner, record.path, "listing", "roster");
        proof.set(record.path, { stamp, settled: settledDirectory(facts, Date.now()) });'''
PAIRS=[("      let record = cached?.stamp === stamp && cached.root === root ? cached : undefined;\n","      let record = cached?.stamp === stamp && cached.root === root ? cached : undefined, latest = before;\n"),
("        if (after.kind !== \"directory\") throw Error(\"Bun workspace requires a real directory: \" + path);\n        record = { stamp,","        if (after.kind !== \"directory\") throw Error(\"Bun workspace requires a real directory: \" + path);\n        latest = after.entry;\n        record = { stamp,"),
("        const after = fs.lstatSync(path, { bigint: true });\n        if (after.isDirectory() && directoryStamp(after) === stamp) observations.frontier.set(path, { stamp, settled: settledDirectory(after, listedAt) });","        if (latest.isDirectory() && directoryStamp(latest) === stamp) observations.frontier.set(path, { stamp, settled: settledDirectory(latest, listedAt) });"),
(OLD2,NEW2)]
TPAIRS=[("one recheck of an unchanged custody replays every observed path at most twice (call-count law)","one recheck of an unchanged custody replays every observed path once (call-count law)"),("toBeLessThanOrEqual(2*records+50*manifests)","toBeLessThanOrEqual(records+50*manifests)")]
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    old=read(F);new=old
    a=new.index(L1_START);b=new.index(L1_END);assert new.count(L1_START)==1 and a<b
    new=new[:a]+NEW1+new[b:]
    for x,y in PAIRS:
        assert new.count(x)==1,("anchor",x[:50],new.count(x));new=new.replace(x,y)
    told=read(TF);tnew=told
    for x,y in TPAIRS:
        assert tnew.count(x)==1,("test anchor",x[:40]);tnew=tnew.replace(x,y)
    os.makedirs(backup,exist_ok=True);write(backup+"/0.bak",old);write(backup+"/1.bak",told)
    json.dump({"before":sha(old),"after":sha(new),"tbefore":sha(told),"tafter":sha(tnew)},open(backup+"/manifest.json","w"))
    write(F,new);write(TF,tnew);print("applied")
else:
    m=json.load(open(backup+"/manifest.json"));now=sha(read(F));tnow=sha(read(TF))
    if now==m["after"]:write(F,read(backup+"/0.bak"));print("restored cjs")
    elif now==m["before"]:print("cjs already original")
    else:print("CONFLICT cjs");sys.exit(4)
    if tnow==m["tafter"]:write(TF,read(backup+"/1.bak"))
