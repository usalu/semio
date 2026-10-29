import sys
from body import *

H, M, WF = 1.78, 84.0, 1.05
h = Human(H, M, WF)
s = h.s
BOX_KG = 20.0
A = dubois_m2(H, M)

# ---------------------------------------------------------------- rig: de Leva body + two half-box bones (one per hand, the far hand coincides in elevation)
def box_bone(side):
    en, de = ("Right", "Rechte") if side == "r" else ("Left", "Linke")
    return h.bone(f"box-{side}", f"hand-{side}", f"Box half held by the {en.lower()} hand", f"Kistenhälfte in der {de.lower()} Hand", 0.075 * s, 0, 0, 0.40, {"type": "revolute", "minDeg": -150, "maxDeg": 150},
                  {"kg": BOX_KG / 2, "comX": 0, "comY": 0, "distribution": "point", "source": "literature", "reference": "niosh-1994"})

bones = h.bones() + [box_bone("l"), box_bone("r")]
anchors = h.anchors() + [{"id": "box-base", "bone": "box-r", "x": 0, "y": -0.15, "kind": "contact", "title": T("Underside of the box", "Kistenboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.40, "footprintWidthM": 0.30, "frictionCoefficient": 0.5},
                         {"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")}]
rig = Rig(bones, anchors)
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]

# ---------------------------------------------------------------- geometry (profile coordinates u forward, v up / d down)
up, down, fwd = h.up, h.down, h.fwd
HELMET = [("M", -0.112, 0.166), ("C", -0.120, 0.255, -0.062, 0.314, 0.016, 0.316), ("C", 0.082, 0.317, 0.114, 0.276, 0.118, 0.232), ("L", 0.170, 0.226), ("C", 0.180, 0.225, 0.181, 0.210, 0.170, 0.208), ("L", 0.120, 0.202), ("L", 0.100, 0.188), ("L", -0.110, 0.158), ("Z",)]
elev = h.standard(arm_sleeve="long", legs="long", shoe="boot", tones={"shirt": 0.45, "trousers": 0.26, "shoes": 0.10}, hair="short", leg_hem=0.30)
byid = {x["id"]: x for x in elev}
for side in "lr":
    byid[f"hand-{side}"].update({"region": "gloves", "material": "glove", "tone": 0.30})
    byid[f"upper-arm-{side}"]["tone"] = 0.45
elev = [x for x in elev if x["id"] not in ("hair-strands",)]
elev += [
    h.sh("vest", "torso", "body", 0, 3.4, 0.90, g_path(up, table_cmds(TORSO, 0.0, 0.43, grow=0.007), True), "vest", "hi-vis-polyester"),
    h.sh("vest-band-low", "torso", "detail", 1, 3.5, 0.55, g_path(up, table_cmds(TORSO, 0.07, 0.12, grow=0.0085), True), "vest", "retroreflective"),
    h.sh("vest-band-high", "torso", "detail", 1, 3.5, 0.55, g_path(up, table_cmds(TORSO, 0.27, 0.32, grow=0.0085), True), "vest", "retroreflective"),
    h.sh("vest-hem", "torso", "detail", 2, 3.55, 0.3, g_poly(up, [(0.117, 0.0), (-0.115, 0.0)], False), "vest", "hi-vis-polyester", False),
    h.sh("vest-strap", "torso", "detail", 2, 3.55, 0.3, g_poly(up, [(0.05, 0.43), (0.095, 0.31), (0.108, 0.1)], False), "vest", "hi-vis-polyester", False),
    h.sh("helmet", "head", "body", 0, 5.7, 0.92, g_path(up, HELMET, True), "helmet", "hdpe-shell"),
    h.sh("helmet-ridge", "head", "detail", 2, 5.8, 0.6, g_poly(up, [(-0.100, 0.232), (-0.060, 0.296), (0.016, 0.316), (0.080, 0.292), (0.108, 0.240)], False), "helmet", "hdpe-shell", False),
    h.sh("helmet-strap", "head", "detail", 3, 5.8, 0.3, g_poly(up, [(0.010, 0.176), (0.028, 0.120), (0.040, 0.080)], False), "helmet", "hdpe-shell", False),
    h.sh("shirt-cuff-glove-l", "forearm-l", "detail", 2, 0.5, 0.3, g_poly(down, [(0.030, 0.255), (-0.028, 0.255)], False), "arm-clothing", "cotton-clothing", False),
    h.sh("knee-pad-r", "shank-r", "detail", 3, 2.1, 0.18, g_path(down, [("M", 0.060, 0.0), ("C", 0.068, 0.020, 0.066, 0.075, 0.058, 0.095), ("L", 0.040, 0.095), ("L", 0.040, 0.0), ("Z",)], True), "leg-clothing", "cotton-clothing"),
    h.sh("trouser-pocket", "thigh-r", "detail", 3, 2.1, 0.2, g_poly(down, [(0.098, 0.12), (0.060, 0.14), (0.058, 0.20)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("boot-toecap", "foot-r", "detail", 3, 2.3, 0.2, g_poly(fwd, [(0.130, -0.074), (0.130, -0.036), (0.160, -0.034)], False), "shoes", "leather-shoe", False),
    h.sh("boot-toecap-l", "foot-l", "detail", 3, 1.3, 0.2, g_poly(fwd, [(0.130, -0.074), (0.130, -0.036), (0.160, -0.034)], False), "shoes", "leather-shoe", False),
    h.sh("box", "box-r", "body", 0, 5.8, 0.66, g_rect(-0.20, -0.15, 0.40, 0.30, 0.008), "box-surface", "cardboard"),
    h.sh("box-tape", "box-r", "detail", 1, 5.82, 0.8, g_rect(-0.20, 0.075, 0.40, 0.04), "box-surface", "cardboard"),
    h.sh("box-handle", "box-r", "detail", 2, 5.82, 0.2, g_rect(-0.075, -0.015, 0.15, 0.045, 0.02), "box-surface", "cardboard", False),
    h.sh("box-label", "box-r", "texture", 2, 5.82, 0.5, g_rect(-0.17, -0.12, 0.09, 0.07), "box-surface", "cardboard", False),
    h.sh("box-flap", "box-r", "texture", 3, 5.82, 0.5, g_poly(Frame("fwd", 1, 1), [(-0.20, -0.11), (0.20, -0.11)], False), "box-surface", "cardboard", False),
]

# ---------------------------------------------------------------- poses
def arms_hold(off, contacts, grip=None, grip_rel=None, bend=-1):
    y = root_y(rig, off, contacts)
    w = rig.solve(off, (0, y, 0))
    sh = rig.origin(w, "upper-arm-r")
    if grip_rel is not None:
        a = math.radians(rig.angle(w, "torso") - 90)
        c, sn = math.cos(a), math.sin(a)
        grip = (sh[0] + grip_rel[0] * c - grip_rel[1] * sn, sh[1] + grip_rel[0] * sn + grip_rel[1] * c)
    gl = 0.075 * s
    phi = 0.0
    for _ in range(4):
        wrist = (grip[0] - gl * math.cos(math.radians(phi)), grip[1] - gl * math.sin(math.radians(phi)))
        th1, th2 = two_link(sh, wrist, rig.bones["forearm-r"]["x"], rig.bones["hand-r"]["x"], bend)
        phi = th2
    out = {}
    for side in "lr":
        out.update(arm_offsets(rig, w, side, wrist, phi, bend))
    ho = out["hand-r"]["angleDeg"]
    hand_world = th2 + ho
    for side in "lr": out[f"box-{side}"] = {"angleDeg": r(-hand_world, 2)}
    return out, y

def build_pose(legs_torso, contacts, grip=None, grip_rel=None):
    arms, y = arms_hold(legs_torso, contacts, grip, grip_rel)
    return merge_offsets(legs_torso, arms)

base_stand = merge_offsets({"torso": {"angleDeg": 6}, "head": {"angleDeg": -6}})
squat = merge_offsets(legs(88, -113, 25, 88, -113, 25), {"torso": {"angleDeg": -75}, "head": {"angleDeg": 40}})
mid = merge_offsets(legs(62, -88, 25, 62, -88, 25), {"torso": {"angleDeg": -50}, "head": {"angleDeg": 30}})
setd = merge_offsets(legs(55, -82, 25, 55, -82, 25), {"torso": {"angleDeg": -42}, "head": {"angleDeg": 25}})
chest = merge_offsets({"torso": {"angleDeg": 10}, "head": {"angleDeg": -10}})

P = {}
P["squat-grip"] = (build_pose(squat, FEET + ["box-base"], grip=(0.60, 0.15)), FEET + ["box-base"])
P["lift-mid"] = (build_pose(mid, FEET, grip=(0.52, 0.52)), FEET)
P["carrying-box-low"] = (build_pose(base_stand, FEET, grip_rel=(0.30, -0.42)), FEET)
P["carrying-box-chest"] = (build_pose(chest, FEET, grip_rel=(0.34, -0.17)), FEET)
P["setting-down"] = (build_pose(setd, FEET, grip=(0.54, 0.40)), FEET)

# walking with the box at hip height: legs by inverse kinematics of a planted-foot gait (gait.py), upper body as in carrying-box-low
from gait import *
arm_low = {k: v for k, v in P["carrying-box-low"][0].items() if k.startswith(("upper-arm", "forearm", "hand", "box"))}
A_STRIDE, WALK_T = 1.10, 1.1
Lfull = h.L["thigh"] + h.L["shank"] + 0.08 * s
GK = gait_keys(h, "walk", A_STRIDE, 0.6, 0.985 * Lfull, 0.03, 35, 0.07)
for k, (nid, *_), ct in zip(GK, GAIT_NAMES, WALK_CONTACTS):
    P[f"walk-{nid}"] = (merge_offsets(leg_only(k), base_stand, arm_low), ct)

# ---------------------------------------------------------------- thermal
ADVANCE = float(sys.argv[1]) if len(sys.argv) > 1 else 1.10000629
mats = [
    material("skin", "Skin", "Haut", 0.98, "literature", "ashrae-2021", 0.6),
    material("cotton-clothing", "Work trousers and long-sleeve shirt (cotton/polyester)", "Arbeitshose und langärmliges Hemd (Baumwolle/Polyester)", 0.95, "literature", "iso-9920-2007", 0.55),
    material("leather-shoe", "Leather safety boot", "Sicherheitsstiefel aus Leder", 0.90, "authored-estimate", None, 0.75),
    material("hair", "Hair", "Haar", 0.95, "authored-estimate", None, 0.85),
    material("glove", "Leather work glove", "Lederarbeitshandschuh", 0.90, "authored-estimate", None, 0.7),
    material("hi-vis-polyester", "Fluorescent yellow polyester vest (ISO 20471 class 2)", "Fluoreszierende gelbe Polyesterweste (ISO 20471 Klasse 2)", 0.90, "literature", "iso-20471", 0.40),
    material("retroreflective", "Retroreflective band", "Retroreflektierendes Band", 0.85, "literature", "iso-20471", 0.30),
    material("hdpe-shell", "Safety helmet shell (HDPE)", "Schutzhelm-Schale (HDPE)", 0.92, "literature", "en-397", 0.35, 950, 0.45),
    material("cardboard", "Corrugated cardboard box", "Wellpappkiste", 0.90, "authored-estimate", None, 0.6, 150, 0.06),
]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A, 0.15, "core-resistance", 36.8, 0.05, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair", "Haar", "hair", "hair", 0.02 * A, 0.15, "core-resistance", 36.8, 0.10, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("helmet", "Safety helmet shell", "Schutzhelm-Schale", "surface", "hdpe-shell", 0.03 * A, 0.20, "passive", None, None, None, "authored-estimate", "en-397"),
    reg("torso-clothing", "Shirt under the vest", "Hemd unter der Weste", "clothing", "cotton-clothing", 0.21 * A, 0.30, "core-resistance", 36.8, 0.15, 0.30),
    reg("vest", "High-visibility vest", "Warnweste", "clothing", "hi-vis-polyester", 0.15 * A, 0.30, "core-resistance", 36.8, 0.18, 0.12, "literature", "iso-20471"),
    reg("arm-clothing", "Long sleeves", "Lange Ärmel", "clothing", "cotton-clothing", 0.13 * A, 0.08, "core-resistance", 36.8, 0.14, 0.13),
    reg("gloves", "Work gloves", "Arbeitshandschuhe", "clothing", "glove", 0.05 * A, 0.07, "core-resistance", 36.8, 0.10, 0.03, "authored-estimate", "iso-9920-2007"),
    reg("leg-clothing", "Work trousers", "Arbeitshose", "clothing", "cotton-clothing", 0.32 * A, 0.13, "core-resistance", 36.8, 0.17, 0.36),
    reg("shoes", "Safety boots", "Sicherheitsstiefel", "surface", "leather-shoe", 0.07 * A, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("box-surface", "Cardboard box surface", "Kistenoberfläche", "surface", "cardboard", 0.66, 0.30, "passive", None, None, None, "authored-estimate", "en-1991-1-1"),
]
emitters = [emitter("breath", "mouth", 3.0, vel=2.0, diam=0.035)]

def pz(id, en, de, den, dde, pkey, met, br, key=False):
    offs, contacts = P[pkey]
    return pose(id, en, de, den, dde, offs, contacts, met, {"breath": br}, key=key)

poses = [
    pz("squat-grip", "Squat, hands on the box", "Hocke, Hände an der Kiste", "Deep squat with a straight back and straight arms; the 20 kg box still rests on the ground and shares its weight with the floor.", "Tiefe Hocke mit geradem Rücken und gestreckten Armen; die 20-kg-Kiste steht noch auf dem Boden und gibt einen Teil ihres Gewichts an den Boden ab.", "squat-grip", 4.5, 0.9),
    pz("lift-mid", "Mid lift", "Mitte des Hebens", "Half-risen, trunk flexed 50 degrees: the box hangs on straight arms far in front of the lumbar joint, the largest lever arm of the lift.", "Halb aufgerichtet, Rumpf 50 Grad vorgeneigt: die Kiste hängt an gestreckten Armen weit vor dem Lendengelenk, der größte Hebelarm des Hebens.", "lift-mid", 5.0, 1.0),
    pz("carrying-box-low", "Carrying at hip height", "Tragen auf Hüfthöhe", "Upright, trunk leaning back 6 degrees to balance the box held at hip height with extended arms.", "Aufrecht, Rumpf 6 Grad zurückgelehnt, um die Kiste mit gestreckten Armen auf Hüfthöhe auszugleichen.", "carrying-box-low", 3.4, 0.7),
    pz("carrying-box-chest", "Carrying at chest height", "Tragen auf Brusthöhe", "Box held close at chest height with the elbows bent: shorter lever arm at the lumbar joint, longer one at the shoulder and elbow.", "Kiste nah vor der Brust mit gebeugten Ellbogen: kürzerer Hebelarm am Lendengelenk, längerer an Schulter und Ellbogen.", "carrying-box-chest", 3.8, 0.8),
    pz("setting-down", "Setting the box down", "Kiste absetzen", "Bent knees and flexed trunk while lowering the box to a pallet; the box hangs free.", "Gebeugte Knie und vorgeneigter Rumpf beim Absenken der Kiste auf eine Palette; die Kiste hängt frei.", "setting-down", 4.2, 0.9),
] + [pz(f"walk-{n[0]}", f"Loaded gait: {n[1]}", f"Belasteter Gang: {n[2]}", n[3], n[4], f"walk-{n[0]}", 3.8, 0.8, key=True) for n in GAIT_NAMES]

clips = [
    clip("squat-lift", "Squat lift from the floor", "Hebevorgang aus der Hocke", "Lifting the 20 kg box from the floor with a straight back: the box leaves the ground at 0.7 s and the load moves entirely onto the arms; the lumbar moment peaks mid lift.", "Anheben der 20-kg-Kiste vom Boden mit geradem Rücken: bei 0,7 s hebt die Kiste ab und die Last geht vollständig auf die Arme; das Lendenmoment ist in der Mitte des Hebens am größten.",
         3.2, False, "none",
         [key(0, "squat-grip"), key(0.6, "squat-grip"), key(1.6, "lift-mid"), key(2.6, "carrying-box-low"), key(3.2, "carrying-box-low")],
         [span("heel-l", 0, 3.2), span("heel-r", 0, 3.2), span("contact-l", 0, 3.2), span("contact-r", 0, 3.2), span("box-base", 0, 0.7)],
         [phase("brace", 0, 0.6, "Grip and brace", "Greifen und Anspannen"), phase("lift", 0.6, 2.6, "Lift", "Heben"), phase("carry-stance", 2.6, 3.2, "Stand with the load", "Stand mit Last")],
         [ev(0.2, "inhale", "Deep breath before the lift", "Tiefes Einatmen vor dem Heben"), ev(0.7, "contact-off", "Box leaves the ground", "Kiste hebt ab", "box-base"), ev(0.7, "load-on", "Full box weight on the arms", "Volles Kistengewicht auf den Armen", "grip-r"), ev(1.2, "exhale", "Effort exhalation", "Anstrengungsausatmung")],
         [track("activityMet", [(0, 3.0), (1.2, 5.0), (2.8, 3.6), (3.2, 3.4)]), track("emitter:breath", [(0, 0.3), (0.5, 0), (1.0, 1.0), (1.8, 0.8), (2.6, 0.4), (3.2, 0.7)])]),
    clip("carry-walk", "Walking with the box", "Gehen mit Kiste", "Level walking with the 20 kg box at hip height, arms locked, shortened stride, planted feet.", "Gehen in der Ebene mit der 20-kg-Kiste auf Hüfthöhe, Arme fixiert, verkürzter Schritt, Füße gleiten nicht.",
         WALK_T, True, "plant",
         [key(round(WALK_T * i / 10, 4), f"walk-{n[0]}") for i, n in enumerate(GAIT_NAMES)],
         walk_spans(WALK_T),
         [phase("double-support-1", 0, 0.11, "Double support, left heel strike", "Doppelstütz, linker Fersenkontakt"), phase("left-single", 0.11, 0.55, "Left single support", "Linke Einzelstütz"), phase("double-support-2", 0.55, 0.66, "Double support, right heel strike", "Doppelstütz, rechter Fersenkontakt"), phase("right-single", 0.66, 1.1, "Right single support", "Rechte Einzelstütz")],
         [ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(0.11, "contact-off", "Right toe-off", "Rechter Abdruck", "contact-r"), ev(0.33, "contact-off", "Left heel-off", "Linke Fersenabhebung", "heel-l"), ev(0.55, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(0.66, "contact-off", "Left toe-off", "Linker Abdruck", "contact-l"), ev(0.88, "contact-off", "Right heel-off", "Rechte Fersenabhebung", "heel-r")],
         [track("emitter:breath", [(0, 0.6), (0.55, 1.0), (1.1, 0.6)])], advance=ADVANCE),
    clip("set-down", "Setting the box down", "Kiste absetzen", "Lowering the carried box to the ground in a squat and releasing its weight onto the floor.", "Absenken der getragenen Kiste in der Hocke bis auf den Boden; das Gewicht geht auf den Boden über.",
         3.0, False, "none",
         [key(0, "carrying-box-low"), key(1.0, "setting-down"), key(2.2, "squat-grip"), key(3.0, "squat-grip")],
         [span("heel-l", 0, 3.0), span("heel-r", 0, 3.0), span("contact-l", 0, 3.0), span("contact-r", 0, 3.0), span("box-base", 2.2, 3.0)],
         [phase("lower", 0, 2.2, "Lower", "Absenken"), phase("release", 2.2, 3.0, "Box on the ground", "Kiste am Boden")],
         [ev(0.5, "exhale", "Controlled exhale while lowering", "Kontrolliertes Ausatmen beim Absenken"), ev(2.2, "contact-on", "Box touches the ground", "Kiste berührt den Boden", "box-base"), ev(2.2, "load-off", "Box weight leaves the arms", "Kistengewicht verlässt die Arme", "grip-r"), ev(2.6, "inhale", "Breath after the set-down", "Atemzug nach dem Absetzen")],
         [track("activityMet", [(0, 3.4), (1.2, 4.2), (3.0, 2.2)]), track("emitter:breath", [(0, 0.7), (1.0, 1.0), (2.2, 0.6), (3.0, 0.3)])]),
]

def build():
    return {
        "id": "worker-carrying-box", "category": "humans",
        "title": T("Worker Carrying Box", "Arbeitende Person mit Kiste"),
        "summary": T("A construction worker in a high-visibility vest and hard hat lifts a 20 kg box from the floor in a squat and carries it at hip or chest height; trunk, shoulder and elbow moments change with the lever arm of the load.",
                     "Eine Bauarbeiterin bzw. ein Bauarbeiter mit Warnweste und Schutzhelm hebt eine 20-kg-Kiste in der Hocke vom Boden und trägt sie auf Hüft- oder Brusthöhe; Rumpf-, Schulter- und Ellbogenmomente ändern sich mit dem Hebelarm der Last."),
        "tags": ["human", "worker", "construction", "lifting", "carrying", "load", "hi-vis", "hard-hat"],
        "provenance": {"author": "Semio staffage humans-b content worker", "method": "handcrafted", "created": "2026-10-29",
                       "references": refs("de-leva-1996", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ainsworth-2011", "ashrae-2021", "niosh-1994", "iso-11228-1", "iso-20471", "en-397", "en-1991-1-1"),
                       "notes": T("Silhouette handcrafted from side-elevation proportions (7.5 heads); segment masses by de Leva male fractions for 84 kg; the box is two 10 kg half-bones, one per hand, because both hands coincide in elevation.",
                                  "Silhouette handgezeichnet nach Seitenansichts-Proportionen (7,5 Köpfe); Segmentmassen nach den männlichen Anteilen von de Leva für 84 kg; die Kiste besteht aus zwei 10-kg-Halbknochen, je einer pro Hand, weil beide Hände in der Ansicht zusammenfallen.")},
        "dimensions": {"heightM": 1.81, "widthM": 0.68, "depthM": 0.5, "massKg": 104.0},
        "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": bones}, "anchors": anchors,
        "geometry": {"elevation": elev}, "poses": poses, "clips": clips,
        "thermal": {"environment": ENV_STD, "baseMet": 1.5, "regions": regions, "emitters": emitters, "assumptions": [
            T("DuBois area for 1.78 m and 84 kg split by Lund-Browder-style fractions; clothing insulation from ISO 9920 (long-sleeve work clothes about 0.9 clo plus vest, boots and gloves).", "DuBois-Fläche für 1,78 m und 84 kg, aufgeteilt nach Lund-Browder-ähnlichen Anteilen; Bekleidungsdämmung nach ISO 9920 (langärmlige Arbeitskleidung etwa 0,9 clo plus Weste, Stiefel und Handschuhe)."),
            T("Metabolic rate by ISO 8996 class 3 to 4: 3.4 met carrying, 5.0 met at the peak of the lift (Compendium: lifting 15-49 lb 3.5 met, heavy 6 met).", "Stoffwechselrate nach ISO 8996 Klasse 3 bis 4: 3,4 met beim Tragen, 5,0 met am Höhepunkt des Hebens (Compendium: Heben 15-49 lb 3,5 met, schwer 6 met)."),
            T("The box is a passive cardboard surface at ambient temperature.", "Die Kiste ist eine passive Wellpappenoberfläche mit Umgebungstemperatur.")]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [
            T("Segment masses are point masses at de Leva centres of mass; hand centres are authored estimates; rotational inertia is neglected.", "Segmentmassen sind Punktmassen in den Schwerpunkten nach de Leva; Handschwerpunkte sind Schätzwerte; Drehträgheit wird vernachlässigt."),
            T("The 20 kg box is two 10 kg point masses at the grips (NIOSH load constant 23 kg); while the box rests on the floor its base contact shares the weight with the feet under the equal-stiffness assumption.", "Die 20-kg-Kiste besteht aus zwei 10-kg-Punktmassen an den Griffen (NIOSH-Lastkonstante 23 kg); steht sie am Boden, teilt ihr Bodenkontakt das Gewicht unter der Annahme gleicher Steifigkeit mit den Füßen."),
            T("Both arms are lumped in the elevation: the near arm carries the left-hand half of the box through its own half bone.", "Beide Arme fallen in der Ansicht zusammen: der nahe Arm trägt die Hälfte der Kiste über seinen eigenen Halbknochen.")], "clinical": False},
        "presentation": {"defaultPose": "carrying-box-low", "defaultClip": "carry-walk", "viewpoints": ["elevation"]},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}

if __name__ == "__main__":
    json.dump(build(), open("out/worker-carrying-box.json", "w"), ensure_ascii=False, indent=1)
    print("written")
