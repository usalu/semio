"""🦴️ Round 5 Labrador: the anatomical rig. Every joint of the standing dog is stated by hand in world metres (x ahead of the hip joint, y above the ground) from the reference table of r3-animals-labrador-dog.md and converted to the parent-relative bone table. Round 5: the thoracic column runs dorsally under the withers (T13 at 0.505 m, T1 at 0.488 m, from the thoracic inlet bounded by the manubrium), the neck leaves the thorax at 45 degrees, the otter tail droops from the croup and lifts at the tip. The legs rest in the standing position; spine, head, jaw, ears and tail keep the offsets of every pose. Each leg of each pose keeps its pad contact point, its paw inclination and its metatarsus or metacarpus inclination (all shifted by the constant difference between the old and the new standing dog); thigh and shank, upper arm and forearm are solved anew for the corrected bone lengths.
Usage: python3 r5-labrador-dog-rig.py <asset-before 🔣️.json> <out roots.json>"""
import json, math, sys

HIP_Y = 0.485
STAND = "standing-alert"
HOCK = (-0.110, 0.155)
FEMUR, TIBIA = 0.198, 0.202
SPINE = {
    "pelvis": ((0.0, 0.485), (0.055, 0.508)),
    "lumbar": ((0.055, 0.508), (0.245, 0.505)),
    "thorax": ((0.245, 0.505), (0.455, 0.488)),
    "neck": ((0.455, 0.488), (0.612, 0.648)),
}
FORE = {"scapula-f": ((0.4074, 0.552), (0.498, 0.4226)), "humerus-f": ((0.498, 0.4226), (0.3896, 0.2839)), "forearm-f": ((0.3896, 0.2839), (0.395, 0.105)), "metacarpus-f": ((0.395, 0.105), (0.410, 0.032)), "paw-f": ((0.410, 0.032), (0.480, 0.012))}
HIND_LOW = {"metatarsus-h": (HOCK, (-0.100, 0.034)), "paw-h": ((-0.100, 0.034), (-0.028, 0.012))}
STANDING_WORLD = {"head": ((0.612, 0.648), -8.0, 0.24), "tail-1": ((-0.072, 0.522), 196.0, 0.13), "tail-2": (None, 186.0, 0.12), "tail-3": (None, 176.0, 0.11)}
HEAD_LOCAL = {"jaw": ((0.03, -0.012), -4.9, 0.186), "ear": ((0.04, 0.04), -90.0, 0.105)}
LATERAL = {"thigh-h": 0.055, "shank-h": 0.025, "metatarsus-h": -0.012, "scapula-f": 0.035, "humerus-f": 0.04, "forearm-f": 0.005, "metacarpus-f": -0.005, "ear-": 0.06}
PAD = {"h": (-0.067, 0.0), "f": (0.445, 0.0)}
ANCHORS = {"nose": ("head", (0.241, 0.004)), "mouth": ("head", (0.2, -0.03)), "collar": ("neck", (0.11, -0.085)), "back-plume": ("thorax", None), "contact-rump": ("pelvis", None)}
BACK_PLUME_WORLD = (0.33, 0.558)
RUMP_WORLD = (-0.085, 0.375)
LIMITS = {"thigh-h": (-50, 75), "shank-h": (-95, 45), "metatarsus-h": (-45, 110), "paw-h": (-60, 60), "scapula-f": (-35, 40), "humerus-f": (-60, 60), "forearm-f": (-45, 110), "metacarpus-f": (-70, 110), "paw-f": (-60, 60)}
REMOVED = ["head-core"]
DIMENSIONS = {"heightM": 0.71, "widthM": 1.29, "depthM": 0.28, "massKg": 30}
BONE_MATERIAL = {"id": "bone", "title": {"en": "Bone", "de": "Knochen"}, "emissivity": 0.95, "densityKgM3": 1900, "elasticModulusPa": 17000000000, "source": "literature", "reference": "evans-2013"}
REFERENCES = [
    {"id": "fci-122", "citation": "Fédération Cynologique Internationale (2011). FCI-Standard No. 122, Labrador Retriever. Ideal height at the withers 56-57 cm (dogs), 54-56 cm (bitches); broad skull, defined stop, deep and wide chest with well sprung barrel ribs, otter tail of medium length."},
    {"id": "akc-labrador-standard", "citation": "American Kennel Club (1994). Official Standard for the Labrador Retriever. Point of shoulder to point of rump equal to or slightly longer than withers to ground; elbow to ground one half of the height at the withers; tail no longer than to the hock."},
    {"id": "evans-2013", "citation": "Evans, H. E., de Lahunta, A. (2013). Miller's Anatomy of the Dog, 4th edition. Elsevier Saunders. Skeleton, joints and surface landmarks; bone lengths here are authored for a 30 kg dog inside the ranges of large breeds."},
]

r4 = lambda v: round(v, 4) + 0.0
rad = math.radians

def fk(bones, offsets):
    by = {b["id"]: b for b in bones}
    out = {}
    def solve(i):
        if i in out: return out[i]
        b = by[i]
        a = b["angleDeg"] + offsets.get(i, {}).get("angleDeg", 0)
        if b["parent"] is None:
            out[i] = (b["x"], b["y"], a)
        else:
            px, py, pa = solve(b["parent"])
            c, s = math.cos(rad(pa)), math.sin(rad(pa))
            out[i] = (px + b["x"] * c - b["y"] * s, py + b["x"] * s + b["y"] * c, pa + a)
        return out[i]
    for b in bones: solve(b["id"])
    return out

def tip(f, length):
    return (f[0] + length * math.cos(rad(f[2])), f[1] + length * math.sin(rad(f[2])))

def to_local(f, p):
    c, s = math.cos(rad(f[2])), math.sin(rad(f[2]))
    dx, dy = p[0] - f[0], p[1] - f[1]
    return (dx * c + dy * s, -dx * s + dy * c)

def to_world(f, p):
    c, s = math.cos(rad(f[2])), math.sin(rad(f[2]))
    return (f[0] + p[0] * c - p[1] * s, f[1] + p[0] * s + p[1] * c)

def norm(a):
    while a > 180: a -= 360
    while a <= -180: a += 360
    return a

def two_bone(a, target, l1, l2, sign):
    dx, dy = target[0] - a[0], target[1] - a[1]
    d = math.hypot(dx, dy)
    pulled = 0.0
    reach = (l1 + l2) * 0.9995
    if d > reach:
        pulled = d - reach
        d = reach
    base = math.atan2(dy, dx)
    inner = math.acos(max(-1, min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d))))
    a1 = base + sign * inner
    mid = (a[0] + l1 * math.cos(a1), a[1] + l1 * math.sin(a1))
    end = (a[0] + d * math.cos(base), a[1] + d * math.sin(base))
    return math.degrees(a1), math.degrees(math.atan2(end[1] - mid[1], end[0] - mid[0])), pulled

def side_of(a, b, p):
    return 1 if (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) > 0 else -1

def stifle():
    a1, a2, _ = two_bone((0.0, HIP_Y), HOCK, FEMUR, TIBIA, 1)
    return (FEMUR * math.cos(rad(a1)), HIP_Y + FEMUR * math.sin(rad(a1)))

def standing_world():
    """World origin, angle and length of every bone of the standing dog (y relative to the hip joint)."""
    w = {}
    def seg(i, pair):
        (ox, oy), t = pair
        w[i] = (ox, oy - HIP_Y, math.degrees(math.atan2(t[1] - oy, t[0] - ox)), math.hypot(t[0] - ox, t[1] - oy))
    for i, pair in SPINE.items(): seg(i, pair)
    knee = stifle()
    for side in "nf":
        seg("thigh-h" + side, ((0.0, HIP_Y), knee))
        seg("shank-h" + side, (knee, HOCK))
        for k, pair in HIND_LOW.items(): seg(k + side, pair)
        for k, pair in FORE.items(): seg(k + side, pair)
    (hx, hy), ang, length = STANDING_WORLD["head"]
    w["head"] = (hx, hy - HIP_Y, ang, length)
    prev = None
    for i in ("tail-1", "tail-2", "tail-3"):
        o, ang, length = STANDING_WORLD[i]
        o = (o[0], o[1] - HIP_Y) if o is not None else tip(w[prev], w[prev][3])
        w[i] = (o[0], o[1], ang, length)
        prev = i
    return w

def group(i):
    for k in list(LIMITS) + ["ear-"]:
        if i.startswith(k): return k
    return i

def main():
    doc = json.load(open(sys.argv[1], encoding="utf-8"))
    asset = doc["asset"]
    old = asset["rig"]["bones"]
    old_by = {b["id"]: b for b in old}
    stand_off = next(p for p in asset["poses"] if p["id"] == STAND)["bones"]
    old_rest = fk(old, {})
    old_stand = fk(old, stand_off)
    world = standing_world()
    legs = lambda i: i.split("-")[0] in ("thigh", "shank", "metatarsus", "paw", "scapula", "humerus", "forearm", "metacarpus")
    S = {}
    for b in old:
        i = b["id"]
        if i in REMOVED: continue
        if i in world:
            S[i] = world[i]
        else:
            (lx, ly), rel, length = HEAD_LOCAL["ear" if i.startswith("ear") else i]
            ox, oy = to_world(S["head"], (lx, ly))
            S[i] = (ox, oy, S["head"][2] + rel + stand_off.get(i, {}).get("angleDeg", 0), length)
    new = []
    for b in old:
        i = b["id"]
        if i in REMOVED: continue
        ox, oy, ang, length = S[i]
        nb = {"id": i, "parent": b["parent"], "title": b["title"]}
        own = 0 if legs(i) else stand_off.get(i, {}).get("angleDeg", 0)
        if b["parent"] is None:
            nb.update(x=0, y=0, angleDeg=r4(ang - own))
        else:
            lx, ly = to_local(S[b["parent"]], (ox, oy))
            nb.update(x=r4(lx), y=r4(ly))
            z = LATERAL.get(group(i))
            if z is not None: nb["z"] = z if i.endswith("f") else -z
            nb["angleDeg"] = r4(norm(ang - S[b["parent"]][2] - own))
        nb["lengthM"] = r4(length)
        joint = dict(b["joint"])
        if group(i) in LIMITS: joint["minDeg"], joint["maxDeg"] = LIMITS[group(i)]
        nb["joint"] = joint
        new.append(nb)
    new_by = {b["id"]: b for b in new}
    new_rest = fk(new, {})
    keep = {k: v for k, v in stand_off.items() if not legs(k)}
    new_stand = fk(new, keep)
    for nb in new:
        b = old_by[nb["id"]]
        m = dict(b["mass"])
        k = nb["lengthM"] / b["lengthM"]
        m["comX"], m["comY"] = r4(m["comX"] * k), r4(m["comY"] * k)
        nb["mass"] = m
    new_by["pelvis"]["mass"]["comX"], new_by["pelvis"]["mass"]["comY"] = [r4(v) for v in to_local(new_rest["pelvis"], (0.0, 0.015))]
    new_by["lumbar"]["mass"]["comX"], new_by["lumbar"]["mass"]["comY"] = [r4(v) for v in to_local(new_rest["lumbar"], (0.15, -0.045))]
    new_by["thorax"]["mass"]["comX"], new_by["thorax"]["mass"]["comY"] = [r4(v) for v in to_local(new_rest["thorax"], (0.37, -0.075))]
    new_by["head"]["mass"]["comX"], new_by["head"]["mass"]["comY"] = 0.095, 0.005
    pads = {}
    old_pad = {a["id"]: (a["x"], a["y"]) for a in asset["anchors"]}
    anchors = []
    for a in asset["anchors"]:
        a = dict(a)
        if a["kind"] == "contact" and a["bone"].startswith("paw"):
            leg = a["bone"][4]
            lx, ly = to_local(new_stand[a["bone"]], (PAD[leg][0], PAD[leg][1] - HIP_Y))
            a["x"], a["y"] = r4(lx), r4(ly)
            pads[a["bone"]] = (old_pad[a["id"]], (a["x"], a["y"]))
        elif a["id"] == "back-plume":
            a["x"], a["y"] = [r4(v) for v in to_local(new_stand["thorax"], (BACK_PLUME_WORLD[0], BACK_PLUME_WORLD[1] - HIP_Y))]
        elif a["id"] == "contact-rump":
            a["x"], a["y"] = [r4(v) for v in to_local(new_stand["pelvis"], (RUMP_WORLD[0], RUMP_WORLD[1] - HIP_Y))]
        else:
            a["x"], a["y"] = ANCHORS[a["id"]][1]
        anchors.append(a)
    report = []
    poses = []
    for pose in asset["poses"]:
        off = pose["bones"]
        fo = fk(old, off)
        out = {k: dict(v) for k, v in off.items() if not legs(k) and k not in REMOVED}
        for side in "nf":
            for leg, up, lo, mid, paw, pre in (("h", "thigh-h", "shank-h", "metatarsus-h", "paw-h", []), ("f", "humerus-f", "forearm-f", "metacarpus-f", "paw-f", ["scapula-f"])):
                up, lo, mid, paw = up + side, lo + side, mid + side, paw + side
                for p in pre:
                    d = off.get(p + side, {}).get("angleDeg", 0) - stand_off.get(p + side, {}).get("angleDeg", 0)
                    if d != 0: out[p + side] = {"angleDeg": r4(d)}
                f = fk(new, out)
                start = (f[up][0], f[up][1])
                old_pad, new_pad = pads[paw]
                target = to_world(fo[paw], old_pad)
                s_old, s_new = to_world(old_stand[paw], old_pad), to_world(new_stand[paw], new_pad)
                target = (target[0] + s_new[0] - s_old[0], target[1] + s_new[1] - s_old[1])
                a_paw = fo[paw][2] + new_stand[paw][2] - old_stand[paw][2]
                a_mid = fo[mid][2] + new_stand[mid][2] - old_stand[mid][2]
                c, s = math.cos(rad(a_paw)), math.sin(rad(a_paw))
                joint = (target[0] - (new_pad[0] * c - new_pad[1] * s), target[1] - (new_pad[0] * s + new_pad[1] * c))
                lm = new_by[mid]["lengthM"]
                end = (joint[0] - lm * math.cos(rad(a_mid)), joint[1] - lm * math.sin(rad(a_mid)))
                sign = side_of((new_stand[up][0], new_stand[up][1]), (new_stand[mid][0], new_stand[mid][1]), (new_stand[lo][0], new_stand[lo][1]))
                a1, a2, pulled = two_bone(start, end, new_by[up]["lengthM"], new_by[lo]["lengthM"], sign)
                if pulled > 5e-4 and pre:
                    base = out.get(pre[0] + side, {}).get("angleDeg", 0)
                    for step in range(1, 61):
                        for d in (step * 0.5, -step * 0.5):
                            trial = dict(out)
                            trial[pre[0] + side] = {"angleDeg": r4(base + d)}
                            f2 = fk(new, trial)
                            s2 = (f2[up][0], f2[up][1])
                            b1, b2, p2 = two_bone(s2, end, new_by[up]["lengthM"], new_by[lo]["lengthM"], sign)
                            if p2 <= 5e-4:
                                out, f, start, a1, a2, pulled = trial, f2, s2, b1, b2, p2
                                report.append("adjusted %s %s by %.1f to reach" % (pose["id"], pre[0] + side, d))
                                break
                        if pulled <= 5e-4: break
                if pulled > 5e-4: report.append("REACH %s %s short by %.4f" % (pose["id"], up, pulled))
                parent = f[new_by[up]["parent"]][2]
                for bone, o in ((up, norm(a1 - parent - new_by[up]["angleDeg"])), (lo, norm(a2 - a1 - new_by[lo]["angleDeg"])), (mid, norm(a_mid - a2 - new_by[mid]["angleDeg"])), (paw, norm(a_paw - a_mid - new_by[paw]["angleDeg"]))):
                    j = new_by[bone]["joint"]
                    if abs(o) > 0.005: out[bone] = {"angleDeg": round(o, 2) + 0.0}
                    if not (j["minDeg"] <= o <= j["maxDeg"]): report.append("LIMIT %s %s %.2f not in %s..%s" % (pose["id"], bone, o, j["minDeg"], j["maxDeg"]))
        fn = fk(new, out)
        for side in "nf":
            for paw in ("paw-h" + side, "paw-f" + side):
                old_pad, new_pad = pads[paw]
                t0, t1 = to_world(fo[paw], old_pad), to_world(fn[paw], new_pad)
                s_old, s_new = to_world(old_stand[paw], old_pad), to_world(new_stand[paw], new_pad)
                e = math.hypot(t0[0] + s_new[0] - s_old[0] - t1[0], t0[1] + s_new[1] - s_old[1] - t1[1])
                if e > 5e-4 and ("contact-" + paw[4:]) in pose["contacts"]: report.append("pad %s %s off %.4f" % (pose["id"], paw, e))
        p = dict(pose)
        p["bones"] = out
        poses.append(p)
    refs = [r for r in asset["provenance"]["references"] if r["id"] not in {q["id"] for q in REFERENCES}] + REFERENCES
    materials = [m for m in asset["materials"] if m["id"] != "bone"] + [BONE_MATERIAL]
    roots = {"provenance": {**asset["provenance"], "references": refs}, "dimensions": DIMENSIONS, "materials": materials, "rig": {**asset["rig"], "bones": new}, "anchors": anchors, "poses": poses}
    json.dump(roots, open(sys.argv[2], "w", encoding="utf-8"), ensure_ascii=False)
    for b in new: print(b["id"].ljust(14), str(b["parent"]).ljust(14), b["x"], b["y"], b.get("z", ""), b["angleDeg"], b["lengthM"], b["mass"]["comX"], b["mass"]["comY"])
    for i, f in new_stand.items(): print("stand", i.ljust(14), round(f[0], 4), round(f[1] + HIP_Y, 4), round(f[2], 2), "->", round(tip(f, new_by[i]["lengthM"])[0], 4), round(tip(f, new_by[i]["lengthM"])[1] + HIP_Y, 4))
    k = stifle()
    print("stifle", round(k[0], 4), round(k[1], 4))
    print("\n".join(report) or "no findings")

main()
