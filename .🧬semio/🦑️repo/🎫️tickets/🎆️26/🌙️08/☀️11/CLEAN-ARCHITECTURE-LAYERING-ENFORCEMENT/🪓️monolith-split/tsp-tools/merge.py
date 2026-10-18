import json,sys,re
F=sys.argv[1]; out=sys.argv[2]; dirs=sys.argv[3:]
cfg=set(l.strip() for l in open(F+"/tsp-tools/vitest-configs.txt"))
m={}
for d in dirs:
    for x in json.load(open(F+"/🗑️generated/"+d+"/results.json")): m[x["path"]]=x
res=[x for p,x in m.items() if p not in cfg]
json.dump(res,open(out,"w"),ensure_ascii=False)
import collections
print(len(res),collections.Counter(x["status"] for x in res),sum(x["pass_"] for x in res),sum(x["fail"] for x in res))
