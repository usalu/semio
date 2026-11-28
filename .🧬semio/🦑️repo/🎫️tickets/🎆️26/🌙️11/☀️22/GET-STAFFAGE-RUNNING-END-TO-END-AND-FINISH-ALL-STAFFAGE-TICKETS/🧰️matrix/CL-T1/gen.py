"""🧾️ Writes `ops/<id>.json` of a tree of lane CL-T1 from three choices: `gen.py <id> <scale:radius,…> [--all16]`. Every ellipsoid of body, thermal and structure whose smallest radius is at least 0.52 of the smallest feature of its scale becomes a faceted lobe at the reference facets; the listed scales give lobes of body and thermal up to that radius 16 segments (the spheres of the structure follow one rule for all trees, growing with the scale) (`--all16`: every lobe at every scale); the decision text states exactly these numbers."""
import json, sys

asset, small = sys.argv[1], dict((int(s), float(r)) for s, r in (pair.split(":") for pair in sys.argv[2].split(",") if pair))
all16 = "--all16" in sys.argv
STRUCTURE = {50: 0.28, 100: 0.56, 200: 1.12, 500: 2.8}
ops = []
for lst in ("body", "thermal", "structure"):
    for scale in (50, 100, 200, 500):
        above = round(0.52 * 0.00015 * scale, 5)
        if all16:
            ops.append({"facet": ".*", "list": lst, "kinds": ["ellipsoid"], "segments": 16, "above": above, "scales": [scale]})
            continue
        rule = STRUCTURE if lst == "structure" else small
        if scale in rule:
            ops.append({"facet": ".*", "list": lst, "kinds": ["ellipsoid"], "segments": 16, "below": rule[scale], "above": above, "scales": [scale]})
        ops.append({"facet": ".*", "list": lst, "kinds": ["ellipsoid"], "above": above, "scales": [scale]})
en_small = " and ".join(f"{r:g} m at 1:{s}" for s, r in sorted(small.items()))
de_small = " und ".join(f"{r:g} m bei 1:{s}".replace(".", ",") for s, r in sorted(small.items()))
ops.append({"decision": {
    "en": "Budget: every soft mass (leaf lobe, flower, fruit, transpiration outlet) and every joint and mass sphere of the structure is a faceted lobe, a polyhedron with the facets of its ellipsoid at 24 to 64 segments around (the surface every check of the volumes measures), so it keeps about 530 triangles at every scale instead of the 2000 to 4000 of a smooth ellipsoid; " + (f"a lobe of at most {en_small} radius has 16 segments around and 8 rings (224 triangles) and is sized so that its outline straddles that measured surface, within 0.54 % of its radius (a joint or mass sphere of the structure up to 0.28 m at 1:50, 0.56 m at 1:100, 1.12 m at 1:200 and 2.8 m at 1:500, so that its triangles never grow towards a coarser scale); " if small else "") + "a sphere of just the smallest feature of its scale stays a smooth ellipsoid.",
    "de": "Budget: Jede weiche Masse (Laubballen, Blüte, Frucht, Transpirationsauslass) und jede Gelenk- und Massenkugel des Tragwerks ist ein facettierter Ballen, ein Polyeder mit den Facetten seines Ellipsoids bei 24 bis 64 Segmenten im Umfang (die Fläche, die jede Prüfung der Volumen misst), und behält damit in jedem Maßstab rund 530 Dreiecke statt der 2000 bis 4000 eines glatten Ellipsoids; " + (f"ein Ballen mit höchstens {de_small} Radius hat 16 Segmente im Umfang und 8 Ringe (224 Dreiecke) und ist so bemessen, dass sein Umriss diese gemessene Fläche umspielt, auf 0,54 % seines Radius genau (eine Gelenk- oder Massenkugel des Tragwerks bis 0,28 m bei 1:50, 0,56 m bei 1:100, 1,12 m bei 1:200 und 2,8 m bei 1:500, damit ihre Dreiecke zu einem gröberen Maßstab hin nie zunehmen); " if small else "") + "eine Kugel von gerade dem kleinsten Merkmal ihres Maßstabs bleibt ein glattes Ellipsoid.",
}})
json.dump(ops, open(f"ops/{asset}.json", "w"), ensure_ascii=False, indent=1)
print(asset, len(ops), "ops")
