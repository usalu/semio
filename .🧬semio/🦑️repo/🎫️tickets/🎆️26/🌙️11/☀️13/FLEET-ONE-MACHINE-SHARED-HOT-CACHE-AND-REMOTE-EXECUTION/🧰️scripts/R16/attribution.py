#!/usr/bin/env python3
"""R16 attribution of the 745 crates that show more than one miss key (recorded logs hold only part hashes, so the evidence is the local unit entries of each key: output names carry Cargo's metadata hash, output bytes are content hashes).
usage: attribution.py [--list N]"""
import json,glob,collections,re,os,sys
U="/Users/Shared/semio/.cache/b/r/m/u/"
by=collections.defaultdict(dict)
for f in glob.glob(U+"x/p*.jsonl"):
    for l in open(f,errors="ignore"):
        try:r=json.loads(l)
        except ValueError:continue
        if r["kind"]=="miss" and r.get("reason"):
            m=re.match(r"toolchain:(\w+) args:(\w+) source:(\w+) externs:(\w+) environment:(\w+)",r["reason"])
            if m:by[r["crate"]][r["key"]]=(m.groups(),r["lane"])
names=["toolchain","args","source","externs","env"]
def entry(k):
    d=U+"e/%s/%s"%(k[:2],k)
    if not os.path.isdir(d):return None
    return json.load(open(d+"/"+sorted(os.listdir(d))[-1]))
table=collections.defaultdict(list)
for crate,keys in by.items():
    if len(keys)<2:continue
    ents={k:entry(k) for k in keys}
    parts=[v[0] for v in keys.values()]
    differs={names[i] for i in range(5) if len({p[i] for p in parts})>1}
    if "source" in differs:cause="real input: source bytes changed"+(" (and environment)" if "env" in differs else "")
    elif any(e is None for e in ents.values()):cause="not attributable: entry already collected"
    else:
        outs={k:{o["name"]:o["sha256"] for o in e["outputs"] if not o.get("template")} for k,e in ents.items()}
        exts={k:tuple(sorted(os.path.splitext(n)[1] for n in v)) for k,v in outs.items()}
        nameset={tuple(sorted(v)) for v in outs.values()}
        if len(set(exts.values()))>1:cause="real variant: emit differs (check/metadata-only against build)"
        elif len(nameset)>1:cause="real variant: Cargo metadata hash differs (features, profile, incremental or dependency set)"
        else:
            groups=collections.defaultdict(set)
            for k,v in outs.items():
                for n,s in v.items():groups[n].add(s)
            if all(len(s)==1 for s in groups.values()):cause="noise: same metadata, same output bytes, only the command line differs"+(" (dependents follow through externs)" if "externs" in differs else "")
            else:cause="same metadata, output bytes differ between keys (nondeterministic or argument-dependent output)"
    table[cause].append(crate)
total=sum(len(v) for v in table.values())
print("multi-key crates:",total)
for cause,crates in sorted(table.items(),key=lambda x:-len(x[1])):print(len(crates),cause,sorted(crates)[:3])
if "--list" in sys.argv:
    n=int(sys.argv[sys.argv.index("--list")+1]);rows=[]
    for cause,crates in table.items():rows+= [(c,cause) for c in sorted(crates)]
    import random;random.Random(16).shuffle(rows)
    for c,cause in rows[:n]:print("  ",c.ljust(58),cause)
