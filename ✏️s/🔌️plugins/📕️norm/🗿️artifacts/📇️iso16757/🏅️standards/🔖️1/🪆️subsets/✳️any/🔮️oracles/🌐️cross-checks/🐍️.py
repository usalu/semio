"""Independent third-party cross-check oracle for ISO 16757-2 7.1 solid volumes. Method (no Rust formula reused): every primitive is a convex solid (box = 8 vertices, cylinder = 4096-gon prism hull, sphere = analytic circle sections); a CSG tree is sliced at height z, the planar sections are combined with shapely polygon booleans and the area is integrated over z with scipy.integrate.quad (breakpoints at all primitive z extents). Convex primitives are also checked against scipy ConvexHull volumes and closed forms. Run `python -I 🐍️.py` to verify the committed vectors, `--write` to regenerate vectors and the derived Rust table."""
import json
import math
import pathlib
import sys

import numpy as np
from scipy.integrate import quad
from scipy.spatial import ConvexHull
from shapely.geometry import MultiPoint, Point
from shapely.ops import unary_union

SUBSET = pathlib.Path(__file__).resolve().parents[2]
VECTORS = SUBSET / "🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json"
RUST_TABLE = SUBSET / "🧪️tests/🧫️fixtures/🔮️cross-checks/🦀️.rs"
RING = 4096


def prim(kind, **p):
    return {"node": "primitive", "kind": kind, "parameters": {k: float(v) for k, v in p.items()}}


def box(w, h, d):
    return prim("box", width=w, height=h, depth=d)


def cyl(r, h):
    return prim("cylinder", radius=r, height=h)


def sph(r):
    return prim("sphere", radius=r)


def tf(child, t=(0, 0, 0), rot=(0, 0, 0)):
    return {"node": "transform", "translation": list(map(float, t)), "rotationDeg": list(map(float, rot)), "child": child}


def op(name, *children):
    return {"node": "boolean", "operator": name, "children": list(children)}


def rotation(deg):
    a, b, g = map(math.radians, deg)
    rx = np.array([[1, 0, 0], [0, math.cos(a), -math.sin(a)], [0, math.sin(a), math.cos(a)]])
    ry = np.array([[math.cos(b), 0, math.sin(b)], [0, 1, 0], [-math.sin(b), 0, math.cos(b)]])
    rz = np.array([[math.cos(g), -math.sin(g), 0], [math.sin(g), math.cos(g), 0], [0, 0, 1]])
    return rz @ ry @ rx


def leaves(node, rot=np.eye(3), org=np.zeros(3)):
    """Yield (kind, params, rot, origin) for every primitive with its world frame."""
    t = node["node"]
    if t == "primitive":
        yield node["kind"], node["parameters"], rot, org
    elif t == "transform":
        yield from leaves(node["child"], rot @ rotation(node["rotationDeg"]), org + rot @ np.array(node["translation"]))
    else:
        for c in node["children"]:
            yield from leaves(c, rot, org)


def local_points(kind, p):
    if kind == "box":
        return np.array([[x, y, z] for x in (0, p["width"]) for y in (0, p["height"]) for z in (0, p["depth"])], dtype=float)
    r, h = p["radius"], p["height"]
    a = np.linspace(0, 2 * math.pi, RING, endpoint=False)
    ring = np.stack([r + r * np.cos(a), np.zeros(RING), r + r * np.sin(a)], axis=1)
    return np.vstack([ring, ring + [0, h, 0]])


class Convex:
    def __init__(self, kind, p, rot, org):
        self.kind, self.p = kind, p
        if kind == "sphere":
            r = p["radius"]
            self.c = org + rot @ np.array([r, r, r])
            self.r = r
            self.zlo, self.zhi = self.c[2] - r, self.c[2] + r
            return
        self.pts = local_points(kind, p) @ rot.T + org
        self.hull = ConvexHull(self.pts)
        self.tri = self.pts[self.hull.simplices]
        self.zlo, self.zhi = self.pts[:, 2].min(), self.pts[:, 2].max()

    def volume(self):
        return 4 / 3 * math.pi * self.r ** 3 if self.kind == "sphere" else float(self.hull.volume)

    def section(self, z):
        if z <= self.zlo or z >= self.zhi:
            return Point().buffer(0)
        if self.kind == "sphere":
            rr = self.r ** 2 - (z - self.c[2]) ** 2
            return Point(self.c[0], self.c[1]).buffer(math.sqrt(max(rr, 0.0)), 1024)
        d = self.tri[:, :, 2] - z
        pts = []
        for i, j in ((0, 1), (1, 2), (2, 0)):
            m = d[:, i] * d[:, j] < 0
            a, b, da, db = self.tri[m, i], self.tri[m, j], d[m, i], d[m, j]
            t = (da / (da - db))[:, None]
            pts.append((a + t * (b - a))[:, :2])
        pts = np.vstack(pts)
        return MultiPoint(pts).convex_hull if len(pts) >= 3 else Point().buffer(0)


def section(node, solids, z):
    t = node["node"]
    if t == "primitive":
        return solids.pop(0).section(z)
    if t == "transform":
        return section(node["child"], solids, z)
    parts = [section(c, solids, z) for c in node["children"]]
    name = node["operator"]
    if name == "union":
        return unary_union(parts)
    if name == "intersection":
        out = parts[0]
        for q in parts[1:]:
            out = out.intersection(q)
        return out
    out = parts[0]
    for q in parts[1:]:
        out = out.difference(q)
    return out


def volume(node):
    convex = [Convex(*l) for l in leaves(node)]
    zs = sorted({v for c in convex for v in (c.zlo, c.zhi)})
    total = 0.0
    for lo, hi in zip(zs, zs[1:]):
        v, _ = quad(lambda z: section(node, list(convex), z).area, lo, hi, epsabs=1e-10, epsrel=1e-10, limit=200)
        total += v
    return total


def rust_expr(node):
    t = node["node"]
    if t == "primitive":
        params = ", ".join(f'("{k}".into(), {v!r})' for k, v in node["parameters"].items())
        return f'GeometryNode::Primitive {{ kind: "{node["kind"]}".into(), parameters: BTreeMap::from([{params}]) }}'
    if t == "transform":
        return f'GeometryNode::Transform {{ translation: {node["translation"]!r}, rotation_deg: {node["rotationDeg"]!r}, child: Box::new({rust_expr(node["child"])}) }}'
    kids = ", ".join(rust_expr(c) for c in node["children"])
    return f'GeometryNode::Boolean {{ operator: BooleanOperator::{node["operator"].capitalize()}, children: vec![{kids}] }}'


PI = math.pi
CASES = [
    ("box-1x2x3", box(1, 2, 3), 6.0, "box"),
    ("cylinder-r1-h2", cyl(1, 2), PI * 2, "cylinder"),
    ("sphere-r1.5", sph(1.5), 4 / 3 * PI * 1.5 ** 3, "sphere"),
    ("box-2x1x1-rotated-45-about-z", tf(box(2, 1, 1), rot=(0, 0, 45)), 2.0, "rotation"),
    ("cylinder-r0.5-h3-rotated-90-about-x", tf(cyl(0.5, 3), rot=(90, 0, 0)), PI * 0.25 * 3, "rotation"),
    ("union-of-two-unit-boxes-5m-apart", op("union", box(1, 1, 1), tf(box(1, 1, 1), t=(5, 5, 5))), 2.0, "disjoint union"),
    ("intersection-of-two-unit-boxes-offset-0.5", op("intersection", box(1, 1, 1), tf(box(1, 1, 1), t=(.5, .5, .5))), 0.125, "overlap"),
    ("difference-unit-box-minus-half-box", op("difference", box(1, 1, 1), box(0.5, 1, 1)), 0.5, "difference"),
    ("union-of-two-unit-spheres-1m-apart", op("union", sph(1), tf(sph(1), t=(1, 0, 0))), 8 / 3 * PI - PI * 5 / 12, "lens closed form"),
    ("cylinder-with-square-hole", op("difference", cyl(1, 2), tf(box(1, 3, 1), t=(.5, -.5, .5))), None, "difference"),
    ("two-boxes-one-rotated-45-about-z", op("union", box(2, 1, 1), tf(box(2, 1, 1), t=(1, .5, 0), rot=(0, 0, 45))), None, "rotated overlap"),
    ("rotated-cylinder-inside-sphere", op("intersection", tf(cyl(0.5, 2), t=(1, 0, 0), rot=(0, 0, 90)), tf(sph(1), t=(0, 0, 0))), None, "rotated overlap"),
]


def vectors():
    rows = []
    for name, tree, closed, note in CASES:
        v = volume(tree)
        convex = [Convex(*l) for l in leaves(tree)]
        row = {"id": name, "geometry": tree, "solidVolumeM3": v, "closedFormM3": closed, "note": note}
        if tree["node"] != "boolean" and len(convex) == 1:
            row["convexHullVolumeM3"] = convex[0].volume()
        rows.append(row)
        assert closed is None or abs(v - closed) <= 5e-6 * closed, (name, v, closed)
    return {"oracle": "shapely polygon booleans on z-sections + scipy.integrate.quad, scipy ConvexHull, closed forms; ISO 16757-2 7.1 solid volume", "volumes": rows}


def rust_table(doc):
    out = ["// Derived from 🔣️.json by 🔮️oracles/🌐️cross-checks/🐍️.py --write; do not edit.", "pub fn volume_cases() -> Vec<(&'static str, GeometryNode, f64)> {", "    vec!["]
    for r, (_, tree, _, _) in zip(doc["volumes"], CASES):
        out.append(f'        ("{r["id"]}", {rust_expr(tree)}, {r["solidVolumeM3"]!r}),')
    return "\n".join(out + ["    ]", "}", ""])


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, float) or isinstance(b, float):
        return a is not None and b is not None and abs(a - b) <= 1e-7 * max(abs(a), abs(b), 1e-300) + 1e-12
    return a == b


def main():
    doc = json.loads(json.dumps(vectors()))
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        RUST_TABLE.write_text(rust_table(doc), encoding="utf8")
        return 0
    ok = close(doc, json.loads(VECTORS.read_text(encoding="utf8")))
    print(json.dumps({"ok": ok}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
