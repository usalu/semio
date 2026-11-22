"""☕️ barista: a 1.70 m barista in a charcoal long-sleeve top, a waxed-canvas bib apron, dark jeans and white canvas sneakers, hair in a
bun, pouring steamed milk from a stainless jug into a cappuccino cup she holds tilted in the other hand. Derived from the archive's
standing adult (rig, outlines and volumes scaled by 1.70/1.69; a factor below 1 would push the solids that sit exactly on the smallest feature of their scale under it); garments, jug, cup, poses and clips are authored here.

python3 barista.py [--stage]   writes 🗂️catalog/👤️humans/barista/🔣️.json (or a scratch copy with --stage)
"""
import copy, json, math, sys
import lib, figure, rot, ik
from lib import T, SCALES, VIEWS

SLUG = "barista"
STATURE, STATURE_BASE = 1.70, 1.69
K = STATURE / STATURE_BASE
SEX, AGE, BODY_KG, CLOTHES_KG = "female", 26, 61.0, 1.9

doc = lib.load_base()
a = doc["asset"]
a["id"] = SLUG

# ------------------------------------------------------------------ garments
lib.strip_hoodie(a)
figure.drop_hem_and_hood_collar(a)
figure.torso_from_loft(a)
figure.dress(
    a,
    {"top": ("hoodie", lambda b: True), "jeans": ("jeans", lambda b: True), "sneakers": ("sneakers", lambda b: True)},
    [
        (lambda s: s.get("region") == "top" or s.get("material") == "fleece-sweatshirt", 0.25),
        (lambda s: s.get("region") == "jeans" or s.get("material") == "denim", 0.38),
        (lambda s: s.get("region") == "hair" or s.get("material") == "hair", 0.12),
        (lambda s: s.get("id", "").startswith(("shoe-", "sil-shoe-")) and "sole" not in s.get("id", ""), 0.92),
        (lambda s: s.get("id", "").startswith(("toe-cap-", "sole-", "toe-sole-")), 0.8),
        (lambda s: s.get("id", "").startswith("lace-"), 0.55),
    ],
    {("top", "tee"): "camisole", ("top", "sweatshirt"): "top"},
    keep_hatch=("denim",),
)
figure.drop_thermal_solids(a, {"th-tee-sleeve-l", "th-tee-sleeve-r"})
for sc in SCALES:
    for view in VIEWS:
        lib.shapes(a, sc, view)[:] = [s for s in lib.shapes(a, sc, view) if not s["id"].startswith(("jeans-knee",))]
lib.rename_ids(a, lambda i: i.replace("hoodie", "top").replace("cuff-rib", "top-cuff").replace("collar-rib", "top-neckline"))
lib.set_material(a, "fleece-sweatshirt", "cotton-jersey")
lib.revolve_limbs(a, {"arm-l", "forearm-l", "arm-r", "forearm-r"})
# the sleeve lofts of the volume are oval and sit towards the trunk of the arm axis; with the elbows turned out they bound the figure
# laterally, so their bodies of revolution are drawn 10 % slimmer and shifted inwards (axisZ 4 mm, measured against the volume in the pouring pose) and the lateral extent of the drawings stays with the volume
for sc in SCALES:
    for view in VIEWS:
        for sh in lib.shapes(a, sc, view):
            if sh["bone"] in ("arm-l", "arm-r") and (sh.get("solid") or {}).get("kind") == "revolved":
                sh["points"] = [[u, lib.r5(v * 0.9)] for u, v in sh["points"]]
                sh["solid"]["axisZ"] = -0.004 if sh["bone"] == "arm-l" else 0.004

# ------------------------------------------------------------------ bib apron: bib on the chest, two skirt panels over the thighs blended with the pelvis
BIB = {"bottom": -0.205, "top": 0.215, "front": -0.165, "back": -0.06, "half": 0.125}
for sc in SCALES:
    lib.extrude_part(a, sc, "apron-bib", "torso", [[BIB["bottom"], BIB["back"]], [BIB["bottom"], BIB["front"]], [BIB["top"], BIB["front"] + 0.03], [BIB["top"], BIB["back"]]],
                     -BIB["half"], BIB["half"], 0.0, "waxed-canvas", "apron", 0.55, 4.6, structure=False,
                     weights=[{"bone": "torso", "weight": 0.6}, {"bone": "lumbar", "weight": 0.4}])
SKIRT = {"top": -0.10, "bottom": 0.43, "front": 0.104, "back": 0.0, "inner": 0.085, "outer": 0.072}
for side, sg, zpaint in (("l", 1, 1.5), ("r", -1, 3.5)):
    w0, w1 = (-SKIRT["inner"], SKIRT["outer"]) if sg > 0 else (-SKIRT["outer"], SKIRT["inner"])
    prof = [[SKIRT["top"], SKIRT["back"]], [SKIRT["top"], SKIRT["front"]], [SKIRT["bottom"], SKIRT["front"]], [SKIRT["bottom"], SKIRT["back"]]]
    for sc in SCALES:
        lib.extrude_part(a, sc, f"apron-{side}", f"thigh-{side}", prof, w0, w1, sg * 0.085, "waxed-canvas", "apron", 0.55, zpaint, structure=False,
                         weights=[{"bone": "pelvis", "weight": 0.6}, {"bone": f"thigh-{side}", "weight": 0.4}])
# neck strap, pocket and seams of the apron (front view, rest coordinates)
for sc, parts in (("50", ("strap", "pocket", "pen")), ("100", ("strap", "pocket")), ("200", ("strap",))):
    f = lib.shapes(a, sc, "front")
    if "strap" in parts:
        for side, sg in (("l", 1), ("r", -1)):
            f.append({"id": f"apron-strap-{side}", "bone": "torso", "role": "detail", "z": 4.95, "filled": False, "tone": 0.3, "region": "apron", "kind": "poly",
                      "points": [[sg * 0.118, 1.382], [sg * 0.085, 1.425], [sg * 0.06, 1.452]], "closed": False})
    if "pocket" in parts:
        f.append({"id": "apron-pocket", "bone": "torso", "role": "detail", "z": 4.96, "filled": False, "tone": 0.3, "region": "apron", "kind": "poly",
                  "points": [[-0.075, 1.255], [-0.075, 1.17], [0.075, 1.17], [0.075, 1.255]], "closed": False})
    if "pen" in parts:
        f.append({"id": "apron-pocket-seam", "bone": "torso", "role": "texture", "z": 4.97, "filled": False, "tone": 0.3, "region": "apron", "kind": "poly",
                  "points": [[0.0, 1.17], [0.0, 1.255]], "closed": False})

# ------------------------------------------------------------------ skeleton detail without the hood
t1 = lib.solid(a, "50", "structure", "t1-spinous")
t1["p1"] = [round(t1["p0"][i] + 0.55 * (t1["p1"][i] - t1["p0"][i]), 5) for i in range(3)]

# ------------------------------------------------------------------ person
lib.scale_asset(a, K)
lib.set_masses(a, SEX, BODY_KG + CLOTHES_KG)
for b in a["rig"]["bones"]:
    b["title"] = {"en": b["title"]["en"].replace("with sweatshirt", "with top and bib apron"), "de": b["title"]["de"].replace("mit Sweatshirt", "mit Oberteil und Latzschürze")}

resp = a["thermal"]["respiration"]
resp["patterns"] = [
    {"id": "quiet-standing", "title": T("Quiet standing", "Ruhiges Stehen"), "rateBpm": 14, "tidalVolumeL": 0.42, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
    {"id": "concentrated-pouring", "title": T("Concentrated pouring, breath held short", "Konzentriertes Gießen, kurz gehaltener Atem"), "rateBpm": 12, "tidalVolumeL": 0.5, "inhaleFraction": 0.34, "pauseAfterInhaleFraction": 0.16, "exhaleFraction": 0.4, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
    {"id": "talking", "title": T("Talking to the guest", "Sprechen mit dem Gast"), "rateBpm": 16, "tidalVolumeL": 0.55, "inhaleFraction": 0.24, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.69, "pauseAfterExhaleFraction": 0.05, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
]
resp["motion"] = [{"bone": "torso", "angleDeg": 0.5, "dy": lib.r5(0.003 * K)}]


# ------------------------------------------------------------------ poses: trunk bowed over the cup, arms by inverse kinematics to the jug handle and the cup
base_pose = next(p for p in a["poses"] if p["id"] == "relaxed")
relaxed_bones = base_pose["bones"]
LA, LF = 0.314 * K, 0.247 * K
PALM_U, PALM_W = 0.058 * K, 0.012 * K
TRUNK_POUR = {"torso": {"angleDeg": -6.0}, "lumbar": {"angleDeg": -3.0}, "neck": {"angleDeg": -12}, "head": {"angleDeg": -16}}


def arm_rest(trunk):
    probe = copy.deepcopy(doc)
    pp = next(p for p in probe["asset"]["poses"] if p["id"] == "relaxed")
    pp["bones"].update(trunk)
    for side in "lr":
        for b in (f"arm-{side}", f"forearm-{side}", f"hand-{side}"):
            pp["bones"].pop(b, None)
    F = lib.frames(lib.write_tmp(probe, "t-barista-probe.json"), "relaxed")
    return {side: (rot.frame(F[f"arm-{side}"]), [F[f"arm-{side}"]["o"][c] for c in "xyz"]) for side in "lr"}


def hand_target(side, palm, u, palm_normal):
    """Hand origin and frame for a palm centre `palm` (world), finger direction u and outward palm normal (left palm = -w, right palm = +w)."""
    w = [-c for c in palm_normal] if side == "l" else list(palm_normal)
    Rt = ik.look_at(u, w)
    o = [palm[i] - Rt[i][0] * PALM_U - Rt[i][2] * (-PALM_W if side == "l" else PALM_W) for i in range(3)]
    return Rt, o


def solve_arms(trunk, cup_c, cup_axis, jug_b, jug_axis, jug_spout, cup_r=0.046, jug_r=0.043):
    rest = arm_rest(trunk)
    # left palm against the far side of the cup body, fingers forward round it
    side_dir = [0, -cup_axis[2], cup_axis[1]]
    palm_l = [cup_c[i] + cup_axis[i] * 0.035 + side_dir[i] * (cup_r + 0.012) for i in range(3)]
    Rl, ol = hand_target("l", palm_l, [1, 0, -0.15], [-c for c in side_dir])
    off_l, err_l = ik.solve(rest["l"][0], rest["l"][1], LA, LF, Rl, ol, "l")
    # right palm on the handle behind the jug (opposite the spout)
    grip = [jug_b[i] + jug_axis[i] * 0.06 - jug_spout[i] * (jug_r + 0.026) for i in range(3)]
    Rr, orr = hand_target("r", grip, [1, 0, 0.1], [c for c in jug_spout])
    off_r, err_r = ik.solve(rest["r"][0], rest["r"][1], LA, LF, Rr, orr, "r")
    return {**off_l, **off_r}, (err_l, err_r)


F = K / (1.63 / 1.69)
CUP_C = [0.30 * F, 1.0 * F, 0.035 * F]
CUP_AX = [0, math.cos(math.radians(15)), -math.sin(math.radians(15))]
JUG_B = [0.29 * F, 1.055 * F, -0.075 * F]
JUG_AX = [0, math.cos(math.radians(40)), math.sin(math.radians(40))]
JUG_SP = [0, -math.sin(math.radians(40)), math.cos(math.radians(40))]
POUR_ARMS, POUR_ERR = solve_arms(TRUNK_POUR, CUP_C, CUP_AX, JUG_B, JUG_AX, JUG_SP)


TRUNK_STEAM = {"torso": {"angleDeg": -2.0}, "lumbar": {"angleDeg": -1.0}, "neck": {"angleDeg": -10}, "head": {"angleDeg": -14}}
TRUNK_SERVE = {"torso": {"angleDeg": -4.0}, "lumbar": {"angleDeg": -2.0}, "neck": {"angleDeg": -2}, "head": {"angleDeg": 2}}
UP = [0, 1, 0]
# steaming: jug upright at chest height, the left palm flat against its side to feel the temperature
STEAM_ARMS, STEAM_ERR = solve_arms(TRUNK_STEAM, [0.27 * F, 1.13 * F, -0.03 * F], UP, [0.27 * F, 1.13 * F, -0.03 * F], UP, [0, 0, 1], cup_r=0.043)
# serving: the cup held out over the counter, the jug resting upright in the right hand beside the hip
SERVE_ARMS, SERVE_ERR = solve_arms(TRUNK_SERVE, [0.43 * F, 1.06 * F, 0.06 * F], UP, [0.24 * F, 0.97 * F, -0.16 * F], UP, [0, 0, 1])
for name, err in (("pour", POUR_ERR), ("steam", STEAM_ERR), ("serve", SERVE_ERR)):
    print(f"[DEBUG] {name} IK", err)


GRIP_R = figure.finger_pose("r", flex=2.6, spread=0.4, thumb=(10, 20, 20, 10, 15, 4))
HOLD_L = figure.finger_pose("l", flex=2.0, spread=0.6, thumb=(5, 25, 15, 10, 10, 4))
FLAT_L = figure.finger_pose("l", flex=0.2, spread=0.8)
REGIONS = ["face-skin", "hair", "hand-skin", "top", "apron", "jeans", "sneakers", "milk-jug", "cup"]
JUG_KG, CUP_KG = 0.56, 0.36


def pose(pid, en, de, den, dde, bones, met, breathing, hot):
    th = lib.pose_thermal(REGIONS, {"body-plume": 0.6}, met=met, breathing=breathing, exposed={"top": 0.85},
                          contact_regions={"sneakers": {"fraction": 0.3, "tempC": 21}, "hand-skin": {"fraction": 0.15, "tempC": 45}}, operating={"hot-drinks": hot})
    return {"id": pid, "title": T(en, de), "description": T(den, dde), "bones": bones, "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"], "thermal": th,
            "loads": [{"id": "milk-jug-load", "kind": "carried", "title": T("Jug with steamed milk", "Kännchen mit aufgeschäumter Milch"), "anchor": "jug-cog", "fxN": 0, "fyN": round(-JUG_KG * 9.80665, 2), "source": "authored-estimate"},
                      {"id": "cup-load", "kind": "carried", "title": T("Cup with espresso", "Tasse mit Espresso"), "anchor": "cup-cog", "fxN": 0, "fyN": round(-CUP_KG * 9.80665, 2), "source": "authored-estimate"}]}


pour_bones = {**relaxed_bones, **TRUNK_POUR, **POUR_ARMS, **GRIP_R, **HOLD_L}
steam_bones = {**relaxed_bones, **TRUNK_STEAM, **STEAM_ARMS, **GRIP_R, **FLAT_L}
serve_bones = {**relaxed_bones, **TRUNK_SERVE, **SERVE_ARMS, **GRIP_R, **HOLD_L}
a["poses"] = [
    pose("pouring-latte-art", "Pouring latte art", "Latte Art gießen",
         "Bowed over the cup she holds tilted in the left hand, she pours the steamed milk from the stainless jug in the right hand, the spout a few centimetres above the crema.",
         "Über die in der linken Hand schräg gehaltene Tasse gebeugt gießt sie die aufgeschäumte Milch aus dem Edelstahlkännchen in der rechten Hand, der Ausguss wenige Zentimeter über der Crema.",
         pour_bones, 1.8, "concentrated-pouring", 1.0),
    pose("steaming-milk", "Feeling the milk temperature", "Milchtemperatur fühlen",
         "The jug held upright at chest height as under the steam wand, the left palm flat against its side to judge the temperature.",
         "Das Kännchen aufrecht auf Brusthöhe wie unter der Dampflanze, die linke Handfläche flach an seiner Seite, um die Temperatur zu fühlen.",
         steam_bones, 1.7, "quiet-standing", 0.8),
    pose("serving-cup", "Serving the cup", "Tasse servieren",
         "Upright again, she holds the finished cup out over the counter to the guest and speaks, the jug resting upright in the right hand beside the hip.",
         "Wieder aufgerichtet reicht sie die fertige Tasse über die Theke zum Gast und spricht, das Kännchen ruht aufrecht in der rechten Hand neben der Hüfte.",
         serve_bones, 1.9, "talking", 0.6),
]
a["presentation"] = {"defaultPose": "pouring-latte-art", "defaultClip": "latte-art-pour", "defaultScale": 100}

# ------------------------------------------------------------------ jug and cup bones, placed in the hands in the default pose
probe = copy.deepcopy(doc)
for p in probe["asset"]["poses"]:
    p["loads"] = []
F = lib.frames(lib.write_tmp(probe, "t-barista-probe.json"), "pouring-latte-art")


def place(parent, origin, u, v):
    Hm = rot.frame(F[parent])
    ho = [F[parent]["o"][c] for c in "xyz"]
    w = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
    ang, abd, tw = rot.decompose(rot.mul(rot.T(Hm), rot.mat([u, v, w])))
    off = [sum(Hm[j][i] * (origin[j] - ho[j]) for j in range(3)) for i in range(3)]
    return {"x": lib.r5(off[0]), "y": lib.r5(off[1]), "z": lib.r5(off[2]), "angleDeg": round(ang, 3), "abductionDeg": round(abd, 3), "twistDeg": round(tw, 3)}


cup_side = [0, -CUP_AX[2], CUP_AX[1]]
lib.add_bone(a, {"id": "milk-jug", "parent": "hand-r", "title": T("Stainless milk jug, 0.6 l", "Milchkännchen aus Edelstahl, 0,6 l"), **place("hand-r", JUG_B, JUG_AX, JUG_SP), "lengthM": 0.115,
                 "joint": {"type": "revolute", "minDeg": -40, "maxDeg": 40}, "mass": {"kg": JUG_KG, "comX": 0.045, "comY": 0, "distribution": "point", "source": "authored-estimate"}}, after="hand-r")
lib.add_bone(a, {"id": "cup", "parent": "hand-l", "title": T("Porcelain cappuccino cup", "Cappuccinotasse aus Porzellan"), **place("hand-l", CUP_C, CUP_AX, cup_side), "lengthM": 0.065,
                 "joint": {"type": "revolute", "minDeg": -30, "maxDeg": 30}, "mass": {"kg": CUP_KG, "comX": 0.03, "comY": 0, "distribution": "point", "source": "authored-estimate"}}, after="little-distal-l")
a["anchors"] += [
    {"id": "jug-cog", "bone": "milk-jug", "x": 0.045, "y": 0.0, "kind": "load", "title": T("Centre of gravity of the filled jug", "Schwerpunkt des gefüllten Kännchens")},
    {"id": "cup-cog", "bone": "cup", "x": 0.03, "y": 0.0, "kind": "load", "title": T("Centre of gravity of the cup", "Schwerpunkt der Tasse")},
    {"id": "jug-spout", "bone": "milk-jug", "x": 0.118, "y": 0.05, "kind": "attachment", "title": T("Spout of the jug", "Ausguss des Kännchens")},
    {"id": "jug-handle", "bone": "hand-r", "x": lib.r5(PALM_U), "y": 0, "z": lib.r5(PALM_W), "kind": "attachment", "title": T("Grip on the jug handle", "Griff am Kännchenhenkel")},
]


# ------------------------------------------------------------------ hair bun on its own bone at the back of the head
lib.add_bone(a, {"id": "hair-bun", "parent": "head", "title": T("Hair bun", "Haarknoten"), "x": lib.r5(0.092 * K), "y": lib.r5(0.083 * K), "angleDeg": 55, "lengthM": 0.06,
                 "joint": {"type": "fixed"}}, after="head")
BUN = [[0, 0], [0, 0.022], [0.008, 0.032], [0.025, 0.037], [0.045, 0.032], [0.058, 0.018], [0.062, 0]]
for sc in ("50", "100", "200"):
    lib.revolve_part(a, sc, "hair-bun", "hair-bun", BUN if sc != "200" else [[0, 0], [0, 0.03], [0.03, 0.038], [0.062, 0]], "hair", "hair", 0.12, 6.15, structure=False)

# ------------------------------------------------------------------ jug and cup per scale (bone-local: u along the axis, v to the spout or handle)
R0 = lib.frames(lib.write_tmp(doc, "t-barista-rest.json"), "rest")
JZ, CZ = R0["milk-jug"]["o"]["z"], R0["cup"]["o"]["z"]
JUG = {"50": [[0, 0], [0, 0.041], [0.003, 0.043], [0.01, 0.0445], [0.055, 0.042], [0.09, 0.0365], [0.108, 0.035], [0.112, 0.036], [0.115, 0.0365], [0.115, 0]],
       "100": [[0, 0], [0, 0.043], [0.01, 0.0445], [0.09, 0.0365], [0.115, 0.0365], [0.115, 0]],
       "200": [[0, 0], [0, 0.044], [0.115, 0.038], [0.115, 0]],
       "500": [[0, 0], [0, 0.044], [0.115, 0.038], [0.115, 0]]}
CUP = {"50": [[0, 0], [0, 0.024], [0.005, 0.029], [0.02, 0.039], [0.045, 0.045], [0.063, 0.0465], [0.065, 0.046], [0.065, 0]],
       "100": [[0, 0], [0, 0.026], [0.02, 0.04], [0.065, 0.046], [0.065, 0]],
       "200": [[0, 0], [0, 0.03], [0.065, 0.046], [0.065, 0]],
       "500": [[0, 0], [0, 0.034], [0.065, 0.046], [0.065, 0]]}
SPOUT = [[0.088, 0.03], [0.106, 0.033], [0.118, 0.038], [0.126, 0.053], [0.113, 0.048], [0.098, 0.042], [0.088, 0.036]]
JUG_HANDLE = {"50": [[0.022, -0.036], [0.022, -0.07], [0.03, -0.078], [0.09, -0.078], [0.1, -0.068], [0.1, -0.033], [0.089, -0.033], [0.089, -0.064], [0.085, -0.067], [0.036, -0.067], [0.033, -0.064], [0.033, -0.037]],
              "100": [[0.022, -0.036], [0.022, -0.078], [0.1, -0.078], [0.1, -0.033], [0.086, -0.033], [0.086, -0.062], [0.036, -0.062], [0.036, -0.037]]}
CUP_HANDLE = {"50": [[0.016, 0.035], [0.021, 0.056], [0.04, 0.064], [0.053, 0.05], [0.05, 0.043], [0.041, 0.054], [0.027, 0.049], [0.026, 0.037]],
              "100": [[0.016, 0.035], [0.021, 0.06], [0.054, 0.06], [0.05, 0.043], [0.03, 0.045], [0.026, 0.037]]}
for sc in SCALES:
    lib.revolve_part(a, sc, "milk-jug", "milk-jug", JUG[sc], "stainless-steel", "milk-jug", 0.74, 7.0)
    lib.revolve_part(a, sc, "cup", "cup", CUP[sc], "porcelain", "cup", 0.95, 7.1)
    if sc in ("50", "100"):
        th = 0.006 if sc == "50" else 0.008
        lib.extrude_part(a, sc, "jug-spout", "milk-jug", SPOUT, -0.014, 0.014, JZ, "stainless-steel", "milk-jug", 0.74, 7.01, structure=False)
        lib.extrude_part(a, sc, "jug-handle", "milk-jug", JUG_HANDLE[sc], -th, th, JZ, "stainless-steel", "milk-jug", 0.66, 7.02, structure=False)
        lib.extrude_part(a, sc, "cup-handle", "cup", CUP_HANDLE[sc], -0.005 if sc == "50" else -0.0075, 0.005 if sc == "50" else 0.0075, CZ, "porcelain", "cup", 0.95, 7.11, structure=False)


# ------------------------------------------------------------------ closed hands at 1:200 and 1:500: the open hand mass of the base is drawn in at the fingers
def close_coarse_hands(knuckle=0.10 * K, factor=0.72):
    """✊️ The hand masses of the two coarse scales (palm outline, hand loft, hand bones) keep the palm and shorten everything beyond the knuckles, as fingers closed round a vessel do; 1:50 and 1:100 curl their finger bones instead."""
    squeeze = lambda u: lib.r5(u if u <= knuckle else knuckle + (u - knuckle) * factor)
    for sc in ("200", "500"):
        for view in VIEWS:
            for sh in lib.shapes(a, sc, view):
                if sh["id"] in ("palm-l", "palm-r"):
                    sh["points"] = [[squeeze(u), v] for u, v in sh["points"]]
        for lst in ("body", "structure"):
            for so in a["volumes"][sc][lst]:
                if so.get("bone") in ("hand-l", "hand-r") and so.get("kind") == "loft":
                    for sec in so["sections"]:
                        sec["at"][0] = squeeze(sec["at"][0])


close_coarse_hands()

# ------------------------------------------------------------------ clips
def de(x):
    """🔢 A number in German notation (decimal comma)."""
    return str(x).replace(".", ",")


def keyed(t, pose_id, bones=None):
    k = {"t": round(t, 4), "pose": pose_id, "ease": "smooth"}
    if bones:
        k["bones"] = bones
    return k


def track(target, keys, ease="smooth"):
    return {"target": target, "ease": ease, "keys": [{"t": t, "v": v} for t, v in keys]}


FEET = [{"anchor": x, "fromS": 0, "toS": None} for x in ("heel-l", "ball-l", "heel-r", "ball-r")]
POUR_S, IDLE_S, SERVE_S = 6.0, 5.0, 4.6
pour_keys = [keyed(0, "pouring-latte-art")]
for t, jug, cup, head in ((1.2, 6, 0, -1), (1.8, 10, -2, -2), (2.2, 7, -3, -2), (2.6, 11, -4, -2.5), (3.0, 7, -5, -2.5), (3.4, 12, -7, -3), (4.4, 15, -13, -3), (5.2, -4, -13, -1)):
    pour_keys.append(keyed(t, "pouring-latte-art", {"milk-jug": {"angleDeg": jug}, "cup": {"angleDeg": cup}, "head": {"angleDeg": round(TRUNK_POUR["head"]["angleDeg"] + head, 2)}}))
pour_keys.append(keyed(POUR_S, "pouring-latte-art"))
pour_clip = {
    "id": "latte-art-pour", "title": T("Pouring latte art", "Latte Art gießen"),
    "description": T("A six-second loop of one pour: the jug tips over the tilted cup, the milk sinks under the crema, the spout comes down and the jug rocks from side to side three times to draw the leaves of a rosetta while the cup is levelled, then one lifted stroke through the pattern and the jug is set back. Both feet stay planted, the head follows the spout.",
                     "Eine Schleife von sechs Sekunden für einen Guss: Das Kännchen kippt über die schräg gehaltene Tasse, die Milch sinkt unter die Crema, der Ausguss senkt sich und das Kännchen schwenkt dreimal hin und her und zeichnet die Blätter einer Rosetta, während die Tasse gerade gestellt wird, dann ein angehobener Zug durch das Muster und das Kännchen geht zurück. Beide Füße bleiben stehen, der Kopf folgt dem Ausguss."),
    "durationS": POUR_S, "fps": 30, "loop": True, "rootMotion": "none", "keys": pour_keys,
    "contacts": [{**c, "toS": POUR_S} for c in FEET],
    "phases": [{"id": "sink", "fromS": 0, "toS": 1.8, "title": T("Sinking the milk under the crema", "Milch unter die Crema sinken lassen")},
               {"id": "draw", "fromS": 1.8, "toS": 3.4, "title": T("Rocking the jug to draw the pattern", "Kännchen schwenken und das Muster zeichnen")},
               {"id": "cut", "fromS": 3.4, "toS": 5.2, "title": T("Levelling the cup and cutting through", "Tasse gerade stellen und durchziehen")},
               {"id": "reset", "fromS": 5.2, "toS": POUR_S, "title": T("Setting the jug back", "Kännchen zurücknehmen")}],
    "events": [{"t": 1.8, "kind": "motion", "title": T("Spout lowered to the surface", "Ausguss zur Oberfläche gesenkt")}, {"t": 4.4, "kind": "motion", "title": T("Stroke through the pattern", "Zug durch das Muster")}],
    "tracks": [track("activityMet", [(0, 1.8), (POUR_S, 1.8)]), track("breathing", [(0, 1), (POUR_S, 1)], "step"), track("operating:hot-drinks", [(0, 1), (POUR_S, 1)])],
}
idle_keys = []
for t, dt, dl, dh in ((0, 0, 0, 0), (1.25, -0.34, -0.22, 1.6), (2.5, 0.22, 0.14, 0.4), (3.75, -0.28, -0.18, 1.3), (IDLE_S, 0, 0, 0)):
    idle_keys.append(keyed(t, "steaming-milk", {"torso": {"angleDeg": round(TRUNK_STEAM["torso"]["angleDeg"] + dt, 3)}, "lumbar": {"angleDeg": round(TRUNK_STEAM["lumbar"]["angleDeg"] + dl, 3)},
                                                "head": {"angleDeg": round(TRUNK_STEAM["head"]["angleDeg"] + dh, 3)}}))
idle_clip = {
    "id": "idle-breathing", "title": T("Waiting for the milk", "Warten auf die Milch"),
    "description": T("A five-second loop with the jug held upright at chest height and the left palm on its side: quiet breathing lifts the chest and the head settles in a small sway while the milk comes up to temperature.",
                     "Eine Schleife von fünf Sekunden mit dem aufrecht auf Brusthöhe gehaltenen Kännchen und der linken Handfläche an seiner Seite: Ruhiges Atmen hebt die Brust und der Kopf wiegt sich leicht, während die Milch auf Temperatur kommt."),
    "durationS": IDLE_S, "fps": 30, "loop": True, "rootMotion": "none", "keys": idle_keys,
    "contacts": [{**c, "toS": IDLE_S} for c in FEET], "phases": [{"id": "waiting", "fromS": 0, "toS": IDLE_S, "title": T("Feeling the temperature", "Temperatur fühlen")}], "events": [],
    "tracks": [track("activityMet", [(0, 1.7), (IDLE_S, 1.7)]), track("breathing", [(0, 0), (IDLE_S, 0)], "step"), track("operating:hot-drinks", [(0, 0.8), (IDLE_S, 0.8)])],
}
serve_clip = {
    "id": "serve-to-guest", "title": T("Serving the cup to the guest", "Tasse dem Gast reichen"),
    "description": T("From the last stroke of the pour she straightens, lowers the jug beside the hip and holds the finished cup out over the counter, looks up and speaks to the guest.",
                     "Aus dem letzten Zug des Gusses richtet sie sich auf, senkt das Kännchen neben die Hüfte und reicht die fertige Tasse über die Theke, blickt auf und spricht mit dem Gast."),
    "durationS": SERVE_S, "fps": 30, "loop": False, "rootMotion": "none",
    "keys": [keyed(0, "pouring-latte-art", {"milk-jug": {"angleDeg": -4}, "cup": {"angleDeg": -13}}), keyed(1.6, "serving-cup"),
             keyed(3.0, "serving-cup", {"head": {"angleDeg": round(TRUNK_SERVE["head"]["angleDeg"] - 4, 2)}, "neck": {"angleDeg": round(TRUNK_SERVE["neck"]["angleDeg"] - 2, 2)}}), keyed(SERVE_S, "serving-cup")],
    "contacts": [{**c, "toS": SERVE_S} for c in FEET],
    "phases": [{"id": "straighten", "fromS": 0, "toS": 1.6, "title": T("Straightening and holding the cup out", "Aufrichten und Tasse reichen")}, {"id": "speak", "fromS": 1.6, "toS": SERVE_S, "title": T("Speaking to the guest", "Sprechen mit dem Gast")}],
    "events": [{"t": 1.6, "kind": "motion", "title": T("Cup over the counter", "Tasse über der Theke")}],
    "tracks": [track("activityMet", [(0, 1.8), (1.6, 1.9), (SERVE_S, 1.9)]), track("breathing", [(0, 1), (1.6, 2), (SERVE_S, 2)], "step"), track("operating:hot-drinks", [(0, 1), (1.6, 0.6), (SERVE_S, 0.6)])],
}
a["clips"] = [pour_clip, idle_clip, serve_clip]

# ------------------------------------------------------------------ materials
a["materials"] = [
    {"id": "skin", "title": T("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "hair", "title": T("Hair", "Haar"), "emissivity": 0.98, "solarAbsorptivity": 0.85, "source": "authored-estimate"},
    {"id": "cotton-underwear", "title": T("Cotton camisole and briefs", "Baumwollunterhemd und -unterhose"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "cotton-jersey", "title": T("Charcoal cotton jersey long-sleeve top", "Anthrazitfarbenes Langarmshirt aus Baumwolljersey"), "emissivity": 0.9, "solarAbsorptivity": 0.82, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "waxed-canvas", "title": T("Waxed cotton canvas bib apron", "Latzschürze aus gewachstem Baumwollcanvas"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "denim", "title": T("Dark cotton denim", "Dunkler Baumwolldenim"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "cotton-sock", "title": T("Cotton socks", "Baumwollsocken"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "canvas-sneaker", "title": T("White canvas sneaker with rubber sole", "Weißer Stoffsneaker mit Gummisohle"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "stainless-steel", "title": T("Brushed stainless steel of the milk jug", "Gebürsteter Edelstahl des Milchkännchens"), "emissivity": 0.3, "solarAbsorptivity": 0.4, "specificHeatJKgK": 500, "densityKgM3": 7900, "conductivityWmK": 15, "source": "literature", "reference": "ashrae-2021"},
    {"id": "porcelain", "title": T("Glazed white porcelain", "Glasiertes weißes Porzellan"), "emissivity": 0.92, "solarAbsorptivity": 0.25, "specificHeatJKgK": 1070, "densityKgM3": 2400, "conductivityWmK": 1.5, "source": "literature", "reference": "ashrae-2021"},
    {"id": "bone", "title": T("Bone", "Knochen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"},
    {"id": "costal-cartilage", "title": T("Costal cartilage", "Rippenknorpel"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "source": "authored-estimate"},
]

# ------------------------------------------------------------------ thermal model
L = figure.layer
regions_old = {r["id"]: r for r in a["thermal"]["regions"]}
face, hair, hands = regions_old["face-skin"], regions_old["hair"], regions_old["hand-skin"]
JUG_HEAT_W, CUP_HEAT_W = 12.0, 5.0
a["thermal"]["regions"] = [
    face, hair, hands,
    figure.region("top", "Long-sleeve top over a camisole", "Langarmshirt über einem Unterhemd", "clothing", "body", "cotton-jersey", 0.7, 0.3, "vertical-plate", 0.45,
                  [L("camisole", "Cotton camisole", "Baumwollunterhemd", "cotton-underwear", 0.0006, 0.06, 110), L("top", "Cotton jersey long-sleeve top", "Langarmshirt aus Baumwolljersey", "cotton-jersey", 0.001, 0.25, 220)],
                  13500, "trunk", 0.47, 0.41),
    figure.region("apron", "Waxed-canvas bib apron over chest and thighs", "Latzschürze aus gewachstem Canvas über Brust und Oberschenkeln", "clothing", "body", "waxed-canvas", 0.3, 0.4, "vertical-plate", 0.3,
                  [L("apron", "Waxed canvas apron over the garment beneath", "Schürze aus gewachstem Canvas über dem Kleidungsstück darunter", "waxed-canvas", 0.0022, 0.42, 780)],
                  13800, "trunk", 0.10, 0.11),
    figure.region("jeans", "Dark jeans", "Dunkle Jeans", "clothing", "body", "denim", 0.5, 0.12, "cylinder", 0.1,
                  [L("briefs", "Cotton briefs", "Baumwollunterhose", "cotton-underwear", 0.0008, 0.04, 100), L("jeans", "Cotton denim jeans", "Jeans aus Baumwolldenim", "denim", 0.0012, 0.24, 420)],
                  13500, "leg", 0.25, 0.27),
    figure.region("sneakers", "Socks and canvas sneakers", "Socken und Stoffsneaker", "surface", "body", "canvas-sneaker", 0.14, 0.09, "horizontal-plate-up", 0.1,
                  [L("socks", "Cotton socks", "Baumwollsocken", "cotton-sock", 0.0015, 0.02, 100), L("sneaker", "Canvas sneaker", "Stoffsneaker", "canvas-sneaker", 0.003, 0.03, 600)],
                  13800, "foot", 0.03, 0.06),
    figure.region("milk-jug", "Stainless jug with steamed milk", "Edelstahlkännchen mit aufgeschäumter Milch", "surface", "operating", "stainless-steel", 0.04, 0.115, "cylinder", 0.4, [], 30000, ref="ashrae-2021",
                  operating={"key": "hot-drinks", "heatW": JUG_HEAT_W}, ratedPowerW=0, ratedPowerBasis="passive"),
    figure.region("cup", "Porcelain cup with espresso and milk", "Porzellantasse mit Espresso und Milch", "surface", "operating", "porcelain", 0.025, 0.065, "cylinder", 0.5, [], 36000, ref="ashrae-2021",
                  operating={"key": "hot-drinks", "heatW": CUP_HEAT_W}, ratedPowerW=0, ratedPowerBasis="passive"),
]
for sc in SCALES:
    for s in a["volumes"][sc]["thermal"]:
        if s["id"].startswith("th-apron-"):
            s["layer"] = "apron"
a["thermal"]["operating"] = [{"id": "hot-drinks", "title": T("Hot milk and coffee", "Heiße Milch und Kaffee"),
                              "levels": [{"v": 0, "title": T("Cold, empty", "Kalt, leer")}, {"v": 0.6, "title": T("Finished drink cooling", "Fertiges Getränk kühlt ab")}, {"v": 1, "title": T("Milk at 62 degrees C during the pour", "Milch mit 62 Grad C beim Gießen")}]}]
a["thermal"]["environment"] = {"airTempC": 22, "meanRadiantTempC": 23, "skyTempC": 22, "groundTempC": 22, "relativeHumidity": 0.5, "windMS": 0.12, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
a["thermal"]["baseMet"] = 1.8
skin = figure.physiology(a, SEX, STATURE, BODY_KG, AGE)
for e in a["thermal"]["emitters"]:
    if e["id"] == "body-plume":
        e["region"] = "top"
a["thermal"]["assumptions"] = [
    T(f"Skin area {skin} m2 after DuBois and DuBois (1916) for {BODY_KG:.0f} kg and {STATURE} m; skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.41 trunk and arms under the top, 0.11 chest and thighs under the apron, 0.27 legs in jeans, 0.06 feet (rule-of-nines proportions, authored).",
      f"Hautfläche {de(skin)} m2 nach DuBois und DuBois (1916) für {BODY_KG:.0f} kg und {de(STATURE)} m; Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,41 Rumpf und Arme unter dem Oberteil, 0,11 Brust und Oberschenkel unter der Schürze, 0,27 Beine in Jeans, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
    T("Work clothing of about 0.7 clo (camisole, long-sleeve jersey top, jeans, waxed bib apron over chest and thighs, socks and canvas sneakers) after the garment tables of ISO 9920 (1 clo = 0.155 m2K/W), as local layer insulation per region; vapour resistance R/(0.0165 K/Pa x 0.38). The default surroundings are a café behind the espresso machine at 22 degrees C with a mean radiant temperature of 23 degrees C.",
      "Arbeitskleidung von etwa 0,7 clo (Unterhemd, Langarmshirt aus Jersey, Jeans, gewachste Latzschürze über Brust und Oberschenkeln, Socken und Stoffsneaker) nach den Kleidungstabellen von ISO 9920 (1 clo = 0,155 m2K/W), als örtliche Schichtisolation je Region; Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Standardumgebung ist ein Café hinter der Espressomaschine mit 22 Grad C und einer mittleren Strahlungstemperatur von 23 Grad C."),
    T("Activity: standing bar work with light arm movement is 1.8 met while pouring, 1.7 met while steaming and 1.9 met while serving and speaking (ISO 8996 Annex B, standing light manual work; Ainsworth compendium, occupation codes for food service standing, 2.0 to 2.5 met with walking, reduced for work on the spot; authored). The basal metabolism follows Mifflin-St Jeor for a woman of 26 years.",
      "Aktivität: Thekenarbeit im Stehen mit leichter Armbewegung sind 1,8 met beim Gießen, 1,7 met beim Aufschäumen und 1,9 met beim Servieren und Sprechen (ISO 8996 Anhang B, leichte Handarbeit im Stehen; Ainsworth-Kompendium, Berufscodes für Gastronomie im Stehen, 2,0 bis 2,5 met mit Gehen, für Arbeit auf der Stelle reduziert; geschätzt). Der Grundumsatz folgt Mifflin-St Jeor für eine Frau von 26 Jahren."),
    T(f"The jug (0.35 l of milk at about 62 degrees C) and the cup (espresso and milk at about 60 degrees C) are surfaces heated from inside that release about {JUG_HEAT_W:.0f} W and {CUP_HEAT_W:.0f} W to the room at the level 1 of the key hot-drinks (free convection and radiation of the small hot surfaces plus evaporation from the open milk, authored); they draw no power and are passive on the nameplate. The espresso machine is not part of this figure.",
      f"Das Kännchen (0,35 l Milch mit etwa 62 Grad C) und die Tasse (Espresso und Milch mit etwa 60 Grad C) sind von innen beheizte Flächen, die auf Stufe 1 des Schlüssels hot-drinks etwa {JUG_HEAT_W:.0f} W und {CUP_HEAT_W:.0f} W an den Raum abgeben (freie Konvektion und Strahlung der kleinen heißen Flächen und Verdunstung der offenen Milch, geschätzt); sie nehmen keine Leistung auf und sind auf dem Typenschild passiv. Die Espressomaschine gehört nicht zu dieser Figur."),
]
a["structure"]["dynamics"] = "quasi-static"
a["structure"]["assumptions"] = [
    T(f"Segments are point masses at the de Leva (1996) centres of the female table, {BODY_KG + CLOTHES_KG:.1f} kg with clothing; the filled jug ({JUG_KG} kg) and the cup ({CUP_KG} kg) are bones of their own in the hands and act also as carried loads at their centres of gravity.",
      f"Segmente sind Punktmassen in den Schwerpunkten nach de Leva (1996), Frauentabelle, {de(round(BODY_KG + CLOTHES_KG, 1))} kg mit Kleidung; das gefüllte Kännchen ({de(JUG_KG)} kg) und die Tasse ({de(CUP_KG)} kg) sind eigene Knochen in den Händen und wirken zugleich als getragene Lasten in ihren Schwerpunkten."),
    T("Contacts are heel and ball of each foot; both feet stand flat at hip-joint width in every named pose and clip. The pour is slow hand work and is solved quasi-statically.",
      "Kontakte sind Ferse und Ballen jedes Fußes; beide Füße stehen in jeder benannten Pose und jedem Clip flach in Hüftgelenkbreite. Das Gießen ist langsame Handarbeit und wird quasistatisch gelöst."),
]

# ------------------------------------------------------------------ measured quantities: dimensions, region areas, mass
sys.path.insert(0, lib.HERE + "/../FIN-A")
import fit_dims  # noqa: E402  (the dimension that satisfies the volume guard and the drawing guard of every scale at once)
a["dimensions"]["massKg"] = round(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if b.get("mass")), 2)
dims, DIM_DEFECTS = fit_dims.fit(lib.write_tmp(doc, "t-dims.json"))
a["dimensions"].update(dims)
print("[DEBUG] dimension defects", DIM_DEFECTS)
AREAS = figure.calibrate_areas(doc, {"top", "apron", "jeans", "sneakers", "milk-jug", "cup", "face-skin", "hair", "hand-skin"})

# ------------------------------------------------------------------ texts, provenance, decisions
a["title"] = T("Barista", "Barista")
a["summary"] = T(
    "A 26-year-old barista of 1.70 m in a charcoal long-sleeve top, a waxed-canvas bib apron, dark jeans and white canvas sneakers, her hair in a bun, bows over a cappuccino cup held tilted in the left hand and pours steamed milk from a stainless jug in the right. Poses for pouring latte art, feeling the milk temperature and serving the cup; a pouring loop with the rocking jug, a waiting loop and a serving sequence with speech. Jug and cup are hot surfaces without a power rating.",
    "Eine 26-jährige Barista von 1,70 m in anthrazitfarbenem Langarmshirt, Latzschürze aus gewachstem Canvas, dunkler Jeans und weißen Stoffsneakern, das Haar zum Knoten gebunden, beugt sich über eine in der linken Hand schräg gehaltene Cappuccinotasse und gießt aufgeschäumte Milch aus einem Edelstahlkännchen in der rechten. Posen für das Gießen von Latte Art, das Fühlen der Milchtemperatur und das Servieren der Tasse; eine Gießschleife mit schwenkendem Kännchen, eine Warteschleife und ein Serviervorgang mit Sprechen. Kännchen und Tasse sind heiße Flächen ohne Leistungsangabe.")
a["tags"] = ["human", "adult", "standing", "hospitality", "cafe", "barista", "coffee", "pouring", "apron", "service", "female"]
prov = a["provenance"]
prov["author"] = "Semio staffage human worker HW-02"
prov["method"] = "handcrafted"
prov["created"] = "2026-11-22"
refs = {r["id"]: r for r in prov["references"]}
refs["iso-8996-2021"] = {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate; Annex B metabolic rates for activities and Annex C walking speeds."}
refs["ainsworth-2011"] = {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; occupation codes 11790-11795 (food service, waiter or waitress)."}
prov["references"] = list(refs.values())
prov["notes"] = T(
    f"Derived by hand from the archive's standing-adult (rig, outlines and per-scale volumes scaled uniformly by 1.70/1.69 = {K:.3f}) with the hood, kangaroo pocket, drawstrings and rib hem removed; the sweatshirt became a charcoal long-sleeve top, the jeans are darker, the sneakers white. Segment masses re-derived from de Leva's female table for 61 kg body plus 1.9 kg clothing. Added by hand: the bib apron (bib on the chest, two skirt panels blended 60/40 between pelvis and thigh), its neck strap, pocket and seam in the front view, the hair bun on its own bone, the 0.6 l stainless jug with spout and handle on the right hand and the cappuccino cup with handle on the left hand, each a revolved body drawn in all three views. The arms of the three poses are solved by inverse kinematics onto the jug handle and the cup body. Finished after an interruption of the authoring session: clips, thermal model, texts and measured quantities were completed by the finisher of the wave from the worker's generator.",
    f"Von Hand aus dem Archivobjekt standing-adult abgeleitet (Rig, Umrisse und Volumen je Maßstab gleichmäßig mit 1,70/1,69 = {K:.3f} skaliert), Kapuze, Kängurutasche, Kordeln und Rippensaum entfernt; aus dem Sweatshirt wurde ein anthrazitfarbenes Langarmshirt, die Jeans ist dunkler, die Sneaker sind weiß. Segmentmassen aus der Frauentabelle von de Leva für 61 kg Körper und 1,9 kg Kleidung neu bestimmt. Von Hand ergänzt: die Latzschürze (Latz auf der Brust, zwei Schürzenbahnen, 60/40 zwischen Becken und Oberschenkel gemischt), ihr Nackenband, Tasche und Naht in der Vorderansicht, der Haarknoten auf eigenem Knochen, das Edelstahlkännchen von 0,6 l mit Ausguss und Henkel an der rechten Hand und die Cappuccinotasse mit Henkel an der linken Hand, jeweils als Drehkörper in allen drei Ansichten gezeichnet. Die Arme der drei Posen sind mit inverser Kinematik auf den Kännchenhenkel und den Tassenkörper gelöst. Nach einer Unterbrechung der Sitzung fertiggestellt: Clips, thermisches Modell, Texte und Messgrößen hat die abschließende Bearbeitung der Welle aus dem Generator ergänzt.".replace(f"{K:.3f}", f"{K:.3f}".replace(".", ",")))

DEC = {
    "50": dict(
        silhouette=T("The three views of the 1:50 body bowed over the cup, 34 mm tall: head with ear, eye, nose and mouth under the hair with its bun, neck, long-sleeve top, the bib apron on the chest and its skirt over the thighs, hands with thumb and fingers round jug and cup, jeans, sneakers with sole and toe cap; the jug with spout and handle, the cup with its handle.",
                     "Die drei Ansichten des 1:50-Körpers über die Tasse gebeugt, 34 mm hoch: Kopf mit Ohr, Auge, Nase und Mund unter dem Haar mit Knoten, Hals, Langarmshirt, die Latzschürze auf der Brust und ihre Bahnen über den Oberschenkeln, Hände mit Daumen und Fingern um Kännchen und Tasse, Jeans, Sneaker mit Sohle und Zehenkappe; das Kännchen mit Ausguss und Henkel, die Tasse mit Henkel."),
        retained=T("Everything of 1:100 plus the pocket seam of the apron, the rounded jug and cup profiles and the finger separations.", "Alles aus 1:100 und dazu die Taschennaht der Schürze, die gerundeten Profile von Kännchen und Tasse und die Fingertrennungen."),
        merged=T("Brows, lips and chin into the head outline; the saucer is left on the counter and not drawn.", "Brauen, Lippen und Kinn im Kopfumriss; die Untertasse bleibt auf der Theke und ist nicht gezeichnet."),
        omitted=T("Apron ties at the back, stitching, fabric texture, the milk level in the jug, the milk stream and the pattern in the cup.", "Schürzenbänder am Rücken, Nähte, Gewebestruktur, der Milchspiegel im Kännchen, der Milchstrahl und das Muster in der Tasse."),
        thinElements=T("The 12 mm jug handle and the 10 mm cup handle are drawn at their true width; the apron strap and pocket are 0.13 mm detail lines.", "Der 12 mm starke Kännchenhenkel und der 10 mm starke Tassenhenkel sind in wahrer Breite gezeichnet; Schürzenband und Tasche sind Detaillinien von 0,13 mm."),
        hatching=T("The denim hatch of the base figure on the jeans; the apron and the top read by tone.", "Die Denimschraffur der Ausgangsfigur auf der Jeans; Schürze und Oberteil lesen sich über den Tonwert."),
        styles=T("Mid-grey apron between the dark top and the dark jeans, light steel jug and white cup; body pen 0.25 mm, detail 0.13 mm, texture 0.10 mm; dark mode inverts the tones.", "Mittelgraue Schürze zwischen dunklem Oberteil und dunkler Jeans, helles Stahlkännchen und weiße Tasse; Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; der dunkle Modus kehrt die Tonwerte um."),
        overlays=T("Every shape carries one thermal region: top, apron, jeans, sneakers, skin and hair; jug and cup are heated regions of the key hot-drinks; the structural views colour the bones the shapes ride on.", "Jede Form trägt eine thermische Region: Oberteil, Schürze, Jeans, Sneaker, Haut und Haar; Kännchen und Tasse sind beheizte Regionen des Schlüssels hot-drinks; die statischen Ansichten färben die Knochen, auf denen die Formen sitzen.")),
    "100": dict(
        silhouette=T("The three views of the 1:100 body, 17 mm tall: head with nose and ear under the hair and bun, neck, top with sleeves, bib and skirt of the apron, hands, jeans, sneakers; jug and cup as simple revolved profiles with spout and handles.",
                     "Die drei Ansichten des 1:100-Körpers, 17 mm hoch: Kopf mit Nase und Ohr unter Haar und Knoten, Hals, Oberteil mit Ärmeln, Latz und Bahnen der Schürze, Hände, Jeans, Sneaker; Kännchen und Tasse als einfache Drehprofile mit Ausguss und Henkeln."),
        retained=T("The apron strap and pocket, the spout and both handles, the gap between the legs.", "Schürzenband und Tasche, der Ausguss und beide Henkel, der Spalt zwischen den Beinen."),
        merged=T("Fingers into a mitten with thumb; the rounded jug foot and rim into straight profile lines.", "Finger zu einem Fäustling mit Daumen; der gerundete Kännchenboden und -rand zu geraden Profillinien."),
        omitted=T("Pocket seam, finger separations.", "Taschennaht, Fingertrennungen."),
        thinElements=T("The jug handle is thickened from 12 to 16 mm and the cup handle from 10 to 15 mm so that they print at 0.15 mm.", "Der Kännchenhenkel ist von 12 auf 16 mm und der Tassenhenkel von 10 auf 15 mm verdickt, damit sie mit 0,15 mm drucken."),
        hatching=T("The denim hatch on the jeans.", "Die Denimschraffur auf der Jeans."),
        styles=T("As 1:50 with body pen 0.20 mm and detail 0.10 mm, no texture pen.", "Wie 1:50 mit Körperstift 0,20 mm und Detail 0,10 mm, ohne Texturstift."),
        overlays=T("As 1:50.", "Wie 1:50.")),
    "200": dict(
        silhouette=T("The three views of the 1:200 body, 8.5 mm tall, one mass per part: head, hair cap with bun, neck, top, sleeves, forearms, hands, apron bib and skirt, jeans, sneakers; the jug and the cup as two tapered masses in the hands.",
                     "Die drei Ansichten des 1:200-Körpers, 8,5 mm hoch, eine Masse je Teil: Kopf, Haarkappe mit Knoten, Hals, Oberteil, Ärmel, Unterarme, Hände, Latz und Bahnen der Schürze, Jeans, Sneaker; Kännchen und Tasse als zwei konische Massen in den Händen."),
        retained=T("The bow over the cup, the two raised forearms with jug and cup, the apron and its strap that make the barista.", "Die Beugung über die Tasse, die zwei erhobenen Unterarme mit Kännchen und Tasse, die Schürze und ihr Band, die die Barista ausmachen."),
        merged=T("Spout and handles into the jug and cup masses; the closed fingers into shortened hand masses.", "Ausguss und Henkel in den Massen von Kännchen und Tasse; die geschlossenen Finger zu verkürzten Handmassen."),
        omitted=T("Pocket, handles, spout, denim hatch.", "Tasche, Henkel, Ausguss, Denimschraffur."),
        thinElements=T("None is drawn thinner than the smallest feature; the apron strap is a 0.10 mm detail line.", "Nichts ist dünner als das kleinste Merkmal gezeichnet; das Schürzenband ist eine Detaillinie von 0,10 mm."),
        hatching=T("None.", "Keine."),
        styles=T("Body pen 0.18 mm, detail 0.10 mm; trunk outlines are the views of the trunk body of this scale.", "Körperstift 0,18 mm, Detail 0,10 mm; die Rumpfumrisse sind die Ansichten des Rumpfkörpers dieses Maßstabs."),
        overlays=T("As 1:50.", "Wie 1:50.")),
    "500": dict(
        silhouette=T("A standing figure of 1.70 m (3.4 mm) bowed forward, the views of the 1:500 body: head with hair, neck, trunk, apron, and per side sleeve, forearm, hand, thigh, shank and shoe; jug and cup as two small cones in front of the chest.",
                     "Eine stehende, nach vorn gebeugte Figur von 1,70 m (3,4 mm), die Ansichten des 1:500-Körpers: Kopf mit Haar, Hals, Rumpf, Schürze und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel und Schuh; Kännchen und Tasse als zwei kleine Kegel vor der Brust."),
        retained=T("The forearms raised in front of the chest with the two vessels, the apron, stature and extent.", "Die vor der Brust erhobenen Unterarme mit den zwei Gefäßen, die Schürze, Körpergröße und Ausdehnung."),
        merged=T("Face and hair; the hair bun into the hair mass; the fingers closed round jug and cup into shortened hand masses.", "Gesicht und Haar; der Haarknoten in der Haarmasse; die um Kännchen und Tasse geschlossenen Finger zu verkürzten Handmassen."),
        omitted=T("Apron strap and pocket, fingers, spout and handles.", "Schürzenband und Tasche, Finger, Ausguss und Henkel."),
        thinElements=T("Jug and cup are drawn at their true diameters of 88 and 92 mm, never thickened.", "Kännchen und Tasse sind mit ihren wahren Durchmessern von 88 und 92 mm gezeichnet, nie verdickt."),
        hatching=T("None.", "Keine."),
        styles=T("One pen of 0.13 mm; dark trunk and legs, mid-grey apron, light vessels.", "Ein Stift von 0,13 mm; dunkler Rumpf und Beine, mittelgraue Schürze, helle Gefäße."),
        overlays=T("As 1:50, one region per shape.", "Wie 1:50, eine Region je Form.")),
}
for sc in SCALES:
    a["representations"][sc]["decisions"] = DEC[sc]
VOLDEC = {
    "50": T("Body at 1:50: the standing adult's lofted body (head with face, ears and hair cap, neck, trunk, sleeves, forearms, hands with every finger phalanx, jeans, shoes) scaled to 1.70 m, without hood, pocket, drawstrings and rib hem; the bib as a 0.42 m panel on the chest and the skirt as two panels skinned 60/40 to pelvis and thigh; the hair bun a revolved body on its own bone; the jug a revolved steel body with extruded spout and handle, the cup a revolved porcelain body with an extruded handle.",
            "Körper bei 1:50: der geloftete Körper der stehenden erwachsenen Person (Kopf mit Gesicht, Ohren und Haarkappe, Hals, Rumpf, Ärmel, Unterarme, Hände mit jedem Fingerglied, Jeans, Schuhe) auf 1,70 m skaliert, ohne Kapuze, Tasche, Kordeln und Rippensaum; der Latz als Bahn von 0,42 m auf der Brust und die Schürze als zwei Bahnen, 60/40 an Becken und Oberschenkel gebunden; der Haarknoten ein Drehkörper auf eigenem Knochen; das Kännchen ein Drehkörper aus Stahl mit extrudiertem Ausguss und Henkel, die Tasse ein Drehkörper aus Porzellan mit extrudiertem Henkel."),
    "100": T("Body at 1:100 with fewer sections for a 15 mm feature; apron, bun, jug and cup as at 1:50 with straight profiles, the handles 16 and 15 mm thick.", "Körper bei 1:100 mit weniger Schnitten für ein Merkmal von 15 mm; Schürze, Knoten, Kännchen und Tasse wie bei 1:50 mit geraden Profilen, die Henkel 16 und 15 mm stark."),
    "200": T("Body at 1:200, one mass per part for a 30 mm feature; the bun a small revolved mass, jug and cup truncated cones without spout and handles.", "Körper bei 1:200, eine Masse je Teil für ein Merkmal von 30 mm; der Knoten eine kleine Drehmasse, Kännchen und Tasse Kegelstümpfe ohne Ausguss und Henkel."),
    "500": T("Body at 1:500, the fewest masses for a 75 mm feature; jug and cup truncated cones of 88 and 92 mm diameter, no bun.", "Körper bei 1:500, die wenigsten Massen für ein Merkmal von 75 mm; Kännchen und Tasse Kegelstümpfe von 88 und 92 mm Durchmesser, kein Knoten."),
}
THERMDEC = {
    "50": T("Thermal at 1:50: the outer garment, skin, jug and cup surfaces are the body solids themselves; inside lie the camisole as a shell under the top, the briefs under the jeans and the socks in the shoes. The apron panels are the apron region over top and jeans; jug and cup are heated regions of the key hot-drinks without a power rating.",
            "Thermik bei 1:50: Die äußeren Kleidungs-, Haut-, Kännchen- und Tassenflächen sind die Körper selbst; innen liegen das Unterhemd als Schale unter dem Oberteil, die Unterhose unter der Jeans und die Socken in den Schuhen. Die Schürzenbahnen sind die Schürzenregion über Oberteil und Jeans; Kännchen und Tasse sind beheizte Regionen des Schlüssels hot-drinks ohne Leistungsangabe."),
    "100": T("Thermal at 1:100: every region lies on the body solids of this scale, the layers of a garment merged into its outer surface; the three outlets are patches of one smallest feature.", "Thermik bei 1:100: Jede Region liegt auf den Körpern dieses Maßstabs, die Schichten eines Kleidungsstücks in seiner Außenfläche zusammengefasst; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
    "200": T("Thermal at 1:200: every region lies on the masses of this scale, layers merged; the three outlets are patches of one smallest feature.", "Thermik bei 1:200: Jede Region liegt auf den Massen dieses Maßstabs, Schichten zusammengefasst; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
    "500": T("Thermal at 1:500: every region lies on the masses of this scale, jug and cup on their cones; the three outlets are patches of one smallest feature.", "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, Kännchen und Tasse auf ihren Kegeln; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
}
for sc in SCALES:
    a["volumes"][sc]["decisions"] = [VOLDEC[sc], THERMDEC[sc], a["volumes"][sc]["decisions"][2]]

if __name__ == "__main__" and "--stage" in sys.argv:
    print(lib.write_tmp(doc, "barista-stage.json"), dims, AREAS)
elif __name__ == "__main__":
    print(lib.write_asset(doc, SLUG), dims, AREAS)
