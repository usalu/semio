"""🕊️ Pigeon second round: shoulder and pelvic girdle on lateral rest placement, wing poses solved from anatomical targets, remiges as true vanes.

Writes the candidate document to 🗑️generated/r3-pigeon/🔣️.json; with --write also the catalog rig, poses and the volumes source.
"""
import copy, json, math, os, sys
from rework_pigeon_fk import CATALOG, app, euler, frames, mul, rot, tr

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/animals/pigeon.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-pigeon/🔣️.json")
SCALES = ("50", "100", "200", "500")
SIDES = (("f", "far", 1.0), ("n", "near", -1.0))


def unit(v):
    n = math.sqrt(sum(x * x for x in v))
    return [x / n for x in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cols(u, v, w):
    return [[u[i], v[i], w[i]] for i in range(3)]


def frame(u, w):
    u = unit(u)
    w = unit([w[i] - dot(w, u) * u[i] for i in range(3)])
    return cols(u, cross(w, u), w)


def turned(d, n, deg):
    a = math.radians(deg)
    p = cross(n, d)
    return [math.cos(a) * d[i] + math.sin(a) * p[i] for i in range(3)]


def quat(m):
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        return [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    if m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        return [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
    return [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]


def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def r5(v):
    return [round(x, 5) for x in v]


SHOULDER_STERNAL = [0.0634, -0.0071, 0.009]
ILIAC_CREST = [-0.004, 0.014, 0.005]
LOUVRE = float(os.environ.get("LOUVRE", "10"))


def girdle(asset):
    """🦴 Coracoid from the sternum out to the shoulder joint, hip bone from the synsacrum out to the hip joint; limbs re-parented without moving."""
    bones = asset["rig"]["bones"]
    rest = frames(bones)
    by = {b["id"]: b for b in bones}
    out = []
    moves = {}
    for bone in bones:
        for s, _, sign in SIDES:
            for limb, parent, gid, start, titles, limits in (
                ("humerus", "trunk", "coracoid", SHOULDER_STERNAL, ("coracoid (sternum to shoulder joint)", "Rabenbein (Brustbein bis Schultergelenk)"), (-3, 3)),
                ("thigh", "pelvis", "ilium", ILIAC_CREST, ("hip bone (synsacrum to hip joint)", "Hüftbein (Synsacrum bis Hüftgelenk)"), (-1, 1)),
            ):
                if bone["id"] != f"{limb}-{s}" or limb not in os.environ.get("GIRDLE", "humerus,thigh"):
                    continue
                a = [start[0], start[1], start[2] * sign]
                po, pr = rest[parent]
                lo, lr = rest[bone["id"]]
                d = [lo[i] - a[i] for i in range(3)]
                length = math.sqrt(dot(d, d))
                g = frame(d, [-1.0, 0.0, 0.0] if sign > 0 else [1.0, 0.0, 0.0])
                gl = mul(tr(pr), g)
                ang = euler(gl)
                p = app(tr(pr), [a[i] - po[i] for i in range(3)])
                near = s == "n"
                out.append({
                    "id": f"{gid}-{s}", "parent": parent,
                    "title": {"en": ("Near " if near else "Far ") + titles[0], "de": ("Nahes " if near else "Fernes ") + titles[1]},
                    "x": round(p[0], 5), "y": round(p[1], 5), "z": round(p[2], 5),
                    "angleDeg": round(ang[0], 4), "abductionDeg": round(ang[1], 4), "twistDeg": round(ang[2], 4),
                    "lengthM": round(length, 5), "joint": {"type": "revolute", "minDeg": limits[0], "maxDeg": limits[1]},
                })
                ll = euler(mul(tr(g), lr))
                bone["parent"] = f"{gid}-{s}"
                bone["x"], bone["y"] = round(length, 5), 0
                bone.pop("z", None)
                bone["angleDeg"], bone["abductionDeg"], bone["twistDeg"] = round(ll[0], 4), round(ll[1], 4), round(ll[2], 4)
                moves[f"{gid}-{s}"] = (a, g, parent)
        out.append(bone)
    for bone in out:
        m = bone["id"].split("-")
        if len(m) == 2 and m[0][0] in "ps" and m[0][1:].isdigit():
            bone["twistDeg"] = LOUVRE if m[1] == "f" else -LOUVRE
    by = {b["id"]: b for b in out}
    for s, _, sign in SIDES:
        if os.environ.get("LIMITS") == "0":
            break
        by[f"humerus-{s}"]["joint"].update({"minDeg": -180, "maxDeg": 180, "twistMinDeg": -180, "twistMaxDeg": 180})
        by[f"forearm-{s}"]["joint"].update({"minDeg": -165, "maxDeg": 15})
        by[f"hand-{s}"]["joint"].update({"minDeg": -15, "maxDeg": 165})
    asset["rig"]["bones"] = out
    after = frames(out)
    worst = max(abs(after[k][0][i] - rest[k][0][i]) for k in rest for i in range(3))
    worst_r = max(abs(after[k][1][i][j] - rest[k][1][i][j]) for k in rest for i in range(3) for j in range(3) if k.split("-")[0][0] not in "ps" or not k.split("-")[0][1:].isdigit())
    print(f"girdle: largest move of an existing bone origin {worst * 1000:.4f} mm, of a frame axis {worst_r:.6f}")
    return rest, after


WING = {
    "wings-half-open": dict(h=[-0.30, 0.75, 0.60], n=[-0.20, 0.60, -0.75], elbow=85, wrist=105, open=0.6),
    "takeoff-crouch": dict(h=[-0.85, 0.10, 0.50], n=[0.30, 0.50, 0.80], elbow=45, wrist=50, open=0.25),
    "takeoff-wings-up": dict(h=[-0.25, 0.80, 0.55], n=[-0.10, 0.55, -0.83], elbow=105, wrist=135, open=0.9),
    "takeoff-push": dict(h=[-0.20, 0.10, 0.97], n=[0.12, 1.00, 0.00], elbow=125, wrist=155, open=1.0),
    "takeoff-liftoff": dict(h=[0.10, -0.55, 0.83], n=[0.25, 0.80, 0.55], elbow=125, wrist=155, open=1.0),
    "airborne-wings-up": dict(h=[-0.25, 0.80, 0.55], n=[-0.10, 0.55, -0.83], elbow=105, wrist=135, open=0.9),
    "airborne-wings-down": dict(h=[0.20, -0.65, 0.73], n=[0.25, 0.72, 0.65], elbow=125, wrist=155, open=1.0),
    "airborne-climb": dict(h=[-0.30, 0.72, 0.62], n=[-0.10, 0.62, -0.78], elbow=110, wrist=140, open=0.95),
}


def spread_secondary(i):
    return -(85 + 27 * (i - 1) / 9)


def spread_primary(i):
    return -(72 - 68 * (i - 1) / 9)


def wing_poses(asset):
    """🪽 Far wing solved from humerus direction, dorsal normal, elbow and wrist angle and tract spread; the near wing is its mirror."""
    bones = asset["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    rest = frames(bones)
    skip = next((a.split("=")[1].split(",") for a in sys.argv if a.startswith("--skip=")), [])
    push = next(([float(x) for x in a.split("=")[1].split(",")] for a in sys.argv if a.startswith("--push=")), None)
    if push:
        WING["takeoff-push"].update(h=push[:3], n=push[3:])
    for pose in asset["poses"]:
        spec = WING.get(pose["id"])
        if not spec or pose["id"] in skip:
            continue
        off = pose["bones"]
        for k in list(off):
            head = k.split("-")[0]
            if head in ("humerus", "forearm", "hand", "coracoid") or (head[0] in "ps" and head[1:].isdigit()):
                del off[k]
        posed = frames(bones, off)
        body = mul(posed["trunk"][1], tr(rest["trunk"][1]))
        h = unit(app(body, unit(spec["h"])))
        n = app(body, unit(spec["n"]))
        n = unit([n[i] - dot(n, h) * h[i] for i in range(3)])
        fa = turned(h, n, 180 - spec["elbow"])
        hd = turned(fa, n, -(180 - spec["wrist"]))
        target = {"humerus-f": frame(h, n), "forearm-f": frame(fa, n), "hand-f": frame(hd, n)}
        o = spec["open"]
        for i in range(1, 11):
            rs = by[f"s{i}-f"]["angleDeg"]
            target[f"s{i}-f"] = mul(frame(turned(fa, n, rs + o * (spread_secondary(i) - rs)), n), rot(0, 0, LOUVRE))
            target[f"p{i}-f"] = mul(frame(turned(hd, n, o * spread_primary(i)), n), rot(0, 0, LOUVRE))
        world = {k: f[1] for k, f in posed.items()}
        for k in ["humerus-f", "forearm-f", "hand-f"] + [f"s{i}-f" for i in range(1, 11)] + [f"p{i}-f" for i in range(1, 11)]:
            b = by[k]
            restlocal = rot(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0))
            a = euler(mul(tr(mul(world[b["parent"]], restlocal)), target[k]))
            world[k] = target[k]
            far = {"angleDeg": round(a[0], 4), "abductionDeg": round(a[1], 4), "twistDeg": round(a[2], 4)}
            off[k] = {q: v for q, v in far.items() if abs(v) > 1e-4}
            mirror = {"angleDeg": far["angleDeg"], "abductionDeg": -far["abductionDeg"], "twistDeg": -far["twistDeg"]}
            off[k[:-1] + "n"] = {q: v for q, v in mirror.items() if abs(v) > 1e-4}
        check = frames(bones, off)
        tip = [check["p9-f"][0][i] + 0.15 * check["p9-f"][1][i][0] for i in range(3)]
        tipn = [check["p9-n"][0][i] + 0.15 * check["p9-n"][1][i][0] for i in range(3)]
        print(f"{pose['id']}: humerus {off['humerus-f']} forearm {off['forearm-f']} hand {off['hand-f']} p9 tip far {r5(tip)} near {r5(tipn)} span {abs(tip[2] - tipn[2]):.3f}")


VANE_AT = [0.0, 0.10, 0.28, 0.55, 0.80, 0.93, 1.0]
VANE_PRIMARY = [0.12, 0.45, 0.88, 1.0, 0.90, 0.60, 0.28]
VANE_SECONDARY = [0.12, 0.50, 0.90, 1.0, 1.0, 0.80, 0.45]
VANE_ROTATION = [0.70710678, 0, -0.70710678, 0]
TRACTS = {
    "50": dict(primaries=range(1, 11), secondaries=range(1, 11), half=(0.0115, 0.0095, 0.0135), thick=0.00045),
    "100": dict(primaries=range(1, 11), secondaries=range(1, 11), half=(0.0115, 0.0095, 0.0135), thick=0.00045),
    "200": dict(primaries=(2, 4, 6, 8, 10), secondaries=(1, 3, 5, 7, 9), half=(0.0158, 0.0158, 0.0158), thick=0.00075),
    "500": dict(primaries=(3, 8), secondaries=(3, 8), half=(0.038, 0.038, 0.038), thick=0.0018),
}


def vane(old, length, half, profile, thick):
    """🪶 One remex: a lens-thin vane from the quill to the rounded tip, declared as a sheet."""
    solid = {k: v for k, v in old.items() if k != "sections"}
    solid["sections"] = [{"at": [round(f * length, 5), 0, 0], "rx": thick, "ry": round(max(half * p, 0.0015), 5), "exponent": 2, "rotation": VANE_ROTATION} for f, p in zip(VANE_AT, profile)]
    solid["sheet"] = True
    solid["material"] = "remex"
    return solid


def thicken(solid, target):
    """📏 Bring the thin direction of a loft to the smallest feature of its scale."""
    field = "rx" if max(s["rx"] for s in solid["sections"]) < max(s["ry"] for s in solid["sections"]) else "ry"
    factor = target / (2 * max(s[field] for s in solid["sections"]))
    for s in solid["sections"]:
        s[field] = round(s[field] * factor, 5)


def volumes(asset, source, moves_rest, moves_after):
    lengths = {b["id"]: b["lengthM"] for b in asset["rig"]["bones"]}
    for scale in SCALES:
        v = source["volumes"][scale]
        spec = TRACTS[scale]
        body = []
        gone = set()
        for solid in v["body"]:
            parts = solid["id"].split("-")
            if len(parts) == 3 and parts[1] in ("primary", "secondary"):
                i = int(parts[2])
                keep = spec["primaries"] if parts[1] == "primary" else spec["secondaries"]
                if i not in keep:
                    gone.add(solid["id"])
                    continue
                half = spec["half"][2] if parts[1] == "secondary" else spec["half"][1 if i >= 8 else 0]
                body.append(vane(solid, lengths[solid["bone"]], half, VANE_SECONDARY if parts[1] == "secondary" else VANE_PRIMARY, spec["thick"]))
                continue
            name = solid["id"].split("-", 1)[-1]
            if scale in ("100", "200") and name == "shoulder":
                thicken(solid, 0.0141)
            if scale in ("100", "200") and name == "hand":
                thicken(solid, 0.0153 if scale == "100" else 0.0161)
            if scale == "100" and name == "scapulars":
                thicken(solid, 0.0153)
            if scale == "200" and name == "scapulars":
                gone.add(solid["id"])
                continue
            body.append(solid)
        v["body"] = body
        v["thermal"] = [t for t in v["thermal"] if t.get("of") not in gone]
        structure = []
        for solid in v["structure"]:
            if solid["id"].endswith("-rachis"):
                solid = {k: x for k, x in solid.items()}
                solid["p1"] = [round(lengths[solid["bone"]] - 0.002, 5), 0, 0]
                solid["r0"], solid["r1"] = 0.0008, 0.0005
                solid["slender"] = True
            if solid["id"] in ("near-coracoid", "far-coracoid") and "coracoid-f" in moves_after:
                bone = "coracoid-" + solid["id"][0]
                o_old, r_old = moves_rest["trunk"]
                o_new, r_new = moves_after[bone]
                m = mul(tr(r_new), r_old)
                shift = app(tr(r_new), [o_old[i] - o_new[i] for i in range(3)])
                q = quat(m)
                for s in solid["sections"]:
                    p = app(m, s["at"])
                    s["at"] = r5([p[i] + shift[i] for i in range(3)])
                    if "rotation" in s:
                        s["rotation"] = r5(qmul(q, s["rotation"]))
                solid["bone"] = solid["member"] = bone
            if solid["id"] == "synsacrum" and scale != "50":
                solid["members"] = ["ilium-n", "ilium-f"]
            if solid["id"] in ("near-ilium", "far-ilium"):
                continue
            structure.append(solid)
        if scale == "50":
            for s, name, _ in SIDES:
                length = lengths[f"ilium-{s}"]
                structure.append({
                    "id": f"{name}-ilium", "kind": "loft", "bone": f"ilium-{s}", "material": "bone", "role": "body", "tone": 0.88,
                    "sections": [
                        {"at": [-0.002, 0, 0], "rx": 0.0038, "ry": 0.0052, "exponent": 2, "rotation": VANE_ROTATION},
                        {"at": [round(0.45 * length, 5), 0, 0], "rx": 0.0038, "ry": 0.0046, "exponent": 2, "rotation": VANE_ROTATION},
                        {"at": [round(length, 5), 0, 0], "rx": 0.0044, "ry": 0.0044, "exponent": 2, "rotation": VANE_ROTATION},
                    ],
                    "capStart": True, "capEnd": True, "member": f"ilium-{s}", "part": "member",
                })
        v["structure"] = structure
        print(scale, v.get("status"), "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]))


def main():
    text = open(CATALOG, encoding="utf-8").read()
    doc = json.loads(text)
    stable = json.dumps(doc, ensure_ascii=False, indent=2) + "\n" == text
    print("catalog file round-trips byte-identical with indent 2:", stable)
    asset = doc.get("asset", doc)
    source = json.load(open(SOURCE, encoding="utf-8"))
    if any(b["id"] == "coracoid-f" for b in asset["rig"]["bones"]):
        sys.exit("girdle already present: this script runs once on the round-one rig")
    rest, after = girdle(asset)
    plumage = next(m for m in asset["materials"] if m["id"] == "plumage")
    asset["materials"].append({**plumage, "id": "remex", "title": {"en": "Flight feather vane (remex)", "de": "Fahne der Schwungfeder"}})
    wing_poses(asset)
    if "--no-volumes" not in sys.argv:
        volumes(asset, source, rest, after)
    candidate = copy.deepcopy(doc)
    candidate.get("asset", candidate)["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(candidate, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    if "--write" in sys.argv:
        from rework_08_root import splice

        def change(target):
            for key in ("rig", "poses", "materials"):
                target[key] = asset[key]
            return {"rig", "poses", "materials"}

        splice("pigeon", change)
        before = open(SOURCE, encoding="utf-8").read()
        compact = json.dumps(json.loads(before), ensure_ascii=False) == before.rstrip("\n")
        open(SOURCE, "w", encoding="utf-8").write(json.dumps(source, ensure_ascii=False, indent=None if compact else 2) + ("\n" if before.endswith("\n") else ""))
        print("written: catalog rig, poses, materials and the volumes source; source compact:", compact)

main()
