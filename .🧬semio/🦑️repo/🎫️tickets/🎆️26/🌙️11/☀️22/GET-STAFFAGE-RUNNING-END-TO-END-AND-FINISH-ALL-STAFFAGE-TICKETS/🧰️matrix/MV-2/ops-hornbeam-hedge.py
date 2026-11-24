"""🌿 Writes `ops/hornbeam-hedge.json`: at 1:50, 1:100 and 1:200 the plan of every hedge block is its cross-section swept along the block (`sweep: "u"`) over the whole height, while the front draws the block as its outline, which narrows towards the top (0.70 m at the foot, 0.36 to 0.56 m at the top). At rest both are 0.70 m wide; when the gale leans the blocks sideways the plan prism carried the full cross-section up to the top and reached 83 to 93 mm beyond the front. The plan block becomes the two solids the block is: the prism of the full cross-section up to the height `top` where the front outline is still within half the tolerance (2 % of the largest dimension) of its greatest half width, and above it `<block>-top`, the front outline from that height on, swept along the bone normal over the thickness the side elevation draws there (drawn below the prism, taken in until its corners lie inside the plan outline, so the plan at rest is unchanged). Run: python3 ops-hornbeam-hedge.py (reads the catalog file, prints the ops file path)."""
import json, os, re

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
HERE = os.path.dirname(os.path.abspath(__file__))
asset = json.load(open(f"{S}/🗂️catalog/🌳️vegetation/hornbeam-hedge/🔣️.json"))["asset"]
tolerance = 0.02 * max(asset["dimensions"][key] for key in ["heightM", "widthM", "depthM"])


def ring(shape):
    numbers = [float(x) for x in re.findall(r"-?\d+\.?\d*", shape["d"])] if shape["kind"] == "path" else [c for p in shape["points"] for c in p]
    return list(zip(numbers[0::2], numbers[1::2]))


def inside(p, poly):
    result = False
    for i in range(len(poly)):
        a, b = poly[i], poly[i - 1]
        if (a[1] > p[1]) != (b[1] > p[1]) and p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]:
            result = not result
    return result


ops = []
for scale in ["50", "100", "200"]:
    geometry = asset["representations"][scale]["geometry"]
    for n in range(1, 7):
        name = f"block-{n}"
        find = lambda view: next(s for s in geometry[view] if s["id"] == name and s["bone"] == name)
        side, front, plan = ring(find("elevation")), ring(find("front")), find("plan")
        limit = max(abs(w) for _, w in front) - tolerance / 2
        tops = [u for u, w in front if abs(w) >= limit]
        for i in range(len(front)):
            (u0, w0), (u1, w1) = front[i - 1], front[i]
            if (abs(w0) - limit) * (abs(w1) - limit) < 0:
                tops.append(u0 + (u1 - u0) * (limit - abs(w0)) / (abs(w1) - abs(w0)))
        top = round(max(tops), 4)
        upper = []
        for i in range(len(front)):
            a, b = front[i - 1], front[i]
            if (a[0] - top) * (b[0] - top) < 0:
                upper.append([top, round(a[1] + (b[1] - a[1]) * (top - a[0]) / (b[0] - a[0]), 4)])
            if b[0] >= top:
                upper.append([b[0], b[1]])
        thickness = [v for u, v in side if u >= top] or [v for _, v in side]
        z0, z1 = min(thickness), max(thickness)
        outline = ring(plan)
        while not all(inside((v, w), outline) for v in [z0, z1] for w in [min(p[1] for p in upper), max(p[1] for p in upper)]) and z1 - z0 > 0.2:
            z0, z1 = round(z0 + 0.005, 4), round(z1 - 0.005, 4)
        ops.append({"op": "set", "scale": scale, "view": "plan", "id": name, "bone": name, "set": {"solid": {**plan["solid"], "z1": top}}})
        shape = {key: plan[key] for key in ["bone", "role", "filled", "tone", "region", "material", "penMm"] if key in plan}
        ops.append({"op": "add", "scale": scale, "view": "plan", "before": name, "shape": {"id": f"{name}-top", **shape, "z": round(plan["z"] - 0.01, 4), "kind": "poly", "closed": True, "points": upper, "solid": {"kind": "extruded", "sweep": "v", "z0": z0, "z1": z1}}})
out = os.path.join(HERE, "ops", "hornbeam-hedge.json")
open(out, "w").write("[\n" + ",\n".join("  " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
print(len(ops), "ops", [(op["set"]["solid"]["z1"]) for op in ops if op["op"] == "set"], [(len(op["shape"]["points"]), op["shape"]["solid"]["z0"], op["shape"]["solid"]["z1"]) for op in ops if op["op"] == "add"])
