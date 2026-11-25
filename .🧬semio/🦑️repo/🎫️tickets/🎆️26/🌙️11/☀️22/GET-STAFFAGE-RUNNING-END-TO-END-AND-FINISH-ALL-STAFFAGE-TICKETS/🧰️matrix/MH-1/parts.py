"""🔎️ What a bone draws and what its body is, at one scale: `python3 parts.py <doc.json> <scale> <bone>…` prints per view the shapes of the bone (id, kind, points, solid, extents of the points) and the body solids of the volume on the bone."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]; s = sys.argv[2]
def ext(p):
    return " ".join(f"[{min(c):.3f},{max(c):.3f}]" for c in zip(*p)) if p else ""
for b in sys.argv[3:]:
    bone = next(x for x in a["rig"]["bones"] if x["id"] == b)
    print("==", {k: bone.get(k) for k in ("id","parent","x","y","z","angleDeg","abductionDeg","twistDeg","lengthM")})
    for v, l in a["representations"][s]["geometry"].items():
        for sh in l:
            if sh.get("bone") == b: print(f"  {v:9} {sh['id']:22} {sh['kind']:7} {len(sh.get('points', sh.get('sections', [])))} {ext(sh.get('points'))} z {sh.get('z')} role {sh.get('role')} solid {json.dumps(sh.get('solid'))} {'follow' if sh.get('followTip') else ''} w {sh.get('weights')}")
    for so in a["volumes"][s]["body"]:
        if so.get("bone") == b or any(w["bone"] == b for w in so.get("weights", [])):
            print("  body", so["id"], so["kind"], json.dumps({k: v for k, v in so.items() if k not in ("id","kind","bone","material","role","tone","texture","region")})[:1000])
