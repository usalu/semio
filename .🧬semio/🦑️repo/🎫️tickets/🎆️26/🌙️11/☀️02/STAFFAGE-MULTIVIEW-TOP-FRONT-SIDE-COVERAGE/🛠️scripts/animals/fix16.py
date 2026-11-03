import json
C = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/"
def save(p, d): open(p, "w", encoding="utf-8", newline="\n").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
p = C + "labrador-dog/🔣️.json"; d = json.load(open(p, encoding="utf-8")); a = d["asset"]
bones = a["rig"]["bones"]
if not any(b["id"] == "head-core" for b in bones):
    i = [b["id"] for b in bones].index("head")
    bones.insert(i + 1, {"id": "head-core", "parent": "head", "title": {"en": "Head axis (centre line of the skull for the front and top solids)", "de": "Kopfachse (Mittellinie des Schädels für die Körper in Vorder- und Draufsicht)"}, "x": 0, "y": 0.019, "angleDeg": 0, "lengthM": 0.23, "joint": {"type": "fixed"}})
n = 0
for sc, r in a["representations"].items():
    if r["status"] != "authored": continue
    for v in ("front", "plan"):
        for s in r["geometry"][v]:
            if s["id"] == "head" and s["bone"] == "head" and s.get("solid", {}).get("kind") == "revolved":
                s["bone"] = "head-core"; s["solid"]["axisBone"] = "head-core"; n += 1
save(p, d); print("dog head-core", n)
p = C + "horse/🔣️.json"; d = json.load(open(p, encoding="utf-8")); a = d["asset"]
GEOM = ("kind", "points", "closed", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d")
n = 0
for sc, r in a["representations"].items():
    if r["status"] != "authored": continue
    g = r["geometry"]
    elev = {e["id"]: e for e in g["elevation"] if e["bone"] == "head" and e["role"] == "body"}
    for v, wh, wm in (("front", 0.09, 0.052), ("plan", 0.095, 0.052)):
        out = []
        for s in g[v]:
            if s["id"] == "head" and s["bone"] == "head" and s.get("solid", {}).get("kind") == "revolved":
                for eid, w in (("head", wh), ("muzzle", wm)):
                    if eid in elev and (eid == "head" or sc in ("50", "100")):
                        e = elev[eid]
                        t = {k: s[k] for k in ("id", "bone", "role", "z", "filled", "tone", "region", "regions", "material") if k in s}
                        t["id"] = eid; t.update({k: e[k] for k in GEOM if k in e}); t["solid"] = {"kind": "extruded", "z0": -w, "z1": w}
                        out.append(t); n += 1
            else: out.append(s)
        g[v] = out
save(p, d); print("horse head extruded", n)
