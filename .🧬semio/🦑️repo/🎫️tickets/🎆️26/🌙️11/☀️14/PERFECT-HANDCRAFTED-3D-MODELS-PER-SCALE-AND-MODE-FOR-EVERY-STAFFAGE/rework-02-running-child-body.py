"""🧒️ Running child: body and thermal solids of all four scales from hand-chosen stations (author 02 rework).

Reference: six-year-old, 1.15 m, 22 kg: head 0.185 m high (one sixth of the stature), short limbs, narrow shoulders, no waist.
Frames: trunk, neck, head and pack u up, +v back; limbs u distal, +v front; +w the child's left. The foot rows are written ahead / up
from the ankle and turned onto the oblique foot bone (34.66 degrees below the horizontal). A row is (station, centre v, centre w,
half depth, half breadth, exponent, coarsest scale that keeps the station).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
loft, capsule, ellipsoid, box, mirror, shell, cut, of, keep, tilted = lib.loft, lib.capsule, lib.ellipsoid, lib.box, lib.mirror, lib.shell, lib.cut, lib.of, lib.keep, lib.tilted

ASSET = "humans/running-child"
SUB = {50: 4, 100: 3, 200: 2, 500: 2}
FOOT = -34.66
SKIN, SHIRT, SHORTS, SOCK, SHOE, SOLE, HAIR, PACK = ("skin", 0.84), ("cotton-jersey", 0.7), ("cotton-short", 0.35), ("cotton-sock", 0.95), ("canvas-sneaker", 0.88), ("canvas-sneaker", 0.97), ("hair", 0.3), ("nylon-pack", 0.3)

SHIRT_ROWS = [
    (-0.070, 0.006, 0, 0.084, 0.112, 2.4, 500),
    (0.030, -0.004, 0, 0.076, 0.102, 2.4, 500),
    (0.110, -0.008, 0, 0.078, 0.104, 2.4, 200),
    (0.180, -0.006, 0, 0.078, 0.108, 2.5, 500),
    (0.230, 0.000, 0, 0.070, 0.108, 2.5, 200),
    (0.262, 0.004, 0, 0.054, 0.100, 2.4, 500),
    (0.282, 0.006, 0, 0.044, 0.060, 2.2, 100),
    (0.292, 0.006, 0, 0.042, 0.050, 2.1, 500),
]
NECK_ROWS = [(-0.030, 0.008, 0, 0.042, 0.046, 2, 500), (0.020, 0.004, 0, 0.040, 0.040, 2, 500), (0.085, 0.000, 0, 0.042, 0.040, 2, 500)]
HEAD_ROWS = [
    (-0.049, -0.058, 0, 0.016, 0.020, 2.1, 500),
    (-0.036, -0.050, 0, 0.030, 0.034, 2.2, 100),
    (-0.015, -0.036, 0, 0.048, 0.050, 2.3, 500),
    (0.010, -0.020, 0, 0.066, 0.062, 2.3, 100),
    (0.040, -0.006, 0, 0.082, 0.068, 2.2, 500),
    (0.075, 0.002, 0, 0.089, 0.070, 2.1, 200),
    (0.105, 0.004, 0, 0.080, 0.064, 2, 500),
    (0.123, 0.004, 0, 0.058, 0.048, 2, 500),
]
HAIR_ROWS = [
    (-0.020, 0.048, 0, 0.028, 0.048, 2, 500),
    (0.010, 0.050, 0, 0.032, 0.058, 2, 200),
    (0.036, 0.046, 0, 0.038, 0.062, 2, 500),
    (0.056, 0.012, 0, 0.076, 0.073, 2, 500),
    (0.080, 0.002, 0, 0.093, 0.074, 2, 500),
    (0.110, 0.004, 0, 0.086, 0.070, 2, 200),
    (0.128, 0.004, 0, 0.066, 0.054, 2, 500),
]
FRINGE = 0.062
SHORTS_ROWS = [
    (0.060, 0.002, 0, 0.072, 0.098, 2.4, 500),
    (0.020, 0.008, 0, 0.080, 0.105, 2.5, 500),
    (-0.020, 0.010, 0, 0.080, 0.108, 2.5, 200),
    (-0.050, 0.006, 0, 0.068, 0.098, 2.5, 500),
    (-0.068, 0.004, 0, 0.048, 0.070, 2.4, 500),
]
SHORTS_LEG_ROWS = [(-0.020, -0.002, 0.002, 0.060, 0.052, 2.2, 500), (0.040, 0.000, 0.003, 0.062, 0.054, 2.2, 500), (0.100, 0.002, 0.004, 0.062, 0.056, 2.2, 500)]
THIGH_ROWS = [(0.070, 0.002, 0.002, 0.056, 0.050, 2.1, 500), (0.130, 0.004, 0.002, 0.051, 0.046, 2.1, 200), (0.200, 0.004, 0, 0.044, 0.040, 2.1, 500), (0.2565, 0.002, 0, 0.040, 0.037, 2.1, 500)]
SHANK_ROWS = [
    (0.000, 0.002, 0, 0.039, 0.037, 2.1, 500),
    (0.035, -0.002, 0, 0.038, 0.037, 2, 200),
    (0.085, -0.008, -0.002, 0.040, 0.037, 2, 500),
    (0.150, -0.006, 0, 0.034, 0.030, 2, 100),
    (0.205, -0.002, 0, 0.027, 0.024, 2, 500),
    (0.245, 0.000, 0, 0.027, 0.025, 2, 500),
    (0.275, 0.002, 0, 0.026, 0.024, 2, 500),
]
SOCK_TOP = 0.222
SOCK_ROWS = [(SOCK_TOP, 0, 0, 0.0295, 0.0265, 2, 500), (0.250, 0, 0, 0.0295, 0.0275, 2, 500), (0.272, 0.002, 0, 0.029, 0.027, 2, 500)]
SOLE_ROWS = [(-0.050, -0.049, 0, 0.007, 0.022, 4, 500), (-0.020, -0.049, 0, 0.007, 0.029, 4, 500), (0.030, -0.049, 0.001, 0.007, 0.030, 4, 200), (0.081, -0.049, 0.002, 0.007, 0.036, 4, 500)]
UPPER_ROWS = [
    (-0.046, -0.026, 0, 0.018, 0.021, 2.5, 500),
    (-0.020, -0.020, 0, 0.024, 0.027, 2.5, 500),
    (0.010, -0.022, 0.001, 0.022, 0.029, 2.5, 200),
    (0.045, -0.030, 0.002, 0.016, 0.033, 2.5, 200),
    (0.081, -0.034, 0.002, 0.012, 0.035, 2.5, 500),
]
SOLE_TOE_ROWS = [(-0.002, 0.007, 0.002, 0.007, 0.036, 4, 500), (0.030, 0.007, 0.001, 0.007, 0.033, 4, 200), (0.046, 0.008, 0, 0.006, 0.026, 4, 500)]
UPPER_TOE_ROWS = [(-0.002, 0.022, 0.002, 0.012, 0.035, 2.5, 500), (0.025, 0.021, 0.001, 0.011, 0.032, 2.5, 200), (0.044, 0.018, 0, 0.008, 0.025, 2.5, 500)]
SHOE_ROWS = [(-0.046, -0.033, 0, 0.023, 0.022, 3, 500), (-0.015, -0.028, 0, 0.028, 0.028, 3, 500), (0.081, -0.040, 0.002, 0.016, 0.036, 3, 500)]
SHOE_ROWS_500 = [(-0.054, -0.021, 0, 0.035, 0.030, 3, 500), (-0.010, -0.021, 0, 0.035, 0.032, 3, 500), (0.081, -0.030, 0.002, 0.026, 0.036, 3, 500)]
SHOE_TOE_ROWS_500 = [(-0.002, 0.026, 0.002, 0.026, 0.036, 3, 500), (0.052, 0.022, 0, 0.022, 0.028, 3, 500)]
ARM_ROWS_500 = [(0.000, -0.001, 0, 0.030, 0.030, 2, 500), (0.040, 0.002, 0, 0.036, 0.036, 2, 500), (0.150, 0.000, 0, 0.030, 0.028, 2.3, 500), (0.205, 0.002, -0.003, 0.034, 0.028, 2.6, 500), (0.261, 0.001, -0.010, 0.026, 0.026, 2.6, 500)]
SHOE_TOE_ROWS = [(-0.002, 0.016, 0.002, 0.016, 0.036, 3, 500), (0.044, 0.014, 0, 0.013, 0.026, 3, 500)]
SLEEVE_ROWS = [(-0.026, 0.000, -0.007, 0.029, 0.027, 2, 500), (-0.004, 0.001, -0.004, 0.037, 0.035, 2, 500), (0.018, 0.002, -0.001, 0.039, 0.037, 2, 200), (0.045, 0.002, 0, 0.039, 0.037, 2, 200), (0.105, 0.002, 0, 0.039, 0.037, 2, 500)]
SLEEVE_HEM = 0.105
UPPER_ARM_ROWS = [(0.090, 0.002, 0, 0.030, 0.027, 2, 500), (0.120, 0.002, 0, 0.028, 0.026, 2, 200), (0.1848, -0.001, 0, 0.026, 0.027, 2, 500)]
FOREARM_ROWS = [(0.000, -0.001, 0, 0.026, 0.027, 2, 500), (0.035, 0.002, 0, 0.028, 0.027, 2, 500), (0.100, 0.000, 0, 0.022, 0.019, 2, 200), (0.150, 0.000, 0, 0.020, 0.015, 2.3, 500)]
PALM_ROWS = [(-0.006, 0.000, 0, 0.019, 0.014, 2.4, 500), (0.020, 0.001, -0.001, 0.025, 0.012, 2.6, 200), (0.045, 0.000, -0.002, 0.029, 0.011, 3, 500), (0.064, -0.001, -0.003, 0.029, 0.010, 3, 500)]
MITTEN_ROWS = [(0.082, 0.000, -0.008, 0.026, 0.010, 3, 500), (0.094, 0.000, -0.020, 0.023, 0.010, 3, 500)]
HAND_ROWS = [(-0.006, 0.000, 0, 0.020, 0.015, 2.4, 500), (0.045, 0.002, -0.003, 0.031, 0.015, 2.6, 500), (0.095, 0.001, -0.014, 0.022, 0.015, 2.6, 500)]
FINGERS = [(0.018, 1.0), (0.005, 1.08), (-0.008, 1.0), (-0.020, 0.82)]
PACK_ROWS = [(-0.080, 0.063, 0, 0.035, 0.085, 4, 500), (0.000, 0.070, 0, 0.042, 0.095, 4, 500), (0.100, 0.066, 0, 0.039, 0.092, 4, 200), (0.165, 0.054, 0, 0.027, 0.070, 3.5, 500)]


def shirt_front(u, w):
    """👕️ Where the shirt front lies at a station of the trunk and a lateral offset (for the straps that lie on it)."""
    rows = sorted(SHIRT_ROWS)
    for a, b in zip(rows, rows[1:]):
        if a[0] <= u <= b[0]:
            t = (u - a[0]) / (b[0] - a[0])
            cv, rv, rw, e = (a[i] + (b[i] - a[i]) * t for i in (1, 3, 4, 5))
            return cv - rv * (1 - min(0.98, abs(w) / rw) ** e) ** (1 / e)
    return rows[-1][1] - rows[-1][3]


def body(sc):
    """🧒️ The body solids of one scale."""
    sub = SUB[sc]
    L = lambda id, bone, look, rows, **kw: loft(id, bone, look[0], look[1], keep(rows, sc), sub, **kw)
    out = [L("shirt", "torso", SHIRT, SHIRT_ROWS)]
    if sc == 50:
        out.append(loft("collar", "torso", "cotton-jersey", 0.58, [(0.284, 0.006, 0, 0.046, 0.058, 2.1), (0.296, 0.006, 0, 0.043, 0.050, 2.1)], 1, role="detail"))
    out.append(L("neck", "neck", SKIN, NECK_ROWS))
    out.append(L("head", "head", SKIN, HEAD_ROWS, round0=0.004, round1=0.012))
    if sc <= 100:
        out.append(capsule("nose", "head", "skin", 0.84, (0.022, -0.084, 0), (0.004, -0.0885, 0), 0.0055, 0.008))
    if sc == 50:
        for side in (1, -1):
            tag = "l" if side > 0 else "r"
            out.append(ellipsoid(f"ear-{tag}", "head", "skin", 0.84, (0.030, 0.004, 0.075 * side), (0.022, 0.013, 0.009), role="detail", rotation=[0.1305 * side, 0, 0, 0.9914]))
            out.append(ellipsoid(f"eye-{tag}", "head", "skin", 0.1, (0.036, -0.082, 0.027 * side), (0.006, 0.005, 0.010), role="detail"))
    out.append(L("hair", "head", HAIR, HAIR_ROWS, round1=0.014))
    out.append(L("shorts", "pelvis", SHORTS, SHORTS_ROWS, round1=0.012))
    out.append(L("pack", "pack", PACK, PACK_ROWS, round0=0.012, round1=0.012))
    if sc == 50:
        out.append(box("pack-pocket", "pack", "nylon-pack", 0.4, (0.000, 0.114, 0), (0.110, 0.020, 0.130), 0.008, role="detail"))
        arch = [(0.170, 0.034, 0), (0.190, 0.040, 0), (0.190, 0.060, 0), (0.170, 0.066, 0)]
        out += [capsule(f"pack-handle-{i}", "pack", "nylon-pack", 0.22, a, b, 0.005, role="detail") for i, (a, b) in enumerate(zip(arch, arch[1:]), 1)]
    if sc == 100:
        out.append(box("pack-pocket", "pack", "nylon-pack", 0.4, (0.000, 0.114, 0), (0.110, 0.022, 0.130), 0.008, role="detail"))
    left = []
    if sc <= 100:
        strap = [(u, shirt_front(u, w) - 0.002, w, 0.004 if sc == 50 else 0.0076, 0.012, 2) for u, w in ((0.060, 0.090), (0.120, 0.082), (0.190, 0.070), (0.245, 0.062))]
        left.append(loft("strap-l", "torso", "nylon-pack", 0.22, strap, sub, role="detail"))
        left.append(capsule("strap-top-l", "torso", "nylon-pack", 0.22, (0.250, shirt_front(0.245, 0.062), 0.060), (0.262, 0.050, 0.060), 0.0046 if sc == 50 else 0.0076, role="detail", lateralScale=2.4 if sc == 50 else 1.5))
    left += [L("shorts-leg-l", "thigh-l", SHORTS, SHORTS_LEG_ROWS), L("thigh-l", "thigh-l", SKIN, THIGH_ROWS, round1=0.03)]
    if sc <= 100:
        left.append(ellipsoid("patella-l", "shank-l", "skin", 0.84, (-0.006, 0.030, 0), (0.018, 0.010, 0.016)))
    left.append(L("shank-l", "shank-l", SKIN, SHANK_ROWS, round0=0.028))
    left.append(L("sock-l", "shank-l", SOCK, SOCK_ROWS))
    if sc == 50:
        left += [tilted(L("sole-l", "foot-l", SOLE, SOLE_ROWS, round0=0.006), FOOT), tilted(L("shoe-l", "foot-l", SHOE, UPPER_ROWS, round0=0.008), FOOT), L("sole-toe-l", "toe-l", SOLE, SOLE_TOE_ROWS, round1=0.010), L("shoe-toe-l", "toe-l", SHOE, UPPER_TOE_ROWS, round1=0.010)]
    else:
        left += [tilted(L("shoe-l", "foot-l", SHOE, SHOE_ROWS_500 if sc == 500 else SHOE_ROWS, round0=0.008), FOOT), L("shoe-toe-l", "toe-l", SHOE, SHOE_TOE_ROWS_500 if sc == 500 else SHOE_TOE_ROWS, round1=0.010)]
    left += [L("sleeve-l", "arm-l", SHIRT, SLEEVE_ROWS, round0=0.012), L("upper-arm-l", "arm-l", SKIN, UPPER_ARM_ROWS, round1=0.022), L("forearm-l", "forearm-l", SKIN, ARM_ROWS_500 if sc == 500 else FOREARM_ROWS, round0=0.02, **({"round1": 0.01} if sc == 500 else {}))]
    if sc == 50:
        left.append(L("hand-l", "hand-l", SKIN, PALM_ROWS, round1=0.005))
        for i, (v0, k) in enumerate(FINGERS, 1):
            a, b, c, d = (0.064, v0, -0.003), (0.064 + 0.018 * k, v0, -0.008), (0.064 + 0.026 * k, v0, -0.008 - 0.012 * k), (0.064 + 0.023 * k, v0, -0.008 - 0.023 * k)
            left += [capsule(f"finger-{i}-l", "hand-l", "skin", 0.84, a, b, 0.0058, 0.0054), capsule(f"finger-{i}-mid-l", "hand-l", "skin", 0.84, b, c, 0.0054, 0.005), capsule(f"finger-{i}-tip-l", "hand-l", "skin", 0.84, c, d, 0.005, 0.0046)]
        left += [capsule("thumb-l", "hand-l", "skin", 0.84, (0.012, 0.016, -0.005), (0.040, 0.027, -0.011), 0.008, 0.007), capsule("thumb-tip-l", "hand-l", "skin", 0.84, (0.040, 0.027, -0.011), (0.064, 0.029, -0.021), 0.0062, 0.0054)]
    elif sc == 100:
        left += [L("hand-l", "hand-l", SKIN, PALM_ROWS + MITTEN_ROWS, round1=0.008), capsule("thumb-l", "hand-l", "skin", 0.84, (0.012, 0.016, -0.005), (0.062, 0.029, -0.020), 0.008, 0.0076)]
    elif sc == 200:
        left.append(L("hand-l", "hand-l", SKIN, HAND_ROWS, round1=0.010))
    return out + left + [mirror(s) for s in left]


def thermal(sc, solids, emitters):
    """🌡️ The thermal solids of one scale over the body of that scale; the emitter solids stay at their anchors."""
    B = {s["id"]: s for s in solids}
    fine = sc == 50
    out = []

    def worn(id, region, layer, faces, under=None):
        extra = {"sectionFaces": faces} if faces else {}
        if not fine:
            out.append(of(f"th-{id}", id, region, **extra))
            return
        out.append(of(f"th-{id}-{layer}", id, region, layer=layer, **extra))
        if under:
            out.append(shell(B[id], 0.0008, f"th-{id}-{under}", material="cotton-underwear", region=region, layer=under, **extra))

    worn("shirt", "tshirt", "tshirt", ["-u", "+u"], "briefs")
    if fine:
        worn("collar", "tshirt", "tshirt", ["-u", "+u"], "briefs")
        out += [of(f"th-{i}", i, "face-skin") for i in ("ear-l", "eye-l", "ear-r", "eye-r")]
    if sc <= 100:
        out.append(of("th-nose", "nose", "face-skin"))
    worn("hair", "hair", "hair", None)
    worn("shorts", "shorts", "shorts", ["-u", "+u"])
    out += [of(f"th-{s['id']}", s["id"], "backpack") for s in solids if s["bone"] == "pack" or s["id"].startswith("strap-")]
    for tag in ("l", "r"):
        worn(f"shorts-leg-{tag}", "shorts", "shorts", ["-u", "+u"])
        out.append(cut(B[f"thigh-{tag}"], 0.100, 0.270, f"th-thigh-{tag}", region="leg-skin", sectionFaces=["-u", "+u"]))
        if sc <= 100:
            out.append(of(f"th-patella-{tag}", f"patella-{tag}", "leg-skin"))
        out.append(cut(B[f"shank-{tag}"], 0.0, SOCK_TOP, f"th-shank-{tag}", region="leg-skin", sectionFaces=["-u", "+u"]))
        worn(f"sock-{tag}", "socks", "socks", ["-u", "+u"])
        for part, face in ((("sole", "+u"), ("shoe", "+u"), ("sole-toe", "-u"), ("shoe-toe", "-u")) if sc == 50 else (("shoe", "+u"), ("shoe-toe", "-u"))):
            worn(f"{part}-{tag}", "shoes", "sneaker", [face])
        worn(f"sleeve-{tag}", "tshirt", "tshirt", ["-u", "+u"], "briefs")
        out.append(cut(B[f"upper-arm-{tag}"], SLEEVE_HEM, 0.190, f"th-upper-arm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"]))
        out.append(cut(B[f"forearm-{tag}"], 0.0, 0.150 if sc < 500 else 0.261, f"th-forearm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"]))
        out += [of(f"th-{s['id']}", s["id"], "arm-skin") for s in solids if s["bone"] == f"hand-{tag}"]
    head, neck = B["head"], B["neck"]
    out.append(loft("th-face", "head", "skin", 0.84, [(u, cv - rv * 0.5, cw, rv * 0.5, rw, e) for u, cv, cw, rv, rw, e in keep(HEAD_ROWS, sc) if u < FRINGE], SUB[sc], round0=0.004, region="face-skin", sectionFaces=["+u"]))
    out.append(cut(neck, *((0.004, 0.050) if sc <= 100 else (-0.022, 0.056)), "th-neck", region="face-skin", sectionFaces=["-u", "+u"]))
    return out + emitters


DECISIONS = {
    50: (
        "Body 1:50: a six-year-old of 1.15 m rebuilt by hand from child anthropometry (DIN 33402-2 and WHO growth tables: head 18.5 cm high, 14 cm wide, 18 cm deep, one sixth of the stature; shoulders 27 cm, chest 21 x 15 cm with the belly carried forward and no waist, hips 21 cm; thigh 33 cm girth, knee 24 cm, calf 24 cm, ankle 16 cm; upper arm 18 cm, forearm 17 cm, hand 12 cm long). Round child skull with full cheeks, small chin, short nose, ears, eyes and a mop of hair with a fringe that covers the ear tops; T-shirt with collar and sleeves over the deltoids, shorts with two legs, knee caps, calves, socks, canvas sneakers of rubber sole and upper turned onto the oblique foot bone, palms with four curled three-segment fingers and a two-segment thumb, backpack with front pocket, grab handle and shoulder straps that lie on the shirt.",
        "Körper 1:50: ein sechsjähriges Kind von 1,15 m, von Hand aus Kinder-Anthropometrie neu aufgebaut (DIN 33402-2 und WHO-Wachstumstabellen: Kopf 18,5 cm hoch, 14 cm breit, 18 cm tief, ein Sechstel der Körperhöhe; Schultern 27 cm, Brustkorb 21 x 15 cm mit vorgewölbtem Bauch und ohne Taille, Hüfte 21 cm; Oberschenkel 33 cm Umfang, Knie 24 cm, Wade 24 cm, Knöchel 16 cm; Oberarm 18 cm, Unterarm 17 cm, Hand 12 cm lang). Runder Kinderschädel mit vollen Wangen, kleinem Kinn, kurzer Nase, Ohren, Augen und einem Haarschopf mit Pony, der die Ohrspitzen bedeckt; T-Shirt mit Kragen und Ärmeln über den Deltamuskeln, Shorts mit zwei Beinen, Kniescheiben, Waden, Socken, Stoffturnschuhe aus Gummisohle und Schaft auf dem schrägen Fussknochen, Handteller mit vier gekrümmten dreigliedrigen Fingern und zweigliedrigem Daumen, Rucksack mit Vortasche, Griffschlaufe und Schulterriemen, die auf dem Shirt liegen.",
        "Thermal 1:50: T-shirt and sleeves are the jersey over the vest layer, shorts, socks, sneakers and hair one layer each on their body solids, the backpack its solids; bare skin is cut at the hems (thigh at the shorts, upper arm at the sleeve, calf at the sock), the face is the head below the fringe, the neck the part above the collar; the breath outlets and the plume sit at their anchors.",
        "Thermik 1:50: T-Shirt und Ärmel sind der Jersey über der Unterhemdlage, Shorts, Socken, Turnschuhe und Haar je eine Lage auf ihren Körpervolumen, der Rucksack seine Volumen; nackte Haut ist an den Säumen geschnitten (Oberschenkel an den Shorts, Oberarm am Ärmel, Wade an der Socke), das Gesicht ist der Kopf unter dem Pony, der Hals der Teil über dem Kragen; Atemöffnungen und Auftriebsfahne sitzen an ihren Ankern.",
    ),
    100: (
        "Body 1:100: the same child for a 15 mm feature: the same girths through fewer stations, hands as palm with one curled finger blade and a thumb, head with nose but without ears and eyes, sneakers still of sole and upper, knee caps and straps kept (straps 15 mm thick), pocket, handle and collar left out.",
        "Körper 1:100: dasselbe Kind für 15 mm Mindestmass: dieselben Umfänge durch weniger Stationen, Hände als Handteller mit einer gekrümmten Fingerfläche und Daumen, Kopf mit Nase, aber ohne Ohren und Augen, Turnschuhe weiterhin aus Sohle und Schaft, Kniescheiben und Riemen bleiben (Riemen 15 mm dick), Vortasche, Griff und Kragen entfallen.",
        "Thermal 1:100: one surface per region on the body solids of this scale with the same cuts at the hems.",
        "Thermik 1:100: eine Oberfläche je Region auf den Körpervolumen dieses Massstabs mit denselben Schnitten an den Säumen.",
    ),
    200: (
        "Body 1:200: masses of at least 30 mm: shirt, shorts, pack, head under the hair cap, and per side shorts leg, thigh, shank, sock, one shoe with toe cap, sleeve, upper arm, forearm and a mitten hand; girths and joints as at 1:50.",
        "Körper 1:200: Massen ab 30 mm: Shirt, Shorts, Rucksack, Kopf unter der Haarkappe und je Seite Hosenbein, Oberschenkel, Unterschenkel, Socke, ein Schuh mit Zehenkappe, Ärmel, Oberarm, Unterarm und Fäustlingshand; Umfänge und Gelenke wie bei 1:50.",
        "Thermal 1:200: one surface per region with the same cuts at the hems.",
        "Thermik 1:200: eine Oberfläche je Region mit denselben Schnitten an den Säumen.",
    ),
    500: (
        "Body 1:500: the child in its fewest masses through two to four stations each; the large head, the short limbs and the pack keep it readable as a child at a glance. Forearm and hand are one mass and the shoe is 70 mm high so that they print at the end of the band.",
        "Körper 1:500: das Kind in seinen wenigsten Massen durch je zwei bis vier Stationen; der grosse Kopf, die kurzen Glieder und der Rucksack halten es auf einen Blick als Kind lesbar. Unterarm und Hand sind eine Masse und der Schuh ist 70 mm hoch, damit sie am Ende des Massstabsbands druckbar bleiben.",
        "Thermal 1:500: one surface per region with the same cuts at the hems; the breath outlets are enlarged to the printable feature.",
        "Thermik 1:500: eine Oberfläche je Region mit denselben Schnitten an den Säumen; die Atemöffnungen sind auf das druckbare Mindestmass vergrössert.",
    ),
}


def main():
    source = lib.load_source(ASSET)
    for sc in (50, 100, 200, 500):
        entry = source["volumes"][str(sc)]
        solids = body(sc)
        emitters = [s for s in entry["thermal"] if "emitter" in s.get("tags", [])]
        if sc == 500:
            for s in emitters:
                s["radii"] = [max(r, 0.0376) for r in s["radii"]]
        entry["body"] = solids
        entry["thermal"] = thermal(sc, solids, emitters)
        b_en, b_de, t_en, t_de = DECISIONS[sc]
        rest = [d for d in entry["decisions"] if not d["en"].startswith(("Body ", "Thermal "))]
        entry["decisions"] = [{"en": b_en, "de": b_de}, {"en": t_en, "de": t_de}] + rest
        print(f"[DEBUG] {ASSET} 1:{sc}: body {len(solids)}, thermal {len(entry['thermal'])} ({len(emitters)} emitters), structure {len(entry['structure'])}")
    lib.save_source(ASSET, source)


if __name__ == "__main__":
    main()
