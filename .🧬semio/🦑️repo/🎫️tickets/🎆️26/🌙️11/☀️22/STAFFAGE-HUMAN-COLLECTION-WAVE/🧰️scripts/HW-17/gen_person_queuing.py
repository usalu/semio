"""🧍️ Generator of `👤️humans/person-queuing`: a 1.78 m man waiting in a queue, hands in the front pockets, a daypack on the back.

Derived from `standing-adult` (rig, outlines and the four handcrafted volumes) by a similarity of 1.78/1.69 about the ground origin and a
re-distribution of the segment masses after the male table of de Leva (1996); the daypack, all poses, clips, texts, materials, tones and the
outdoor spring scenario are authored here. Run: `python3 gen_person_queuing.py` (writes the catalog file and prints the pose solves)."""
import copy
import json
import math

import pack as pk
from hw17lib import *

SLUG = "person-queuing"
STATURE, BASE_STATURE = 1.78, 1.69
S_ = STATURE / BASE_STATURE
BODY_KG, CLOTHES_KG, PACK_KG, AGE = 78.0, 1.5, 4.2, 31

DE_LEVA_MALE = {"head": 0.0694, "lower-trunk": 0.1117, "middle-trunk": 0.1633, "upper-trunk": 0.1596, "upper-arm": 0.0271, "forearm": 0.0162, "hand": 0.0061, "thigh": 0.1416, "shank": 0.0433, "foot": 0.0137}


def group(bone):
    """🧩️ The de Leva segment of a bone."""
    b = bone
    if b in ("neck", "head"):
        return "head"
    if b == "pelvis":
        return "lower-trunk"
    if b == "lumbar":
        return "middle-trunk"
    if b == "torso":
        return "upper-trunk"
    for pre, g in (("arm-", "upper-arm"), ("forearm-", "forearm"), ("thigh-", "thigh"), ("shank-", "shank"), ("foot-", "foot"), ("toe-", "foot")):
        if b.startswith(pre):
            return g
    if b.startswith(("hand-", "thumb-", "index-", "middle-", "ring-", "little-")):
        return "hand"
    return None


def remass(a):
    """⚖️ Segment masses after the male fractions of de Leva (1996) for 78 kg plus 1.5 kg of clothing spread in proportion."""
    total = BODY_KG + CLOTHES_KG
    groups = {}
    for b in a["rig"]["bones"]:
        if "mass" in b:
            g = group(b["id"])
            groups.setdefault(g, []).append(b)
    for g, bones in groups.items():
        sides = 2 if g not in ("head", "lower-trunk", "middle-trunk", "upper-trunk") else 1
        target = DE_LEVA_MALE[g] * total * sides
        have = sum(b["mass"]["kg"] for b in bones)
        for b in bones:
            b["mass"]["kg"] = round(b["mass"]["kg"] * target / have, 4)
            if b["mass"].get("source") == "literature":
                b["mass"]["reference"] = "de-leva-1996"


TONES_REGION = {"hoodie": {0.4: 0.24, 0.45: 0.28}, "jeans": {0.62: 0.72, 0.6: 0.72}, "hair": {0.75: 0.38}, "sneakers": {0.12: 0.9, 0.25: 0.9}}
TONES_MATERIAL = {"fleece-sweatshirt": {0.35: 0.2, 0.4: 0.24, 0.42: 0.26, 0.45: 0.28}, "denim": {0.62: 0.72}, "hair": {0.75: 0.38}, "canvas-sneaker": {0.12: 0.9, 0.88: 0.88}}


def retone(a):
    """🎨️ The queuing man's own palette (tone is lightness): charcoal sweatshirt, light-washed jeans, mid-brown hair, white leather sneakers."""
    for rep in a["representations"].values():
        for view in rep["geometry"].values():
            for sh in view:
                if sh["role"] != "body":
                    continue
                regs = [sh["region"]] if "region" in sh else sh.get("regions", [])
                if len(regs) == 1 and regs[0] in TONES_REGION:
                    sh["tone"] = TONES_REGION[regs[0]].get(sh["tone"], sh["tone"])
    for vol in a["volumes"].values():
        for so in vol["body"]:
            m = TONES_MATERIAL.get(so.get("material"))
            if m:
                so["tone"] = m.get(so["tone"], so["tone"])


def materials(a):
    """🧱️ Materials of the outfit and the pack."""
    by = {m["id"]: m for m in a["materials"]}
    by["fleece-sweatshirt"]["title"] = tx("Cotton-polyester fleece, charcoal hooded sweatshirt", "Baumwoll-Polyester-Fleece, anthrazitfarbenes Kapuzensweatshirt")
    by["fleece-sweatshirt"]["solarAbsorptivity"] = 0.85
    by["denim"]["title"] = tx("Light-washed denim", "Hell gewaschener Denim")
    by["denim"]["solarAbsorptivity"] = 0.62
    by["canvas-sneaker"]["title"] = tx("White leather sneaker with rubber cupsole", "Weißer Ledersneaker mit Gummisohle")
    by["canvas-sneaker"]["solarAbsorptivity"] = 0.3
    by["canvas-sneaker"]["emissivity"] = 0.92
    by["hair"]["solarAbsorptivity"] = 0.8
    a["materials"].append({"id": "polyester-pack", "title": tx("Polyester 600D pack fabric with PU coating", "Rucksackgewebe aus Polyester 600D mit PU-Beschichtung"),
                           "emissivity": 0.88, "solarAbsorptivity": 0.55, "densityKgM3": 1380, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "ashrae-2021"})


PACK_PROFILE = [(0.0, 0.118, 0.042), (0.014, 0.138, 0.062), (0.045, 0.145, 0.071), (0.2, 0.145, 0.074), (0.31, 0.14, 0.068), (0.372, 0.128, 0.057), (0.405, 0.106, 0.044), (0.42, 0.072, 0.03)]
PACK_PROFILE_COARSE = {"100": [0, 1, 2, 3, 4, 5, 6, 7], "200": [0, 2, 3, 4, 6, 7], "500": [1, 3, 5, 7]}
PACK_EXP = 3.0
POCKET_MERGE = 0.008
POCKET_PROFILE = [(0.03, 0.08, 0.006), (0.045, 0.1, 0.012), (0.17, 0.1, 0.013), (0.2, 0.085, 0.008)]


def add_pack(a, rest):
    """🎒️ Bone, volumes, drawings, region and structure of the daypack at every scale."""
    loft50 = next(s for s in a["volumes"]["50"]["body"] if s["id"] == "hoodie")
    u0 = 0.226 - 0.42
    secs, origin = pk.build(PACK_PROFILE, loft50, u0, 0.004, PACK_EXP)
    a["rig"]["bones"].append({"id": "backpack", "parent": "torso", "title": tx("Daypack (18 l)", "Tagesrucksack (18 l)"), "x": round(origin[0], 5), "y": round(origin[1], 5),
                              "angleDeg": 0, "lengthM": 0.42, "joint": {"type": "fixed"},
                              "mass": {"kg": PACK_KG, "comX": 0.19, "comY": 0.07, "distribution": "point", "source": "authored-estimate"}})
    torso = rest["torso"]
    out = {}
    for key in ("50", "100", "200", "500"):
        loft = next(s for s in a["volumes"][key]["body"] if s["id"] == "hoodie")
        rows = PACK_PROFILE if key == "50" else [PACK_PROFILE[i] for i in PACK_PROFILE_COARSE[key]]
        if key in ("200", "500"):
            rows = [(du, hw, hd + POCKET_MERGE if 0.04 <= du <= 0.21 else hd) for du, hw, hd in rows]
        ss, _ = pk.build(rows, loft, u0, 0.004, PACK_EXP)
        out[key] = ss
    pocket_secs = []
    mid = out["50"]
    for du, hw, hd in POCKET_PROFILE:
        u = u0 + du
        outer = None
        for a_, b_ in zip(mid, mid[1:]):
            if a_["u"] <= u <= b_["u"]:
                t = (u - a_["u"]) / (b_["u"] - a_["u"])
                outer = a_["vc"] + a_["hd"] + (b_["vc"] + b_["hd"] - a_["vc"] - a_["hd"]) * t
        pocket_secs.append({"u": u, "vc": outer - 0.006 + hd, "hw": hw, "hd": hd})
    for key, ss in out.items():
        vol = a["volumes"][key]
        vol["body"].append(pk.loft_solid("backpack", ss, origin, "polyester-pack", 0.55, PACK_EXP))
        vol["thermal"].append({"id": "th-backpack", "of": "backpack", "region": "backpack"})
        if key in ("50", "100"):
            vol["body"].append(pk.loft_solid("backpack-pocket", pocket_secs, origin, "polyester-pack", 0.62, PACK_EXP, "detail"))
            vol["thermal"].append({"id": "th-backpack-pocket", "of": "backpack-pocket", "region": "backpack"})
        torso_struct = [s for s in vol["structure"] if s.get("bone") == "torso" and s.get("part") == "member"]
        target = (origin[0] + 0.3, origin[1] - 0.03)

        def centre(s):
            if s["kind"] == "capsule":
                return [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)]
            if s["kind"] == "loft":
                return s["sections"][len(s["sections"]) // 2]["at"]
            return (s.get("transform") or {}).get("translation", [0, 0, 0])

        best = min(torso_struct, key=lambda s: (centre(s)[0] - target[0]) ** 2 + (centre(s)[1] - target[1]) ** 2 + centre(s)[2] ** 2)
        c = centre(best)
        r = {"50": 0.006, "100": 0.008, "200": 0.016, "500": 0.038}[key]
        thick = {"50": 0.012, "100": 0.016, "200": 0.032, "500": 0.076}[key]
        panel_u = 0.2
        vol["structure"].append({"id": "sk-backpack-panel", "kind": "box", "bone": "backpack", "material": "polyester-pack", "role": "body", "tone": 0.85,
                                 "size": [0.34, thick, 0.22], "cornerRadius": min(0.004, thick / 3), "transform": {"translation": [panel_u, round(thick / 2 + 0.004, 5), 0.0]}, "member": "backpack", "part": "member"})
        vol["structure"].append({"id": "sk-backpack-bearing", "kind": "capsule", "bone": "backpack", "material": "polyester-pack", "role": "body", "tone": 0.85,
                                 "p0": [panel_u + 0.06, round(thick / 2 + 0.004, 5), 0.0], "p1": [round(c[0] - origin[0], 5), round(c[1] - origin[1], 5), round(c[2], 5)], "r0": r, "r1": r, "member": "backpack", "part": "member"})
        vol["structure"].append({"id": "mass-backpack", "kind": "ellipsoid", "bone": "backpack", "material": "polyester-pack", "role": "body", "tone": 0.4,
                                 "radii": [{"50": 0.02, "100": 0.025, "200": 0.03, "500": 0.04}[key]] * 3, "transform": {"translation": [0.19, 0.07, 0.0]}, "member": "backpack", "part": "mass"})
    return out, pocket_secs, origin


def pack_shapes(a, rest, out, pocket_secs, origin):
    """✏️ The pack and its straps in every scale and view."""
    loft50 = next(s for s in a["volumes"]["50"]["body"] if s["id"] == "hoodie")
    torso = rest["torso"]
    pens = {"500": 0.13, "200": 0.18, "100": 0.2, "50": 0.25}
    for key, ss in out.items():
        g = a["representations"][key]["geometry"]
        base = dict(role="body", filled=True, tone=0.55, region="backpack", material="polyester-pack")
        g["elevation"].append(dict(id="backpack", bone="backpack", z=3.8, kind="poly", points=pk.elevation_poly(ss, origin), closed=True, **base))
        front, plan = pk.views(ss, torso, PACK_EXP)
        g["front"].append(dict(id="backpack", bone="backpack", z=0.4, kind="poly", points=front, closed=True, **base))
        g["plan"].append(dict(id="backpack", bone="backpack", z=3.85, kind="poly", points=plan, closed=True, **base))
        if key in ("50", "100"):
            pb = dict(base, tone=0.62, role="body")
            g["elevation"].append(dict(id="backpack-pocket", bone="backpack", z=3.85, kind="poly", points=pk.elevation_poly(pocket_secs, origin), closed=True, **pb))
            _, pplan = pk.views(pocket_secs, torso, PACK_EXP)
            g["plan"].append(dict(id="backpack-pocket", bone="backpack", z=3.9, kind="poly", points=pplan, closed=True, **pb))
            strap = straps(loft50, ss, torso)
            line = dict(role="detail", filled=False, tone=0.1, region="backpack", material="polyester-pack")
            g["elevation"].append(dict(id="backpack-strap", bone="torso", z=4.2, kind="poly", points=strap["elevation"], closed=False, **line))
            for side, pts in strap["front"].items():
                g["front"].append(dict(id=f"backpack-strap-{side}", bone="torso", z=4.6, kind="poly", points=pts, closed=False, **line))
            for side, pts in strap["plan"].items():
                g["plan"].append(dict(id=f"backpack-strap-{side}", bone="torso", z=4.6, kind="poly", points=pts, closed=False, **line))
            if key == "50":
                zip_ = [[round(s["u"] - origin[0], 5), round(s["vc"] + s["hd"] - origin[1] - 0.012, 5)] for s in ss[2:-1]]
                g["elevation"].append(dict(id="backpack-zip", bone="backpack", z=3.9, kind="poly", points=zip_, closed=False, **dict(line, role="texture")))
                g["front"].append(dict(id="backpack-sternum-strap", bone="torso", z=4.6, kind="poly", points=strap["sternum"], closed=False, **line))


def straps(loft, ss, torso):
    """🪢️ The shoulder straps as lines on the sweatshirt: up the back from the pack top, over the shoulder at w = 0.075, down the chest."""
    w = 0.075
    secs = loft["sections"]
    top = max(s["at"][0] for s in secs if s["rx"] > w + 0.01)
    u_pack = ss[-2]["u"]
    back, frontl = [], []
    n = 6
    for i in range(n + 1):
        u = u_pack + (top - u_pack) * i / n
        back.append((u, pk.back_surface(loft, u, w) - 0.002))
    def front_surface(u):
        for a, b in zip(secs, secs[1:]):
            if a["at"][0] <= u <= b["at"][0]:
                t = (u - a["at"][0]) / (b["at"][0] - a["at"][0])
                vc = a["at"][1] + (b["at"][1] - a["at"][1]) * t
                rx = a["rx"] + (b["rx"] - a["rx"]) * t
                ry = a["ry"] + (b["ry"] - a["ry"]) * t
                nn = a["exponent"] + (b["exponent"] - a["exponent"]) * t
                return vc - ry * max(0.0, 1 - abs(w / rx) ** nn) ** (1 / nn)
    u_low = 0.07
    for i in range(n + 1):
        u = top - (top - u_low) * i / n
        frontl.append((u, front_surface(u) + 0.002))
    elev = [[round(u, 5), round(v, 5)] for u, v in back + frontl[1:]]
    f = {}
    p = {}
    for side, sz in (("l", 1), ("r", -1)):
        pts = [pk.world(torso, (u, v, sz * (w + 0.012 * (top - u) / (top - u_low)))) for u, v in frontl]
        f[side] = [[round(q[2], 5), round(q[1], 5)] for q in pts]
        over = [pk.world(torso, (top - 0.004, v, sz * w)) for v in (back[-2][1], back[-1][1], (back[-1][1] + frontl[0][1]) / 2, frontl[1][1])]
        p[side] = [[round(q[0], 5), round(q[2], 5)] for q in over]
    us = 0.17
    vf = front_surface(us) + 0.002
    st = [pk.world(torso, (us, vf, w + 0.004)), pk.world(torso, (us, vf, -w - 0.004))]
    return {"elevation": elev, "front": f, "plan": p, "sternum": [[round(q[2], 5), round(q[1], 5)] for q in st]}


def legs_neutral():
    """🦵️ Both feet flat at hip-joint width (the base figure's stance)."""
    return {"thigh-l": {"angleDeg": -2.5}, "shank-l": {"angleDeg": -2.5}, "foot-l": {"angleDeg": 5}, "toe-l": {"angleDeg": 0},
            "thigh-r": {"angleDeg": -2.5}, "shank-r": {"angleDeg": -2.5}, "foot-r": {"angleDeg": 5}, "toe-r": {"angleDeg": 0}}


def free_leg(side, knee):
    """🦵️ The relaxed leg of a weight shift: hip flexed by `knee`/2, knee by `knee`, the ball planted where it stands in the neutral stance."""
    return {f"thigh-{side}": {"angleDeg": -2.5 + FREE["thigh"]}, f"shank-{side}": {"angleDeg": -2.5 - FREE["shank"]}, f"foot-{side}": {"angleDeg": FREE["foot"]}, f"toe-{side}": {"angleDeg": FREE["toe"]}}


FREE = {"thigh": 10.0, "shank": 20.0, "foot": 8.9, "toe": 12.0}


def fingers(side, curl):
    """✋️ Finger flexion of a hand pushed into a pocket (curl 0 = the relaxed hand of the base figure)."""
    sg = 1 if side == "l" else -1
    out = {f"thumb-metacarpal-{side}": {"angleDeg": 0, "abductionDeg": sg * 9.8, "twistDeg": sg * 90}, f"thumb-proximal-{side}": {"angleDeg": 4 * curl, "abductionDeg": sg * 16}, f"thumb-distal-{side}": {"angleDeg": 6 * curl, "abductionDeg": sg * 6}}
    for i, f in enumerate(("index", "middle", "ring", "little")):
        out[f"{f}-proximal-{side}"] = {"angleDeg": 0, "abductionDeg": sg * (7 + i), "twistDeg": sg * 90}
        out[f"{f}-middle-{side}"] = {"angleDeg": 15 + 2 * i + 20 * curl}
        out[f"{f}-distal-{side}"] = {"angleDeg": 10 + i + 14 * curl}
    return out


def pockets(ang=-8, ab=0, tw=0, fa=36, hand=-30):
    """👖️ Both hands in the front jeans pockets: upper arm a little back, elbow bent so the hand slides into the pocket beside the hip (no twist, so the solids of the arm stay true in the front view)."""
    out = {"arm-l": {"angleDeg": ang, "abductionDeg": ab, "twistDeg": tw}, "forearm-l": {"angleDeg": fa}, "hand-l": {"angleDeg": hand},
           "arm-r": {"angleDeg": ang, "abductionDeg": -ab, "twistDeg": -tw}, "forearm-r": {"angleDeg": fa}, "hand-r": {"angleDeg": hand}}
    out.update(fingers("l", 1.5))
    out.update(fingers("r", 1.5))
    return out


REGIONS = ("face-skin", "hair", "hand-skin", "hoodie", "jeans", "sneakers", "backpack")


def binding(met, breathing, hands=0.35, sun=0.3):
    """🌡️ Thermal binding of a standing pose outdoors: hands mostly in the pockets, the pack over a sixth of the sweatshirt."""
    r = {"face-skin": {"exposedFraction": 1, "sunlitFraction": sun}, "hair": {"exposedFraction": 1, "sunlitFraction": sun + 0.15},
         "hand-skin": {"exposedFraction": hands, "sunlitFraction": round(sun * hands, 3)}, "hoodie": {"exposedFraction": 0.82, "sunlitFraction": sun},
         "jeans": {"exposedFraction": 0.96, "sunlitFraction": sun}, "sneakers": {"exposedFraction": 1, "sunlitFraction": round(sun * 0.6, 3), "contact": {"fraction": 0.3, "tempC": 13}},
         "backpack": {"exposedFraction": 0.9, "sunlitFraction": sun}}
    return {"activityMet": met, "breathing": breathing, "regions": r, "emitters": {"body-plume": 0.6}, "operating": {}}


def pose(pid, title, desc, bones, contacts, met, breathing, key=False, **kw):
    """🧍️ A named or key pose."""
    p = {"id": pid, "title": title, "description": desc}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(met, breathing, **kw), "loads": []})
    return p


def poses():
    """🧍️ The poses of the queue."""
    upright = {"lumbar": {"angleDeg": 0.0}, "torso": {"angleDeg": 0.0}, "neck": {"angleDeg": -1}, "head": {"angleDeg": 1}}
    n = {**legs_neutral(), **upright, **pockets()}
    wl = {**legs_neutral(), **free_leg("r", 1), **pockets(), "lumbar": {"angleDeg": 0.8}, "torso": {"angleDeg": -0.5}, "neck": {"angleDeg": -2}, "head": {"angleDeg": 2.5}}
    wl["arm-r"] = dict(wl["arm-r"], angleDeg=-7)
    wr = {**legs_neutral(), **free_leg("l", 1), **pockets(), "lumbar": {"angleDeg": 0.8}, "torso": {"angleDeg": -0.5}, "neck": {"angleDeg": -2.5}, "head": {"angleDeg": 0}}
    wr["arm-l"] = dict(wr["arm-l"], angleDeg=-7)
    hp = {**legs_neutral(), **upright, "head": {"angleDeg": -6}, "neck": {"angleDeg": -4}, **HOODIE_POCKET}
    hp.update(fingers("l", 0.6)); hp.update(fingers("r", 0.6))
    look = {**legs_neutral(), **pockets(), "lumbar": {"angleDeg": -1.5}, "torso": {"angleDeg": -1.5}, "neck": {"angleDeg": 6}, "head": {"angleDeg": 9}}
    four = ["heel-l", "ball-l", "heel-r", "ball-r"]
    return [
        pose("hands-in-pockets", tx("Hands in the pockets", "Hände in den Taschen"),
             tx("Waiting in line with the weight on both feet at hip-joint width, both hands pushed into the front pockets of the jeans up to the knuckles, elbows a little back, the daypack on both shoulders.",
                "In der Schlange wartend, das Gewicht auf beiden Füßen in Hüftgelenkbreite, beide Hände bis zu den Knöcheln in die vorderen Jeanstaschen geschoben, die Ellbogen etwas nach hinten, der Tagesrucksack auf beiden Schultern."),
             n, four, 1.2, "quiet-standing"),
        pose("weight-on-left", tx("Weight on the left leg", "Gewicht auf dem linken Bein"),
             tx("Standing on the straight left leg; the right knee is relaxed and bent by 20 degrees, the right heel lifted about 20 mm, the ball of the right foot still on the floor; the hips settle a little forward over the standing leg.",
                "Auf dem gestreckten linken Bein stehend; das rechte Knie ist locker um 20 Grad gebeugt, die rechte Ferse etwa 20 mm angehoben, der rechte Fußballen bleibt am Boden; die Hüfte schiebt sich etwas nach vorn über das Standbein."),
             wl, ["heel-l", "ball-l", "ball-r"], 1.25, "quiet-standing"),
        pose("weight-on-right", tx("Weight on the right leg", "Gewicht auf dem rechten Bein"),
             tx("The mirror of the left stance: standing on the straight right leg, the left knee relaxed, the left heel lifted, the hips settled over the standing leg.",
                "Das Spiegelbild des linken Stands: auf dem gestreckten rechten Bein stehend, das linke Knie locker, die linke Ferse angehoben, die Hüfte über dem Standbein."),
             wr, ["heel-r", "ball-r", "ball-l"], 1.25, "quiet-standing"),
        pose("hands-in-hoodie-pocket", tx("Hands in the sweatshirt pocket", "Hände in der Sweatshirttasche"),
             tx("Both hands pushed into the kangaroo pocket of the sweatshirt in front of the belly, forearms across the body, head lowered a little: keeping the hands warm while the queue does not move.",
                "Beide Hände in der Kängurutasche des Sweatshirts vor dem Bauch, die Unterarme quer vor dem Körper, der Kopf etwas gesenkt: Hände warm halten, während die Schlange stillsteht."),
             hp, four, 1.2, "quiet-standing", hands=0.15),
        pose("looking-ahead", tx("Looking ahead along the queue", "Nach vorne die Schlange entlang schauen"),
             tx("Hands in the front pockets, the chin raised and the upper body straightened to look over the heads in front towards the counter.",
                "Hände in den vorderen Taschen, das Kinn gehoben und der Oberkörper aufgerichtet, um über die Köpfe der Vorderen zum Schalter zu schauen."),
             look, four, 1.2, "quiet-standing"),
    ]


HOODIE_POCKET = {"arm-l": {"angleDeg": 18, "abductionDeg": 2, "twistDeg": 70}, "forearm-l": {"angleDeg": 74}, "hand-l": {"angleDeg": 10},
                 "arm-r": {"angleDeg": 18, "abductionDeg": -2, "twistDeg": -70}, "forearm-r": {"angleDeg": 74}, "hand-r": {"angleDeg": 10}}


def clips():
    """🎞️ The weight-shift loop and quiet breathing."""
    four = ("heel-l", "ball-l", "heel-r", "ball-r")
    shift = {"id": "weight-shift", "title": tx("Shifting the weight while waiting", "Gewicht verlagern beim Warten"),
             "description": tx("An eight-second loop of waiting in line: from the left stance through both feet flat to the right stance and back; the relaxed knee bends, its heel lifts and sets down again, the hips settle over the standing leg and the head nods a little; the balls of both feet stay planted.",
                               "Eine Schleife von acht Sekunden in der Warteschlange: vom linken Stand über beide flachen Füße zum rechten Stand und zurück; das lockere Knie beugt sich, seine Ferse hebt sich und setzt wieder auf, die Hüfte setzt sich über das Standbein und der Kopf nickt ein wenig; die Ballen beider Füße bleiben stehen."),
             "durationS": 8, "fps": 30, "loop": True, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "weight-on-left", "ease": "smooth"},
                      {"t": 1.4, "pose": "weight-on-left", "ease": "smooth", "bones": {"head": {"angleDeg": 1}, "neck": {"angleDeg": -1}}},
                      {"t": 2.6, "pose": "hands-in-pockets", "ease": "smooth"},
                      {"t": 4.0, "pose": "weight-on-right", "ease": "smooth"},
                      {"t": 5.4, "pose": "weight-on-right", "ease": "smooth", "bones": {"head": {"angleDeg": 1.5}, "neck": {"angleDeg": -1.5}}},
                      {"t": 6.6, "pose": "hands-in-pockets", "ease": "smooth"},
                      {"t": 8.0, "pose": "weight-on-left", "ease": "smooth"}],
             "contacts": [{"anchor": "ball-l", "fromS": 0, "toS": 8}, {"anchor": "ball-r", "fromS": 0, "toS": 8},
                          {"anchor": "heel-l", "fromS": 0, "toS": 2.6}, {"anchor": "heel-l", "fromS": 6.6, "toS": 8},
                          {"anchor": "heel-r", "fromS": 2.6, "toS": 6.6}],
             "phases": [{"id": "left-stance", "fromS": 0, "toS": 1.4, "title": tx("Standing on the left leg", "Stand auf dem linken Bein")},
                        {"id": "to-both", "fromS": 1.4, "toS": 2.6, "title": tx("Setting the right heel down", "Rechte Ferse aufsetzen")},
                        {"id": "to-right", "fromS": 2.6, "toS": 4.0, "title": tx("Shifting onto the right leg", "Auf das rechte Bein verlagern")},
                        {"id": "right-stance", "fromS": 4.0, "toS": 5.4, "title": tx("Standing on the right leg", "Stand auf dem rechten Bein")},
                        {"id": "back-to-both", "fromS": 5.4, "toS": 6.6, "title": tx("Setting the left heel down", "Linke Ferse aufsetzen")},
                        {"id": "to-left", "fromS": 6.6, "toS": 8.0, "title": tx("Shifting onto the left leg", "Auf das linke Bein verlagern")}],
             "events": [{"t": 2.6, "kind": "contact-on", "title": tx("Right heel down", "Rechte Ferse auf"), "target": "heel-r"},
                        {"t": 2.6, "kind": "contact-off", "title": tx("Left heel up", "Linke Ferse hoch"), "target": "heel-l"},
                        {"t": 6.6, "kind": "contact-on", "title": tx("Left heel down", "Linke Ferse auf"), "target": "heel-l"},
                        {"t": 6.6, "kind": "contact-off", "title": tx("Right heel up", "Rechte Ferse hoch"), "target": "heel-r"}],
             "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 8, "v": 0}]},
                        {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.25}, {"t": 2.6, "v": 1.2}, {"t": 4.0, "v": 1.25}, {"t": 6.6, "v": 1.2}, {"t": 8, "v": 1.25}]}]}
    idle = {"id": "idle-breathing", "title": tx("Idle breathing", "Ruhiges Atmen"),
            "description": tx("A five-second loop at 1.2 met with the hands in the pockets: the chest rises and falls with quiet nose breathing, the head settles in a small slow nod; the feet stay planted.",
                              "Eine Schleife von fünf Sekunden bei 1,2 met mit den Händen in den Taschen: Die Brust hebt und senkt sich mit ruhigem Nasenatmen, der Kopf nickt klein und langsam; die Füße bleiben stehen."),
            "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "hands-in-pockets", "ease": "smooth"},
                     {"t": 1.25, "pose": "hands-in-pockets", "ease": "smooth", "bones": {"torso": {"angleDeg": -0.36}, "head": {"angleDeg": 2}, "lumbar": {"angleDeg": -0.24}}},
                     {"t": 2.5, "pose": "hands-in-pockets", "ease": "smooth", "bones": {"torso": {"angleDeg": 0.24}, "head": {"angleDeg": 0.2}, "lumbar": {"angleDeg": 0.16}}},
                     {"t": 3.75, "pose": "hands-in-pockets", "ease": "smooth", "bones": {"torso": {"angleDeg": -0.3}, "head": {"angleDeg": 1.6}, "lumbar": {"angleDeg": -0.2}}},
                     {"t": 5, "pose": "hands-in-pockets", "ease": "smooth"}],
            "contacts": [{"anchor": k, "fromS": 0, "toS": 5} for k in four],
            "phases": [{"id": "idle", "fromS": 0, "toS": 5, "title": tx("Quiet standing", "Ruhiges Stehen")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 0}]}]}
    return [shift, idle]


def thermal(a):
    """🌡️ Body, respiration, regions and the spring morning outdoors."""
    t = a["thermal"]
    t["environment"] = {"airTempC": 14, "meanRadiantTempC": 17, "skyTempC": 2, "groundTempC": 13, "relativeHumidity": 0.62, "windMS": 1.2, "windDirectionDeg": 160,
                        "solarWM2": 320, "diffuseFraction": 0.35, "pressurePa": 101325}
    t["baseMet"] = 1.2
    skin = round(dubois(BODY_KG, STATURE), 3)
    bmr = (10 * BODY_KG + 6.25 * STATURE * 100 - 5 * AGE + 5) * 4184 / 86400
    t["body"].update({"massKg": BODY_KG, "skinAreaM2": skin, "basalMetabolismW": round(bmr, 1), "coreSetPointC": 36.8, "skinSetPointC": 33.9, "source": "authored-estimate", "reference": "mifflin-1990"})
    t["respiration"]["patterns"] = [
        {"id": "quiet-standing", "title": tx("Quiet standing", "Ruhiges Stehen"), "rateBpm": 12, "tidalVolumeL": 0.55, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "sighing", "title": tx("Impatient sigh", "Ungeduldiges Seufzen"), "rateBpm": 9, "tidalVolumeL": 1.1, "inhaleFraction": 0.34, "pauseAfterInhaleFraction": 0.06, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.1, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "talking", "title": tx("Chatting in the queue", "Plaudern in der Schlange"), "rateBpm": 16, "tidalVolumeL": 0.6, "inhaleFraction": 0.25, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.68, "pauseAfterExhaleFraction": 0.05, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"}]
    reg = {r["id"]: r for r in t["regions"]}
    reg["hair"]["title"] = tx("Short brown hair", "Kurzes braunes Haar")
    reg["hair"]["layers"][0]["title"] = tx("Short brown hair", "Kurzes braunes Haar")
    reg["hand-skin"]["title"] = tx("Hands, mostly in the pockets", "Hände, meist in den Taschen")
    reg["hoodie"]["title"] = tx("T-shirt and charcoal fleece sweatshirt on trunk and arms", "T-Shirt und anthrazitfarbenes Fleece-Sweatshirt an Rumpf und Armen")
    reg["hoodie"]["layers"][1]["title"] = tx("Fleece sweatshirt, hood down", "Fleece-Sweatshirt, Kapuze unten")
    reg["jeans"]["title"] = tx("Light-washed jeans over the legs", "Hell gewaschene Jeans über den Beinen")
    reg["jeans"]["layers"][1]["title"] = tx("Straight light-washed jeans", "Gerade geschnittene, hell gewaschene Jeans")
    reg["sneakers"]["title"] = tx("Socks and white leather sneakers", "Socken und weiße Ledersneaker")
    reg["sneakers"]["layers"][1]["title"] = tx("Leather sneaker", "Ledersneaker")
    reg["sneakers"]["layers"][1]["resistanceM2KW"] = 0.0093
    reg["sneakers"]["layers"][1]["vapourResistanceM2PaW"] = 2.5
    t["regions"].append({"id": "backpack", "title": tx("Daypack on the back", "Tagesrucksack auf dem Rücken"), "kind": "surface", "model": "passive", "material": "polyester-pack",
                         "areaM2": PACK_AREA, "characteristicLengthM": 0.3, "geometry": "vertical-plate", "skyViewFactor": 0.45, "layers": [], "heatCapacityJM2K": 3200,
                         "source": "authored-estimate", "reference": "ashrae-2021"})
    t["emitters"] = [e for e in t["emitters"]]
    for e in t["emitters"]:
        if e["kind"] == "plume":
            e["region"] = "hoodie"
    t["assumptions"] = [
        tx(f"Skin area {skin} m2 after DuBois and DuBois (1916) for {BODY_KG:.0f} kg and {STATURE} m; regions keep the skin fractions of the base figure (0.07 face and neck, 0.03 scalp, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet). Basal metabolism {bmr:.1f} W after Mifflin et al. (1990) for a man of {AGE}.",
           f"Hautfläche {skin} m2 nach DuBois und DuBois (1916) für {BODY_KG:.0f} kg und {STATURE} m; die Regionen behalten die Hautanteile der Ausgangsfigur (0,07 Gesicht und Hals, 0,03 Kopfhaut, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße). Grundumsatz {bmr:.1f} W nach Mifflin et al. (1990) für einen Mann von {AGE} Jahren."),
        tx("Clothing is about 0.7 clo (T-shirt, fleece sweatshirt, jeans, socks, leather sneakers) after ISO 9920 (1 clo = 0.155 m2K/W); the daypack covers about a sixth of the sweatshirt, which the pose bindings state as an exposed fraction of 0.82; the pack itself is a passive surface warmed by the back. Hands in the pockets expose only about a third of their skin.",
           "Die Kleidung hat etwa 0,7 clo (T-Shirt, Fleece-Sweatshirt, Jeans, Socken, Ledersneaker) nach ISO 9920 (1 clo = 0,155 m2K/W); der Tagesrucksack bedeckt etwa ein Sechstel des Sweatshirts, was die Posenbindungen als freien Anteil von 0,82 angeben; der Rucksack selbst ist eine passive Fläche, die vom Rücken erwärmt wird. Hände in den Taschen legen nur etwa ein Drittel ihrer Haut frei."),
        tx("Reference case: a queue outdoors on a spring morning, 14 degrees C air, 1.2 m/s wind, 320 W/m2 of sun with a clear sky at 2 degrees C; standing relaxed is 1.2 met (ISO 7730 Annex A), shifting the weight 1.25 met (authored).",
           "Referenzfall: eine Warteschlange im Freien an einem Frühlingsmorgen, Luft 14 Grad C, Wind 1,2 m/s, 320 W/m2 Sonne bei klarem Himmel von 2 Grad C; entspanntes Stehen sind 1,2 met (ISO 7730 Anhang A), Gewichtsverlagern 1,25 met (geschätzt)."),
        tx("Breathing patterns are authored after quiet adult values (ICRP 66) and Gupta et al. (2010) for talking; the impatient sigh is a slow deep breath through nose and mouth.",
           "Die Atemmuster sind nach ruhigen Erwachsenenwerten (ICRP 66) und Gupta et al. (2010) für das Sprechen geschätzt; das ungeduldige Seufzen ist ein langsamer tiefer Atemzug durch Nase und Mund."),
    ]


PACK_AREA = 0.42


def texts(a):
    """🗣️ Title, summary, tags and provenance."""
    a["id"] = SLUG
    a["title"] = tx("Person Queuing", "Person in der Warteschlange")
    a["summary"] = tx(
        "A man of 1.78 m and 78 kg waits in a queue outdoors in a charcoal hooded sweatshirt, light-washed jeans and white leather sneakers, an 18 litre daypack on his back and both hands in the front pockets of his jeans. Poses: hands in the pockets, standing on the left or the right leg with the other knee relaxed, hands in the sweatshirt pocket and looking ahead along the queue; loops: shifting the weight from leg to leg and quiet breathing at about 1.2 met.",
        "Ein Mann von 1,78 m und 78 kg wartet im Freien in einer Schlange, in anthrazitfarbenem Kapuzensweatshirt, hell gewaschenen Jeans und weißen Ledersneakern, einen Tagesrucksack von 18 Litern auf dem Rücken und beide Hände in den vorderen Jeanstaschen. Posen: Hände in den Taschen, Stand auf dem linken oder rechten Bein mit lockerem anderem Knie, Hände in der Sweatshirttasche und nach vorne die Schlange entlang schauen; Schleifen: Gewicht von Bein zu Bein verlagern und ruhiges Atmen bei etwa 1,2 met.")
    a["tags"] = ["human", "adult", "man", "standing", "waiting", "queue", "crowd", "hands-in-pockets", "weight-shift", "backpack", "outdoor"]
    pv = a["provenance"]
    pv["author"] = "Semio staffage human worker HW-17"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    pv["notes"] = tx(
        "Rig, outlines and the four handcrafted volumes are derived from the archive's standing-adult figure: every length is scaled by 1.78/1.69 about the ground origin (stature 1.78 m barefoot, 1.80 m shod), the segment masses follow the male table of de Leva (1996) for 78 kg with 1.5 kg of clothing; the daypack (0.42 x 0.29 x 0.15 m, 4.2 kg), all poses and clips, materials, tones and the outdoor scenario are authored for this person. Breadths and depths scale with the stature and are within the ANSUR II and DIN 33402-2 ranges for men of 1.78 m; the centres of mass keep the scaled positions of the base figure.",
        "Skelett, Umrisse und die vier handgefertigten Volumen sind aus der Archivfigur standing-adult abgeleitet: Jede Länge ist um 1,78/1,69 um den Bodenursprung skaliert (Körperhöhe barfuß 1,78 m, mit Schuhen 1,80 m), die Segmentmassen folgen der Männertabelle von de Leva (1996) für 78 kg mit 1,5 kg Kleidung; der Tagesrucksack (0,42 x 0,29 x 0,15 m, 4,2 kg), alle Posen und Clips, Materialien, Tonwerte und das Szenario im Freien sind für diese Person geschaffen. Breiten und Tiefen skalieren mit der Körperhöhe und liegen in den Bereichen von ANSUR II und DIN 33402-2 für Männer von 1,78 m; die Schwerpunkte behalten die skalierten Lagen der Ausgangsfigur.")


def main():
    doc = load("standing-adult")
    a = doc["asset"]
    scale_asset(a, S_, 1.0)
    remass(a)
    retone(a)
    materials(a)
    tmp = f"{GEN}/pq-rest.json"
    json.dump(doc, open(tmp, "w"))
    rest = frames(tmp)["rest"]
    out, pocket_secs, origin = add_pack(a, rest)
    pack_shapes(a, rest, out, pocket_secs, origin)
    global PACK_AREA
    PACK_AREA = round(pk.surface_area(out["50"], PACK_EXP, 0.8), 3)
    texts(a)
    thermal(a)
    a["poses"] = poses()
    a["clips"] = clips()
    a["presentation"] = {"defaultPose": "hands-in-pockets", "defaultClip": "weight-shift", "defaultScale": 100}
    a["structure"]["supports"] = ["ball-l", "ball-r"]
    for vol in a["volumes"].values():
        vol["structure"] = [s for s in vol["structure"] if not (s.get("part") == "support" and s.get("anchor") in ("heel-l", "heel-r"))]
    a["structure"]["assumptions"] = [
        tx("Segments are point masses at the scaled centres of the base figure with the masses of the male table of de Leva (1996); the daypack is a 4.2 kg point mass on its own fixed bone, carried into the thoracic spine.",
           "Segmente sind Punktmassen in den skalierten Schwerpunkten der Ausgangsfigur mit den Massen der Männertabelle von de Leva (1996); der Tagesrucksack ist eine Punktmasse von 4,2 kg an einem eigenen festen Knochen, die in die Brustwirbelsäule geleitet wird."),
        tx("Contacts are heel and ball of each foot; in the weight-shift stances the relaxed leg keeps only the ball on the floor and carries little load.",
           "Kontakte sind Ferse und Ballen jedes Fußes; in den Ständen der Gewichtsverlagerung behält das lockere Bein nur den Ballen am Boden und trägt wenig Last.")]
    a["dimensions"]["massKg"] = round(mass_sum(a), 2)
    write(SLUG, doc)
    fit_arms(SLUG, "hands-in-pockets", {"50": ("sleeve-{s}", "sleeve-{s}"), "100": ("sleeve-{s}", "sleeve-{s}"), "200": ("sleeve-{s}", "sleeve-{s}"), "500": ("sil-arm-{s}", "sleeve-{s}")})
    w, h, d = volume_dims(SLUG, "hands-in-pockets", 50)
    doc = load(SLUG)
    doc["asset"]["dimensions"].update({"widthM": round(w, 3), "heightM": round(h, 3), "depthM": round(d, 3)})
    write(SLUG, doc)
    print("[DEBUG] written", SLUG, "mass", a["dimensions"]["massKg"], "pack area", PACK_AREA, "dims", doc["asset"]["dimensions"])


if __name__ == "__main__":
    main()
