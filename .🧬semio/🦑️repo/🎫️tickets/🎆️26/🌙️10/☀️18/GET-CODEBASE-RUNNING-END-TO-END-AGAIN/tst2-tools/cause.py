import json,re,sys,collections
rows=[json.loads(l) for l in open(sys.argv[1],encoding="utf-8")]
groups=collections.defaultdict(list)
for r in rows:
    if r["rc"]==0: continue
    t=open(r["log"],encoding="utf-8").read()
    m=re.search(r"Cannot find (?:package|module) '([^']+)'",t)
    if m: key="MISSING "+m.group(1).split("/")[0] if not m.group(1).startswith("@") else "MISSING "+"/".join(m.group(1).split("/")[:2]); groups[key].append(r["file"]); continue
    m=re.search(r"^error: (.{0,90})",t,re.M)
    key=("ERR "+m.group(1)) if m else "OTHER"
    groups[key].append(r["file"])
for k,v in sorted(groups.items(),key=lambda x:-len(x[1])):
    print(len(v),k)
    if len(sys.argv)>2:
        for f in v: print("    ",f[-100:])
