import sys, pathlib, json
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1996", "m12.eccentricity-limit", "6.1.2.2", "e_decl <= x t", 0.2, 0.2, "t", "Limit of the declared load eccentricity at the wall head and foot as a fraction of the thickness t, as coded by the artifact (0.2); recalled, not checked against the print, no national value recorded.", part="1-1")
for band, (lo, hi) in {"general": (0.006, 0.015), "thin": (0.0005, 0.003)}.items():
    for edge, v in (("min", lo), ("max", hi)):
        put(d, "1996", f"m70.bed-joint.{band}.{edge}", "EN 1996-2 3.6 / 8", f"t_bed {edge} ({band})", v, v, "m", f"Admissible bed joint thickness {edge} for {'general purpose and lightweight' if band == 'general' else 'thin layer'} mortar as coded by the artifact; recalled, not checked against the print.", part="2")
put(d, "1996", "m01.conformance", "EN 1996-1-1 3.1 / DE NA", "conformance rules", None, None, "", "Conformance rules of the wall material check as coded by the artifact: mortar strength within 15 % of its class, least unit dimension >= 90 mm, group 2 height/width < 2.5, unit strength f_b within 1.5-10 MPa (aerated), 4-100 MPa (clay), 2-100 MPa (other); recalled, not checked against the print.", part="1-1")
sys.path.insert(0, "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📚️masonry-tables/📐️annex-e")
classes = [c["code"] for c in json.load(open("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📚️masonry-tables/📐️annex-e/🔣️.json"))["classes"]]
for code in classes:
    put(d, "1996", f"annex-e.{code.lower()}", "Annex E", f"alpha1 cells {code}", None, None, "", f"Bending moment coefficients alpha1 of EN 1996-1-1 Annex E for the edge class {code} (F free, S simple, X fixed, canonical order); the cells are not keyed, the code uses the Johansen yield-line stand-in.", part="1-1")
save(d)
print(len(d))
