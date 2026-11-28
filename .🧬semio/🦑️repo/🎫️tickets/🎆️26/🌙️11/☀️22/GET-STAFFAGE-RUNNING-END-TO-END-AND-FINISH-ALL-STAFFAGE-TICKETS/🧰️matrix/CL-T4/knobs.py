"""🔘️ The ops of a figure: the `remove` ops of `within.ts` for its enclosed joint and mass solids, then per scale one `facet` op that authors every other joint and mass sphere or stub of the structure with `segments` facets (spheres straddle their reference outline, stubs are inscribed), then the decisions: knobs.py <asset file.json> <within ops.json> <segments> <out ops.json>"""
import json, re, sys
asset = json.load(open(sys.argv[1]))["asset"]; removes = json.load(open(sys.argv[2])); n = int(sys.argv[3]); ops = list(removes)
gone = {op["scales"][0]: re.compile("^(?:" + op["remove"] + ")$") for op in removes}
def stout(s, scale, factor):
    """Whether the solid stays above the smallest feature of the scale when faceted (a faceted body is up to 8 % thinner across its flats)."""
    thin = 2 * (min(s["radii"]) if s.get("kind") == "ellipsoid" else min(s["r0"], s.get("r1", s["r0"])))
    return thin >= factor * 0.00015 * int(scale)
for scale in ["50", "100", "200", "500"]:
    ids = [s["id"] for s in asset["volumes"][scale]["structure"] if s.get("part") in ("joint", "mass") and s.get("kind") in ("ellipsoid", "capsule") and stout(s, scale, 1.1) and not (int(scale) in gone and gone[int(scale)].match(s["id"]))]
    out = [s["id"] for s in asset["volumes"][scale]["structure"] if int(scale) in gone and gone[int(scale)].match(s["id"])]
    if ids: ops.append({"facet": "|".join(ids), "list": "structure", "kinds": ["ellipsoid", "capsule"], "scales": [int(scale)], "segments": n})
    en = f"Structure budget at 1:{scale}: " + (f"{len(out)} joint and mass spheres that lie wholly inside the members they tie in every pose ({', '.join(out)}) are merged into those members; " if out else "") + f"the other {len(ids)} joint and mass knobs are faceted bodies of {n} facets around instead of smooth spheres of 24 to 64, (a knob of just the smallest feature of the scale stays a smooth sphere), which leaves their outline within {round((1 - __import__('math').cos(__import__('math').pi / n)) * 100, 1)} % of their radius."
    de = f"Tragwerksbudget bei 1:{scale}: " + (f"{len(out)} Gelenk- und Massenkugeln, die in jeder Pose ganz in den Gliedern liegen, die sie verbinden ({', '.join(out)}), gehen in diesen Gliedern auf; " if out else "") + f"die übrigen {len(ids)} Gelenk- und Massenknoten sind facettierte Körper mit {n} Facetten im Umfang statt glatter Kugeln mit 24 bis 64, (ein Knoten von gerade dem kleinsten Merkmal des Massstabs bleibt eine glatte Kugel), ihr Umriss bleibt innerhalb von {str(round((1 - __import__('math').cos(__import__('math').pi / n)) * 100, 1)).replace('.', ',')} % ihres Radius."
    d = 0.00004 * int(scale); members = [s for s in asset["volumes"][scale]["structure"] if s.get("part") == "member" and not s.get("slender") and s.get("kind") in ("ellipsoid", "capsule") and stout(s, scale, 1.1)]
    e12 = [s["id"] for s in members if s.get("kind") == "ellipsoid" and 0.0341 * max(s["radii"]) <= d]
    e16 = [s["id"] for s in members if s.get("kind") == "ellipsoid" and 0.0341 * max(s["radii"]) > d and 0.0193 * max(s["radii"]) <= d]
    thin = [s["id"] for s in members if s.get("kind") == "capsule" and max(s["r0"], s.get("r1", s["r0"])) <= 0.00015 * int(scale)]
    for ids2, n2 in ((e12, 12), (e16, 16)):
        if ids2: ops.append({"facet": "|".join(ids2), "list": "structure", "kinds": ["ellipsoid"], "scales": [int(scale)], "segments": n2})
    if thin: ops.append({"facet": "|".join(thin), "list": "structure", "kinds": ["capsule"], "scales": [int(scale)], "segments": 8})
    arcs = [s for s in asset["volumes"][scale]["structure"] if s.get("part") == "member" and s.get("kind") == "revolve"]
    for n3, limit in ((16, 0.0193), (24, 0.0086)):
        ids3 = [s["id"] for s in arcs if limit * max(abs(p[1]) for p in s["profile"]) * s.get("lateralScale", 1) <= d and not (n3 == 24 and 0.0193 * max(abs(p[1]) for p in s["profile"]) * s.get("lateralScale", 1) <= d)]
        if ids3:
            ops.append({"facet": "|".join(ids3), "list": "structure", "kinds": ["revolve"], "scales": [int(scale)], "segments": n3})
            en += f" {len(ids3)} rib arcs are swept in steps of 1/{n3} of a turn, within 0.04 mm of paper of their arc."
            de += f" {len(ids3)} Rippenbögen sind in Schritten von 1/{n3} einer Umdrehung gezogen, innerhalb von 0,04 mm Papier ihres Bogens."
            print(scale, "arcs", n3, len(ids3))
    en += f" Of the members, {len(e12)} ellipsoid bone masses are faceted bodies of 12 facets around and {len(e16)} of 16, each within 0.04 mm of paper of its ellipsoid at this scale, and {len(thin)} short round bones at most 0.3 mm of paper across are prisms of 8 facets with pointed ends, within 0.012 mm of paper of their capsule; the long shafts, the lofted, extruded and revolved members stay as they were."
    de += f" Von den Gliedern sind {len(e12)} ellipsoide Knochenmassen facettierte Körper mit 12 Facetten im Umfang und {len(e16)} mit 16, jede innerhalb von 0,04 mm Papier ihres Ellipsoids in diesem Massstab, und {len(thin)} kurze runde Knochen, höchstens 0,3 mm Papier im Durchmesser, sind Prismen mit 8 Facetten und spitzen Enden, innerhalb von 0,012 mm Papier ihrer Kapsel; die langen Schäfte und die gelofteten, extrudierten und rotierten Glieder bleiben, wie sie waren."
    ops.append({"decision": {"en": en, "de": de}, "scales": [int(scale)]})
    print(scale, "knobs", len(ids), "removed", len(out), "e12", len(e12), "e16", len(e16), "thin capsules", len(thin))
json.dump(ops, open(sys.argv[4], "w"), ensure_ascii=False, indent=1); print(sys.argv[4], len(ops), "ops")
