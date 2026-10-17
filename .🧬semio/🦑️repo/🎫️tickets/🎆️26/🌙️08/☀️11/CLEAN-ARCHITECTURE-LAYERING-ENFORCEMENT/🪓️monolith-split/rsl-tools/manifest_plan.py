"""📋️ Read-only: from rsl-plan.json (zones applied) compute, per owner manifest, the crates to add and the anchor line (the facade, plugin or host dependency line whose section/style the new line copies). Output rsl-manifest-plan.json."""
import json, os, re, sys, collections
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from facade_lib import *
plan=json.load(open(G+"/rsl-plan.json"))
need=collections.defaultdict(lambda: collections.defaultdict(set))
for p in plan:
    f=p["file"]
    if in_skip_zone(f): continue
    for h in p["hits"]:
        pkg=PKG.get(h["alias"])
        if not pkg: continue
        for o in p["owners"]:
            need[o["manifest"]][pkg].add((h["pref"],f))
out={}
problems=[]
for m,d in sorted(need.items()):
    if in_skip_zone(m): problems.append(("manifest in skip zone",m,sorted(d))); continue
    if m==FACADE_MANIFEST: continue
    text=open(m,encoding="utf8").read()
    ents=dep_entries(text)
    byname={}
    for e in ents: byname.setdefault(e["pkg"],[]).append(e)
    row={}
    for pkg,srcs in d.items():
        prefs={s[0] for s in srcs}
        anchors=[]
        for pref in prefs:
            apkg={"semio_framework":"semio-framework","semio_framework_plugin":"semio-framework-plugin","semio_framework_os_host":"semio-framework-os-host"}[pref]
            anchors+= byname.get(apkg,[])
        have=byname.get(pkg,[])
        row[pkg]=dict(prefs=sorted(prefs),anchors=[(a["section"],a["key"],a["style"]) for a in anchors],have=[(a["section"],a["key"],a["style"]) for a in have],files=sorted({s[1] for s in srcs}))
        if not anchors: problems.append(("no anchor",m,pkg,sorted(prefs)))
    out[m]=row
json.dump(dict(manifests=out,problems=problems),open(G+"/rsl-manifest-plan.json","w"),ensure_ascii=False)
print("manifests",len(out)); print("problems",len(problems))
for p in problems: print(" ",p)
sty=collections.Counter()
for m,row in out.items():
    for pkg,r in row.items():
        for a in r["anchors"]: sty[(a[0],a[2])]+=1
        if r["have"]: sty[("HAVE",tuple(x[0] for x in r["have"]))]+=1
for k,v in sty.most_common(): print(v,k)
