"""🔍️ Audits the drawn `member` shapes of an asset against the bar solids of the same name (the id itself, or the id without a copy suffix): prints every member whose end points or section differ from its solid by more than a millimetre, and the members that have no solid. Reads an asset file with optional ops applied (solid and shape replacements by id): python3 audit.py <asset.json> [ops.json] [scales]"""
import json, math, re, sys

asset = json.load(open(sys.argv[1]))["asset"]
ops = json.load(open(sys.argv[2])) if len(sys.argv) > 2 and sys.argv[2] != "-" else []
scales = (sys.argv[3] if len(sys.argv) > 3 else "50,100,200,500").split(",")
for o in ops:
    if o["op"] == "solid":
        body = asset["volumes"][o["scale"]]["body"]
        body[[s["id"] for s in body].index(o["id"])] = o["solid"]
    elif o["op"] == "shape":
        g = asset["representations"][o["scale"]]["geometry"][o["view"]]
        g[[s["id"] for s in g].index(o["id"])] = o["shape"]
    elif o["op"] == "shape-add":
        g = asset["representations"][o["scale"]]["geometry"][o["view"]]
        ids = [s["id"] for s in g]
        if o["shape"]["id"] in ids:
            g[ids.index(o["shape"]["id"])] = o["shape"]
        else:
            g.append(o["shape"])


def bars(solid):
    """🪵️ The bars of a solid as (from, to, section) per copy, empty when it is no straight bar."""
    out = []
    if solid["kind"] == "revolve" and len(solid["profile"]) == 2 and solid["profile"][0][1] == solid["profile"][1][1] and "transform" not in solid:
        (l0, r), (l1, _) = solid["profile"]
        base = ([o + a * l0 for o, a in zip(solid["origin"], solid["axis"])], [o + a * l1 for o, a in zip(solid["origin"], solid["axis"])], ("circle", 2 * r, 2 * r))
    elif solid["kind"] == "box" and set(solid.get("transform", {})) <= {"translation"}:
        t = solid.get("transform", {}).get("translation", [0, 0, 0])
        k = solid["size"].index(max(solid["size"]))
        w, h = [(solid["size"][1], solid["size"][2]), (solid["size"][2], solid["size"][0]), (solid["size"][1], solid["size"][0])][k]
        base = ([v - (solid["size"][k] / 2 if i == k else 0) for i, v in enumerate(t)], [v + (solid["size"][k] / 2 if i == k else 0) for i, v in enumerate(t)], ("rect", w, h))
    else:
        return out
    rep = solid.get("repeat", {"count": 1, "translation": [0, 0, 0]})
    if "rotationRad" in rep:
        return out
    for c in range(rep["count"]):
        step = [v * c for v in rep.get("translation", [0, 0, 0])]
        out.append(([a + s for a, s in zip(base[0], step)], [a + s for a, s in zip(base[1], step)], base[2]))
    return out


for scale in scales:
    solids = {s["id"]: s for s in asset["volumes"][scale]["body"]}
    for view in ("elevation", "front", "plan"):
        bad, orphan = [], []
        for shape in asset["representations"][scale]["geometry"][view]:
            if shape["kind"] != "member":
                continue
            m = re.fullmatch(r"(.*?)-(\d+)", shape["id"])
            solid, copy = (solids[shape["id"]], 0) if shape["id"] in solids else (solids[m[1]], int(m[2]) - 1) if m and m[1] in solids else (None, 0)
            if solid is None:
                orphan.append(shape["id"])
                continue
            list_ = bars(solid)
            if copy >= len(list_):
                orphan.append(shape["id"] + "(no bar)")
                continue
            a, b, (kind, w, h) = list_[copy]
            p, q = [shape["from"][k] for k in "xyz"], [shape["to"][k] for k in "xyz"]
            ends = min(max(math.dist(p, a), math.dist(q, b)), max(math.dist(p, b), math.dist(q, a)))
            sec = shape["section"]
            size = (sec["d"], sec["d"]) if sec["kind"] == "circle" else (sec["w"], sec["h"])
            off = max(abs(size[0] - w), abs(size[1] - h)) if sec["kind"] == kind else 9
            if ends > 0.001 or off > 0.001:
                bad.append(f"{shape['id']} ends {ends * 1000:.0f} mm section {sec['kind']} {size[0]}x{size[1]} / {kind} {w:g}x{h:g}")
        print(f"1:{scale} {view}: {len(bad)} differ, {len(orphan)} without a bar solid")
        for line in bad[: int(__import__('os').environ.get('N', 12))]:
            print("   ", line)
        if orphan:
            print("    no solid:", " ".join(orphan[:30]))
