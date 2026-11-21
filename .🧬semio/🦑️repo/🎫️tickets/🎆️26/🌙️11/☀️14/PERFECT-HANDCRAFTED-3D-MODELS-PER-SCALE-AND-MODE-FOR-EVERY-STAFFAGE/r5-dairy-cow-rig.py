"""🐄 Round 5 dairy cow: root fix of the hind legs of the rig against the reference table of r3-animals-dairy-cow.md ("Round 5 handcraft").

The hind leg had a tibia of 0.48 m and a femur of 0.40 m (real Holstein about 0.39 and 0.43 joint to joint), which put the hock joint 5 cm and the point of the hock 8 cm too low. The stifle and the hock now sit at their real centres; fetlock, claws, trunk, neck and forelimbs keep their places, so every claw plant of every pose and clip is kept.

Every child bone, mass, anchor and elevation 2D shape keeps its rest place in the world. Every pose and clip key is solved again: hind legs by inverse kinematics onto the old fetlock with the cannon keeping its deviation from rest, so the claws stay where they were.

Usage: python3 r5-dairy-cow-rig.py [--candidate | --apply]
"""
import copy
import json
import math
import sys

FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json"
HERE = __file__.rsplit("/", 1)[0]
CANDIDATE = HERE + "/🗑️generated/r5-dairy-cow/🔣️.json"
HIP = 1.106

TARGET = {
    "stifle": (0.22, 0.735),
    "hock": (-0.05, 0.46),
}
HIND = ("thigh", "shank", "metatarsus")
TRUNK = ("pelvis", "lumbar", "thorax")
POSE_FIX = {}


def raw(p):
    return (p[0], p[1] - HIP)


def fk(bones, offsets=None, root=None):
    """🧭 World frames (x, y, angle) of every bone for angle offsets and a root."""
    by = {b["id"]: b for b in bones}
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = by[i]
        d = (offsets or {}).get(i, {})
        if b["parent"] is None:
            r = root or {}
            out[i] = (b["x"] + r.get("x", 0) + d.get("dx", 0), b["y"] + r.get("y", 0) + d.get("dy", 0), b["angleDeg"] + r.get("angleDeg", 0) + d.get("angleDeg", 0))
        else:
            px, py, pa = solve(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            x, y = b["x"] + d.get("dx", 0), b["y"] + d.get("dy", 0)
            out[i] = (px + x * c - y * s, py + x * s + y * c, pa + b["angleDeg"] + d.get("angleDeg", 0))
        return out[i]

    for b in bones:
        solve(b["id"])
    return out


def to_world(f, x, y):
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    return (f[0] + x * c - y * s, f[1] + x * s + y * c)


def to_local(f, wx, wy):
    c, s = math.cos(math.radians(f[2])), math.sin(math.radians(f[2]))
    dx, dy = wx - f[0], wy - f[1]
    return (dx * c + dy * s, -dx * s + dy * c)


def ang(a, b):
    return math.degrees(math.atan2(b[1] - a[1], b[0] - a[0]))


def dist(a, b):
    return math.hypot(b[0] - a[0], b[1] - a[1])


def r5(v):
    v = round(v, 5)
    return 0.0 if v == 0 else v


def wrap(a):
    return (a + 180.0) % 360.0 - 180.0


def two_link(p, t, l1, l2, sign):
    """🦿 Angles of a two-link chain from p reaching t; `sign` picks the bend (+1: the middle joint left of the line p-t)."""
    d = min(max(dist(p, t), abs(l1 - l2) + 1e-9), l1 + l2 - 1e-9)
    base = ang(p, t)
    a = math.degrees(math.acos((l1 * l1 + d * d - l2 * l2) / (2 * l1 * d)))
    a1 = base + sign * a
    m = (p[0] + l1 * math.cos(math.radians(a1)), p[1] + l1 * math.sin(math.radians(a1)))
    return a1, ang(m, t)


def world_targets(old):
    """🎯 The new rest frames (x, y, angle) and lengths of the hind leg bones, in the raw frame of the rig (hip joint at the origin)."""
    T = {k: raw(v) for k, v in TARGET.items()}
    w, L = {}, {}
    hip = (0.0, 0.0)
    for s in ("n", "f"):
        fet = old[f"hoof-h{s}"][:2]
        chain = [hip, T["stifle"], T["hock"], fet]
        for k, a, b in zip(HIND, chain, chain[1:]):
            w[f"{k}-h{s}"] = a + (ang(a, b),)
            L[f"{k}-h{s}"] = dist(a, b)
    return w, L


def new_rig(asset):
    """🦴 Bones with the new rest frames; every other bone keeps its world rest frame; masses and limits follow."""
    bones = copy.deepcopy(asset["rig"]["bones"])
    old = fk(bones)
    w, L = world_targets(old)
    target = {i: w.get(i, old[i]) for i in old}
    for b in bones:
        i = b["id"]
        if b["parent"] is None:
            b["x"], b["y"], b["angleDeg"] = r5(target[i][0]), r5(target[i][1]), r5(target[i][2])
        else:
            p = target[b["parent"]]
            x, y = to_local(p, target[i][0], target[i][1])
            b["x"], b["y"], b["angleDeg"] = r5(x), r5(y), r5(wrap(target[i][2] - p[2]))
        if i in L:
            k = L[i] / b["lengthM"]
            if i in TRUNK:
                wx, wy = to_world(old[i], b["mass"]["comX"], b["mass"]["comY"])
                b["mass"]["comX"], b["mass"]["comY"] = map(r5, to_local(target[i], wx, wy))
            else:
                b["mass"]["comX"] = r5(b["mass"]["comX"] * k)
            b["lengthM"] = r5(L[i])
    return bones, old


def solve_offsets(asset, bones_old, bones_new, offsets, root):
    """🧩 Offsets for the new rig that put the poll, the head and every fetlock, pastern and hoof where the old rig had them."""
    o_old = fk(bones_old, offsets, root)
    r_old, r_new = fk(bones_old), fk(bones_new)
    turn = {i: r_new[i][2] - r_old[i][2] for i in r_old}
    new = {k: dict(v) for k, v in offsets.items()}
    by = {b["id"]: b for b in bones_new}

    def put(i, world_angle, frames):
        parent = frames[by[i]["parent"]][2]
        new.setdefault(i, {})["angleDeg"] = r5(wrap(world_angle - parent - by[i]["angleDeg"]))

    for s in ("n", "f"):
        f = fk(bones_new, new, root)
        fet = o_old[f"hoof-h{s}"][:2]
        ac = o_old[f"metatarsus-h{s}"][2] + turn[f"metatarsus-h{s}"]
        hock = (fet[0] - by[f"metatarsus-h{s}"]["lengthM"] * math.cos(math.radians(ac)), fet[1] - by[f"metatarsus-h{s}"]["lengthM"] * math.sin(math.radians(ac)))
        hip = f[f"thigh-h{s}"][:2]
        b1, b2 = two_link(hip, hock, by[f"thigh-h{s}"]["lengthM"], by[f"shank-h{s}"]["lengthM"], 1)
        put(f"thigh-h{s}", b1, f)
        f = fk(bones_new, new, root)
        put(f"shank-h{s}", b2, f)
        f = fk(bones_new, new, root)
        put(f"metatarsus-h{s}", ac, f)
        f = fk(bones_new, new, root)
        put(f"hoof-h{s}", o_old[f"hoof-h{s}"][2], f)
    f = fk(bones_new, new, root)
    err = max(dist(f[i][:2], o_old[i][:2]) for i in o_old if i.startswith(("hoof", "tail", "udder", "head")))
    return new, err


CHAINS = {
    "hn": ["thigh-hn", "shank-hn", "metatarsus-hn", "hoof-hn"],
    "hf": ["thigh-hf", "shank-hf", "metatarsus-hf", "hoof-hf"],
}


def clean(bones_dict):
    out = {}
    for k, v in bones_dict.items():
        v = {f: x for f, x in v.items() if not (f == "angleDeg" and x == 0 and len(v) == 1)}
        if v:
            out[k] = v
    return out


def fix_motion(asset, bones_old, bones_new, log):
    poses = copy.deepcopy(asset["poses"])
    clips = copy.deepcopy(asset["clips"])
    worst = 0.0
    solved = {}
    for p in poses:
        new, err = solve_offsets(asset, bones_old, bones_new, p["bones"], p.get("root"))
        for i, v in POSE_FIX.get(p["id"], {}).items():
            new.setdefault(i, {})["angleDeg"] = v
        worst = max(worst, err)
        changed = {i for c in CHAINS.values() for i in c} | set(POSE_FIX.get(p["id"], {}))
        p["bones"] = clean({**{k: v for k, v in p["bones"].items()}, **{i: {**p["bones"].get(i, {}), "angleDeg": new[i]["angleDeg"]} for i in changed}})
        solved[p["id"]] = p
        log.append("pose %s: residual %.4f m" % (p["id"], err))
    for c in clips:
        for k in c["keys"]:
            base = solved[k["pose"]]
            orig = next(x for x in asset["poses"] if x["id"] == k["pose"])
            over = k.get("bones") or {}
            merged = {i: {**orig["bones"].get(i, {}), **over.get(i, {})} for i in set(orig["bones"]) | set(over)}
            new, err = solve_offsets(asset, bones_old, bones_new, merged, orig.get("root"))
            worst = max(worst, err)
            touched = set(over) & set(TRUNK)
            out = dict(over)
            for name, chain in CHAINS.items():
                if touched or set(chain) & set(over):
                    for i in chain:
                        out[i] = {**over.get(i, {}), "angleDeg": new[i]["angleDeg"]}
            if out:
                k["bones"] = out
        log.append("clip %s: %d keys" % (c["id"], len(c["keys"])))
    log.append("worst residual of fetlocks, hooves, head, ears, tail: %.5f m" % worst)
    return poses, clips


def limits_ok(bones, poses, clips, log, old_poses):
    by = {b["id"]: b for b in bones}
    for i in {i for c in CHAINS.values() for i in c}:
        d = [p["bones"].get(i, {}).get("angleDeg", 0) - q["bones"].get(i, {}).get("angleDeg", 0) for p, q in zip(poses, old_poses)]
        shift = round(sum(d) / len(d))
        j = by[i]["joint"]
        if shift:
            log.append("limit %s shifted by %d: %s..%s -> %s..%s" % (i, shift, j["minDeg"], j["maxDeg"], j["minDeg"] + shift, j["maxDeg"] + shift))
            j["minDeg"], j["maxDeg"] = j["minDeg"] + shift, j["maxDeg"] + shift
    bad = {}
    sets = [(p["id"], p["bones"]) for p in poses]
    for c in clips:
        for k in c["keys"]:
            pose = next(p for p in poses if p["id"] == k["pose"])
            sets.append((c["id"], {i: {**pose["bones"].get(i, {}), **(k.get("bones") or {}).get(i, {})} for i in set(pose["bones"]) | set(k.get("bones") or {})}))
    for name, offs in sets:
        for i, v in offs.items():
            j = by[i].get("joint", {})
            a = v.get("angleDeg", 0)
            if "minDeg" in j and not (j["minDeg"] - 1e-6 <= a <= j["maxDeg"] + 1e-6):
                lo, hi = bad.get(i, (j["minDeg"], j["maxDeg"]))
                bad[i] = (min(lo, math.floor(a - 2)), max(hi, math.ceil(a + 2)))
    for i, (lo, hi) in bad.items():
        log.append("limit %s widened %s..%s -> %s..%s" % (i, by[i]["joint"]["minDeg"], by[i]["joint"]["maxDeg"], lo, hi))
        by[i]["joint"]["minDeg"], by[i]["joint"]["maxDeg"] = lo, hi


def change(asset, log):
    bones_old = asset["rig"]["bones"]
    bones, old = new_rig(asset)
    newf = fk(bones)
    for b0, b1 in zip(bones_old, bones):
        if any(b0[k] != b1[k] for k in ("x", "y", "angleDeg", "lengthM")):
            log.append("bone %s: %s" % (b0["id"], {k: (b0[k], b1[k]) for k in ("x", "y", "angleDeg", "lengthM") if b0[k] != b1[k]}))
    anchors = copy.deepcopy(asset["anchors"])
    for a in anchors:
        wx, wy = to_world(old[a["bone"]], a["x"], a["y"])
        x, y = to_local(newf[a["bone"]], wx, wy)
        if (r5(x), r5(y)) != (a["x"], a["y"]):
            log.append("anchor %s: (%s, %s) -> (%s, %s)" % (a["id"], a["x"], a["y"], r5(x), r5(y)))
            a["x"], a["y"] = r5(x), r5(y)
    reps = copy.deepcopy(asset["representations"])
    moved = 0
    for n, r in reps.items():
        for sh in r.get("geometry", {}).get("elevation", []):
            b = sh["bone"]
            if old[b] == newf[b]:
                continue
            sh["points"] = [[r5(c) for c in to_local(newf[b], *to_world(old[b], x, y))] for x, y in sh["points"]]
            moved += 1
    log.append("elevation shapes carried into the new bone frames: %d" % moved)
    poses, clips = fix_motion(asset, bones_old, bones, log)
    limits_ok(bones, poses, clips, log, asset["poses"])
    return {"rig": {**asset["rig"], "bones": bones}, "anchors": anchors, "representations": reps, "poses": poses, "clips": clips}


def block(key, value):
    text = json.dumps({key: value}, indent=2, ensure_ascii=False)[2:-2]
    return "\n".join("  " + line for line in text.split("\n"))


def main():
    text = open(FILE, encoding="utf-8").read()
    doc = json.loads(text)
    asset = doc["asset"]
    log = []
    members = change(asset, log)
    for line in log:
        print(line)
    if "--candidate" in sys.argv:
        cand = json.loads(text)
        cand["asset"].update(members)
        vol = json.load(open(HERE + "/volumes/animals/dairy-cow.json", encoding="utf-8"))["volumes"]
        cand["asset"]["volumes"] = vol
        json.dump(cand, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        print("candidate", CANDIDATE)
    if "--apply" in sys.argv:
        for key, new in members.items():
            old = block(key, asset[key])
            if text.count(old) != 1:
                raise SystemExit("member %s not found exactly once" % key)
            text = text.replace(old, block(key, new))
        json.loads(text)
        open(FILE, "w", encoding="utf-8").write(text)
        print("written", FILE)


if __name__ == "__main__":
    main()
