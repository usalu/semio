import json,sys
for l in open(sys.argv[1]):
    if l.startswith("{"):
        d=json.loads(l)
        if "retained" in d: print(d["label"][:16], "heap",d["heapUsed"],"ext",d["external"],"| rowStore",d["retained"]["rowStore"].get("bytes") if isinstance(d["retained"]["rowStore"],dict) else "", "| colls:", d.get("colls"))
