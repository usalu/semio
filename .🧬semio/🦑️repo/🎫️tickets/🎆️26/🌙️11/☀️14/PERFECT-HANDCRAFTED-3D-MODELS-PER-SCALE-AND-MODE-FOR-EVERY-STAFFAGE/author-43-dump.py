import json,sys,re
cat={"city-bicycle":"city-bicycle","cargo-bike":"cargo-bike","motor-scooter":"motor-scooter"}
slug=sys.argv[1]; scs=sys.argv[2].split(","); views=sys.argv[3].split(",") if len(sys.argv)>3 else ["elevation","front","plan"]
d=json.load(open(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/{slug}/🔣️.json"))["asset"]
def bbox(s):
    k=s["kind"]
    if k=="rect": return (s["x"],s["y"],s["x"]+s["w"],s["y"]+s["h"])
    if k=="ellipse": return (s["cx"]-s["rx"],s["cy"]-s["ry"],s["cx"]+s["rx"],s["cy"]+s["ry"])
    if k=="capsule": 
        r=max(s["r0"],s["r1"]); return (min(s["x0"],s["x1"])-r,min(s["y0"],s["y1"])-r,max(s["x0"],s["x1"])+r,max(s["y0"],s["y1"])+r)
    if k=="poly": 
        xs=[p[0] for p in s["points"]];ys=[p[1] for p in s["points"]];return (min(xs),min(ys),max(xs),max(ys))
    if k=="path":
        n=[float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?",s["d"])]
        xs=n[0::2];ys=n[1::2];return (min(xs),min(ys),max(xs),max(ys))
    if k=="member": return (s["from"]["x"],s["from"]["y"],s["to"]["x"],s["to"]["y"])
    return None
for sc in scs:
    g=d["representations"][sc]["geometry"]
    print("######",sc,"pens",d["representations"][sc]["pens"])
    for v in views:
        print("##",v)
        for s in g[v]:
            b=bbox(s)
            extra=""
            if s["kind"]=="poly" and len(s["points"])<=4: extra=" pts="+json.dumps(s["points"])
            if s["kind"]=="capsule": extra=f" cap {s['x0']},{s['y0']}->{s['x1']},{s['y1']} r{s['r0']}"
            if s["kind"]=="member": extra=f" mem {s['from']}->{s['to']} {s['section']}"
            print(f"{s['id']:18s} {s['bone']:9s} {s['kind']:7s} {'F' if s.get('filled') else '-'} {s.get('role','')[:3]} pen={s.get('penMm','')} z={s.get('z')} bb="+(",".join(f"{x:.3f}" for x in b) if b else "")+extra)
