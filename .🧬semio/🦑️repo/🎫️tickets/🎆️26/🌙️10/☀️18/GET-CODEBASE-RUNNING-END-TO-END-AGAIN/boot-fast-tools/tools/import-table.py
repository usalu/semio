#!/usr/bin/env python3
"""import-table.py TRACE.json ISOLATE.json [--top N] [--groups GROUPS.json] : the import graph cost table. TRACE.json comes from import-trace.ts (loaded modules with their static import lists), ISOLATE.json from import-isolate.ts run on the same modules in end order (per module wall ms of its own import with its dependencies already loaded: load + transpile cache + top-level evaluation). Prints markdown: totals, per group counts, the top modules by self ms and by subtree (total) ms, the eager (static) and lazy (dynamic) split."""
import json,os,sys,collections
a=sys.argv[1:];tr=json.load(open(a[0]));iso=json.load(open(a[1]));top=int(a[a.index("--top")+1]) if "--top" in a else 15
groups=json.load(open(a[a.index("--groups")+1]))["groups"] if "--groups" in a else []
ev={e["path"]:e for e in tr["events"]};ms={r["path"]:r["ms"] for r in iso["rows"]}
ended=sorted([e for e in tr["events"] if e["endMs"] is not None],key=lambda e:e["endMs"])
def res(frm,s):
    if not s.startswith("."):return None
    p=os.path.normpath(os.path.join(os.path.dirname(frm),s));return p if p in ev else None
first={};children=collections.defaultdict(list)
for e in ended:
    for s in e["static"]:
        c=res(e["path"],s)
        if c and c not in first and ev[c]["endMs"]<=e["endMs"] and c!=e["path"]:first[c]=e["path"];children[e["path"]].append(c)
entry=ended[-1]["path"]
eager=set();todo=[entry]
while todo:
    p=todo.pop()
    if p in eager:continue
    eager.add(p);todo+= [c for s in ev[p]["static"] if (c:=res(p,s))]
selfms={p:ms.get(p,0.0) for p in ev};tot={}
def total(p):
    if p in tot:return tot[p]
    tot[p]=0;tot[p]=selfms[p]+sum(total(c) for c in children[p]);return tot[p]
for p in ev:total(p)
def short(p,n=3):return "/".join(p.split("/")[-n:])
print(f"modules {len(ev)} ({len(eager)} eager static, {len(ev)-len(eager)} lazy dynamic), source {sum(e['bytes'] for e in ev.values())/1024:.0f} KiB, sum of own import times {sum(selfms.values()):.0f} ms (warm transpile cache, host load applies)")
if groups:
    print("\n| group | modules | KiB | own import ms |\n|---|---:|---:|---:|")
    for g in groups:
        sel=[p for p in ev if any(m in p for m in g["match"])]
        if sel:print(f"| {g['id']} | {len(sel)} | {sum(ev[p]['bytes'] for p in sel)/1024:.0f} | {sum(selfms[p] for p in sel):.1f} |")
print("\n| module | self ms | total ms | KiB | kind |\n|---|---:|---:|---:|---|")
for p in sorted(ev,key=lambda p:-selfms[p])[:top]:print(f"| {short(p)} | {selfms[p]:.1f} | {tot[p]:.1f} | {ev[p]['bytes']/1024:.1f} | {'eager' if p in eager else 'lazy'} |")
print("\n| module (by subtree) | total ms | self ms |\n|---|---:|---:|")
for p in sorted(ev,key=lambda p:-tot[p])[:top//2]:print(f"| {short(p)} | {tot[p]:.1f} | {selfms[p]:.1f} |")
