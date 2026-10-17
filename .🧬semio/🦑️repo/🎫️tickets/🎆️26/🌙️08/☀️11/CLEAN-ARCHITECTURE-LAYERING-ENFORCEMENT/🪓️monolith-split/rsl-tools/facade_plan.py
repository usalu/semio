"""📋️ Read-only planner for the F-04(a) facade re-pointing: finds every textual path that reaches an alias of the facade (`semio_framework::A`, or through a glob-leaking crate), the replacement, the owner manifests of the file and the crates the owners must depend on. Output JSON: 🗑️generated/rsl-plan.json plus a summary.
usage: facade_plan.py <alias,alias,...>"""
import json, os, re, subprocess, sys, collections
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
REAL={
 "abi":("semio_framework_abi","semio-framework-abi"),
 "action_bus":("semio_framework_action_bus","semio-framework-action-bus"),
 "interaction":("semio_framework_interaction","semio-framework-interaction"),
 "onboarding":("semio_framework_onboarding","semio-framework-onboarding"),
 "history_edit":("semio_framework_time_travel_actions","semio-framework-time-travel-actions"),
 "tool_run":("semio_framework_tool_run_actions","semio-framework-tool-run-actions"),
 "platform":("semio_framework_action_bus::platform","semio-framework-action-bus"),
 "ui":("semio_framework::manifest",None),
 "io_schema":("semio_framework_io_schema","semio-framework-io-schema"),
 "sqlite_snapshot":("semio_framework_io_sqlite_snapshot","semio-framework-io-sqlite-snapshot"),
 "mesh_io":("semio_framework_mesh_engine::io","semio-framework-mesh-engine"),
}
ALIASES=sys.argv[1].split(",")
PREF=["semio_framework","semio_framework_plugin","semio_framework_os_host"]
reach=json.load(open(G+"/rsl-reach.json"))
def ls(*a):
    return [p for p in subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z",*a],capture_output=True,text=True).stdout.split("\0") if p]
cands=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","grep","-l","-E","(semio_framework|semio_framework_plugin|semio_framework_os_host)::("+"|".join(ALIASES)+")([^A-Za-z0-9_]|$)","--","*.rs"],capture_output=True,text=True).stdout.split("\n")
oth=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","--others","--exclude-standard","-z","--","*.rs"],capture_output=True,text=True).stdout.split("\0")
files=sorted({f for f in cands+oth if f and not f.startswith(".🧬semio/") and os.path.isfile(f)})
rxs={a:re.compile(r'(?<![A-Za-z0-9_])('+"|".join(PREF)+r')::'+a+r'(?![A-Za-z0-9_])') for a in ALIASES}
plan=[]
for f in files:
    t=open(f,encoding="utf8").read()
    hits=[]
    for a in ALIASES:
        for m in rxs[a].finditer(t):
            nxt=t[m.end():m.end()+2]
            hits.append(dict(alias=a,pref=m.group(1),pos=m.start(),end=m.end(),auto=(nxt=="::"),ctx=t[max(0,m.start()-30):m.end()+30].replace("\n","\\n")))
    if not hits: continue
    owners=reach.get(f,[])
    plan.append(dict(file=f,hits=hits,owners=owners))
json.dump(plan,open(G+"/rsl-plan.json","w"),ensure_ascii=False)
cnt=collections.Counter(); files_by=collections.defaultdict(set); man=collections.defaultdict(set); noown=[]; manual=[]
for p in plan:
    if not p["owners"]: noown.append(p["file"])
    for h in p["hits"]:
        cnt[(h["pref"],h["alias"])]+=1; files_by[h["alias"]].add(p["file"])
        if not h["auto"]: manual.append((p["file"],h["ctx"]))
        for o in p["owners"]: man[h["alias"]].add(o["manifest"])
for k,n in sorted(cnt.items()): print(n,k)
print("files",len(plan),"no owner",len(noown)); 
for a in ALIASES: print(a,"files",len(files_by[a]),"manifests",len(man[a]))
print("all manifests",len({m for s in man.values() for m in s}))
print("manual",len(manual))
for f,c in manual: print("  MANUAL",f[-80:],c)
for f in noown: print("  NOOWNER",f)
