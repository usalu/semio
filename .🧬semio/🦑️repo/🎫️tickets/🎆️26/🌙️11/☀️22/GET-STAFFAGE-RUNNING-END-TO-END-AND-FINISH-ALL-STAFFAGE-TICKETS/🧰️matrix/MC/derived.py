"""📐️ The derived-view measure of the suite (`T/🪞️derived.ts`: vertices and edge midpoints, centred on the box, scaled by the longest side, symmetric Hausdorff distance under the best of eight transforms) for views made of `rect` and `poly` shapes only, in Python, so that candidate drawings of a prismatic asset can be measured without a tool run: `python3 derived.py <id> [<scale>…]` prints front and plan against the elevation and plan against front for the catalog document; `distance(a, b)` and `points(shapes)` serve the candidate scripts. The numbers of the catalog documents are checked against `derived.ts`."""
import glob, json, math, sys
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
T = [(1, 0, 0, 1), (-1, 0, 0, 1), (1, 0, 0, -1), (-1, 0, 0, -1), (0, -1, 1, 0), (0, 1, -1, 0), (0, 1, 1, 0), (0, -1, -1, 0)]


def ring(s):
    """🔲️ The ring of a rect or poly shape and whether it is closed."""
    if s["kind"] == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        return [(x, y), (x + w, y), (x + w, y + h), (x, y + h)], True
    if s["kind"] == "poly":
        return [tuple(p) for p in s["points"]], s.get("closed", False)
    raise ValueError(f"shape {s['id']} is a {s['kind']}")


def points(shapes):
    """✂️ The unit point set of shapes; with it the longest side in metres."""
    raw = []
    for s in shapes:
        if s.get("role") == "ground":
            continue
        r, closed = ring(s)
        for i, p in enumerate(r):
            raw.append(p)
            q = r[i + 1] if i + 1 < len(r) else (r[0] if closed else None)
            if q is not None:
                raw.append(((p[0] + q[0]) / 2, (p[1] + q[1]) / 2))
    xs, ys = [p[0] for p in raw], [p[1] for p in raw]
    side = max(max(xs) - min(xs), max(ys) - min(ys))
    cx, cy = (max(xs) + min(xs)) / 2, (max(ys) + min(ys)) / 2
    stride = max(1, math.ceil(len(raw) / 240))
    return [((x - cx) / side, (y - cy) / side) for i, (x, y) in enumerate(raw) if i % stride == 0], side


def directed(a, b):
    """➡️ The largest distance of a point of a to its nearest point of b, with that point."""
    return max((min(math.hypot(p[0] - q[0], p[1] - q[1]) for q in b), p) for p in a)


def distance(a, b):
    """🧮️ The distance under the best transform, with the transform index and the farthest point of either set."""
    best = None
    for i, m in enumerate(T):
        moved = [(m[0] * x + m[1] * y, m[2] * x + m[3] * y) for x, y in b]
        ab, ba = directed(a, moved), directed(moved, a)
        d = max(ab[0], ba[0])
        if best is None or d < best[0]:
            best = (d, i, ab, ba)
    return best


def views(slug, scale):
    """📖️ The three views of a band of a catalog document."""
    return json.load(open(glob.glob(f"{CATALOG}/*/{slug}/🔣️.json")[0]))["asset"]["representations"][str(scale)]["geometry"]


def report(name, g):
    """🖨️ Prints the three in-band distances of a set of views."""
    for view, against in (("front", "elevation"), ("plan", "elevation"), ("plan", "front")):
        try:
            a, side = points(g[view])
            b, _ = points(g[against])
        except ValueError as e:
            print(f"   {name} {view} against {against}: not measured here ({e})")
            continue
        d, i, ab, ba = distance(a, b)
        print(f"   {name} {view} against {against}: {d:.5f} = {d * side * 1000:.1f} mm of {side:.2f} m (limit {30 * side:.0f} mm){'  FLAGGED' if d < 0.03 else ''}; farthest of {view} {ab[0] * side * 1000:.1f} mm at ({ab[1][0] * side:.3f}, {ab[1][1] * side:.3f}) m from the centre, of {against} {ba[0] * side * 1000:.1f} mm at ({ba[1][0] * side:.3f}, {ba[1][1] * side:.3f}) under transform {i}")


if __name__ == "__main__":
    for scale in sys.argv[2:] or ["50", "100", "200", "500"]:
        report(f"{sys.argv[1]} 1:{scale}", views(sys.argv[1], scale))
