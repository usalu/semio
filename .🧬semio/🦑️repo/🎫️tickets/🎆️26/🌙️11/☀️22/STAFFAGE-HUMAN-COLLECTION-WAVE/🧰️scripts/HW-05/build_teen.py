"""🎧️ Builds `👤️humans/teenager-with-headphones`: a fifteen-year-old standing slouched over a phone with over-ear headphones on the head.

Derived by hand from the archive's `standing-phone-user` (an adult of 1.69 m): trunk, neck and head scaled to a fifteen-year-old of
1.66 m (legs and arms keep their adult length, the trunk is 4 % shorter: the long-legged proportion of the growth spurt), ponytail and
neck-worn headphones removed, short hair, over-ear headphones redrawn on the head for every scale (cups as elliptic prisms over the ears,
a padded band arching over the crown), slouched poses, a nodding clip to the beat of the music and a glance up from the phone.

Run: python3 build_teen.py
@see ./lib.py ./tools.py ./texts_teen.py
"""
import copy
import math

from lib import T, load, r, rescale
from tools import drop_shapes, dubois, feature_loop, save, set_dims, shapes, solids, summarize

SLUG = "teenager-with-headphones"
STATURE = 1.66
BODY_KG = 54.0
CLOTHES_KG = 1.3
PHONES_KG = 0.25

F = {"pelvis": 0.96, "torso": 0.96, "clavicle-l": 0.96, "clavicle-r": 0.96, "ilium-l": 0.96, "ilium-r": 0.96, "neck": 0.95, "head": 0.98}

CUP = {"u": 0.025, "v": 0.01, "au": 0.047, "av": 0.040, "win": 0.072}
BAND = {"win": 0.076, "uflat": 0.069, "top": 0.153, "half": 0.016}
PER_SCALE = {"50": {"cup": 0.038, "band": 0.014}, "100": {"cup": 0.040, "band": 0.016}, "200": {"cup": 0.040, "band": None}, "500": {"cup": None, "band": None}}
Z = {"elevation": {"cup": 5.7, "band": 5.65}, "front": {"cup": 5.6, "band": 5.65}, "plan": {"cup": 6.35, "band": 6.4}}


def drop_old(doc):
    a = doc["asset"]
    a["rig"]["bones"] = [b for b in a["rig"]["bones"] if b["id"] != "ponytail"]
    gone = lambda sid: "pony" in sid or "headphone" in sid
    drop_shapes(doc, lambda s: s["bone"] == "ponytail" or gone(s["id"]))
    for vol in a["volumes"].values():
        if vol.get("status") != "authored":
            continue
        ids = {s["id"] for s in vol["body"] if gone(s["id"]) or s.get("bone") == "ponytail"}
        vol["body"] = [s for s in vol["body"] if s["id"] not in ids]
        vol["thermal"] = [s for s in vol["thermal"] if not gone(s["id"]) and s.get("of") not in ids]
        vol["structure"] = [s for s in vol["structure"] if not gone(s["id"]) and s.get("bone") != "ponytail" and s.get("of") not in ids]
    for p in a["poses"]:
        p["bones"].pop("ponytail", None)
    for c in a["clips"]:
        for k in c["keys"]:
            if "bones" in k:
                k["bones"].pop("ponytail", None)


def ellipse(cu, cv, au, av, n=28):
    return [[r(cu + au * math.cos(2 * math.pi * i / n)), r(cv + av * math.sin(2 * math.pi * i / n))] for i in range(n)]


def arch(win, uflat, top, thick, ubot, n=14):
    """The band profile in (w, u): an outer and an inner arch over the crown, both standing on the cups at u = ubot."""
    def half(w0, h):
        return [(w0 * math.cos(math.pi / 2 * i / n), uflat + (h - uflat) * math.sin(math.pi / 2 * i / n)) for i in range(n + 1)]
    wo, ho = win + thick, top + thick
    outer = [(wo, ubot)] + half(wo, ho) + [(-w, u) for w, u in reversed(half(wo, ho)[:-1])] + [(-wo, ubot)]
    inner = [(-win, ubot)] + [(-w, u) for w, u in half(win, top)] + [(w, u) for w, u in reversed(half(win, top)[:-1])] + [(win, ubot)]
    out = []
    for w, u in outer + inner:
        p = [r(w), r(u)]
        if not out or out[-1] != p:
            out.append(p)
    return out


def headphones(doc):
    a = doc["asset"]
    k = F["head"]
    cu, cv, au, av, win = CUP["u"] * k, CUP["v"] * k, CUP["au"], CUP["av"], CUP["win"] * k
    ubot = cu + 0.025
    profile = ellipse(cu, cv, au, av)
    base = {"bone": "head", "role": "body", "filled": True, "tone": 0.18, "region": "headphones", "material": "headphone-polymer", "kind": "poly", "closed": True}
    for sc, rep in a["representations"].items():
        spec = PER_SCALE[sc]
        g = rep["geometry"]
        if spec["cup"] is None:
            for view in g:
                for s in g[view]:
                    if s["id"] == "sil-head":
                        s.pop("region", None)
                        s["regions"] = list(dict.fromkeys((s.get("regions") or []) + ["headphones"]))
            continue
        t = spec["cup"]
        g["elevation"].append(dict(base, id="headphone-cup-l", z=Z["elevation"]["cup"], points=profile))
        for side, sg in (("l", 1), ("r", -1)):
            z0, z1 = sorted((sg * win, sg * (win + t)))
            for view in ("front", "plan"):
                g[view].append(dict(base, id=f"headphone-cup-{side}", z=Z[view]["cup"], points=profile, solid={"kind": "extruded", "sweep": "w", "z0": r(z0), "z1": r(z1)}))
        if spec["band"] is not None:
            bt = spec["band"]
            prof = arch(BAND["win"] * k, BAND["uflat"] * k, BAND["top"] * k, bt, ubot)
            v0, v1 = cv - BAND["half"], cv + BAND["half"]
            top = BAND["top"] * k + bt
            g["elevation"].append(dict(base, id="headphone-band", z=Z["elevation"]["band"], points=[[r(ubot), r(v0)], [r(top), r(v0)], [r(top), r(v1)], [r(ubot), r(v1)]]))
            for view in ("front", "plan"):
                g[view].append(dict(base, id="headphone-band", z=Z[view]["band"], points=[[u, w] for w, u in prof], solid={"kind": "extruded", "sweep": "v", "z0": r(v0), "z1": r(v1)}))
    for sc, vol in a["volumes"].items():
        spec = PER_SCALE[sc]
        sb = {"bone": "head", "material": "headphone-polymer", "role": "body", "tone": 0.18}
        if spec["cup"] is None:
            for side, sg in (("l", 1), ("r", -1)):
                vol["thermal"].append(dict(sb, id=f"th-headphone-cup-{side}", kind="ellipsoid", radii=[0.047, 0.04, 0.0375], transform={"translation": [r(cu), r(cv), r(sg * (win + 0.027))]}, region="headphones"))
            continue
        t = spec["cup"]
        for side, sg in (("l", 1), ("r", -1)):
            lo, hi = sorted((sg * win, sg * (win + t)))
            vol["body"].append(dict(sb, id=f"headphone-cup-{side}", kind="extrude", profile=profile, plane="xy", **{"from": r(lo), "to": r(hi)}))
            vol["thermal"].append({"id": f"th-headphone-cup-{side}", "of": f"headphone-cup-{side}", "region": "headphones"})
        if spec["band"] is not None:
            prof = arch(BAND["win"] * k, BAND["uflat"] * k, BAND["top"] * k, spec["band"], ubot)
            vol["body"].append(dict(sb, id="headphone-band", kind="extrude", profile=prof, plane="zx", **{"from": r(cv - BAND["half"]), "to": r(cv + BAND["half"])}))
            vol["thermal"].append({"id": "th-headphone-band", "of": "headphone-band", "region": "headphones"})


def masses(doc):
    a = doc["asset"]
    body = [b for b in a["rig"]["bones"] if "mass" in b and b["id"] != "phone"]
    total = sum(b["mass"]["kg"] * F.get(b["id"], 1.0) ** 3 for b in body)
    k = (BODY_KG + CLOTHES_KG) / total
    for b in body:
        b["mass"]["kg"] = r(b["mass"]["kg"] * F.get(b["id"], 1.0) ** 3 * k, 3)
    head = next(b for b in a["rig"]["bones"] if b["id"] == "head")
    head["mass"]["kg"] = r(head["mass"]["kg"] + PHONES_KG, 3)
    head["title"] = T("Head with short hair and headphones", "Kopf mit kurzem Haar und Kopfhörern")
    a["dimensions"]["massKg"] = r(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b), 2)


def thermal(doc):
    import texts_teen as X
    a = doc["asset"]
    t = a["thermal"]
    t["environment"] = {"airTempC": 15, "meanRadiantTempC": 14, "skyTempC": 4, "groundTempC": 13, "relativeHumidity": 0.65, "windMS": 0.8, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
    area = dubois(BODY_KG, STATURE)
    t["body"].update({"massKg": BODY_KG, "skinAreaM2": r(area, 3), "basalMetabolismW": 73.2, "source": "authored-estimate", "reference": "schofield-1985"})
    for reg in t["regions"]:
        if "body" in reg:
            reg["areaM2"] = r(reg["areaM2"] * area / 1.675, 4)
        if reg["id"] == "hair":
            reg["title"] = T("Short hair under the headband", "Kurzes Haar unter dem Kopfbügel")
            reg["layers"][0].update({"title": T("Short hair", "Kurzes Haar"), "thicknessM": 0.02, "resistanceM2KW": 0.06, "vapourResistanceM2PaW": 9.6})
        if reg["id"] == "hoodie":
            reg["title"] = T("T-shirt and oversized hoodie on trunk and arms", "T-Shirt und weiter Kapuzenpullover an Rumpf und Armen")
            reg["layers"][1]["title"] = T("Oversized fleece hoodie, hood down", "Weiter Fleece-Kapuzenpullover, Kapuze unten")
        if reg["id"] == "headphones":
            reg.update({"title": T("Wireless over-ear headphones on the head", "Kabellose Over-Ear-Kopfhörer auf dem Kopf"), "areaM2": 0.075, "geometry": "sphere", "skyViewFactor": 0.6, "ratedPowerW": 2.5, "ratedPowerBasis": "electrical-input", "reference": "iec-62368-1-2018"})
    pats = t["respiration"]["patterns"]
    pats[:] = [p for p in pats if p["id"] != "talking"] + [{"id": "listening", "title": T("Listening to music, nodding along", "Musik hören, im Takt nicken"), "rateBpm": 16, "tidalVolumeL": 0.45, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    t["assumptions"] = X.thermal_assumptions(area, BODY_KG, STATURE, a["dimensions"]["massKg"])


LEGS = {"thigh-l": {"angleDeg": 4}, "shank-l": {"angleDeg": -9}, "foot-l": {"angleDeg": 5}, "toe-l": {"angleDeg": 0}, "thigh-r": {"angleDeg": 4}, "shank-r": {"angleDeg": -9}, "foot-r": {"angleDeg": 5}, "toe-r": {"angleDeg": 0}}


def poses_and_clips(doc):
    import texts_teen as X
    a = doc["asset"]
    old = {p["id"]: p for p in a["poses"]}
    tex = copy.deepcopy(old["texting"])
    tex["bones"].update(LEGS)
    tex["bones"].update({"torso": {"angleDeg": -4}, "neck": {"angleDeg": -23}, "head": {"angleDeg": -22}, "clavicle-l": {"angleDeg": -4}, "clavicle-r": {"angleDeg": -4}})
    tex["thermal"]["breathing"] = "typing"
    up = copy.deepcopy(tex)
    up["bones"].update({"torso": {"angleDeg": -3}, "neck": {"angleDeg": -6}, "head": {"angleDeg": 2}})
    down = copy.deepcopy(tex)
    down["bones"].update({"torso": {"angleDeg": -4}, "neck": {"angleDeg": -14}, "head": {"angleDeg": -8},
                          "arm-r": {"angleDeg": 2, "abductionDeg": -4, "twistDeg": -10}, "forearm-r": {"angleDeg": 18, "twistDeg": 20}, "hand-r": {"angleDeg": 0, "abductionDeg": 0, "twistDeg": 0}, "phone": {"angleDeg": -30},
                          "arm-l": {"angleDeg": 2, "abductionDeg": 4, "twistDeg": 10}, "forearm-l": {"angleDeg": 14, "twistDeg": -20}, "hand-l": {"angleDeg": 0, "abductionDeg": 0, "twistDeg": 0}})
    down["thermal"]["breathing"] = "listening"
    down["thermal"]["activityMet"] = 1.2
    down["thermal"]["operating"] = {"phone": 0.3}
    up["thermal"]["operating"] = {"phone": 0.6}
    poses = []
    for pid, p in (("slouched-texting", tex), ("glancing-up", up), ("listening-phone-down", down)):
        p["id"] = pid
        p["title"], p["description"] = X.POSE_TEXT[pid]["title"], X.POSE_TEXT[pid]["description"]
        p["thermal"]["regions"]["headphones"] = {"exposedFraction": 0.9, "sunlitFraction": 0}
        p["thermal"]["regions"]["hair"] = {"exposedFraction": 0.85, "sunlitFraction": 0}
        p.pop("key", None)
        poses.append(p)
    a["poses"] = poses
    nod = lambda d, bounce: {"head": {"angleDeg": -8 - d}, "neck": {"angleDeg": -14 - d / 3}, "thigh-l": {"angleDeg": 4 + bounce}, "thigh-r": {"angleDeg": 4 + bounce}, "shank-l": {"angleDeg": -9 - 2 * bounce}, "shank-r": {"angleDeg": -9 - 2 * bounce}, "foot-l": {"angleDeg": 5 + bounce}, "foot-r": {"angleDeg": 5 + bounce}}
    beat = 0.5
    keys = []
    for i in range(8):
        tk = i * beat / 2
        down_beat = i % 2 == 1
        keys.append({"t": r(tk, 3), "pose": "listening-phone-down", "ease": "smooth", "bones": nod(9 if down_beat else 0, 2.5 if down_beat else 0)})
    feet = ("heel-l", "ball-l", "heel-r", "ball-r")
    nodclip = {"id": "nod-to-the-beat", **X.CLIP_TEXT["nod-to-the-beat"], "durationS": 2.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
               "contacts": [{"anchor": x, "fromS": 0, "toS": 2.0} for x in feet],
               "phases": [{"id": "bar", "fromS": 0, "toS": 2.0, "title": T("One bar of four beats at 120 per minute", "Ein Takt mit vier Schlägen bei 120 je Minute")}],
               "events": [{"t": r(i * beat + beat / 2, 3), "kind": "motion", "title": T(f"Beat {i + 1}: head down", f"Schlag {i + 1}: Kopf unten"), "target": "head"} for i in range(4)],
               "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 2.0, "v": 1.3}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 2.0, "v": 2}]}, {"target": "operating:phone", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 2.0, "v": 0.3}]}]}
    sway = lambda dh, dt: {"head": {"angleDeg": -22 + dh}, "neck": {"angleDeg": -23 + dh / 2}, "torso": {"angleDeg": -4 + dt}}
    idle = {"id": "idle-scrolling", **X.CLIP_TEXT["idle-scrolling"], "durationS": 6.0, "fps": 15, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "slouched-texting", "ease": "smooth", "bones": sway(0, 0)}, {"t": 1.5, "pose": "slouched-texting", "ease": "smooth", "bones": sway(-3, -1)}, {"t": 3.0, "pose": "slouched-texting", "ease": "smooth", "bones": sway(1.5, 0.8)}, {"t": 4.5, "pose": "slouched-texting", "ease": "smooth", "bones": sway(-2, -0.5)}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 6.0} for x in feet],
            "phases": [{"id": "scrolling", "fromS": 0, "toS": 6.0, "title": T("Scrolling", "Scrollen")}], "events": [],
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 6.0, "v": 1.2}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 6.0, "v": 1}]}, {"target": "operating:phone", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 3.0, "v": 0.8}, {"t": 6.0, "v": 0.6}]}]}
    glance = {"id": "glance-up", **X.CLIP_TEXT["glance-up"], "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
              "keys": [{"t": 0, "pose": "slouched-texting", "ease": "smooth"}, {"t": 0.8, "pose": "glancing-up", "ease": "smooth"}, {"t": 1.9, "pose": "glancing-up", "ease": "smooth"}, {"t": 3.0, "pose": "slouched-texting", "ease": "smooth"}],
              "contacts": [{"anchor": x, "fromS": 0, "toS": 3.0} for x in feet],
              "phases": [{"id": "look-up", "fromS": 0, "toS": 0.8, "title": T("Raising the head", "Kopf heben")}, {"id": "check", "fromS": 0.8, "toS": 1.9, "title": T("Checking the surroundings", "Umgebung prüfen")}, {"id": "back", "fromS": 1.9, "toS": 3.0, "title": T("Back to the screen", "Zurück zum Bildschirm")}],
              "events": [{"t": 0.8, "kind": "motion", "title": T("Eyes up", "Blick hoch"), "target": "head"}],
              "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 3.0, "v": 1.2}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 3.0, "v": 1}]}, {"target": "operating:phone", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 3.0, "v": 0.6}]}]}
    a["clips"] = [nodclip, idle, glance]
    a["presentation"] = {"defaultPose": "slouched-texting", "defaultClip": "nod-to-the-beat", "defaultScale": 100}


def texts(doc):
    import texts_teen as X
    a = doc["asset"]
    a["title"], a["summary"], a["tags"] = X.TITLE, X.SUMMARY, X.TAGS
    prov = a["provenance"]
    prov["author"] = "Semio staffage human worker HW-05"
    prov["created"] = "2026-11-22"
    prov["notes"] = X.NOTES
    have = {ref["id"] for ref in prov["references"]}
    prov["references"] += [ref for ref in X.NEW_REFERENCES if ref["id"] not in have]
    for sc in ("50", "100", "200", "500"):
        a["representations"][sc]["decisions"] = X.DECISIONS[sc]
        a["volumes"][sc]["decisions"] = X.VOLUME_DECISIONS[sc]
    a["structure"]["assumptions"] = X.STRUCTURE_ASSUMPTIONS
    for m in a["materials"]:
        if m["id"] == "headphone-polymer":
            m["title"] = T("Polymer headband and cups with imitation-leather cushions", "Kopfbügel und Muscheln aus Polymer mit Kunstlederpolstern")
        if m["id"] == "fleece-sweatshirt":
            m["title"] = T("Fleece hoodie", "Fleece-Kapuzenpullover")
    for b in a["rig"]["bones"]:
        if b["id"] == "torso":
            b["title"] = T("Trunk with oversized hoodie", "Rumpf mit weitem Kapuzenpullover")


def build():
    doc = rescale(load("standing-phone-user"), F)
    doc["asset"]["id"] = SLUG
    drop_old(doc)
    headphones(doc)
    masses(doc)
    thermal(doc)
    poses_and_clips(doc)
    texts(doc)
    return doc


if __name__ == "__main__":
    doc = build()
    print("[DEBUG] dims", set_dims(doc, SLUG))
    found = feature_loop(doc, SLUG)
    summarize(found)
