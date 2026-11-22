"""🏃️ Jogger: body and thermal solids of all four scales from hand-chosen anatomical stations (author 02 rework).

Reference: lean male runner, 1.795 m barefoot (1.82 m in shoes), 72 kg. Frames: trunk, neck and head u up, +v back; limbs u distal,
+v front; foot and toe u ahead, v up; +w the figure's left. A row is (u, centre v, centre w, half depth v, half breadth w, exponent,
coarsest scale that keeps the station). The structure lists are kept as stored and edited by `rework-02-jogger-structure.py`.
"""
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
loft, capsule, ellipsoid, box, both, mirror, shell, cut, of = lib.loft, lib.capsule, lib.ellipsoid, lib.box, lib.both, lib.mirror, lib.shell, lib.cut, lib.of

ASSET = "humans/jogger"
SUB = {50: 4, 100: 3, 200: 2, 500: 2}
SKIN, SHIRT, SHORTS, SOCK, SHOE, SOLE, HAIR = ("skin", 0.72), ("nylon-clothing", 0.88), ("nylon-clothing", 0.22), ("cotton-clothing", 0.95), ("leather-shoe", 0.92), ("leather-shoe", 0.55), ("hair", 0.15)

SHIRT_ROWS = [
    (-0.050, 0.006, 0, 0.122, 0.168, 2.4, 500),
    (-0.020, 0.004, 0, 0.117, 0.161, 2.4, 100),
    (0.040, -0.006, 0, 0.106, 0.148, 2.4, 500),
    (0.130, -0.012, 0, 0.114, 0.154, 2.4, 200),
    (0.230, -0.020, 0, 0.130, 0.170, 2.3, 500),
    (0.310, -0.014, 0, 0.130, 0.180, 2.4, 100),
    (0.380, -0.002, 0, 0.122, 0.186, 2.5, 500),
    (0.430, 0.006, 0, 0.106, 0.180, 2.4, 200),
    (0.466, 0.010, 0, 0.082, 0.158, 2.3, 500),
    (0.486, 0.012, 0, 0.070, 0.108, 2.3, 100),
    (0.498, 0.012, 0, 0.066, 0.086, 2.2, 500),
]
SHORTS_ROWS = [
    (0.135, 0.000, 0, 0.100, 0.146, 2.3, 500),
    (0.090, 0.004, 0, 0.108, 0.153, 2.4, 200),
    (0.040, 0.012, 0, 0.118, 0.161, 2.5, 500),
    (0.000, 0.018, 0, 0.124, 0.170, 2.5, 100),
    (-0.040, 0.018, 0, 0.120, 0.168, 2.5, 500),
    (-0.075, 0.012, 0, 0.104, 0.150, 2.6, 200),
    (-0.100, 0.006, 0, 0.074, 0.110, 2.4, 500),
]
SHORTS_LEG_ROWS = [
    (-0.030, -0.004, 0.004, 0.100, 0.084, 2.2, 500),
    (0.040, 0.000, 0.006, 0.108, 0.090, 2.2, 200),
    (0.110, 0.004, 0.007, 0.107, 0.093, 2.2, 500),
    (0.175, 0.006, 0.008, 0.105, 0.095, 2.2, 500),
]
THIGH_ROWS = [
    (0.120, 0.004, 0.006, 0.100, 0.089, 2.2, 500),
    (0.200, 0.010, 0.006, 0.095, 0.085, 2.2, 200),
    (0.290, 0.013, 0.003, 0.082, 0.074, 2.1, 500),
    (0.370, 0.009, -0.002, 0.068, 0.062, 2.1, 100),
    (0.420, 0.004, -0.001, 0.061, 0.056, 2.1, 200),
    (0.4447, 0.002, 0, 0.060, 0.054, 2.2, 500),
]
SHANK_ROWS = [
    (0.000, 0.004, 0, 0.058, 0.054, 2.1, 500),
    (0.045, -0.002, -0.001, 0.057, 0.055, 2, 200),
    (0.125, -0.016, -0.004, 0.065, 0.061, 2, 500),
    (0.205, -0.014, -0.003, 0.058, 0.053, 2, 100),
    (0.290, -0.006, -0.001, 0.044, 0.039, 2, 500),
    (0.360, -0.002, 0, 0.036, 0.031, 2, 200),
    (0.405, 0.000, 0, 0.036, 0.032, 2, 100),
    (0.432, 0.000, 0, 0.039, 0.037, 2, 500),
    (0.458, 0.002, 0, 0.034, 0.032, 2, 500),
]
SOCK_ROWS = [(0.392, 0, 0, 0.041, 0.037, 2, 500), (0.425, 0, 0, 0.043, 0.041, 2, 500), (0.455, 0.002, 0, 0.040, 0.038, 2, 500)]
SOLE_ROWS = [
    (-0.070, -0.067, 0, 0.016, 0.031, 4, 500),
    (-0.040, -0.067, 0, 0.016, 0.038, 4, 200),
    (0.030, -0.069, 0.002, 0.014, 0.039, 4, 500),
    (0.090, -0.071, 0.004, 0.012, 0.049, 4, 200),
    (0.138, -0.072, 0.004, 0.011, 0.053, 4, 500),
]
UPPER_ROWS = [
    (-0.064, -0.034, 0, 0.026, 0.029, 2.5, 500),
    (-0.030, -0.026, 0, 0.032, 0.036, 2.5, 200),
    (0.010, -0.028, 0.001, 0.030, 0.039, 2.5, 500),
    (0.060, -0.040, 0.003, 0.022, 0.045, 2.5, 200),
    (0.138, -0.050, 0.004, 0.014, 0.051, 2.5, 500),
]
SOLE_TOE_ROWS = [(-0.004, 0.011, 0.004, 0.011, 0.053, 4, 500), (0.040, 0.011, 0.003, 0.010, 0.050, 4, 200), (0.074, 0.013, 0.001, 0.009, 0.040, 4, 500)]
UPPER_TOE_ROWS = [(-0.004, 0.033, 0.004, 0.014, 0.051, 2.5, 500), (0.035, 0.031, 0.003, 0.013, 0.048, 2.5, 200), (0.070, 0.027, 0.001, 0.010, 0.038, 2.5, 500)]
SHOE_ROWS = [(-0.064, -0.044, 0, 0.038, 0.031, 3, 500), (-0.030, -0.040, 0, 0.044, 0.037, 3, 200), (0.010, -0.040, 0.001, 0.043, 0.039, 3, 500), (0.138, -0.060, 0.004, 0.023, 0.052, 3, 500)]
SHOE_TOE_ROWS = [(-0.004, 0.0256, 0.004, 0.0256, 0.052, 3, 500), (0.074, 0.0256, 0.001, 0.0256, 0.039, 3, 500)]
SHOULDER_ROWS = [
    (-0.014, 0.000, -0.016, 0.046, 0.042, 2, 500),
    (0.000, 0.002, -0.010, 0.058, 0.054, 2, 200),
    (0.045, 0.004, -0.004, 0.059, 0.056, 2, 500),
    (0.085, 0.004, 0.000, 0.057, 0.054, 2, 500),
]
SLEEVE_ROWS = [(0.070, 0.004, 0, 0.058, 0.055, 2, 500), (0.115, 0.005, 0, 0.057, 0.053, 2, 500), (0.155, 0.006, 0, 0.058, 0.054, 2, 500)]
UPPER_ARM_ROWS = [
    (0.130, 0.005, 0, 0.052, 0.046, 2, 500),
    (0.180, 0.007, 0, 0.051, 0.045, 2, 200),
    (0.250, 0.004, 0, 0.044, 0.040, 2, 500),
    (0.300, 0.000, 0, 0.040, 0.039, 2, 100),
    (0.336, -0.002, 0, 0.040, 0.042, 2, 500),
]
FOREARM_ROWS = [
    (0.000, -0.002, 0, 0.040, 0.041, 2, 500),
    (0.055, 0.003, 0, 0.046, 0.043, 2, 500),
    (0.120, 0.002, 0, 0.041, 0.037, 2, 200),
    (0.200, 0.000, 0, 0.033, 0.027, 2, 500),
    (0.255, 0.000, 0, 0.029, 0.022, 2, 100),
    (0.285, 0.000, 0, 0.029, 0.021, 2.3, 500),
]
PALM_ROWS = [(-0.010, 0.000, 0, 0.028, 0.020, 2.4, 500), (0.025, 0.002, -0.001, 0.036, 0.017, 2.6, 200), (0.065, 0.000, -0.003, 0.042, 0.015, 3, 500), (0.100, -0.002, -0.005, 0.042, 0.013, 3, 500)]
MITTEN_ROWS = [(0.126, 0.000, -0.012, 0.038, 0.013, 3, 500), (0.146, 0.000, -0.030, 0.034, 0.014, 3, 500)]
HAND_ROWS = [(-0.010, 0.000, 0, 0.029, 0.020, 2.4, 500), (0.075, 0.004, -0.004, 0.046, 0.019, 2.6, 500), (0.150, 0.002, -0.022, 0.032, 0.019, 2.6, 500)]
HAND_ROWS_500 = [(-0.010, 0.000, 0, 0.030, 0.026, 2.4, 500), (0.075, 0.004, -0.004, 0.046, 0.026, 2.6, 500), (0.150, 0.002, -0.018, 0.032, 0.026, 2.6, 500)]
FINGERS = [(0.026, 0.127, 0.140, 0.136, 0.048), (0.006, 0.1285, 0.1423, 0.138, 0.050), (-0.014, 0.127, 0.140, 0.136, 0.048), (-0.031, 0.123, 0.134, 0.1306, 0.042)]
NECK_ROWS = [(-0.045, 0.012, 0, 0.066, 0.078, 2, 500), (-0.015, 0.011, 0, 0.062, 0.064, 2, 500), (0.020, 0.008, 0, 0.059, 0.058, 2, 200), (0.060, 0.004, 0, 0.059, 0.057, 2, 500), (0.105, 0.002, 0, 0.062, 0.058, 2, 500)]
HEAD_ROWS = [
    (-0.084, -0.076, 0, 0.022, 0.026, 2.1, 500),
    (-0.072, -0.068, 0, 0.036, 0.040, 2.2, 100),
    (-0.055, -0.060, 0, 0.049, 0.052, 2.3, 200),
    (-0.025, -0.044, 0, 0.068, 0.063, 2.3, 500),
    (0.005, -0.030, 0, 0.082, 0.070, 2.2, 100),
    (0.040, -0.022, 0, 0.092, 0.075, 2.1, 500),
    (0.075, -0.020, 0, 0.097, 0.0775, 2.1, 200),
    (0.110, -0.018, 0, 0.090, 0.073, 2, 500),
    (0.135, -0.016, 0, 0.068, 0.056, 2, 500),
]
HAIR_ROWS = [
    (-0.005, 0.022, 0, 0.040, 0.060, 2, 500),
    (0.030, 0.012, 0, 0.066, 0.079, 2, 200),
    (0.065, -0.006, 0, 0.086, 0.080, 2, 500),
    (0.095, -0.012, 0, 0.101, 0.079, 2, 100),
    (0.125, -0.016, 0, 0.086, 0.070, 2, 500),
    (0.146, -0.015, 0, 0.060, 0.050, 2, 500),
]
BAND = (0.080, 0.110)
BAND_ROWS = [(BAND[0], -0.012, 0, 0.107, 0.0835, 2, 500), (BAND[1], -0.012, 0, 0.104, 0.081, 2, 500)]
SOCK_TOP = 0.392
SLEEVE_SEAM = 0.075


def keep(rows, sc):
    """🪜️ The stations of a part that the scale keeps."""
    return [r[:6] for r in rows if r[6] >= sc]


def body(sc):
    """🧍️ The body solids of one scale."""
    sub = SUB[sc]
    L = lambda id, bone, look, rows, **kw: loft(id, bone, look[0], look[1], keep(rows, sc), sub, **kw)
    out = [L("shirt", "torso", SHIRT, SHIRT_ROWS)]
    if sc == 50:
        out.append(loft("shirt-collar", "torso", "nylon-clothing", 0.6, [(0.488, 0.012, 0, 0.071, 0.104, 2.2), (0.502, 0.012, 0, 0.067, 0.086, 2.2)], 1, role="detail"))
    out.append(L("neck", "neck", SKIN, NECK_ROWS))
    out.append(L("head", "head", SKIN, HEAD_ROWS, round0=0.006, round1=0.016))
    if sc <= 100:
        out.append(capsule("nose", "head", "skin", 0.72, (0.036, -0.110, 0), (-0.006, -0.1265, 0), 0.008, 0.0125))
    if sc == 50:
        for side in (1, -1):
            tag = "l" if side > 0 else "r"
            out.append(ellipsoid(f"ear-{tag}", "head", "skin", 0.72, (0.018, -0.010, 0.077 * side), (0.031, 0.018, 0.009), role="detail", rotation=[0.1305 * side, 0, 0, 0.9914]))
            out.append(ellipsoid(f"eye-{tag}", "head", "skin", 0.1, (0.040, -0.107, 0.031 * side), (0.006, 0.005, 0.012), role="detail"))
    out.append(L("hair", "head", HAIR, HAIR_ROWS, round1=0.016))
    if sc <= 200:
        out.append(loft("headband", "head", "nylon-clothing", 0.9, keep(BAND_ROWS, sc), 1, role="detail"))
    out.append(L("shorts", "pelvis", SHORTS, SHORTS_ROWS, round1=0.02))
    left = [L("shorts-leg-l", "thigh-l", SHORTS, SHORTS_LEG_ROWS), L("thigh-l", "thigh-l", SKIN, THIGH_ROWS, round1=0.045)]
    if sc <= 100:
        left.append(ellipsoid("patella-l", "shank-l", "skin", 0.72, (-0.012, 0.048, 0), (0.026, 0.015, 0.024)))
    left.append(L("shank-l", "shank-l", SKIN, SHANK_ROWS, round0=0.04))
    if sc <= 100:
        left.append(L("sock-l", "shank-l", SOCK, SOCK_ROWS))
        left += [L("sole-l", "foot-l", SOLE, SOLE_ROWS, round0=0.008), L("shoe-l", "foot-l", SHOE, UPPER_ROWS, round0=0.01), L("sole-toe-l", "toe-l", SOLE, SOLE_TOE_ROWS, round1=0.016), L("shoe-toe-l", "toe-l", SHOE, UPPER_TOE_ROWS, round1=0.016)]
    else:
        left += [L("shoe-l", "foot-l", SHOE, SHOE_ROWS, round0=0.012), L("shoe-toe-l", "toe-l", SHOE, SHOE_TOE_ROWS, round1=0.016)]
    if sc <= 100:
        left += [L("shoulder-l", "upper-arm-l", SHIRT, SHOULDER_ROWS, round0=0.018), L("sleeve-l", "upper-arm-l", SHIRT, SLEEVE_ROWS)]
    else:
        left.append(L("sleeve-l", "upper-arm-l", SHIRT, SHOULDER_ROWS[:3] + SLEEVE_ROWS[1:], round0=0.018))
    left += [L("upper-arm-l", "upper-arm-l", SKIN, UPPER_ARM_ROWS, round1=0.035), L("forearm-l", "forearm-l", SKIN, FOREARM_ROWS, round0=0.032)]
    if sc == 50:
        left.append(L("hand-l", "hand-l", SKIN, PALM_ROWS, round1=0.008))
        for i, (v0, u1, u2, u3, w3) in enumerate(FINGERS, 1):
            knuckle, first, second, tip = (0.100, v0, -0.005), (u1, v0, -0.012), (u2, v0, -0.6 * w3), (u3, v0, -w3)
            left += [capsule(f"finger-{i}-l", "hand-l", "skin", 0.72, knuckle, first, 0.0095, 0.0088), capsule(f"finger-{i}-mid-l", "hand-l", "skin", 0.72, first, second, 0.0088, 0.008), capsule(f"finger-{i}-tip-l", "hand-l", "skin", 0.72, second, tip, 0.008, 0.0075)]
        left += [capsule("thumb-l", "hand-l", "skin", 0.72, (0.020, 0.024, -0.008), (0.062, 0.040, -0.018), 0.013, 0.011), capsule("thumb-tip-l", "hand-l", "skin", 0.72, (0.062, 0.040, -0.018), (0.104, 0.042, -0.034), 0.010, 0.0085)]
    elif sc == 100:
        left += [L("hand-l", "hand-l", SKIN, PALM_ROWS + MITTEN_ROWS, round1=0.008), capsule("thumb-l", "hand-l", "skin", 0.72, (0.020, 0.024, -0.008), (0.100, 0.042, -0.030), 0.012, 0.009)]
    else:
        left.append(L("hand-l", "hand-l", SKIN, HAND_ROWS_500 if sc == 500 else HAND_ROWS, round1=0.012))
    out += left + [mirror(s) for s in left]
    if sc <= 100:
        out.append(loft("armband", "upper-arm-l", "nylon-clothing", 0.25, [(0.195, 0.006, 0, 0.055, 0.049, 2), (0.255, 0.004, 0, 0.048, 0.044, 2)], 1))
    if sc == 50:
        out.append(box("armband-pouch", "upper-arm-l", "nylon-clothing", 0.25, (0.225, 0.004, 0.050), (0.130, 0.070, 0.014), 0.005))
    return out


def thermal(sc, solids):
    """🌡️ The thermal solids of one scale over the body of that scale."""
    B = {s["id"]: s for s in solids}
    fine = sc == 50
    out = []

    def clothing(id, region, layer, air, gauge, inner=None):
        if not fine:
            out.append(of(f"th-{id}", id, region))
            return
        out.append(of(f"th-{id}-{layer}", id, region, layer=layer))
        out.append(shell(B[id], gauge, f"th-{id}-{air}", material="still-air", region=region, layer=air))

    clothing("shirt", "torso-clothing", "shirt", "torso-air", 0.0008)
    if fine:
        clothing("shirt-collar", "torso-clothing", "shirt", "torso-air", 0.0008)
        out += [of(f"th-{i}", i, "head-skin") for i in ("ear-l", "eye-l", "ear-r", "eye-r")]
    if sc <= 100:
        out.append(of("th-nose", "nose", "head-skin"))
    hair = B["hair"]
    layer = {"layer": "hair-pack"} if sc in (50, 500) else {}
    out.append(cut(hair, hair["sections"][0]["at"][0], BAND[0], "th-hair-nape", region="hair", sectionFaces=["+u"], **layer))
    out.append(cut(hair, BAND[1], hair["sections"][-1]["at"][0], "th-hair-crown", region="hair", sectionFaces=["-u"], **layer))
    if sc <= 200:
        out.append(of("th-headband-hair-pack" if fine else "th-headband", "headband", "headband", sectionFaces=["-u", "+u"], **({"layer": "nylon-band"} if fine else {})))
    else:
        out.append(loft("th-headband", "head", "nylon-clothing", 0.9, keep(BAND_ROWS, sc), 1, role="detail", region="headband", sectionFaces=["-u", "+u"]))
    clothing("shorts", "leg-clothing", "shorts", "shorts-air", 0.0007)
    for tag in ("l", "r"):
        clothing(f"shorts-leg-{tag}", "leg-clothing", "shorts", "shorts-air", 0.0007)
        out.append(cut(B[f"thigh-{tag}"], 0.175, 0.465, f"th-thigh-{tag}", region="leg-skin", sectionFaces=["-u", "+u"]))
        if sc <= 100:
            out.append(of(f"th-patella-{tag}", f"patella-{tag}", "leg-skin"))
        shank = B[f"shank-{tag}"]
        out.append(cut(shank, 0.0, SOCK_TOP, f"th-shank-{tag}", region="leg-skin", sectionFaces=["-u", "+u"]))
        if sc <= 100:
            out.append(of(f"th-sock-{tag}", f"sock-{tag}", "ankle-socks", layer="sock-layer", sectionFaces=["-u", "+u"]))
        else:
            out.append(shell(cut(shank, SOCK_TOP, 0.455, "x"), -0.003, f"th-sock-{tag}", material="cotton-clothing", tone=0.95, region="ankle-socks", sectionFaces=["-u", "+u"]))
        for part in (("sole", "shoe", "sole-toe", "shoe-toe") if sc <= 100 else ("shoe", "shoe-toe")):
            id = f"{part}-{tag}"
            if not fine:
                out.append(of(f"th-{id}", id, "shoes"))
                continue
            flat = part.startswith("sole")
            out.append(of(f"th-{id}-shoe-upper", id, "shoes", layer="shoe-upper"))
            out.append(shell2(B[id], 0.008, 0.0045 if flat else 0.008, f"th-{id}-shoe-air", material="still-air", region="shoes", layer="shoe-air"))
            out.append(shell2(B[id], 0.016, 0.0055 if flat else 0.0105, f"th-{id}-sock-layer", region="shoes", layer="sock-layer"))
        if sc <= 100:
            clothing(f"shoulder-{tag}", "torso-clothing", "shirt", "torso-air", 0.0008)
            clothing(f"sleeve-{tag}", "arm-clothing", "sleeve", "sleeve-air", 0.0008)
        else:
            sleeve = B[f"sleeve-{tag}"]
            out.append(cut(sleeve, sleeve["sections"][0]["at"][0], SLEEVE_SEAM, f"th-shoulder-{tag}", region="torso-clothing"))
            out.append(cut(sleeve, SLEEVE_SEAM, sleeve["sections"][-1]["at"][0], f"th-sleeve-{tag}", region="arm-clothing"))
        out += [cut(B[f"upper-arm-{tag}"], 0.155, 0.345, f"th-upper-arm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"]), cut(B[f"forearm-{tag}"], 0.0, 0.285, f"th-forearm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"])]
        out += [of(f"th-{s['id']}", s["id"], "hand-skin") for s in solids if s["bone"] == f"hand-{tag}"]
    if sc <= 100:
        clothing("armband", "arm-clothing", "sleeve", "sleeve-air", 0.0008)
    if fine:
        clothing("armband-pouch", "arm-clothing", "sleeve", "sleeve-air", 0.0008)
    head, neck = B["head"], B["neck"]
    out.append(cut(head, head["sections"][0]["at"][0], BAND[0], "th-face", region="head-skin", sectionFaces=["+u"]))
    out.append(cut(neck, -0.012, 0.060 if sc <= 100 else 0.066, "th-neck", region="head-skin", sectionFaces=["-u", "+u"]))
    r = {50: 0.00385, 100: 0.0077, 200: 0.0154, 500: 0.0385}[sc]
    wide = sc >= 200
    out.append(ellipsoid("th-nose-breath", "head", "skin", 0.3, (-0.014, -0.128 + min(r, 0.03) * (0 if fine else 0.6), 0), (r, r, r if wide else 0.008), region="head-skin", tags=["emitter", "nose-breath"]))
    out.append(ellipsoid("th-mouth-breath", "head", "skin", 0.3, (-0.043, -0.116 + min(r, 0.03) * (0 if fine else 0.6), 0), (r, r, r if wide else 0.0105), region="head-skin", tags=["emitter", "mouth-breath"]))
    out.append(ellipsoid("th-plume", "head", "hair", 0.3, (0.1583 - r * 0.6, -0.016, 0), (r, 0.05, 0.042), region="torso-clothing", tags=["emitter", "plume"]))
    return out


def shell2(solid, inset_w, inset_v, id, **extra):
    """🧅️ A shell with its own inset across the breadth (w) and the depth (v): a flat sole has less depth to give."""
    s = shell(solid, 0, id, **extra)
    for sec in s["sections"]:
        sec["rx"], sec["ry"] = lib.r5(max(sec["rx"] - inset_w, sec["rx"] * 0.35)), lib.r5(max(sec["ry"] - inset_v, sec["ry"] * 0.35))
    return s


DECISIONS = {
    50: (
        "Body 1:50: a lean 1.82 m runner rebuilt by hand from anthropometric stations (DIN 33402-2, Drillis and Contini, ANSUR II girths): thigh 56 cm girth below the shorts tapering over the vastus medialis to a 36 cm knee, calf 37 cm with the gastrocnemius set back over a straight shin, 21 cm ankle; upper arm 30 cm, forearm 28 cm turning into the flat upright wrist; chest 33 x 25 cm under a loose shirt that rises over the trapezius to the collar, deltoid caps that carry the shirt into the sleeves, 46 cm across the shoulders, hips 34 cm with the gluteal mass. Head 23 cm high, 15.5 cm wide, 19.5 cm deep with jaw, chin, nose, ears and eyes, a short hair cap and the sweatband; hands with a palm, four curled two-segment fingers and a two-segment thumb; running shoes of midsole and upper with heel counter, instep, toe spring; armband with phone pouch on the left arm. Every limb end is rounded into its neighbour so no joint opens in any pose.",
        "Körper 1:50: ein schlanker, 1,82 m grosser Läufer, von Hand aus anthropometrischen Stationen neu aufgebaut (DIN 33402-2, Drillis und Contini, Umfänge nach ANSUR II): Oberschenkel 56 cm Umfang unter den Shorts, über den Vastus medialis auf 36 cm am Knie verjüngt, Wade 37 cm mit nach hinten gesetztem Gastrocnemius über geradem Schienbein, Knöchel 21 cm; Oberarm 30 cm, Unterarm 28 cm, der in das flache, aufrechte Handgelenk übergeht; Brustkorb 33 x 25 cm unter einem weiten Shirt, das über den Trapezius zum Kragen ansteigt, Deltakappen, die das Shirt in die Ärmel führen, 46 cm Schulterbreite, Hüfte 34 cm mit Gesässmasse. Kopf 23 cm hoch, 15,5 cm breit, 19,5 cm tief mit Kiefer, Kinn, Nase, Ohren und Augen, kurzer Haarkappe und Schweissband; Hände mit Handteller, vier gekrümmten zweigliedrigen Fingern und zweigliedrigem Daumen; Laufschuhe aus Zwischensohle und Schaft mit Fersenkappe, Rist und Zehensprengung; Armband mit Handytasche am linken Arm. Jedes Gliedende ist in das Nachbarglied gerundet, sodass in keiner Pose ein Gelenk aufklafft.",
        "Thermal 1:50: every clothing region is its fabric over an enclosed air shell (shirt and sleeves 0.8 mm, shorts 0.7 mm, shoes with upper, air and sock layer), the bare skin regions are the body solids themselves, the calf is cut at the sock top, the head skin is the face below the sweatband and the neck above the collar, the hair is cut by the band into nape and crown, and the breath outlets and the plume are small solids at their anchors.",
        "Thermik 1:50: jede Kleidungsregion ist ihr Stoff über einer eingeschlossenen Luftschale (Shirt und Ärmel 0,8 mm, Shorts 0,7 mm, Schuhe mit Schaft, Luft und Sockenlage), die nackten Hautregionen sind die Körpervolumen selbst, die Wade ist an der Sockenkante geschnitten, die Kopfhaut ist das Gesicht unter dem Schweissband und der Hals über dem Kragen, das Haar wird vom Band in Nacken und Scheitel geteilt, Atemöffnungen und Auftriebsfahne sind kleine Volumen an ihren Ankern.",
    ),
    100: (
        "Body 1:100: the same runner for a 15 mm feature: the same girths through fewer stations, the hand a palm with the fingers as one curled blade and a separate thumb, the head with nose and sweatband but without ears and eyes, shoes still of midsole and upper, knee caps and the armband kept, the phone pouch and the collar left out.",
        "Körper 1:100: derselbe Läufer für 15 mm Mindestmass: dieselben Umfänge durch weniger Stationen, die Hand ein Handteller mit den Fingern als eine gekrümmte Fläche und eigenem Daumen, der Kopf mit Nase und Schweissband, aber ohne Ohren und Augen, Schuhe weiterhin aus Zwischensohle und Schaft, Kniescheiben und Armband bleiben, Handytasche und Kragen entfallen.",
        "Thermal 1:100: one surface per region on the body solids of this scale; the calf is cut at the sock top, the hair by the sweatband, the face and neck are the uncovered parts of head and neck.",
        "Thermik 1:100: eine Oberfläche je Region auf den Körpervolumen dieses Massstabs; die Wade ist an der Sockenkante geschnitten, das Haar am Schweissband, Gesicht und Hals sind die unbedeckten Teile von Kopf und Hals.",
    ),
    200: (
        "Body 1:200: masses of at least 30 mm: one shirt, one sleeve per arm from the deltoid to the hem, one shoe per foot with its toe cap, the hand a single mitten, the head a skull with jaw under the hair cap, no knee cap, nose, band or armband; limb girths and joint positions as at 1:50.",
        "Körper 1:200: Massen ab 30 mm: ein Shirt, ein Ärmel je Arm vom Deltamuskel bis zum Saum, ein Schuh je Fuss mit Zehenkappe, die Hand ein einzelner Fäustling, der Kopf ein Schädel mit Kiefer unter der Haarkappe, keine Kniescheibe, Nase, Band oder Armband; Gliederumfänge und Gelenklagen wie bei 1:50.",
        "Thermal 1:200: one surface per region; sleeve and shoulder are the two cuts of the sleeve mass, the sock is the ankle band of the shank, the sweatband a ring of its own on the hair.",
        "Thermik 1:200: eine Oberfläche je Region; Ärmel und Schulter sind die zwei Schnitte der Ärmelmasse, die Socke ist das Knöchelband des Unterschenkels, das Schweissband ein eigener Ring auf dem Haar.",
    ),
    500: (
        "Body 1:500: the figure in its fewest masses, each through two to four stations: head with neck under a hair dome, shirt, shorts, and per side shorts leg, thigh, shank, shoe, toe cap, sleeve, upper arm, forearm and mitten hand. The real girths are kept so that the runner reads as the same person at every scale.",
        "Körper 1:500: die Figur in ihren wenigsten Massen, jede durch zwei bis vier Stationen: Kopf mit Hals unter einer Haarkuppel, Shirt, Shorts und je Seite Hosenbein, Oberschenkel, Unterschenkel, Schuh, Zehenkappe, Ärmel, Oberarm, Unterarm und Fäustlingshand. Die wirklichen Umfänge bleiben, damit der Läufer in jedem Massstab als dieselbe Person lesbar ist.",
        "Thermal 1:500: one surface per region with the same cuts as at 1:200; the breath outlets are enlarged to the printable feature and sit against the face.",
        "Thermik 1:500: eine Oberfläche je Region mit denselben Schnitten wie bei 1:200; die Atemöffnungen sind auf das druckbare Mindestmass vergrössert und liegen am Gesicht an.",
    ),
}


def main():
    source = lib.load_source(ASSET)
    for sc in (50, 100, 200, 500):
        entry = source["volumes"][str(sc)]
        solids = body(sc)
        entry["body"] = solids
        entry["thermal"] = thermal(sc, solids)
        b_en, b_de, t_en, t_de = DECISIONS[sc]
        rest = [d for d in entry["decisions"] if not d["en"].startswith(("Body ", "Thermal "))]
        entry["decisions"] = [{"en": b_en, "de": b_de}, {"en": t_en, "de": t_de}] + rest
        print(f"[DEBUG] {ASSET} 1:{sc}: body {len(solids)}, thermal {len(entry['thermal'])}, structure {len(entry['structure'])}")
    lib.save_source(ASSET, source)


if __name__ == "__main__":
    main()
