"""🧾 One line per solid of the volumes source of an asset: `python3 rework-machinery-summary.py <category>/<slug> <scale> [body|thermal|structure …]` (kind, profile box, placement, flags; W=<columns> widens the lines)."""
import json,sys
p,sc,*lists=sys.argv[1:]
d=json.load(open("volumes/"+p+".json"))["volumes"][sc]
def r(v): return round(v,4)
def g(s):
    k=s.get("kind")
    o=[]
    if "of" in s: o.append("of="+s["of"])
    if k in("extrude","revolve"):
        pr=s["profile"];xs=[q[0] for q in pr];ys=[q[1] for q in pr]
        o.append(f"{k} P[{r(min(xs))}..{r(max(xs))} x {r(min(ys))}..{r(max(ys))} n{len(pr)}]")
        if k=="extrude": o.append(f"{s.get('plane')} {s.get('from')}..{s.get('to')}"+(" holes" if "holes" in s else ""))
        else: o.append(f"axis{s.get('axis')} o{s.get('origin')}")
    elif k=="box": o.append(f"box {s['size']}")
    elif k=="capsule": o.append(f"capsule {s['p0']}->{s['p1']} r{s['r0']}")
    elif k=="ellipsoid": o.append(f"ell {s['radii']}")
    elif k: o.append(k)
    for kk in ("transform","repeat","member","members","part","region","layer","slender","sheet","envelope","texture","anchor","tags","sectionFaces","taperTo"):
        if kk in s: o.append(f"{kk}={json.dumps(s[kk])}")
    return " ".join(o)
for l in lists or ("body","thermal","structure"):
    for i,s in enumerate(d[l]): print(l[0],i,s["id"],s.get("bone",""),g(s)[:int(__import__('os').environ.get('W','230'))])
if not lists:
    for x in d["decisions"]: print("D",x["en"][:400])
