# [scratch] expands the hand-authored roe-deer limb templates for the near and far side (mirroring only) into rig, anchor and geometry fragments
import json, math, sys, os
D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "roe-deer")
J = lambda o: json.dumps(o, ensure_ascii=False)
def T(en, de): return {"en": en, "de": de}
def hoof_local(pts):
    c, s = math.cos(math.radians(52)), math.sin(math.radians(52))
    return [(round(c*x - s*y, 4), round(s*x + c*y, 4)) for x, y in pts]
def hoof_path(world):
    out = []
    for cmd in world:
        op, pts = cmd[0], cmd[1:]
        if op == "Z": out.append("Z"); continue
        flat = [(pts[i], pts[i+1]) for i in range(0, len(pts), 2)]
        out.append(op + " " + " ".join(f"{x} {y}" for x, y in hoof_local(flat)))
    return " ".join(out)
HOOF = [("M", 0.005, -0.034), ("L", 0.03, -0.028), ("C", 0.04, -0.045, 0.055, -0.062, 0.067, -0.071), ("L", 0.067, -0.073), ("L", -0.008, -0.073), ("C", -0.016, -0.06, -0.014, -0.046, 0.005, -0.034), ("Z",)]
hoof_d = hoof_path(HOOF)
contact = hoof_local([(0.028, -0.073)])[0]
sole_line = hoof_local([(-0.01, -0.0715), (0.065, -0.0715)])
dewclaw = hoof_path([("M", -0.006, -0.034), ("C", -0.02, -0.04, -0.026, -0.054, -0.016, -0.057), ("C", -0.01, -0.057, -0.008, -0.05, -0.008, -0.04), ("Z",)])
toe_cleft = hoof_local([(0.03, -0.03), (0.048, -0.05), (0.06, -0.07)])
M = lambda kg, cx, cy, d="uniform", ref="weiner-1977": {"kg": kg, "comX": cx, "comY": cy, "distribution": d, "source": "authored-estimate", "reference": ref}
def bone(id, parent, en, de, x, y, ang, ln, lo, hi, mass=None):
    b = {"id": id, "parent": parent, "title": T(en, de), "x": x, "y": y, "angleDeg": ang, "lengthM": ln, "joint": {"type": "revolute", "minDeg": lo, "maxDeg": hi}}
    if mass: b["mass"] = mass
    return b
R = "andersen-1998"
bones = [
    bone("pelvis", None, "Pelvis and hip joint", "Becken und Hüftgelenk", 0, 0, 0, 0.22, -30, 30, M(3.2, 0.05, -0.03, "point", R)),
    bone("lumbar", "pelvis", "Lumbar spine", "Lendenwirbelsäule", 0.22, 0, 0, 0.26, -25, 30, M(2.6, 0.13, -0.04, "uniform", R)),
    bone("thorax", "lumbar", "Thoracic spine and ribcage", "Brustwirbelsäule und Brustkorb", 0.26, 0, 0, 0.36, -15, 15, M(8.5, 0.18, -0.07, "uniform", R)),
    bone("neck", "thorax", "Neck", "Hals", 0.33, 0.05, 40, 0.34, -125, 45, M(1.7, 0.17, 0, "uniform", R)),
    bone("head", "neck", "Head", "Kopf", 0.34, 0, -52, 0.22, -45, 60, M(1.25, 0.1, 0, "point", R)),
    bone("jaw", "head", "Lower jaw", "Unterkiefer", 0.05, -0.035, -3, 0.12, -30, 2, M(0.15, 0.06, -0.01, "point")),
    bone("ear-n", "head", "Near ear", "Nahes Ohr", 0.03, 0.065, 100, 0.14, -50, 60, M(0.04, 0.07, 0, "point")),
    bone("ear-f", "head", "Far ear", "Fernes Ohr", 0.03, 0.065, 100, 0.14, -50, 60, M(0.04, 0.07, 0, "point")),
]
anchors = []
geo = []
def sh(id, bone, role, lod, z, filled, tone, kind, region=None, material=None, **kw):
    s = {"id": id, "bone": bone, "role": role, "lod": lod, "z": z, "filled": filled, "tone": tone}
    if region: s["region"] = region
    if material: s["material"] = material
    s["kind"] = kind; s.update(kw); geo.append(s)
FUR, SK = "summer-coat", "bare-skin"
def limbs(s):
    side, tone, z = ("near", 0.7, 5.0) if s == "n" else ("far", 0.55, 1.0)
    en, de = ("Near", "Nah") if s == "n" else ("Far", "Fern")
    bones.extend([
        bone(f"thigh-h{s}", "pelvis", f"{en} thigh (femur)", f"{de}er Oberschenkel", 0, 0, -65, 0.21, -50, 62, M(1.2, 0.1, -0.01)),
        bone(f"shank-h{s}", f"thigh-h{s}", f"{en} shank (tibia)", f"{de}er Unterschenkel", 0.21, 0, -60, 0.25, -85, 45, M(0.5, 0.12, 0)),
        bone(f"cannon-h{s}", f"shank-h{s}", f"{en} hind cannon (metatarsus, hock to fetlock)", f"{de}er Hintermittelfuß (Sprunggelenk bis Fesselgelenk)", 0.25, 0, 40, 0.23, -40, 115, M(0.14, 0.115, 0)),
        bone(f"hoof-h{s}", f"cannon-h{s}", f"{en} hind hoof (pastern and hoof)", f"{de}er Hinterhuf (Fessel und Huf)", 0.23, 0, 33, 0.09, -95, 40, M(0.1, 0.045, -0.005, "point")),
        bone(f"scapula-f{s}", "thorax", f"{en} shoulder blade", f"{de}es Schulterblatt", 0.2, 0.05, -55, 0.17, -35, 40, M(0.7, 0.085, 0)),
        bone(f"humerus-f{s}", f"scapula-f{s}", f"{en} upper arm (humerus)", f"{de}er Oberarm", 0.17, 0, -75, 0.16, -45, 75, M(0.5, 0.08, 0)),
        bone(f"forearm-f{s}", f"humerus-f{s}", f"{en} forearm (radius)", f"{de}er Unterarm", 0.16, 0, 42, 0.21, -30, 120, M(0.4, 0.1, 0)),
        bone(f"cannon-f{s}", f"forearm-f{s}", f"{en} fore cannon (metacarpus, knee to fetlock)", f"{de}er Vordermittelfuß (Vorderfußwurzel bis Fesselgelenk)", 0.21, 0, 0, 0.19, -135, 10, M(0.12, 0.095, 0)),
        bone(f"hoof-f{s}", f"cannon-f{s}", f"{en} fore hoof (pastern and hoof)", f"{de}er Vorderhuf (Fessel und Huf)", 0.19, 0, 36, 0.09, -95, 40, M(0.1, 0.045, -0.005, "point")),
    ])
    for h, fl, fw, ten, tde in (("h", 0.06, 0.05, "hind hoof", "Hinterhuf"), ("f", 0.065, 0.055, "fore hoof", "Vorderhuf")):
        anchors.append({"id": f"contact-h{s}" if h == "h" else f"contact-f{s}", "bone": f"hoof-{h}{s}", "x": round(contact[0], 4), "y": round(contact[1], 4), "kind": "contact", "title": T(f"{en} {ten} sole", f"Sohle des {de.lower()}en {tde}es" if s == "n" else f"Sohle des fernen {tde}es"), "support": "contact", "normalDeg": 90, "footprintLengthM": fl, "footprintWidthM": fw, "frictionCoefficient": 0.6})
    p = f"{side}-"
    sh(p + "thigh", f"thigh-h{s}", "body", 0, z, True, tone, "path", "coat-limbs", FUR, d="M -0.05 0.06 C 0.0 0.088 0.1 0.078 0.17 0.045 L 0.225 0.03 C 0.238 0.0 0.234 -0.03 0.218 -0.042 C 0.15 -0.072 0.07 -0.098 0.0 -0.1 C -0.06 -0.092 -0.088 -0.03 -0.072 0.02 C -0.066 0.04 -0.06 0.05 -0.05 0.06 Z")
    sh(p + "shank", f"shank-h{s}", "body", 0, z, True, tone, "path", "coat-limbs", FUR, d="M -0.03 0.02 C 0.0 0.05 0.08 0.045 0.14 0.03 L 0.252 0.019 C 0.262 0.01 0.266 -0.012 0.258 -0.026 C 0.252 -0.036 0.24 -0.036 0.232 -0.03 C 0.2 -0.04 0.12 -0.05 0.06 -0.06 C 0.01 -0.06 -0.03 -0.035 -0.03 0.02 Z")
    sh(p + "hind-cannon", f"cannon-h{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.23, y1=0.0, r0=0.017, r1=0.011)
    sh(p + "hind-hoof", f"hoof-h{s}", "body", 0, z, True, tone * 0.4, "path", "hooves", "hoof-horn", d=hoof_d)
    sh(p + "hind-pastern", f"hoof-h{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.05, y1=0.0, r0=0.0135, r1=0.011)
    sh(p + "fore-pastern", f"hoof-f{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.05, y1=0.0, r0=0.0135, r1=0.011)
    sh(p + "scapula", f"scapula-f{s}", "body", 0, z, True, tone, "path", "coat-limbs", FUR, d="M -0.03 0.03 C 0.0 0.056 0.08 0.058 0.13 0.052 L 0.172 0.044 L 0.172 -0.05 C 0.12 -0.075 0.05 -0.08 0.0 -0.07 C -0.04 -0.06 -0.06 0.0 -0.03 0.03 Z")
    sh(p + "humerus", f"humerus-f{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.16, y1=0.0, r0=0.058, r1=0.038)
    sh(p + "forearm", f"forearm-f{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.21, y1=0.0, r0=0.04, r1=0.021)
    sh(p + "fore-cannon", f"cannon-f{s}", "body", 0, z, True, tone, "capsule", "coat-limbs", FUR, x0=0.0, y0=0.0, x1=0.19, y1=0.0, r0=0.02, r1=0.0105)
    sh(p + "fore-hoof", f"hoof-f{s}", "body", 0, z, True, tone * 0.4, "path", "hooves", "hoof-horn", d=hoof_d)
    sh(p + "ear", f"ear-{s}", "body", 0, 4.5 if s == "n" else 2.0, True, 0.6 if s == "n" else 0.5, "path", "ears", FUR, d="M -0.012 0.022 C 0.03 0.05 0.1 0.045 0.145 0.0 C 0.1 -0.045 0.03 -0.045 -0.012 -0.022 Z")
    sh(p + "ear-inner", f"ear-{s}", "detail", 2, 4.6 if s == "n" else 2.1, True, 0.28 if s == "n" else 0.25, "path", "ears", FUR, d="M 0.015 0.012 C 0.05 0.028 0.09 0.024 0.115 0.0 C 0.09 -0.026 0.05 -0.026 0.015 -0.012 Z")
    if s == "n":
        sh("near-dewclaw-hind", "hoof-hn", "detail", 3, 5.1, True, 0.1, "path", "hooves", "hoof-horn", d=dewclaw)
        sh("near-hind-sole", "hoof-hn", "detail", 3, 5.2, False, 0.15, "poly", closed=False, points=[list(p_) for p_ in sole_line])
        sh("near-hind-cleft", "hoof-hn", "detail", 3, 5.2, False, 0.15, "poly", closed=False, points=[list(p_) for p_ in toe_cleft])
        sh("near-fore-cleft", "hoof-fn", "detail", 3, 5.2, False, 0.15, "poly", closed=False, points=[list(p_) for p_ in toe_cleft])
        sh("near-elbow-line", "humerus-fn", "detail", 3, 5.5, False, 0.25, "poly", closed=False, points=[[0.03, 0.03], [0.1, 0.018], [0.158, 0.0]])
        sh("near-hock-line", "shank-hn", "detail", 3, 5.5, False, 0.25, "poly", closed=False, points=[[0.19, 0.012], [0.232, 0.0], [0.248, -0.01]])
        sh("near-knee-line", "forearm-fn", "detail", 3, 5.5, False, 0.25, "poly", closed=False, points=[[0.18, 0.012], [0.2, 0.0], [0.21, -0.008]])
limbs("f"); limbs("n")
anchors += [
    {"id": "nose", "bone": "head", "x": 0.232, "y": 0.0, "kind": "emitter", "title": T("Nostrils", "Nasenlöcher")},
    {"id": "eye-anchor", "bone": "head", "x": 0.105, "y": 0.04, "kind": "attachment", "title": T("Eye", "Auge")},
]
B = lambda *a, **k: sh(*a, **k)
body = [
    ("rump", "pelvis", "M -0.14 0.02 C -0.145 0.075 -0.09 0.105 0.0 0.103 C 0.08 0.1 0.15 0.092 0.22 0.088 L 0.22 -0.15 C 0.14 -0.15 0.04 -0.145 -0.04 -0.125 C -0.11 -0.1 -0.14 -0.04 -0.14 0.02 Z", 3.0),
    ("loin", "lumbar", "M -0.03 0.09 C 0.06 0.09 0.18 0.082 0.26 0.078 L 0.26 -0.17 C 0.18 -0.16 0.08 -0.15 -0.03 -0.15 Z", 3.1),
    ("chest", "thorax", "M -0.03 0.078 C 0.06 0.076 0.12 0.074 0.19 0.075 C 0.26 0.074 0.32 0.065 0.37 0.04 C 0.4 0.0 0.4 -0.08 0.38 -0.13 C 0.35 -0.19 0.3 -0.225 0.22 -0.228 C 0.12 -0.23 0.04 -0.21 -0.03 -0.17 Z", 3.2),
    ("neck", "neck", "M -0.09 0.1 C 0.0 0.095 0.1 0.066 0.2 0.052 C 0.26 0.046 0.3 0.044 0.33 0.036 L 0.33 -0.042 C 0.27 -0.06 0.21 -0.076 0.15 -0.098 C 0.08 -0.125 -0.0 -0.15 -0.09 -0.15 Z", 3.3),
    ("head", "head", "M -0.03 0.0 C -0.03 0.05 0.0 0.078 0.045 0.08 C 0.09 0.08 0.12 0.062 0.15 0.046 L 0.205 0.022 C 0.225 0.016 0.234 0.004 0.228 -0.01 C 0.218 -0.024 0.2 -0.026 0.18 -0.03 L 0.1 -0.04 C 0.07 -0.058 0.03 -0.062 0.0 -0.05 C -0.022 -0.036 -0.03 -0.02 -0.03 0.0 Z", 4.0),
    ("jaw", "jaw", "M -0.03 0.012 L 0.115 0.012 C 0.132 0.01 0.14 -0.004 0.13 -0.014 L 0.05 -0.034 C 0.02 -0.042 -0.01 -0.036 -0.03 -0.01 Z", 3.9),
]
for id, bn, d, z in body:
    reg = "coat-head-neck" if id in ("neck", "head", "jaw") else "coat-trunk"
    sh(id, bn, "body", 0, z, True, 0.7, "path", reg, FUR, d=d)
sh("nose-leather", "head", "body", 0, 4.2, True, 0.06, "ellipse", "nose-skin", SK, cx=0.222, cy=0.008, rx=0.0135, ry=0.0105)
sh("eye", "head", "detail", 1, 5.0, True, 0.04, "ellipse", "eyes", SK, cx=0.105, cy=0.04, rx=0.0105, ry=0.0085)
sh("brow", "head", "detail", 2, 5.0, False, 0.2, "poly", closed=False, points=[[0.125, 0.056], [0.108, 0.054], [0.088, 0.056]])
sh("mouth-line", "head", "detail", 1, 5.0, False, 0.2, "poly", closed=False, points=[[0.225, -0.006], [0.18, -0.022], [0.1, -0.034], [0.05, -0.046]])
sh("chin-patch", "jaw", "detail", 2, 4.1, True, 0.96, "path", None, "white-coat", d="M 0.075 -0.003 C 0.1 0.004 0.125 0.004 0.133 -0.008 C 0.125 -0.02 0.1 -0.026 0.075 -0.026 Z")
sh("rump-patch", "pelvis", "detail", 1, 5.3, True, 0.97, "path", None, "white-coat", d="M -0.105 0.062 C -0.134 0.078 -0.146 0.03 -0.128 0.0 C -0.118 -0.02 -0.108 -0.03 -0.098 -0.044 C -0.088 -0.016 -0.082 0.024 -0.105 0.062 Z")
sh("tail", "pelvis", "body", 0, 2.9, True, 0.7, "path", "coat-trunk", FUR, d="M -0.118 0.0 L -0.14 -0.026 L -0.108 -0.03 Z")
sh("antler-near", "head", "detail", 1, 5.2, False, 0.35, "path", None, "antler-bone", d="M 0.075 0.07 C 0.062 0.11 0.058 0.15 0.072 0.185 C 0.082 0.2 0.095 0.21 0.105 0.23 M 0.066 0.115 L 0.11 0.14 M 0.072 0.185 L 0.05 0.21 M 0.062 0.09 L 0.03 0.12")
sh("antler-far", "head", "detail", 1, 1.5, False, 0.5, "path", None, "antler-bone", d="M 0.092 0.066 C 0.082 0.105 0.082 0.14 0.095 0.172 C 0.105 0.188 0.116 0.198 0.124 0.216 M 0.088 0.11 L 0.128 0.132 M 0.095 0.172 L 0.075 0.196")
sh("shoulder-line", "thorax", "detail", 2, 5.5, False, 0.25, "poly", closed=False, points=[[0.21, 0.07], [0.25, 0.0], [0.23, -0.1]])
sh("hip-line", "pelvis", "detail", 2, 5.5, False, 0.25, "poly", closed=False, points=[[0.14, 0.09], [0.085, 0.0], [0.07, -0.07]])
sh("flank-line", "lumbar", "detail", 3, 5.5, False, 0.25, "poly", closed=False, points=[[0.02, -0.13], [0.12, -0.12], [0.25, -0.15]])
sh("belly-light", "thorax", "detail", 2, 5.5, False, 0.85, "poly", closed=False, points=[[0.06, -0.19], [0.17, -0.215], [0.28, -0.205], [0.35, -0.16]])
sh("fur-neck", "neck", "texture", 2, 5.5, False, 0.35, "poly", closed=False, points=[[0.03, 0.06], [0.08, 0.05], [0.13, 0.044], [0.05, 0.0], [0.11, -0.01], [0.17, -0.02]])
sh("fur-trunk", "thorax", "texture", 2, 5.5, False, 0.35, "poly", closed=False, points=[[0.04, 0.05], [0.1, 0.045], [0.16, 0.05], [0.06, -0.02], [0.12, -0.03], [0.1, -0.1], [0.17, -0.11]])
sh("fur-rump", "pelvis", "texture", 2, 5.5, False, 0.35, "poly", closed=False, points=[[-0.02, 0.07], [0.04, 0.065], [0.1, 0.07], [0.0, 0.03], [0.06, 0.02]])
sh("fur-ear", "ear-n", "texture", 2, 4.7, False, 0.2, "poly", closed=False, points=[[0.03, 0.0], [0.08, 0.004], [0.12, 0.0]])
total = sum(b["mass"]["kg"] for b in bones if "mass" in b)
print("mass", round(total, 3), "bones", len(bones), "shapes", len(geo), file=sys.stderr)
open(D + "/10-rig.json", "w").write('      "rig": {\n        "kind": "anatomical",\n        "root": "pelvis",\n        "bones": [\n' + ",\n".join("          " + J(b) for b in bones) + '\n        ]\n      },\n')
open(D + "/20-anchors.json", "w").write('      "anchors": [\n' + ",\n".join("        " + J(a) for a in anchors) + '\n      ],\n')
open(D + "/30-geometry.json", "w").write('      "geometry": {\n        "elevation": [\n' + ",\n".join("          " + J(g) for g in geo) + '\n        ]\n      },\n')
