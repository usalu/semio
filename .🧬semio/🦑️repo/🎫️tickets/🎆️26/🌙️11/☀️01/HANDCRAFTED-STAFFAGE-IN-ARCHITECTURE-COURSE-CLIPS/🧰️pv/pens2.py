import json,glob,collections
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ISO={0.13,0.18,0.25,0.35,0.5,0.7}
cnt=collections.Counter(); off=collections.defaultdict(list); bycat=collections.defaultdict(collections.Counter)
for f in sorted(glob.glob(f"{B}/*/*/🔣️.json")):
    d=json.load(open(f,encoding="utf-8"))["asset"]
    for k,R in d["representations"].items():
        if R["status"]!="authored": continue
        for role,v in R["pens"].items():
            if v is None: continue
            cnt[v]+=1; bycat[d["category"]][v]+=1
            if v not in ISO: off[d["id"]].append((k,role,v))
print(sorted(cnt.items()))
for c,v in bycat.items(): print(c,sorted(v.items()))
for k,v in off.items(): print(k,v)
