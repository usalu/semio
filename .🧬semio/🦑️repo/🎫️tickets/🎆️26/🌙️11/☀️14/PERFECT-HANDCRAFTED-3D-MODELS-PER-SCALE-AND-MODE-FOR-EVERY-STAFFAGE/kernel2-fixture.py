"""Writes the language-agnostic fixtures 🩻️integrity and 🪤️traps of the kernel (hand-computed expectations) and appends the loft orientation cases to 🧵️loft."""
import json, math, os
F = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🧫️fixtures"
Q = {"chordM": 0.0005, "minSegments": 24, "maxSegments": 128}
def solid(name, body, **expected):
    return {"name": name, "solid": {"id": name, "node": "root", "material": "m", **body}, "expected": expected}
def cyl(r, h): return math.pi * r * r * h
def path_len(pts): return sum(math.dist(pts[i], pts[i + 1]) for i in range(len(pts) - 1))
def tube(pts, r, **extra):
    return {"kind": "loft", "sections": [{"at": p, "rx": r, "ry": r} for p in pts], "capStart": True, "capEnd": True, **extra}
nozzle = [[0, round(0.5 + 0.4 * u - 0.9 * u * u, 6), round(0.4 * u, 6)] for u in [k / 8 for k in range(9)]]
arch = [[0, 0, 0], [0, 0.4, 0.1], [0, 0.6, 0.3], [0, 0.6, 0.6], [0, 0.4, 0.8], [0, 0, 0.9]]
hook = [[0, 0, 0], [0, 0.3, 0], [0, 0.5, 0.2], [0, 0.4, 0.4], [0, 0.2, 0.5]]
closed = dict(closed=True, manifold=True, oriented=True)
tol = 0.02
solids = [
  solid("box", {"kind": "box", "size": [0.4, 0.3, 0.2]}, volume=0.024, volumeTolerance=1e-9, **closed),
  solid("ellipsoid", {"kind": "ellipsoid", "radii": [0.3, 0.2, 0.1]}, volume=4 / 3 * math.pi * 0.006, volumeTolerance=tol, **closed),
  solid("capsule", {"kind": "capsule", "p0": [0, 0, 0], "p1": [0, 0.5, 0], "r0": 0.1, "r1": 0.1}, volume=cyl(0.1, 0.5) + 4 / 3 * math.pi * 0.001, volumeTolerance=tol, **closed),
  solid("revolve-cylinder-on-axis", {"kind": "revolve", "profile": [[0, 0], [0, 0.1], [0.5, 0.1], [0.5, 0]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, volume=cyl(0.1, 0.5), volumeTolerance=tol, **closed),
  solid("revolve-open-profile-is-filled", {"kind": "revolve", "profile": [[0, 0.1], [0.5, 0.1]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, volume=cyl(0.1, 0.5), volumeTolerance=tol, **closed),
  solid("revolve-shell-without-repeat-is-filled", {"kind": "revolve", "profile": [[0, 0.09], [0, 0.1], [0.5, 0.1], [0.5, 0.09]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, volume=cyl(0.1, 0.5), volumeTolerance=tol, **closed),
  solid("revolve-shell-with-repeat-is-hollow", {"kind": "revolve", "profile": [[0, 0.09], [0, 0.1], [0.5, 0.1], [0.5, 0.09], [0, 0.09]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, volume=cyl(0.1, 0.5) - cyl(0.09, 0.5), volumeTolerance=tol, **closed),
  solid("revolve-half-sweep", {"kind": "revolve", "profile": [[0, 0], [0, 0.1], [0.5, 0.1], [0.5, 0]], "axis": [0, 1, 0], "origin": [0, 0, 0], "sweepRad": math.pi}, volume=cyl(0.1, 0.5) / 2, volumeTolerance=tol, **closed),
  solid("extrude-square-with-hole", {"kind": "extrude", "profile": [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]], "holes": [[[-0.1, -0.1], [-0.1, 0.1], [0.1, 0.1], [0.1, -0.1]]], "plane": "xy", "from": 0, "to": 0.3}, volume=(0.16 - 0.04) * 0.3, volumeTolerance=1e-9, **closed),
  solid("extrude-pyramid-apex-at-to", {"kind": "extrude", "profile": [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]], "plane": "xy", "from": 0, "to": 0.3, "taperTo": [1, 0]}, volume=0.16 * 0.3 / 3, volumeTolerance=1e-9, **closed),
  solid("extrude-frustum", {"kind": "extrude", "profile": [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]], "plane": "xy", "from": 0, "to": 0.3, "taperTo": [1, 0.5]}, volume=0.3 / 3 * (0.16 + 0.04 + math.sqrt(0.16 * 0.04)), volumeTolerance=1e-9, **closed),
  solid("loft-straight", tube([[0, 0, 0], [0, 0, 1]], 0.1), volume=cyl(0.1, 1), volumeTolerance=tol, **closed),
  solid("loft-reversed-order", tube([[0, 0, 1], [0, 0, 0]], 0.1), volume=cyl(0.1, 1), volumeTolerance=tol, **closed),
  solid("loft-nozzle-first-tangent-up-net-travel-down", tube(nozzle, 0.03), volume=cyl(0.03, path_len(nozzle)), volumeTolerance=0.08, **closed),
  solid("loft-nozzle-reversed", tube(nozzle[::-1], 0.03), volume=cyl(0.03, path_len(nozzle)), volumeTolerance=0.08, **closed),
  solid("loft-arch", tube(arch, 0.03), volume=cyl(0.03, path_len(arch)), volumeTolerance=0.08, **closed),
  solid("loft-arch-reversed", tube(arch[::-1], 0.03), volume=cyl(0.03, path_len(arch)), volumeTolerance=0.08, **closed),
  solid("loft-hook", tube(hook, 0.03), volume=cyl(0.03, path_len(hook)), volumeTolerance=0.12, **closed),
  solid("loft-rotation-handedness-flipped-in-the-middle", {"kind": "loft", "sections": [{"at": [0, 0, 0], "rx": 0.1, "ry": 0.1, "rotation": [0, 0, 0, 1]}, {"at": [0, 0, 1], "rx": 0.1, "ry": 0.1}, {"at": [0, 0, 2], "rx": 0.1, "ry": 0.1, "rotation": [1, 0, 0, 0]}], "capStart": True, "capEnd": True}, volume=cyl(0.1, 2), volumeTolerance=tol, **closed),
  solid("loft-rotated-sections-opposing-the-path", {"kind": "loft", "sections": [{"at": [0, 0, 1], "rx": 0.1, "ry": 0.1, "rotation": [1, 0, 0, 0]}, {"at": [0, 0, 0], "rx": 0.1, "ry": 0.1, "rotation": [1, 0, 0, 0]}], "capStart": True, "capEnd": True}, volume=cyl(0.1, 1), volumeTolerance=tol, **closed),
  solid("loft-open-tube-start-cap-off", {**tube([[0, 0, 0], [0, 0, 1]], 0.1), "capStart": False}, closed=False, manifold=False, oriented=True, openEdgesPositive=True),
  solid("loft-open-tube-both-caps-off-outward", {**tube([[0, 0, 0], [0, 0, 1]], 0.1), "capStart": False, "capEnd": False}, closed=False, manifold=False, oriented=True, openEdgesPositive=True),
]
def cube(x0=0, flip=False, y0=0, z0=0):
    p = [[x0 + (i & 1), y0 + ((i >> 1) & 1), z0 + ((i >> 2) & 1)] for i in range(8)]
    t = [[0, 2, 3], [0, 3, 1], [4, 5, 7], [4, 7, 6], [0, 1, 5], [0, 5, 4], [2, 6, 7], [2, 7, 3], [0, 4, 6], [0, 6, 2], [1, 3, 7], [1, 7, 5]]
    if flip: t = [[a, c, b] for a, b, c in t]
    return p, t
def flat(p, t): return [c for v in p for c in v], [i for tri in t for i in tri]
def mesh(name, p, t, **expected):
    positions, indices = flat(p, t)
    return {"name": name, "positions": positions, "indices": indices, "expected": expected}
p, t = cube()
pi, ti = cube(flip=True)
p2, t2 = cube(x0=2)
pt = p + p2
tt = t + [[a + 8 for a in tri] for tri in t2]
inconsistent = [list(tri) for tri in t]
inconsistent[0] = [inconsistent[0][0], inconsistent[0][2], inconsistent[0][1]]
tetra = lambda o: ([[o[0], o[1], o[2]], [o[0] + 1, o[1], o[2]], [o[0], o[1] + 1, o[2]], [o[0], o[1], o[2] + 1]], [[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]])
a, ta = tetra([0, 0, 0])
b = [[-v[0], -v[1], -v[2]] for v in a]
tb = [[x, z, y] for x, y, z in ta]
pinch_p = a + b[1:]
pinch_t = ta + [[(0 if i == 0 else i + 3) for i in tri] for tri in tb]
meshes = [
  mesh("cube-outward", p, t, closed=True, manifold=True, oriented=True, signedVolume=1.0, outwardFrom=[[0.5, 0.5, 0.5]], components=[[0, 12]]),
  mesh("cube-inside-out-is-turned-outward", pi, ti, closed=True, manifold=True, oriented=True, signedVolume=-1.0, outwardFrom=[[0.5, 0.5, 0.5]], components=[[0, 12]]),
  mesh("cube-one-triangle-flipped", p, inconsistent, closed=True, manifold=True, oriented=False, outwardFrom=[[0.5, 0.5, 0.5]], components=[[0, 12]]),
  mesh("two-components-apart", pt, tt, closed=True, manifold=True, oriented=True, signedVolume=2.0, outwardFrom=[[0.5, 0.5, 0.5], [2.5, 0.5, 0.5]], components=[[0, 12], [12, 12]]),
  mesh("two-components-one-inside-out", pt, t + [[a + 8 for a in tri] for tri in ti], closed=True, manifold=True, oriented=True, signedVolume=0.0, outwardFrom=[[0.5, 0.5, 0.5], [2.5, 0.5, 0.5]], components=[[0, 12], [12, 12]]),
  mesh("open-quad", [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]], [[0, 1, 2], [0, 2, 3]], closed=False, manifold=False, oriented=True, openEdges=4),
  mesh("three-triangles-on-one-edge", [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1]], [[0, 1, 2], [0, 3, 1], [0, 1, 4]], closed=False, manifold=False, nonManifoldEdges=1),
  mesh("two-tetrahedra-sharing-one-vertex", pinch_p, pinch_t, closed=True, manifold=False, oriented=True, pinchVertices=1),
]
with open(f"{F}/🩻️integrity/🔣️.json", "w", encoding="utf8") if os.makedirs(f"{F}/🩻️integrity", exist_ok=True) is None else None as f:
    json.dump({"quality": Q, "solids": solids, "meshes": meshes}, f, indent=1, ensure_ascii=False)
    f.write("\n")

traps = {
  "revolveClosure": [
    {"name": "cylinder-reaching-the-axis", "profile": [[0, 0], [0, 0.1], [0.5, 0.1], [0.5, 0]], "kind": "onAxis"},
    {"name": "open-profile-one-end-on-axis", "profile": [[0, 0], [0.2, 0.1], [0.5, 0.1]], "kind": "onAxis"},
    {"name": "closed-loop-with-repeated-first-point", "profile": [[0, 0.09], [0, 0.1], [0.5, 0.1], [0.5, 0.09], [0, 0.09]], "kind": "loop"},
    {"name": "wall-without-repeated-first-point", "profile": [[0, 0.09], [0, 0.1], [0.5, 0.1], [0.5, 0.09]], "kind": "shell", "filledAreaM2": 0.05, "loopAreaM2": 0.5 * 0.01},
    {"name": "two-points-off-axis-is-a-plain-cylinder", "profile": [[0, 0.1], [0.5, 0.1]], "kind": "axis", "filledAreaM2": 0.05},
    {"name": "cup-section-open", "profile": [[0, 0.04], [0, 0.05], [0.1, 0.05], [0.1, 0.04]], "kind": "shell", "filledAreaM2": 0.1 * 0.05, "loopAreaM2": 0.1 * 0.01},
    {"name": "end-within-two-percent-of-the-axis", "profile": [[0, 0.001], [0, 0.1], [0.5, 0.1], [0.5, 0.002]], "kind": "onAxis"},
  ],
  "taper": [
    {"name": "absent", "taperTo": None, "fault": None},
    {"name": "pyramid", "taperTo": [1, 0], "fault": None},
    {"name": "apex-at-from", "taperTo": [0, 1], "fault": None},
    {"name": "frustum", "taperTo": [1, 0.5], "fault": None},
    {"name": "widening", "taperTo": [0.5, 1], "fault": None},
    {"name": "identity", "taperTo": [1, 1], "fault": None},
    {"name": "both-zero", "taperTo": [0, 0], "fault": "collapsed"},
    {"name": "speck", "taperTo": [1, 0.0001], "fault": "needle"},
    {"name": "read-as-scale-u-and-v", "taperTo": [0.4, 0.4], "fault": "uniform"},
  ],
  "loftFold": [
    {"name": "straight", "sections": [{"at": [0, 0, 0], "rx": 0.1, "ry": 0.1}, {"at": [0, 0, 1], "rx": 0.1, "ry": 0.1}], "fold": None},
    {"name": "gentle-arch", "sections": [{"at": a, "rx": 0.03, "ry": 0.03} for a in arch], "fold": None},
    {"name": "reversed-gentle-arch", "sections": [{"at": a, "rx": 0.03, "ry": 0.03} for a in arch[::-1]], "fold": None},
    {"name": "hairpin-tighter-than-the-section", "sections": [{"at": [0, 0, 0], "rx": 0.2, "ry": 0.2}, {"at": [0, 0, 1], "rx": 0.2, "ry": 0.2}, {"at": [0, 0.05, 0.9], "rx": 0.2, "ry": 0.2}], "fold": [1, 2]},
    {"name": "right-angle-elbow-tighter-than-the-section", "sections": [{"at": [0, 0, 0], "rx": 0.2, "ry": 0.2}, {"at": [0, 0, 0.05], "rx": 0.2, "ry": 0.2}, {"at": [0.05, 0, 0.05], "rx": 0.2, "ry": 0.2}], "fold": [0, 1]},
    {"name": "right-angle-elbow-wide-enough", "sections": [{"at": [0, 0, 0], "rx": 0.02, "ry": 0.02}, {"at": [0, 0, 0.3], "rx": 0.02, "ry": 0.02}, {"at": [0.3, 0, 0.3], "rx": 0.02, "ry": 0.02}], "fold": None},
  ],
}
os.makedirs(f"{F}/🪤️traps", exist_ok=True)
with open(f"{F}/🪤️traps/🔣️.json", "w", encoding="utf8") as f:
    json.dump(traps, f, indent=1, ensure_ascii=False); f.write("\n")

lp = f"{F}/🧵️loft/🔣️.json"
d = json.load(open(lp, encoding="utf8"))
names = {c["name"] for c in d["cases"]}
base = next(c for c in d["cases"] if c["name"] == "cylinder")
def case(name, pts, r, **extra):
    c = json.loads(json.dumps(base))
    c["name"] = name
    c["document"]["id"] = name
    c["document"]["solids"][0]["sections"] = [{"at": p_, "rx": r, "ry": r} for p_ in pts]
    c["document"]["solids"][0].update(extra)
    lo = [min(p_[i] for p_ in pts) - r for i in range(3)]
    hi = [max(p_[i] for p_ in pts) + r for i in range(3)]
    c["expected"] = {"volume": cyl(r, path_len(pts)), "bounds": {"min": lo, "max": hi}, "closed": True}
    c["volumeRelativeTolerance"] = 0.08
    c["boundsTolerance"] = 0.05
    return c
added = [case("nozzle-first-tangent-up-net-travel-down", nozzle, 0.03), case("nozzle-reversed", nozzle[::-1], 0.03), case("arch-from-the-top-down", arch[::-1], 0.03)]
d["cases"] = [c for c in d["cases"] if c["name"] not in {a["name"] for a in added}] + added
with open(lp, "w", encoding="utf8") as f:
    json.dump(d, f, indent=1, ensure_ascii=False); f.write("\n")
