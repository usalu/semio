import json,sys,re
ROOT=r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage\🗂️catalog\👤️humans"
OUT=r"C:\git\semio\.🧬semio\🦑️repo\🎫️tickets\🎆️26\🌙️11\☀️01\TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE\🗑️generated\H3"
def bbox(s):
    k=s["kind"]
    if k=="capsule": return (min(s["x0"]-s["r0"],s["x1"]-s["r1"]),min(s["y0"]-s["r0"],s["y1"]-s["r1"]),max(s["x0"]+s["r0"],s["x1"]+s["r1"]),max(s["y0"]+s["r0"],s["y1"]+s["r1"]))
    if k=="ellipse": return (s["cx"]-s["rx"],s["cy"]-s["ry"],s["cx"]+s["rx"],s["cy"]+s["ry"])
    if k=="rect": return (s["x"],s["y"],s["x"]+s["w"],s["y"]+s["h"])
    if k=="poly":
        xs=[p[0] for p in s["points"]];ys=[p[1] for p in s["points"]];return (min(xs),min(ys),max(xs),max(ys))
    nums=[float(n) for n in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?",s["d"])]
    xs=nums[0::2];ys=nums[1::2];return (min(xs),min(ys),max(xs),max(ys))
L=[]
for a in sys.argv[1:]:
    d=json.load(open(f"{ROOT}\\{a}\\🔣️.json",encoding="utf8"))["asset"]
    L.append(f"##### {a}")
    for sc in ("50","100","200","500"):
        r=d["representations"].get(sc) or d["representations"].get(int(sc))
        if not r or "geometry" not in r: continue
        L.append(f"== {sc}")
        for s in r["geometry"]["elevation"]:
            b=bbox(s)
            extra={k:s[k] for k in ("hatch","material","regions","closed") if k in s}
            L.append(f"{s['id']}|{s['bone']}|{s['kind']}|{s['role']}|z{s['z']}|t{s.get('tone')}|{s.get('region')}|{'F' if s.get('filled') else 'S'}|[{b[0]:.3f},{b[1]:.3f},{b[2]:.3f},{b[3]:.3f}]|{extra if extra else ''}")
open(f"{OUT}\\styles.txt","w",encoding="utf8").write("\n".join(L))
print("ok")
