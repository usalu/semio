import sys
from body import *
from gait import leg_ik

H, M, WF = 1.72, 66.0, 0.98
h = Human(H, M, WF)
s = h.s
A_BODY = dubois_m2(H, M)
fr = Frame("fwd", 1.0, 1.0)
up, down, fwd = h.up, h.down, h.fwd
rv = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
pr = lambda ax, lo, hi: {"type": "prismatic", "axisDeg": ax, "minM": lo, "maxM": hi}
wall_bones = [
    h.bone("wall-frame", "pelvis", "World-aligned frame of the wall", "Weltausgerichteter Wandrahmen", 0, 0, -90, 0.1, rv(-60, 60)),
    h.bone("wall-x", "wall-frame", "Wall position across", "Wandlage quer", 0, 0, 0, 0.1, pr(0, -1.5, 1.5)),
    h.bone("wall", "wall-x", "Wall", "Wand", 0, 0, 0, 2.2, pr(90, -1.5, 1.5)),
]
bones = h.bones() + wall_bones
SH_U, SH_V = -0.128, 0.33
anchors = h.anchors() + [
    {"id": "wall-shoulder", "bone": "torso", "x": r(SH_V * s), "y": r(-SH_U * s), "kind": "support", "title": T("Shoulder blades on the wall", "Schulterblätter an der Wand"), "support": "contact", "normalDeg": 0, "footprintLengthM": 0.30, "footprintWidthM": 0.30, "frictionCoefficient": 0.4},
    {"id": "wall-seat", "bone": "pelvis", "x": r(-0.005 * s), "y": r(0.135 * s), "kind": "support", "title": T("Seat on the wall", "Gesäß an der Wand"), "support": "contact", "normalDeg": 0, "footprintLengthM": 0.20, "footprintWidthM": 0.30, "frictionCoefficient": 0.4},
    {"id": "phone-ear", "bone": "hand-r", "x": r(0.10 * s), "y": 0.02, "kind": "attachment", "title": T("Phone at the ear", "Telefon am Ohr"), "support": None} if False else {"id": "phone-ear", "bone": "hand-r", "x": r(0.10 * s), "y": 0.02, "kind": "attachment", "title": T("Phone at the ear", "Telefon am Ohr")},
    {"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")},
]
rig = Rig(bones, anchors)
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
Lleg = h.L["thigh"] + h.L["shank"]

# ---------------------------------------------------------------- geometry: hooded sweatshirt, jeans, trainers, beanie, wall
elev = h.standard(arm_sleeve="long", legs="long", shoe="sneaker", tones={"shirt": 0.34, "trousers": 0.30, "shoes": 0.9, "skin": 0.80}, hair="short", hair_tone=0.2)
byid = {x["id"]: x for x in elev}
elev = [x for x in elev if x["id"] not in ("torso",)]
elev += [
    h.sh("torso", "torso", "body", 0, 3.2, 0.34, g_path(up, table_cmds(TORSO, -0.14, 0.46, grow=0.010), True), "torso-clothing", "cotton-clothing"),
    h.sh("hoodie-hem", "torso", "detail", 2, 3.3, 0.2, g_poly(up, [(0.108, -0.14), (-0.108, -0.14)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("hoodie-pocket", "torso", "detail", 2, 3.3, 0.2, g_poly(up, [(0.128, 0.02), (0.05, -0.02), (0.03, 0.08)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("hoodie-hood", "head", "body", 1, 4.4, 0.38, g_path(up, [("M", 0.05, 0.0), ("C", 0.07, 0.04, 0.0, 0.085, -0.06, 0.08), ("C", -0.12, 0.075, -0.15, 0.03, -0.12, -0.02), ("C", -0.06, -0.03, 0.0, -0.02, 0.05, 0.0), ("Z",)], True), "torso-clothing", "cotton-clothing"),
    h.sh("hood-string", "head", "texture", 3, 4.45, 0.7, g_poly(up, [(0.05, 0.0), (0.06, -0.08), (0.055, -0.15)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("beanie", "head", "body", 1, 5.7, 0.3, g_path(up, [("M", -0.104, 0.19), ("C", -0.108, 0.27, -0.05, 0.305, 0.02, 0.303), ("C", 0.07, 0.30, 0.095, 0.27, 0.092, 0.215), ("L", 0.09, 0.20), ("C", 0.05, 0.215, -0.04, 0.205, -0.104, 0.175), ("Z",)], True), "hair", "wool-hat"),
    h.sh("beanie-rim", "head", "detail", 2, 5.75, 0.2, g_poly(up, [(0.09, 0.203), (0.04, 0.213), (-0.04, 0.203), (-0.104, 0.178)], False), "hair", "wool-hat", False),
    h.sh("jeans-seam-l", "thigh-l", "detail", 3, 1.1, 0.2, g_poly(down, [(0.098, 0.06), (0.074, 0.40)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("phone", "hand-r", "body", 1, 6.3, 0.12, g_rect(0.025, -0.045, 0.125, 0.032, 0.008), "phone-surface", "phone-glass"),
    h.sh("phone-screen", "hand-r", "glazing", 2, 6.35, 0.4, g_rect(0.032, -0.040, 0.105, 0.023, 0.005), "phone-surface", "phone-glass"),
    h.sh("wall-surface", "wall", "ground", 0, 0.1, 0.6, g_poly(Frame("fwd", 1, 1), [(0, 0), (0, 1.9)], False), "wall-face", "masonry", False),
    h.sh("wall-courses", "wall", "texture", 3, 0.2, 0.5, {"kind": "path", "d": "M -0.10 0.3 L 0 0.3 M -0.10 0.6 L 0 0.6 M -0.10 0.9 L 0 0.9 M -0.10 1.2 L 0 1.2 M -0.10 1.5 L 0 1.5 M -0.10 1.8 L 0 1.8"}, "wall-face", "masonry", False),
]

# ---------------------------------------------------------------- pose solver
def build(theta, back_gap, feet_x, torso_off=0.0, head_off=None, arms=None, H_frac=0.08, both=False, stand_dx=0.0):
    """Whole body rotated by theta (deg, CCW = leaning back) about the hips; the shoulder anchor sits back_gap from the wall (x = 0);
    the feet are flat at feet_x (ball x of left and right) corrected so that the moment of the wall reaction H_frac*W is balanced."""
    th = math.radians(theta)
    lean = {"torso": {"angleDeg": torso_off}, "head": {"angleDeg": head_off if head_off is not None else -torso_off - 0.6 * theta}}
    # shoulder anchor local -> world x for root (0, 0, theta)
    w0 = rig.solve(merge_offsets(lean, arms or {}), (0, 0, theta))
    ax = rig.anchor(w0, "wall-shoulder")[0]
    px = back_gap - ax
    fx = list(feet_x)
    for _ in range(8):
        offs = {}
        ankle_y = 0.08 * s
        hip = (px, 0.0)
        legs_off = {}
        ys = []
        for sd, bx in zip("lr", fx):
            ankle = (bx - 0.13 * s, ankle_y)
            dx = ankle[0] - px
            dy = math.sqrt(max(1e-6, (0.985 * Lleg) ** 2 - dx * dx))
            ys.append(ankle_y + dy)
        py = min(ys)
        for sd, bx in zip("lr", fx):
            ankle = (bx - 0.13 * s, ankle_y)
            t1, t2 = leg_ik((px, py), ankle, h.L["thigh"], h.L["shank"])
            legs_off[f"thigh-{sd}"] = {"angleDeg": r(norm(t1 + 90 - theta), 2)}
            legs_off[f"shank-{sd}"] = {"angleDeg": r(norm(t2 - t1), 2)}
            legs_off[f"foot-{sd}"] = {"angleDeg": r(norm(0 - t2 - 90), 2)}
        off = merge_offsets(lean, arms or {}, legs_off)
        cx, cy, m = com_x(rig, off, (px, py, theta))
        w = rig.solve(off, (px, py, theta))
        cop = 0.5 * (rig.anchor(w, "heel-r")[0] + rig.anchor(w, "contact-r")[0])
        ya = rig.anchor(w, "wall-shoulder")[1]
        target = cx + H_frac * ya
        shift = target - cop
        if abs(shift) < 1e-4: break
        fx = [b + shift for b in fx]
    off["wall-frame"] = {"angleDeg": r(-theta, 2)}
    off["wall-x"] = {"dx": r(-px, 4)}
    y = root_y(rig, off, FEET, root_angle=theta, root_x=px)
    off["wall"] = {"dy": r(-y, 4)}
    return off, px, y

pocket = {"upper-arm-l": {"angleDeg": -8}, "forearm-l": {"angleDeg": 38}, "hand-l": {"angleDeg": 0}}
hang_r = {"upper-arm-r": {"angleDeg": 4}, "forearm-r": {"angleDeg": 12}, "hand-r": {"angleDeg": 0}}
fold_arms = {"upper-arm-l": {"angleDeg": 18}, "forearm-l": {"angleDeg": 125}, "hand-l": {"angleDeg": -10}, "upper-arm-r": {"angleDeg": 24}, "forearm-r": {"angleDeg": 128}, "hand-r": {"angleDeg": -12}}
phone_arm = {"upper-arm-r": {"angleDeg": 38}, "forearm-r": {"angleDeg": 132}, "hand-r": {"angleDeg": 5}}

P = {}
P["shoulder"] = build(11, 0.0, (0.34, 0.31), arms=merge_offsets(pocket, hang_r), H_frac=0.08)
P["folded"] = build(4, 0.0, (0.24, 0.27), torso_off=2, arms=fold_arms, H_frac=0.05)
P["phone"] = build(9, 0.0, (0.30, 0.38), torso_off=0, head_off=-14, arms=merge_offsets(pocket, phone_arm), H_frac=0.07)
P["pushing"] = build(-2, 0.07, (0.20, 0.17), torso_off=-4, arms=merge_offsets({"upper-arm-l": {"angleDeg": -30}, "forearm-l": {"angleDeg": 20}}, {"upper-arm-r": {"angleDeg": -35}, "forearm-r": {"angleDeg": 25}}), H_frac=0.0)
P["away"] = build(0, 0.20, (0.12, 0.10), torso_off=0, arms={"upper-arm-l": {"angleDeg": 4}, "forearm-l": {"angleDeg": 8}, "upper-arm-r": {"angleDeg": 2}, "forearm-r": {"angleDeg": 6}}, H_frac=0.0)
CT = {"shoulder": FEET + ["wall-shoulder"], "folded": FEET + ["wall-shoulder", "wall-seat"], "phone": FEET + ["wall-shoulder"], "pushing": FEET + ["wall-shoulder"], "away": FEET}

def pz(id, en, de, den, dde, key_, met, br, key=False, wet=0.0):
    off, px, y = P[key_]
    th = -off["wall-frame"]["angleDeg"]
    return pose(id, en, de, den, dde, off, CT[key_], met, {"breath": br}, root={"x": r(px, 4), "angleDeg": th}, key=key)

poses = [
    pz("leaning-shoulder-wall", "Leaning on the wall with the shoulders", "Mit den Schultern an die Wand gelehnt", "The body rests 11 degrees backwards with the shoulder blades on the wall and the feet 0.3 m ahead; the wall presses horizontally and the feet resist by friction.", "Der Körper ruht 11 Grad nach hinten geneigt mit den Schulterblättern an der Wand und den Füßen 0,3 m vorn; die Wand drückt horizontal, die Füße halten durch Reibung.", "shoulder", 1.1, 0.5),
    pz("leaning-arms-folded", "Leaning with folded arms", "Angelehnt mit verschränkten Armen", "Back almost flat against the wall with shoulders and seat touching, arms folded across the chest.", "Rücken fast flach an der Wand mit Kontakt an Schultern und Gesäß, Arme vor der Brust verschränkt.", "folded", 1.0, 0.45),
    pz("phone-call", "Phone call", "Telefonat", "Leaning on the wall with one hand in the pocket and a phone held to the ear, head tilted towards the phone.", "An die Wand gelehnt, eine Hand in der Tasche und ein Telefon am Ohr, der Kopf zum Telefon geneigt.", "phone", 1.2, 0.6),
    pz("pushing-off", "Pushing off the wall", "Von der Wand abstoßen", "The feet move back under the body and the shoulders leave the wall; the wall reaction drops to zero.", "Die Füße rücken unter den Körper zurück und die Schultern lösen sich von der Wand; die Wandreaktion sinkt auf null.", "pushing", 1.8, 0.9),
    pz("standing-away", "Standing away from the wall", "Frei vor der Wand stehend", "Upright 0.2 m in front of the wall, weight over both feet.", "Aufrecht 0,2 m vor der Wand, das Gewicht über beiden Füßen.", "away", 1.3, 0.6),
]

ENV = {"airTempC": 18, "meanRadiantTempC": 17, "relativeHumidity": 0.5, "windMS": 0.3, "windDirectionDeg": 180, "solarWM2": 0, "pressurePa": 101325}
mats = std_materials() + [
    material("wool-hat", "Wool beanie", "Wollmütze", 0.93, "authored-estimate", None, 0.6),
    material("phone-glass", "Phone glass and aluminium", "Telefonglas und Aluminium", 0.85, "authored-estimate", None, 0.7, 2500, 1.0),
    material("masonry", "Brick wall", "Ziegelwand", 0.93, "literature", "en-1991-1-1", 0.7, 1800, 0.8),
]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.05, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair and beanie", "Haar und Mütze", "hair", "hair", 0.04 * A_BODY, 0.15, "core-resistance", 36.8, 0.14, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("torso-clothing", "Hooded sweatshirt", "Kapuzenpullover", "clothing", "cotton-clothing", 0.36 * A_BODY, 0.30, "core-resistance", 36.8, 0.17, 0.40, "literature", "iso-9920-2007"),
    reg("arm-clothing", "Long sleeves", "Lange Ärmel", "clothing", "cotton-clothing", 0.13 * A_BODY, 0.08, "core-resistance", 36.8, 0.15, 0.13, "literature", "iso-9920-2007"),
    reg("arm-skin", "Bare wrists", "Nackte Handgelenke", "skin", "skin", 0.005 * A_BODY, 0.05, "core-resistance", 36.8, 0.05, 0.0, "literature", "dubois-1916"),
    reg("hand-skin", "Hands", "Hände", "skin", "skin", 0.05 * A_BODY, 0.07, "core-resistance", 36.8, 0.05, 0.03, "literature", "dubois-1916", wet=0.06),
    reg("leg-clothing", "Jeans", "Jeans", "clothing", "cotton-clothing", 0.31 * A_BODY, 0.13, "core-resistance", 36.8, 0.15, 0.38, "literature", "iso-9920-2007"),
    reg("shoes", "Trainers", "Turnschuhe", "surface", "leather-shoe", 0.07 * A_BODY, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("phone-surface", "Phone", "Telefon", "surface", "phone-glass", 0.02, 0.14, "passive", None, None, None, "authored-estimate", "iso-9920-2007"),
    reg("wall-face", "Wall face", "Wandfläche", "wall", "masonry", 3.5, 2.0, "passive", None, None, None, "authored-estimate", "en-1991-1-1"),
]
elev = [x for x in elev if x["id"] not in ()]
emitters = [emitter("breath", "mouth", 4.0, vel=1.5, diam=0.03)]
# bare wrists use the skin region through a small shape
elev.append(h.sh("wrist-skin-r", "forearm-r", "detail", 3, 6.1, 0.8, g_poly(down, [(0.026, 0.25), (-0.024, 0.25)], False), "arm-skin", "skin", False))

SW = lambda *a: [pz_(*a)] if False else None
clips = [
    clip("phone-call-idle", "Idle phone call against the wall", "Untätiges Telefonieren an der Wand", "Talking on the phone: slow weight shifts, the head nods and the free arm sways, while the shoulders stay on the wall and breathing alternates between talking and listening.", "Telefonieren: langsame Gewichtsverlagerungen, der Kopf nickt und der freie Arm pendelt, während die Schultern an der Wand bleiben und die Atmung zwischen Sprechen und Zuhören wechselt.",
         6.0, True, "none",
         [key(0, "phone-call"), key(1.5, "phone-call", bones={"head": {"angleDeg": -9}, "upper-arm-r": {"angleDeg": 42}, "torso": {"angleDeg": 1.5}}), key(3.0, "phone-call", bones={"head": {"angleDeg": -18}, "forearm-l": {"angleDeg": 44}, "upper-arm-l": {"angleDeg": -12}}), key(4.5, "phone-call", bones={"head": {"angleDeg": -11}, "upper-arm-r": {"angleDeg": 36}}), key(6.0, "phone-call")],
         [span(a, 0, 6.0) for a in FEET + ["wall-shoulder"]],
         [phase("listen", 0, 3.0, "Listening", "Zuhören"), phase("talk", 3.0, 6.0, "Talking", "Sprechen")],
         [ev(0.5, "inhale", "Breath while listening", "Atmen beim Zuhören"), ev(3.2, "exhale", "Exhalation while talking", "Ausatmen beim Sprechen"), ev(3.0, "motion", "Nod", "Nicken")],
         [track("emitter:breath", [(0, 0.3), (1.5, 0.2), (3.0, 0.8), (4.5, 0.6), (6.0, 0.3)]), track("activityMet", [(0, 1.1), (3.0, 1.3), (6.0, 1.1)])]),
    clip("push-off", "Pushing off the wall", "Von der Wand abstoßen", "The person pushes the feet back under the body, the shoulders leave the wall at 1.4 s and the wall reaction vanishes; at the end the person stands upright 0.2 m in front of the wall.", "Die Person rückt die Füße unter den Körper, bei 1,4 s lösen sich die Schultern von der Wand und die Wandreaktion verschwindet; am Ende steht die Person aufrecht 0,2 m vor der Wand.",
         3.5, False, "none",
         [key(0, "leaning-shoulder-wall"), key(0.8, "leaning-shoulder-wall"), key(1.4, "pushing-off"), key(2.4, "standing-away"), key(3.5, "standing-away")],
         [span(a, 0, 3.5) for a in FEET] + [span("wall-shoulder", 0, 1.4)],
         [phase("lean", 0, 0.8, "Lean", "Anlehnen"), phase("push", 0.8, 1.4, "Push off", "Abstoßen"), phase("free", 1.4, 3.5, "Free standing", "Frei stehen")],
         [ev(1.4, "contact-off", "Shoulders leave the wall", "Schultern lösen sich von der Wand", "wall-shoulder"), ev(0.8, "inhale", "Breath before the push", "Atemzug vor dem Abstoßen"), ev(1.4, "exhale", "Exhalation on the push", "Ausatmen beim Abstoßen")],
         [track("activityMet", [(0, 1.1), (1.4, 2.0), (3.5, 1.3)]), track("emitter:breath", [(0, 0.4), (1.0, 0.5), (1.6, 1.0), (3.5, 0.5)])]),
]

def build_asset():
    return assemble("leaning-person", T("Leaning Person", "Angelehnte Person"),
        T("A person in a hooded sweatshirt and beanie leans back against a wall: shoulder and seat anchors take a horizontal wall reaction, the feet resist it by friction; idle phone-call and push-off clips show the reaction building and vanishing.",
          "Eine Person in Kapuzenpullover und Mütze lehnt sich an eine Wand: Schulter- und Gesäßanker nehmen eine horizontale Wandreaktion auf, die Füße halten sie durch Reibung; Clips für das Telefonieren im Stehen und das Abstoßen zeigen, wie die Reaktion entsteht und verschwindet."),
        ["human", "leaning", "wall", "support", "idle", "phone", "friction"], ["de-leva-1996", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ashrae-2021", "en-1991-1-1", "winter-2009"],
        T("The wall is a world-fixed rig branch (frame, x and y compensation bones) so it stays vertical while the body leans; the lean and feet positions are solved so that the moment of the wall reaction (8 % of the weight at 1.3 m height) balances the weight about the middle of the foot.", "Die Wand ist ein weltfester Rig-Zweig (Rahmen, x- und y-Ausgleichsknochen), sodass sie senkrecht bleibt, während sich der Körper neigt; Vorlage und Fußlage sind so gelöst, dass das Moment der Wandreaktion (8 % des Gewichts in 1,3 m Höhe) das Gewicht um die Fußmitte ausgleicht."),
        {"heightM": 1.72, "widthM": 0.6, "depthM": 0.45, "massKg": 66.0}, mats, bones, anchors, elev, poses, clips, regions, emitters,
        [T("Reference case 18 degrees C indoors; hooded sweatshirt and jeans about 0.9 clo (ISO 9920); metabolic rate 1.0 to 1.3 met for standing or leaning (ISO 8996 class 0 to 1).", "Referenzfall 18 Grad C im Innenraum; Kapuzenpullover und Jeans etwa 0,9 clo (ISO 9920); Stoffwechselrate 1,0 bis 1,3 met im Stehen oder Anlehnen (ISO 8996 Klasse 0 bis 1)."),
         T("The wall face is a passive masonry surface; contact conduction between back and wall is not modelled.", "Die Wandfläche ist eine passive Mauerwerksoberfläche; Wärmeleitung zwischen Rücken und Wand wird nicht modelliert.")],
        [T("The wall contacts are unilateral supports with normal 0 degrees (horizontal reaction, friction 0.4) at the shoulder blades and the seat; the equal-stiffness assumption distributes the horizontal force between wall and feet.", "Die Wandkontakte sind einseitige Auflager mit Normale 0 Grad (horizontale Reaktion, Reibung 0,4) an Schulterblättern und Gesäß; die Gleichsteifigkeitsannahme verteilt die horizontale Kraft auf Wand und Füße."),
         T("The wall bones have no mass; they only carry the drawing of the wall.", "Die Wandknochen haben keine Masse; sie tragen nur die Zeichnung der Wand.")],
        "leaning-shoulder-wall", "phone-call-idle", base_met=1.1, env=ENV)

if __name__ == "__main__":
    json.dump(build_asset(), open("out/leaning-person.json", "w"), ensure_ascii=False, indent=1)
    print("written", {k: (round(v[1], 3), round(v[2], 3)) for k, v in P.items()})
