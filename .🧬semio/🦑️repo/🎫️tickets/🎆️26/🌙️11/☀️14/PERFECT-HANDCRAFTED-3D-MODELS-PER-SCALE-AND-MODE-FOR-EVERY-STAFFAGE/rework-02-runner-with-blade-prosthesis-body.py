"""🦿️ Runner with blade prosthesis: body and thermal solids of all four scales from hand-chosen stations (author 02 rework).

Reference: athletic man, 1.80 m, about 70 kg, right transtibial amputation. Frames: trunk, neck and head u up, +v back; limbs u
distal, +v front; +w his left. The left foot rows are written ahead / up from the ankle and turned onto the oblique foot bone
(28.57 degrees below the horizontal). A row is (station, centre v, centre w, half depth, half breadth, exponent, coarsest scale that
keeps the station). The prosthesis hardware (liner, socket, adapter, pyramid, blade, tread) is authored in
`rework-02-runner-with-blade-prosthesis-hardware.py`; this script keeps those solids as stored.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(HERE, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
loft, capsule, ellipsoid, mirror, shell, cut, of, keep, tilted = lib.loft, lib.capsule, lib.ellipsoid, lib.mirror, lib.shell, lib.cut, lib.of, lib.keep, lib.tilted

ASSET = "humans/runner-with-blade-prosthesis"
SUB = {50: 4, 100: 3, 200: 2, 500: 2}
FOOT = -28.5658
HARDWARE = ("liner-", "socket", "blade-")
SKIN, TOP, SHORTS, SOCK, SHOE, SOLE, HAIR = ("skin", 0.72), ("running-singlet", 0.9), ("compression-shorts", 0.2), ("cotton-sock", 0.95), ("running-shoe", 0.82), ("running-shoe", 0.5), ("hair", 0.15)

SINGLET_ROWS = [
    (-0.060, 0.006, 0, 0.112, 0.158, 2.4, 500),
    (0.040, -0.006, 0, 0.100, 0.142, 2.4, 500),
    (0.130, -0.012, 0, 0.108, 0.150, 2.4, 200),
    (0.230, -0.020, 0, 0.124, 0.165, 2.3, 500),
    (0.310, -0.014, 0, 0.126, 0.174, 2.4, 100),
    (0.375, -0.002, 0, 0.118, 0.180, 2.5, 500),
    (0.420, 0.006, 0, 0.104, 0.176, 2.4, 200),
    (0.455, 0.010, 0, 0.082, 0.154, 2.3, 500),
    (0.476, 0.012, 0, 0.070, 0.108, 2.3, 100),
    (0.490, 0.012, 0, 0.066, 0.086, 2.2, 500),
]
NECK_ROWS = [(-0.045, 0.012, 0, 0.066, 0.078, 2, 500), (-0.015, 0.011, 0, 0.062, 0.064, 2, 500), (0.020, 0.008, 0, 0.059, 0.058, 2, 200), (0.060, 0.004, 0, 0.059, 0.057, 2, 500), (0.112, 0.002, 0, 0.062, 0.058, 2, 500)]
HEAD_ROWS = [
    (-0.084, -0.056, 0, 0.022, 0.026, 2.1, 500),
    (-0.072, -0.048, 0, 0.036, 0.040, 2.2, 100),
    (-0.055, -0.040, 0, 0.049, 0.052, 2.3, 200),
    (-0.025, -0.024, 0, 0.068, 0.063, 2.3, 500),
    (0.005, -0.010, 0, 0.082, 0.070, 2.2, 100),
    (0.040, -0.002, 0, 0.092, 0.075, 2.1, 500),
    (0.075, 0.000, 0, 0.097, 0.0775, 2.1, 200),
    (0.110, 0.002, 0, 0.090, 0.073, 2, 500),
    (0.135, 0.004, 0, 0.068, 0.056, 2, 500),
]
HAIR_ROWS = [
    (-0.005, 0.042, 0, 0.040, 0.060, 2, 500),
    (0.030, 0.032, 0, 0.066, 0.079, 2, 200),
    (0.065, 0.014, 0, 0.086, 0.080, 2, 500),
    (0.095, 0.006, 0, 0.099, 0.079, 2, 100),
    (0.125, 0.004, 0, 0.086, 0.070, 2, 500),
    (0.146, 0.005, 0, 0.060, 0.050, 2, 500),
]
HAIRLINE = 0.070
SHORTS_ROWS = [
    (0.135, 0.000, 0, 0.097, 0.143, 2.3, 500),
    (0.090, 0.004, 0, 0.105, 0.150, 2.4, 200),
    (0.040, 0.012, 0, 0.115, 0.158, 2.5, 500),
    (0.000, 0.018, 0, 0.121, 0.167, 2.5, 100),
    (-0.040, 0.018, 0, 0.118, 0.165, 2.5, 500),
    (-0.075, 0.012, 0, 0.102, 0.148, 2.6, 200),
    (-0.100, 0.006, 0, 0.074, 0.110, 2.4, 500),
]
SHORTS_LEG_ROWS = [(-0.030, -0.004, 0.004, 0.100, 0.084, 2.2, 500), (0.040, 0.000, 0.006, 0.104, 0.089, 2.2, 200), (0.125, 0.005, 0.006, 0.102, 0.091, 2.2, 500)]
SHORTS_HEM = 0.125
THIGH_ROWS = [
    (0.105, 0.004, 0.006, 0.100, 0.089, 2.2, 500),
    (0.190, 0.010, 0.006, 0.095, 0.085, 2.2, 200),
    (0.280, 0.013, 0.003, 0.082, 0.074, 2.1, 500),
    (0.360, 0.009, -0.002, 0.068, 0.062, 2.1, 100),
    (0.410, 0.004, -0.001, 0.061, 0.056, 2.1, 200),
    (0.435, 0.002, 0, 0.060, 0.054, 2.2, 500),
]
LINER_TOP = 0.345
SHANK_ROWS = [
    (0.000, 0.004, 0, 0.058, 0.054, 2.1, 500),
    (0.044, -0.002, -0.001, 0.057, 0.055, 2, 200),
    (0.122, -0.016, -0.004, 0.065, 0.061, 2, 500),
    (0.200, -0.014, -0.003, 0.058, 0.053, 2, 100),
    (0.284, -0.006, -0.001, 0.044, 0.039, 2, 500),
    (0.352, -0.002, 0, 0.036, 0.031, 2, 200),
    (0.396, 0.000, 0, 0.036, 0.032, 2, 100),
    (0.423, 0.000, 0, 0.039, 0.037, 2, 500),
    (0.448, 0.002, 0, 0.034, 0.032, 2, 500),
]
SOCK_TOP = 0.385
SOCK_ROWS = [(SOCK_TOP, 0, 0, 0.041, 0.037, 2, 500), (0.416, 0, 0, 0.043, 0.041, 2, 500), (0.446, 0.002, 0, 0.040, 0.038, 2, 500)]
SOLE_ROWS = [
    (-0.066, -0.060, 0, 0.0135, 0.031, 4, 500),
    (-0.036, -0.060, 0, 0.0135, 0.038, 4, 200),
    (0.030, -0.062, 0.002, 0.0115, 0.039, 4, 500),
    (0.090, -0.063, 0.004, 0.0105, 0.049, 4, 200),
    (0.135, -0.063, 0.004, 0.0105, 0.053, 4, 500),
]
UPPER_ROWS = [
    (-0.060, -0.030, 0, 0.024, 0.029, 2.5, 500),
    (-0.028, -0.022, 0, 0.030, 0.036, 2.5, 200),
    (0.010, -0.024, 0.001, 0.028, 0.039, 2.5, 500),
    (0.060, -0.036, 0.003, 0.020, 0.045, 2.5, 200),
    (0.135, -0.044, 0.004, 0.012, 0.051, 2.5, 500),
]
SOLE_TOE_ROWS = [(-0.004, 0.0105, 0.004, 0.0105, 0.053, 4, 500), (0.040, 0.0105, 0.003, 0.010, 0.050, 4, 200), (0.070, 0.0125, 0.001, 0.009, 0.040, 4, 500)]
UPPER_TOE_ROWS = [(-0.004, 0.031, 0.004, 0.013, 0.051, 2.5, 500), (0.035, 0.030, 0.003, 0.012, 0.048, 2.5, 200), (0.068, 0.026, 0.001, 0.010, 0.038, 2.5, 500)]
SHOE_ROWS = [(-0.060, -0.040, 0, 0.0335, 0.031, 3, 500), (-0.028, -0.036, 0, 0.0375, 0.037, 3, 200), (0.010, -0.036, 0.001, 0.0375, 0.039, 3, 500), (0.135, -0.053, 0.004, 0.0205, 0.052, 3, 500)]
SHOE_TOE_ROWS = [(-0.004, 0.0256, 0.004, 0.0256, 0.052, 3, 500), (0.070, 0.0256, 0.001, 0.0256, 0.039, 3, 500)]
ARM_ROWS = [
    (-0.014, 0.000, -0.016, 0.046, 0.042, 2, 500),
    (0.000, 0.002, -0.010, 0.057, 0.053, 2, 200),
    (0.045, 0.004, -0.004, 0.058, 0.055, 2, 500),
    (0.100, 0.005, 0.000, 0.053, 0.047, 2, 100),
    (0.175, 0.007, 0, 0.051, 0.045, 2, 500),
    (0.245, 0.004, 0, 0.044, 0.040, 2, 200),
    (0.295, 0.000, 0, 0.040, 0.039, 2, 100),
    (0.330, -0.002, 0, 0.040, 0.042, 2, 500),
]
FOREARM_ROWS = [
    (0.000, -0.002, 0, 0.040, 0.041, 2, 500),
    (0.052, 0.003, 0, 0.046, 0.043, 2, 500),
    (0.112, 0.002, 0, 0.041, 0.037, 2, 200),
    (0.186, 0.000, 0, 0.033, 0.027, 2, 500),
    (0.238, 0.000, 0, 0.029, 0.022, 2, 100),
    (0.266, 0.000, 0, 0.029, 0.021, 2.3, 500),
]
PALM_ROWS = [(-0.010, 0.000, 0, 0.028, 0.020, 2.4, 500), (0.025, 0.002, -0.001, 0.036, 0.017, 2.6, 200), (0.065, 0.000, -0.003, 0.042, 0.015, 3, 500), (0.100, -0.002, -0.005, 0.042, 0.013, 3, 500)]
MITTEN_ROWS = [(0.126, 0.000, -0.012, 0.038, 0.013, 3, 500), (0.146, 0.000, -0.030, 0.034, 0.014, 3, 500)]
HAND_ROWS = [(-0.010, 0.000, 0, 0.029, 0.020, 2.4, 500), (0.075, 0.004, -0.004, 0.046, 0.019, 2.6, 500), (0.150, 0.002, -0.022, 0.032, 0.019, 2.6, 500)]
HAND_ROWS_500 = [(-0.010, 0.000, 0, 0.030, 0.026, 2.4, 500), (0.075, 0.004, -0.004, 0.046, 0.026, 2.6, 500), (0.150, 0.002, -0.018, 0.032, 0.026, 2.6, 500)]
FINGERS = [(0.026, 0.127, 0.140, 0.136, 0.048), (0.006, 0.1285, 0.1423, 0.138, 0.050), (-0.014, 0.127, 0.140, 0.136, 0.048), (-0.031, 0.123, 0.134, 0.1306, 0.042)]


def body(sc, stored):
    """🏃️ The body solids of one scale; the prosthesis hardware is taken as stored."""
    sub = SUB[sc]
    L = lambda id, bone, look, rows, **kw: loft(id, bone, look[0], look[1], keep(rows, sc), sub, **kw)
    out = [L("singlet", "torso", TOP, SINGLET_ROWS)]
    if sc == 50:
        out.append(loft("singlet-collar", "torso", "running-singlet", 0.6, [(0.480, 0.012, 0, 0.071, 0.104, 2.2), (0.494, 0.012, 0, 0.067, 0.086, 2.2)], 1, role="detail"))
    if sc <= 200:
        out.append(L("neck", "neck", SKIN, NECK_ROWS))
        out.append(L("head", "head", SKIN, HEAD_ROWS, round0=0.006, round1=0.016))
    else:
        out.append(L("head", "head", SKIN, [(-0.150, 0.010, 0, 0.066, 0.070, 2, 500), (-0.100, 0.002, 0, 0.060, 0.058, 2, 500)] + HEAD_ROWS[2:], round1=0.016))
    if sc <= 100:
        out.append(capsule("nose", "head", "skin", 0.72, (0.036, -0.090, 0), (-0.006, -0.1065, 0), 0.008, 0.0125))
    if sc == 50:
        for side in (1, -1):
            tag = "l" if side > 0 else "r"
            out.append(ellipsoid(f"ear-{tag}", "head", "skin", 0.72, (0.018, 0.010, 0.078 * side), (0.031, 0.018, 0.009), role="detail", rotation=[0.1305 * side, 0, 0, 0.9914]))
            out.append(ellipsoid(f"eye-{tag}", "head", "skin", 0.1, (0.040, -0.087, 0.031 * side), (0.006, 0.005, 0.012), role="detail"))
    out.append(L("hair", "head", HAIR, HAIR_ROWS, round1=0.016))
    out.append(L("shorts", "pelvis", SHORTS, SHORTS_ROWS, round1=0.02))
    if sc == 50:
        out += [ellipsoid(f"seat-{tag}", "pelvis", "compression-shorts", 0.2, (-0.030, 0.070, 0.072 * side), (0.080, 0.062, 0.074)) for tag, side in (("l", 1), ("r", -1))]
    left = [L("shorts-leg-l", "thigh-l", SHORTS, SHORTS_LEG_ROWS), L("thigh-l", "thigh-l", SKIN, THIGH_ROWS, round1=0.045)]
    right = [mirror(left[0]), mirror(left[1]) if sc == 500 else mirror(cut(L("thigh-l", "thigh-l", SKIN, THIGH_ROWS), THIGH_ROWS[0][0], LINER_TOP + 0.012, "thigh-l"))]
    if sc <= 100:
        left.append(ellipsoid("patella-l", "shank-l", "skin", 0.72, (-0.012, 0.048, 0), (0.026, 0.015, 0.024)))
    left.append(L("shank-l", "shank-l", SKIN, SHANK_ROWS, round0=0.04))
    if sc <= 100:
        left.append(L("sock-l", "shank-l", SOCK, SOCK_ROWS))
    if sc == 50:
        left += [tilted(L("sole-l", "foot-l", SOLE, SOLE_ROWS, round0=0.008), FOOT), tilted(L("shoe-l", "foot-l", SHOE, UPPER_ROWS, round0=0.01), FOOT), L("sole-toe-l", "toe-l", SOLE, SOLE_TOE_ROWS, round1=0.016), L("shoe-toe-l", "toe-l", SHOE, UPPER_TOE_ROWS, round1=0.016)]
    else:
        left += [tilted(L("shoe-l", "foot-l", SHOE, SHOE_ROWS, round0=0.012), FOOT), L("shoe-toe-l", "toe-l", SHOE, SHOE_TOE_ROWS, round1=0.016)]
    arm = [L("arm-l", "arm-l", SKIN, ARM_ROWS, round0=0.018, round1=0.035), L("forearm-l", "forearm-l", SKIN, FOREARM_ROWS, round0=0.032)]
    if sc == 50:
        arm.append(L("hand-l", "hand-l", SKIN, PALM_ROWS, round1=0.008))
        for i, (v0, u1, u2, u3, w3) in enumerate(FINGERS, 1):
            knuckle, first, second, tip = (0.100, v0, -0.005), (u1, v0, -0.012), (u2, v0, -0.6 * w3), (u3, v0, -w3)
            arm += [capsule(f"finger-{i}-l", "hand-l", "skin", 0.72, knuckle, first, 0.0095, 0.0088), capsule(f"finger-{i}-mid-l", "hand-l", "skin", 0.72, first, second, 0.0088, 0.008), capsule(f"fingertip-{i}-l", "hand-l", "skin", 0.72, second, tip, 0.008, 0.0075)]
        arm += [capsule("thumb-l", "hand-l", "skin", 0.72, (0.020, 0.024, -0.008), (0.062, 0.040, -0.018), 0.013, 0.011), capsule("thumbtip-l", "hand-l", "skin", 0.72, (0.062, 0.040, -0.018), (0.104, 0.042, -0.034), 0.010, 0.0085)]
    elif sc == 100:
        arm += [L("hand-l", "hand-l", SKIN, PALM_ROWS + MITTEN_ROWS, round1=0.008), capsule("thumb-l", "hand-l", "skin", 0.72, (0.020, 0.024, -0.008), (0.100, 0.042, -0.030), 0.012, 0.009)]
    else:
        arm.append(L("hand-l", "hand-l", SKIN, HAND_ROWS_500 if sc == 500 else HAND_ROWS, round1=0.012))
    hardware = [s for s in stored if s["id"].startswith(HARDWARE)]
    return out + left + right + hardware + arm + [mirror(s) for s in arm]


def thermal(sc, solids, stored):
    """🌡️ The thermal solids of one scale over the body of that scale; hardware regions and emitters are taken as stored."""
    B = {s["id"]: s for s in solids}
    fine = sc == 50
    out = []

    def worn(id, region, layer):
        out.append(of(f"th-{id}", id, region, **({"layer": layer} if fine else {})))

    worn("singlet", "singlet", "singlet")
    if fine:
        worn("singlet-collar", "singlet", "singlet")
        out += [of(f"th-{i}", i, "face-skin") for i in ("ear-l", "eye-l", "ear-r", "eye-r")]
    if sc <= 100:
        out.append(of("th-nose", "nose", "face-skin"))
    worn("hair", "hair", "hair")
    worn("shorts", "shorts", "shorts")
    if fine:
        worn("seat-l", "shorts", "shorts")
        worn("seat-r", "shorts", "shorts")
    worn("shorts-leg-l", "shorts", "shorts")
    worn("shorts-leg-r", "shorts", "shorts")
    out.append(cut(B["thigh-l"], SHORTS_HEM, 0.455, "th-thigh-l", region="leg-skin", sectionFaces=["-u", "+u"]))
    out.append(cut(B["thigh-r"], SHORTS_HEM, LINER_TOP, "th-thigh-r", region="leg-skin", sectionFaces=["-u", "+u"]))
    if sc <= 100:
        out.append(of("th-patella-l", "patella-l", "leg-skin"))
        out.append(cut(B["shank-l"], 0.0, SOCK_TOP, "th-shank-l", region="leg-skin", sectionFaces=["-u", "+u"]))
        worn("sock-l", "running-shoe", "socks")
    else:
        out.append(cut(B["shank-l"], 0.0, 0.440, "th-shank-l", region="leg-skin", sectionFaces=["-u", "+u"]))
    for part in (("sole-l", "shoe-l", "sole-toe-l", "shoe-toe-l") if fine else ("shoe-l", "shoe-toe-l")):
        worn(part, "running-shoe", "shoe")
        if fine and part.startswith("shoe"):
            out.append(shell(B[part], 0.006, f"th-{part}-socks", material="cotton-sock", region="running-shoe", layer="socks"))
    out += [s for s in stored if s["id"].startswith(tuple("th-" + h for h in HARDWARE))]
    for tag in ("l", "r"):
        out.append(cut(B[f"arm-{tag}"], -0.014, 0.340, f"th-arm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"]))
        out.append(cut(B[f"forearm-{tag}"], 0.0, 0.266, f"th-forearm-{tag}", region="arm-skin", sectionFaces=["-u", "+u"]))
        out += [of(f"th-{s['id']}", s["id"], "hand-skin") for s in solids if s["bone"] == f"hand-{tag}"]
    rows = [(u, cv - rv * 0.5, cw, rv * 0.5, rw, e) for u, cv, cw, rv, rw, e in keep(HEAD_ROWS, sc) if u < HAIRLINE + 0.02]
    out.append(loft("th-face", "head", "skin", 0.72, rows, SUB[sc], round0=0.006, region="face-skin", sectionFaces=["+u"]))
    if sc <= 200:
        out.append(cut(B["neck"], -0.012, 0.066, "th-neck", region="face-skin", sectionFaces=["-u", "+u"]))
    else:
        out.append(cut(B["head"], -0.150, -0.072, "th-neck", region="face-skin", sectionFaces=["-u", "+u"]))
    return out + [s for s in stored if "emitter" in s.get("tags", [])]


DECISIONS = {
    50: (
        "Body 1:50: an athletic 1.80 m sprinter rebuilt by hand from anthropometric stations (DIN 33402-2, Drillis and Contini, ANSUR II girths): tight singlet over a 33 x 25 cm chest and a 28 x 20 cm waist rising over the trapezius to the collar, bare deltoids of 46 cm shoulder breadth running into 30 cm upper arms and 28 cm forearms with flat upright wrists, hands with four curled three-segment fingers and two-segment thumbs, compression shorts with the gluteal masses and two tight legs, thighs of 59 cm girth tapering over the vastus medialis to the knee, the left calf of 39 cm set back over a straight shin, ankle sock, running shoe of midsole and upper turned onto the oblique foot bone. The right thigh ends in the silicone liner; liner, carbon socket, adapter, pyramid and the J-shaped carbon blade with its tread are the prosthesis hardware of this scale. Head with jaw, chin, nose, ears, eyes and short hair.",
        "Körper 1:50: ein athletischer, 1,80 m grosser Sprinter, von Hand aus anthropometrischen Stationen neu aufgebaut (DIN 33402-2, Drillis und Contini, Umfänge nach ANSUR II): enges Trägershirt über einem Brustkorb von 33 x 25 cm und einer Taille von 28 x 20 cm, das über den Trapezius zum Kragen ansteigt, nackte Deltamuskeln bei 46 cm Schulterbreite, die in 30 cm starke Oberarme und 28 cm starke Unterarme mit flachen, aufrechten Handgelenken übergehen, Hände mit vier gekrümmten dreigliedrigen Fingern und zweigliedrigen Daumen, Kompressionsshorts mit den Gesässmassen und zwei engen Beinen, Oberschenkel mit 59 cm Umfang, über den Vastus medialis zum Knie verjüngt, die linke Wade mit 39 cm nach hinten gesetzt über geradem Schienbein, Knöchelsocke, Laufschuh aus Zwischensohle und Schaft auf dem schrägen Fussknochen. Der rechte Oberschenkel endet im Silikonliner; Liner, Karbonschaft, Adapter, Pyramide und die J-förmige Karbonfeder mit ihrer Lauffläche sind die Prothesenteile dieses Massstabs. Kopf mit Kiefer, Kinn, Nase, Ohren, Augen und kurzem Haar.",
        "Thermal 1:50: singlet, shorts and hair one fabric layer each on their body solids, the shoe its upper over a sock layer, bare skin cut at the hems (thighs at the shorts hem, the right thigh also at the liner, the calf at the sock, the arms whole from the deltoid), the face the front of the head below the hairline, the neck above the collar; liner, socket with its inner liner and the blade are the stored hardware solids, the breath outlets and the plume sit at their anchors.",
        "Thermik 1:50: Trägershirt, Shorts und Haar je eine Stofflage auf ihren Körpervolumen, der Schuh sein Schaft über einer Sockenlage, nackte Haut an den Säumen geschnitten (Oberschenkel am Shortssaum, der rechte zusätzlich am Liner, die Wade an der Socke, die Arme ganz ab dem Deltamuskel), das Gesicht die Vorderseite des Kopfes unter dem Haaransatz, der Hals über dem Kragen; Liner, Schaft mit Innenliner und Feder sind die gespeicherten Prothesenvolumen, Atemöffnungen und Auftriebsfahne sitzen an ihren Ankern.",
    ),
    100: (
        "Body 1:100: the same runner for a 15 mm feature: the same girths through fewer stations, hands as palm with one curled finger blade and a thumb, head with nose but without ears and eyes, one shoe mass with toe cap, knee cap and sock kept, collar and gluteal masses merged into their neighbours; liner, socket and blade as the hardware of this scale.",
        "Körper 1:100: derselbe Läufer für 15 mm Mindestmass: dieselben Umfänge durch weniger Stationen, Hände als Handteller mit einer gekrümmten Fingerfläche und Daumen, Kopf mit Nase, aber ohne Ohren und Augen, eine Schuhmasse mit Zehenkappe, Kniescheibe und Socke bleiben, Kragen und Gesässmassen gehen in ihren Nachbarn auf; Liner, Schaft und Feder als Prothesenteile dieses Massstabs.",
        "Thermal 1:100: one surface per region on the body solids of this scale with the same cuts at the hems.",
        "Thermik 1:100: eine Oberfläche je Region auf den Körpervolumen dieses Massstabs mit denselben Schnitten an den Säumen.",
    ),
    200: (
        "Body 1:200: masses of at least 30 mm: singlet, shorts with legs, thighs, the left shank with one shoe and toe cap, whole arms from the deltoid, forearms, mitten hands, head with jaw under the hair cap; the right thigh ends in the liner above socket and blade.",
        "Körper 1:200: Massen ab 30 mm: Trägershirt, Shorts mit Beinen, Oberschenkel, der linke Unterschenkel mit einem Schuh und Zehenkappe, ganze Arme ab dem Deltamuskel, Unterarme, Fäustlingshände, Kopf mit Kiefer unter der Haarkappe; der rechte Oberschenkel endet im Liner über Schaft und Feder.",
        "Thermal 1:200: one surface per region with the same cuts at the hems.",
        "Thermik 1:200: eine Oberfläche je Region mit denselben Schnitten an den Säumen.",
    ),
    500: (
        "Body 1:500: the runner in his fewest masses through two to four stations each, head and neck one mass under the hair dome; the missing right shank with the socket and the forward-curling blade stays the mark that identifies him. The hand is 52 mm thick so that it prints at the end of the band.",
        "Körper 1:500: der Läufer in seinen wenigsten Massen durch je zwei bis vier Stationen, Kopf und Hals eine Masse unter der Haarkuppel; der fehlende rechte Unterschenkel mit Schaft und nach vorn geschwungener Feder bleibt das Merkmal, an dem man ihn erkennt. Die Hand ist 52 mm dick, damit sie am Ende des Massstabsbands druckbar bleibt.",
        "Thermal 1:500: one surface per region with the same cuts at the hems; the neck is the lower part of the head mass.",
        "Thermik 1:500: eine Oberfläche je Region mit denselben Schnitten an den Säumen; der Hals ist der untere Teil der Kopfmasse.",
    ),
}


def main():
    source = lib.load_source(ASSET)
    for sc in (50, 100, 200, 500):
        entry = source["volumes"][str(sc)]
        solids = body(sc, entry["body"])
        entry["thermal"] = thermal(sc, solids, entry["thermal"])
        entry["body"] = solids
        b_en, b_de, t_en, t_de = DECISIONS[sc]
        rest = [d for d in entry["decisions"] if not d["en"].startswith(("Body ", "Thermal "))]
        entry["decisions"] = [{"en": b_en, "de": b_de}, {"en": t_en, "de": t_de}] + rest
        print(f"[DEBUG] {ASSET} 1:{sc}: body {len(solids)}, thermal {len(entry['thermal'])}, structure {len(entry['structure'])}")
    lib.save_source(ASSET, source)


if __name__ == "__main__":
    main()
