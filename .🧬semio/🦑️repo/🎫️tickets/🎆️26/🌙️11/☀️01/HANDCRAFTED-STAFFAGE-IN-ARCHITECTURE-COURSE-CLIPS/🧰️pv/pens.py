import json,glob,collections
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
ISO={0.13,0.18,0.25,0.35,0.5,0.7}
cnt=collections.Counter(); off=collections.defaultdict(list)
for f in sorted(glob.glob(f"{B}/*/*/🔣️.json")):
    d=json.load(open(f,encoding="utf-8"))["asset"]
    for k,R in d["representations"].items():
        if R["status"]!="authored": continue
        for role,v in R["pens"].items():
            if v is None: continue
            cnt[v]+=1
            if v not in ISO: off[(d["category"],d["id"])].append((k,role,v))
        for h in R["hatches"]:
            cnt[("h",h["penMm"])]+=1
        for v,ss in R["geometry"].items():
            for s in ss:
                if "penMm" in s: cnt[("shape",s["penMm"])]+=1; 
                if "penMm" in s and s["penMm"] not in ISO: off[(d["category"],d["id"])].append((k,s["id"],s["penMm"]))
print(sorted(cnt.items(),key=lambda x:str(x[0])))
for k,v in off.items(): print(k,v)
