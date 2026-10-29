import sys
from body import *
from gait import *

H, M, WF = 1.82, 72.0, 0.96
h = Human(H, M, WF)
s = h.s
A = dubois_m2(H, M)
bones = h.bones()
anchors = [dict(a, frictionCoefficient=0.8) if a["id"].startswith(("contact", "heel")) else a for a in h.anchors()] + [{"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")}]
rig = Rig(bones, anchors)
up, down, fwd = h.up, h.down, h.fwd

# ------------------------------------------------------------------------------------------------ geometry
elev = h.standard(arm_sleeve="short", legs="short", shoe="runner", tones={"shirt": 0.80, "trousers": 0.22, "shoes": 0.92, "skin": 0.80}, hair="tied", leg_hem=0.20, hair_tone=0.15)
elev += [
    h.sh("headband", "head", "detail", 1, 5.5, 0.9, g_poly(up, [(-0.099, 0.205), (-0.04, 0.236), (0.03, 0.242), (0.087, 0.226)], False), "hair", "hair", False),
    h.sh("shirt-collar", "torso", "detail", 2, 3.3, 0.3, g_poly(up, [(0.058, 0.46), (0.045, 0.43), (-0.02, 0.42), (-0.066, 0.46)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("shirt-hem", "torso", "detail", 2, 3.3, 0.3, g_poly(up, [(0.108, -0.055), (-0.106, -0.055)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("shirt-stripe", "torso", "texture", 3, 3.3, 0.55, g_poly(up, [(0.13, 0.30), (-0.125, 0.30)], False), "torso-clothing", "cotton-clothing", False),
    h.sh("shirt-number", "torso", "texture", 3, 3.3, 0.4, g_rect(-0.01, 0.17, 0.0, 0.0) if False else g_poly(up, [(0.112, 0.20), (0.112, 0.27), (0.07, 0.27), (0.07, 0.20)], True), "torso-clothing", "cotton-clothing", False),
    h.sh("shorts-stripe-r", "thigh-r", "detail", 2, 2.1, 0.75, g_poly(down, [(0.0, 0.0), (0.0, 0.19)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("shorts-hem-r", "thigh-r", "detail", 2, 2.1, 0.3, g_poly(down, [(0.082, 0.20), (-0.085, 0.20)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("shorts-hem-l", "thigh-l", "detail", 2, 1.1, 0.3, g_poly(down, [(0.082, 0.20), (-0.085, 0.20)], False), "leg-clothing", "cotton-clothing", False),
    h.sh("armband", "upper-arm-l", "body", 1, 0.42, 0.25, g_path(down, table_cmds(UPPER_ARM, 0.07, 0.17, grow=0.004), True), "arm-clothing", "cotton-clothing"),
    h.sh("sock-r", "shank-r", "body", 1, 2.05, 0.95, g_path(down, table_cmds(SHANK, 0.33, 0.42), True), "leg-skin", "skin"),
    h.sh("sock-l", "shank-l", "body", 1, 1.05, 0.95, g_path(down, table_cmds(SHANK, 0.33, 0.42), True), "leg-skin", "skin"),
    h.sh("lace-r", "foot-r", "texture", 3, 2.3, 0.4, g_poly(fwd, [(0.04, 0.03), (0.07, 0.014), (0.10, 0.0)], False), "shoes", "leather-shoe", False),
    h.sh("lace-l", "foot-l", "texture", 3, 1.3, 0.4, g_poly(fwd, [(0.04, 0.03), (0.07, 0.014), (0.10, 0.0)], False), "shoes", "leather-shoe", False),
]
# socks use the skin material at a lighter tone: they belong to the leg-skin region but are drawn as fabric
for x in elev:
    if x["id"].startswith("sock"): x["material"] = "cotton-clothing"

# ------------------------------------------------------------------------------------------------ run gait
T_RUN, SIGMA = 0.72, 1 / 3
DIP_SCALE = 1.34
ADV = 1.87
Lfull = h.L["thigh"] + h.L["shank"] + 0.08 * s
GK = gait_keys(h, "run", ADV, SIGMA, 0.94 * Lfull, 0.0, 35, 0.30, n=12, stance_dip=DIP_SCALE * 9.80665 * T_RUN ** 2 / (16 * math.pi ** 2))
RUN_NAMES = []
for side in "lr":
    o = "r" if side == "l" else "l"
    RUN_NAMES += [(f"initial-contact-{side}", f"Initial contact, {'left' if side == 'l' else 'right'}", f"Erstkontakt {'links' if side == 'l' else 'rechts'}", f"{'Left' if side == 'l' else 'Right'} foot lands flat under a forward-leaning trunk; the other leg is behind and still extended.", f"Der {'linke' if side == 'l' else 'rechte'} Fuß setzt flach unter dem vorgeneigten Rumpf auf; das andere Bein ist hinten und noch gestreckt."),
                  (f"loading-{side}", f"Loading, {'left' if side == 'l' else 'right'}", f"Belastung {'links' if side == 'l' else 'rechts'}", "Knee flexes while the body weight is absorbed; the other knee swings forward.", "Das Knie beugt sich, während das Körpergewicht abgefangen wird; das andere Knie schwingt nach vorn."),
                  (f"midstance-{side}", f"Midstance, {'left' if side == 'l' else 'right'}", f"Mittlere Standphase {'links' if side == 'l' else 'rechts'}", "Lowest hip position with the full body weight over a flexed leg.", "Tiefste Hüftlage mit dem ganzen Körpergewicht über einem gebeugten Bein."),
                  (f"propulsion-{side}", f"Propulsion, {'left' if side == 'l' else 'right'}", f"Abdruckphase {'links' if side == 'l' else 'rechts'}", "Heel rises, the leg extends and pushes the body forward and up.", "Die Ferse hebt sich, das Bein streckt sich und schiebt den Körper nach vorn und oben."),
                  (f"toe-off-{side}", f"Toe-off, {'left' if side == 'l' else 'right'}", f"Abdruck {'links' if side == 'l' else 'rechts'}", "Last instant of ground contact before flight.", "Letzter Moment des Bodenkontakts vor der Flugphase."),
                  (f"flight-{side}", f"Flight after {'left' if side == 'l' else 'right'} toe-off", f"Flugphase nach Abdruck {'links' if side == 'l' else 'rechts'}", "Both feet are off the ground: the body flies ballistically for about 0.12 s.", "Beide Füße sind in der Luft: der Körper fliegt etwa 0,12 s ballistisch.")]
RUN_CONTACTS = [["heel-l", "contact-l"], ["heel-l", "contact-l"], ["heel-l", "contact-l"], ["contact-l"], ["contact-l"], []] + [["heel-r", "contact-r"], ["heel-r", "contact-r"], ["heel-r", "contact-r"], ["contact-r"], ["contact-r"], []]

def run_pose_offsets(k):
    th = {"l": k["thigh-l"]["angleDeg"], "r": k["thigh-r"]["angleDeg"]}
    arms = {}
    for sd in "lr":
        arms[f"upper-arm-{sd}"] = {"angleDeg": r(max(-55, min(60, 8 - 0.85 * th[sd])), 1)}
        arms[f"forearm-{sd}"] = {"angleDeg": 85}
        arms[f"hand-{sd}"] = {"angleDeg": 5}
    return merge_offsets(leg_only(k), {"torso": {"angleDeg": -9}, "head": {"angleDeg": 9}}, arms)

def run_pose(i):
    k = GK[i]
    nid, en, de, den, dde = RUN_NAMES[i]
    sweat = {"head-skin": {"wettedness": 0.3}, "arm-skin": {"wettedness": 0.3}, "leg-skin": {"wettedness": 0.3}, "hand-skin": {"wettedness": 0.25}, "torso-clothing": {"wettedness": 0.2}}
    return pose(nid, en, de, den, dde, run_pose_offsets(k), RUN_CONTACTS[i], 8.0, {"breath": 1.0}, root={"y": r(k["hipY"], 4)}, regions=sweat, key=nid not in ("initial-contact-l", "midstance-l", "flight-l"))

# ------------------------------------------------------------------------------------------------ stretching
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]
def tune_com(off, contacts, bone, lo, hi, side_keys):
    """Bisect the hip-extension offset `bone` until the centre of mass sits above the middle of the support."""
    def f(v):
        o = merge_offsets(off, {bone: {"angleDeg": v}})
        y = root_y(rig, o, contacts)
        w = rig.solve(o, (0, y, 0))
        xs = [rig.anchor(w, c)[0] for c in contacts]
        return com_x(rig, o, (0, y, 0))[0] - sum(xs) / len(xs)
    a, b = lo, hi
    for _ in range(40):
        m = (a + b) / 2
        if f(a) * f(m) <= 0: b = m
        else: a = m
    return round((a + b) / 2, 2)

hands_hip = {"upper-arm-l": {"angleDeg": -12}, "forearm-l": {"angleDeg": 105}, "hand-l": {"angleDeg": 10}, "upper-arm-r": {"angleDeg": -12}, "forearm-r": {"angleDeg": 105}, "hand-r": {"angleDeg": 10}}
stand_off = merge_offsets({"torso": {"angleDeg": 2}, "head": {"angleDeg": -2}}, hands_hip)

# quad stretch: stand on the left leg, right heel to the buttock held by the right hand
quad = merge_offsets({"thigh-l": {"angleDeg": 0}, "shank-l": {"angleDeg": -4}, "foot-l": {"angleDeg": 4}, "thigh-r": {"angleDeg": -8}, "shank-r": {"angleDeg": -142}, "foot-r": {"angleDeg": -25}, "torso": {"angleDeg": 3}, "head": {"angleDeg": -3},
                      "upper-arm-l": {"angleDeg": 75}, "forearm-l": {"angleDeg": 15}, "hand-l": {"angleDeg": 0}})
y = root_y(rig, quad, ["heel-l", "contact-l"]); w = rig.solve(quad, (0, y, 0))
ankle_r = rig.origin(w, "foot-r")
grip = (ankle_r[0] + 0.0, ankle_r[1] + 0.02)
sh = rig.origin(w, "upper-arm-r")
wr = (grip[0] - 0.075 * s * math.cos(math.radians(-60)), grip[1] - 0.075 * s * math.sin(math.radians(-60)))
quad = merge_offsets(quad, arm_offsets(rig, w, "r", wr, -60, bend=1))
quad["torso"] = {"angleDeg": tune_com(quad, ["heel-l", "contact-l"], "torso", -25, 25, None) if False else quad["torso"]["angleDeg"]}

# hamstring fold: legs straight, trunk folded, hands to the shins
fold = merge_offsets({"thigh-l": {"angleDeg": 0}, "thigh-r": {"angleDeg": 0}, "shank-l": {"angleDeg": -2}, "shank-r": {"angleDeg": -2}, "torso": {"angleDeg": -80}, "head": {"angleDeg": 30}})
fold["pelvis"] = {"angleDeg": -30}
for sd in "lr": fold[f"thigh-{sd}"] = {"angleDeg": 30}
for sd in "lr": fold[f"foot-{sd}"] = {"angleDeg": 0}
y = root_y(rig, fold, FEET); w = rig.solve(fold, (0, y, 0))
tx = rig.origin(w, "shank-r")
for sd in "lr":
    fold = merge_offsets(fold, arm_offsets(rig, w, sd, (tx[0] + 0.02, tx[1] - 0.12), -80, bend=1))
fold["pelvis"] = {"angleDeg": tune_com(fold, FEET, "pelvis", -60, 25, None)}

def stretch_pose(id, en, de, den, dde, off, contacts, met, br):
    return pose(id, en, de, den, dde, off, contacts, met, {"breath": br}, regions={"head-skin": {"wettedness": 0.2}, "arm-skin": {"wettedness": 0.2}, "leg-skin": {"wettedness": 0.2}, "torso-clothing": {"wettedness": 0.15}})

# ------------------------------------------------------------------------------------------------ thermal
mats = std_materials() + [material("nylon-clothing", "Technical running fabric (polyester)", "Technisches Laufgewebe (Polyester)", 0.93, "literature", "iso-9920-2007", 0.45)]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A, 0.15, "core-resistance", 36.9, 0.04, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair and headband", "Haar und Stirnband", "hair", "hair", 0.03 * A, 0.15, "core-resistance", 36.9, 0.08, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("torso-clothing", "Running shirt", "Laufshirt", "clothing", "nylon-clothing", 0.36 * A, 0.30, "core-resistance", 36.9, 0.10, 0.40, "literature", "iso-9920-2007", wet=0.0),
    reg("arm-clothing", "Short sleeves and armband", "Kurze Ärmel und Armband", "clothing", "nylon-clothing", 0.035 * A, 0.08, "core-resistance", 36.9, 0.09, 0.04),
    reg("arm-skin", "Bare arms", "Nackte Arme", "skin", "skin", 0.095 * A, 0.08, "core-resistance", 36.9, 0.04, 0.09, "literature", "dubois-1916", wet=0.06),
    reg("hand-skin", "Hands", "Hände", "skin", "skin", 0.05 * A, 0.07, "core-resistance", 36.9, 0.04, 0.03, "literature", "dubois-1916", wet=0.06),
    reg("leg-clothing", "Running shorts", "Laufshorts", "clothing", "nylon-clothing", 0.10 * A, 0.15, "core-resistance", 36.9, 0.08, 0.12),
    reg("leg-skin", "Bare legs and socks", "Nackte Beine und Socken", "skin", "skin", 0.22 * A, 0.12, "core-resistance", 36.9, 0.04, 0.26, "literature", "dubois-1916", wet=0.06),
    reg("shoes", "Running shoes", "Laufschuhe", "surface", "leather-shoe", 0.07 * A, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
]
emitters = [emitter("breath", "mouth", 1.5, vel=3.0, diam=0.04, dfrom=0.45, dto=1.0)]

poses = [run_pose(i) for i in range(12)]
poses += [
    stretch_pose("stretch-stand", "Standing, hands on hips", "Stehen, Hände in den Hüften", "Upright after the run, hands on the hips, breathing recovering.", "Aufrecht nach dem Lauf, Hände in den Hüften, die Atmung beruhigt sich.", stand_off, FEET, 2.6, 0.8),
    stretch_pose("stretch-quad-r", "Quadriceps stretch, right", "Dehnung des Oberschenkelstreckers rechts", "Standing on the left leg, the right heel pulled to the buttock with the right hand; the left arm balances.", "Stand auf dem linken Bein, die rechte Ferse mit der rechten Hand zum Gesäß gezogen; der linke Arm balanciert.", quad, ["heel-l", "contact-l"], 2.2, 0.6),
    stretch_pose("stretch-hamstring", "Hamstring fold", "Dehnung der Beinbeuger", "Legs straight, trunk folded forward, hands on the shins; the hips move back to keep the centre of mass over the feet.", "Beine gestreckt, Rumpf nach vorn gefaltet, Hände an den Schienbeinen; die Hüften wandern nach hinten, damit der Schwerpunkt über den Füßen bleibt.", fold, FEET, 2.0, 0.5),
]
TT = T_RUN
run_keys = [key(round(TT * i / 12, 4), RUN_NAMES[i][0]) for i in range(12)]
run_spans = [span("heel-l", 0, 0.35 * SIGMA * TT), span("contact-l", 0, SIGMA * TT), span("heel-r", 0.5 * TT, 0.5 * TT + 0.35 * SIGMA * TT), span("contact-r", 0.5 * TT, 0.5 * TT + SIGMA * TT)]
ADVANCE = float(sys.argv[1]) if len(sys.argv) > 1 else 1.69495154
clips = [
    clip("run-cycle", "Running cycle", "Laufzyklus", "Jogging at about 2.6 m/s with a step rate of 2.8 Hz: each foot is on the ground for a third of the cycle and the body flies for two phases of about 0.12 s per cycle; feet are planted while in contact.", "Joggen mit etwa 2,6 m/s bei 2,8 Schritten pro Sekunde: jeder Fuß hat ein Drittel des Zyklus Bodenkontakt, und der Körper fliegt in zwei Phasen von etwa 0,12 s pro Zyklus; die Füße gleiten im Kontakt nicht.",
         TT, True, "plant", run_keys, run_spans,
         [phase("stance-l", 0, SIGMA * TT, "Left stance", "Linke Standphase"), phase("flight-1", SIGMA * TT, 0.5 * TT, "Flight", "Flugphase", True), phase("stance-r", 0.5 * TT, (0.5 + SIGMA) * TT, "Right stance", "Rechte Standphase"), phase("flight-2", (0.5 + SIGMA) * TT, TT, "Flight", "Flugphase", True)],
         [ev(0, "contact-on", "Left initial contact", "Linker Erstkontakt", "heel-l"), ev(SIGMA * TT, "contact-off", "Left toe-off, flight starts", "Linker Abdruck, Flugphase beginnt", "contact-l"), ev(0.5 * TT, "contact-on", "Right initial contact", "Rechter Erstkontakt", "heel-r"), ev((0.5 + SIGMA) * TT, "contact-off", "Right toe-off, flight starts", "Rechter Abdruck, Flugphase beginnt", "contact-r"), ev(0, "exhale", "Exhalation on the left foot strike", "Ausatmen beim linken Fußaufsatz"), ev(0.75 * TT, "inhale", "Inhalation", "Einatmen")],
         [track("emitter:breath", [(0, 1.0), (0.25 * TT, 0.4), (0.5 * TT, 0.2), (0.75 * TT, 0.8), (TT, 1.0)])], advance=ADVANCE),
    clip("stretch", "Stretching after the run", "Dehnen nach dem Lauf", "Standing recovery with a held quadriceps stretch on the left leg and a forward hamstring fold; metabolic rate and breathing settle from 4 to 2 met.", "Erholung im Stand mit gehaltener Dehnung des Oberschenkelstreckers auf dem linken Bein und einer Vorbeuge für die Beinbeuger; Stoffwechsel und Atmung beruhigen sich von 4 auf 2 met.",
         12.0, False, "none",
         [key(0, "stretch-stand"), key(1.5, "stretch-stand"), key(3.5, "stretch-quad-r"), key(6.5, "stretch-quad-r"), key(8.0, "stretch-stand"), key(10.0, "stretch-hamstring"), key(12.0, "stretch-hamstring")],
         [span("heel-l", 0, 12), span("contact-l", 0, 12), span("heel-r", 0, 3.0), span("contact-r", 0, 3.0), span("heel-r", 7.2, 12), span("contact-r", 7.2, 12)],
         [phase("recover", 0, 1.5, "Recover", "Erholen"), phase("quad", 1.5, 8.0, "Quadriceps stretch", "Dehnung Oberschenkelstrecker"), phase("hamstring", 8.0, 12.0, "Hamstring fold", "Dehnung Beinbeuger")],
         [ev(0.5, "exhale", "Deep recovery breath", "Tiefer Atemzug zur Erholung"), ev(3.0, "contact-off", "Right foot leaves the ground", "Rechter Fuß verlässt den Boden", "contact-r"), ev(7.2, "contact-on", "Right foot back on the ground", "Rechter Fuß wieder am Boden", "contact-r"), ev(10.0, "motion", "Fold reaches the shins", "Vorbeuge erreicht die Schienbeine")],
         [track("activityMet", [(0, 4.0), (3, 2.8), (8, 2.2), (12, 2.0)]), track("emitter:breath", [(0, 1.0), (3, 0.7), (6, 0.5), (12, 0.4)])]),
]

def build():
    return assemble("jogger", T("Jogger", "Jogger:in"),
        T("A 1.82 m athletic jogger in running shirt and shorts: forward trunk lean, arm drive, and two flight phases per cycle at about 8 met with fast breathing and sweating skin; a stretching clip follows the run.",
          "Eine 1,82 m große, sportliche Joggerin bzw. ein Jogger in Laufshirt und Shorts: Vorlage des Rumpfes, Armschwung und zwei Flugphasen pro Zyklus bei etwa 8 met mit schneller Atmung und schwitzender Haut; ein Dehn-Clip schließt sich an den Lauf an."),
        ["human", "runner", "jogger", "sport", "flight-phase", "stretching"], ["de-leva-1996", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ainsworth-2011", "ashrae-2021", "cavanagh-1982", "mcmahon-1986"],
        T("Run key frames are derived from a planted-foot inverse-kinematics gait (hip path, stance and swing foot paths) so planted contacts stay fixed; silhouette handcrafted; 72 kg, 1.82 m.", "Lauf-Schlüsselbilder aus einer Inverskinematik mit festen Standfüßen (Hüftbahn, Stand- und Schwungfußbahnen), damit Kontakte nicht gleiten; Silhouette von Hand gezeichnet; 72 kg, 1,82 m."),
        {"heightM": 1.82, "widthM": 1.04, "depthM": 0.45, "massKg": 72.0}, mats, bones, anchors, elev, poses, clips, regions, emitters,
        [T("DuBois area for 1.82 m and 72 kg; bare arms and legs and a light technical shirt (about 0.3 clo, ISO 9920) give the high heat loss of runners.", "DuBois-Fläche für 1,82 m und 72 kg; nackte Arme und Beine und ein leichtes Funktionsshirt (etwa 0,3 clo, ISO 9920) ergeben den hohen Wärmeverlust beim Laufen."),
         T("Metabolic rate 8 met while running at 2.6 m/s (Compendium: running 5 mph 8.3 met, jogging 7 met; ISO 8996 class 4), 2 to 4 met while stretching; skin wettedness 0.3 while running (ASHRAE).", "Stoffwechselrate 8 met beim Laufen mit 2,6 m/s (Compendium: Laufen 5 mph 8,3 met, Joggen 7 met; ISO 8996 Klasse 4), 2 bis 4 met beim Dehnen; Hautbenetzung 0,3 beim Laufen (ASHRAE)."),
         T("Breathing at 40 breaths per minute (period 1.5 s) with a stronger, faster jet than at rest.", "Atmung mit 40 Atemzügen pro Minute (Periode 1,5 s) mit stärkerem, schnellerem Strahl als in Ruhe.")],
        [T("Inertial model: segment masses as point masses at de Leva centres; the ground contact force reaches about 2 body weights at midstance (Cavanagh and Lafortune: 2.0-2.5 body weights).", "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva; die Bodenreaktionskraft erreicht in der mittleren Standphase etwa das Doppelte des Körpergewichts (Cavanagh und Lafortune: 2,0-2,5 Körpergewichte)."),
         T("In the flight phases no contact is active, so the structural solution is the free flight with zero reactions and the d'Alembert residual only.", "In den Flugphasen ist kein Kontakt aktiv, die Tragwerkslösung ist daher der freie Flug mit Reaktionen null und nur dem d'Alembert-Rest.")],
        "initial-contact-l", "run-cycle", base_met=1.5)

if __name__ == "__main__":
    json.dump(build(), open("out/jogger.json", "w"), ensure_ascii=False, indent=1)
    print("written")
