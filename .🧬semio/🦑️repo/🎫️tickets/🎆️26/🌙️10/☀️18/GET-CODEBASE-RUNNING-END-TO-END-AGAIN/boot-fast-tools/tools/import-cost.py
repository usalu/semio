#!/usr/bin/env python3
"""import-cost.py trace.json [--top N] [--json out] : import graph cost table (module, self ms, total ms, bytes, eager, first importer) from import-trace.ts output."""
import json,os,sys,collections
a=sys.argv[1:];tr=json.load(open(a[0]));top=int(a[a.index("--top")+1]) if "--top" in a else 25
ev=tr["events"];by={e["path"]:e for e in ev}
def resolve(frm,spec):
    if not spec.startswith("."):return None
    p=os.path.normpath(os.path.join(os.path.dirname(frm),spec));return p if p in by else None
ended=sorted([e for e in ev if e["endMs"] is not None],key=lambda e:e["endMs"])
prev=None
for e in ended:
    e["self"]=e["endMs"]-(prev["endMs"] if prev else e["endMs"]-0.0);prev=e
if ended:ended[0]["self"]=0.0
children={e["path"]:[] for e in ev};first_importer={}
for e in ended:
    for s in e["static"]:
        c=resolve(e["path"],s)
        if c and c not in first_importer and by[c]["endMs"] is not None and by[c]["endMs"]<=e["endMs"] and c!=e["path"]:
            first_importer[c]=e["path"];children[e["path"]].append(c)
tot={}
def total(p):
    if p in tot:return tot[p]
    tot[p]=0;t=by[p].get("self",0)+sum(total(c) for c in children[p]);tot[p]=t;return t
for e in ended:total(e["path"])
def short(p,n=3):return "/".join(p.split("/")[-n:])
rows=[(short(e["path"]),round(e.get("self",0),1),round(tot[e["path"]],1),e["bytes"],"static" if e["path"] in first_importer or e is ended[-1] else "dynamic") for e in ended]
print(f"modules={len(ev)} bytes={sum(e['bytes'] for e in ev)} sum-self={sum(e.get('self',0) for e in ended):.0f}ms exit={tr['exitMs']:.0f}ms marks={[(m['name'],round(m['t'])) for m in tr['marks']]}")
print("\nTOP SELF");print(f"{'self':>8} {'total':>8} {'bytes':>8}  kind    module")
for r in sorted(rows,key=lambda r:-r[1])[:top]:print(f"{r[1]:8.1f} {r[2]:8.1f} {r[3]:8d}  {r[4]:7} {r[0]}")
print("\nTOP TOTAL (subtree)")
for r in sorted(rows,key=lambda r:-r[2])[:top]:print(f"{r[1]:8.1f} {r[2]:8.1f} {r[3]:8d}  {r[4]:7} {r[0]}")
lstat=collections.Counter(c["path"] for c in tr["calls"] if c["api"]=="lstatSync")
print("\nREDUNDANT fs (same path, same api, count>=4):");pc=collections.Counter((c["api"],c["path"]) for c in tr["calls"])
for (api,p),n in pc.most_common(8):
    if n>=4:print(f"{n:5d}x {api:12} {short(p,4)}")
print("fs calls",len(tr["calls"]),"ms",round(sum(c["ms"] for c in tr["calls"]),1))
if "--json" in a:json.dump({"rows":rows,"marks":tr["marks"],"exitMs":tr["exitMs"]},open(a[a.index("--json")+1],"w"))
