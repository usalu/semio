"""📄️ The shapes of one view at one scale with their raw box in metres and their sides on paper in mm: `python3 paper.py <asset.json> <view> <scale> [max-paper-mm]`."""
import json, re, sys
a = json.load(open(sys.argv[1]))["asset"]
view, scale = sys.argv[2], sys.argv[3]
limit = float(sys.argv[4]) if len(sys.argv) > 4 else 1e9


def box(s):
    """📦️ The raw box of a shape, None for a kind without plain coordinates."""
    k = s["kind"]
    if k == "rect":
        return s["x"], s["y"], s["x"] + s["w"], s["y"] + s["h"]
    if k == "ellipse":
        return s["cx"] - s["rx"], s["cy"] - s["ry"], s["cx"] + s["rx"], s["cy"] + s["ry"]
    if k == "poly":
        xs, ys = [p[0] for p in s["points"]], [p[1] for p in s["points"]]
    elif k == "path":
        d = re.sub(r"A[^LMZCQ]*", lambda m: "L " + " ".join(m.group(0).split()[-2:]) + " ", s["d"])
        n = [float(v) for v in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", d)]
        xs, ys = n[0::2], n[1::2]
    else:
        return None
    return min(xs), min(ys), max(xs), max(ys)


for s in a["representations"][scale]["geometry"][view]:
    b = box(s)
    if b is None:
        print(f"  {s['id']} | {s.get('bone')} | {s['kind']} ?")
        continue
    small = min(b[2] - b[0], b[3] - b[1]) * 1000 / int(scale)
    big = max(b[2] - b[0], b[3] - b[1]) * 1000 / int(scale)
    if small <= limit:
        flags = ("solid " if s.get("solid") else "") + ("" if s.get("filled") else "open ") + s.get("role", "")
        print(f"  {s['id']} | {s.get('bone')} | {s['kind']} x[{b[0]:.4f},{b[2]:.4f}] y[{b[1]:.4f},{b[3]:.4f}] paper {small:.2f}x{big:.2f} {flags} z{s.get('z')}")
