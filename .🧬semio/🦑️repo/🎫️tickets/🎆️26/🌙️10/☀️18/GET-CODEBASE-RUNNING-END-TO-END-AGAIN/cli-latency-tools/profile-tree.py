#!/usr/bin/env python3
"""Reads one SEMIO_REPO_PROFILE=json document (last JSON line of the file) and prints the phase tree with, per phase, the number and the total time of descendant child processes (spawn.*), so image opens can be priced by their hdiutil rounds.
usage: profile-tree.py file [--min-ms N]"""
import sys,json,collections
path=sys.argv[1];min_ms=float(sys.argv[sys.argv.index("--min-ms")+1]) if "--min-ms" in sys.argv else 0
doc=None
for line in open(path,errors="replace"):
    line=line.strip()
    if line.startswith("{") and '"rows"' in line:
        try: doc=json.loads(line)
        except Exception: pass
rows=doc["rows"];kids=collections.defaultdict(list)
for r in rows: kids[r["parent"]].append(r)
def spawns(r):
    n=0;t=0.0
    for c in kids[r["id"]]:
        if c["phase"].startswith("spawn."): n+=1;t+=c["duration"]
        a,b=spawns(c);n+=a;t+=b
    return n,t
def show(r,depth):
    if r["phase"].startswith("spawn."): return
    n,t=spawns(r)
    if r["duration"]>=min_ms or r["duration"]==0:
        print(f"{'  '*depth}{r['phase']:<{46-2*depth}} start {r['start']:8.1f} dur {r['duration']:8.1f} self {r['self']:8.1f}  spawns {n:3d} ({t:6.0f} ms)")
    for c in kids[r["id"]]: show(c,depth+1)
print(f"pid {doc.get('pid')} total {doc['total']} covered {doc['covered']} uncovered {doc['uncovered']}")
for r in kids[0]: show(r,0)
