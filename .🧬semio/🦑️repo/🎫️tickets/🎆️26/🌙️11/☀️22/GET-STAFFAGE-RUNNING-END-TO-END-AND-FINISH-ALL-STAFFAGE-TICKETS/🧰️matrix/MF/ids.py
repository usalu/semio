"""🪪️ Prints per scale and view the shapes of an asset as `id[bone] x-range y-range` in one compact line each, filled closed shapes only unless ALL=1: ids.py <asset id> [view] [bone]"""
import glob, json, os, re, sys

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
a = json.load(open(glob.glob(f"{CATALOG}/*/{sys.argv[1]}/🔣️.json")[0]))["asset"]
want_view = sys.argv[2] if len(sys.argv) > 2 else None
want_bone = sys.argv[3] if len(sys.argv) > 3 else None


def box(s):
    if s["kind"] == "rect":
        return s["x"], s["x"] + s["w"], s["y"], s["y"] + s["h"]
    if s["kind"] == "poly":
        xs, ys = [p[0] for p in s["points"]], [p[1] for p in s["points"]]
    elif s["kind"] in ("ellipse", "circle"):
        rx, ry = s.get("rx", s.get("r")), s.get("ry", s.get("r"))
        return s["cx"] - rx, s["cx"] + rx, s["cy"] - ry, s["cy"] + ry
    elif s["kind"] == "path":
        n = [float(v) for v in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", s["d"])]
        xs, ys = n[0::2], n[1::2]
    elif s["kind"] == "capsule":
        xs, ys = [s["x0"], s["x1"]], [s["y0"], s["y1"]]
    else:
        return 0, 0, 0, 0
    return min(xs), max(xs), min(ys), max(ys)


for key, rep in a["representations"].items():
    if "geometry" not in rep:
        continue
    for view in ("elevation", "front", "plan"):
        if want_view and view != want_view:
            continue
        for s in rep["geometry"][view]:
            if want_bone and s["bone"] != want_bone:
                continue
            if not os.environ.get("ALL") and not (s.get("filled") and s.get("closed", True)):
                continue
            b = box(s)
            solid = s.get("solid")
            print(f"{key} {view[:4]} {s['id']}[{s['bone']}] z{s.get('z')} {s['kind']} x {b[0]:.3f} {b[1]:.3f} y {b[2]:.3f} {b[3]:.3f}" + (f" SOLID {solid.get('sweep', solid['kind'])} {solid.get('z0')} {solid.get('z1')}" if solid else ""))
