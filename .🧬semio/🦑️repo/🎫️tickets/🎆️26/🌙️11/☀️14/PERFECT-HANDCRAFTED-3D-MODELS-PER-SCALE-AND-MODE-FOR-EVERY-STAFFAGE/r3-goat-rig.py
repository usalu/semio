"""🦴️ Round 3 goat: the anatomical rig. Every joint of the rest pose is stated by hand in world metres (x ahead, y up, hip joint at x = 0, ground y = 0) from the reference table of r3-animals-goat.md, converted to the parent-relative bone table, and every pose is carried over: trunk, neck, head, ear and tail offsets stay, each leg keeps its toe tip and sole inclination of the old pose and its hock or carpus is solved anew for the corrected bone lengths.
Usage: python3 r3-goat-rig.py <asset 🔣️.json> <out roots.json>"""
import json, math, sys

HIP_Y = 0.640
JOINTS = {
    "pelvis": ((0.0, 0.640), (0.108, 0.712)),
    "lumbar": ((0.108, 0.712), (0.375, 0.712)),
    "thorax": ((0.375, 0.712), (0.700, 0.655)),
    "neck": ((0.700, 0.655), (0.985, 0.895)),
    "head": ((0.985, 0.895), None),
    "tail-1": ((-0.048, 0.722), None),
    "thigh-h": ((0.0, 0.640), (0.118, 0.462)),
    "shank-h": ((0.118, 0.462), (0.014, 0.250)),
    "metatarsus-h": ((0.014, 0.250), (0.029, 0.079)),
    "hoof-h": ((0.029, 0.079), (0.090, 0.0)),
    "scapula-f": ((0.636, 0.745), (0.728, 0.596)),
    "humerus-f": ((0.728, 0.596), (0.655, 0.438)),
    "forearm-f": ((0.655, 0.438), (0.664, 0.226)),
    "metacarpus-f": ((0.664, 0.226), (0.659, 0.079)),
    "hoof-f": ((0.659, 0.079), (0.720, 0.0)),
}
POLAR = {"head": (-42.0, 0.23), "tail-1": (70.0, 0.07), "tail-2": (60.0, 0.06), "jaw": (-55.2, 0.1746), "ear": (38.0, 0.12)}
HEAD_LOCAL = {"jaw": (0.035, -0.005), "ear": (0.022, 0.030)}
LATERAL = {"thigh-hn": -0.07, "thigh-hf": 0.07, "scapula-fn": -0.04, "scapula-ff": 0.04, "humerus-fn": -0.035, "humerus-ff": 0.035, "ear-n": -0.048, "ear-f": 0.048}
ANCHORS_HEAD = {"nose": (0.236, 0.0), "mouth": (0.205, -0.024)}
KEEP_COM = ["pelvis", "lumbar", "thorax", "neck", "head", "tail-1", "tail-2"]

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

def rest_world():
    w = {}
    def seg(i, key):
        (ox, oy), t = JOINTS[key]
        w[i] = (ox, oy - HIP_Y, math.degrees(math.atan2(t[1] - oy, t[0] - ox)), math.hypot(t[0] - ox, t[1] - oy))
    for i in ["pelvis", "lumbar", "thorax", "neck"]: seg(i, i)
    for side in "nf":
        for k in ["thigh-h", "shank-h", "metatarsus-h", "hoof-h", "scapula-f", "humerus-f", "forearm-f", "metacarpus-f", "hoof-f"]: seg(k + side, k)
    (hx, hy), _ = JOINTS["head"]
    w["head"] = (hx, hy - HIP_Y, POLAR["head"][0], POLAR["head"][1])
    (tx, ty), _ = JOINTS["tail-1"]
    w["tail-1"] = (tx, ty - HIP_Y, POLAR["tail-1"][0], POLAR["tail-1"][1])
    t1 = tip(w["tail-1"], w["tail-1"][3])
    w["tail-2"] = (t1[0], t1[1], POLAR["tail-2"][0], POLAR["tail-2"][1])
    j = to_world(w["head"], HEAD_LOCAL["jaw"])
    w["jaw"] = (j[0], j[1], POLAR["jaw"][0], POLAR["jaw"][1])
    e = to_world(w["head"], HEAD_LOCAL["ear"])
    for side in "nf": w["ear-" + side] = (e[0], e[1], POLAR["ear"][0], POLAR["ear"][1])
    return w

def two_bone(a, target, l1, l2, sign):
    dx, dy = target[0] - a[0], target[1] - a[1]
    d = math.hypot(dx, dy)
    if d > l1 + l2 - 1e-6 or d < abs(l1 - l2) + 1e-6: return None
    base = math.atan2(dy, dx)
    inner = math.acos(max(-1, min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d))))
    a1 = base + sign * inner
    mid = (a[0] + l1 * math.cos(a1), a[1] + l1 * math.sin(a1))
    return math.degrees(a1), math.degrees(math.atan2(target[1] - mid[1], target[0] - mid[0])), mid

def side_of(a, b, p):
    return 1 if (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) > 0 else -1

def norm(a):
    while a > 180: a -= 360
    while a <= -180: a += 360
    return a

def main():
    doc = json.load(open(sys.argv[1], encoding="utf-8"))
    asset = doc["asset"]
    old = asset["rig"]["bones"]
    old_by = {b["id"]: b for b in old}
    old_rest = fk(old, {})
    world = rest_world()
    new = []
    new_rest = {}
    for b in old:
        i = b["id"]
        ox, oy, ang, length = world[i]
        nb = {"id": i, "parent": b["parent"], "title": b["title"]}
        if b["parent"] is None:
            nb.update(x=0, y=0, angleDeg=r4(ang))
        else:
            lx, ly = to_local(new_rest[b["parent"]], (ox, oy))
            nb.update(x=r4(lx), y=r4(ly))
            if i in LATERAL: nb["z"] = LATERAL[i]
            nb["angleDeg"] = r4(norm(ang - new_rest[b["parent"]][2]))
        nb["lengthM"] = r4(length)
        nb["joint"] = b["joint"]
        new.append(nb)
        new_rest = fk(new, {})
    for b, nb in zip(old, new):
        i = b["id"]
        m = dict(b["mass"])
        if i in KEEP_COM:
            lx, ly = to_local(new_rest[i], to_world(old_rest[i], (m["comX"], m["comY"])))
            m["comX"], m["comY"] = r4(lx), r4(ly)
        elif i == "jaw" or i.startswith("ear"):
            m["comX"], m["comY"] = r4(nb["lengthM"] / 2), 0
        else:
            m["comX"], m["comY"] = r4(nb["lengthM"] * m["comX"] / b["lengthM"]), 0
        nb["mass"] = m
        if "section" in b: nb["section"] = b["section"]
    new_by = {b["id"]: b for b in new}
    new_rest = fk(new, {})
    anchors = []
    for a in asset["anchors"]:
        a = dict(a)
        if a["kind"] == "contact":
            a["x"], a["y"] = new_by[a["bone"]]["lengthM"], 0
        elif a["id"] in ANCHORS_HEAD:
            a["x"], a["y"] = ANCHORS_HEAD[a["id"]]
        else:
            lx, ly = to_local(new_rest[a["bone"]], to_world(old_rest[a["bone"]], (a["x"], a["y"])))
            a["x"], a["y"] = r4(lx), r4(ly)
        anchors.append(a)
    report = []
    poses = []
    for pose in asset["poses"]:
        off = pose["bones"]
        if not any(k.startswith(("thigh", "shank", "meta", "hoof", "scapula", "humerus", "forearm")) for k in off):
            poses.append(pose)
            continue
        fo = fk(old, off)
        out = {k: dict(v) for k, v in off.items()}
        for side in "nf":
            for upper, chain in ((["thigh-h"], ["shank-h", "metatarsus-h", "hoof-h"]), (["scapula-f", "humerus-f"], ["forearm-f", "metacarpus-f", "hoof-f"])):
                a, b, h = (c + side for c in chain)
                toe = tip(fo[h], old_by[h]["lengthM"])
                sole = fo[h][2] + (new_rest[h][2] - old_rest[h][2])
                lh = new_by[h]["lengthM"]
                fet = (toe[0] - lh * math.cos(rad(sole)), toe[1] - lh * math.sin(rad(sole)))
                sign = side_of((new_rest[a][0], new_rest[a][1]), (new_rest[h][0], new_rest[h][1]), (new_rest[b][0], new_rest[b][1]))
                last = upper[-1] + side
                best = None
                for step in range(0, 1201):
                    for d in ((0,) if step == 0 else (step * 0.05, -step * 0.05)):
                        trial = {k: dict(v) for k, v in out.items()}
                        trial.setdefault(last, {})["angleDeg"] = out.get(last, {}).get("angleDeg", 0) + d
                        f = fk(new, {k: v for k, v in trial.items() if k.split("-")[0] in ("pelvis", "lumbar", "thorax") or k in [u + side for u in upper]})
                        start = (f[a][0], f[a][1])
                        sol = two_bone(start, fet, new_by[a]["lengthM"], new_by[b]["lengthM"], sign)
                        if sol is None: continue
                        a1, a2, mid = sol
                        inner = abs(norm(a2 - a1))
                        if inner > 150: continue
                        best = (d, f, a1, a2)
                        break
                    if best: break
                if best is None: raise SystemExit("no solution %s %s %s" % (pose["id"], side, chain))
                d, f, a1, a2 = best
                if d != 0: out.setdefault(last, {})["angleDeg"] = r4(out.get(last, {}).get("angleDeg", 0) + d)
                parent = f[new_by[a]["parent"]][2]
                o1 = norm(a1 - parent - new_by[a]["angleDeg"])
                o2 = norm(a2 - a1 - new_by[b]["angleDeg"])
                o3 = norm(sole - a2 - new_by[h]["angleDeg"])
                for bone, o in ((a, o1), (b, o2), (h, o3)):
                    j = new_by[bone]["joint"]
                    if bone == h and ("contact-" + h[5:]) not in pose["contacts"]: o = max(j["minDeg"], min(j["maxDeg"], o))
                    out.setdefault(bone, {})["angleDeg"] = r4(o)
                    if not (j["minDeg"] <= o <= j["maxDeg"]): report.append("LIMIT %s %s %.2f not in %s..%s" % (pose["id"], bone, o, j["minDeg"], j["maxDeg"]))
                if d != 0: report.append("adjusted %s %s by %.2f" % (pose["id"], last, d))
        fn = fk(new, out)
        for side in "nf":
            for h in ("hoof-h" + side, "hoof-f" + side):
                t0, t1 = tip(fo[h], old_by[h]["lengthM"]), tip(fn[h], new_by[h]["lengthM"])
                e = math.hypot(t0[0] - t1[0], t0[1] - t1[1])
                if e > 5e-4: report.append("toe %s %s off %.4f" % (pose["id"], h, e))
        p = dict(pose)
        p["bones"] = {k: out[k] for k in list(off.keys()) + [k for k in out if k not in off]}
        poses.append(p)
    json.dump({"rig": {**asset["rig"], "bones": new}, "anchors": anchors, "poses": poses}, open(sys.argv[2], "w", encoding="utf-8"), ensure_ascii=False)
    for b in new: print(b["id"], b["parent"], b["x"], b["y"], b.get("z", ""), b["angleDeg"], b["lengthM"], b["mass"]["comX"], b["mass"]["comY"])
    for i, f in new_rest.items(): print("rest", i, round(f[0], 4), round(f[1] + HIP_Y, 4), round(f[2], 2), [round(v, 4) for v in tip(f, new_by[i]["lengthM"])][0], round(tip(f, new_by[i]["lengthM"])[1] + HIP_Y, 4))
    print("\n".join(report) or "no findings")

main()
