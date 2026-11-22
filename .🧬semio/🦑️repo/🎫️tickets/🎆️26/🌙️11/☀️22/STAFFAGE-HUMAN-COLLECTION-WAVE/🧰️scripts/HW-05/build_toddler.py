"""🧸️ Builds `👤️humans/walking-toddler`: a two-year-old walking with the wide-based, short-stepped gait of a young walker.

Derived by hand from the archive's `running-child` (a six-year-old) with per-segment factors from six back to two years (head 0.92, neck 0.75,
trunk 0.84, arms 0.70, legs 0.68), so the head becomes a fifth of the stature and the hip joints sit at 0.45 of it; the backpack is
removed, the shorts sit over a nappy, and a new immature gait (Sutherland et al. 1980: cadence about 170 steps per minute, flat-foot
contact, wide base, arms held out in a middle guard) is solved by inverse kinematics of the shrunk rig.

Run: python3 build_toddler.py
@see ./lib.py ./gait.py ./tools.py ./texts_toddler.py
"""
import copy

from gait import solve_cycle, walk_clip
from lib import T, affine_bone, load, r, rescale
from tools import dubois, drop_shapes, feature_loop, save, set_dims, shapes, solids, summarize

SLUG = "walking-toddler"
STATURE = 0.87
BODY_KG = 12.0
CLOTHES_KG = 0.35

F = {"head": 0.92, "neck": 0.75, "pelvis": 0.84, "torso": 0.84, "pack": 0.84, "clavicle-l": 0.84, "clavicle-r": 0.84, "ilium-l": 0.84, "ilium-r": 0.84}
for side in "lr":
    for b in ("thigh", "shank", "foot", "toe"):
        F[f"{b}-{side}"] = 0.68
    for b in ("arm", "forearm", "hand"):
        F[f"{b}-{side}"] = 0.70

GAIT = dict(T=0.72, stride=0.44, footFlat=0.06, heelOff=0.46, toeOff=0.66, pitchStrike=3, pitchOff=-16, pitchSwing=4, clearance=0.03, clearancePeak=0.4, midStance=0.33, hipAhead=0.03, kneeMid=16, kneeMin=7)
GAIT["speed"] = GAIT["stride"] / GAIT["T"]
N_KEYS = 16
SPREAD = 5


def remove_backpack(doc):
    a = doc["asset"]
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] != "pack"]
    drop_shapes(doc, lambda s: s["bone"] == "pack" or s.get("region") == "backpack" or "backpack" in s.get("regions", []))
    for vol in a["volumes"].values():
        body_ids = {s["id"] for s in vol["body"] if s.get("bone") == "pack" or s.get("material") == "nylon-pack"}
        vol["body"] = [s for s in vol["body"] if s["id"] not in body_ids]
        vol["thermal"] = [s for s in vol["thermal"] if s.get("region") != "backpack" and s.get("of") not in body_ids]
        vol["structure"] = [s for s in vol["structure"] if s.get("bone") != "pack" and s.get("member") != "pack" and s.get("of") not in body_ids]
    a["thermal"]["regions"] = [reg for reg in a["thermal"]["regions"] if reg["id"] != "backpack"]
    a["materials"] = [m for m in a["materials"] if m["id"] != "nylon-pack"]
    for s in [x for x in a["symbols"]]:
        pass


def nappy(doc):
    sel = lambda s: s.get("region") == "shorts" or s.get("material") == "cotton-short" or s["id"].startswith("shorts")
    affine_bone(doc, "pelvis", (0.0, 0.0, 0.0), (1.0, 1.14, 1.08), select=sel)


def masses(doc):
    a = doc["asset"]
    total = 0.0
    for b in a["rig"]["bones"]:
        if "mass" in b:
            b["mass"]["kg"] = b["mass"]["kg"] * F[b["id"]] ** 3
            total += b["mass"]["kg"]
    k = (BODY_KG + CLOTHES_KG) / total
    for b in a["rig"]["bones"]:
        if "mass" in b:
            b["mass"]["kg"] = r(b["mass"]["kg"] * k, 3)
            b["mass"]["reference"] = "schneider-1992"
    a["dimensions"]["massKg"] = r(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b), 2)


def materials(doc):
    for m in doc["asset"]["materials"]:
        if m["id"] == "skin":
            m["title"] = T("Toddler skin", "Kleinkindhaut")
        if m["id"] == "cotton-short":
            m["title"] = T("Cotton shorts over a disposable nappy", "Baumwollshorts über einer Wegwerfwindel")
        if m["id"] == "canvas-sneaker":
            m["title"] = T("Soft first-walker shoe", "Weicher Lauflernschuh")
        if m["id"] == "cotton-jersey":
            m["title"] = T("Cotton jersey T-shirt", "Baumwolljersey-T-Shirt")
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"].startswith("foot-"):
            b["title"] = T(("Left" if b["id"].endswith("l") else "Right") + " foot in a first-walker shoe", ("Linker" if b["id"].endswith("l") else "Rechter") + " Fuß im Lauflernschuh")
        if b["id"] == "pelvis":
            b["title"] = T("Pelvis with nappy", "Becken mit Windel")
        if b["id"] == "head":
            b["title"] = T("Head with fine hair", "Kopf mit feinem Haar")


def thermal(doc):
    a = doc["asset"]
    t = a["thermal"]
    t["environment"] = {"airTempC": 22, "meanRadiantTempC": 23, "skyTempC": 12, "groundTempC": 26, "relativeHumidity": 0.5, "windMS": 0.3, "windDirectionDeg": 180, "solarWM2": 300, "diffuseFraction": 0.35, "pressurePa": 101325}
    t["baseMet"] = 1.4
    area = dubois(BODY_KG, STATURE)
    t["body"].update({"massKg": BODY_KG, "skinAreaM2": r(area, 3), "basalMetabolismW": 32.7, "source": "authored-estimate", "reference": "schofield-1985"})
    spec = {
        "face-skin": (0.075, 0.11, 1.0, T("Face and neck skin", "Haut von Gesicht und Hals")),
        "hair": (0.055, 0.03, 1.0, T("Fine toddler hair", "Feines Kleinkindhaar")),
        "arm-skin": (0.10, 0.05, 1.0, T("Bare forearms and hands", "Nackte Unterarme und Hände")),
        "leg-skin": (0.17, 0.13, 1.0, T("Bare legs", "Nackte Beine")),
        "tshirt": (0.33, 0.52, 1.1, T("Bodysuit and T-shirt on trunk and upper arms", "Body und T-Shirt an Rumpf und Oberarmen")),
        "shorts": (0.16, 0.12, 1.3, T("Nappy and shorts over the hips", "Windel und Shorts über der Hüfte")),
        "socks": (0.03, 0.01, 1.05, T("Short socks", "Kurze Socken")),
        "shoes": (0.08, 0.03, 1.1, T("First-walker shoes", "Lauflernschuhe")),
    }
    for reg in t["regions"]:
        sf, ms, fcl, title = spec[reg["id"]]
        reg["title"] = title
        reg["areaM2"] = r(sf * area * fcl, 4)
        reg["body"]["skinFraction"] = sf
        reg["body"]["metabolicShare"] = ms
        reg["characteristicLengthM"] = r(reg["characteristicLengthM"] * 0.8, 3)
        if reg["id"] == "shorts":
            reg["layers"] = [{"id": "shorts", "title": T("Disposable nappy under cotton shorts", "Wegwerfwindel unter Baumwollshorts"), "kind": "clothing", "material": "cotton-short", "thicknessM": 0.007, "resistanceM2KW": r(0.26 * 0.155, 5), "vapourResistanceM2PaW": 60, "heatCapacityJM2K": 800, "source": "authored-estimate", "reference": "iso-9920-2007"}]
            reg["heatCapacityJM2K"] = 13800
        if reg["id"] == "tshirt":
            reg["layers"][0]["title"] = T("Cotton bodysuit", "Baumwollbody")
    t["respiration"]["patterns"] = [
        {"id": "rest-toddler", "title": T("Toddler at rest, standing", "Kleinkind in Ruhe, stehend"), "rateBpm": 26, "tidalVolumeL": 0.1, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "fleming-2011"},
        {"id": "toddling", "title": T("Toddler walking", "Kleinkind beim Gehen"), "rateBpm": 34, "tidalVolumeL": 0.17, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.48, "pauseAfterExhaleFraction": 0.08, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "fleming-2011"},
    ]
    t["respiration"]["motion"] = [{"bone": "torso", "angleDeg": 0.9, "dy": 0.0015}]
    for e in t["emitters"]:
        if e["kind"] == "breath":
            e["diameterM"] = r(e["diameterM"] * 0.92, 4)
        if e["id"] == "body-plume":
            e["diameterM"] = 0.12
    import texts_toddler as X
    t["assumptions"] = X.thermal_assumptions(area, BODY_KG, STATURE, a["dimensions"]["massKg"])


REGIONS = ["face-skin", "hair", "arm-skin", "leg-skin", "tshirt", "shorts", "socks", "shoes"]


def binding(met, breathing, sun=0.6, plume=0.6, ground=None):
    regions = {rid: {"exposedFraction": 1, "sunlitFraction": sun} for rid in REGIONS}
    regions["tshirt"]["exposedFraction"] = 0.9
    if ground is not None:
        regions["shoes"]["contact"] = {"fraction": ground, "tempC": 26}
    return {"activityMet": met, "breathing": breathing, "regions": regions, "emitters": {"body-plume": plume}}


ARMS = {"arm-l": {"angleDeg": 8, "abductionDeg": 18}, "arm-r": {"angleDeg": 8, "abductionDeg": -18}, "forearm-l": {"angleDeg": 38}, "forearm-r": {"angleDeg": 38}, "hand-l": {"angleDeg": -6}, "hand-r": {"angleDeg": -6}}


def wide(bones):
    out = copy.deepcopy(bones)
    out["thigh-l"]["abductionDeg"] = SPREAD
    out["thigh-r"]["abductionDeg"] = -SPREAD
    return out


def poses_and_clips(doc):
    import texts_toddler as X
    a = doc["asset"]
    p = dict(GAIT, armBias=0, armSwing=0, elbowBias=0, elbowSwing=0, lean=1.5, leanSwing=0.8)
    cyc = solve_cycle(a, p, N_KEYS)
    print(f"[DEBUG] toddle h0 {cyc['h0']:.4f} drop {cyc['drop']:.4f} clearance {cyc['clearance']:.4f}")
    import math
    def upper(t):
        ph = 2 * math.pi * t / p["T"]
        return {"neck": {"angleDeg": 0}, "head": {"angleDeg": 3}, "clavicle-l": {"angleDeg": 4}, "clavicle-r": {"angleDeg": 4},
                "arm-l": {"angleDeg": r(8 - 3 * math.cos(ph), 3), "abductionDeg": 18}, "arm-r": {"angleDeg": r(8 + 3 * math.cos(ph), 3), "abductionDeg": -18},
                "forearm-l": {"angleDeg": 38}, "forearm-r": {"angleDeg": 38}, "hand-l": {"angleDeg": -6}, "hand-r": {"angleDeg": -6}}
    for k in cyc["keys"]:
        k["bones"] = wide(dict(k["bones"], **upper(k["t"])))
    at = lambda frac: cyc["keys"][round(frac * N_KEYS) % N_KEYS]
    contacts = lambda frac: [f"{an}-{side}" for side in ("l", "r") for an in at(frac)["contacts"][side]]
    stand = wide({"thigh-l": {"angleDeg": 4}, "thigh-r": {"angleDeg": 4}, "shank-l": {"angleDeg": -9}, "shank-r": {"angleDeg": -9}, "foot-l": {"angleDeg": 5}, "foot-r": {"angleDeg": 5}, "toe-l": {"angleDeg": 0}, "toe-r": {"angleDeg": 0}})
    squat = {"thigh-l": {"angleDeg": 112, "abductionDeg": 7}, "thigh-r": {"angleDeg": 112, "abductionDeg": -7}, "shank-l": {"angleDeg": -138}, "shank-r": {"angleDeg": -138}, "foot-l": {"angleDeg": 26}, "foot-r": {"angleDeg": 26}, "toe-l": {"angleDeg": 0}, "toe-r": {"angleDeg": 0},
             "torso": {"angleDeg": -34}, "neck": {"angleDeg": 10}, "head": {"angleDeg": 14}, "clavicle-l": {"angleDeg": 0}, "clavicle-r": {"angleDeg": 0},
             "arm-l": {"angleDeg": 62, "abductionDeg": 6}, "arm-r": {"angleDeg": 48, "abductionDeg": -6}, "forearm-l": {"angleDeg": 20}, "forearm-r": {"angleDeg": 34}, "hand-l": {"angleDeg": -10}, "hand-r": {"angleDeg": 0}}
    poses = [
        {"id": "toddling-midstance", **X.POSE_TEXT["toddling-midstance"], "bones": at(0.33)["bones"], "ground": "auto", "contacts": contacts(0.33), "thermal": binding(2.4, "toddling", plume=0.8), "loads": []},
        {"id": "toddling-foot-lands", **X.POSE_TEXT["toddling-foot-lands"], "bones": at(0.0)["bones"], "ground": "auto", "contacts": contacts(0.0), "thermal": binding(2.4, "toddling", plume=0.8), "loads": []},
        {"id": "standing-wide", **X.POSE_TEXT["standing-wide"], "bones": dict(stand, **{"torso": {"angleDeg": 3}, "neck": {"angleDeg": 0}, "head": {"angleDeg": 2}, "clavicle-l": {"angleDeg": 2}, "clavicle-r": {"angleDeg": 2}}, **ARMS), "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"], "thermal": binding(1.4, "rest-toddler", ground=0.35), "loads": []},
        {"id": "squatting", **X.POSE_TEXT["squatting"], "bones": squat, "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"], "thermal": binding(1.6, "rest-toddler", ground=0.4), "loads": []},
    ]
    a["poses"] = poses
    clip = walk_clip(cyc, p, "toddle", X.CLIP_TEXT["toddle"]["title"], X.CLIP_TEXT["toddle"]["description"](p), "toddling-midstance", 2.4, 1)
    stand_b = poses[2]["bones"]
    sway = lambda d: dict(copy.deepcopy(stand_b), **{"torso": {"angleDeg": 3 + d}, "head": {"angleDeg": 2 - 2 * d}, "arm-l": {"angleDeg": 8 - 2 * d, "abductionDeg": 18}, "arm-r": {"angleDeg": 8 + 2 * d, "abductionDeg": -18}})
    idle = {"id": "idle-breathing", **X.CLIP_TEXT["idle-breathing"], "durationS": 4.6, "fps": 15, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "standing-wide", "ease": "smooth", "bones": sway(0)}, {"t": 1.15, "pose": "standing-wide", "ease": "smooth", "bones": sway(1)}, {"t": 2.3, "pose": "standing-wide", "ease": "smooth", "bones": sway(0)}, {"t": 3.45, "pose": "standing-wide", "ease": "smooth", "bones": sway(-1)}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 4.6} for x in ("heel-l", "ball-l", "heel-r", "ball-r")],
            "phases": [{"id": "standing", "fromS": 0, "toS": 4.6, "title": T("Standing and looking around", "Stehen und Umschauen")}], "events": [],
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.4}, {"t": 4.6, "v": 1.4}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 4.6, "v": 0}]}]}
    half = {k: {kk: (vv / 2 if isinstance(vv, (int, float)) else vv) for kk, vv in v.items()} for k, v in squat.items()}
    for k in half:
        for kk in half[k]:
            half[k][kk] = r((squat[k][kk] + stand_b.get(k, {}).get(kk, poses[2]["bones"].get(k, {}).get(kk, 0))) / 2, 3)
    pick = {"id": "squat-and-stand", **X.CLIP_TEXT["squat-and-stand"], "durationS": 3.2, "fps": 30, "loop": False, "rootMotion": "plant",
            "keys": [{"t": 0, "pose": "standing-wide", "ease": "smooth"}, {"t": 0.6, "pose": "standing-wide", "ease": "smooth", "bones": half}, {"t": 1.2, "pose": "squatting", "ease": "smooth"}, {"t": 2.0, "pose": "squatting", "ease": "smooth"}, {"t": 2.6, "pose": "standing-wide", "ease": "smooth", "bones": half}, {"t": 3.2, "pose": "standing-wide", "ease": "smooth"}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 3.2} for x in ("heel-l", "ball-l", "heel-r", "ball-r")],
            "phases": [{"id": "going-down", "fromS": 0, "toS": 1.2, "title": T("Squatting down", "In die Hocke gehen")}, {"id": "picking", "fromS": 1.2, "toS": 2.0, "title": T("Picking something up", "Etwas aufheben")}, {"id": "standing-up", "fromS": 2.0, "toS": 3.2, "title": T("Standing up", "Aufstehen")}],
            "events": [{"t": 1.6, "kind": "motion", "title": T("Hand reaches the ground", "Hand erreicht den Boden"), "target": "hand-l"}],
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.4}, {"t": 1.6, "v": 1.8}, {"t": 3.2, "v": 1.4}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.2, "v": 0}]}]}
    a["clips"] = [clip, pick, idle]
    a["presentation"] = {"defaultPose": "toddling-midstance", "defaultClip": "toddle", "defaultScale": 50}


def texts(doc):
    import texts_toddler as X
    a = doc["asset"]
    a["title"], a["summary"], a["tags"] = X.TITLE, X.SUMMARY, X.TAGS
    prov = a["provenance"]
    prov["author"] = "Semio staffage human worker HW-05"
    prov["created"] = "2026-11-22"
    prov["notes"] = X.NOTES
    have = {ref["id"] for ref in prov["references"]}
    prov["references"] += [ref for ref in X.NEW_REFERENCES if ref["id"] not in have]
    for sc, dec in X.DECISIONS.items():
        a["representations"][sc]["decisions"] = dec
    for sc in ("50", "100", "200", "500"):
        a["volumes"][sc]["decisions"] = X.VOLUME_DECISIONS[sc]
    a["structure"]["assumptions"] = X.STRUCTURE_ASSUMPTIONS
    sym = a["symbols"][0]
    sym["id"] = "toddler-marker"
    sym["title"], sym["description"] = X.SYMBOL["title"], X.SYMBOL["description"]


def build():
    doc = rescale(load("running-child"), F)
    doc["asset"]["id"] = SLUG
    remove_backpack(doc)
    nappy(doc)
    masses(doc)
    materials(doc)
    thermal(doc)
    poses_and_clips(doc)
    texts(doc)
    return doc


if __name__ == "__main__":
    doc = build()
    print("[DEBUG] dims", set_dims(doc, SLUG))
    found = feature_loop(doc, SLUG)
    doc["asset"]["dimensions"]["depthM"] = 0.38
    found = feature_loop(doc, SLUG)
    summarize(found)
