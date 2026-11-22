"""🩹 Multi-stem birch 2D: two crown pieces of one bone that stand closer than the narrowest printable gap of their band (0.15 mm of white between two pens at the coarse end of the band) are drawn touching: the smaller piece is shifted towards the larger one until they overlap; the same holds for two bark pieces of one bone; the three stems of 1:500 are drawn a little stouter with blunt tips so that they print where the veil shows them, as a draughtsman closes a slit the printer would fill. Larger gaps stay open. Crown pieces are shifted in the elevation only: in front and plan the islands of a bone are one shape. The bark shapes stay exactly as measured (taken back from the state before), except the three top branches of 1:200 that leave the centre leader side by side: the outer two are drawn joined to the middle one at their base where they are nearer than the pens, and the third is the second island of the shape of the middle one, because two filled slivers 0.16 m apart under the body pen leave no printable white at 1:350. Run once after `rework-multi-stem-birch-2d.py` (a second run takes the bark back and would lose the island). Usage: python3 rework-multi-stem-birch-2d-gaps.py [--write]"""
import json
import math
import re
import sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/multi-stem-birch/🔣️.json"
INK_GAP_MM = 0.16
BARK_AS_MEASURED = ("50", "100", "200", "500")
BARK_JOINS = {"200": [("branch-tc3", "branch-tc4"), ("branch-tc5", "branch-tc4")]}
BARK_ONE_SHAPE = {"200": [("branch-tc5", "branch-tc4")]}
BARK_OVERLAP_M = 0.06
BARK_REACH_M = 0.12
BEFORE = __file__.replace("rework-multi-stem-birch-2d-gaps.py", "🗑️generated/rework-multi-stem-birch/catalog-before-2d.json")
STEMS_500 = {
    "elevation": [[5.4994, 0.08], [0.551, 0.1251], [4.8832, 1.164], [11.06, 1.76], [11.04, 1.82], [4.8687, 1.306], [-0.0197, 0.223], [-0.2937, 0.3659], [-0.3062, -0.3554], [-0.0421, -0.2262], [4.7616, -1.646], [10.94, -2.37], [10.955, -2.31], [4.7931, -1.496], [0.4534, -0.1263], [5.5022, -0.08], [12.19, 0.2], [12.19, 0.26]],
    "front": [[0.272, 0.1241], [1.47, 10.5884], [1.55, 12.49], [1.48, 12.49], [1.33, 10.5962], [0.75, 5.8071], [0.0601, 0.5], [-0.33, 5.1143], [-0.76, 11.29], [-0.83, 11.29], [-0.78, 9.6], [-0.87, 11.33], [-0.94, 11.33], [-0.9, 9.6237], [-0.25, 0.3538], [-0.35, 0], [0.35, 0]],
    "plan": [[-0.2486, -0.2489], [0, -0.35], [0.2634, -0.2377], [1.4682, -0.53], [1.9509, -0.81], [2.16, -0.84], [2.13, -0.76], [1.5026, -0.35], [0.3607, -0.0278], [0.2893, 0.2182], [0.0926, 0.33], [0.0, 0.878], [-0.28, 1.4147], [-0.41, 1.54], [-0.48, 1.5], [-0.2, 0.8365], [-0.1385, 0.3201], [-0.2893, 0.2182], [-0.3607, -0.0642], [-1.3553, -0.41], [-1.99, -0.86], [-1.96, -0.93], [-1.7694, -0.88], [-1.2969, -0.58]],
}


def rings(shape):
    if shape["kind"] == "poly":
        return [[tuple(p) for p in shape["points"]]]
    out = []
    for sub in shape["d"].split("M")[1:]:
        n = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", sub)]
        out.append(list(zip(n[0::2], n[1::2])))
    return out


def shift(shape, dx, dy):
    if shape["kind"] == "poly":
        shape["points"] = [[round(p[0] + dx, 5), round(p[1] + dy, 5)] for p in shape["points"]]
        return
    parts = []
    for ring in rings(shape):
        parts.append("M " + " L ".join(f"{round(x + dx, 5)} {round(y + dy, 5)}" for x, y in ring) + " Z")
    shape["d"] = " ".join(parts)


def area(ring):
    return abs(sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(ring, ring[1:] + ring[:1]))) / 2


def inside(p, ring):
    hit = False
    for a, b in zip(ring, ring[1:] + ring[:1]):
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < a[0] + (p[1] - a[1]) * (b[0] - a[0]) / (b[1] - a[1]):
            hit = not hit
    return hit


def box(ring):
    return min(p[0] for p in ring), min(p[1] for p in ring), max(p[0] for p in ring), max(p[1] for p in ring)


def reach(a, b, limit):
    """📏 Nearest pair (distance, point of a, point of b) of two rings, or None when their boxes are further apart than the limit."""
    ba, bb = box(a), box(b)
    if ba[0] > bb[2] + limit or bb[0] > ba[2] + limit or ba[1] > bb[3] + limit or bb[1] > ba[3] + limit:
        return None
    best = None
    for ring, other, flip in ((a, b, False), (b, a, True)):
        near = [p for p in ring if box(other)[0] - limit <= p[0] <= box(other)[2] + limit and box(other)[1] - limit <= p[1] <= box(other)[3] + limit]
        for p in near:
            for s, e in zip(other, other[1:] + other[:1]):
                dx, dy = e[0] - s[0], e[1] - s[1]
                length = dx * dx + dy * dy
                t = 0 if length == 0 else max(0, min(1, ((p[0] - s[0]) * dx + (p[1] - s[1]) * dy) / length))
                q = (s[0] + dx * t, s[1] + dy * t)
                d = math.dist(p, q)
                if best is None or d < best[0]:
                    best = (d, q, p) if flip else (d, p, q)
    return best


def main():
    text = open(CATALOG).read()
    doc = json.loads(text)
    before = json.load(open(BEFORE))["asset"]["representations"]
    for scale in BARK_AS_MEASURED:
        for view, shapes in doc["asset"]["representations"][scale]["geometry"].items():
            old = {s["id"]: s for s in before[scale]["geometry"][view]}
            for s in shapes:
                if s.get("region") != "leaf-crown" and s["id"] in old:
                    s.update(old[s["id"]])
    for scale, drawing in doc["asset"]["representations"].items():
        for view, shapes in drawing["geometry"].items():
            if scale == "500":
                for s in shapes:
                    if s["id"] == "stems":
                        s["points"] = STEMS_500[view]
            by = {s["id"]: s for s in shapes}
            for mover, anchor in BARK_JOINS.get(scale, []):
                if mover in by and anchor in by:
                    d, pm, pa = reach(rings(by[mover])[0], rings(by[anchor])[0], 1.0)
                    if 1e-9 < d < BARK_REACH_M:
                        k = (d + BARK_OVERLAP_M) / d
                        shift(by[mover], (pa[0] - pm[0]) * k, (pa[1] - pm[1]) * k)
                        print("[DEBUG]", scale, view, mover, "joined to", anchor, round(d * 1000, 1), "mm")
            for guest, host in BARK_ONE_SHAPE.get(scale, []):
                if guest in by and host in by:
                    both = rings(by[host]) + rings(by[guest])
                    by[host].pop("points", None)
                    by[host].pop("closed", None)
                    by[host]["kind"] = "path"
                    by[host]["d"] = " ".join("M " + " L ".join(f"{x} {y}" for x, y in ring) + " Z" for ring in both)
                    shapes.remove(by[guest])
                    print("[DEBUG]", scale, view, guest, "drawn as second island of", host)
            crown = [s for s in shapes if s.get("filled") and s.get("closed", True) and s.get("role") != "ground" and s["id"] != "stems"]
            limit = (INK_GAP_MM + max(s.get("penMm") or 0.25 for s in crown)) * drawing["validRange"]["maxDenominator"] / 1000
            for _ in range(6):
                moved = 0
                for i, a in enumerate(crown):
                    for b in crown[i + 1:]:
                        if a.get("bone") != b.get("bone") or a.get("z") != b.get("z"):
                            continue
                        if a.get("region") != "leaf-crown" or b.get("region") != "leaf-crown" or view != "elevation":
                            continue
                        ra, rb = max(rings(a), key=area), max(rings(b), key=area)
                        found = None
                        for one in rings(a):
                            for two in rings(b):
                                got = reach(one, two, limit)
                                if got is not None and (found is None or got[0] < found[0]):
                                    found = got
                        if found is None or found[0] >= limit or found[0] < 1e-9:
                            continue
                        if any(inside(p, ra) for p in rb) or any(inside(p, rb) for p in ra):
                            continue
                        d, pa, pb = found
                        small, sign = (b, 1) if area(rb) <= area(ra) else (a, -1)
                        k = (d + limit / 8) / d * sign
                        shift(small, (pa[0] - pb[0]) * k, (pa[1] - pb[1]) * k)
                        moved += 1
                        print("[DEBUG]", scale, view, a["id"], b["id"], "gap", round(d * 1000, 1), "mm closed")
                if not moved:
                    break
    out = json.dumps(doc, ensure_ascii=False)
    if "--write" in sys.argv:
        for scale in doc["asset"]["representations"]:
            for view in ("elevation", "front", "plan"):
                import importlib.util
                spec = importlib.util.spec_from_file_location("draw", __file__.replace("-gaps.py", ".py"))
                draw = importlib.util.module_from_spec(spec)
                spec.loader.exec_module(draw)
                start, end = draw.array_range(text, ["representations", scale, "geometry", view])
                text = text[:start] + json.dumps(doc["asset"]["representations"][scale]["geometry"][view], ensure_ascii=False, separators=(",", ":")) + text[end:]
        assert json.loads(text) == doc
        open(CATALOG, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    main()
