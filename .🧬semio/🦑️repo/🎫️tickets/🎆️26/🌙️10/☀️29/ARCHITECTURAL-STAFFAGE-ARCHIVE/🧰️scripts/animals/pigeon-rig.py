# [scratch] emits pigeon/10-rig.json from authored bone data (near and far chains mirrored from one definition)
import json, sys
A = sys.argv[1]
def B(id, parent, en, de, x, y, ang, ln, lo, hi, kg, com, dist="uniform", cy=0.0, ref="pennycuick-2008"):
    m = {"kg": kg, "comX": com, "comY": cy, "distribution": dist, "source": "authored-estimate"}
    if ref: m["reference"] = ref
    return {"id": id, "parent": parent, "title": {"en": en, "de": de}, "x": x, "y": y, "angleDeg": ang, "lengthM": ln, "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}, "mass": m}
bones = [
 B("pelvis", None, "Pelvis and hip joints (synsacrum)", "Becken und Hüftgelenke (Synsacrum)", 0, 0, 0, 0.07, -30, 30, 0.030, 0.0, "point", 0.0),
 B("trunk", "pelvis", "Trunk with keel and flight muscles", "Rumpf mit Brustbeinkamm und Flugmuskeln", -0.05, 0.0, 16, 0.165, -40, 35, 0.174, 0.085, "point", 0.0),
 B("neck-1", "trunk", "Lower neck", "Unterer Hals", 0.13, 0.03, 39, 0.03, -70, 45, 0.012, 0.02),
 B("neck-2", "neck-1", "Middle neck", "Mittlerer Hals", 0.03, 0, 40, 0.025, -70, 50, 0.008, 0.02),
 B("neck-3", "neck-2", "Upper neck", "Oberer Hals", 0.025, 0, -45, 0.02, -60, 60, 0.006, 0.015),
 B("head", "neck-3", "Head with upper beak", "Kopf mit Oberschnabel", 0.02, 0, -60, 0.054, -75, 60, 0.022, 0.022, "point", 0.0),
 B("jaw", "head", "Lower beak (mandible)", "Unterschnabel (Mandibula)", 0.034, -0.006, -3, 0.02, -30, 0, 0.003, 0.01, "uniform", 0.0),
 B("tail", "trunk", "Tail (rectrices)", "Schwanz (Steuerfedern)", -0.02, -0.004, 184, 0.105, -45, 35, 0.010, 0.06),
 B("tail-f", "tail", "Far tail feathers (fan)", "Ferne Schwanzfedern (Fächer)", 0.0, 0.0, 0, 0.105, -5, 30, 0.004, 0.06),
]
for s, en, de in (("f", "Far", "Fern"), ("n", "Near", "Nah")):
    ge = lambda a, b: (a if s == "n" else b)
    bones += [
     B(f"thigh-{s}", "pelvis", f"{en} thigh (femur)", f"{de}er Oberschenkel (Femur)", 0, 0, -25, 0.045, -60, 60, 0.010, 0.0225, "uniform"),
     B(f"shank-{s}", f"thigh-{s}", f"{en} shank (tibiotarsus)", f"{de}er Unterschenkel (Tibiotarsus)", 0.045, 0, -87, 0.055, -60, 55, 0.007, 0.0275),
     B(f"tarsus-{s}", f"shank-{s}", f"{en} tarsometatarsus", f"{de}er Lauf (Tarsometatarsus)", 0.055, 0, 32, 0.028, -50, 70, 0.0025, 0.015),
     B(f"toes-{s}", f"tarsus-{s}", f"{en} toes (middle toe and hallux)", f"{de}e Zehen (Mittelzehe und Hinterzehe)", 0.028, 0, 77, 0.036, -60, 45, 0.002, 0.012, "point", -0.002),
     B(f"humerus-{s}", "trunk", f"{en} upper arm (humerus)", f"{de}er Oberarm (Humerus)", 0.1, 0.012, 176, 0.06, -150, 100, 0.007, 0.03),
     B(f"forearm-{s}", f"humerus-{s}", f"{en} forearm (ulna and secondaries)", f"{de}er Unterarm (Elle und Armschwingen)", 0.06, 0, -167, 0.075, -10, 178, 0.006, 0.038),
     B(f"hand-{s}", f"forearm-{s}", f"{en} hand (carpometacarpus and primaries)", f"{de}e Hand (Handwurzel-Mittelhand und Handschwingen)", 0.075, 0, 163, 0.15, -165, 10, 0.006, 0.07),
    ]
out = '      "rig": {\n        "kind": "anatomical",\n        "root": "pelvis",\n        "bones": [\n' + ",\n".join("          " + json.dumps(b, ensure_ascii=False) for b in bones) + '\n        ]\n      },\n'
open(A + "/pigeon/10-rig.json", "w").write(out)
print(len(bones), "bones", round(sum(b["mass"]["kg"] for b in bones), 4), "kg")
