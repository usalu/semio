import json, sys, collections
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
ca = collections.Counter(x["id"] for x in a); cb = collections.Counter(x["id"] for x in b)
print(f"total {len(a)} -> {len(b)}")
for k in sorted(set(ca) | set(cb), key=lambda k: cb[k] - ca[k]):
    if ca[k] != cb[k] or len(sys.argv) > 3: print(f"{cb[k]-ca[k]:+6d} {ca[k]:6d} -> {cb[k]:6d}  {k}")
