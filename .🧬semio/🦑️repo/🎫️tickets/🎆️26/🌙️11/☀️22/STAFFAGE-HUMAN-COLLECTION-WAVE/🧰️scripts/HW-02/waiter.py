"""🍽️ waiter-with-tray: a 1.78 m waiter in white shirt, black waistcoat, black trousers and a long bistro apron walking with a round
tray of three glasses held palm-up at shoulder height on the left hand. Derived from the archive's standing adult (rig, outlines and
volumes scaled by 1.78/1.69) and the walking commuter's planted gait; every garment and carried part is authored here.

python3 waiter.py [--tmp]   writes 🗂️catalog/👤️humans/waiter-with-tray/🔣️.json (or a scratch copy with --tmp)
"""
import copy, json, math, sys
import lib, figure, rot
from lib import T, SCALES, VIEWS

SLUG = "waiter-with-tray"
STATURE, STATURE_BASE = 1.78, 1.69
K = STATURE / STATURE_BASE
SEX, AGE, BODY_KG, CLOTHES_KG = "male", 31, 74.0, 2.1

doc = lib.load_base()
a = doc["asset"]
a["id"] = SLUG

# ------------------------------------------------------------------ garments
lib.strip_hoodie(a)
figure.drop_hem_and_hood_collar(a)
figure.torso_from_loft(a)
TRUNK = {"torso", "lumbar", "pelvis", "neck", "head"}
ARMS = lambda b: b.split("-")[0] in ("arm", "forearm", "clavicle")
figure.dress(
    a,
    {"waistcoat": ("hoodie", lambda b: b in TRUNK), "shirt": ("hoodie", ARMS), "trousers": ("jeans", lambda b: True), "shoes": ("sneakers", lambda b: True)},
    [
        (lambda s: s.get("region") == "waistcoat" or s.get("material") == "fleece-sweatshirt" and s.get("bone") in TRUNK, 0.14),
        (lambda s: s.get("region") == "shirt" or s.get("material") == "fleece-sweatshirt" and ARMS(s.get("bone", "")), 0.9),
        (lambda s: s.get("region") == "trousers" or s.get("material") == "denim", 0.22),
        (lambda s: s.get("region") == "shoes" or s.get("material") in ("canvas-sneaker",), 0.1),
        (lambda s: s.get("region") == "hair" or s.get("material") == "hair", 0.16),
    ],
    {("waistcoat", "tee"): "shirt", ("waistcoat", "sweatshirt"): "waistcoat", ("shirt", "sweatshirt"): "shirt", ("shirt", "tee"): "shirt",
     ("trousers", "jeans"): "trousers", ("shoes", "sneaker"): "shoe"},
)
figure.drop_thermal_solids(a, {"th-tee-sleeve-l", "th-tee-sleeve-r"})
for sc in SCALES:
    for view in VIEWS:
        lib.shapes(a, sc, view)[:] = [s for s in lib.shapes(a, sc, view) if not s["id"].startswith(("jeans-knee",))]
    for s in a["volumes"][sc]["thermal"]:
        if s.get("tags") and "body-plume" in s["tags"]:
            s["region"] = "waistcoat"
# the shirt collar band and cuffs stay white, the waistcoat hem dark
lib.retone(a, lambda s: s["id"] in ("collar-rib", "collar"), 0.9)
for sc in SCALES:
    for view in VIEWS:
        for s in lib.shapes(a, sc, view):
            if s["id"] in ("collar-rib", "collar"):
                s["region"] = "shirt"
lib.rename_ids(a, lambda i: i.replace("hoodie", "waistcoat").replace("jeans-", "trouser-").replace("hem-rib", "waistcoat-hem").replace("cuff-rib", "shirt-cuff").replace("collar-rib", "shirt-collar"))
for old, new in (("fleece-sweatshirt", "waistcoat-wool"), ("denim", "trouser-wool"), ("canvas-sneaker", "leather-shoe")):
    lib.set_material(a, old, new)
# shirt sleeves are cotton, not waistcoat cloth
for sc in SCALES:
    for s in a["volumes"][sc]["body"]:
        if ARMS(s["bone"]) and s["material"] == "waistcoat-wool":
            s["material"] = "cotton-shirt"
    for s in a["volumes"][sc]["thermal"]:
        if s["id"] == "th-tee":
            s["material"] = "cotton-shirt"
    for view in VIEWS:
        for s in lib.shapes(a, sc, view):
            if s.get("material") == "waistcoat-wool" and ARMS(s["bone"]):
                s["material"] = "cotton-shirt"


# raised, twisted and swinging arms are bodies of revolution in every view
lib.revolve_limbs(a, {"arm-l", "forearm-l", "arm-r", "forearm-r"})

# ------------------------------------------------------------------ bistro apron: one panel over the front of each thigh, waist to below the knee
APRON = {"top": -0.10, "bottom": 0.50, "front": 0.100, "back": 0.0, "inner": 0.085, "outer": 0.075}
for side, sg, zpaint in (("l", 1, 1.5), ("r", -1, 3.5)):
    w0, w1 = (-APRON["inner"], APRON["outer"]) if sg > 0 else (-APRON["outer"], APRON["inner"])
    prof = [[APRON["top"], APRON["back"]], [APRON["top"], APRON["front"]], [APRON["bottom"], APRON["front"]], [APRON["bottom"], APRON["back"]]]
    for sc in SCALES:
        lib.extrude_part(a, sc, f"apron-{side}", f"thigh-{side}", prof, w0, w1, sg * 0.085, "apron-cotton", "apron", 0.08, zpaint, structure=False,
                         weights=[{"bone": "pelvis", "weight": 0.55}, {"bone": f"thigh-{side}", "weight": 0.45}])

# ------------------------------------------------------------------ white shirt front in the V of the waistcoat, placket and welt pockets (front view)
VNECK = [[-0.072, 1.452], [-0.036, 1.447], [0.0, 1.445], [0.036, 1.447], [0.072, 1.452], [0.0, 1.300]]
for sc, parts in (("50", ("v", "placket", "welts", "buttons")), ("100", ("v", "placket", "welts")), ("200", ("v",))):
    f = lib.shapes(a, sc, "front")
    if "v" in parts:
        f.append({"id": "shirt-front", "bone": "torso", "role": "detail", "z": 4.9, "filled": True, "tone": 0.92, "region": "shirt", "kind": "poly", "points": VNECK, "closed": True})
    if "placket" in parts:
        f.append({"id": "waistcoat-placket", "bone": "torso", "role": "detail", "z": 4.91, "filled": False, "tone": 0.6, "region": "waistcoat", "kind": "poly", "points": [[0.0, 1.300], [0.0, 1.005]], "closed": False})
    if "welts" in parts:
        for side, sg in (("l", 1), ("r", -1)):
            f.append({"id": f"waistcoat-welt-{side}", "bone": "torso", "role": "detail", "z": 4.92, "filled": False, "tone": 0.6, "region": "waistcoat", "kind": "poly", "points": [[sg * 0.055, 1.105], [sg * 0.125, 1.112]], "closed": False})
    if "buttons" in parts:
        for n, y in enumerate((1.27, 1.205, 1.14, 1.075)):
            f.append({"id": f"waistcoat-button-{n}", "bone": "torso", "role": "texture", "z": 4.93, "filled": False, "tone": 0.6, "region": "waistcoat", "kind": "poly", "points": [[-0.012, y], [0.012, y]], "closed": False})

# ------------------------------------------------------------------ skeleton: without the hood the T1 spinous process is shortened to stay under the collar;
# at 1:500 the bent wrist of the tray hand gets its own member instead of riding in the forearm bar
t1 = lib.solid(a, "50", "structure", "t1-spinous")
t1["p1"] = [round(t1["p0"][i] + 0.55 * (t1["p1"][i] - t1["p0"][i]), 5) for i in range(3)]
fb = lib.solid(a, "500", "structure", "forearm-bones-l")
fb.pop("members")
fb["sections"][-2]["at"][0], fb["sections"][-1]["at"][0] = 0.247, 0.255
a["volumes"]["500"]["structure"].append({"id": "hand-bones-l", "kind": "capsule", "bone": "hand-l", "material": "bone", "role": "body", "tone": 0.85,
                                         "p0": [0.0, 0.0, -0.01], "p1": [0.07, 0.0, -0.01], "r0": 0.0375, "r1": 0.0375, "member": "hand-l", "part": "member"})

# ------------------------------------------------------------------ person: stature, masses
lib.scale_asset(a, K)
lib.set_masses(a, SEX, BODY_KG + CLOTHES_KG)
for b in a["rig"]["bones"]:
    b["title"] = {"en": b["title"]["en"].replace("with sweatshirt", "with shirt and waistcoat"), "de": b["title"]["de"].replace("mit Sweatshirt", "mit Hemd und Weste")}

# ------------------------------------------------------------------ respiration (patterns are indexed by the clip track "breathing")
resp = a["thermal"]["respiration"]
resp["patterns"] = [
    {"id": "quiet-standing", "title": T("Quiet standing", "Ruhiges Stehen"), "rateBpm": 13, "tidalVolumeL": 0.6, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
    {"id": "walking", "title": T("Walking briskly with the tray", "Zügiges Gehen mit dem Tablett"), "rateBpm": 19, "tidalVolumeL": 1.15, "inhaleFraction": 0.44, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.04, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"},
    {"id": "talking", "title": T("Talking to the guests", "Sprechen mit den Gästen"), "rateBpm": 16, "tidalVolumeL": 0.7, "inhaleFraction": 0.24, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.69, "pauseAfterExhaleFraction": 0.05, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
]
resp["motion"] = [{"bone": "torso", "angleDeg": 0.6, "dy": lib.r5(0.0035 * K)}]

# ------------------------------------------------------------------ poses
com = lib.load_base("walking-commuter")["asset"]
walk = com["clips"][0]
ckeys = {round(k["t"], 3): k["bones"] for k in walk["keys"]}
LEGS_TRUNK = [f"{p}-{s}" for p in ("thigh", "shank", "foot", "toe") for s in "lr"] + ["torso", "lumbar", "neck", "head"]
TRAY_ARM = {"arm-l": {"angleDeg": 50, "abductionDeg": 10}, "forearm-l": {"angleDeg": 90, "twistDeg": -60}, "hand-l": {"angleDeg": 30, "abductionDeg": 60}}
TRAY_FINGERS = figure.finger_pose("l", flex=0.0, spread=1.6, thumb=(-10, 30, 0, 12, 0, 4))
RIGHT_FINGERS = figure.finger_pose("r", flex=1.0)


def gait_bones(t):
    k = ckeys[round(t % 1.2, 3)]
    o = {b: dict(k[b]) for b in LEGS_TRUNK}
    sw = ckeys[round((t + 0.6) % 1.2, 3)]
    o["arm-r"] = {"angleDeg": sw["arm-l"]["angleDeg"], "abductionDeg": -6}
    o["forearm-r"] = {"angleDeg": sw["forearm-l"]["angleDeg"]}
    o["hand-r"] = {"angleDeg": sw["hand-l"]["angleDeg"]}
    return o


REF_TRUNK = ckeys[0.0]["torso"]["angleDeg"] + ckeys[0.0]["lumbar"]["angleDeg"]


def tray_level(bones):
    return {"tray": {"angleDeg": round(REF_TRUNK - bones.get("torso", {}).get("angleDeg", 0) - bones.get("lumbar", {}).get("angleDeg", 0), 3)}}


base_pose = next(p for p in a["poses"] if p["id"] == "relaxed")
relaxed_bones = base_pose["bones"]
REGIONS = ["face-skin", "hair", "hand-skin", "waistcoat", "shirt", "trousers", "apron", "shoes", "tray", "glassware"]


def thermal_binding(met, breathing, exposed=None, plume=0.6, contact=None):
    th = lib.pose_thermal(REGIONS, {"body-plume": plume}, exposed=exposed, met=met, breathing=breathing,
                          contact_regions=contact if contact is not None else {"shoes": {"fraction": 0.3, "tempC": 21}})
    return th


def pose(pid, en, de, den, dde, bones, contacts, met, breathing, exposed=None, key=False, contact=None):
    p = {"id": pid, "title": T(en, de), "description": T(den, dde), "bones": bones, "ground": "auto", "contacts": contacts,
         "thermal": thermal_binding(met, breathing, exposed, contact=contact),
         "loads": [{"id": "tray-load", "kind": "carried", "title": T("Tray with three filled glasses", "Tablett mit drei gefüllten Gläsern"), "anchor": "tray-cog", "fxN": 0, "fyN": round(-TRAY_KG * 9.80665, 2), "source": "authored-estimate"}]}
    if key:
        p["key"] = True
    return p


TRAY_KG = 0.42 + 0.30 + 0.52 + 0.64
walk_heel = {**relaxed_bones, **RIGHT_FINGERS, **TRAY_FINGERS, **gait_bones(0.0), **TRAY_ARM}
walk_mid = {**relaxed_bones, **RIGHT_FINGERS, **TRAY_FINGERS, **{b: dict(v) for b, v in next(p for p in com["poses"] if p["id"] == "passing-right")["bones"].items() if b in LEGS_TRUNK}, **TRAY_ARM}
sw = next(p for p in com["poses"] if p["id"] == "passing-right")["bones"]
walk_mid.update({"arm-r": {"angleDeg": round(-sw["arm-l"]["angleDeg"] * 0.6, 2), "abductionDeg": -6}, "forearm-r": {"angleDeg": 14}, "hand-r": {"angleDeg": 4}})
stand = {**relaxed_bones, **RIGHT_FINGERS, **TRAY_FINGERS, **TRAY_ARM, "torso": {"angleDeg": -1.0}, "lumbar": {"angleDeg": -0.8}}
serve = {**stand, "torso": {"angleDeg": -7.0}, "lumbar": {"angleDeg": -3.0}, "neck": {"angleDeg": -3}, "head": {"angleDeg": -6},
         "arm-r": {"angleDeg": 52, "abductionDeg": -14}, "forearm-r": {"angleDeg": 22, "twistDeg": -40}, "hand-r": {"angleDeg": -12},
         **figure.finger_pose("r", flex=0.2, spread=1.3),
         "thigh-l": {"angleDeg": 6}, "shank-l": {"angleDeg": -9}, "foot-l": {"angleDeg": 3}}
for bones in (walk_heel, walk_mid, stand, serve):
    bones.update(tray_level(bones))
LEFT_FLAT = ["heel-l", "ball-l", "heel-r", "ball-r"]
a["poses"] = [
    pose("walking-heel-strike", "Walking with the tray, left heel strike", "Gehen mit dem Tablett, linker Fersenkontakt",
         "Mid-gait at about 1.05 m/s: the left heel strikes, the right foot rolls off its forefoot and the free right arm swings forward; the left hand carries the tray palm-up at shoulder height and stays level.",
         "Mitten im Gang mit etwa 1,05 m/s: Die linke Ferse setzt auf, der rechte Fuß rollt über den Vorfuß ab und der freie rechte Arm schwingt nach vorn; die linke Hand trägt das Tablett mit der Handfläche nach oben auf Schulterhöhe und hält es waagrecht.",
         walk_heel, ["heel-l", "ball-r"], 2.7, "walking"),
    pose("walking-midstance", "Walking with the tray, midstance", "Gehen mit dem Tablett, Standbeinphase",
         "Right single support with the left leg swinging through; the tray rides level over the left shoulder.",
         "Rechte Einbeinstandphase, das linke Bein schwingt durch; das Tablett fährt waagrecht über der linken Schulter mit.",
         walk_mid, ["heel-r", "ball-r"], 2.7, "walking"),
    pose("standing-with-tray", "Standing with the tray", "Stehen mit dem Tablett",
         "Waiting at a table, both feet flat, the tray balanced on the spread fingers of the raised left hand.",
         "Wartend an einem Tisch, beide Füße flach, das Tablett auf den gespreizten Fingern der erhobenen linken Hand balanciert.",
         stand, LEFT_FLAT, 1.5, "quiet-standing"),
    pose("serving-gesture", "Serving with an open hand", "Servieren mit offener Hand",
         "A slight bow towards the guest, the free right hand open towards the table, the tray held level on the left.",
         "Eine leichte Verbeugung zum Gast, die freie rechte Hand offen zum Tisch, das Tablett links waagrecht gehalten.",
         serve, LEFT_FLAT, 1.6, "talking"),
]
a["presentation"] = {"defaultPose": "walking-midstance", "defaultClip": "walk-with-tray", "defaultScale": 100}


# ------------------------------------------------------------------ tray and glasses: bones placed on the palm in the default pose
probe = copy.deepcopy(doc)
for p in probe["asset"]["poses"]:
    p["bones"].pop("tray", None)
    p["loads"] = []
F = lib.frames(lib.write_tmp(probe, "t-waiter-probe.json"), "walking-heel-strike")
H = rot.frame(F["hand-l"])
ho = [F["hand-l"]["o"][c] for c in "xyz"]
PALM_U, PALM_W = 0.058 * K, -0.012 * K
palm = [ho[i] + H[i][0] * PALM_U + H[i][2] * PALM_W for i in range(3)]
TRAY_R, TRAY_H, TRAY_FLOOR = 0.18, 0.016, 0.004
target = rot.mat([[0, 1, 0], [-1, 0, 0], [0, 0, 1]])
rel = rot.mul(rot.T(H), target)
ang, abd, tw = rot.decompose(rel)
off = [sum(H[j][i] * (palm[j] - ho[j]) for j in range(3)) for i in range(3)]
lib.add_bone(a, {"id": "tray", "parent": "hand-l", "title": T("Round serving tray", "Rundes Serviertablett"), "x": lib.r5(off[0]), "y": lib.r5(off[1]), "z": lib.r5(off[2]),
                 "angleDeg": round(ang, 3), "abductionDeg": round(abd, 3), "twistDeg": round(tw, 3), "lengthM": TRAY_H,
                 "joint": {"type": "revolute", "minDeg": -20, "maxDeg": 20}, "mass": {"kg": 0.42, "comX": 0.006, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
             after="little-distal-l")
# glasses stand on the tray top (u = TRAY_H); tray v points backwards, w to the left
GLASSES = [
    ("beer-glass", T("Pilsner glass, 0.3 l beer", "Pilsglas, 0,3 l Bier"), -0.035, 0.045, 0.64),
    ("wine-glass", T("Wine glass, 0.15 l red wine", "Weinglas, 0,15 l Rotwein"), 0.055, -0.05, 0.30),
    ("water-tumbler", T("Tumbler, 0.25 l water", "Wasserglas, 0,25 l Wasser"), -0.07, -0.075, 0.52),
]
for gid, title, gv, gw, kg in GLASSES:
    lib.add_bone(a, {"id": gid, "parent": "tray", "title": title, "x": TRAY_FLOOR, "y": gv, "z": gw, "angleDeg": 0, "lengthM": 0.06,
                     "joint": {"type": "fixed"}, "mass": {"kg": kg, "comX": 0.06, "comY": 0, "distribution": "point", "source": "authored-estimate"}}, after="tray")
a["anchors"].append({"id": "tray-cog", "bone": "tray", "x": 0.06, "y": 0.0, "kind": "load", "title": T("Centre of gravity of the loaded tray", "Schwerpunkt des beladenen Tabletts")})
a["anchors"].append({"id": "tray-palm", "bone": "hand-l", "x": lib.r5(PALM_U), "y": 0, "z": lib.r5(PALM_W), "kind": "attachment", "title": T("Tray resting on the palm and fingers", "Tablett auf Handfläche und Fingern")})


# ------------------------------------------------------------------ tray and glasses drawn and modelled per scale (tray-local: u up)
TRAY_PROFILE = [[0, 0], [0, 0.172], [0.002, 0.178], [0.006, 0.18], [TRAY_H, 0.18], [TRAY_H, 0.174], [TRAY_FLOOR + 0.002, 0.171], [TRAY_FLOOR, 0.169], [TRAY_FLOOR, 0]]
GLASS = {
    "50": {"wine-glass": [[0, 0], [0, 0.034], [0.003, 0.034], [0.006, 0.009], [0.012, 0.0048], [0.07, 0.0048], [0.08, 0.016], [0.095, 0.033], [0.12, 0.042], [0.15, 0.041], [0.175, 0.037], [0.198, 0.034], [0.198, 0]],
           "water-tumbler": [[0, 0], [0, 0.032], [0.006, 0.033], [0.1, 0.037], [0.1, 0]],
           "beer-glass": [[0, 0], [0, 0.027], [0.012, 0.029], [0.16, 0.04], [0.185, 0.041], [0.185, 0]]},
    "100": {"wine-glass": [[0, 0], [0, 0.034], [0.006, 0.03], [0.014, 0.0075], [0.07, 0.0075], [0.09, 0.03], [0.12, 0.042], [0.17, 0.038], [0.198, 0.035], [0.198, 0]],
            "water-tumbler": [[0, 0], [0, 0.033], [0.1, 0.037], [0.1, 0]],
            "beer-glass": [[0, 0], [0, 0.028], [0.17, 0.04], [0.185, 0.041], [0.185, 0]]},
    "200": {"wine-glass": [[0, 0], [0, 0.034], [0.012, 0.017], [0.085, 0.017], [0.11, 0.042], [0.198, 0.037], [0.198, 0]],
            "water-tumbler": [[0, 0], [0, 0.035], [0.1, 0.037], [0.1, 0]],
            "beer-glass": [[0, 0], [0, 0.031], [0.185, 0.04], [0.185, 0]]},
}
LIQUID = {  # filled detail shapes: drink and foam inside the glass outline (1:50 and 1:100)
    "wine-glass": [("wine", [[0.082, 0], [0.082, 0.019], [0.095, 0.031], [0.122, 0.04], [0.122, 0]], 0.2)],
    "water-tumbler": [("water", [[0.008, 0], [0.008, 0.031], [0.085, 0.034], [0.085, 0]], 0.86)],
    "beer-glass": [("beer", [[0.014, 0], [0.014, 0.027], [0.15, 0.037], [0.15, 0]], 0.5), ("foam", [[0.15, 0], [0.15, 0.037], [0.175, 0.039], [0.175, 0]], 0.97)],
}
for sc in ("50", "100", "200"):
    lib.revolve_part(a, sc, "tray", "tray", TRAY_PROFILE, "tray-polymer", "tray", 0.55, 7.0, extra={"sheet": True} if sc == "200" else None)
    for n, (gid, _t, _v, _w, _kg) in enumerate(GLASSES):
        lib.revolve_part(a, sc, gid, gid, GLASS[sc][gid], "glass", "glassware", 0.94, 7.1 + 0.01 * n)
        if sc in ("50", "100"):
            for m, (lid, prof, tone) in enumerate(LIQUID[gid]):
                for view in ("elevation", "front"):
                    lib.shapes(a, sc, view).append({"id": f"{gid}-{lid}", "bone": gid, "role": "detail", "z": 7.15 + 0.01 * n + 0.001 * m, "filled": True, "tone": tone, "region": "glassware", "kind": "poly",
                                                    "points": [[lib.r5(u), lib.r5(r)] for u, r in prof], "closed": True, "solid": {"kind": "revolved", "axisBone": gid, "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}})
# 1:500: the tray and its glasses are one load: the disc and a typified cylinder of the glasses over its centre
LOAD = [[0, 0], [0, 0.18], [TRAY_H, 0.18], [TRAY_H, 0.115], [0.2, 0.105], [0.2, 0]]
lib.revolve_part(a, "500", "tray-load", "tray", LOAD, "tray-polymer", "tray", 0.5, 7.0, volume=False, regions=["glassware", "tray"])
lib.add_volume_part(a, "500", {"id": "tray", "kind": "revolve", "bone": "tray", "material": "tray-polymer", "role": "body", "tone": 0.5, "profile": [[0, 0], [0, 0.18], [TRAY_H, 0.18], [TRAY_H, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0], "sheet": True}, "tray")
lib.add_volume_part(a, "500", {"id": "glasses", "kind": "revolve", "bone": "tray", "material": "glass", "role": "body", "tone": 0.5, "profile": [[TRAY_H, 0], [TRAY_H, 0.115], [0.2, 0.105], [0.2, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, "glassware", member="tray")
PALM_R = {"50": 0.006, "100": 0.008, "200": 0.016, "500": 0.038}
# the palm carries the tray: a short member from the hand axis to the tray underside closes the load path at every scale
for sc in SCALES:
    a["volumes"][sc]["structure"].append({"id": "st-palm-tray", "kind": "capsule", "bone": "hand-l", "material": "bone", "role": "body", "tone": 0.5,
                                          "p0": [lib.r5(PALM_U), 0, 0], "p1": [lib.r5(PALM_U), 0, lib.r5(PALM_W - 0.002)], "r0": PALM_R[sc], "r1": PALM_R[sc], "member": "hand-l", "part": "member"})


# ------------------------------------------------------------------ clips
def keyed(t, pose_id, bones):
    k = {"t": round(t, 4), "pose": pose_id, "ease": "smooth"}
    if bones:
        k["bones"] = bones
    return k


walk_keys = []
for k in walk["keys"]:
    b = gait_bones(k["t"])
    b.update(tray_level(b))
    walk_keys.append(keyed(k["t"], "walking-heel-strike", b))
walk_clip = {
    "id": "walk-with-tray", "title": T("Walking with a loaded tray", "Gehen mit beladenem Tablett"),
    "description": T("Level walking at about 1.05 m/s and 100 steps per minute with the feet planted from heel strike to toe-off; the free right arm swings against the left leg, the left arm stays still and the tray is held level against the sway of the trunk. Joint angles of the legs and trunk from the archive's planted walking gait.",
                     "Gehen in der Ebene mit etwa 1,05 m/s und 100 Schritten je Minute, die Füße stehen vom Fersenkontakt bis zum Abdruck fest; der freie rechte Arm schwingt gegen das linke Bein, der linke Arm bleibt ruhig und das Tablett wird gegen das Wiegen des Rumpfs waagrecht gehalten. Gelenkwinkel von Beinen und Rumpf aus dem geplanten Gang des Archivs."),
    "durationS": walk["durationS"], "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": 1.0,
    "keys": walk_keys, "contacts": copy.deepcopy(walk["contacts"]), "phases": copy.deepcopy(walk["phases"]), "events": copy.deepcopy(walk["events"]),
    "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.7}, {"t": walk["durationS"], "v": 2.7}]},
               {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": walk["durationS"], "v": 1}]}],
}
idle_keys = []
for t, dt, dl, dh, ar, fr in ((0, 0, 0, 0, 0, 0), (1.25, -0.36, -0.24, 1.8, 2.5, 0), (2.5, 0.24, 0.16, 0.4, 0, 10.5), (3.75, -0.3, -0.2, 1.5, 2, 0), (5, 0, 0, 0, 0, 0)):
    b = {"torso": {"angleDeg": round(stand["torso"]["angleDeg"] + dt, 3)}, "lumbar": {"angleDeg": round(stand["lumbar"]["angleDeg"] + dl, 3)},
         "head": {"angleDeg": round(stand["head"]["angleDeg"] + dh, 3)}, "arm-r": {"angleDeg": round(stand["arm-r"]["angleDeg"] + ar, 2), "abductionDeg": stand["arm-r"].get("abductionDeg", 0)},
         "forearm-r": {"angleDeg": round(stand["forearm-r"]["angleDeg"] + fr, 2)}}
    b.update(tray_level(b))
    idle_keys.append(keyed(t, "standing-with-tray", b))
FEET = [{"anchor": x, "fromS": 0, "toS": None} for x in ("heel-l", "ball-l", "heel-r", "ball-r")]
idle_clip = {
    "id": "idle-breathing", "title": T("Waiting with the tray", "Warten mit dem Tablett"),
    "description": T("A five-second loop at the table: quiet breathing lifts the chest, the head and the free arm settle in a small sway while the tray stays level on the raised hand.",
                     "Eine Schleife von fünf Sekunden am Tisch: Ruhiges Atmen hebt die Brust, Kopf und freier Arm wiegen sich leicht, während das Tablett auf der erhobenen Hand waagrecht bleibt."),
    "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none", "keys": idle_keys,
    "contacts": [{**c, "toS": 5} for c in FEET], "phases": [{"id": "waiting", "fromS": 0, "toS": 5, "title": T("Waiting at the table", "Warten am Tisch")}], "events": [],
    "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.5}, {"t": 5, "v": 1.5}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}],
}
serve_keys = [keyed(0, "standing-with-tray", None), keyed(1.0, "serving-gesture", None), keyed(2.2, "serving-gesture", {"head": {"angleDeg": -2}, "neck": {"angleDeg": -1}, **tray_level(serve)}), keyed(3.4, "standing-with-tray", None)]
serve_clip = {
    "id": "serve-at-table", "title": T("Serving at the table", "Servieren am Tisch"),
    "description": T("From waiting with the tray the waiter bows slightly towards the guest, opens the free hand to the table while speaking and straightens again; the tray stays level throughout.",
                     "Aus dem Warten mit dem Tablett verbeugt sich der Kellner leicht zum Gast, öffnet beim Sprechen die freie Hand zum Tisch und richtet sich wieder auf; das Tablett bleibt dabei waagrecht."),
    "durationS": 3.4, "fps": 30, "loop": False, "rootMotion": "none", "keys": serve_keys,
    "contacts": [{**c, "toS": 3.4} for c in FEET],
    "phases": [{"id": "bow", "fromS": 0, "toS": 1.0, "title": T("Bowing to the guest", "Verbeugung zum Gast")}, {"id": "speak", "fromS": 1.0, "toS": 2.2, "title": T("Speaking with an open hand", "Sprechen mit offener Hand")}, {"id": "straighten", "fromS": 2.2, "toS": 3.4, "title": T("Straightening up", "Aufrichten")}],
    "events": [{"t": 1.0, "kind": "motion", "title": T("Hand open to the table", "Hand offen zum Tisch")}],
    "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.5}, {"t": 1.0, "v": 1.6}, {"t": 3.4, "v": 1.5}]},
               {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.0, "v": 2}, {"t": 2.2, "v": 0}, {"t": 3.4, "v": 0}]}],
}
a["clips"] = [walk_clip, idle_clip, serve_clip]


# ------------------------------------------------------------------ materials
a["materials"] = [
    {"id": "skin", "title": T("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "hair", "title": T("Hair", "Haar"), "emissivity": 0.98, "solarAbsorptivity": 0.85, "source": "authored-estimate"},
    {"id": "cotton-shirt", "title": T("White cotton poplin shirt", "Weißes Baumwollpopelinehemd"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "cotton-underwear", "title": T("Cotton briefs", "Baumwollunterhose"), "emissivity": 0.93, "solarAbsorptivity": 0.5, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "waistcoat-wool", "title": T("Black wool-blend waistcoat with satin back", "Schwarze Weste aus Wollmischgewebe mit Satinrücken"), "emissivity": 0.9, "solarAbsorptivity": 0.88, "specificHeatJKgK": 1360, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "trouser-wool", "title": T("Black wool-blend trousers", "Schwarze Hose aus Wollmischgewebe"), "emissivity": 0.9, "solarAbsorptivity": 0.85, "specificHeatJKgK": 1360, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "apron-cotton", "title": T("Black cotton twill bistro apron", "Schwarze Bistroschürze aus Baumwollköper"), "emissivity": 0.9, "solarAbsorptivity": 0.88, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "cotton-sock", "title": T("Black cotton socks", "Schwarze Baumwollsocken"), "emissivity": 0.93, "solarAbsorptivity": 0.85, "specificHeatJKgK": 1300, "source": "authored-estimate"},
    {"id": "leather-shoe", "title": T("Black leather lace-up shoe with rubber sole", "Schwarzer Lederschnürschuh mit Gummisohle"), "emissivity": 0.95, "solarAbsorptivity": 0.85, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "tray-polymer", "title": T("Polypropylene tray with rubberised non-slip top", "Polypropylentablett mit gummierter Antirutschoberfläche"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "specificHeatJKgK": 1900, "densityKgM3": 950, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "glass", "title": T("Soda-lime glassware filled with drinks", "Kalk-Natron-Gläser mit Getränken"), "emissivity": 0.92, "solarAbsorptivity": 0.15, "specificHeatJKgK": 3600, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "bone", "title": T("Bone", "Knochen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1900, "source": "authored-estimate"},
    {"id": "costal-cartilage", "title": T("Costal cartilage", "Rippenknorpel"), "emissivity": 0.96, "solarAbsorptivity": 0.5, "source": "authored-estimate"},
]

# ------------------------------------------------------------------ thermal model
L = figure.layer
regions_old = {r["id"]: r for r in a["thermal"]["regions"]}
face, hair, hands = regions_old["face-skin"], regions_old["hair"], regions_old["hand-skin"]
a["thermal"]["regions"] = [
    face, hair, hands,
    figure.region("waistcoat", "White shirt and black waistcoat over the trunk", "Weißes Hemd und schwarze Weste über dem Rumpf", "clothing", "body", "waistcoat-wool", 0.6, 0.3, "vertical-plate", 0.45,
                  [L("shirt", "Cotton poplin shirt", "Baumwollpopelinehemd", "cotton-shirt", 0.0004, 0.25, 160), L("waistcoat", "Lined wool-blend waistcoat", "Gefütterte Weste aus Wollmischgewebe", "waistcoat-wool", 0.0025, 0.2, 420)],
                  13600, "trunk", 0.38, 0.31),
    figure.region("shirt", "White shirt sleeves", "Weiße Hemdsärmel", "clothing", "body", "cotton-shirt", 0.3, 0.08, "cylinder", 0.35,
                  [L("shirt", "Cotton poplin shirt", "Baumwollpopelinehemd", "cotton-shirt", 0.0004, 0.25, 160)], 13200, "arm", 0.17, 0.18),
    figure.region("trousers", "Black trousers", "Schwarze Hose", "clothing", "body", "trouser-wool", 0.5, 0.13, "cylinder", 0.1,
                  [L("briefs", "Cotton briefs", "Baumwollunterhose", "cotton-underwear", 0.0008, 0.04, 100), L("trousers", "Wool-blend trousers", "Hose aus Wollmischgewebe", "trouser-wool", 0.0012, 0.24, 380)],
                  13500, "leg", 0.22, 0.24),
    figure.region("apron", "Long black bistro apron over the thighs", "Lange schwarze Bistroschürze über den Oberschenkeln", "clothing", "body", "apron-cotton", 0.2, 0.3, "vertical-plate", 0.2,
                  [L("apron", "Cotton twill apron over the wool-blend trousers", "Schürze aus Baumwollköper über der Hose aus Wollmischgewebe", "apron-cotton", 0.0019, 0.30, 640)],
                  13700, "leg", 0.05, 0.06),
    figure.region("shoes", "Socks and leather shoes", "Socken und Lederschuhe", "surface", "body", "leather-shoe", 0.15, 0.09, "horizontal-plate-up", 0.1,
                  [L("socks", "Cotton socks", "Baumwollsocken", "cotton-sock", 0.0015, 0.02, 100), L("shoe", "Leather shoe", "Lederschuh", "leather-shoe", 0.003, 0.04, 700)],
                  13800, "foot", 0.03, 0.06),
    figure.region("tray", "Serving tray", "Serviertablett", "surface", "passive", "tray-polymer", 0.2, 0.36, "horizontal-plate-up", 0.4, [], 4000, ref="ashrae-2021"),
    figure.region("glassware", "Three glasses with beer, wine and water", "Drei Gläser mit Bier, Wein und Wasser", "surface", "passive", "glass", 0.12, 0.08, "cylinder", 0.5, [], 42000, ref="ashrae-2021"),
]
for r in (face, hair, hands):
    r["characteristicLengthM"] = r["characteristicLengthM"]
for sc in SCALES:
    for s in a["volumes"][sc]["thermal"]:
        if s["id"].startswith("th-apron-"):
            s["layer"] = "apron"
a["thermal"]["environment"] = {"airTempC": 22, "meanRadiantTempC": 22, "skyTempC": 22, "groundTempC": 22, "relativeHumidity": 0.45, "windMS": 0.15, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
a["thermal"]["baseMet"] = 1.5
skin = figure.physiology(a, SEX, STATURE, BODY_KG, AGE)
a["thermal"]["emitters"] = [e for e in a["thermal"]["emitters"]]
for e in a["thermal"]["emitters"]:
    if e["id"] == "body-plume":
        e["region"] = "waistcoat"
a["thermal"]["assumptions"] = [
    T(f"Skin area {skin} m2 after DuBois and DuBois (1916) for {BODY_KG:.0f} kg and {STATURE} m; skin fractions 0.07 face and neck, 0.03 scalp, 0.05 hands, 0.31 trunk under the waistcoat, 0.18 arms in shirt sleeves, 0.24 legs in trousers, 0.06 thighs under the apron, 0.06 feet (rule-of-nines proportions, authored).",
      f"Hautfläche {skin} m2 nach DuBois und DuBois (1916) für {BODY_KG:.0f} kg und {STATURE} m; Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,31 Rumpf unter der Weste, 0,18 Arme in Hemdsärmeln, 0,24 Beine in der Hose, 0,06 Oberschenkel unter der Schürze, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
    T("Service clothing of about 0.75 clo (shirt, lined waistcoat, trousers, apron over the thighs, socks and leather shoes) after the garment tables of ISO 9920 (1 clo = 0.155 m2K/W), as local layer insulation per region; vapour resistance R/(0.0165 K/Pa x 0.38). The default surroundings are a restaurant at 22 degrees C with slight air movement.",
      "Servicekleidung von etwa 0,75 clo (Hemd, gefütterte Weste, Hose, Schürze über den Oberschenkeln, Socken und Lederschuhe) nach den Kleidungstabellen von ISO 9920 (1 clo = 0,155 m2K/W), als örtliche Schichtisolation je Region; Dampfwiderstand R/(0,0165 K/Pa x 0,38). Die Standardumgebung ist ein Restaurant mit 22 Grad C und leichter Luftbewegung."),
    T("Activity: walking with a loaded tray at about 3.8 km/h is 2.7 met (ISO 8996 Annex B and the Ainsworth compendium, code 11795 waiter, food server, with walking), waiting with the raised tray 1.5 met and serving with speech 1.6 met (static arm load added to standing, authored). The basal metabolism follows Mifflin-St Jeor for a man of 31 years.",
      "Aktivität: Gehen mit beladenem Tablett mit etwa 3,8 km/h sind 2,7 met (ISO 8996 Anhang B und Ainsworth-Kompendium, Code 11795 Kellner, Servicekraft, mit Gehen), Warten mit erhobenem Tablett 1,5 met und Servieren mit Sprechen 1,6 met (statische Armlast zum Stehen addiert, geschätzt). Der Grundumsatz folgt Mifflin-St Jeor für einen Mann von 31 Jahren."),
    T("The tray and the glasses are passive: their heat capacity holds the drinks near the bar temperature over a service round; they draw no power.",
      "Tablett und Gläser sind passiv: Ihre Wärmekapazität hält die Getränke über einen Servicegang nahe der Thekentemperatur; sie haben keine Leistungsaufnahme."),
]
a["structure"]["supports"] = []
a["structure"]["dynamics"] = "quasi-static"
for sc in SCALES:
    a["volumes"][sc]["structure"] = [x for x in a["volumes"][sc]["structure"] if x.get("part") != "support"]
a["structure"]["assumptions"] = [
    T("Segments are point masses at the de Leva (1996) centres of the male table, 76.1 kg with clothing; the tray (0.42 kg) and the three filled glasses (1.46 kg) are bones of their own on the left hand and act also as a carried load at their common centre of gravity.",
      "Segmente sind Punktmassen in den Schwerpunkten nach de Leva (1996), Männertabelle, 76,1 kg mit Kleidung; Tablett (0,42 kg) und die drei gefüllten Gläser (1,46 kg) sind eigene Knochen an der linken Hand und wirken zugleich als getragene Last in ihrem gemeinsamen Schwerpunkt."),
    T("Contacts are heel and ball of each foot; supports follow the clip contact spans. The walking loop is solved quasi-statically: the gait keys are transplanted to a body with a shorter foot, so the planted root path has a small jump at the loop seam that an inertial solution would read as a false impact. The palm carries the tray through a short member from the hand axis to the tray underside.",
      "Kontakte sind Ferse und Ballen jedes Fußes; Auflager folgen den Kontaktspannen des Clips. Die Gehschleife wird quasistatisch gelöst: Die Gangschlüssel sind auf einen Körper mit kürzerem Fuß übertragen, der geplante Wurzelpfad hat daher an der Schleifennaht einen kleinen Sprung, den eine Lösung mit Trägheit als falschen Stoß läse. Die Handfläche trägt das Tablett über einen kurzen Stab von der Handachse zur Tablettunterseite."),
]


# ------------------------------------------------------------------ measured quantities: advance, dimensions, region areas, mass
walk_clip["advanceM"] = lib.advance_of(doc, "walk-with-tray") or walk_clip["advanceM"]
walk_clip["advanceM"] = lib.advance_of(doc, "walk-with-tray") or walk_clip["advanceM"]
dims = lib.measure_dims(doc)
a["dimensions"] = {"heightM": dims["h"][0], "widthM": dims["w"][0], "depthM": dims["d"][0], "massKg": round(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if b.get("mass")), 2)}
AREAS = figure.calibrate_areas(doc, {"waistcoat", "shirt", "trousers", "apron", "shoes", "tray", "glassware", "face-skin", "hair", "hand-skin"})


# ------------------------------------------------------------------ texts, provenance, decisions
a["title"] = T("Waiter with Tray", "Servicekraft mit Tablett")
a["summary"] = T(
    "A 31-year-old waiter of 1.78 m in a white shirt, black waistcoat, black trousers, a long black bistro apron and leather shoes walks through the restaurant with a round tray held palm-up at shoulder height on the left hand, carrying a glass of beer, a glass of red wine and a tumbler of water; the free right arm swings with the gait. Poses for walking, waiting with the tray and serving with an open hand; a planted walking loop with the tray held level, a waiting loop and a serving sequence.",
    "Ein 31-jähriger Kellner von 1,78 m in weißem Hemd, schwarzer Weste, schwarzer Hose, langer schwarzer Bistroschürze und Lederschuhen geht durch das Restaurant und trägt auf der linken Hand mit nach oben gewandter Handfläche auf Schulterhöhe ein rundes Tablett mit einem Glas Bier, einem Glas Rotwein und einem Glas Wasser; der freie rechte Arm schwingt im Gang mit. Posen für Gehen, Warten mit dem Tablett und Servieren mit offener Hand; eine geplante Gehschleife mit waagrecht gehaltenem Tablett, eine Warteschleife und ein Serviervorgang.")
a["tags"] = ["human", "adult", "walking", "hospitality", "restaurant", "waiter", "carrying", "tray", "service"]
prov = a["provenance"]
prov["author"] = "Semio staffage human worker HW-02"
prov["method"] = "handcrafted"
prov["created"] = "2026-11-22"
refs = {r["id"]: r for r in prov["references"]}
refs["iso-8996-2021"] = {"id": "iso-8996-2021", "citation": "ISO 8996:2021 Ergonomics of the thermal environment, determination of metabolic rate; Annex B metabolic rates for activities and Annex C walking speeds."}
refs["ainsworth-2011"] = {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; occupation codes 11790-11795 (food service, waiter or waitress)."}
refs["perry-burnfield-2010"] = {"id": "perry-burnfield-2010", "citation": "Perry, J. and Burnfield, J. M. (2010). Gait Analysis: Normal and Pathological Function, 2nd ed., SLACK; phases, timing and joint ranges of level walking."}
prov["references"] = list(refs.values())
prov["notes"] = T(
    "Derived by hand from the archive's standing-adult (rig, outlines and per-scale volumes scaled uniformly by 1.78/1.69 = 1.053) with the hood, kangaroo pocket, drawstrings and rib hem removed; the sweatshirt became a white shirt (sleeves) under a black waistcoat (trunk), the jeans black trousers, the sneakers leather shoes. Segment masses re-derived from de Leva's male table for 74 kg body plus 2.1 kg clothing. The legs and trunk walk with the joint angles of the walking-commuter's planted gait (two-link inverse kinematics of smooth foot paths, 100 steps per minute), the free right arm swings with the commuter's far-arm curve shifted by half a cycle. Added by hand: the bistro apron as two panels blended 55/45 between pelvis and thigh so that it hangs between the legs in the stride, the shirt front, placket and welt pockets of the waistcoat, the 0.36 m tray on the left palm with three glasses on their own fixed bones (tray rest placement solved so that it is level in the default pose; every pose and clip key tilts it back against the lean of the trunk), the arms drawn as bodies of revolution in every view, the coarse trunk outlines from the trunk body of the same scale.",
    "Von Hand aus dem Archivobjekt standing-adult abgeleitet (Rig, Umrisse und Volumen je Maßstab gleichmäßig mit 1,78/1,69 = 1,053 skaliert), Kapuze, Kängurutasche, Kordeln und Rippensaum entfernt; aus dem Sweatshirt wurden ein weißes Hemd (Ärmel) unter einer schwarzen Weste (Rumpf), aus der Jeans eine schwarze Hose, aus den Sneakern Lederschuhe. Segmentmassen aus der Männertabelle von de Leva für 74 kg Körper und 2,1 kg Kleidung neu bestimmt. Beine und Rumpf gehen mit den Gelenkwinkeln des geplanten Gangs von walking-commuter (zweigliedrige inverse Kinematik glatter Fußbahnen, 100 Schritte je Minute), der freie rechte Arm schwingt mit der um einen halben Zyklus verschobenen Kurve des fernen Arms. Von Hand ergänzt: die Bistroschürze als zwei Bahnen, 55/45 zwischen Becken und Oberschenkel gemischt, damit sie im Schritt zwischen den Beinen hängt, Hemdbrust, Knopfleiste und Paspeltaschen der Weste, das Tablett von 0,36 m auf der linken Handfläche mit drei Gläsern auf eigenen festen Knochen (Ruhelage des Tabletts so bestimmt, dass es in der Standardpose waagrecht liegt; jede Pose und jeder Clip-Schlüssel kippt es gegen die Neigung des Rumpfs zurück), die Arme in jeder Ansicht als Drehkörper, die groben Rumpfumrisse aus dem Rumpfkörper desselben Maßstabs.")

DEC = {
    "50": dict(
        silhouette=T("The three views of the 1:50 body in the walking midstance, 36 mm tall: head with ear, eye, nose and mouth under the short hair, neck with the white collar, black waistcoat with the white shirt front in its V, white shirt sleeves and cuffs, hands with thumb and fingers, black trousers, the long apron over the thighs, leather shoes with sole and toe cap; on the left hand the round tray with a wine glass (bowl, stem and foot), a tumbler and a pilsner glass. Inner lines: placket, welt pockets and buttons of the waistcoat, drink levels and foam, laces, hems.",
                     "Die drei Ansichten des 1:50-Körpers in der Standbeinphase des Gehens, 36 mm hoch: Kopf mit Ohr, Auge, Nase und Mund unter dem kurzen Haar, Hals mit weißem Kragen, schwarze Weste mit weißer Hemdbrust im Ausschnitt, weiße Hemdsärmel und Manschetten, Hände mit Daumen und Fingern, schwarze Hose, die lange Schürze über den Oberschenkeln, Lederschuhe mit Sohle und Zehenkappe; auf der linken Hand das runde Tablett mit Weinglas (Kelch, Stiel und Fuß), Wasserglas und Pilsglas. Innenlinien: Knopfleiste, Paspeltaschen und Knöpfe der Weste, Füllhöhen und Schaum, Schnürung, Säume."),
        retained=T("Everything of 1:100 plus the stem and foot of the wine glass, the waistcoat buttons and the finger separations.", "Alles aus 1:100 und dazu Stiel und Fuß des Weinglases, die Westenknöpfe und die Fingertrennungen."),
        merged=T("Brows, lips and chin into the head outline; the drinks are drawn inside their glasses.", "Brauen, Lippen und Kinn im Kopfumriss; die Getränke sind in ihren Gläsern gezeichnet."),
        omitted=T("Apron ties, shirt buttons, stitching, fabric texture; the hand under the tray is mostly hidden by it.", "Schürzenbänder, Hemdknöpfe, Nähte, Gewebestruktur; die Hand unter dem Tablett ist größtenteils verdeckt."),
        thinElements=T("The 9 mm stem of the wine glass is drawn at its true width (0.18 mm); buttons and laces are 0.10 mm texture lines.", "Der 9 mm starke Stiel des Weinglases ist in wahrer Breite gezeichnet (0,18 mm); Knöpfe und Schnürung sind Texturlinien von 0,10 mm."),
        hatching=T("None: the black cloth reads by tone, the glass by its light fill and the drink levels.", "Keine: das schwarze Tuch liest sich über den Tonwert, das Glas über seine helle Füllung und die Füllhöhen."),
        styles=T("Dark waistcoat, trousers and apron against white sleeves and shirt front, light glass with darker drinks; body pen 0.25 mm, detail 0.13 mm, texture 0.10 mm; dark mode inverts the tones.", "Dunkle Weste, Hose und Schürze gegen weiße Ärmel und Hemdbrust, helles Glas mit dunkleren Getränken; Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; der dunkle Modus kehrt die Tonwerte um."),
        overlays=T("Every shape carries one thermal region: waistcoat, shirt, trousers, apron, shoes, skin and hair; tray and glassware are passive regions; the structural views colour the bones the shapes ride on.", "Jede Form trägt eine thermische Region: Weste, Hemd, Hose, Schürze, Schuhe, Haut und Haar; Tablett und Gläser sind passive Regionen; die statischen Ansichten färben die Knochen, auf denen die Formen sitzen.")),
    "100": dict(
        silhouette=T("The three views of the 1:100 body, 18 mm tall: head with nose and ear under the hair, neck, waistcoat with the white V of the shirt, shirt sleeves, hands, trousers, apron panels, shoes and toe boxes; the tray with three simple glass profiles and their drink levels.",
                     "Die drei Ansichten des 1:100-Körpers, 18 mm hoch: Kopf mit Nase und Ohr unter dem Haar, Hals, Weste mit dem weißen Ausschnitt des Hemds, Hemdsärmel, Hände, Hose, Schürzenbahnen, Schuhe und Zehenkappen; das Tablett mit drei einfachen Glasprofilen und ihren Füllhöhen."),
        retained=T("Placket and welt pockets of the waistcoat, the drinks, the glass shapes and the gap between the legs.", "Knopfleiste und Paspeltaschen der Weste, die Getränke, die Glasformen und der Spalt zwischen den Beinen."),
        merged=T("Fingers into a mitten with thumb; the wine-glass foot and stem into one stem of 15 mm.", "Finger zu einem Fäustling mit Daumen; Fuß und Stiel des Weinglases zu einem Stiel von 15 mm."),
        omitted=T("Buttons, laces, finger separations, foam line.", "Knöpfe, Schnürung, Fingertrennungen, Schaumlinie."),
        thinElements=T("The wine-glass stem is thickened from 9 to 15 mm so that it prints at 0.15 mm; the tray rim is drawn at its true 16 mm.", "Der Stiel des Weinglases ist von 9 auf 15 mm verdickt, damit er mit 0,15 mm druckt; der Tablettrand ist in seinen wahren 16 mm gezeichnet."),
        hatching=T("None.", "Keine."),
        styles=T("As 1:50 with body pen 0.20 mm and detail 0.10 mm, no texture pen.", "Wie 1:50 mit Körperstift 0,20 mm und Detail 0,10 mm, ohne Texturstift."),
        overlays=T("As 1:50.", "Wie 1:50.")),
    "200": dict(
        silhouette=T("The three views of the 1:200 body, 9 mm tall, one mass per part: head, hair cap, neck, waistcoat with the white V, sleeves, forearms, hands, trousers, apron panels, shoes; the tray and three glass masses on it.",
                     "Die drei Ansichten des 1:200-Körpers, 9 mm hoch, eine Masse je Teil: Kopf, Haarkappe, Hals, Weste mit weißem Ausschnitt, Ärmel, Unterarme, Hände, Hose, Schürzenbahnen, Schuhe; das Tablett und drei Glasmassen darauf."),
        retained=T("The raised tray arm, the level tray with its glasses, the white shirt front and the dark apron that make the waiter.", "Der erhobene Tablettarm, das waagrechte Tablett mit seinen Gläsern, die weiße Hemdbrust und die dunkle Schürze, die den Kellner ausmachen."),
        merged=T("Glass and drink into one mass per glass; wine glass bowl and stem into a goblet of 34 mm stem width.", "Glas und Getränk zu einer Masse je Glas; Kelch und Stiel des Weinglases zu einem Pokal mit 34 mm Stielbreite."),
        omitted=T("Placket, pockets, drink levels, cuffs.", "Knopfleiste, Taschen, Füllhöhen, Manschetten."),
        thinElements=T("The 16 mm tray is drawn at its true thickness with the 0.18 mm body pen; it is declared a sheet in the volume.", "Das 16 mm starke Tablett ist in wahrer Dicke mit dem Körperstift von 0,18 mm gezeichnet; im Volumen ist es als Platte erklärt."),
        hatching=T("None.", "Keine."),
        styles=T("Body pen 0.18 mm, detail 0.10 mm; trunk outlines are the views of the trunk body of this scale.", "Körperstift 0,18 mm, Detail 0,10 mm; die Rumpfumrisse sind die Ansichten des Rumpfkörpers dieses Maßstabs."),
        overlays=T("As 1:50.", "Wie 1:50.")),
    "500": dict(
        silhouette=T("An upright walking figure of 1.80 m (3.6 mm), the views of the 1:500 body: head with hair, neck, trunk, apron, and per side sleeve, forearm, hand, thigh, shank and shoe; on the raised left hand the tray as a disc with the glasses typified as one cylinder.",
                     "Eine aufrecht gehende Figur von 1,80 m (3,6 mm), die Ansichten des 1:500-Körpers: Kopf mit Haar, Hals, Rumpf, Schürze und je Seite Ärmel, Unterarm, Hand, Oberschenkel, Unterschenkel und Schuh; auf der erhobenen linken Hand das Tablett als Scheibe mit den Gläsern als ein typisierter Zylinder."),
        retained=T("The raised tray over the shoulder, the stride, stature and extent.", "Das erhobene Tablett über der Schulter, die Schrittstellung, Körpergröße und Ausdehnung."),
        merged=T("Face and hair; the three glasses into one load cylinder of 0.21 m diameter on the tray (shape listing tray and glassware).", "Gesicht und Haar; die drei Gläser zu einem Lastzylinder von 0,21 m Durchmesser auf dem Tablett (die Form nennt Tablett und Gläser)."),
        omitted=T("Shirt front, waistcoat details, fingers, drink levels.", "Hemdbrust, Westendetails, Finger, Füllhöhen."),
        thinElements=T("Glass stems and the tray rim are not drawn separately, never thickened.", "Glasstiele und Tablettrand werden nicht einzeln gezeichnet, nie verdickt."),
        hatching=T("None.", "Keine."),
        styles=T("One pen of 0.13 mm; dark trunk, legs and apron, light sleeves.", "Ein Stift von 0,13 mm; dunkler Rumpf, Beine und Schürze, helle Ärmel."),
        overlays=T("The tray load lists both passive regions, tray and glassware; the other shapes keep one region each.", "Die Tablettlast nennt beide passiven Regionen, Tablett und Gläser; die übrigen Formen behalten je eine Region.")),
}
for sc in SCALES:
    a["representations"][sc]["decisions"] = DEC[sc]
VOLDEC = {
    "50": T("Body at 1:50: the standing adult's lofted body (head with face, ears and hair cap, neck, trunk, sleeves, forearms, hands with every finger phalanx, trousers, shoes) scaled to 1.78 m, without hood, pocket, drawstrings and rib hem; the apron as two 19 mm panels skinned 55/45 to pelvis and thigh; the polypropylene tray as a revolved dish with a 16 mm rim; three glasses as revolved solids on their own bones (wine glass with bowl, stem and foot, tumbler, pilsner glass), filled with their drinks.",
            "Körper bei 1:50: der geloftete Körper der stehenden erwachsenen Person (Kopf mit Gesicht, Ohren und Haarkappe, Hals, Rumpf, Ärmel, Unterarme, Hände mit jedem Fingerglied, Hose, Schuhe) auf 1,78 m skaliert, ohne Kapuze, Tasche, Kordeln und Rippensaum; die Schürze als zwei Bahnen von 19 mm, 55/45 an Becken und Oberschenkel gebunden; das Polypropylentablett als gedrehte Schale mit 16 mm Rand; drei Gläser als Drehkörper auf eigenen Knochen (Weinglas mit Kelch, Stiel und Fuß, Wasserglas, Pilsglas), mit ihren Getränken gefüllt."),
    "100": T("Body at 1:100 with fewer sections for a 15 mm feature; apron panels and tray as at 1:50, the wine-glass stem 15 mm thick.", "Körper bei 1:100 mit weniger Schnitten für ein Merkmal von 15 mm; Schürzenbahnen und Tablett wie bei 1:50, der Stiel des Weinglases 15 mm stark."),
    "200": T("Body at 1:200, one mass per part for a 30 mm feature; the tray declared a sheet of 16 mm, the glasses as simple goblet, tumbler and tapered beer glass.", "Körper bei 1:200, eine Masse je Teil für ein Merkmal von 30 mm; das Tablett als Platte von 16 mm erklärt, die Gläser als einfacher Pokal, Becher und konisches Bierglas."),
    "500": T("Body at 1:500, the fewest masses for a 75 mm feature; the tray a 16 mm sheet disc and the glasses one typified cylinder of 0.21 m diameter and 0.18 m height on it; the bent wrist of the tray hand has its own member.", "Körper bei 1:500, die wenigsten Massen für ein Merkmal von 75 mm; das Tablett eine Plattenscheibe von 16 mm und die Gläser darauf ein typisierter Zylinder von 0,21 m Durchmesser und 0,18 m Höhe; das abgewinkelte Handgelenk der Tablett-Hand hat einen eigenen Stab."),
}
THERMDEC = {
    "50": T("Thermal at 1:50: the outer garment, skin, tray and glass surfaces are the body solids themselves; inside lie the shirt as a shell under the waistcoat, the briefs under the trousers and the socks in the shoes. The apron panels are the apron region over the trousers; tray and glassware are passive regions without power.",
            "Thermik bei 1:50: Die äußeren Kleidungs-, Haut-, Tablett- und Glasflächen sind die Körper selbst; innen liegen das Hemd als Schale unter der Weste, die Unterhose unter der Hose und die Socken in den Schuhen. Die Schürzenbahnen sind die Schürzenregion über der Hose; Tablett und Gläser sind passive Regionen ohne Leistung."),
    "100": T("Thermal at 1:100: every region lies on the body solids of this scale, the layers of a garment merged into its outer surface; the three outlets are patches of one smallest feature.", "Thermik bei 1:100: Jede Region liegt auf den Körpern dieses Maßstabs, die Schichten eines Kleidungsstücks in seiner Außenfläche zusammengefasst; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
    "200": T("Thermal at 1:200: every region lies on the masses of this scale, layers merged; the three outlets are patches of one smallest feature.", "Thermik bei 1:200: Jede Region liegt auf den Massen dieses Maßstabs, Schichten zusammengefasst; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
    "500": T("Thermal at 1:500: every region lies on the masses of this scale, the glassware on the typified load cylinder; the three outlets are patches of one smallest feature.", "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, die Gläser auf dem typisierten Lastzylinder; die drei Auslässe sind Flecken von einem kleinsten Merkmal."),
}
for sc in SCALES:
    a["volumes"][sc]["decisions"] = [VOLDEC[sc], THERMDEC[sc], a["volumes"][sc]["decisions"][2]]

if __name__ == "__main__" and "--stage" in sys.argv:
    lib.write_tmp(doc, "waiter-stage.json")
    print(dims, AREAS, walk_clip["advanceM"])
elif __name__ == "__main__":
    print(lib.write_asset(doc, SLUG), dims, AREAS, walk_clip["advanceM"])
