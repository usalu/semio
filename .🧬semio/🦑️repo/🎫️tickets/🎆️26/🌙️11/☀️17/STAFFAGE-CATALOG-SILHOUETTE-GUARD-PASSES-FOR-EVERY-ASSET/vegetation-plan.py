import json,sys,re
CAT="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/"
slug,tr=sys.argv[1],sys.argv[2]
a=json.load(open(CAT+slug+"/🔣️.json"))["asset"]
T=json.load(open(tr))
def bbox(s):
    k=s["kind"]
    if k=="path":
        n=[float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?",s["d"])]
        xs=n[0::2]; ys=n[1::2]
    elif k=="poly":
        xs=[p[0] for p in s["points"]]; ys=[p[1] for p in s["points"]]
    elif k=="rect": xs=[s["x"],s["x"]+s["w"]]; ys=[s["y"],s["y"]+s["h"]]
    elif k=="ellipse": xs=[s["cx"]-s["rx"],s["cx"]+s["rx"]]; ys=[s["cy"]-s["ry"],s["cy"]+s["ry"]]
    else: return None
    return (max(xs)-min(xs))*(max(ys)-min(ys))
for sc,views in T.items():
    for view,bones in views.items():
        if "section" in bones: print(sc,view,"SECTION"); continue
        geo=a["representations"][sc]["geometry"][view]
        for bone,t in bones.items():
            cands=[s for s in geo if s["bone"]==bone and s.get("filled") is True]
            npts=sum(len(r.split(";")) for r in t["rings"])
            if not cands: print(sc,view,bone,"NO DRAWN FILLED; traced area",t["area"],"rings",len(t["rings"]),"pts",npts); continue
            main=max(cands,key=lambda s:bbox(s) or 0)
            print(sc,view,bone,"main",main["id"],main["kind"],round(bbox(main) or 0,3),"others",[c["id"] for c in cands if c is not main],"traced",t["area"],"rings",len(t["rings"]),"pts",npts)
