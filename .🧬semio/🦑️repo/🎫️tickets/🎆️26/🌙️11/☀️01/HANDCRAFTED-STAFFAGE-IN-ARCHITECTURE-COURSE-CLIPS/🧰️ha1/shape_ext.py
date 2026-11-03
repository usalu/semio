import re
NUM = re.compile(r"-?\d+(?:\.\d+)?(?:e-?\d+)?")
def shape_ext(s):
    k = s.get("kind")
    if k == "rect":
        return (s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"])
    if k == "poly":
        xs = [p[0] for p in s["points"]]; ys = [p[1] for p in s["points"]]
        return (min(xs), min(ys), max(xs), max(ys))
    if k == "ellipse":
        rx, ry = s["rx"], s["ry"]
        return (s["cx"] - rx, s["cy"] - ry, s["cx"] + rx, s["cy"] + ry)
    if k == "path":
        n = [float(v) for v in NUM.findall(s["d"])]
        xs, ys = n[0::2], n[1::2]
        return (min(xs), min(ys), max(xs), max(ys))
    if k == "capsule":
        return (min(s["x0"], s["x1"]) - s["r0"], min(s["y0"], s["y1"]) - s["r1"], max(s["x0"], s["x1"]) + s["r0"], max(s["y0"], s["y1"]) + s["r1"])
    return None
