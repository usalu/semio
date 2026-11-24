"""🪚️ Repair of `reclaimed-timber-roof-truss` at 1:50: the front elevation and the plan drew the bolts of each joint as one continuous strip beside the gusset (27 mm deep, up to 560 mm long), where the body has the single bolts with air between them (26.3 mm; a quarter of the strip was drawing without body). Each strip is replaced by the outlines of its bolts, taken from the bolt solids at their places in the default pose (washer, head or nut and shank end as they are seen from ahead and from above). Idempotent: python3 fix_reclaimed_timber_roof_truss.py"""
import math, re, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-timber-roof-truss"
text, doc = sc.load(ID)
rig = {b["id"]: b for b in doc["asset"]["rig"]["bones"]}


def world(bone, x, y):
    """🌍️ A point of a bone in the plane of the truss in the default pose (the bones of the truss are fixed)."""
    while bone is not None:
        b = rig[bone]
        a = math.radians(b.get("angleDeg", 0.0))
        x, y = b["x"] + x * math.cos(a) - y * math.sin(a), b["y"] + x * math.sin(a) + y * math.cos(a)
        bone = b.get("parent")
    return x, y


bolts = {}
for solid in sc.solids(doc, 50):
    m = re.fullmatch(r"bolt-([a-z])(\d+)-([lr])", solid["id"])
    if m:
        bolts.setdefault((m.group(1), m.group(3)), []).append((int(m.group(2)), solid))
assert sum(len(v) for v in bolts.values()) == 56
for view in ("front", "plan"):
    shapes = sc.shapes(doc, 50, view)
    for (joint, side), members in sorted(bolts.items()):
        strip = next((s for s in shapes if s["id"] == f"bolts-{joint}-{side}"), None)
        template = strip if strip is not None else next(s for s in shapes if s["id"] == f"bolts-{joint}-{side}-1")
        anchor = template["id"]
        for n, solid in sorted(members, key=lambda item: item[0]):
            cz = solid["origin"][2]
            cx, cy = world(solid["bone"], solid["origin"][0], solid["origin"][1])
            sign = 1 if solid["axis"][2] > 0 else -1
            chain = [(a, r) for a, r in solid["profile"]]
            upper = [(a, r) for a, r in chain if r > 0]
            ring = [(chain[0][0], 0.0)] + upper + [(chain[-1][0], 0.0)]
            both = upper + [(a, -r) for a, r in reversed(upper)]
            if view == "front":
                points = [[sc.r6(cz + sign * a), sc.r6(cy + r)] for a, r in both]
            else:
                points = [[sc.r6(cx + r), sc.r6(cz + sign * a)] for a, r in both]
            bolt = {k: v for k, v in template.items() if k not in ("points", "id")}
            bolt = {"id": f"bolts-{joint}-{side}-{n}", **bolt, "points": points, "closed": True}
            sc.put(shapes, bolt, before=anchor if strip is not None else None)
        if strip is not None:
            sc.drop(shapes, strip["id"])
sc.save(ID, text, doc)
