"""✅ F-04(a) dependency completeness: for every Rust file listed (one path per line) and each owner manifest of the file (reach), every `semio_framework_*` crate of the TARGETS used as a path head or in a `use` group must be a declared dependency key (normalised) of the owner. Read-only. usage: dep_check.py <files.txt> [--targets a,b]"""
import json, os, re, sys, collections
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade_lib import *
reach=json.load(open(G+"/rsl-reach.json"))
files=[l for l in open(sys.argv[1]).read().split("\n") if l]
TARGETS=["semio_framework_abi","semio_framework_action_bus","semio_framework_interaction","semio_framework_onboarding","semio_framework_time_travel_actions","semio_framework_tool_run_actions","semio_framework_io_schema","semio_framework_io_sqlite_snapshot","semio_framework_mesh_engine"]
if "--targets" in sys.argv: TARGETS=sys.argv[sys.argv.index("--targets")+1].split(",")
rx=re.compile(r'(?<![A-Za-z0-9_:])('+"|".join(TARGETS)+r')(?![A-Za-z0-9_])')
mkeys={}
def keys(m):
    if m not in mkeys:
        t=open(m,encoding="utf8").read()
        mkeys[m]={e["key"].replace("-","_") for e in dep_entries(t)}|{e["pkg"].replace("-","_") for e in dep_entries(t) if e["pkg"]==e["key"]}
    return mkeys[m]
missing=collections.defaultdict(set); n=0
for f in files:
    if not os.path.exists(f): continue
    t=open(f,encoding="utf8").read()
    used={m.group(1) for m in rx.finditer(re.sub(r'(?m)^\s*//.*$','',t))}
    if not used: continue
    n+=1
    for o in reach.get(f,[]):
        if o["lib"] in used: continue
        for u in used:
            if u not in keys(o["manifest"]): missing[o["manifest"]].add((u,f))
print("files checked",n,"manifests with a missing dependency",len(missing))
for m,s in sorted(missing.items())[:40]:
    print(" ",m[-90:],sorted({u for u,_ in s}),sorted({f for _,f in s})[:1])
