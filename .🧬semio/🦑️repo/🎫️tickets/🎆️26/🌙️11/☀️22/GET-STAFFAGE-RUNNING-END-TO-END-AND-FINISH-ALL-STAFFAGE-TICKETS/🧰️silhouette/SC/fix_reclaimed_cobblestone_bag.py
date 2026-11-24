"""🪨️ Repair of `reclaimed-cobblestone-bag`. Why the rasteriser failed: the body of 1:50 held each of the 384 setts as a box with `cornerRadius` 8 mm, which the kernel rounds on all twelve arrises and eight corners with about 2032 triangles each: 780332 triangles, a rendered view of 39 MB (the librsvg reference reads at most 10 MB), and as many triangles in every model export. (1) Each sett becomes a polyhedron of the same 100 mm cube with its twelve worn arrises as flat 5 mm chamfers (the chord of the 8 mm rounding; 24 vertices, 44 triangles, the silhouette within 1 mm of the rounded one), same place, turn and repeat: 16896 triangles for all setts. (2) The front elevation of 1:50 drew the joint between the stringer board and the deck boards as one line across the four open gaps between the deck boards; the line took 4.5 mm of each 22 mm gap, which the drawing then showed too small to count as an opening while the body shows it (48.8 mm to the next drawn edge): the joint is drawn under the five deck boards only, each stroke ending half a pen width (4.5 mm) inside its board so that its cap ends at the board edge. Idempotent: python3 fix_reclaimed_cobblestone_bag.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-cobblestone-bag"
CHAMFER = 0.005
CAP = 0.0045


def worn_cube(size):
    """🧊️ Vertices and outward triangles of a cube with all arrises chamfered."""
    h = [s / 2 for s in size]
    verts, index = [], {}

    def v(p):
        key = tuple(sc.r6(x) for x in p)
        if key not in index:
            index[key] = len(verts)
            verts.append(list(key))
        return index[key]

    def facet(points):
        c = [sum(p[k] for p in points) / len(points) for k in range(3)]
        a, b, d = points[0], points[1], points[2]
        n = [(b[1] - a[1]) * (d[2] - a[2]) - (b[2] - a[2]) * (d[1] - a[1]), (b[2] - a[2]) * (d[0] - a[0]) - (b[0] - a[0]) * (d[2] - a[2]), (b[0] - a[0]) * (d[1] - a[1]) - (b[1] - a[1]) * (d[0] - a[0])]
        if sum(n[k] * c[k] for k in range(3)) < 0:
            points = points[::-1]
        ids = [v(p) for p in points]
        return [[ids[0], ids[i], ids[i + 1]] for i in range(1, len(ids) - 1)]

    def at(axis, sign, s1, s2):
        p = [0.0, 0.0, 0.0]
        a1, a2 = [k for k in range(3) if k != axis]
        p[axis] = sign * h[axis]
        p[a1] = s1 * (h[a1] - CHAMFER)
        p[a2] = s2 * (h[a2] - CHAMFER)
        return p

    tris = []
    for axis in range(3):
        for sign in (-1, 1):
            tris += facet([at(axis, sign, -1, -1), at(axis, sign, 1, -1), at(axis, sign, 1, 1), at(axis, sign, -1, 1)])
    for a1 in range(3):
        for a2 in range(a1 + 1, 3):
            a3 = 3 - a1 - a2
            for s1 in (-1, 1):
                for s2 in (-1, 1):
                    quad = []
                    for s3 in (-1, 1):
                        p = [0.0, 0.0, 0.0]
                        p[a1], p[a2], p[a3] = s1 * h[a1], s2 * (h[a2] - CHAMFER), s3 * (h[a3] - CHAMFER)
                        q = [0.0, 0.0, 0.0]
                        q[a1], q[a2], q[a3] = s1 * (h[a1] - CHAMFER), s2 * h[a2], s3 * (h[a3] - CHAMFER)
                        quad.append((p, q))
                    tris += facet([quad[0][0], quad[0][1], quad[1][1], quad[1][0]])
    for sx in (-1, 1):
        for sy in (-1, 1):
            for sz in (-1, 1):
                tris += facet([[sx * h[0], sy * (h[1] - CHAMFER), sz * (h[2] - CHAMFER)], [sx * (h[0] - CHAMFER), sy * h[1], sz * (h[2] - CHAMFER)], [sx * (h[0] - CHAMFER), sy * (h[1] - CHAMFER), sz * h[2]]])
    assert len(verts) == 24 and len(tris) == 44, (len(verts), len(tris))
    return verts, tris


text, doc = sc.load(ID)
setts = 0
for solid in sc.solids(doc, 50):
    if solid["id"].startswith("sett-") and (solid["kind"] == "box" and solid.get("cornerRadius") == 0.008 or solid["kind"] == "polyhedron"):
        size = solid.pop("size", [0.1, 0.1, 0.1])
        solid.pop("cornerRadius", None)
        verts, tris = worn_cube(size)
        rebuilt = {}
        for key, value in solid.items():
            if key in ("vertices", "triangles"):
                continue
            rebuilt[key] = value
            if key == "kind":
                rebuilt["kind"] = "polyhedron"
                rebuilt["vertices"] = verts
                rebuilt["triangles"] = tris
        solid.clear()
        solid.update(rebuilt)
        setts += 1
assert setts == 64, setts
joint = sc.shape(doc, 50, "front", "fr-pallet-joint-2")
boards = [(-0.4, -0.255), (-0.21375, -0.11375), (-0.0725, 0.0725), (0.11375, 0.21375), (0.255, 0.4)]
joint.pop("points", None)
joint.pop("closed", None)
joint["kind"] = "path"
joint["d"] = " ".join(f"M {sc.r6(a + CAP)} 0.122 L {sc.r6(b - CAP)} 0.122" for a, b in boards)
sc.save(ID, text, doc)
