#!/usr/bin/env python3
"""Aggregates SEMIO_REPO_PROFILE=1 tab separated blocks (one per process) from files or stdin into a median table per phase.
usage: profile-agg.py [--pid-first] file...   (a block starts with #repo-profile; only the first block of a file is the CLI door unless --all)"""
import sys,statistics,collections
def blocks(text):
    out=[];cur=None
    for line in text.splitlines():
        if line.startswith("#repo-profile"):
            cur={"rows":[],"total":None};out.append(cur)
        elif cur is not None and line.startswith("#total"):
            cur["total"]=float(line.split("\t")[1])
        elif cur is not None and not line.startswith("#"):
            f=line.split("\t")
            if len(f)>=6: cur["rows"].append((f[2],int(f[1]),float(f[3]),float(f[4]),float(f[5]),f[6] if len(f)>6 else ""))
    return out
def main():
    files=[a for a in sys.argv[1:] if not a.startswith("--")]
    runs=[]
    for f in files:
        found=blocks(open(f,errors="replace").read())
        if found: runs.append(found[-1] if "--last" in sys.argv else found[0])
    if not runs: print("no profile blocks");return
    order=[];agg=collections.defaultdict(lambda:{"start":[],"dur":[],"self":[],"n":0,"depth":0})
    for r in runs:
        seen=collections.Counter()
        for (ph,dep,st,du,se,det) in r["rows"]:
            key=ph if not ph.startswith("spawn.") else ph
            if key not in agg or key not in order:
                if key not in order: order.append(key)
            a=agg[key];a["depth"]=dep;a["start"].append(st);a["dur"].append(du);a["self"].append(se);seen[key]+=1
        for k,v in seen.items(): agg[k]["n"]=max(agg[k]["n"],v)
    tot=[r["total"] for r in runs if r["total"]]
    print(f"runs={len(runs)} total median {statistics.median(tot):.0f} ms (min {min(tot):.0f}, max {max(tot):.0f})")
    print(f"{'phase':44} {'start':>8} {'dur':>8} {'self':>8} {'n':>3}")
    for k in order:
        a=agg[k];print(f"{'  '*a['depth']+k:44} {statistics.median(a['start']):8.1f} {statistics.median(a['dur']):8.1f} {statistics.median(a['self']):8.1f} {a['n']:3}")
main()
