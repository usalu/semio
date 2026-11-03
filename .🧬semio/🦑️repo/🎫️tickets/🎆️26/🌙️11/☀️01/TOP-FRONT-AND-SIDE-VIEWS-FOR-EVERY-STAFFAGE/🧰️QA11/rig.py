"""QA11 rig pass, run after the VE builders: real steering head axes and real vertical door hinges.

usage: python rig.py <asset-id>
"""
import copy
import json
import math
import re
import sys
import time

sys.path.insert(0, "../🧰️VE")
from lib import dumps, path_of, r4, raw, rest_world, write_atomic


def rot(x, y, deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return x * c - y * s, x * s + y * c


def rot_path(d, deg):
    toks = re.findall(r"[A-Za-z]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out, i, cmd = [], 0, None
    while i < len(toks):
        if re.match(r"[A-Za-z]", toks[i]):
            cmd = toks[i]
            out.append(cmd)
            i += 1
            if cmd in "Zz":
                continue
        if cmd in "MLCQST":
            n = {"M": 1, "L": 1, "C": 3, "Q": 2, "S": 2, "T": 1}[cmd]
            for _ in range(n):
                x, y = rot(float(toks[i]), float(toks[i + 1]), deg)
                out.append(f"{r4(x)},{r4(y)}")
                i += 2
        elif cmd == "H" or cmd == "V":
            raise ValueError("H/V path commands cannot be rotated")
        else:
            raise ValueError("unsupported path command " + str(cmd))
    return " ".join(out)


def rot_shape(s, deg):
    k = s["kind"]
    if k == "poly":
        s["points"] = [[r4(a), r4(b)] for a, b in (rot(x, y, deg) for x, y in s["points"])]
    elif k == "capsule":
        s["x0"], s["y0"] = (r4(v) for v in rot(s["x0"], s["y0"], deg))
        s["x1"], s["y1"] = (r4(v) for v in rot(s["x1"], s["y1"], deg))
    elif k == "ellipse":
        s["cx"], s["cy"] = (r4(v) for v in rot(s["cx"], s["cy"], deg))
        if s["rx"] != s["ry"]:
            s["rotationDeg"] = r4(s.get("rotationDeg", 0) + deg)
    elif k == "rect":
        pts = [(s["x"], s["y"]), (s["x"] + s["w"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"]), (s["x"], s["y"] + s["h"])]
        for key in ("x", "y", "w", "h", "r"):
            s.pop(key, None)
        s["kind"] = "poly"
        s["points"] = [[r4(a), r4(b)] for a, b in (rot(x, y, deg) for x, y in pts)]
        s["closed"] = True
    elif k == "path":
        s["d"] = rot_path(s["d"], deg)
    else:
        raise ValueError(k)


def reaxe(asset, bone_id, world_deg, length=None, limits=(0, 0)):
    """Turns the local frame of a bone to a new world angle while everything stays where it is: children, shapes in the bone frame, mass centre, anchors."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    bone = bones[bone_id]
    delta = world_deg - math.degrees(rest_world(asset)[bone_id][2])
    bone["angleDeg"] = r4(bone.get("angleDeg", 0) + delta)
    if length is not None:
        bone["lengthM"] = length
    if limits is not None and bone["joint"]["type"] == "revolute":
        bone["joint"]["minDeg"], bone["joint"]["maxDeg"] = limits
    m = bone.get("mass")
    if m and "comX" in m:
        m["comX"], m["comY"] = (r4(v) for v in rot(m["comX"], m["comY"], -delta))
    for c in bones.values():
        if c["parent"] == bone_id:
            c["x"], c["y"] = (r4(v) for v in rot(c["x"], c["y"], -delta))
            c["angleDeg"] = r4(c.get("angleDeg", 0) - delta)
    for an in asset["anchors"]:
        if an.get("bone") == bone_id:
            an["x"], an["y"] = (r4(v) for v in rot(an["x"], an["y"], -delta))
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        for view, shapes in rep["geometry"].items():
            for s in shapes:
                if s["bone"] == bone_id and (view == "elevation" or "solid" in s):
                    rot_shape(s, -delta)


def lateral_extent(s, view):
    k = s["kind"]
    if k == "rect":
        return (s["x"], s["x"] + s["w"]) if view == "front" else (s["y"], s["y"] + s["h"])
    i = 0 if view == "front" else 1
    if k == "poly":
        v = [p[i] for p in s["points"]]
        return min(v), max(v)
    if k == "capsule":
        v = [s["x0"], s["x1"]] if i == 0 else [s["y0"], s["y1"]]
        r = max(s["r0"], s["r1"])
        return min(v) - r, max(v) + r
    if k == "ellipse":
        c, r = (s["cx"], s["rx"]) if i == 0 else (s["cy"], s["ry"])
        return c - r, c + r
    raise ValueError(k)


def wide_solids(asset, bones, min_width=0.12):
    """Wide front and top shapes of a steered bone become extruded solids over the profile of their side view, so the side view follows the steering too."""
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        g = rep["geometry"]
        for view in ("front", "plan"):
            for s in g[view]:
                if s["bone"] not in bones or "solid" in s or s["kind"] not in ("rect", "poly", "capsule", "ellipse"):
                    continue
                e = next((e for e in g["elevation"] if e["id"] == s["id"] and e["bone"] == s["bone"]), None)
                if e is None:
                    continue
                lo, hi = lateral_extent(s, view)
                if hi - lo <= min_width:
                    continue
                for key in [k for k in list(s) if k in ("kind", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "followTip", "d", "points", "closed")]:
                    s.pop(key)
                s.update({k: copy.deepcopy(v) for k, v in e.items() if k in ("kind", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d", "points", "closed")})
                s["solid"] = {"kind": "extruded", "z0": r4(lo), "z1": r4(hi)}


def leg_solids(asset, bone, ids=("fork-l", "fork-r")):
    """Fork legs lie along the steering axis: a capsule from the bone origin to the wheel hub in the bone frame, extruded over the lateral range of the authored leg."""
    hub = next(b for b in asset["rig"]["bones"] if b["parent"] == bone and b["id"] == "wheel-f")
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        for view in ("front", "plan"):
            for s in rep["geometry"][view]:
                if s["bone"] != bone or s["id"] not in ids or "solid" in s:
                    continue
                lo, hi = lateral_extent(s, view)
                half = (hi - lo) / 2
                for key in [k for k in list(s) if k in ("kind", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "points", "closed")]:
                    s.pop(key)
                s.update({"kind": "capsule", "x0": 0, "y0": 0, "x1": hub["x"], "y1": hub["y"], "r0": r4(min(half, 0.02)), "r1": r4(min(half, 0.02)), "solid": {"kind": "extruded", "z0": r4(lo), "z1": r4(hi)}})


def wheel_solids(asset, wheel="wheel-f", steer=None):
    """The front and top shapes of a steered wheel as extruded discs on the wheel bone: a disc is invariant under the spin of the wheel and its lateral sweep follows the steering."""
    hx, hy, _ = rest_world(asset)[wheel]
    extras = []
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        for view in ("front", "plan"):
            for s in rep["geometry"][view]:
                if s["bone"] != wheel or "solid" in s or s["kind"] != "rect":
                    continue
                x, y, w, h = s["x"], s["y"], s["w"], s["h"]
                lat0, lat1, r = (x, x + w, min(hy - y, y + h - hy)) if view == "front" else (y, y + h, min(hx - x, x + w - hx))
                if abs(r - (h if view == "front" else w) / 2) > 0.04:
                    raise ValueError(f"{s['id']} is not centred on the hub")
                top = (y + h - hy - r) if view == "front" else 0
                for key in ("x", "y", "w", "h", "r"):
                    s.pop(key, None)
                s.update({"kind": "ellipse", "cx": 0, "cy": 0, "rx": r4(r), "ry": r4(r), "solid": {"kind": "extruded", "z0": r4(lat0), "z1": r4(lat1)}})
                if top > 0.015 and steer is not None:
                    sx, sy, sa = rest_world(asset)[steer]
                    pts = [rot(hx + dx - sx, y0 - sy, -math.degrees(sa)) for dx, y0 in ((-0.02, hy + r - 0.01), (0.02, hy + r - 0.01), (0.02, hy + r + top), (-0.02, hy + r + top))]
                    extra = copy.deepcopy(s)
                    for key in ("rx", "ry", "cx", "cy"):
                        extra.pop(key)
                    extra.update({"id": s["id"] + "-top", "bone": steer, "kind": "poly", "points": [[r4(a), r4(b)] for a, b in pts], "closed": True, "z": r4(s["z"] + 0.01)})
                    extras.append((rep["geometry"][view], extra))


    for shapes, extra in extras:
        shapes.append(extra)


def fender_solids(asset, steer, ids=("fender-f",)):
    """The front fender follows the steering with the outline of its side view as the profile."""
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        profile = next((e for e in rep["geometry"]["elevation"] if e["id"] == "fender-f" and e["bone"] == steer), None)
        if profile is None:
            continue
        for view in ("front", "plan"):
            for s in rep["geometry"][view]:
                if s["id"] in ids and s["bone"] == "wheel-f":
                    lat0, lat1 = (s["x"], s["x"] + s["w"]) if view == "front" else (s["y"], s["y"] + s["h"])
                    for key in ("x", "y", "w", "h", "r"):
                        s.pop(key, None)
                    s.update({"kind": "poly", "points": copy.deepcopy(profile["points"]), "closed": True, "bone": steer, "solid": {"kind": "extruded", "z0": r4(lat0), "z1": r4(lat1)}})


def clone_pose(asset, src, new_id, title, description, bones):
    p = copy.deepcopy(next(p for p in asset["poses"] if p["id"] == src))
    p.pop("key", None)
    p["id"] = new_id
    p["title"], p["description"] = title, description
    p["bones"].update(bones)
    asset["poses"].append(p)


def pose_bones(asset, pid, fn):
    for p in asset["poses"]:
        if p["id"] == pid:
            fn(p["bones"])


def sub_text(asset, pid, old_en, new_en, old_de, new_de):
    for p in asset["poses"]:
        if p["id"] == pid:
            p["description"]["en"] = p["description"]["en"].replace(old_en, new_en)
            p["description"]["de"] = p["description"]["de"].replace(old_de, new_de)
            if "title" in p:
                pass


def steer_decision(asset, en, de):
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        cur = rep["decisions"]["overlays"]
        cur["en"] = cur["en"].rstrip() + " " + en
        cur["de"] = cur["de"].rstrip() + " " + de


def bike(asset):
    reaxe(asset, "steer", -70, 0.625)
    wide_solids(asset, ("steer",))
    fender_solids(asset, "steer")
    wheel_solids(asset)
    plan = asset["representations"]["200"]["geometry"]["plan"]
    plan[:] = [s for s in plan if s["id"] != "fork-r"]
    next(s for s in plan if s["id"] == "fork")["solid"].update({"z0": -0.057, "z1": 0.057})
    for pid in ("parked-on-stand", "parked-stopped"):
        pose_bones(asset, pid, lambda b: b.update({"steer": {"twistDeg": -15}}))
        for p in asset["poses"]:
            if p["id"] == pid:
                for lang in ("en", "de"):
                    for k in ("description",):
                        p[k][lang] = p[k][lang].replace("6 degrees", "15 degrees").replace("6 Grad", "15 Grad")
    steer_decision(asset,
                   "Steering: the steer bone lies along the head tube (70 degrees, fork rake 4.5 cm), fork, wheel and handlebar ride on it, and a steered pose is a twist about that axis, so front and top turn the front wheel and the handlebar like the real bike.",
                   "Lenkung: Der Lenkknochen liegt in der Steuerrohrachse (70 Grad, Gabelvorlauf 4,5 cm), Gabel, Rad und Lenker hängen an ihm, und eine Lenkpose ist eine Drehung um diese Achse, sodass Vorder- und Draufsicht das Vorderrad und den Lenker wie am echten Rad drehen.")


def motor(asset):
    reaxe(asset, "steer", -73, 0.78)
    wide_solids(asset, ("steer",))
    wheel_solids(asset, steer="steer")
    leg_solids(asset, "steer")
    clone_pose(asset, "cruising", "turning-left", {"en": "Turning left at 15 km/h", "de": "Linkskurve mit 15 km/h"},
               {"en": "Stand up, both wheels on the ground, handlebar turned 14 degrees to the left, engine at part load.", "de": "Ständer oben, beide Räder am Boden, Lenker 14 Grad nach links eingeschlagen, Motor im Teillastbetrieb."},
               {"steer": {"twistDeg": -14}})
    steer_decision(asset,
                   "Steering: the steer bone lies along the steering axis (73 degrees), fork, front wheel, apron and handlebar ride on it, and steering is a twist about that axis, so front and top turn the wheel and the handlebar.",
                   "Lenkung: Der Lenkknochen liegt in der Lenkachse (73 Grad), Gabel, Vorderrad, Schürze und Lenker hängen an ihm, und das Lenken ist eine Drehung um diese Achse, sodass Vorder- und Draufsicht Rad und Lenker drehen.")


def kick(asset):
    reaxe(asset, "fork", 98, 0.8)
    wide_solids(asset, ("fork",))
    wheel_solids(asset)
    pose_bones(asset, "braking-stop", lambda b: b.update({"fork": {"twistDeg": 10}}))
    sub_text(asset, "braking-stop", "4 degrees", "10 degrees", "4 Grad", "10 Grad")
    steer_decision(asset,
                   "Steering: the fork bone lies along the steering stem (98 degrees from the deck), wheel and bars ride on it, and the steered braking pose is a twist about that axis, so front and top turn the wheel and the handlebar.",
                   "Lenkung: Der Gabelknochen liegt in der Lenkstange (98 Grad zum Trittbrett), Rad und Lenker hängen an ihm, und die gelenkte Bremspose ist eine Drehung um diese Achse, sodass Vorder- und Draufsicht Rad und Lenker drehen.")



def van(asset):
    bones = asset["rig"]["bones"]
    old = next(b for b in bones if b["id"] == "rear-door")
    idx = bones.index(old)
    new = []
    for side, z, twist in (("r", -0.88, -90), ("l", 0.88, 90)):
        b = copy.deepcopy(old)
        b.update({"id": f"rear-door-{side}", "x": -2.67, "y": 0.55, "z": z, "angleDeg": 90, "lengthM": 1.35, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0}})
        b["title"] = {"en": f"Rear door leaf, {'right' if side == 'r' else 'left'}, vertical hinge", "de": f"Hecktürflügel {'rechts' if side == 'r' else 'links'}, senkrechtes Scharnier"}
        b["mass"].update({"kg": 21, "comX": 0.675, "comY": 0.45})
        new.append(b)
    bones[idx:idx + 1] = new
    for scale, rep in asset["representations"].items():
        if rep["status"] != "authored":
            continue
        g = rep["geometry"]
        leaf = next(s for s in g["elevation"] if s["bone"] == "rear-door")
        at = g["elevation"].index(leaf)
        shapes = []
        for side, dz in (("r", 0), ("l", 0.05)):
            s = copy.deepcopy(leaf)
            s.update({"id": f"rear-leaf-{side}", "bone": f"rear-door-{side}", "points": [[0, 0], [1.35, 0], [1.35, 0.9], [0, 0.9]], "z": r4(leaf["z"] - dz)})
            if scale in ("200", "500"):
                s["filled"] = False
            shapes.append(s)
        g["elevation"][at:at + 1] = shapes
        for s in g["plan"]:
            if s["id"] in ("rear-leaf-r", "rear-leaf-l"):
                right = s["id"].endswith("r")
                s["bone"] = "rear-door-" + s["id"][-1]
                th = {"500": 0.14, "200": 0.06}.get(scale, 0.05)
                s.update({"x": -3.57, "y": (-0.88 if right else 0.88) - th / 2, "w": 0.9, "h": th})
                s.pop("r", None)
    for p in asset["poses"]:
        if p["id"] == "rear-doors-open":
            p["bones"].pop("rear-door", None)
            p["description"]["en"] = p["description"]["en"].replace("(projected 0.95 m behind the rear panel)", "(0.9 m behind the rear panel)")
            p["description"]["de"] = p["description"]["de"].replace("(projiziert 0,95 m hinter der Heckwand)", "(0,9 m hinter der Heckwand)")
        else:
            p["bones"].update({"rear-door-r": {"twistDeg": -90}, "rear-door-l": {"twistDeg": 90}})
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        cur = rep["decisions"]["overlays"]
        cur["en"] = cur["en"].rstrip() + " Rear doors: each leaf hangs on its own vertical hinge bone at the rear corner; the closed poses turn the leaves 90 degrees about the hinge and the open pose leaves them out, so front and top swing the leaves like the real doors."
        cur["de"] = cur["de"].rstrip() + " Hecktüren: Jeder Flügel hängt an einem eigenen senkrechten Scharnierknochen an der hinteren Ecke; die geschlossenen Posen drehen die Flügel um 90 Grad um das Scharnier, die offene Pose lässt sie ausgeschwenkt, sodass Vorder- und Draufsicht die Flügel wie die echten Türen schwenken."



def cargo(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    for b in ("fork", "bar"):
        bones[b]["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": 0}
    fx, fy = 1.85, 0.62
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        g = rep["geometry"]
        for s in g["elevation"]:
            if s["id"] == "steer-rod":
                s["bone"] = "frame"
                if s["kind"] == "capsule":
                    s["x0"], s["y0"], s["x1"], s["y1"] = r4(s["x0"] + fx), r4(s["y0"] + fy), r4(s["x1"] + fx), r4(s["y1"] + fy)
                else:
                    s["d"] = f"M {r4(fx - 1.11)},{fy} L {fx},{fy}"
        for view in ("front", "plan"):
            for s in g[view]:
                if s["id"] == "steer-rod":
                    s["bone"] = "frame"
    reaxe(asset, "fork", -55.58, 0.4424)
    reaxe(asset, "bar", 90, 0.43)
    wide_solids(asset, ("bar", "fork"))
    leg = {"kind": "capsule", "x0": 0, "y0": 0, "x1": 0.4424, "y1": 0, "r0": 0.015, "r1": 0.015}
    for rep in asset["representations"].values():
        if rep["status"] != "authored":
            continue
        g = rep["geometry"]
        fender = next((e for e in g["elevation"] if e["id"] == "front-fender"), None)
        for view in ("front", "plan"):
            for s in g[view]:
                if s["bone"] != "fork":
                    continue
                sid = s["id"]
                if sid in ("steer-fork", "fork-leg-l", "fork-leg-r", "fork") and not (view == "plan" and s["kind"] == "poly" and sid == "fork"):
                    lat0, lat1 = (s["x"], s["x"] + s["w"]) if view == "front" else ((s["y0"] - s["r0"], s["y0"] + s["r0"]) if s["kind"] == "capsule" else (s["y"], s["y"] + s["h"]))
                    geom = dict(leg)
                elif sid == "fork" and view == "plan":
                    lat0, lat1, geom = -0.07, 0.07, dict(leg)
                elif sid == "front-fender" and fender is not None:
                    lat0, lat1 = (s["x"], s["x"] + s["w"]) if view == "front" else (s["y"], s["y"] + s["h"])
                    geom = {"kind": "path", "d": fender["d"]}
                else:
                    continue
                for key in [k for k in list(s) if k in ("kind", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "followTip", "d", "points", "closed")]:
                    s.pop(key)
                s.update(geom)
                s["solid"] = {"kind": "extruded", "z0": r4(lat0), "z1": r4(lat1)}
    wheel_solids(asset, steer="fork")
    for s in asset["representations"]["100"]["geometry"]["front"]:
        if s["id"] == "bar-stem":
            s.update({"x": -0.015, "w": 0.03})
    clone_pose(asset, "rolling-loaded", "turning-left", {"en": "Pushed through a left bend", "de": "Geschoben in einer Linkskurve"},
               {"en": "Stand folded up, both tyres on the ground, 60 kg in the box, handlebar and front wheel turned 14 degrees to the left.", "de": "Ständer eingeklappt, beide Reifen am Boden, 60 kg in der Box, Lenker und Vorderrad 14 Grad nach links eingeschlagen."},
               {"bar": {"twistDeg": 14}, "fork": {"twistDeg": -14}})
    steer_decision(asset,
                   "Steering: the handlebar bone stands along its vertical stem and the fork bone along the fork axis (34 degrees from vertical), both turn by the same twist, and the steering rod is part of the frame because its lateral travel is a few centimetres; front and top turn the handlebar, fork and wheel.",
                   "Lenkung: Der Lenkerknochen steht in seinem senkrechten Vorbau und der Gabelknochen in der Gabelachse (34 Grad zur Senkrechten), beide drehen um denselben Winkel, und die Lenkstange gehört zum Rahmen, weil ihr seitlicher Weg nur wenige Zentimeter beträgt; vorn und oben drehen Lenker, Gabel und Rad.")


RECIPES = {"cargo-bike": cargo, "delivery-van": van, "city-bicycle": bike, "motor-scooter": motor, "electric-kick-scooter": kick}

if __name__ == "__main__":
    aid = sys.argv[1]
    doc = json.loads(raw(aid))
    RECIPES[aid](doc["asset"])
    text = dumps(doc)
    if raw(aid).endswith("\n"):
        text += "\n"
    write_atomic(path_of(aid), text)
    print("rigged", aid)
