import json,sys,collections
out=collections.defaultdict(list)
fails=[]
for l in open(sys.argv[1]):
    if not l.startswith("{"): continue
    try:e=json.loads(l)
    except: continue
    t=e.get("Test")
    if e.get("Action")=="output":
        out[(e.get("Package"),t)].append(e["Output"])
    if e.get("Action")=="fail" and t and "/" not in t: fails.append((e["Package"],t))
for p,t in fails:
    print("==",t); print("".join(out[(p,t)])[:int(sys.argv[2]) if len(sys.argv)>2 else 1200])
print("pkg-level:", "".join(out[(fails[0][0] if fails else None,None)])[:800] if fails else "")
