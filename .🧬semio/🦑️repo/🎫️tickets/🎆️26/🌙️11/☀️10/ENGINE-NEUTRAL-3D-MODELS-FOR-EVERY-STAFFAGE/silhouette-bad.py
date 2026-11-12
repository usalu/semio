import json,sys
rows=[json.loads(l) for l in open(sys.argv[1])]
bad=[]
for r in rows:
    if "error" in r: print("ERR",r); continue
    f=[]
    if any(abs(x)>0.08 for x in r["bboxError"]): f.append("bbox "+"/".join(f"{x*100:.1f}" for x in r["bboxError"]))
    for v,m in r["views"].items():
        if m["iou"]<0.8: f.append(f"{v} {m['iou']} r{m['recall']} p{m['precision']}")
    if f: bad.append((r["asset"],f))
print(len(rows),len(bad))
for b in bad: print(b)
