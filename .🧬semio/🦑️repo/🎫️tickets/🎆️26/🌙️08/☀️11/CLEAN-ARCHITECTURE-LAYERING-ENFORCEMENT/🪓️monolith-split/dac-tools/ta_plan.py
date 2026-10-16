"""🧭️ Read-only: plan the test_allocation re-point. Per crate: definition style, files using the token."""
import json,re,os,subprocess,collections
os.chdir("/Users/ueli/Documents/semio")
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
reach=json.load(open(G+"/dac-reach.json"))
files=[f for f in subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z","--","*.rs",":!.🧬semio"],capture_output=True,text=True).stdout.split("\0") if f and os.path.exists(f)]
tok=re.compile(r"(?<![A-Za-z0-9_])test_allocation(?![A-Za-z0-9_])")
text={}
for f in files:
    t=open(f,encoding="utf8").read()
    if tok.search(t): text[f]=t
DEF=re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+semio_framework_trace_memory_testing\s+as\s+test_allocation\s*;",re.M)
MODDEF=re.compile(r"(?:pub(?:\([^)]*\))?\s+)?mod\s+test_allocation\b")
crate_def=collections.defaultdict(set)   # (manifest,lib)-> files defining alias
crate_files=collections.defaultdict(set)
for f,t in text.items():
    for o in reach.get(f,[]):
        k=(o["manifest"],o["lib"])
        crate_files[k].add(f)
        if DEF.search(t): crate_def[k].add(f)
bad=[]
for k,fs in sorted(crate_files.items()):
    d=crate_def.get(k,set())
    nonroot=[f for f in fs if f not in d]
    print(k[1], "defs:",len(d),"users:",len(nonroot))
json.dump({"text_files":sorted(text), "defs":{k[1]:sorted(v) for k,v in crate_def.items()}}, open(G+"/dac-ta-plan.json","w"),ensure_ascii=False)
print(len(text),"files")
# files with no owner or no alias def in some owning crate
for f,t in text.items():
    owners=reach.get(f,[])
    if not owners: print("NOOWNER",f[-100:]); continue
    for o in owners:
        k=(o["manifest"],o["lib"])
        if not crate_def.get(k) and not DEF.search(t): print("NODEF",o["lib"],f[-90:], bool(MODDEF.search(t)))
