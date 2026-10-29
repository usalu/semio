import sys
from body import *
from gait import leg_ik

H, M, WF = 1.75, 78.0, 1.03
h = Human(H, M, WF)
s = h.s
A_BODY = dubois_m2(H, M)
fr = Frame("fwd", 1.0, 1.0)
up, down, fwd = h.up, h.down, h.fwd
rv = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
def lit(kg, comX, comY=0.0, src="authored-estimate", ref=None):
    m = {"kg": kg, "comX": comX, "comY": comY, "distribution": "point", "source": src}
    if ref: m["reference"] = ref
    return m
POLE_TOP, POLE_END, FLAG_L = 1.0, -0.30, 0.46
extra = [
    h.bone("pointer", "hand-r", "Telescopic pointer", "Teleskop-Zeigestab", 0.10 * s, 0, 0, 0.55, {"type": "fixed"}, lit(0.12, 0.25)),
    h.bone("flag-pole", "hand-l", "Flag pole", "Fahnenstange", 0.075 * s, 0, 0, POLE_TOP, rv(-170, 170), lit(0.35, 0.35)),
    h.bone("flag", "flag-pole", "Flag cloth", "Fahnentuch", POLE_TOP - 0.03, 0, 90, FLAG_L, rv(-25, 25), lit(0.12, 0.23)),
]
bones = h.bones() + extra
anchors = h.anchors() + [
    {"id": "pointer-tip", "bone": "pointer", "x": 0.55, "y": 0, "kind": "attachment", "title": T("Pointer tip", "Spitze des Zeigestabs")},
    {"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")},
    {"id": "shoulder-r", "bone": "upper-arm-r", "x": 0, "y": 0, "kind": "load", "title": T("Right shoulder joint", "Rechtes Schultergelenk")},
]
rig = Rig(bones, anchors)
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
Lleg = h.L["thigh"] + h.L["shank"]

elev = h.standard(arm_sleeve="long", legs="long", shoe="sneaker", tones={"shirt": 0.42, "trousers": 0.24, "shoes": 0.85, "skin": 0.84}, hair="short", hair_tone=0.35)
elev = [x for x in elev if x["id"] not in ("torso",)]
elev += [
    h.sh("torso", "torso", "body", 0, 3.2, 0.42, g_path(up, table_cmds(TORSO, -0.12, 0.46, grow=0.008), True), "torso-clothing", "cotton-clothing"),
    h.sh("jacket-hem", "torso", "detail", 2, 3.3, 0.2, g_poly(up, [(0.108, -0.12), (-0.106, -0.12)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("jacket-zip", "torso", "detail", 2, 3.3, 0.2, g_poly(up, [(0.112, -0.1), (0.126, 0.2), (0.1, 0.43)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("lanyard", "torso", "detail", 2, 3.5, 0.9, g_poly(up, [(0.07, 0.45), (0.134, 0.30), (0.136, 0.22)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("badge", "torso", "body", 2, 3.55, 0.95, g_rect(0.215, -0.14, 0.085, 0.056, 0.004), "torso-clothing", "cotton-clothing"),
    h.sh("cap", "head", "body", 1, 5.7, 0.2, g_path(up, [("M", -0.106, 0.18), ("C", -0.108, 0.27, -0.05, 0.305, 0.02, 0.303), ("C", 0.07, 0.30, 0.095, 0.27, 0.093, 0.215), ("L", 0.165, 0.205), ("C", 0.172, 0.205, 0.172, 0.195, 0.165, 0.193), ("L", 0.09, 0.196), ("C", 0.04, 0.21, -0.04, 0.2, -0.106, 0.175), ("Z",)], True), "hair", "cap-fabric"),
    h.sh("cap-seam", "head", "detail", 2, 5.75, 0.6, g_poly(up, [(-0.09, 0.26), (0.0, 0.303), (0.08, 0.265)], False), "hair", "cap-fabric", False),
    h.sh("pointer", "pointer", "body", 0, 6.4, 0.15, g_capsule(fr, 0.0, 0, 0.55, 0, 0.005, 0.003), "pointer-metal", "pointer-aluminium"),
    h.sh("pointer-tip", "pointer", "detail", 2, 6.45, 0.35, g_capsule(fr, 0.50, 0, 0.55, 0, 0.0065, 0.004), "pointer-metal", "pointer-aluminium"),
    h.sh("pole", "flag-pole", "body", 0, 6.3, 0.2, g_capsule(fr, POLE_END, 0, POLE_TOP + 0.05, 0, 0.009, 0.009), "flag-cloth", "flag-textile"),
    h.sh("pole-finial", "flag-pole", "detail", 1, 6.35, 0.8, g_ellipse(fr, POLE_TOP + 0.06, 0, 0.016, 0.016), "flag-cloth", "flag-textile"),
    h.sh("flag", "flag", "body", 0, 6.2, 0.78, g_path(fr, [("M", 0, 0), ("C", 0.12, -0.035, 0.26, 0.04, FLAG_L, 0.0), ("L", FLAG_L, 0.28), ("C", 0.28, 0.32, 0.14, 0.25, 0, 0.28), ("Z",)], True), "flag-cloth", "flag-textile"),
    h.sh("flag-emblem", "flag", "detail", 2, 6.25, 0.2, g_ellipse(fr, 0.22, 0.14, 0.07, 0.07), "flag-cloth", "flag-textile", False),
    h.sh("flag-stripe", "flag", "texture", 3, 6.25, 0.3, g_poly(fr, [(0.02, 0.14), (0.15, 0.17), (0.30, 0.12), (0.44, 0.15)], False), "flag-cloth", "flag-textile", False),
]
for x in elev:
    if x["id"].startswith("hand-"): x["region"] = "hand-skin"

def stand_legs(px, bxl, bxr, theta=0.0):
    ys = []
    for bx in (bxl, bxr):
        ankle = (bx - 0.13 * s, 0.08 * s)
        dx = ankle[0] - px
        ys.append(0.08 * s + math.sqrt(max(1e-6, (0.99 * Lleg) ** 2 - dx * dx)))
    py = min(ys)
    off = {}
    for sd, bx in zip("lr", (bxl, bxr)):
        ankle = (bx - 0.13 * s, 0.08 * s)
        t1, t2 = leg_ik((px, py), ankle, h.L["thigh"], h.L["shank"])
        off[f"thigh-{sd}"] = {"angleDeg": r(norm(t1 + 90 - theta), 2)}
        off[f"shank-{sd}"] = {"angleDeg": r(norm(t2 - t1), 2)}
        off[f"foot-{sd}"] = {"angleDeg": r(norm(0 - t2 - 90 + 0), 2)}
    return off, py

def pose_off(feet, torso=0.0, head=0.0, arm_r=None, arm_l=None, pole_deg=None, flag_deg=0.0, theta=0.0):
    legs_off, py = stand_legs(0.0, feet[0], feet[1], theta)
    off = merge_offsets(legs_off, {"torso": {"angleDeg": torso}, "head": {"angleDeg": head}})
    w = rig.solve(off, (0, py, theta))
    def reach(sd, spec):
        kind = spec[0]
        if kind == "off": return spec[1]
        sh = rig.origin(w, f"upper-arm-{sd}")
        tgt = (sh[0] + spec[1], sh[1] + spec[2])
        phi = spec[3]
        wr = (tgt[0] - 0.075 * s * math.cos(math.radians(phi)), tgt[1] - 0.075 * s * math.sin(math.radians(phi)))
        return arm_offsets(rig, w, sd, wr, phi, bend=spec[4])
    if arm_r: off = merge_offsets(off, reach("r", arm_r))
    if arm_l: off = merge_offsets(off, reach("l", arm_l))
    w2 = rig.solve(off, (0, py, theta))
    if pole_deg is not None:
        off["flag-pole"] = {"angleDeg": r(norm(pole_deg - rig.angle(w2, "hand-l")), 2)}
    off["flag"] = {"angleDeg": flag_deg}
    return off, py

# stances (ball x of left, right foot): relaxed, one foot ahead
STAND = (0.17, 0.05)
POINT_R = ("ik", 0.58, 0.40, 40.0, -1)           # wrist/grip target relative to the shoulder: arm straight up-forward
OPEN_R = ("off", {"upper-arm-r": {"angleDeg": 52}, "forearm-r": {"angleDeg": 38}, "hand-r": {"angleDeg": -25}})
FLAG_HOLD = ("ik", 0.22, -0.24, 25.0, -1)         # left hand at chest holding the pole upright
FLAG_HIGH = ("ik", 0.30, 0.18, 35.0, -1)
WAVE_R = ("off", {"upper-arm-r": {"angleDeg": 158}, "forearm-r": {"angleDeg": 55}, "hand-r": {"angleDeg": 10}})

P = {}
P["pointing"] = pose_off(STAND, torso=-3, head=2, arm_r=POINT_R, arm_l=FLAG_HOLD, pole_deg=90, flag_deg=-8)
P["gesturing"] = pose_off(STAND, torso=-2, head=-2, arm_r=OPEN_R, arm_l=FLAG_HOLD, pole_deg=90, flag_deg=6)
P["flag"] = pose_off(STAND, torso=0, head=0, arm_r=("off", {"upper-arm-r": {"angleDeg": 6}, "forearm-r": {"angleDeg": 14}, "hand-r": {"angleDeg": 0}}), arm_l=FLAG_HIGH, pole_deg=90, flag_deg=0)
P["waving"] = pose_off((0.10, 0.08), torso=2, head=-6, arm_r=WAVE_R, arm_l=FLAG_HOLD, pole_deg=90, flag_deg=10)
P["wave-a"] = pose_off((0.10, 0.08), torso=2, head=-6, arm_r=("off", {"upper-arm-r": {"angleDeg": 150}, "forearm-r": {"angleDeg": 80}, "hand-r": {"angleDeg": 25}}), arm_l=FLAG_HOLD, pole_deg=90, flag_deg=-10)
P["wave-b"] = pose_off((0.10, 0.08), torso=2, head=-6, arm_r=("off", {"upper-arm-r": {"angleDeg": 164}, "forearm-r": {"angleDeg": 32}, "hand-r": {"angleDeg": -5}}), arm_l=FLAG_HOLD, pole_deg=90, flag_deg=12)
P["turn"] = pose_off((0.02, 0.20), torso=4, head=-14, arm_r=("off", {"upper-arm-r": {"angleDeg": 20}, "forearm-r": {"angleDeg": 40}, "hand-r": {"angleDeg": 0}}), arm_l=FLAG_HOLD, pole_deg=90, flag_deg=4)

def pz(id, en, de, den, dde, k, met, br, key=False):
    off, py = P[k]
    return pose(id, en, de, den, dde, off, FEET, met, {"breath": br}, key=key)

poses = [
    pz("standing-pointing", "Pointing at a building", "Auf ein Gebäude zeigen", "The right arm is raised straight up and forward with a telescopic pointer while the left hand holds the flag pole; the raised arm loads the shoulder with a moment of about 20 N m.", "Der rechte Arm ist mit einem Teleskop-Zeigestab gestreckt nach vorn oben gehoben, die linke Hand hält die Fahnenstange; der gehobene Arm belastet die Schulter mit einem Moment von etwa 20 N m.", "pointing", 1.5, 0.9),
    pz("gesturing-to-group", "Gesturing to the group", "Zur Gruppe gestikulieren", "Open gesture of the right forearm towards the group at chest height, flag held upright.", "Offene Geste des rechten Unterarms zur Gruppe auf Brusthöhe, Fahne aufrecht gehalten.", "gesturing", 1.4, 0.8),
    pz("holding-flag", "Holding the flag high", "Fahne hoch halten", "Standing relaxed with the flag raised above the head so that the group can follow it; the right arm hangs.", "Entspannt stehend mit hochgehaltener Fahne, damit die Gruppe ihr folgen kann; der rechte Arm hängt.", "flag", 1.3, 0.6),
    pz("waving", "Waving with a raised arm", "Mit erhobenem Arm winken", "Feet turned towards the group, head turned back, the right arm raised above the head in a wave.", "Füße zur Gruppe gedreht, Kopf zurückgewandt, der rechte Arm winkend über den Kopf gehoben.", "waving", 1.7, 0.9),
    pz("wave-swing-a", "Wave, forearm out", "Winken, Unterarm außen", "Wave key frame: forearm swung out.", "Winken-Schlüsselbild: Unterarm ausgeschwungen.", "wave-a", 1.7, 0.9, key=True),
    pz("wave-swing-b", "Wave, forearm in", "Winken, Unterarm innen", "Wave key frame: forearm swung in.", "Winken-Schlüsselbild: Unterarm eingeschwungen.", "wave-b", 1.7, 0.9, key=True),
    pz("turning-to-group", "Turning to the group", "Der Gruppe zuwenden", "Weight moved to the left foot, head and trunk turn towards the group, the right arm lifts.", "Das Gewicht wandert auf den linken Fuß, Kopf und Rumpf wenden sich der Gruppe zu, der rechte Arm hebt sich.", "turn", 1.5, 0.8, key=True),
]
mats = std_materials() + [
    material("cap-fabric", "Cotton cap", "Baumwollkappe", 0.92, "authored-estimate", None, 0.6),
    material("pointer-aluminium", "Anodised aluminium pointer", "Eloxierter Aluminium-Zeigestab", 0.2, "literature", "en-1991-1-1", 0.4, 2700, 160),
    material("flag-textile", "Polyester flag cloth and fibreglass pole", "Polyester-Fahnentuch und Glasfaserstange", 0.90, "authored-estimate", None, 0.5, 1380, 0.2),
]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.05, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair and cap", "Haar und Kappe", "hair", "hair", 0.035 * A_BODY, 0.15, "core-resistance", 36.8, 0.12, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("torso-clothing", "Jacket, lanyard and badge", "Jacke, Band und Ausweis", "clothing", "cotton-clothing", 0.36 * A_BODY, 0.30, "core-resistance", 36.8, 0.17, 0.40, "literature", "iso-9920-2007"),
    reg("arm-clothing", "Jacket sleeves", "Jackenärmel", "clothing", "cotton-clothing", 0.13 * A_BODY, 0.08, "core-resistance", 36.8, 0.15, 0.13, "literature", "iso-9920-2007"),
    reg("arm-skin", "Bare wrists", "Nackte Handgelenke", "skin", "skin", 0.002 * A_BODY, 0.05, "core-resistance", 36.8, 0.05, 0.0, "literature", "dubois-1916"),
    reg("hand-skin", "Hands", "Hände", "skin", "skin", 0.05 * A_BODY, 0.07, "core-resistance", 36.8, 0.05, 0.03, "literature", "dubois-1916", wet=0.06),
    reg("leg-clothing", "Trousers", "Hose", "clothing", "cotton-clothing", 0.32 * A_BODY, 0.13, "core-resistance", 36.8, 0.15, 0.38, "literature", "iso-9920-2007"),
    reg("shoes", "Trainers", "Turnschuhe", "surface", "leather-shoe", 0.07 * A_BODY, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("pointer-metal", "Pointer", "Zeigestab", "surface", "pointer-aluminium", 0.01, 0.01, "passive", None, None, None, "authored-estimate", "en-1991-1-1"),
    reg("flag-cloth", "Flag cloth and pole", "Fahnentuch und Stange", "surface", "flag-textile", 0.4, 0.3, "passive", None, None, None, "authored-estimate", "en-1991-1-4"),
]
elev.append(h.sh("wrist-skin-r", "forearm-r", "detail", 3, 6.1, 0.8, g_poly(down, [(0.026, 0.25), (-0.024, 0.25)], False), "arm-skin", "skin", False))
emitters = [emitter("breath", "mouth", 3.2, vel=1.8, diam=0.032)]
ENV = {"airTempC": 16, "meanRadiantTempC": 17, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 250, "pressurePa": 101325}

clips = [
    clip("gesture-point", "Gesture and point", "Gestikulieren und Zeigen", "The guide raises the right arm with the pointer to a building, holds the pointing arm (shoulder moment peaks), opens to a gesture towards the group and relaxes with the flag raised.", "Der Guide hebt den rechten Arm mit dem Zeigestab zu einem Gebäude, hält den Zeigearm (das Schultermoment ist maximal), öffnet sich zur Gruppe und entspannt mit hochgehaltener Fahne.",
         8.0, False, "none",
         [key(0, "holding-flag"), key(1.2, "holding-flag"), key(2.6, "standing-pointing"), key(4.6, "standing-pointing"), key(6.0, "gesturing-to-group"), key(8.0, "holding-flag")],
         [span(a, 0, 8.0) for a in FEET],
         [phase("rest", 0, 1.2, "Flag held high", "Fahne hoch gehalten"), phase("raise", 1.2, 2.6, "Raise the arm", "Arm heben"), phase("point", 2.6, 4.6, "Point and hold", "Zeigen und halten"), phase("gesture", 4.6, 6.0, "Open gesture", "Offene Geste"), phase("relax", 6.0, 8.0, "Relax", "Entspannen")],
         [ev(0.8, "inhale", "Breath before speaking", "Atemzug vor dem Sprechen"), ev(2.6, "motion", "Arm fully raised", "Arm ganz erhoben"), ev(3.4, "exhale", "Speaking while pointing", "Sprechen beim Zeigen"), ev(6.0, "motion", "Open gesture", "Offene Geste")],
         [track("activityMet", [(0, 1.3), (2.6, 1.5), (4.6, 1.5), (8.0, 1.3)]), track("emitter:breath", [(0, 0.5), (3.0, 0.9), (5.0, 0.8), (8.0, 0.5)])]),
    clip("turn-and-wave", "Turn and wave", "Umdrehen und Winken", "The guide shifts weight, turns head and trunk towards the group and waves with the raised arm while the flag sways.", "Der Guide verlagert das Gewicht, dreht Kopf und Rumpf zur Gruppe und winkt mit erhobenem Arm, während die Fahne schwingt.",
         6.0, False, "none",
         [key(0, "holding-flag"), key(1.2, "turning-to-group"), key(2.2, "waving"), key(2.8, "wave-swing-a"), key(3.4, "wave-swing-b"), key(4.0, "wave-swing-a"), key(4.6, "wave-swing-b"), key(5.2, "waving"), key(6.0, "holding-flag")],
         [span(a, 0, 6.0) for a in FEET],
         [phase("turn", 0, 2.2, "Turn to the group", "Der Gruppe zuwenden"), phase("wave", 2.2, 5.2, "Wave", "Winken"), phase("return", 5.2, 6.0, "Lower the arm", "Arm senken")],
         [ev(1.2, "motion", "Weight shift and head turn", "Gewichtsverlagerung und Kopfdrehung"), ev(2.8, "motion", "Wave", "Winken"), ev(5.2, "exhale", "Exhalation after the wave", "Ausatmen nach dem Winken")],
         [track("activityMet", [(0, 1.3), (2.8, 1.8), (4.6, 1.8), (6.0, 1.4)]), track("emitter:breath", [(0, 0.5), (3.0, 0.9), (6.0, 0.5)])]),
]

def build():
    return assemble("tour-guide", T("Tour Guide", "Stadtführende Person"),
        T("A tour guide in a jacket and cap points with a telescopic pointer and waves with the raised right arm while the left hand holds a flag high; the raised arm loads the shoulder with a clearly visible moment.", "Eine stadtführende Person in Jacke und Kappe zeigt mit einem Teleskop-Zeigestab und winkt mit dem erhobenen rechten Arm, während die linke Hand eine Fahne hochhält; der erhobene Arm belastet die Schulter mit einem deutlich sichtbaren Moment."),
        ["human", "guide", "tourism", "gesture", "pointing", "waving", "flag"], ["de-leva-1996", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ashrae-2021", "winter-2009", "en-1991-1-4", "en-1991-1-1"],
        T("Arm poses by inverse kinematics from shoulder-relative targets; the pole bone counter-rotates the hand so the flag pole stays vertical; feet by planted-foot inverse kinematics.", "Armposen per Inverskinematik aus schulterrelativen Zielen; der Stangenknochen gleicht die Handdrehung aus, sodass die Fahnenstange senkrecht bleibt; Füße per Inverskinematik mit festen Standfüßen."),
        {"heightM": 2.275, "widthM": 1.254, "depthM": 0.5, "massKg": 78.59}, mats, bones, anchors, elev, poses, clips, regions, emitters,
        [T("Reference case 16 degrees C with 250 W/m2 solar radiation outdoors; jacket and trousers about 1.0 clo (ISO 9920); speaking and gesturing 1.3 to 1.8 met (ISO 8996 class 1).", "Referenzfall 16 Grad C mit 250 W/m2 Solarstrahlung im Freien; Jacke und Hose etwa 1,0 clo (ISO 9920); Sprechen und Gestikulieren 1,3 bis 1,8 met (ISO 8996 Klasse 1)."),
         T("Speech raises the breath emitter strength; the flag and pointer are passive surfaces.", "Sprechen erhöht die Stärke des Atemstrahls; Fahne und Zeigestab sind passive Oberflächen.")],
        [T("The raised arm carries the pointer (0.12 kg); with the straight arm raised 40 degrees the shoulder moment is the weight of arm, hand and pointer times their lever arms; the flag (0.47 kg) loads the left arm.", "Der gehobene Arm trägt den Zeigestab (0,12 kg); beim gestreckten, 40 Grad gehobenen Arm ist das Schultermoment das Gewicht von Arm, Hand und Zeigestab mal ihren Hebelarmen; die Fahne (0,47 kg) belastet den linken Arm."),
         T("Gestures are quasi-static key frames; the inertial model adds the segment accelerations of the interpolated motion.", "Gesten sind quasistatische Schlüsselbilder; das Trägheitsmodell fügt die Segmentbeschleunigungen der interpolierten Bewegung hinzu.")],
        "standing-pointing", "gesture-point", base_met=1.3, env=ENV)

if __name__ == "__main__":
    json.dump(build(), open("out/tour-guide.json", "w"), ensure_ascii=False, indent=1)
    print("written")
