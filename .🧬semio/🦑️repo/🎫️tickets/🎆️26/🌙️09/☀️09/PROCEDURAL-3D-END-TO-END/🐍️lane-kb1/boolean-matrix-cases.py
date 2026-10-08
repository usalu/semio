"""Emits the specification half of the boolean differential fixture (no expected values; the manifold-3d script fills those)."""
import json, math, sys

def rot(r, v):
    x, y, z = v
    a = math.radians(r[0]); y, z = y * math.cos(a) - z * math.sin(a), y * math.sin(a) + z * math.cos(a)
    b = math.radians(r[1]); x, z = x * math.cos(b) + z * math.sin(b), -x * math.sin(b) + z * math.cos(b)
    c = math.radians(r[2]); x, y = x * math.cos(c) - y * math.sin(c), x * math.sin(c) + y * math.cos(c)
    return [x, y, z]

def at(spec, t=None, r=None):
    spec = dict(spec)
    if r: spec["rotate"] = list(r)
    if t: spec["translate"] = [round(c, 12) for c in t]
    return spec

def centered(spec, local_center, r):
    rc = rot(r, local_center)
    return at(spec, [-c for c in rc], r)

def box(w, d, h, t=None, r=None): return at({"kind": "box", "size": [w, d, h]}, rot(r, t) if (t and r) else t, r)
def sphere(rad, t=None, r=None): return at({"kind": "sphere", "radius": rad}, t, r)
def cyl(rad, h, t=None, r=None): return at({"kind": "cylinder", "radius": rad, "height": h}, t, r)
def cone(rad, h, t=None, r=None): return at({"kind": "cone", "radius": rad, "height": h}, t, r)
def torus(major, minor, t=None, r=None): return at({"kind": "torus", "major": major, "minor": minor}, t, r)
def prism(poly, h, t=None, r=None): return at({"kind": "prism", "polygon": poly, "height": h}, t, r)
def revolved(profile, t=None, r=None): return at({"kind": "revolved", "profile": profile}, t, r)

OPS = ["fuse", "cut", "intersect"]
cases = []
def pair(name, a, b, ops=OPS, tags=()):
    for op in ops:
        cases.append({"id": f"{name}-{op}", "operation": op, "a": a, "b": b, "tags": list(tags)})

S = sphere(2)
# sphere x box
pair("sphere-box-poles", S, box(3, 6, 6, [0, -3, -3]), tags=["pole-crossing"])
pair("sphere-box-poles-negative-side", S, box(3, 6, 6, [-3, -3, -3]), tags=["pole-crossing", "seam"])
pair("sphere-box-poles-diagonal", S, box(3, 6, 6, [0, -3, -3], [0, 0, 45]), tags=["pole-crossing"])
pair("sphere-box-equator", S, box(6, 6, 3, [-3, -3, 0]), tags=["seam"])
pair("sphere-box-cap-x", S, box(3, 6, 6, [1, -3, -3]), tags=["seam"])
pair("sphere-box-cap-pole", S, box(6, 6, 3, [-3, -3, 1.2]), tags=["pole-crossing"])
pair("sphere-box-corner", S, box(3, 3, 3, [1, 1, 1]))
pair("sphere-box-tilted", S, box(6, 6, 3, [-3, -3, 0], [0, 30, 0]), tags=["seam"])
pair("sphere-box-pole-poke", S, box(1, 1, 3, [-0.5, -0.5, 0]), tags=["pole-crossing"])
pair("sphere-box-nested-outside", S, box(6, 6, 6, [-3, -3, -3]), tags=["nested"])
pair("sphere-box-inscribed", S, box(4, 4, 4, [-2, -2, -2]), tags=["tangent"])
pair("sphere-box-nested-inside", S, box(2, 2, 2, [-1, -1, -1]), tags=["nested"])
pair("sphere-box-disjoint", S, box(1, 1, 1, [5, 0, 0]), tags=["disjoint"])
pair("sphere-box-touching-point", S, box(1, 2, 2, [2, -1, -1]), tags=["tangent"])
# sphere x cylinder
pair("sphere-cylinder-axial", S, cyl(1, 6, [0, 0, -3]), tags=["pole-crossing"])
pair("sphere-cylinder-lateral", S, cyl(1, 6, [-3, 0, 0], [0, 90, 0]), tags=["seam"])
pair("sphere-cylinder-offset", S, cyl(0.6, 6, [1, 0, -3]))
pair("sphere-cylinder-skew", S, centered(cyl(0.5, 6), [0, 0, 3], [30, 0, 0]))
pair("sphere-cylinder-blind", S, cyl(1, 3, [0, 0, 0.5]), tags=["pole-crossing"])
pair("sphere-cylinder-tangent-circle", S, cyl(2, 6, [0, 0, -3]), tags=["tangent"])
# sphere x cone
pair("sphere-cone-axial", S, cone(1.5, 4, [0, 0, -1.5]), tags=["pole-crossing"])
pair("sphere-cone-apex-inside", S, cone(1.5, 3, [0, 0, -3]), tags=["pole-crossing"])
pair("sphere-cone-offset", S, cone(1, 2, [1, 0, -0.5]))
# sphere x torus
T = torus(2, 0.5)
pair("sphere-torus-equatorial", S, T)
pair("torus-sphere-in-tube", T, sphere(0.4, [2, 0, 0]), tags=["nested"])
pair("torus-sphere-crossing-tube", T, sphere(0.8, [2, 0, 0]))
pair("torus-sphere-hole", T, sphere(1, [0, 0, 0]), tags=["disjoint"])
# box x box
B2 = box(2, 2, 2)
pair("box-box-corner-overlap", B2, box(2, 2, 2, [1, 1, 1]))
pair("box-box-face-share", B2, box(2, 2, 2, [2, 0, 0]), tags=["coincident"])
pair("box-box-face-partial", B2, box(2, 2, 2, [2, 1, 0]), tags=["coincident"])
pair("box-box-nested", B2, box(1, 1, 1, [0.5, 0.5, 0.5]), tags=["nested"])
pair("box-box-flush-corner", B2, box(1, 1, 1), tags=["coincident"])
pair("box-box-flush-face", B2, box(1, 1, 1, [0, 0.5, 0.5]), tags=["coincident"])
pair("box-box-edge-touch", B2, box(2, 2, 2, [2, 2, 0]), tags=["tangent"])
pair("box-box-vertex-touch", B2, box(2, 2, 2, [2, 2, 2]), tags=["tangent"])
pair("box-box-cross", box(4, 1, 1, [-2, -0.5, -0.5]), box(1, 4, 1, [-0.5, -2, -0.5]))
# cylinder x box
C1 = cyl(1, 2)
pair("cylinder-box-through", cyl(0.5, 4, [0, 0, -2]), box(4, 4, 2, [-2, -2, -1]))
pair("cylinder-box-blind", box(2, 2, 2), cyl(0.4, 1.5, [1, 1, 1.5]))
pair("cylinder-box-half-axis", C1, box(2, 4, 4, [0, -2, -1]), tags=["seam"])
pair("cylinder-box-half-seam", C1, box(4, 2, 4, [-2, 0, -1]), tags=["seam"])
pair("cylinder-box-chord", C1, box(2.5, 4, 4, [0.5, -2, -1]))
pair("cylinder-box-chord-flush-caps", C1, box(2.5, 4, 2, [0.5, -2, 0]), tags=["coincident"])
pair("cylinder-box-tangent", C1, box(2, 2, 2, [1, -1, 0]), tags=["tangent", "coincident"])
pair("cylinder-box-partially-outside", box(2, 2, 2), cyl(0.6, 3, [1.6, 1, -0.5]))
pair("cylinder-box-stacked", box(2, 2, 2), cyl(1, 1, [1, 1, 2]), tags=["coincident"])
pair("cylinder-box-stacked-overhang", box(2, 2, 2), cyl(1.5, 1, [1, 1, 2]), tags=["coincident"])
# cylinder x cylinder
pair("cylinder-cylinder-cross-unequal", cyl(1, 4, [-2, 0, 0], [0, 90, 0]), cyl(0.6, 4, [0, 0, -2]))
pair("cylinder-cylinder-cross-equal", cyl(1, 4, [-2, 0, 0], [0, 90, 0]), cyl(1, 4, [0, 0, -2]), tags=["tangent"])
pair("cylinder-cylinder-parallel", C1, cyl(1, 2, [1, 0, 0]))
pair("cylinder-cylinder-stacked", cyl(1, 1), cyl(1, 1, [0, 0, 1]), tags=["coincident"])
pair("cylinder-cylinder-bored", C1, cyl(0.5, 3, [0, 0, -0.5]), tags=["nested"])
pair("cylinder-cylinder-bored-flush", C1, cyl(0.5, 2), tags=["coincident", "nested"])
pair("cylinder-cylinder-tee", cyl(1, 4, [0, 0, -2]), cyl(0.5, 2, [0, 0, 0], [0, 90, 0]))
# cone
K = cone(2, 3)
pair("cone-box-band", K, box(6, 6, 2, [-3, -3, 0.5]))
pair("cone-box-half-through-apex", K, box(3, 6, 4, [0, -3, -0.5]), tags=["pole-crossing"])
pair("cone-box-oblique", K, box(6, 6, 3, [-3, -3, 0], [0, 20, 0]), tags=["seam"])
pair("cone-cylinder-coaxial", K, cyl(0.8, 4, [0, 0, -0.5]), tags=["pole-crossing"])
pair("cone-cylinder-stacked", cyl(1, 1), cone(1, 1, [0, 0, 1]), tags=["coincident"])
# torus
pair("torus-box-half-equator", T, box(8, 8, 2, [-4, -4, 0]), tags=["seam", "coincident"])
pair("torus-box-half-meridian", T, box(4, 8, 4, [0, -4, -2]), tags=["seam"])
pair("torus-box-quarter", T, box(4, 4, 4, [0, 0, 0]), tags=["seam"])
pair("torus-box-offset-z", T, box(8, 8, 2, [-4, -4, 0.25]))
pair("torus-box-offset-x", T, box(4, 8, 4, [2, -4, -2]))
pair("torus-box-tangent-top", T, box(8, 8, 2, [-4, -4, 0.5]), tags=["tangent"])
pair("torus-cylinder-through-tube", T, cyl(1.7, 2, [0, 0, -1]))
pair("torus-cylinder-inside-hole", T, cyl(1, 2, [0, 0, -1]), tags=["disjoint"])
pair("torus-torus-linked", T, torus(2, 0.5, [2, 0, 0], [90, 0, 0]), tags=["stress"])
pair("torus-torus-stacked-tangent", T, torus(2, 0.5, [0, 0, 1]), tags=["tangent"])
# extruded polygon
L = [[0, 0], [3, 0], [3, 1], [1, 1], [1, 3], [0, 3]]
P = prism(L, 2)
pair("prism-box-slot", P, box(5, 0.6, 1, [-1, 0.2, 0.5]), tags=["concave"])
pair("prism-box-reflex-corner", P, box(2, 2, 3, [0.5, 0.5, -0.5]), tags=["concave"])
pair("prism-sphere-reflex-corner", P, sphere(1.5, [1, 1, 1]), tags=["concave"])
pair("prism-cylinder-reflex-corner", P, cyl(0.5, 4, [1.4, 1.4, -1]), tags=["concave"])
HEX = [[1.5 * math.cos(math.radians(60 * i)), 1.5 * math.sin(math.radians(60 * i))] for i in range(6)]
pair("hexagon-sphere-bumps", prism(HEX, 2), sphere(1.4, [0, 0, 1]))
# revolved
R1 = revolved([[1, 0], [2, 0], [2, 1], [1, 1]])
R2 = revolved([[1, 0], [2, 0], [2.5, 1], [2, 2], [1, 2]])
pair("revolved-box-half", R1, box(3, 6, 3, [0, -3, -1]), tags=["seam"])
pair("revolved-box-slot", R1, box(0.4, 6, 3, [-0.2, -3, -1]))
pair("revolved-sphere", R1, sphere(1.5, [0, 0, 0.5]))
pair("revolved-cones-box-half", R2, box(3, 6, 4, [0, -3, -1]), tags=["seam"])
pair("revolved-cones-sphere", R2, sphere(2.2, [0, 0, 1]))
pair("revolved-torus-nested", R1, torus(1.5, 0.3, [0, 0, 0.5]), tags=["nested"])
# identical
for name, shape in [("box", box(2, 2, 2)), ("sphere", sphere(1.5)), ("cylinder", cyl(1, 2)), ("cone", cone(1, 2)), ("torus", torus(2, 0.5)), ("prism", P), ("revolved", R1)]:
    pair(f"{name}-identical", shape, shape, tags=["coincident", "identical"])

json.dump({"cases": cases}, sys.stdout, indent=1, ensure_ascii=False)
