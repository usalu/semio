"""🏗️ Repair of `reclaimed-lattice-girder` at 1:50. (1) Front elevation: the cut bracing stub was drawn with a diagonal from its upright leg to the tip of its flat leg; the body is a step (the upright leg ends at 178.8 mm, the 6 mm flat leg runs on to 213.5 mm; 19.9 mm apart): the outline follows the step. (2) Plan: the two slotted holes (22 x 72 mm) of the right bearing plate are open to the ground beside the chord and were filled (35.6 mm): the end piece of the plan is drawn with the two slots open. (3) The two 30 mm round slings were declared `slender`, which compares them as strokes with square caps half a width beyond their rounded tips (26.3 mm in the side elevation): at 1:50 they are four smallest features thick, bodies of their own, and are compared as the capsules they are; the front and the plan draw them to their tips. (4) The 434 snap rivet heads were whole ellipsoids of 528 triangles each (229 000 of the 234 000 triangles of the body, a rendered view of 11.6 MB, which the librsvg reference of the guard cannot read): each head is the same ellipsoid of revolution as a revolve of seven profile points about its shank axis (26 mm across, 10.5 mm high on either side, within 0.5 mm of the ellipsoid), same place and repeat. Idempotent: python3 fix_reclaimed_lattice_girder.py"""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-lattice-girder"
text, doc = sc.load(ID)
n = lambda v: format(sc.r6(v), "g")

end = sc.shape(doc, 50, "front", "girder-end-04")
points = [p for p in end["points"] if p != [0.1788, 0.144]]
at = points.index([0.1788, 0.178])
end["points"] = points[:at] + [[0.1788, 0.144]] + points[at:]

top = sc.shape(doc, 50, "plan", "girder-top-20")
OUTLINE = [[9.7775, 0.085], [9.7775, 0.005], [9.905, 0.005], [9.905, -0.005], [9.7775, -0.005], [9.7775, -0.085], [10.0, -0.085], [10.0, -0.13], [10.24, -0.13], [10.24, 0.13], [10.0, 0.13], [10.0, 0.085]]
rings = [OUTLINE] + [[[p[1], p[0]] for p in hole] for hole in sc.solid(doc, 50, "bearing-plate-right")["holes"]]
top.pop("points", None)
top.pop("closed", None)
top["kind"] = "path"
top["d"] = " ".join("M " + " L ".join(f"{n(x)} {n(y)}" for x, y in ring) + " Z" for ring in rings)

for name in ("sling-a", "sling-b"):
    sc.solid(doc, 50, f"{name}-leg").pop("slender", None)
    for view in ("front", "plan"):
        sc.shape(doc, 50, view, name)["points"] = [[-0.015, -0.015], [2.415, -0.015], [2.415, 0.015], [-0.015, 0.015]]
DOME = [[-0.0105, 0.0], [-0.009093, 0.0065], [-0.00525, 0.011258], [0.0, 0.013], [0.00525, 0.011258], [0.009093, 0.0065], [0.0105, 0.0]]
heads = 0
for solid in sc.solids(doc, 50):
    if not solid["id"].startswith("rivet"):
        continue
    if solid["kind"] == "ellipsoid":
        radii = solid.pop("radii")
        assert sorted(radii) == [0.0105, 0.013, 0.013], radii
        axis = [1.0 if r == 0.0105 else 0.0 for r in radii]
        rebuilt = {}
        for key, value in solid.items():
            rebuilt[key] = value
            if key == "kind":
                rebuilt["kind"] = "revolve"
                rebuilt["profile"] = DOME
                rebuilt["axis"] = axis
                rebuilt["origin"] = [0.0, 0.0, 0.0]
        solid.clear()
        solid.update(rebuilt)
    assert solid["kind"] == "revolve" and solid["profile"] == DOME, solid["id"]
    heads += solid.get("repeat", {}).get("count", 1)
assert heads == 434, heads
sc.save(ID, text, doc)
