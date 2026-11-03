import json,sys
d=json.load(open(sys.argv[1],encoding="utf-8"))
for k,v in d["clips"].items(): print(k, len(v))
print("unused", d["unused"])
