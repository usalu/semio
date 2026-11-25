"""🦴️ The drawn shapes (per view) and the body solids of the bones of an asset document: `python3 bones.py <document.json> <scale> <bone>…`, in painting order, with solid kind, role, z and key numbers; the last column says which volume solids the bone carries."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]; scale = sys.argv[2]
for bone in sys.argv[3:]:
    b = next(x for x in a["rig"]["bones"] if x["id"] == bone)
    print(f"## 1:{scale} {bone} L={b['lengthM']} parent={b.get('parent')} angle={b.get('angleDeg')}")
    for view, shapes in a["representations"][scale]["geometry"].items():
        for i, s in enumerate(shapes):
            if s.get("bone") != bone: continue
            k = s["kind"] + (":" + s["solid"]["kind"] + "/" + str(s["solid"].get("sweep", "")) if "solid" in s else "")
            extra = {k2: v for k2, v in s.get("solid", {}).items() if k2 not in ("kind", "sweep")}
            print(f"  {view:9} #{i:3} {s['id']:22} {k:22} role={s.get('role')} z={s.get('z')} filled={s.get('filled')} tone={s.get('tone')} region={s.get('region')} n={len(s.get('points', s.get('sections', [])))} {json.dumps(extra)[:150]}")
    for s in a["volumes"][scale].get("body", []):
        if s.get("bone") != bone: continue
        d = {k: v for k, v in s.items() if k not in ("id", "kind", "bone", "sections")}
        sec = s.get("sections")
        print(f"  BODY {s['id']:22} {s['kind']} n={len(sec) if sec else ''} {json.dumps(d)[:170]} {json.dumps([[*q['at'], q['rx'], q['ry']] for q in sec][::max(1, len(sec)//3)]) if sec else ''}")
