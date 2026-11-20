"""🦆 Round 3 mallard: the corrected roots outside `volumes` and the 2D drawings, derived from the untouched snapshot `r3-mallard-duck-orig.json`.

Rig: the trunk hinges at the hips (acetabula) instead of the tail base and is as long as the real column from hips to neck base; the hips sit under the synsacrum, not at the belly; hips, knees and shoulders get their lateral positions; the neck has the length of the real cervical column (165 mm, not 205 mm) and rests in the S a standing mallard carries; the folded wing reaches from the shoulder joint to the primary tips (249 mm); the foot is as long as the middle toe (58 mm). Leg bone lengths and leg offsets stay: they were right. Poses: the standing far leg is solved anew for a foot 22 mm behind the near one (knee ahead of the hip-ankle line; it stood 60 mm back with the shank level under the rump); the legs of the water poses are solved anew (ankle 45 mm further towards the belly side, knee ahead) because the old ones were folded for hips at the belly; all neck offsets stay, so every pose keeps its head direction.
"""
import copy
import json
import math

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ORIG = json.load(open(T + "/r3-mallard-duck-orig.json", encoding="utf-8"))

BONES = {
    "trunk": {"x": 0, "y": 0, "lengthM": 0.105, "comX": 0.035, "comY": -0.035},
    "tail": {"x": -0.088, "y": 0.004, "angleDeg": 160},
    "neck-1": {"x": 0.105, "y": -0.02, "angleDeg": 46, "lengthM": 0.06, "comX": 0.03},
    "neck-2": {"x": 0.06, "angleDeg": 53, "lengthM": 0.055, "comX": 0.0275},
    "neck-3": {"x": 0.055, "angleDeg": -18, "lengthM": 0.05, "comX": 0.025},
    "head": {"x": 0.05, "angleDeg": -91},
    "wing": {"x": 0.105, "y": -0.015, "z": 0.03, "angleDeg": 169.5, "lengthM": 0.249, "comX": 0.09},
    "thigh": {"x": 0, "y": 0, "z": 0.02},
    "shank": {"z": 0.018},
    "foot": {"lengthM": 0.058, "comX": 0.029},
}
WATER_DROP = 0.045
FEMUR, TIBIOTARSUS = 0.05, 0.085
REACH = 0.118
STANCE = {"thigh-f": 12.0, "shank-f": -63.0, "tarsus-f": 34.0}
UPRIGHT = ("standing", "wing-stretch", "waddle-")
SOLE = -0.006
ANCHORS = {
    "contact-n": {"x": 0.03, "y": SOLE, "footprintLengthM": 0.058, "footprintWidthM": 0.05},
    "contact-f": {"x": 0.03, "y": SOLE, "footprintLengthM": 0.058, "footprintWidthM": 0.05},
    "waterline": {"x": 0.03, "y": -0.05, "footprintLengthM": 0.26, "footprintWidthM": 0.12},
    "nostrils": {"x": 0.064, "y": 0.0095},
    "bill-tip": {"x": 0.1055, "y": -0.0024},
    "eye": {"x": 0.029, "y": 0.0158},
}
TITLES = {
    "trunk": {"en": "Trunk: thoracic column and synsacrum from the hips to the neck base, with ribcage, keel, flight muscles and gut", "de": "Rumpf: Brustwirbelsäule und Synsacrum von den Hüften bis zum Halsansatz, mit Brustkorb, Kiel, Flugmuskeln und Darm"},
    "wing-n": {"en": "Near wing, folded on the flank, from the shoulder joint to the primary tips", "de": "Naher Flügel, an der Flanke angelegt, vom Schultergelenk bis zu den Handschwingenspitzen"},
    "wing-f": {"en": "Far wing, folded on the flank, from the shoulder joint to the primary tips", "de": "Ferner Flügel, an der Flanke angelegt, vom Schultergelenk bis zu den Handschwingenspitzen"},
}
EYE = {"id": "eye-tissue", "title": {"en": "Eye (cornea and lids)", "de": "Auge (Hornhaut und Lider)"}, "emissivity": 0.97, "solarAbsorptivity": 0.9, "source": "authored-estimate", "reference": "ashrae-2021"}
MATERIAL = {"id": "bone", "title": {"en": "Bone (pneumatised long bones, fused trunk)", "de": "Knochen (pneumatisierte Röhrenknochen, verwachsener Rumpf)"}, "emissivity": 0.95, "densityKgM3": 2100, "elasticModulusPa": 17000000000, "strengthPa": 150000000, "source": "authored-estimate", "reference": "dumont-2010"}
REFERENCES = [
    {"id": "woelfle-1967", "citation": "Woelfle, E. (1967). Vergleichend morphologische Untersuchungen an Einzelknochen des postcranialen Skelettes in Mitteleuropa vorkommender Enten, Halbgänse und Säger. Dissertation, Universität München. Mallard long bones, greatest lengths, about: humerus 92 mm, ulna 78 mm, carpometacarpus 57 mm, femur 50 mm, tibiotarsus 86 mm, tarsometatarsus 45 mm (values as remembered from the tables, rounded; the rig uses them as authored estimates)."},
    {"id": "dumont-2010", "citation": "Dumont, E. R. (2010). Bone density and the lightweight skeletons of birds. Proceedings of the Royal Society B 277, 2193-2198. Bird bone tissue is denser (about 2.1 g/cm3) and stiffer than that of mammals of the same size; modulus and strength here are authored estimates for cortical bone."},
]
CRAMP_ADD = " Drake biometrics: wing 257-285 mm, tail 80-91 mm, bill 51-61 mm, tarsus 42-48 mm, middle toe 52-60 mm."


def rig():
    r = copy.deepcopy(ORIG["rig"])
    for b in r["bones"]:
        key = b["id"] if b["id"] in BONES else b["id"][:-2]
        spec = BONES.get(key, {})
        sign = -1 if b["id"].endswith("-n") else 1
        for k, v in spec.items():
            if k == "z":
                b["z"] = sign * v
            elif k in ("comX", "comY"):
                b["mass"][k] = v
            else:
                b[k] = v
        if b["id"] in TITLES:
            b["title"] = TITLES[b["id"]]
        if "z" in b:
            z = b.pop("z")
            items = list(b.items())
            i = [k for k, _ in items].index("y") + 1
            b.clear()
            b.update(items[:i] + [("z", z)] + items[i:])
    return r


def poses(report):
    limits = {b["id"]: (b["joint"]["minDeg"], b["joint"]["maxDeg"]) for b in rig()["bones"]}
    out = []
    for p in copy.deepcopy(ORIG["poses"]):
        p["root"] = {**p.get("root", {}), "x": 0}
        if p["id"] in ("standing", "wing-stretch"):
            for bone, angle in STANCE.items():
                p["bones"][bone]["angleDeg"] = angle
        if not p["id"].startswith(UPRIGHT):
            water_legs(p, report)
        if p["id"].startswith("waddle-"):
            walk_legs(p, report)
        for bone, off in p["bones"].items():
            lo, hi = limits[bone]
            if not lo <= off.get("angleDeg", 0) <= hi:
                report.append("%s %s %.1f outside %s" % (p["id"], bone, off["angleDeg"], limits[bone]))
        out.append(p)
    return out


def chain(bones, pose):
    """World angle and origin of every bone of the sagittal chain of a pose (no ground shift)."""
    out = {}
    for b in bones:
        off = pose["bones"].get(b["id"], {}).get("angleDeg", 0)
        if b["parent"] is None:
            out[b["id"]] = {"a": b["angleDeg"] + off, "o": (0.0, 0.0)}
            continue
        q = out[b["parent"]]
        c, n = math.cos(math.radians(q["a"])), math.sin(math.radians(q["a"]))
        out[b["id"]] = {"a": q["a"] + b["angleDeg"] + off, "o": (q["o"][0] + c * b["x"] - n * b["y"], q["o"][1] + n * b["x"] + c * b["y"])}
    return out


def norm(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def water_legs(p, report):
    """Legs of a water pose: the old poses were solved for hips at the belly; with the hips under the back the ankle keeps its old place relative to the hip, lowered towards the belly side by WATER_DROP (less where the leg would stretch beyond REACH), and thigh and shank are solved anew with the knee ahead of the hip-ankle line; tarsus and foot keep their world directions."""
    old = chain(ORIG["rig"]["bones"], p)
    trunk = old["trunk"]["a"]
    for side in ("-n", "-f"):
        hip, ankle = old["thigh" + side]["o"], old["tarsus" + side]["o"]
        rel = (ankle[0] - hip[0], ankle[1] - hip[1])
        drop = min(WATER_DROP, max(0.0, REACH - math.hypot(*rel)))
        rel = (rel[0] + math.sin(math.radians(trunk)) * drop, rel[1] - math.cos(math.radians(trunk)) * drop)
        d = math.hypot(*rel)
        if d > REACH + 0.006:
            report.append("%s leg%s shortened from %.1f to %.1f mm" % (p["id"], side, d * 1000, REACH * 1000 + 6))
            rel, d = (rel[0] * (REACH + 0.006) / d, rel[1] * (REACH + 0.006) / d), REACH + 0.006
        base = math.degrees(math.atan2(rel[1], rel[0]))
        inner = math.degrees(math.acos(max(-1, min(1, (FEMUR ** 2 + d * d - TIBIOTARSUS ** 2) / (2 * FEMUR * d)))))
        thigh = base + inner
        knee = (math.cos(math.radians(thigh)) * FEMUR, math.sin(math.radians(thigh)) * FEMUR)
        shank = math.degrees(math.atan2(rel[1] - knee[1], rel[0] - knee[0]))
        tarsus = old["tarsus" + side]["a"]
        p["bones"]["thigh" + side]["angleDeg"] = round(norm(thigh - trunk + 60), 2)
        p["bones"]["shank" + side]["angleDeg"] = round(norm(shank - thigh + 50), 2)
        p["bones"]["tarsus" + side]["angleDeg"] = round(norm(tarsus - shank - 40), 2)


def walk_legs(p, report):
    """Legs of a waddle key: the toe joint (foot origin) and the foot's world direction stay where the clip puts them (stance feet planted, swing feet lifted), but the old offsets on the new hips raised the swinging knee above the back (up to 43 mm over the hip). The tarsometatarsus is turned, as little as possible from its old direction, until the ankle lies 80 to 130 mm from the hip, so that the knee of the forward IK solution stays inside the flank (at most 5 mm over the hip, as in a walking duck whose femur barely moves); thigh and shank are solved for it."""
    bones = rig()["bones"]
    c = chain(bones, p)
    trunk = c["trunk"]["a"]
    for side in ("-n", "-f"):
        mtp, foot, old = c["foot" + side]["o"], c["foot" + side]["a"], c["tarsus" + side]["a"]
        best = None
        for k in range(-180, 181):
            th = old + k * 0.5
            ankle = (mtp[0] - math.cos(math.radians(th)) * 0.045, mtp[1] - math.sin(math.radians(th)) * 0.045)
            d = math.hypot(*ankle)
            if not 0.08 <= d <= 0.13:
                continue
            base = math.degrees(math.atan2(ankle[1], ankle[0]))
            inner = math.degrees(math.acos(max(-1, min(1, (FEMUR ** 2 + d * d - TIBIOTARSUS ** 2) / (2 * FEMUR * d)))))
            sols = [base + inner, base - inner]
            thigh = max(sols, key=lambda a: math.cos(math.radians(a)))
            knee = (math.cos(math.radians(thigh)) * FEMUR, math.sin(math.radians(thigh)) * FEMUR)
            if knee[1] > 0.005:
                continue
            if best is None or abs(k) < abs(best[0]):
                best = (k, th, thigh, math.degrees(math.atan2(ankle[1] - knee[1], ankle[0] - knee[0])))
        if best is None:
            report.append("%s leg%s: no ankle keeps the knee in the flank" % (p["id"], side))
            continue
        _, tarsus, thigh, shank = best
        p["bones"]["thigh" + side]["angleDeg"] = round(norm(thigh - trunk + 60), 2)
        p["bones"]["shank" + side]["angleDeg"] = round(norm(shank - thigh + 50), 2)
        p["bones"]["tarsus" + side]["angleDeg"] = round(norm(tarsus - shank - 40), 2)
        p["bones"]["foot" + side]["angleDeg"] = round(norm(foot - tarsus - 75), 2)


def anchors():
    out = copy.deepcopy(ORIG["anchors"])
    for a in out:
        a.update(ANCHORS.get(a["id"], {}))
    return out


def provenance():
    p = copy.deepcopy(ORIG["provenance"])
    for r in p["references"]:
        if r["id"] == "cramp-simmons-1977" and CRAMP_ADD not in r["citation"]:
            r["citation"] += CRAMP_ADD
    have = {r["id"] for r in p["references"]}
    p["references"] += [r for r in REFERENCES if r["id"] not in have]
    return p


def materials():
    out = copy.deepcopy(ORIG["materials"])
    return out + [m for m in (EYE, MATERIAL) if not any(q["id"] == m["id"] for q in out)]


def members(report=None):
    report = [] if report is None else report
    return {"provenance": provenance(), "materials": materials(), "rig": rig(), "anchors": anchors(), "poses": poses(report)}


if __name__ == "__main__":
    notes = []
    m = members(notes)
    print("\n".join(notes) or "all poses inside the joint limits")
    for b in m["rig"]["bones"]:
        print(b["id"].ljust(9), b["x"], b["y"], b.get("z", ""), b["angleDeg"], b["lengthM"])
