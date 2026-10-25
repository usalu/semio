"""Generates the caching route table `⚡️caching/🗂️suites/🔣️.json` from the legacy hand-coded routes, the tree catch-alls and the process-spawning classification. usage: gen-table.py [summary.txt ...] (census summaries add duration evidence)"""
import json,os,re,sys
sys.path.insert(0,os.path.dirname(__file__))
root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/"
disc=[l for l in open(C+"🗑️generated/cache-d/discovered.txt",encoding="utf8").read().split("\n") if l]
legacy_src=open(C+"cache-d-tools/legacy-script-commands.ts.txt",encoding="utf8").read()
M=re.search(r'inferenceTests:Record<string,string>=\{(.*?)\};',legacy_src,re.S)
infer=dict(re.findall(r'"([^"]+)":"([^"]+)"',M.group(1)))
T="🧪️tests/"
files_suites={k:{"files":[v]} for k,v in infer.items() if k!="import-edges"}
tree_suites={"import-edges":["🧪️tests/🔗️import-edges"]}
files_suites.update({
 "playground-preparation-source":{"files":[T+"🎮️playground-preparation/🧪️tests/🟦️.ts"],"typecheck":"strict"},
 "bootstrap-original-invocation":{"files":["🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts"],"typecheck":"strict"},
 "pruning":{"files":["🧹️pruning/⏱️continuation/🧪️tests/🟦️.ts","🧹️pruning/🔒️store/🧪️tests/🟦️.ts","🧹️pruning/🔒️incremental/🧪️tests/🟦️.ts"],"typecheck":"strict-indexed","typecheckFiles":["🧹️pruning/🟦️.ts"]},
 "sdk-snapshot":{"files":["🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🧪️tests/🟦️.ts","🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🧪️tests/🟦️.ts","🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/🟦️.ts"],"typecheck":"strict-indexed","typecheckFiles":["🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot/🟦️.ts"]},
 "journal-source":{"files":["🕸️graph/🔏️seal/📥️membership/📥️journal/🧪️tests/🟦️.ts"],"typecheck":"strict-indexed","typecheckFiles":["🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts"]},
 "authorizers":{"files":["🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts"],"scratch":"checkout","typecheck":"strict-indexed","typecheckFiles":["🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts","🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts","🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts","🚀️bootstrap/🛠️tools/🟦️.ts"]},
 "graph-cache-waiter":{"files":["🕸️graph/🏇️waiter/🧪️tests/🟦️.ts"]},
 "selected-graph":{"files":["🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts"]},
 "graph-seal":{"files":["🕸️graph/🔏️seal/🧪️tests/🟦️.ts"]},
 "artifact-source":{"files":[T+"🗿️artifact-source/🟦️.ts"]},
 "cache-command-source":{"files":[T+"🧱️command-source/🟦️.ts"]},
 "cargo-provenance":{"files":[T+"🧾️cargo-provenance/🟦️.ts"]},
 "native-input-vocabulary":{"files":["../🧪️tests/⚡️production-cache-input-boundary/🟦️.ts"]},
})
for name in ("law-coverage","law-coverage-fixes","law-coverage-tests"):
    m=re.search(r'args\[0\] === "'+name+r'"\) \{.*?\.\.\.\[(.*?)\]\.map',legacy_src,re.S)
    files_suites[name]={"files":re.findall(r'"([^"]+)"',m.group(1))}
TREES=[
 ("vite",["🌐️vite"]),
 ("storage-compiler",["💾️storage/🔒️enforcement/🏗️compiler"]),
 ("storage-opening",["💾️storage/🔒️enforcement/📦️budget/📥️opening"]),
 ("storage-graph",["💾️storage/🔒️enforcement/📦️budget/🕸️graph"]),
 ("storage-budgets",["💾️storage/🔒️enforcement/📦️budget"]),
 ("storage-nx",["💾️storage/🔒️enforcement/🔌️nx"]),
 ("storage-session",["💾️storage/🔒️enforcement/🧵️session","💾️storage/🧾️ledger"]),
 ("storage",["💾️storage"]),
 ("tools-sdk",["🚀️bootstrap/🛠️tools/📦️sdk"]),
 ("tools-authorizers",["🚀️bootstrap/🛠️tools/🔏️authorizers"]),
 ("tools-retention",["🚀️bootstrap/🛠️tools/🧹️retention"]),
 ("tools",["🚀️bootstrap/🛠️tools"]),
 ("bootstrap-dependencies",["🚀️bootstrap/📦️dependencies"]),
 ("bootstrap",["🚀️bootstrap"]),
 ("cargo-units",["🦀️cargo/📦️units"]),
 ("cargo-unit-cache",["🦀️cargo/🗃️unit-cache"]),
 ("cargo",["🦀️cargo"]),
 ("graph",["🕸️graph"]),
 ("artifacts",["📦️artifacts"]),
 ("bundle",["📦️bundle"]),
 ("texts",["📖️texts"]),
 ("inference",["📥️inference"]),
 ("records",["🗃️records"]),
 ("ci",["🚦️ci"]),
 ("pruning-laws",["🧹️pruning"]),
 ("module-tests",["🧪️tests"]),
 ("route-table",["🗂️suites"]),
]
TOKENS=json.load(open(root+"🗂️suites/🧪️tests/🧫️fixtures/🔣️.json",encoding="utf8"))["spawnTokens"]
evidence={}
for path in sys.argv[1:]:
    for line in open(path,encoding="utf8"):
        parts=line.rstrip("\n").split("\t")
        if len(parts)<4 or parts[0]=="DONE":continue
        dur=int(re.sub(r"\D","",parts[3]) or 0)
        evidence[parts[0]]=max(evidence.get(parts[0],0),dur+(1000 if parts[1]=="rc=WALL" else 0))
def spawns(rel):
    text=open(os.path.normpath(root+rel),encoding="utf8").read()
    return any(t in text for t in TOKENS) or evidence.get(rel,0)>=6
def owner_trees(f):
    best=None
    for sid,trees in tree_suites.items():
        for t in trees:
            if f.startswith(t+"/") and (best is None or len(t)>best[0]):best=(len(t),sid)
    for sid,trees in TREES:
        for t in trees:
            if f.startswith(t+"/") and (best is None or len(t)>best[0]):best=(len(t),sid)
    return best[1] if best else None
CK=C+"cache-d-tools/checkout-files.txt"
checkout=[l for l in open(CK,encoding="utf8").read().split("\n") if l] if os.path.exists(CK) else []
explicit={f for s in files_suites.values() for f in s["files"]}|set(checkout)
tree_files={}
for f in disc:
    if f in explicit:continue
    sid=owner_trees(f)
    tree_files.setdefault(sid,[]).append(f)
suites=[];FUND,QUICK=15000,300000
def row(sid,budget,**kw):
    r={"id":sid,"budgetMs":budget,"typecheck":kw.pop("typecheck","none")};r.update(kw);return r
for sid,spec in files_suites.items():
    flag=any(spawns(f) for f in spec["files"] if not f.startswith(".."))or any(f.startswith("..") for f in spec["files"])
    r=row(sid,QUICK if flag else FUND,typecheck=spec.get("typecheck","none"),files=spec["files"])
    if "typecheckFiles" in spec:r["typecheckFiles"]=spec["typecheckFiles"]
    if "scratch" in spec:r["scratch"]=spec["scratch"]
    suites.append(r)
for sid,trees in [("import-edges",["🧪️tests/🔗️import-edges"])]:
    suites.append(row(sid,QUICK if any(spawns(f) for f in disc if f.startswith(trees[0]+"/")) else FUND,trees=trees))
known=set()
for sid,trees in TREES:
    owned=tree_files.get(sid,[])
    if not owned:continue
    heavy=[f for f in owned if spawns(f)];light=[f for f in owned if f not in heavy]
    if light:suites.append(row(sid,FUND,trees=trees))
    if heavy:
        if len(heavy)>64:raise SystemExit(f"{sid}: {len(heavy)} heavy files exceed the schema maximum; split the tree")
        suites.append(row(sid+"-processes",QUICK,files=sorted(heavy)))
    if heavy and not light:
        suites[-1]["trees"]=trees
for name,flag in (("checkout-scratch",False),("checkout-scratch-processes",True)):
    rows=sorted(f for f in checkout if spawns(f)==flag)
    if rows:
        if len(rows)>64:raise SystemExit("checkout scratch suite exceeds 64 files")
        suites.append(row(name,QUICK if flag else FUND,files=rows,scratch="checkout"))
leftover=tree_files.get(None,[])
if leftover:raise SystemExit("unrouted: "+", ".join(leftover))
CODE=["daemon-journal","binaryen-partition","bazel-storage-runtime","runtime-verification-storage-runtime","javascript-partition","lease-queue","leases-partition","rustup-toolchain","rustup-generation","tools-partition","build-mode","next-runtime","web-build","cache-policy","cargo-build-lease","graph-revision","devcontainer-lifecycle","runtime-bootstrap","native-dependencies","workspace-ownership","sdk-material"]
table={"version":1,"suites":suites,"codeSuites":CODE,"helpers":[]}
print(json.dumps({"suites":len(suites),"fund":sum(1 for s in suites if s["budgetMs"]==FUND),"quick":sum(1 for s in suites if s["budgetMs"]==QUICK)}))
json.dump(table,open(root+"🗂️suites/🔣️.json","w",encoding="utf8"),ensure_ascii=False,indent=2);open(root+"🗂️suites/🔣️.json","a",encoding="utf8").write("\n")
