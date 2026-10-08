import json, math, sys
from shapely.geometry import LineString, Polygon, box, Point
from shapely import affinity

def arc_points(a, b, bulge, n=24):
    ax, ay = a; bx, by = b
    if bulge == 0:
        return [a, b]
    theta = 4 * math.atan(bulge)
    chord = math.hypot(bx - ax, by - ay)
    r = chord / (2 * math.sin(abs(theta) / 2))
    mx, my = (ax + bx) / 2, (ay + by) / 2
    h = math.sqrt(max(r * r - (chord / 2) ** 2, 0))
    dx, dy = (bx - ax) / chord, (by - ay) / chord
    sign = 1 if bulge > 0 else -1
    # centre lies on the left of the chord for a ccw (positive) arc with sweep < pi
    side = 1 if (abs(theta) < math.pi) == (bulge > 0) else -1
    cx, cy = mx - dy * h * side, my + dx * h * side
    a0 = math.atan2(ay - cy, ax - cx)
    pts = []
    for i in range(n + 1):
        t = a0 + theta * i / n
        pts.append((cx + r * math.cos(t), cy + r * math.sin(t)))
    return pts

def axis_line(axis):
    if "Line" in axis:
        s, e = axis["Line"]["start"], axis["Line"]["end"]
        return LineString([(s["x"], s["y"]), (e["x"], e["y"])])
    s, e, b = axis["Arc"]["start"], axis["Arc"]["end"], axis["Arc"]["bulge"]
    return LineString(arc_points((s["x"], s["y"]), (e["x"], e["y"]), b))

def loop_poly(vs):
    pts = []
    n = len(vs)
    for i in range(n):
        a = (vs[i]["point"]["x"], vs[i]["point"]["y"])
        b = (vs[(i + 1) % n]["point"]["x"], vs[(i + 1) % n]["point"]["y"])
        seg = arc_points(a, b, vs[i]["bulge"])
        pts.extend(seg[:-1])
    return Polygon(pts)

m = json.load(open(sys.argv[1], encoding="utf-8"))
thick = {k: sum(l["thickness"] for l in v["layers"]) for k, v in m["wall_types"].items()}
bystorey = {}
for wid, w in m["walls"].items():
    poly = axis_line(w["axis"]).buffer(thick[w["wall_type"]] / 2, cap_style=2)
    bystorey.setdefault(w["storey"], []).append((wid, poly))
issues = 0
for sid, rows in bystorey.items():
    for i in range(len(rows)):
        for j in range(i + 1, len(rows)):
            a = rows[i][1].intersection(rows[j][1]).area
            if a > 0.06:
                print("wall overlap", sid, rows[i][0], rows[j][0], round(a, 3)); issues += 1
    for cid, c in m["columns"].items():
        if c["storey"] != sid: continue
        t = m["column_types"][c["column_type"]]["profile"]
        if "Rectangle" in t: r = t["Rectangle"]; poly = box(c["position"]["x"] - r["width"] / 2, c["position"]["y"] - r["depth"] / 2, c["position"]["x"] + r["width"] / 2, c["position"]["y"] + r["depth"] / 2)
        else: poly = Point(c["position"]["x"], c["position"]["y"]).buffer(t["Circle"]["diameter"] / 2)
        for wid, wp in rows:
            a = poly.intersection(wp).area
            if a > 0.001: print("column/wall clash", cid, wid, round(a, 4)); issues += 1
for sid in m["storeys"]:
    holes = [(k, loop_poly(h)) for k, s in m["slabs"].items() if s["storey"] == sid for h in s["holes"]]
    for k, hp in holes:
        for wid, wp in [r for rows in bystorey.values() for r in rows]:
            pass
for k, s in m["slabs"].items():
    outer = loop_poly(s["boundary"])
    for h in s["holes"]:
        hp = loop_poly(h)
        if not outer.contains(hp): print("hole outside slab", k); issues += 1
        # holes versus walls of the storey below that carry the slab (walls of the same storey stand ON the slab)
        for wid, w in m["walls"].items():
            wp = axis_line(w["axis"]).buffer(thick[w["wall_type"]] / 2, cap_style=2)
            if w["storey"] == s["storey"] and wp.intersection(hp).area > 0.001:
                print("hole/wall clash", k, wid, round(wp.intersection(hp).area, 4)); issues += 1
print("issues", issues)
