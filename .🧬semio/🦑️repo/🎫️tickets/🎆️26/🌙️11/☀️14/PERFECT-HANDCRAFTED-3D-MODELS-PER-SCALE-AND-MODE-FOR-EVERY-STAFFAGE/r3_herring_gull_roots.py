"""🕊️ Round 3 herring gull: the corrected roots outside `volumes` and the 2D drawings, derived from the untouched snapshot `r3-herring-gull-orig.json`.

Rig: the trunk hinges at the hips (acetabula) and runs to the neck base (old: hinged at the tail base, 0.34 m, hips at the belly); hips, knees and shoulders get lateral positions; the wing is the real three-piece wing (humerus 135 mm, forearm 140 mm, hand 415 mm from the wrist to the tip of the longest primary), folded at rest in the real Z (elbow up and back near the mantle, wrist at the side of the breast, primaries over the tail), spread by abduction of the humerus and extension of elbow and wrist; leg bones get the gull's lengths (femur 58, tibiotarsus 105, tarsometatarsus 66 mm, middle toe 58 mm). The rest pose is the standing gull. Poses: neck and head keep their world directions, legs are solved anew for the new lengths (the old ankle position relative to the hip kept, scaled to the new reach, knee ahead), wings are stated anew per pose as world spans solved for humerus, forearm and hand, root x shifted by the moved origin, and the grounded poses are lowered onto their feet.

Usage as module: members(report) ; as script: prints the rig and the checks.
"""
import copy
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
ORIG = json.load(open(T + "/r3-herring-gull-orig.json", encoding="utf-8"))

TRUNK_STAND = 14.0
SOLE = 0.0055
REST = {
    "trunk": dict(x=0, y=0, angleDeg=0, lengthM=0.1, com=(0.035, -0.03)),
    "tail": dict(x=-0.075, y=0.016, angleDeg=172, lengthM=0.17, com=(0.07, 0)),
    "neck-1": dict(x=0.1, y=0.012, angleDeg=16, lengthM=0.07, com=(0.035, 0)),
    "neck-2": dict(x=0.07, y=0, angleDeg=55, lengthM=0.065, com=(0.0325, 0)),
    "head": dict(x=0.065, y=0, angleDeg=-97, lengthM=0.125, com=(0.035, 0.004)),
    "humerus": dict(x=0.095, y=0.016, z=0.036, angleDeg=177, lengthM=0.135, com=(0.06, 0)),
    "forearm": dict(x=0.135, y=0, angleDeg=166, lengthM=0.14, com=(0.06, 0)),
    "hand": dict(x=0.14, y=0, angleDeg=189, lengthM=0.415, com=(0.12, 0)),
    "thigh": dict(x=0, y=0, z=0.022, angleDeg=-29, lengthM=0.058, com=(0.03, 0)),
    "shank": dict(x=0.058, y=0, angleDeg=-101, lengthM=0.105, com=(0.04, 0)),
    "tarsus": dict(x=0.105, y=0, angleDeg=38, lengthM=0.066, com=(0.033, 0)),
    "foot": dict(x=0.066, y=0, angleDeg=78, lengthM=0.058, com=(0.025, 0)),
}
MASS = {"humerus": 0.03, "forearm": 0.015}
LIMITS = {
    "humerus": (-90, 200),
    "forearm": (-200, 40),
    "hand": (-30, 180),
    "thigh": (-100, 80),
    "shank": (-80, 80),
    "tarsus": (-120, 120),
    "foot": (-110, 110),
}
TITLES = {
    "trunk": {"en": "Trunk: thoracic column and synsacrum from the hips to the neck base, with ribcage, keel, flight muscles and gut", "de": "Rumpf: Brustwirbelsäule und Synsacrum von den Hüften bis zum Halsansatz, mit Brustkorb, Kiel, Flugmuskeln und Darm"},
    "humerus": ("Humerus with the inner wing (tertials and inner secondaries)", "Oberarm mit dem inneren Flügel (Schirmfedern und innere Armschwingen)"),
    "forearm": ("Forearm (ulna and radius) with the secondaries", "Unterarm (Elle und Speiche) mit den Armschwingen"),
    "hand": ("Hand (carpometacarpus and digits) with the primaries, to the tip of the longest primary", "Hand (Carpometacarpus und Finger) mit den Handschwingen, bis zur Spitze der längsten Handschwinge"),
    "thigh": ("Thigh (femur, inside the plumage)", "Oberschenkel (Femur, im Gefieder)"),
    "shank": ("Shank (tibiotarsus)", "Unterschenkel (Tibiotarsus)"),
    "tarsus": ("Tarsus (tarsometatarsus, bare)", "Lauf (Tarsometatarsus, unbefiedert)"),
    "foot": ("Webbed foot (three front toes with webs, hind toe), as long as the middle toe", "Schwimmfuß (drei Vorderzehen mit Schwimmhäuten, Hinterzehe), so lang wie die Mittelzehe"),
}
SIDE = {"n": ("Near", "Naher", "Nahes"), "f": ("Far", "Ferner", "Fernes")}
GENDER = {"humerus": 1, "forearm": 1, "hand": 2, "thigh": 2, "shank": 1, "tarsus": 2, "foot": 1}

WINGS = {
    "glide": dict(hum=(-0.17, 0.09, 0.98), fore=(0.27, 0.04, 0.96), hand=(-0.27, -0.04, 0.96), trail=(-1, -0.04, 0), hint=(95, -156, 150)),
    "flare": dict(hum=(-0.05, 0.72, 0.69), fore=(0.45, 0.42, 0.79), hand=(-0.12, 0.5, 0.86), trail=(-0.8, -0.6, 0), hint=(70, -150, 140)),
    "touchdown": dict(hum=(-0.3, 0.6, 0.74), fore=(0.32, 0.45, 0.83), hand=(-0.62, 0.3, 0.73), trail=(-0.9, -0.4, 0), hint=(80, -140, 125)),
}
WING_OF = {"gliding": ("glide", 0), "gliding-up": ("glide", 4), "gliding-down": ("glide", -4), "approach-a": ("glide", 0), "landing": ("flare", 0), "flare-key": ("flare", 0), "touchdown": ("touchdown", 0)}

ANCHORS = {
    "contact-n": {"x": 0.024, "y": -0.0055, "footprintLengthM": 0.06, "footprintWidthM": 0.055},
    "contact-f": {"x": 0.024, "y": -0.0055, "footprintLengthM": 0.06, "footprintWidthM": 0.055},
    "nostrils": {"x": 0.087, "y": 0.0005},
    "bill-tip": {"x": 0.1255, "y": -0.0105},
    "eye": {"x": 0.03, "y": 0.0155},
    "back-plume": {"x": 0.045, "y": 0.0435},
}
MATERIALS = [
    {"id": "eye-tissue", "title": {"en": "Eye (pale yellow iris, cornea and lids)", "de": "Auge (blassgelbe Iris, Hornhaut und Lider)"}, "emissivity": 0.97, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "bone", "title": {"en": "Bone (pneumatised long bones, fused trunk)", "de": "Knochen (pneumatisierte Röhrenknochen, verwachsener Rumpf)"}, "emissivity": 0.95, "densityKgM3": 2100, "elasticModulusPa": 17000000000, "strengthPa": 150000000, "source": "authored-estimate", "reference": "dumont-2010"},
]
REFERENCES = [
    {"id": "imnh-r976", "citation": "Idaho Museum of Natural History, virtual osteology collection, specimen IMNH R-976, Larus argentatus (European herring gull), Pocatello: carpometacarpus 71.7 mm, tarsometatarsus 66.0 mm, femur 58.1 mm, coracoid 50.1 mm, sternal keel 73.1 mm long.", "url": "https://paleo.iri.isu.edu/ViewSpecimen.aspx?id=571"},
    {"id": "dumont-2010", "citation": "Dumont, E. R. (2010). Bone density and the lightweight skeletons of birds. Proceedings of the Royal Society B 277, 2193-2198. Bird bone tissue is denser (about 2.1 g/cm3) and stiffer than that of mammals of the same size; modulus and strength here are authored estimates for cortical bone."},
]
CRAMP_ADD = " Biometrics of argentatus/argenteus: wing (wrist to tip) 400-450 mm, tail 160-180 mm, bill (culmen) 48-60 mm with a depth at the gonys of 17-21 mm, tarsus 58-70 mm; the humerus (about 135 mm) and the ulna (about 140 mm) are authored estimates from the wingspan less the hand and the body."


def bone_spec(bid):
    return REST[bid] if bid in REST else REST[bid[:-2]]


SUMMARY = {"en": "An adult herring gull (Larus argentatus, 1.0 kg, wingspan 1.45 m): upright standing, soaring on spread wings and a landing flare to touchdown; rig with the trunk hinged at the hips, a two-piece neck, three-piece wings (humerus, forearm, hand) that fold in the real Z, webbed legs; plumage thermal model (41 degC core), nares breathing, back plume, breathing patterns for rest, soaring and the flare. Every view is a projection of the same three-dimensional chain, so spread wings are foreshortened in the side elevation.", "de": "Eine erwachsene Silbermöwe (Larus argentatus, 1,0 kg, Spannweite 1,45 m): aufrechtes Stehen, Segeln mit gespreizten Flügeln und ein Landeanflug bis zum Aufsetzen; Rig mit an den Hüften gelagertem Rumpf, zweiteiligem Hals, dreiteiligen Flügeln (Oberarm, Unterarm, Hand), die sich im wirklichen Z falten, Schwimmbeinen; Gefieder-Wärmemodell (41 Grad Kern), Nasenatmung, Rückenfahne, Atemmuster für Ruhe, Segeln und Abfangen. Jede Ansicht ist eine Projektion derselben räumlichen Kette, so sind gespreizte Flügel im Seitenaufriss verkürzt."}
DESCRIPTIONS = {
    "standing": {"en": "Standing upright on both pink webbed feet, body tilted 14 degrees, neck raised in a short S with the bill slightly down, wings folded with the black primaries crossing over the tail and 75 mm beyond it.", "de": "Aufrecht auf beiden rosa Schwimmfüßen stehend, Körper um 14 Grad geneigt, Hals in kurzem S aufgerichtet, Schnabel leicht gesenkt, Flügel angelegt, die schwarzen Handschwingen über dem Schwanz gekreuzt und 75 mm darüber hinaus."},
    "gliding": {"en": "Soaring in lift at about 1.5 m above the ground: both wings spread to the 1.45 m span with a slight dihedral, the elbow back, the wrist forward and the hand swept back (end-on and foreshortened in the side elevation), neck drawn in, tail spread, feet trailing.", "de": "Segeln in der Aufwindzone etwa 1,5 m über dem Boden: beide Flügel auf die Spannweite von 1,45 m gespreizt, leicht V-förmig, Ellbogen hinten, Handgelenk vorn und Hand zurückgeschwungen (im Seitenaufriss von vorn gesehen und verkürzt), Hals eingezogen, Schwanz gespreizt, Füße nach hinten."},
}


def side_of(bid):
    return bid[-1] if bid[-2:] in ("-n", "-f") else None


def build_rig():
    old = {b["id"]: b for b in ORIG["rig"]["bones"]}
    order = ["trunk", "tail", "neck-1", "neck-2", "head"]
    for s in ("n", "f"):
        order += ["humerus-" + s, "forearm-" + s, "hand-" + s]
    for s in ("n", "f"):
        order += ["thigh-" + s, "shank-" + s, "tarsus-" + s, "foot-" + s]
    parents = {"trunk": None, "tail": "trunk", "neck-1": "trunk", "neck-2": "neck-1", "head": "neck-2", "humerus": "trunk", "forearm": "humerus", "hand": "forearm", "thigh": "trunk", "shank": "thigh", "tarsus": "shank", "foot": "tarsus"}
    bones = []
    for bid in order:
        s = side_of(bid)
        key = bid[:-2] if s else bid
        spec = REST[key]
        base = old.get(bid) or old.get({"humerus": "arm", "forearm": "arm"}.get(key, key) + "-" + (s or ""))
        b = {"id": bid, "parent": None if parents[key] is None else parents[key] + ("-" + s if parents[key] not in ("trunk",) and s else "")}
        if key in TITLES:
            t = TITLES[key]
            if isinstance(t, dict):
                b["title"] = t
            else:
                en, de = SIDE[s][0] + " " + t[0][0].lower() + t[0][1:], SIDE[s][GENDER[key]] + " " + t[1]
                b["title"] = {"en": en, "de": de}
        else:
            b["title"] = base["title"]
        b["x"] = spec["x"]
        b["y"] = spec["y"]
        if "z" in spec:
            b["z"] = (-1 if s == "n" else 1) * spec["z"]
        b["angleDeg"] = spec["angleDeg"]
        b["lengthM"] = spec["lengthM"]
        joint = copy.deepcopy(base["joint"])
        if key in LIMITS:
            joint["minDeg"], joint["maxDeg"] = LIMITS[key]
        b["joint"] = joint
        mass = copy.deepcopy(base["mass"])
        if key in MASS:
            mass["kg"] = MASS[key]
        mass["comX"], mass["comY"] = spec["com"]
        b["mass"] = mass
        bones.append(b)
    rig = copy.deepcopy(ORIG["rig"])
    rig["bones"] = bones
    return rig


def rot_frame(P, th, ph, tw):
    u0, v0, w0 = P

    def comb(a, s, b, t):
        return [a[i] * s + b[i] * t for i in range(3)]

    ts, tc = math.sin(math.radians(th)), math.cos(math.radians(th))
    u1, v1 = comb(u0, tc, v0, ts), comb(u0, -ts, v0, tc)
    ps, pc = math.sin(math.radians(ph)), math.cos(math.radians(ph))
    u2, w2 = comb(u1, pc, w0, ps), comb(u1, -ps, w0, pc)
    ws, wc = math.sin(math.radians(tw)), math.cos(math.radians(tw))
    return u2, comb(v1, wc, w2, -ws), comb(v1, ws, w2, wc)


def solve(rig, pose):
    """🦴 World frames of every bone (origin, u, v, w as [ahead, up, left]) exactly as the product's chain: sagittal angle, then abduction about v, then twist about u."""
    root = pose.get("root", {})
    ra = root.get("angleDeg", 0)
    base = ([root.get("x", 0), root.get("y", 0), 0], [math.cos(math.radians(ra)), math.sin(math.radians(ra)), 0], [-math.sin(math.radians(ra)), math.cos(math.radians(ra)), 0], [0, 0, 1])
    out = {}
    for b in rig["bones"]:
        off = pose.get("bones", {}).get(b["id"], {})
        P = base if b["parent"] is None else out[b["parent"]]
        o, u, v, w = P
        along = [u[i] * b["x"] + v[i] * b["y"] for i in range(3)]
        z = b.get("z", 0)
        origin = [o[i] + along[i] + w[i] * z for i in range(3)]
        uu, vv, ww = rot_frame((u, v, w), b["angleDeg"] + off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0))
        out[b["id"]] = (origin, uu, vv, ww)
    return out


def tip(F, bid, rig, s=None, t=0.0):
    o, u, v, _ = F[bid]
    L = next(b["lengthM"] for b in rig["bones"] if b["id"] == bid) if s is None else s
    return [o[i] + u[i] * L + v[i] * t for i in range(3)]


def wang(F, bid):
    u = F[bid][1]
    return math.degrees(math.atan2(u[1], u[0]))


def norm(a):
    while a > 180:
        a -= 360
    while a <= -180:
        a += 360
    return a


def unit(a):
    l = math.sqrt(sum(c * c for c in a)) or 1
    return [c / l for c in a]


def dotv(a, b):
    return sum(a[i] * b[i] for i in range(3))


def solve_bone(P, rest, ut, trail, hint):
    """🎯 Offset (angle, abduction, twist) that turns a bone of rest angle `rest` in the parent frame P so that its axis is `ut` and its trailing side (-v) lies along `trail` (made perpendicular to the axis); of the two solutions the one nearest to `hint` is taken, so clips swing the wing the anatomical way."""
    _, Pu, Pv, Pw = P
    ut = unit(ut)
    vt = unit([-trail[i] + dotv(trail, ut) * ut[i] for i in range(3)])
    best = None
    s = max(-1, min(1, dotv(ut, Pw)))
    for ph in (math.degrees(math.asin(s)), 180 - math.degrees(math.asin(s))):
        c = math.cos(math.radians(ph))
        if abs(c) < 1e-6:
            continue
        th = math.degrees(math.atan2(dotv(ut, Pv) / c, dotv(ut, Pu) / c))
        u2, v1, w2 = rot_frame((Pu, Pv, Pw), th, ph, 0)
        tw = math.degrees(math.atan2(-dotv(vt, w2), dotv(vt, v1)))
        off = norm(th - rest)
        if abs(norm(off - hint)) > 90:
            off = off + 360 if off < hint else off - 360
        cand = (abs(off - hint) + abs(tw) * 0.5, round(off, 2), round(norm(ph), 2), round(norm(tw), 2))
        if best is None or cand < best:
            best = cand
    return {"angleDeg": best[1], "abductionDeg": best[2], "twistDeg": best[3]}


def wings(p, rig, family, lift):
    """🪽 Spread wings of a pose: span directions of humerus, forearm and hand and the trailing direction stated in the trunk frame (x ahead, y dorsal, z outward), the dihedral changed by `lift` degrees, solved bone by bone down the chain."""
    spec = WINGS[family]
    F = solve(rig, p)
    _, tu, tv, tw = F["trunk"]
    for s, sg in (("n", -1), ("f", 1)):
        prev = "trunk"
        for key, hint in zip(("hum", "fore", "hand"), spec["hint"]):
            x, y, z = spec[key]
            r = math.radians(lift)
            y, z = y * math.cos(r) + z * math.sin(r), z * math.cos(r) - y * math.sin(r)
            world = lambda a: [tu[i] * a[0] + tv[i] * a[1] + tw[i] * sg * a[2] for i in range(3)]
            bid = {"hum": "humerus-", "fore": "forearm-", "hand": "hand-"}[key] + s
            rest = REST[{"hum": "humerus", "fore": "forearm", "hand": "hand"}[key]]["angleDeg"]
            p["bones"][bid] = solve_bone(F[prev], rest, world((x, y, z)), world(spec["trail"]), hint)
            F = solve(rig, p)
            prev = bid


def legs(pose, old_frames, new_rig, report):
    """🦵 Thigh and shank solved anew (knee ahead of the hip-ankle line) so that the ankle keeps its old place relative to the hip, scaled to the new reach; tarsus and foot keep their old world directions."""
    old_rig = ORIG["rig"]
    fem, tib = REST["thigh"]["lengthM"], REST["shank"]["lengthM"]
    old_len = {b["id"]: b["lengthM"] for b in old_rig["bones"]}
    for s in ("-n", "-f"):
        hip, ankle = old_frames["thigh" + s][0], old_frames["tarsus" + s][0]
        rel = [ankle[0] - hip[0], ankle[1] - hip[1]]
        k = (fem + tib) / (old_len["thigh" + s] + old_len["shank" + s])
        rel = [rel[0] * k, rel[1] * k]
        d = math.hypot(*rel)
        if d > fem + tib - 0.004:
            rel = [q * (fem + tib - 0.004) / d for q in rel]
            d = fem + tib - 0.004
            report.append("%s leg%s stretched to the reach" % (pose["id"], s))
        base = math.degrees(math.atan2(rel[1], rel[0]))
        inner = math.degrees(math.acos(max(-1, min(1, (fem ** 2 + d * d - tib ** 2) / (2 * fem * d)))))
        thigh_w = base + inner
        knee = [math.cos(math.radians(thigh_w)) * fem, math.sin(math.radians(thigh_w)) * fem]
        shank_w = math.degrees(math.atan2(rel[1] - knee[1], rel[0] - knee[0]))
        trunk_w = wang(old_frames, "trunk")
        tars_w, foot_w = wang(old_frames, "tarsus" + s), wang(old_frames, "foot" + s)
        pose["bones"]["thigh" + s] = {"angleDeg": round(norm(thigh_w - trunk_w - REST["thigh"]["angleDeg"]), 2)}
        pose["bones"]["shank" + s] = {"angleDeg": round(norm(shank_w - thigh_w - REST["shank"]["angleDeg"]), 2)}
        pose["bones"]["tarsus" + s] = {"angleDeg": round(norm(tars_w - shank_w - REST["tarsus"]["angleDeg"]), 2)}
        pose["bones"]["foot" + s] = {"angleDeg": round(norm(foot_w - tars_w - REST["foot"]["angleDeg"]), 2)}


def build_poses(rig, report):
    out = []
    old_rig = ORIG["rig"]
    for p0 in ORIG["poses"]:
        p = copy.deepcopy(p0)
        F0 = solve(old_rig, p0)
        bones = {}
        trunk = p0["bones"].get("trunk", {}).get("angleDeg", 0)
        upright = p["id"] in ("standing", "settled")
        if upright:
            trunk = TRUNK_STAND
        bones["trunk"] = {"angleDeg": trunk}
        if "tail" in p0["bones"]:
            bones["tail"] = {"angleDeg": p0["bones"]["tail"]["angleDeg"]}
        if not upright:
            prev = trunk
            for bid in ("neck-1", "neck-2", "head"):
                wa = wang(F0, bid)
                bones[bid] = {"angleDeg": round(norm(wa - prev - REST[bid]["angleDeg"]), 2)}
                prev = wa
        p["bones"] = bones
        if not upright:
            legs(p, F0, rig, report)
        if p["id"] in WING_OF:
            wings(p, rig, *WING_OF[p["id"]])
        if p["id"] in DESCRIPTIONS:
            p["description"] = DESCRIPTIONS[p["id"]]
        root = dict(p0.get("root", {}))
        root["x"] = round(root.get("x", 0) + 0.17, 4)
        p["root"] = root
        out.append(p)
    for p in out:
        if p.get("ground") == "authored" and p.get("contacts"):
            F = solve(rig, {**p, "root": {**p["root"], "y": 0}})
            low = min(min(tip(F, "foot" + s, rig, 0.03)[1], F["foot" + s][0][1]) for s in ("-n", "-f"))
            p["root"]["y"] = round(-low + SOLE, 4)
    limits = {b["id"]: (b["joint"]["minDeg"], b["joint"]["maxDeg"]) for b in rig["bones"]}
    for p in out:
        for bid, off in p["bones"].items():
            lo, hi = limits[bid]
            if not lo <= off.get("angleDeg", 0) <= hi:
                report.append("%s %s %.1f outside %s" % (p["id"], bid, off["angleDeg"], limits[bid]))
    return out


def anchors():
    out = copy.deepcopy(ORIG["anchors"])
    for a in out:
        a.update(ANCHORS.get(a["id"], {}))
    return out


def provenance():
    p = copy.deepcopy(ORIG["provenance"])
    for r in p["references"]:
        if r["id"] == "cramp-simmons-1983" and CRAMP_ADD not in r["citation"]:
            r["citation"] += CRAMP_ADD
    have = {r["id"] for r in p["references"]}
    p["references"] += [r for r in REFERENCES if r["id"] not in have]
    return p


def materials():
    out = copy.deepcopy(ORIG["materials"])
    return out + [m for m in MATERIALS if not any(q["id"] == m["id"] for q in out)]


def members(report=None):
    report = [] if report is None else report
    rig = build_rig()
    return {"summary": SUMMARY, "provenance": provenance(), "materials": materials(), "rig": rig, "anchors": anchors(), "poses": build_poses(rig, report)}


if __name__ == "__main__":
    notes = []
    m = members(notes)
    print("\n".join(notes) or "all poses inside the joint limits")
    rig = m["rig"]
    for b in rig["bones"]:
        print(b["id"].ljust(10), b["parent"], b["x"], b["y"], b.get("z", ""), b["angleDeg"], b["lengthM"], b["mass"]["kg"])
    print("mass", round(sum(b["mass"]["kg"] for b in rig["bones"]), 4))
    for p in m["poses"]:
        F = solve(rig, p)
        pts = {k: [round(c, 3) for c in F[k][0]] for k in ("trunk", "neck-1", "head", "humerus-f", "forearm-f", "hand-f", "thigh-n", "shank-n", "tarsus-n", "foot-n")}
        print(p["id"], p["root"], "bill", [round(c, 3) for c in tip(F, "head", rig)], "handtip-f", [round(c, 3) for c in tip(F, "hand-f", rig)], "handtip-n", [round(c, 3) for c in tip(F, "hand-n", rig)])
        print("   ", pts)
