# [scratch] writes 10-rig.json lines for the horse from a compact authored bone table (one near/far pair per limb)
import json
B = "buchner-1997"
def m(kg, cx, cy, d="uniform"): return {"kg": kg, "comX": cx, "comY": cy, "distribution": d, "source": "authored-estimate", "reference": B}
def bone(i, p, en, de, x, y, a, L, lo, hi, mass): return {"id": i, "parent": p, "title": {"en": en, "de": de}, "x": x, "y": y, "angleDeg": a, "lengthM": L, "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}, "mass": mass}
rows = [
 bone("pelvis", None, "Pelvis and hip joint", "Becken und Hüftgelenk", 0, 0, 0, 0.2, -25, 25, m(100, 0.0, -0.02, "point")),
 bone("lumbar", "pelvis", "Lumbar spine", "Lendenwirbelsäule", 0.2, 0, 0, 0.3, -12, 12, m(55, 0.15, -0.03)),
 bone("thorax", "lumbar", "Thoracic spine and ribcage", "Brustwirbelsäule und Brustkorb", 0.3, 0, 0, 0.7, -10, 10, m(160.9, 0.35, -0.1, "point")),
 bone("neck-1", "thorax", "Neck base (cervical 5-7)", "Halsbasis (Halswirbel 5-7)", 0.66, 0.13, 40, 0.34, -120, 25, m(18, 0.17, 0)),
 bone("neck-2", "neck-1", "Mid neck (cervical 3-4)", "Halsmitte (Halswirbel 3-4)", 0.34, 0, 12, 0.28, -50, 40, m(14, 0.14, 0)),
 bone("neck-3", "neck-2", "Upper neck (cervical 1-2, poll)", "Oberhals (Halswirbel 1-2, Genick)", 0.28, 0, -8, 0.22, -45, 35, m(11, 0.11, 0)),
 bone("head", "neck-3", "Head", "Kopf", 0.22, 0, -88, 0.58, -30, 90, m(26, 0.28, -0.02, "point")),
 bone("jaw", "head", "Lower jaw", "Unterkiefer", 0.16, -0.075, 5, 0.4, -38, 2, m(3, 0.2, -0.01, "point")),
 bone("ear-n", "head", "Near ear", "Nahes Ohr", 0.03, 0.095, 125, 0.15, -60, 60, m(0.15, 0.08, 0, "point")),
 bone("ear-f", "head", "Far ear", "Fernes Ohr", 0.03, 0.095, 118, 0.15, -60, 60, m(0.15, 0.08, 0, "point")),
 bone("tail-1", "pelvis", "Tail root (dock)", "Schweifrübe", -0.26, 0.14, -118, 0.22, -50, 70, m(1.6, 0.11, 0)),
 bone("tail-2", "tail-1", "Tail, upper third", "Schweif, oberes Drittel", 0.22, 0, 10, 0.22, -45, 60, m(1.2, 0.11, 0)),
 bone("tail-3", "tail-2", "Tail, middle third", "Schweif, mittleres Drittel", 0.22, 0, 8, 0.24, -45, 55, m(0.8, 0.12, 0)),
 bone("tail-4", "tail-3", "Tail, hair tip", "Schweif, Haarspitze", 0.24, 0, 5, 0.22, -40, 50, m(0.4, 0.11, 0)),
]
for s, en, de, z in (("f", "Far", "Fern", 0), ("n", "Near", "Nah", 1)):
    e, d = ("Far", "Fern") if s == "f" else ("Near", "Nah")
    g = "e" if True else ""
    rows += [
     bone(f"scapula-f{s}", "thorax", f"{e} shoulder blade", f"{d}es Schulterblatt", 0.42, 0.3, -62, 0.52, -25, 30, m(14, 0.26, 0)),
     bone(f"humerus-f{s}", f"scapula-f{s}", f"{e} upper arm (humerus)", f"{d}er Oberarm", 0.52, 0, -63, 0.34, -35, 70, m(10, 0.17, 0)),
     bone(f"radius-f{s}", f"humerus-f{s}", f"{e} forearm (radius)", f"{d}er Unterarm", 0.34, 0, 37, 0.4, -37, 125, m(8, 0.2, 0)),
     bone(f"cannon-f{s}", f"radius-f{s}", f"{e} fore cannon (carpus and third metacarpal)", f"{d}es Vorderröhrbein (Vorderfußwurzel und Mittelhandknochen)", 0.4, 0, -2, 0.295, -125, 8, m(2.2, 0.15, 0)),
     bone(f"pastern-f{s}", f"cannon-f{s}", f"{e} fore pastern (fetlock joint)", f"{d}es Vorderfesselbein (Fesselgelenk)", 0.295, 0, 35, 0.13, -70, 35, m(1.0, 0.065, 0)),
     bone(f"hoof-f{s}", f"pastern-f{s}", f"{e} fore hoof (coffin bone)", f"{d}er Vorderhuf (Hufbein)", 0.13, 0, 3, 0.095, -40, 40, m(1.3, 0.04, -0.03, "point")),
    ]
    rows += [
     bone(f"thigh-h{s}", "pelvis", f"{e} thigh (femur)", f"{d}er Oberschenkel", 0, 0, -60, 0.42, -45, 60, m(26, 0.21, 0)),
     bone(f"shank-h{s}", f"thigh-h{s}", f"{e} gaskin (tibia)", f"{d}er Unterschenkel", 0.42, 0, -65, 0.52, -45, 65, m(11, 0.26, 0)),
     bone(f"cannon-h{s}", f"shank-h{s}", f"{e} hind cannon (hock and third metatarsal)", f"{d}es Hinterröhrbein (Sprunggelenk und Mittelfußknochen)", 0.52, 0, 45, 0.34, -45, 85, m(2.8, 0.17, 0)),
     bone(f"pastern-h{s}", f"cannon-h{s}", f"{e} hind pastern (fetlock joint)", f"{d}es Hinterfesselbein (Fesselgelenk)", 0.34, 0, 22, 0.14, -65, 35, m(1.1, 0.07, 0)),
     bone(f"hoof-h{s}", f"pastern-h{s}", f"{e} hind hoof (coffin bone)", f"{d}er Hinterhuf (Hufbein)", 0.14, 0, 8, 0.09, -40, 40, m(1.5, 0.04, -0.03, "point")),
    ]
print(sum(r["mass"]["kg"] for r in rows))
out = '      "rig": {\n        "kind": "anatomical",\n        "root": "pelvis",\n        "bones": [\n' + ",\n".join("          " + json.dumps(r, ensure_ascii=False) for r in rows) + "\n        ]\n      },\n"
open("10-rig.json", "w").write(out)
