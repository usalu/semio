"""🔎 Author 09 ruler: one short line per solid of a source list (kind, bone, key coordinates). Usage: python3 rework-09-dump.py <category/slug> <scale> <list> [id-substring]"""
import json, sys
src = json.load(open(f"volumes/{sys.argv[1]}.json", encoding="utf-8"))
v = src["volumes"][sys.argv[2]]
sub = sys.argv[4] if len(sys.argv) > 4 else ""
r = lambda a: [round(c, 3) for c in a]
for s in v[sys.argv[3]]:
    if sub not in s["id"]:
        continue
    k = s.get("kind", "of:" + s.get("of", "?"))
    g = ""
    if k == "capsule": g = f"{r(s['p0'])}->{r(s['p1'])} r {s['r0']},{s['r1']}"
    elif k == "box": g = f"size {s['size']} at {s.get('transform', {}).get('translation')} rot {s.get('transform', {}).get('rotation')}"
    elif k == "ellipsoid": g = f"radii {s['radii']} at {s.get('transform', {}).get('translation')}"
    elif k == "loft": g = f"{len(s['sections'])} sec {r(s['sections'][0]['at'])} rx{s['sections'][0]['rx']} ry{s['sections'][0]['ry']} .. {r(s['sections'][-1]['at'])} rx{s['sections'][-1]['rx']} ry{s['sections'][-1]['ry']}"
    elif k == "revolve": g = f"origin {s.get('origin')} axis {s.get('axis')} rmax {max(p[1] for p in s['profile'])} ax {min(p[0] for p in s['profile'])}..{max(p[0] for p in s['profile'])}"
    elif k == "extrude": g = f"{len(s['profile'])} pts depth {s.get('depth')} {s.get('transform')}"
    extra = " ".join(f"{a}={s[a]}" for a in ("region", "layer", "member", "part", "material", "sectionFaces", "texture", "slender") if a in s)
    print(f"{s['id']:<26} {k:<9} {s.get('bone', ''):<9} {g} | {extra}")
