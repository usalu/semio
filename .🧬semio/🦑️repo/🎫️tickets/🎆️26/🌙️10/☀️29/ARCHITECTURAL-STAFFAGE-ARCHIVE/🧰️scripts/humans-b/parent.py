import sys
from body import *
from gait import *

H, M, WF = 1.75, 68.0, 0.98
h = Human(H, M, WF)
s = h.s
A_BODY = dubois_m2(H, M)
A_STR, T_WALK = 1.0, 0.9
R_REAR, R_FRONT = A_STR / (2 * math.pi), A_STR / (4 * math.pi)
HANDLE_H = 1.0
GRIP_AHEAD = 0.50
fr = Frame("fwd", 1.0, 1.0)

# ---------------------------------------------------------------- rig: parent + stroller sub-assembly attached at the right hand
rv = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
free = {"type": "revolute"}
def lit(kg, comX, comY=0.0, src="authored-estimate", ref="en-1888"): return {"kg": kg, "comX": comX, "comY": comY, "distribution": "point", "source": src, "reference": ref}
stroller_bones = [
    h.bone("stroller", "hand-r", "Stroller frame with push bar", "Kinderwagenrahmen mit Schiebebügel", 0.075 * s, 0, 0, 0.85, rv(-60, 60), lit(8.0, 0.45, -0.55)),
    h.bone("stroller-hood", "stroller", "Folding hood", "Faltverdeck", 0.18, -0.12, 0, 0.46, rv(-5, 100), lit(0.6, 0.22, 0.08)),
    h.bone("wheel-rear", "stroller", "Rear wheel pair", "Hinteres Radpaar", 0.28, -(HANDLE_H - R_REAR), 0, R_REAR, free, lit(1.8, 0, 0)),
    h.bone("wheel-front", "stroller", "Front wheel pair", "Vorderes Radpaar", 0.80, -(HANDLE_H - R_FRONT), 0, R_FRONT, free, lit(1.0, 0, 0)),
    h.bone("child", "stroller", "Seated toddler", "Sitzendes Kleinkind", 0.32, -0.46, 0, 0.45, {"type": "fixed"}, lit(10.0, 0.18, 0.0, "literature", "winter-2009")),
]
bones = h.bones() + stroller_bones
anchors = h.anchors() + [
    {"id": "wheel-rear", "bone": "stroller", "x": 0.28, "y": -HANDLE_H, "kind": "contact", "title": T("Rear wheel contact patch", "Aufstandsfläche hinteres Rad"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.06, "frictionCoefficient": 0.02},
    {"id": "wheel-front", "bone": "stroller", "x": 0.80, "y": -HANDLE_H, "kind": "contact", "title": T("Front wheel contact patch", "Aufstandsfläche vorderes Rad"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.04, "frictionCoefficient": 0.02},
    {"id": "child-mouth", "bone": "child", "x": 0.16, "y": -0.095, "kind": "emitter", "title": T("Toddler's mouth", "Mund des Kleinkinds")},
    {"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")},
]
rig = Rig(bones, anchors)
up, down, fwd = h.up, h.down, h.fwd

# ---------------------------------------------------------------- geometry
elev = h.standard(arm_sleeve="long", legs="long", shoe="sneaker", tones={"shirt": 0.40, "trousers": 0.28, "shoes": 0.8, "skin": 0.84}, hair="tied", hair_tone=0.22)
byid = {x["id"]: x for x in elev}
byid["torso"] = h.torso_shape(tone=0.40, v0=-0.14)
elev = [x for x in elev if x["id"] != "torso"] + [byid["torso"]]
elev += [
    h.sh("jacket-hem", "torso", "detail", 2, 3.3, 0.3, g_poly(up, [(0.105, -0.14), (-0.104, -0.14)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("jacket-zip", "torso", "detail", 2, 3.3, 0.2, g_poly(up, [(0.112, -0.10), (0.126, 0.20), (0.100, 0.43)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("jacket-collar", "torso", "body", 1, 3.5, 0.34, g_path(up, [("M", 0.07, 0.40), ("C", 0.09, 0.46, 0.06, 0.50, 0.0, 0.50), ("C", -0.06, 0.50, -0.10, 0.46, -0.10, 0.40), ("Z",)], True), "torso-clothing", "cotton-clothing"),
    h.sh("scarf", "head", "body", 1, 4.8, 0.62, g_path(up, [("M", 0.06, 0.0), ("C", 0.07, 0.04, 0.06, 0.07, 0.03, 0.08), ("L", -0.05, 0.075), ("C", -0.08, 0.05, -0.07, 0.01, -0.06, 0.0), ("Z",)], True), "torso-clothing", "cotton-clothing"),
    h.sh("jeans-seam", "thigh-r", "detail", 3, 2.1, 0.2, g_poly(down, [(-0.104, 0.0), (-0.075, 0.28)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("sneaker-stripe-r", "foot-r", "texture", 3, 2.3, 0.3, g_poly(fwd, [(-0.01, -0.07), (0.07, 0.02)], False), "shoes", "leather-shoe", False),
]
for sd in "lr":
    byid2 = {x["id"]: x for x in elev}
    byid2[f"hand-{sd}"].update({"region": "hand-skin"})

# stroller shapes, frame coordinates = world profile (origin at the grip, y up)
SW = "stroller-frame"
elev += [
    h.sh("handle-grip", "stroller", "body", 0, 6.0, 0.25, g_capsule(fr, -0.05, 0.0, 0.05, 0.0, 0.017, 0.017), SW, "rubber"),
    h.sh("push-bar", "stroller", "body", 0, 4.4, 0.8, g_capsule(fr, 0.03, -0.01, 0.22, -0.58, 0.012, 0.012), SW, "aluminium"),
    h.sh("chassis-rail", "stroller", "body", 0, 4.4, 0.8, g_capsule(fr, 0.22, -0.58, 0.79, -0.62, 0.013, 0.013), SW, "aluminium"),
    h.sh("rear-strut", "stroller", "body", 0, 4.3, 0.8, g_capsule(fr, 0.24, -0.58, 0.28, -HANDLE_H + R_REAR, 0.011, 0.011), SW, "aluminium"),
    h.sh("front-fork", "stroller", "body", 0, 4.3, 0.8, g_capsule(fr, 0.77, -0.62, 0.80, -HANDLE_H + R_FRONT, 0.010, 0.010), SW, "aluminium"),
    h.sh("footrest", "stroller", "body", 1, 4.4, 0.8, g_capsule(fr, 0.60, -0.62, 0.74, -0.67, 0.008, 0.008), SW, "aluminium"),
    h.sh("seat-fabric", "stroller", "body", 0, 4.2, 0.55, g_path(fr, [("M", 0.19, -0.58), ("L", 0.17, -0.14), ("L", 0.23, -0.12), ("C", 0.25, -0.30, 0.27, -0.44, 0.30, -0.49), ("L", 0.64, -0.50), ("L", 0.66, -0.56), ("L", 0.30, -0.58), ("Z",)], True), "stroller-fabric", "stroller-textile"),
    h.sh("seat-seam", "stroller", "texture", 2, 4.25, 0.3, g_poly(fr, [(0.21, -0.20), (0.24, -0.45), (0.62, -0.51)], False), "stroller-fabric", "stroller-textile", False),
    h.sh("basket", "stroller", "body", 1, 4.1, 0.45, g_path(fr, [("M", 0.30, -0.70), ("L", 0.68, -0.70), ("L", 0.64, -0.80), ("L", 0.34, -0.80), ("Z",)], True), "stroller-fabric", "stroller-textile"),
    h.sh("basket-mesh", "stroller", "texture", 3, 4.15, 0.3, g_poly(fr, [(0.36, -0.72), (0.38, -0.78), (0.44, -0.72), (0.46, -0.78), (0.52, -0.72), (0.54, -0.78), (0.60, -0.72), (0.62, -0.78)], False), "stroller-fabric", "stroller-textile", False),
    # child
    h.sh("child-body", "child", "body", 0, 4.6, 0.62, g_capsule(fr, 0.0, 0.02, -0.02, 0.26, 0.075, 0.065), "child-clothing", "cotton-clothing"),
    h.sh("child-legs", "child", "body", 0, 4.7, 0.5, g_path(fr, [("M", 0.0, 0.0), ("L", 0.30, 0.02), ("C", 0.34, 0.02, 0.35, -0.04, 0.32, -0.06), ("L", 0.30, -0.12), ("L", 0.26, -0.11), ("L", 0.28, -0.045), ("L", 0.0, -0.075), ("Z",)], True), "child-clothing", "cotton-clothing"),
    h.sh("child-head", "child", "body", 0, 4.8, 0.88, g_ellipse(fr, -0.01, 0.34, 0.075, 0.08), "child-skin", "skin"),
    h.sh("child-hat", "child", "body", 1, 4.9, 0.3, g_path(fr, [("M", -0.085, 0.34), ("C", -0.085, 0.43, 0.06, 0.44, 0.075, 0.355), ("L", 0.085, 0.345), ("C", 0.08, 0.40, 0.05, 0.42, -0.01, 0.42), ("C", -0.07, 0.42, -0.09, 0.39, -0.085, 0.34), ("Z",)], True), "child-clothing", "cotton-clothing"),
    h.sh("child-eye", "child", "detail", 3, 4.95, 0.1, g_ellipse(fr, 0.035, 0.345, 0.006, 0.006), "child-skin", "skin"),
    h.sh("child-arm", "child", "body", 1, 4.85, 0.6, g_capsule(fr, 0.0, 0.2, 0.17, 0.08, 0.022, 0.018), "child-clothing", "cotton-clothing"),
    # hood
    h.sh("hood", "stroller-hood", "body", 0, 5.2, 0.4, g_path(fr, [("M", 0, 0), ("C", 0.02, 0.15, 0.22, 0.24, 0.38, 0.12), ("C", 0.44, 0.07, 0.47, 0.0, 0.46, -0.06), ("L", 0.40, -0.06), ("C", 0.38, 0.0, 0.30, 0.06, 0.20, 0.07), ("C", 0.10, 0.07, 0.03, 0.04, 0, 0), ("Z",)], True), "stroller-fabric", "stroller-textile"),
    h.sh("hood-rib", "stroller-hood", "detail", 2, 5.25, 0.8, g_poly(fr, [(0.01, 0.03), (0.12, 0.17), (0.28, 0.18), (0.40, 0.05)], False), "stroller-fabric", "stroller-textile", False),
    # wheels
    h.sh("tyre-rear", "wheel-rear", "body", 0, 4.0, 0.12, g_ellipse(fr, 0, 0, R_REAR, R_REAR), "tyres", "rubber"),
    h.sh("rim-rear", "wheel-rear", "detail", 1, 4.05, 0.75, g_ellipse(fr, 0, 0, R_REAR * 0.72, R_REAR * 0.72), "tyres", "rubber", False),
    h.sh("spokes-rear", "wheel-rear", "texture", 2, 4.06, 0.5, g_poly(fr, [(R_REAR * 0.72, 0), (-R_REAR * 0.72, 0), (0, 0), (0, R_REAR * 0.72), (0, -R_REAR * 0.72), (0, 0), (R_REAR * 0.51, R_REAR * 0.51), (-R_REAR * 0.51, -R_REAR * 0.51), (0, 0), (-R_REAR * 0.51, R_REAR * 0.51), (R_REAR * 0.51, -R_REAR * 0.51)], False), "tyres", "rubber", False),
    h.sh("tyre-front", "wheel-front", "body", 0, 4.0, 0.12, g_ellipse(fr, 0, 0, R_FRONT, R_FRONT), "tyres", "rubber"),
    h.sh("rim-front", "wheel-front", "detail", 1, 4.05, 0.75, g_ellipse(fr, 0, 0, R_FRONT * 0.66, R_FRONT * 0.66), "tyres", "rubber", False),
    h.sh("spokes-front", "wheel-front", "texture", 2, 4.06, 0.5, g_poly(fr, [(R_FRONT * 0.66, 0), (-R_FRONT * 0.66, 0), (0, 0), (0, R_FRONT * 0.66), (0, -R_FRONT * 0.66)], False), "tyres", "rubber", False),
]

# ---------------------------------------------------------------- poses
TORSO_LEAN = -8
def upper(extra=None):
    return merge_offsets({"torso": {"angleDeg": TORSO_LEAN}, "head": {"angleDeg": -TORSO_LEAN}}, extra or {})

def with_stroller(off, root, grip, contacts_ground=True, stroller_deg=0.0, wheels=(0.0, 0.0), hood=0.0):
    w = rig.solve(off, root)
    phi = -15.0
    arms = {}
    for _ in range(3):
        wr = (grip[0] - 0.075 * s * math.cos(math.radians(phi)), grip[1] - 0.075 * s * math.sin(math.radians(phi)))
        for sd in "lr":
            arms.update(arm_offsets(rig, w, sd, wr, phi, bend=-1))
    out = merge_offsets(off, arms)
    w2 = rig.solve(out, root)
    hand_world = rig.angle(w2, "hand-r")
    out["stroller"] = {"angleDeg": r(stroller_deg - hand_world, 2)}
    out["stroller-hood"] = {"angleDeg": hood}
    out["wheel-rear"] = {"angleDeg": r(wheels[0], 2)}
    out["wheel-front"] = {"angleDeg": r(wheels[1], 2)}
    return out

L_LEG = h.L["thigh"] + h.L["shank"] + 0.08 * s
GK = gait_keys(h, "walk", A_STR, 0.6, 0.985 * L_LEG, 0.03, 32, 0.06)
GP = {}
for i, k in enumerate(GK):
    phi = k["phi"]
    base = merge_offsets(leg_only(k), upper())
    GP[i] = with_stroller(base, (A_STR * phi, k["hipY"], 0), (A_STR * phi + GRIP_AHEAD, HANDLE_H), wheels=(-360 * phi, -720 * phi))
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
WHEELS = ["wheel-rear", "wheel-front"]

def standing(lean=0, hips=0.0):
    return merge_offsets({"torso": {"angleDeg": lean}, "head": {"angleDeg": -lean}})

stand_y = root_y(rig, {}, FEET)
pausing = with_stroller(upper({"head": {"angleDeg": -4}}), (0, stand_y, 0), (GRIP_AHEAD + 0.02, HANDLE_H))
def y_for(off, contacts): return root_y(rig, off, contacts)
# check child: stoop forward over the stroller, right hand stays on the bar, left hand tucks the blanket; hood folded back
check_base = merge_offsets(legs(14, -20, 8, 10, -16, 6), {"torso": {"angleDeg": -32}, "head": {"angleDeg": 38}, "pelvis": {"angleDeg": 0}})
yc = root_y(rig, check_base, FEET)
check = with_stroller(check_base, (0, yc, 0), (GRIP_AHEAD + 0.02, HANDLE_H), hood=95)
wcheck = rig.solve(check, (0, yc, 0))
child_pt = rig.anchor(wcheck, "child-mouth") if "child-mouth" in rig.anchors else None
wr_l = (rig.origin(wcheck, "child")[0] + 0.05, rig.origin(wcheck, "child")[1] + 0.12)
check = merge_offsets(check, arm_offsets(rig, wcheck, "l", wr_l, -40, bend=-1))
# kerb tilt: push the bar down, front wheels lifted by the kerb height
TILT = 14.0
kerb_base = upper({"torso": {"angleDeg": -4}})
gy = math.cos(math.radians(TILT)) * HANDLE_H - math.sin(math.radians(TILT)) * 0.28
yk = root_y(rig, kerb_base, FEET)
kerb = with_stroller(kerb_base, (0, yk, 0), (GRIP_AHEAD, gy), stroller_deg=TILT)

def pz(id, en, de, den, dde, off, contacts, met, br, root=None, key=False, child=0.6):
    return pose(id, en, de, den, dde, off, contacts, met, {"breath": br, "child-breath": child}, root=root, key=key)

NAMES = GAIT_NAMES
KEYPOSES = []
for i, nm in enumerate(NAMES):
    ct = WALK_CONTACTS[i] + WHEELS
    KEYPOSES.append(pz(f"push-{nm[0]}", f"Pushing: {nm[1]}", f"Schieben: {nm[2]}", nm[3], nm[4], GP[i], ct, 2.6, 0.7, root={"y": r(GK[i]["hipY"], 4)}, key=True))
poses = [
    pz("pushing-stroller", "Pushing the stroller", "Kinderwagen schieben", "Upright with a slight forward lean, both hands on the push bar, midstance of a steady push-walk; the stroller rolls on its four wheels.", "Aufrecht mit leichter Vorlage, beide Hände am Schiebebügel, mittlere Standphase beim gleichmäßigen Schieben; der Kinderwagen rollt auf vier Rädern.", GP[2], WALK_CONTACTS[2] + WHEELS, 2.6, 0.7, root={"y": r(GK[2]["hipY"], 4)}),
    pz("pausing-with-stroller", "Pausing with the stroller", "Mit dem Kinderwagen anhalten", "Standing still with both hands on the push bar and a glance at the child.", "Stehen mit beiden Händen am Schiebebügel und einem Blick zum Kind.", pausing, FEET + WHEELS, 1.5, 0.5),
    pz("checking-child", "Checking the child", "Nach dem Kind sehen", "Stooping over the stroller with the hood folded back, one hand on the bar and one on the blanket.", "Über den Kinderwagen gebeugt, Verdeck zurückgeklappt, eine Hand am Bügel, eine an der Decke.", check, FEET + WHEELS, 2.2, 0.6, child=0.8),
    pz("kerb-tilt", "Tilting up at the kerb", "Am Bordstein ankippen", "The bar is pushed down so the front wheels lift by 12 cm above the kerb; only the rear wheels touch the ground.", "Der Bügel wird heruntergedrückt, sodass sich die Vorderräder 12 cm über den Bordstein heben; nur die Hinterräder berühren den Boden.", kerb, FEET + ["wheel-rear"], 3.0, 0.8),
] + KEYPOSES

# ---------------------------------------------------------------- thermal
ADVANCE = float(sys.argv[1]) if len(sys.argv) > 1 else 0.99986792
mats = std_materials() + [
    material("rubber", "Rubber and plastic", "Gummi und Kunststoff", 0.92, "authored-estimate", None, 0.9, 1100, 0.2),
    material("aluminium", "Anodised aluminium tube", "Eloxiertes Aluminiumrohr", 0.2, "literature", "en-1991-1-1", 0.4, 2700, 160),
    material("stroller-textile", "Polyester stroller fabric", "Polyester-Kinderwagenstoff", 0.92, "authored-estimate", None, 0.5, 1380, 0.2),
]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.05, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair", "Haar", "hair", "hair", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.10, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("torso-clothing", "Jacket, scarf and shirt", "Jacke, Schal und Hemd", "clothing", "cotton-clothing", 0.38 * A_BODY, 0.30, "core-resistance", 36.8, 0.19, 0.39, "literature", "iso-9920-2007"),
    reg("arm-clothing", "Jacket sleeves", "Jackenärmel", "clothing", "cotton-clothing", 0.14 * A_BODY, 0.08, "core-resistance", 36.8, 0.17, 0.14, "literature", "iso-9920-2007"),
    reg("arm-skin", "Bare skin", "Nackte Haut", "skin", "skin", 0.001 * A_BODY, 0.05, "core-resistance", 36.8, 0.05, 0.0, "literature", "dubois-1916"),
    reg("hand-skin", "Hands", "Hände", "skin", "skin", 0.05 * A_BODY, 0.07, "core-resistance", 36.8, 0.05, 0.03, "literature", "dubois-1916", wet=0.06),
    reg("leg-clothing", "Jeans", "Jeans", "clothing", "cotton-clothing", 0.31 * A_BODY, 0.13, "core-resistance", 36.8, 0.17, 0.38, "literature", "iso-9920-2007"),
    reg("shoes", "Sneakers", "Turnschuhe", "surface", "leather-shoe", 0.07 * A_BODY, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("child-skin", "Toddler's face", "Gesicht des Kleinkinds", "skin", "skin", 0.03, 0.10, "core-resistance", 37.0, 0.06, None, "authored-estimate", "dubois-1916"),
    reg("child-clothing", "Toddler's snowsuit and hat", "Schneeanzug und Mütze des Kleinkinds", "clothing", "cotton-clothing", 0.42, 0.12, "core-resistance", 37.0, 0.20, None, "authored-estimate", "iso-9920-2007"),
    reg("stroller-fabric", "Seat, hood and basket fabric", "Sitz-, Verdeck- und Korbstoff", "surface", "stroller-textile", 1.1, 0.30, "passive", None, None, None, "authored-estimate", "en-1888"),
    reg("stroller-frame", "Frame tubes and push bar", "Rahmenrohre und Schiebebügel", "surface", "aluminium", 0.35, 0.03, "passive", None, None, None, "authored-estimate", "en-1888"),
    reg("tyres", "Wheels and tyres", "Räder und Reifen", "tyre", "rubber", 0.45, 0.12, "passive", None, None, None, "authored-estimate", "en-1888"),
]
# arm-skin is not drawn in this clothing set; drop the unbound region
regions = [x for x in regions if x["id"] != "arm-skin"]
emitters = [emitter("breath", "mouth", 3.6, vel=1.6, diam=0.03), emitter("child-breath", "child-mouth", 2.0, vel=0.7, diam=0.015, temp=33, dirdeg=-90)]
emitters[1]["title"] = T("Toddler's breath", "Atem des Kleinkinds")

# thermal pose binding for the child emitter: bind 'child-breath' name consistent
clips = [
    clip("push-walk", "Pushing the stroller", "Kinderwagen schieben", "Steady walk at 1.1 m/s: planted feet, the stroller rolls on four wheels with its wheels turning without slip (rear wheel one turn, front wheel two turns per stride), hands stay on the push bar.", "Gleichmäßiges Gehen mit 1,1 m/s: Füße gleiten nicht, der Kinderwagen rollt auf vier Rädern, die Räder drehen schlupffrei (hinten eine, vorn zwei Umdrehungen pro Doppelschritt), die Hände bleiben am Schiebebügel.",
         T_WALK, True, "plant",
         [key(round(T_WALK * i / 10, 4), f"push-{n[0]}") for i, n in enumerate(NAMES)] + [key(T_WALK, f"push-{NAMES[0][0]}", bones={"wheel-rear": {"angleDeg": -360}, "wheel-front": {"angleDeg": -720}})],
         walk_spans(T_WALK) + [span("wheel-rear", 0, T_WALK), span("wheel-front", 0, T_WALK)] if False else walk_spans(T_WALK),
         [phase("double-support-1", 0, 0.1 * T_WALK, "Double support, left heel strike", "Doppelstütz, linker Fersenkontakt"), phase("left-single", 0.1 * T_WALK, 0.5 * T_WALK, "Left single support", "Linke Einzelstütz"), phase("double-support-2", 0.5 * T_WALK, 0.6 * T_WALK, "Double support, right heel strike", "Doppelstütz, rechter Fersenkontakt"), phase("right-single", 0.6 * T_WALK, T_WALK, "Right single support", "Rechte Einzelstütz")],
         [ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(0.5 * T_WALK, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(0.1 * T_WALK, "contact-off", "Right toe-off", "Rechter Abdruck", "contact-r"), ev(0.6 * T_WALK, "contact-off", "Left toe-off", "Linker Abdruck", "contact-l"), ev(0, "exhale", "Exhalation", "Ausatmen")],
         [track("emitter:breath", [(0, 0.6), (0.5 * T_WALK, 1.0), (T_WALK, 0.6)]), track("emitter:child-breath", [(0, 0.5), (T_WALK / 2, 0.7), (T_WALK, 0.5)])], advance=ADVANCE),
    clip("stop-and-check-child", "Stopping to check the child", "Anhalten und nach dem Kind sehen", "The parent stops, folds the hood back, stoops over the stroller to tuck the blanket and straightens up again; all four wheels stay on the ground.", "Die Person bleibt stehen, klappt das Verdeck zurück, beugt sich über den Kinderwagen, richtet die Decke und richtet sich wieder auf; alle vier Räder bleiben am Boden.",
         8.0, False, "none",
         [key(0, "pausing-with-stroller"), key(1.2, "pausing-with-stroller"), key(3.2, "checking-child"), key(5.2, "checking-child"), key(7.0, "pausing-with-stroller"), key(8.0, "pausing-with-stroller")],
         [span(a, 0, 8.0) for a in FEET + WHEELS],
         [phase("stop", 0, 1.2, "Stand still", "Stehenbleiben"), phase("stoop", 1.2, 3.2, "Stoop over the stroller", "Über den Wagen beugen"), phase("check", 3.2, 5.2, "Check the child", "Kind prüfen"), phase("straighten", 5.2, 8.0, "Straighten up", "Aufrichten")],
         [ev(0.4, "exhale", "Breath while stopping", "Atem beim Anhalten"), ev(3.2, "motion", "Hood folded back", "Verdeck zurückgeklappt"), ev(5.2, "inhale", "Breath after the check", "Atemzug nach der Kontrolle")],
         [track("activityMet", [(0, 2.6), (1.2, 1.6), (3.2, 2.4), (5.2, 2.2), (8.0, 1.5)]), track("emitter:breath", [(0, 0.8), (3.2, 0.5), (8.0, 0.4)]), track("emitter:child-breath", [(0, 0.5), (4.0, 1.0), (8.0, 0.6)])]),
    clip("kerb-tilt-up", "Tilting up at the kerb", "Am Bordstein ankippen", "The parent presses the push bar down so the front wheels leave the ground, then sets them down again on the kerb.", "Die Person drückt den Schiebebügel nach unten, sodass die Vorderräder abheben, und setzt sie danach auf dem Bordstein wieder ab.",
         3.0, False, "none",
         [key(0, "pausing-with-stroller"), key(1.0, "kerb-tilt"), key(1.8, "kerb-tilt"), key(3.0, "pausing-with-stroller")],
         [span(a, 0, 3.0) for a in FEET] + [span("wheel-rear", 0, 3.0), span("wheel-front", 0, 0.6), span("wheel-front", 2.2, 3.0)],
         [phase("press", 0, 1.0, "Press the bar down", "Bügel herunterdrücken"), phase("front-lifted", 1.0, 1.8, "Front wheels lifted", "Vorderräder angehoben"), phase("set-down", 1.8, 3.0, "Set the wheels down", "Räder absetzen")],
         [ev(0.6, "contact-off", "Front wheels lift off", "Vorderräder heben ab", "wheel-front"), ev(2.2, "contact-on", "Front wheels touch down", "Vorderräder setzen auf", "wheel-front")],
         [track("activityMet", [(0, 1.8), (1.2, 3.2), (3.0, 1.8)]), track("emitter:breath", [(0, 0.5), (1.2, 1.0), (3.0, 0.5)]), track("emitter:child-breath", [(0, 0.6), (3.0, 0.6)])]),
]

def build():
    return assemble("parent-with-stroller", T("Parent with Stroller", "Elternteil mit Kinderwagen"),
        T("A parent in a jacket and jeans pushes a stroller with a toddler: the stroller is a wheeled sub-assembly attached at the hands with its own wheel contacts; clips for the steady push-walk, stopping to check the child and tilting up at a kerb.",
          "Ein Elternteil in Jacke und Jeans schiebt einen Kinderwagen mit Kleinkind: der Kinderwagen ist eine an den Händen befestigte Baugruppe auf Rädern mit eigenen Radkontakten; Clips für das gleichmäßige Schieben, das Anhalten zur Kontrolle des Kindes und das Ankippen am Bordstein."),
        ["human", "parent", "stroller", "pram", "wheels", "child", "walking", "sub-assembly"], ["de-leva-1996", "winter-2009", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ainsworth-2011", "ashrae-2021", "en-1888", "bohannon-1997", "en-1991-1-1"],
        T("Walking key frames by planted-foot inverse kinematics; the stroller keeps level on its own wheels because the stroller bone counter-rotates the hand; the wheels roll without slip (one turn of the rear wheel and two of the front wheel per stride).", "Geh-Schlüsselbilder aus Inverskinematik mit festen Standfüßen; der Kinderwagen bleibt auf seinen Rädern waagerecht, weil der Wagenknochen die Handdrehung ausgleicht; die Räder rollen schlupffrei (eine Umdrehung des Hinterrads und zwei des Vorderrads pro Doppelschritt)."),
        {"heightM": 1.75, "widthM": 1.7, "depthM": 0.6, "massKg": 89.4}, mats, bones, anchors, elev, poses, clips, regions, emitters,
        [T("Parent 1.75 m and 68 kg (DuBois area), winter jacket and jeans about 1.1 clo; toddler in a snowsuit 10 kg.", "Elternteil 1,75 m und 68 kg (DuBois-Fläche), Winterjacke und Jeans etwa 1,1 clo; Kleinkind im Schneeanzug 10 kg."),
         T("Metabolic rate 2.6 met for pushing a stroller at 1.1 m/s (Compendium: walking pushing a stroller 3.0 to 4.0 METs at 2.5 to 3 mph), 1.5 met standing.", "Stoffwechselrate 2,6 met beim Schieben mit 1,1 m/s (Compendium: Gehen mit Kinderwagen 3,0 bis 4,0 METs bei 2,5 bis 3 mph), 1,5 met im Stand."),
         T("The toddler is a second breath source (30 breaths per minute) inside the stroller; the stroller surfaces are passive.", "Das Kleinkind ist eine zweite Atemquelle (30 Atemzüge pro Minute) im Kinderwagen; die Wagenoberflächen sind passiv.")],
        [T("The stroller (11.4 kg: frame 8, hood 0.6, wheels 2.8) and the toddler (10 kg) are bone masses hanging from the right hand; the left hand grips the same bar, so both arms carry them in the lumped elevation.", "Der Kinderwagen (11,4 kg: Rahmen 8, Verdeck 0,6, Räder 2,8) und das Kleinkind (10 kg) sind Knochenmassen an der rechten Hand; die linke Hand greift denselben Bügel, sodass beide Arme sie in der zusammengefassten Ansicht tragen."),
         T("Load sharing between the feet and the wheel contacts follows the equal-stiffness assumption of the solver, which overestimates the weight the feet carry; wheel friction is rolling friction (0.02).", "Die Lastaufteilung zwischen Füßen und Radkontakten folgt der Gleichsteifigkeitsannahme des Lösers, die das von den Füßen getragene Gewicht überschätzt; Radreibung ist Rollreibung (0,02).")],
        "pushing-stroller", "push-walk", base_met=1.5)

if __name__ == "__main__":
    json.dump(build(), open("out/parent-with-stroller.json", "w"), ensure_ascii=False, indent=1)
    print("written")
