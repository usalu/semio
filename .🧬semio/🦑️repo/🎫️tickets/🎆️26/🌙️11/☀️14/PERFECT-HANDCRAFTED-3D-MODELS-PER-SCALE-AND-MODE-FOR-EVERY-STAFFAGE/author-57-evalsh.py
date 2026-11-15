import json,subprocess,re,sys,importlib.util,math
spec=importlib.util.spec_from_file_location('s','author-57-shower.py'); m=importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
def run(sc, body, rules="volumes-silhouette"):
    d=json.load(open("volumes/appliances/shower-head.json"))
    v=d["volumes"][str(sc)]
    v["body"]=[s for s in v["body"] if not s["id"].startswith("hose-")]+body
    json.dump(d,open("🗑️generated/author-57/sh.tmp.json","w"))
    out=subprocess.run(["bash","-c","$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1) volumes-check.ts appliances/shower-head --product --scale %d --rules %s --source 🗑️generated/author-57/sh.tmp.json"%(sc,rules)],capture_output=True,text=True).stdout
    return re.findall(r"(?:iou|p95M):\w+=[0-9.]+",out)
def allthin(sc, per, r):
    out=[]; n=3*per
    pts=[m.pt(sc,3*i/n) for i in range(n+1)]
    for k in range(n):
        d=m.L.cap(f"hose-{k:02d}","elbow","epdm-hose","body",0.55,pts[k],pts[k+1],r)
        f=k/n
        if f>=0.9167: d["weights"]=[{"bone":"shower","weight":1}]
        elif f>=0.8333: d["weights"]=[{"bone":"elbow","weight":0.4},{"bone":"shower","weight":0.6}]
        elif f>=0.75: d["weights"]=[{"bone":"elbow","weight":0.7},{"bone":"shower","weight":0.3}]
        out.append(d)
    return out
if __name__=="__main__":
    for sc,per,r in ((100,10,0.0077),(100,14,0.0077),(200,6,0.015),(200,10,0.015)):
        print(sc,per,r,run(sc,allthin(sc,per,r)))
