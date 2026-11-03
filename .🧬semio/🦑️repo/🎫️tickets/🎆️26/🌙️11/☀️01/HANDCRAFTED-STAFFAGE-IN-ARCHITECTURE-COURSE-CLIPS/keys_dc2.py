import json, collections, sys
d = json.load(open(sys.argv[1], encoding="utf-8"))
c = collections.Counter(k for v in d["casts"].values() for k in v)
print(len(d["casts"]), dict(c))
