# [scratch] writes the dairy-cow rig and anchor fragments from authored bone tables
import json, sys
D = sys.argv[1]
def seg(kg, cx, cy, dist="point", ref="dyce-2010"): return {"kg": kg, "comX": cx, "comY": cy, "distribution": dist, "source": "authored-estimate", **({"reference": ref} if ref else {})}
def bone(i, p, x, y, a, l, lo, hi, en, de, m=None):
    b = {"id": i, "parent": p, "title": {"en": en, "de": de}, "x": x, "y": y, "angleDeg": a, "lengthM": l, "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}}
    if m: b["mass"] = m
    return b
B = []
B.append(bone("pelvis", None, 0, 0, 0, 0.25, -20, 20, "Pelvis, hindquarters and hip joint", "Becken, Hinterhand und Hüftgelenk", seg(85, -0.04, -0.12)))
B.append(bone("lumbar", "pelvis", 0.25, 0, 0, 0.45, -15, 20, "Lumbar spine and flank with rumen", "Lendenwirbelsäule und Flanke mit Pansen", seg(125, 0.22, -0.22)))
B.append(bone("thorax", "lumbar", 0.45, 0, 1, 0.62, -12, 12, "Thoracic spine, ribcage and brisket", "Brustwirbelsäule, Brustkorb und Brustbein", seg(205.3, 0.3, -0.16)))
B.append(bone("neck", "thorax", 0.55, 0.12, 5, 0.68, -75, 35, "Neck with dewlap", "Hals mit Triel", seg(40, 0.34, -0.02, "uniform")))
B.append(bone("head", "neck", 0.68, 0, -70, 0.5, -45, 55, "Head", "Kopf", seg(25, 0.22, -0.02)))
B.append(bone("jaw", "head", 0.14, -0.07, 3, 0.33, -25, 3, "Lower jaw", "Unterkiefer", seg(4, 0.17, 0)))
for s, en, de in (("n", "Near", "Nahes"), ("f", "Far", "Fernes")):
    B.append(bone(f"ear-{s}", "head", 0.03, 0.03, -126, 0.2, -40, 50, f"{en} ear", f"{de} Ohr", seg(0.3, 0.1, 0, ref=None)))
B.append(bone("tail-1", "pelvis", -0.27, 0.27, -100, 0.3, -60, 70, "Tail root", "Schwanzwurzel", seg(1.4, 0.15, 0, "uniform")))
B.append(bone("tail-2", "tail-1", 0.3, 0, 8, 0.45, -45, 55, "Tail middle", "Schwanzmitte", seg(0.8, 0.22, 0, "uniform")))
B.append(bone("tail-3", "tail-2", 0.45, 0, 2, 0.3, -35, 45, "Tail switch", "Schwanzquaste", seg(0.5, 0.18, 0)))
B.append(bone("udder", "pelvis", 0.18, -0.4, -90, 0.25, -15, 95, "Udder", "Euter", seg(26, 0.1, 0, ref=None)))
for s, en, de in (("f", "Far", "Ferner"), ("n", "Near", "Naher")):
    B += [
     bone(f"thigh-h{s}", "pelvis", 0, 0, -52, 0.4, -45, 50, f"{en} thigh (femur)", f"{de} Oberschenkel", seg(22, 0.17, -0.07)),
     bone(f"shank-h{s}", f"thigh-h{s}", 0.4, 0, -74, 0.48, -90, 60, f"{en} gaskin (tibia)", f"{de} Unterschenkel (Tibia)", seg(8.5, 0.22, -0.02, "uniform")),
     bone(f"metatarsus-h{s}", f"shank-h{s}", 0.48, 0, 40, 0.29, -45, 110, f"{en} hind cannon (hock to fetlock)", f"{de} Hinterfußwurzel bis Fesselgelenk (Röhrbein)", seg(3.3, 0.15, 0, "uniform")),
     bone(f"hoof-h{s}", f"metatarsus-h{s}", 0.29, 0, 34, 0.15, -50, 45, f"{en} hind pastern and cloven hoof", f"{de} Hinterfessel und Klaue", seg(2.4, 0.08, -0.04)),
    ]
for s, en, de in (("f", "Far", "Ferne"), ("n", "Near", "Nahe")):
    B += [
     bone(f"scapula-f{s}", "thorax", 0.45, 0.27, -64, 0.46, -30, 35, f"{en} shoulder blade", f"{de} Schulterblatt", seg(14, 0.23, 0, "uniform")),
     bone(f"humerus-f{s}", f"scapula-f{s}", 0.46, 0, -85, 0.36, -40, 75, f"{en} upper arm (humerus)", f"{de} Oberarm", seg(8, 0.18, 0, "uniform")),
     bone(f"forearm-f{s}", f"humerus-f{s}", 0.36, 0, 62, 0.39, -60, 95, f"{en} forearm (radius)", f"{de} Unterarm (Speiche)", seg(5.5, 0.2, 0, "uniform")),
     bone(f"cannon-f{s}", f"forearm-f{s}", 0.39, 0, 0, 0.26, -140, 12, f"{en} fore cannon (carpus to fetlock)", f"{de} Vorderfußwurzel bis Fesselgelenk (Röhrbein)", seg(2.2, 0.13, 0, "uniform")),
     bone(f"hoof-f{s}", f"cannon-f{s}", 0.26, 0, 34, 0.15, -85, 45, f"{en} fore pastern and cloven hoof", f"{de} Vorderfessel und Klaue", seg(2.3, 0.08, -0.04)),
    ]
tot = sum(b["mass"]["kg"] for b in B if "mass" in b)
print("mass", round(tot, 3), file=sys.stderr)
def lines(items, ind): return ",\n".join(ind + json.dumps(i, ensure_ascii=False) for i in items)
open(D + "/10-rig.json", "w").write('      "rig": {\n        "kind": "anatomical",\n        "root": "pelvis",\n        "bones": [\n' + lines(B, "          ") + '\n        ]\n      },\n')
def anc(i, bn, x, y, kind, en, de, **k): return {"id": i, "bone": bn, "x": x, "y": y, "kind": kind, "title": {"en": en, "de": de}, **k}
def con(i, bn, x, y, en, de, fl, fw, mu=0.6): return anc(i, bn, x, y, "contact", en, de, support="contact", normalDeg=90, footprintLengthM=fl, footprintWidthM=fw, frictionCoefficient=mu)
A = []
for s, en, de in (("n", "near", "nahen"), ("f", "far", "fernen")):
    A.append(con(f"contact-h{s}", f"hoof-h{s}", 0.13, -0.045, f"Sole of the {en} hind cloven hoof", f"Sohle der {de} Hinterklaue", 0.12, 0.09))
for s, en, de in (("n", "near", "nahen"), ("f", "far", "fernen")):
    A.append(con(f"contact-f{s}", f"hoof-f{s}", 0.13, -0.045, f"Sole of the {en} fore cloven hoof", f"Sohle der {de} Vorderklaue", 0.13, 0.1))
for s, en, de in (("n", "near", "nahen"), ("f", "far", "fernen")):
    A.append(con(f"contact-carpus-f{s}", f"cannon-f{s}", 0.01, 0.05, f"Front of the {en} carpus (kneeling)", f"Vorderseite des {de} Vorderfußwurzelgelenks (Knien)", 0.1, 0.12))
for s, en, de in (("n", "near", "nahen"), ("f", "far", "fernen")):
    A.append(con(f"contact-hock-h{s}", f"metatarsus-h{s}", -0.035, -0.05, f"Point of the {en} hock (lying)", f"Sprunggelenkhöcker der {de} Hinterhand (Liegen)", 0.09, 0.1))
A.append(con("contact-sternum", "thorax", 0.3, -0.525, "Sternum and brisket (lying)", "Brustbein und Brustspitze (Liegen)", 0.45, 0.3, 0.5))
A.append(anc("nose", "head", 0.54, 0.0, "emitter", "Nostrils", "Nasenlöcher"))
open(D + "/20-anchors.json", "w").write('      "anchors": [\n' + lines(A, "        ") + '\n      ],\n')
