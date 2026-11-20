"""🦴 Round 3 pigeon: the corrected roots outside `volumes` and the 2D drawings, derived from the untouched snapshot `r3-pigeon-orig.json`.

Rig: limb bones take the skeletal means of Johnston (1990) and the wing chord of Gibbs et al. (2001); hips and shoulders get their lateral positions; the folded wing is the real Z (elbow at the back, wrist at the side of the breast, primaries along the lower edge to the tail). Poses: every leg is re-solved so that the foot joint and the directions of tarsus and toes stay where they were (knee ahead of the hip-ankle line); open-wing offsets are shifted by the change of the folded rest angles so that the open wing keeps its shape.
"""
import copy
import json
import math

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
ORIG = json.load(open(T + "/r3-pigeon-orig.json", encoding="utf-8"))
ORIG_MATERIALS = json.load(open(T + "/r3-pigeon-orig-materials.json", encoding="utf-8"))

FEMUR, TIBIOTARSUS, TARSOMETATARSUS = 0.039, 0.057, 0.031
HUMERUS, ULNA, HAND = 0.045, 0.054, 0.2
HIP_Z, SHOULDER_Z = 0.02, 0.022
FOREARM_REST, HAND_REST = -200, 202
SOLE = -0.003
HIP_IN_TRUNK = (0.05302, 0.00352)
TRUNK_LENGTH = 0.11
TRUNK_COM_Y = -0.022
JAW = {"x": 0.008, "y": -0.009, "angleDeg": 7.5, "lengthM": 0.046, "comX": 0.022}
LIMITS = {"thigh": (-60, 60), "shank": (-60, 55), "tarsus": (-50, 70), "toes": (-60, 45), "forearm": (-10, 215), "hand": (-205, 10)}
MATERIAL = {"id": "bone", "title": {"en": "Bone (pneumatised long bones, fused trunk)", "de": "Knochen (pneumatisierte Röhrenknochen, verwachsener Rumpf)"}, "emissivity": 0.95, "densityKgM3": 2100, "elasticModulusPa": 17000000000, "strengthPa": 150000000, "source": "authored-estimate", "reference": "dumont-2010"}
REFERENCES = [
    {"id": "dumont-2010", "citation": "Dumont, E. R. (2010). Bone density and the lightweight skeletons of birds. Proceedings of the Royal Society B 277, 2193-2198. Bird bone tissue is denser (about 2.1 g/cm3) and stiffer than that of mammals of the same size; modulus and strength here are authored estimates for cortical bone."},
    {"id": "johnston-1990", "citation": "Johnston, R. F. (1990). Variation in size and shape in pigeons, Columba livia. Wilson Bulletin 102, 213-225. Skeletal means of 105 males and 89 females in mm: femur 39.1 / 37.1, tibiotarsus 58.1 / 56.0, tarsometatarsus 31.7 / 30.2, humerus 45.4 / 43.5, ulna 54.9 / 52.6."},
    {"id": "gibbs-2001", "citation": "Gibbs, D., Barnes, E., Cox, J. (2001). Pigeons and Doves: A Guide to the Pigeons and Doves of the World. Pica Press. Rock dove: wing chord about 223 mm, tail 95-110 mm, bill about 18 mm, tarsus 26-35 mm."},
]


def rad(d):
    return d * math.pi / 180


def chain(bones, pose, lengths=None):
    """World angle, origin and tip of every bone of the sagittal chain (root offsets included, no ground shift)."""
    out = {}
    root = pose.get("root", {})
    for b in bones:
        off = pose["bones"].get(b["id"], {})
        if b["parent"] is None:
            ang = root.get("angleDeg", 0) + b["angleDeg"] + off.get("angleDeg", 0)
            o = (root.get("x", 0) + b["x"], root.get("y", 0) + b["y"])
        else:
            p = out[b["parent"]]
            c, s = math.cos(rad(p["a"])), math.sin(rad(p["a"]))
            o = (p["o"][0] + c * b["x"] - s * b["y"], p["o"][1] + s * b["x"] + c * b["y"])
            ang = p["a"] + b["angleDeg"] + off.get("angleDeg", 0)
        out[b["id"]] = {"a": ang, "o": o, "tip": (o[0] + math.cos(rad(ang)) * b["lengthM"], o[1] + math.sin(rad(ang)) * b["lengthM"])}
    return out


def norm(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def rig():
    r = copy.deepcopy(ORIG["rig"])
    for b in r["bones"]:
        side = b["id"][-2:]
        sign = -1 if side == "-n" else 1
        part = b["id"][:-2]
        if b["id"] == "trunk":
            b["x"], b["y"], b["lengthM"] = 0, 0, TRUNK_LENGTH
            b["mass"]["comX"], b["mass"]["comY"] = round(b["mass"]["comX"] - HIP_IN_TRUNK[0], 5), TRUNK_COM_Y
        if b["parent"] == "trunk":
            b["x"], b["y"] = round(b["x"] - HIP_IN_TRUNK[0], 5), round(b["y"] - HIP_IN_TRUNK[1], 5)
        if b["id"] == "jaw":
            b["x"], b["y"], b["angleDeg"], b["lengthM"] = JAW["x"], JAW["y"], JAW["angleDeg"], JAW["lengthM"]
            b["mass"]["comX"] = JAW["comX"]
        if part == "thigh":
            b["z"] = sign * HIP_Z
            b["lengthM"] = FEMUR
            b["mass"]["comX"] = round(FEMUR / 2, 4)
        elif part == "shank":
            b["x"] = FEMUR
            b["lengthM"] = TIBIOTARSUS
            b["mass"]["comX"] = round(TIBIOTARSUS / 2, 4)
        elif part == "tarsus":
            b["x"] = TIBIOTARSUS
            b["lengthM"] = TARSOMETATARSUS
            b["mass"]["comX"] = round(TARSOMETATARSUS / 2, 4)
        elif part == "toes":
            b["x"] = TARSOMETATARSUS
        elif part == "humerus":
            b["z"] = sign * SHOULDER_Z
            b["lengthM"] = HUMERUS
            b["mass"]["comX"] = round(HUMERUS / 2, 4)
        elif part == "forearm":
            b["x"] = HUMERUS
            b["angleDeg"] = FOREARM_REST
            b["lengthM"] = ULNA
            b["mass"]["comX"] = round(ULNA / 2, 4)
        elif part == "hand":
            b["x"] = ULNA
            b["angleDeg"] = HAND_REST
            b["lengthM"] = HAND
            b["mass"]["comX"] = 0.06
        if part in LIMITS:
            b["joint"]["minDeg"], b["joint"]["maxDeg"] = LIMITS[part]
        if "z" in b:
            z = b.pop("z")
            keys = list(b.keys())
            i = keys.index("y") + 1
            items = list(b.items())
            b.clear()
            b.update(items[:i] + [("z", z)] + items[i:])
    return r


def solve_leg(old, hip, side, report):
    """New world angles of thigh, shank, tarsus for the old foot joint, tarsus and toe directions."""
    mtp = old["toes" + side]["o"]
    tarsus_a = old["tarsus" + side]["a"]
    ankle = (mtp[0] - math.cos(rad(tarsus_a)) * TARSOMETATARSUS, mtp[1] - math.sin(rad(tarsus_a)) * TARSOMETATARSUS)
    reach = FEMUR + TIBIOTARSUS - 0.0004
    d = math.hypot(ankle[0] - hip[0], ankle[1] - hip[1])
    if d > reach:
        dm = math.hypot(mtp[0] - hip[0], mtp[1] - hip[1])
        if dm >= reach + TARSOMETATARSUS:
            raise SystemExit("foot out of reach")
        a = (reach * reach - TARSOMETATARSUS ** 2 + dm * dm) / (2 * dm)
        h = math.sqrt(max(0, reach * reach - a * a))
        ux, uy = (mtp[0] - hip[0]) / dm, (mtp[1] - hip[1]) / dm
        cands = [(hip[0] + a * ux - h * uy, hip[1] + a * uy + h * ux), (hip[0] + a * ux + h * uy, hip[1] + a * uy - h * ux)]
        ankle = min(cands, key=lambda q: math.hypot(q[0] - ankle[0], q[1] - ankle[1]))
        new_tarsus = math.degrees(math.atan2(mtp[1] - ankle[1], mtp[0] - ankle[0]))
        report.append("tarsus%s turned %.1f deg (leg at full reach)" % (side, norm(new_tarsus - tarsus_a)))
        tarsus_a = tarsus_a + norm(new_tarsus - tarsus_a)
        d = reach
    base = math.atan2(ankle[1] - hip[1], ankle[0] - hip[0])
    inner = math.acos(max(-1, min(1, (FEMUR ** 2 + d * d - TIBIOTARSUS ** 2) / (2 * FEMUR * d))))
    thigh_a = math.degrees(base + inner)
    knee = (hip[0] + math.cos(rad(thigh_a)) * FEMUR, hip[1] + math.sin(rad(thigh_a)) * FEMUR)
    shank_a = math.degrees(math.atan2(ankle[1] - knee[1], ankle[0] - knee[0]))
    return thigh_a, shank_a, tarsus_a


def poses(report):
    bones_old = ORIG["rig"]["bones"]
    rest = {b["id"]: b["angleDeg"] for b in bones_old}
    out = []
    new_bones = rig()["bones"]
    for p in copy.deepcopy(ORIG["poses"]):
        old = chain(bones_old, p)
        b = p["bones"]
        notes = []
        for side in ("-n", "-f"):
            hip = old["thigh" + side]["o"]
            thigh_a, shank_a, tarsus_a = solve_leg(old, hip, side, notes)
            toes_a = old["toes" + side]["a"]
            vals = {
                "thigh": norm(thigh_a - old["pelvis"]["a"] - rest["thigh" + side]),
                "shank": norm(shank_a - thigh_a - rest["shank" + side]),
                "tarsus": norm(tarsus_a - shank_a - rest["tarsus" + side]),
                "toes": norm(toes_a - tarsus_a - rest["toes" + side]),
            }
            for part, v in vals.items():
                v = round(v, 1)
                lo, hi = LIMITS[part]
                if not lo <= v <= hi:
                    notes.append("%s%s %.1f outside %s" % (part, side, v, LIMITS[part]))
                key = part + side
                if abs(v) < 0.05:
                    if key in b and set(b[key]) <= {"angleDeg"}:
                        del b[key]
                    elif key in b:
                        b[key]["angleDeg"] = 0
                else:
                    b.setdefault(key, {})["angleDeg"] = v
            if "forearm" + side in b and "angleDeg" in b["forearm" + side]:
                b["forearm" + side]["angleDeg"] = round(b["forearm" + side]["angleDeg"] + (-167 - FOREARM_REST), 1)
                hand = b.setdefault("hand" + side, {})
                hand["angleDeg"] = round(hand.get("angleDeg", 0) + (163 - HAND_REST), 1)
        if "forearm-f" not in b:
            for key in ("humerus-n", "humerus-f", "hand-f"):
                if key in b and "abductionDeg" not in b[key]:
                    del b[key]
        if p["id"].startswith("pecking"):
            repitch(p, new_bones, report)
        order = [x["id"] for x in bones_old]
        p["bones"] = {k: b[k] for k in sorted(b, key=order.index)}
        if notes:
            report.append(p["id"] + ": " + "; ".join(notes))
        out.append(p)
    return out


def beak(bones, pose):
    c = chain(bones, pose)
    return c["head"]["tip"], c["head"]["a"]


def repitch(p, new_bones, report):
    """Trunk and neck offsets of a pecking pose, so that the beak tip (raised with the sole) and the head direction stay where the old rig (trunk hinged at the tail base) had them now that the trunk hinges at the hips: the nearest offsets inside the joint limits that reach the point."""
    target, head_a = beak(ORIG["rig"]["bones"], next(q for q in ORIG["poses"] if q["id"] == p["id"]))
    target = (target[0], target[1] + (0.0095 + SOLE))
    b = p["bones"]
    keys = ["trunk", "neck-1", "neck-2", "neck-3"]
    limit = {x["id"]: (x["joint"]["minDeg"], x["joint"]["maxDeg"]) for x in new_bones}
    start = [b.get(k, {}).get("angleDeg", 0) for k in keys]
    h0 = b.get("head", {}).get("angleDeg", 0)

    def put(v):
        for k, x in zip(keys, v):
            b.setdefault(k, {})["angleDeg"] = x
        b.setdefault("head", {})["angleDeg"] = h0 - sum(x - y for x, y in zip(v, start))
        tip, _ = beak(new_bones, p)
        return math.hypot(tip[0] - target[0], tip[1] - target[1])

    def cost(v):
        return put(v) + 0.000004 * sum((x - y) ** 2 for x, y in zip(v, start))

    v = list(start)
    step = 8.0
    while step > 0.04:
        moved = False
        for i, k in enumerate(keys):
            for d in (step, -step):
                w = list(v)
                w[i] = max(limit[k][0], min(limit[k][1], w[i] + d))
                if cost(w) < cost(v) - 1e-9:
                    v, moved = w, True
        if not moved:
            step /= 2
    v = [round(x, 1) for x in v]
    off = put(v)
    b["head"]["angleDeg"] = round(b["head"]["angleDeg"], 1)
    report.append("%s: %s, head %.1f -> %.1f, beak tip off by %.1f mm" % (p["id"], ", ".join("%s %.1f -> %.1f" % (k, x, y) for k, x, y in zip(keys, start, v)), h0, b["head"]["angleDeg"], 1000 * off))


def anchors():
    out = copy.deepcopy(ORIG["anchors"])
    for a in out:
        if a["id"].startswith("contact-"):
            a["y"] = SOLE
    return out


def provenance():
    p = copy.deepcopy(ORIG["provenance"])
    have = {r["id"] for r in p["references"]}
    p["references"] += [r for r in REFERENCES if r["id"] not in have]
    return p


def materials():
    out = copy.deepcopy(ORIG_MATERIALS)
    return out if any(m["id"] == "bone" for m in out) else out + [MATERIAL]


def members(report=None):
    report = [] if report is None else report
    return {"provenance": provenance(), "materials": materials(), "rig": rig(), "anchors": anchors(), "poses": poses(report)}


if __name__ == "__main__":
    notes = []
    m = members(notes)
    print("\n".join(notes) or "all poses solved inside the joint limits")
    new = chain(m["rig"]["bones"], m["poses"][0])
    old = chain(ORIG["rig"]["bones"], ORIG["poses"][0])
    for k in new:
        print(k.ljust(10), "old o %.4f %.4f a %7.1f | new o %.4f %.4f tip %.4f %.4f a %7.1f" % (*old[k]["o"], old[k]["a"], *new[k]["o"], *new[k]["tip"], new[k]["a"]))
    for p in m["poses"]:
        print(p["id"], json.dumps({k: v for k, v in p["bones"].items() if k[:5] in ("thigh", "shank", "tarsu", "toes-")}).replace('"', "").replace("angleDeg: ", ""))
