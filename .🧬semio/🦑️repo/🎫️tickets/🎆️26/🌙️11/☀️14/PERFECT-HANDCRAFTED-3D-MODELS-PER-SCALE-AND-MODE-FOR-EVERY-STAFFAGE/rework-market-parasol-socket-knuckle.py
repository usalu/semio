"""🏖️ Market parasol, second round pass 1: slab base with a welded socket tube instead of the pin clevis, tilt knuckle moved up under the open runner.

Hand-chosen numbers (metres, world height in brackets):
- ballast slab 0.50 x 0.50 x 0.096 concrete (57.6 kg as before), steel flange 0.20 x 0.20 x 0.010, socket tube o60 / wall 5.5, 0.106 to 0.400, four 8 mm gussets, two clamp knobs
- mast bone (fixed in the socket): o48 lower pole down to 0.110, coupling collar at 1.07, o40 pole on to the knuckle at 1.80 (bone length 1.54)
- mast-upper bone: knuckle barrel o56 x 48 across the mast, o40 mast head 0.55 long, revolute +-30 degrees
- runner 0.1574 above the knuckle when open (world 1.957 as before), crown at 0.55

Usage: python3 rework-market-parasol-socket-knuckle.py [--publish]
Without --publish the candidate goes to 🗑️generated/r3-market-parasol/🔣️.json only.
"""
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/market-parasol/🔣️.json"
SOURCE = os.path.join(T, "volumes/objects/market-parasol.json")
CANDIDATE = os.path.join(T, "🗑️generated/r3-market-parasol/🔣️.json")

SHIFT = 0.73
KNUCKLE = 1.54
HEAD = 0.55
SLAB = 0.5
SLAB_H = 0.096
FLANGE_TOP = 0.106
SOCKET_TOP = 0.40
MAST_FOOT = -0.15
ALU = {"material": "aluminium-anodised"}
QUARTER = None


def tube(a, b, r_in, r_out):
    return [[a, r_in], [b, r_in], [b, r_out], [a, r_out], [a, r_in]]


def rod(a, b, r):
    return [[a, 0], [a, r], [b, r], [b, 0]]


def rig(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    mast, upper = bones["mast"], bones["mast-upper"]
    mast["lengthM"] = KNUCKLE
    mast["title"] = {"en": "Mast in the base socket with coupling collar, winch and crank", "de": "Mast in der Fußhülse mit Kupplungsmuffe, Winde und Kurbel"}
    mast["mass"].update({"kg": 2.5, "comX": 0.77})
    upper["x"] = KNUCKLE
    upper["lengthM"] = HEAD
    upper["title"] = {"en": "Mast head above the tilt knuckle", "de": "Mastkopf über dem Knickgelenk"}
    upper["joint"].update({"minDeg": -30, "maxDeg": 30})
    upper["mass"].update({"kg": 0.7, "comX": 0.275})
    bones["crown"]["x"] = HEAD
    bones["runner"]["x"] = round(0.8874 - SHIFT, 4)
    bones["base"]["mass"]["comY"] = SLAB_H / 2
    bones["base"]["title"] = {"en": "Concrete slab base with socket tube", "de": "Betonplattenfuß mit Standrohr"}
    for anchor in asset["anchors"]:
        if anchor["id"] == "socket":
            anchor["footprintLengthM"] = SLAB
            anchor["footprintWidthM"] = SLAB
    for pose in asset["poses"]:
        if pose["id"] == "tilted-to-sun":
            pose["bones"]["mast-upper"] = {"angleDeg": -20}
            pose["description"] = {
                "en": "Canopy tilted 20 degrees towards the sun on the right at the knuckle under the runner, the mast stays upright in its socket; design gust drag and uplift on the canopy.",
                "de": "Schirm am Knickgelenk unter dem Läufer 20 Grad zur Sonne rechts geneigt, der Mast bleibt senkrecht in der Hülse; Bemessungsböe mit Widerstand und Auftrieb am Schirm.",
            }
    for clip in asset["clips"]:
        if clip["id"] == "wind-flap":
            for key in clip["keys"]:
                if "mast-upper" in key.get("bones", {}):
                    key["bones"]["mast-upper"] = {"angleDeg": -3}
        if clip["id"] == "sun-tilt":
            clip["description"] = {
                "en": "The canopy tilts at the knuckle towards the sun and back, without wind; the gust loads are switched off by their tracks.",
                "de": "Der Schirm neigt sich am Knickgelenk zur Sonne und zurück, ohne Wind; die Böenlasten sind durch ihre Spuren abgeschaltet.",
            }
    for region in asset["thermal"]["regions"]:
        if region["id"] == "base-concrete":
            region["areaM2"] = 0.69
            region["characteristicLengthM"] = 0.5
            region["geometry"] = "horizontal-plate-up"


def shift_upper(solid):
    if "profile" in solid:
        solid["profile"] = [[round(max(p[0] - SHIFT, 0.0), 4), p[1]] for p in solid["profile"]]
    if "transform" in solid and "translation" in solid["transform"]:
        t = solid["transform"]["translation"]
        t[0] = round(t[0] - SHIFT, 4)


def volumes(asset):
    for scale, v in asset["volumes"].items():
        repeat = None
        gone = {"clevis-flange", "clevis-cheek-near", "clevis-cheek-far", "mast-pin", "mast-pin-nut-near", "mast-pin-nut-far", "mast-lug"}
        body = []
        for s in v["body"]:
            if s["id"] == "base-bolts":
                repeat = s["repeat"]
            if s["id"] in gone:
                continue
            if s["id"] == "base-block":
                s["size"] = [SLAB, SLAB_H, SLAB]
                s["transform"]["translation"] = [0, SLAB_H / 2, 0]
            elif s["id"] == "base-plate":
                h = 0.01 if scale == "50" else 0.015
                s["size"] = [0.2, h, 0.2]
                s["transform"]["translation"] = [0, SLAB_H + h / 2, 0]
            elif s["id"] == "base-bolts":
                s["origin"] = [0.075, FLANGE_TOP, 0.075]
            elif s["id"] == "base-socket":
                foot = SLAB_H if scale == "200" else 0.111
                s.clear()
                s.update({"id": "base-socket", "kind": "revolve", "bone": "base", "role": "body", **ALU, "tone": 0.3, "profile": rod(foot, SOCKET_TOP, 0.03 if scale == "100" else 0.035), "axis": [0, 1, 0], "origin": [0, 0, 0]})
            elif s["id"] == "mast-lower":
                s["profile"] = rod(MAST_FOOT, 0.79, 0.024)
            elif s["id"] == "mast":
                r = s["profile"][1][1]
                s["profile"] = rod(-0.164 if scale == "500" else MAST_FOOT, 1.52 if scale != "500" else KNUCKLE, r)
            elif s.get("bone") == "mast-upper" and s["id"] != "mast-knuckle-pin":
                shift_upper(s)
                if scale == "500":
                    s["profile"] = rod(0.0, 0.52, 0.0375)
                    s.pop("slender", None)
            body.append(s)
        at = next(i for i, s in enumerate(body) if s["id"] in ("mast-lower", "mast"))
        new = []
        if scale == "50":
            new += [
                {"id": "base-socket", "kind": "revolve", "bone": "base", "role": "body", **ALU, "tone": 0.3, "profile": tube(FLANGE_TOP, SOCKET_TOP, 0.0245, 0.03), "axis": [0, 1, 0], "origin": [0, 0, 0]},
                {"id": "base-socket-rim", "kind": "revolve", "bone": "base", "role": "detail", **ALU, "tone": 0.25, "profile": tube(SOCKET_TOP - 0.012, SOCKET_TOP, 0.0245, 0.034), "axis": [0, 1, 0], "origin": [0, 0, 0]},
                {"id": "base-gussets", "kind": "extrude", "bone": "base", "role": "detail", **ALU, "tone": 0.3, "plane": "xy", "profile": [[0.028, FLANGE_TOP], [0.09, FLANGE_TOP], [0.09, FLANGE_TOP + 0.008], [0.036, 0.19], [0.028, 0.19]], "from": -0.004, "to": 0.004, "repeat": dict(repeat)},
                {"id": "base-clamp-upper", "kind": "capsule", "bone": "base", "role": "detail", **ALU, "tone": 0.2, "p0": [0.026, 0.34, 0], "p1": [0.066, 0.34, 0], "r0": 0.011, "r1": 0.011},
                {"id": "base-clamp-lower", "kind": "capsule", "bone": "base", "role": "detail", **ALU, "tone": 0.2, "p0": [0.026, 0.24, 0], "p1": [0.066, 0.24, 0], "r0": 0.011, "r1": 0.011},
                {"id": "mast-middle", "kind": "revolve", "bone": "mast", "role": "body", **ALU, "tone": 0.45, "profile": rod(0.78, 1.52, 0.02), "axis": [1, 0, 0], "origin": [0, 0, 0]},
            ]
        if scale in ("50", "100", "200"):
            half = {"50": 0.024, "100": 0.024, "200": 0.026}[scale]
            radius = {"50": 0.028, "100": 0.028, "200": 0.03}[scale]
            new.append({"id": "mast-knuckle", "kind": "revolve", "bone": "mast-upper", "role": "body" if scale == "50" else "detail", **ALU, "tone": 0.3, "profile": rod(-half, half, radius), "axis": [0, 0, 1], "origin": [0, 0, 0]})
        body[at + 1:at + 1] = new
        v["body"] = body

        structure = []
        for s in v["structure"]:
            if s.get("of") in gone:
                continue
            if s["id"] == "mass-base":
                s["transform"]["translation"] = [0, SLAB_H / 2, 0]
                s["radii"] = [min(s["radii"][0], 0.075), min(s["radii"][1], 0.04), min(s["radii"][2], 0.075)]
            elif s["id"] == "support-socket":
                s["size"] = [SLAB, s["size"][1], SLAB]
            elif s["id"] == "joint-pivot":
                s.clear()
                s.update({"id": "joint-pivot", "of": "mast-knuckle", "member": "mast-upper", "part": "joint"})
            elif s["id"] == "member-mast-lower":
                s["profile"] = tube(MAST_FOOT, 0.79, 0.022, 0.024)
            elif s["id"] == "member-mast" and "profile" in s:
                s["profile"] = tube(MAST_FOOT, 1.52, 0.02, 0.022)
            elif s["id"] == "member-mast-upper" and "profile" in s:
                r0, r1 = s["profile"][0][1], s["profile"][2][1]
                s["profile"] = tube(0.0, 0.53, r0, r1)
            elif s["id"] == "mass-mast":
                s["transform"]["translation"] = [0.77, 0, 0]
            elif s["id"] == "mass-mast-upper":
                s["transform"]["translation"] = [0.275, 0, 0]
            structure.append(s)
        if scale == "50":
            structure += [
                {"id": "member-mast-middle", "kind": "revolve", "bone": "mast", "role": "detail", **ALU, "tone": 0.15, "profile": tube(0.78, 1.52, 0.0185, 0.02), "axis": [1, 0, 0], "origin": [0, 0, 0], "member": "mast", "part": "member"},
                {"id": "st-base-socket", "of": "base-socket", "member": "base", "part": "member"},
                {"id": "st-base-gussets", "of": "base-gussets", "member": "base", "part": "member"},
                {"id": "joint-socket-clamp", "of": "base-clamp-upper", "member": "mast", "part": "joint"},
                {"id": "st-mast-knuckle", "of": "mast-knuckle", "member": "mast-upper", "part": "joint"},
            ]
        elif scale == "200":
            structure.append({"id": "st-mast-knuckle", "of": "mast-knuckle", "member": "mast-upper", "part": "joint"})
        v["structure"] = structure

        thermal = [s for s in v["thermal"] if s.get("of") not in gone]
        ids = {s.get("of") for s in thermal}
        for s in new:
            if s["id"] not in ids:
                thermal.append({"id": "th-" + s["id"], "of": s["id"], "region": "frame-alu", "layer": "tube"})
        v["thermal"] = thermal


def rect(shape, x, y, w, h):
    for k in ("points", "d", "closed", "cx", "cy", "rx", "ry"):
        shape.pop(k, None)
    shape.update({"kind": "rect", "x": x, "y": y, "w": w, "h": h})


def drawings(asset):
    socket_side = "M -0.03 0.106 L 0.03 0.106 L 0.03 0.4 L -0.03 0.4 Z M 0.03 0.106 L 0.09 0.106 L 0.09 0.114 L 0.036 0.19 L 0.03 0.19 Z M -0.03 0.106 L -0.09 0.106 L -0.09 0.114 L -0.036 0.19 L -0.03 0.19 Z"
    knobs = " M 0.03 0.329 L 0.077 0.329 L 0.077 0.351 L 0.03 0.351 Z M 0.03 0.229 L 0.077 0.229 L 0.077 0.251 L 0.03 0.251 Z"
    for scale, rep in asset["representations"].items():
        for view in ("elevation", "front"):
            shapes = rep["geometry"][view]
            out = []
            for s in shapes:
                i = s["id"]
                if i == "base-block":
                    s.update({"x": -SLAB / 2, "w": SLAB, "h": SLAB_H})
                elif i == "base-seam":
                    s["points"] = [[-0.238, 0.084], [0.238, 0.084]]
                elif i == "base-grit":
                    s["d"] = "M -0.19 0.03 L -0.16 0.04 M 0.07 0.025 L 0.11 0.035 M -0.05 0.06 L -0.01 0.05 M 0.16 0.055 L 0.2 0.045"
                elif i == "base-plate":
                    h = 0.01 if scale == "50" else 0.015
                    s.update({"x": -0.1, "y": SLAB_H, "w": 0.2, "h": h})
                elif i == "base-bolts":
                    s["d"] = "M -0.083 0.106 L -0.067 0.106 L -0.067 0.1155 L -0.083 0.1155 Z M 0.067 0.106 L 0.083 0.106 L 0.083 0.1155 L 0.067 0.1155 Z"
                elif i == "base-socket":
                    if scale == "50":
                        s.pop("x", None), s.pop("w", None), s.pop("h", None), s.pop("y", None)
                        s.update({"kind": "path", "d": socket_side + (knobs if view == "elevation" else "")})
                    else:
                        half = 0.03 if scale == "100" else 0.035
                        foot = SLAB_H if scale == "200" else 0.111
                        rect(s, -half, foot, 2 * half, SOCKET_TOP - foot)
                elif i == "mast" and s["bone"] == "mast":
                    if scale == "500":
                        s["points"] = [[-0.164, 0], [KNUCKLE, 0]] if view == "elevation" else [[0, SLAB_H], [0, 1.8]]
                    else:
                        lo = 0.024 if scale == "50" else 0.022
                        hi = 0.02 if scale == "50" else 0.022
                        if view == "elevation":
                            s["points"] = [[MAST_FOOT, -lo], [0.81, -lo], [0.81, -hi], [KNUCKLE, -hi], [KNUCKLE, hi], [0.81, hi], [0.81, lo], [MAST_FOOT, lo]]
                        else:
                            s["points"] = [[-lo, 0.11], [lo, 0.11], [lo, 1.07], [hi, 1.07], [hi, 1.8], [-hi, 1.8], [-hi, 1.07], [-lo, 1.07]]
                elif i == "mast-upper":
                    if view == "elevation":
                        s["points"] = [[p[0] if p[0] == 0 else HEAD, p[1]] for p in s["points"]]
                    else:
                        s["points"] = [[p[0], 1.8 if p[1] == 1.07 else p[1]] for p in s["points"]]
                elif i == "mast-knuckle":
                    continue
                out.append(s)
            if scale in ("50", "100", "200"):
                r = 0.03 if scale == "200" else 0.028
                half = 0.026 if scale == "200" else 0.024
                knuckle = {"id": "mast-knuckle", "bone": "mast-upper", "role": "detail", "z": 2.2, "filled": True, "tone": 0.3, "region": "frame-alu", "material": "aluminium-anodised"}
                if view == "elevation":
                    knuckle.update({"kind": "ellipse", "cx": 0, "cy": 0, "rx": r, "ry": r})
                else:
                    knuckle.update({"kind": "rect", "x": -half, "y": 1.8 - r, "w": 2 * half, "h": 2 * r})
                at = next(k for k, s in enumerate(out) if s["id"] == "mast-upper")
                out.insert(at + 1, knuckle)
            rep["geometry"][view] = out


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    asset = document["asset"]
    if asset["rig"]["bones"][1]["lengthM"] == KNUCKLE:
        sys.exit("already applied")
    rig(asset)
    volumes(asset)
    drawings(asset)
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    if "--publish" in sys.argv:
        json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        scope = {}
        exec(open(os.path.join(T, "volumes-insert.py"), encoding="utf-8").read().split("\ndef member_end")[0], scope)
        open(CATALOG, "w", encoding="utf-8").write(scope["dump"](document, 0) + "\n")
    print("written", "catalog + source" if "--publish" in sys.argv else "candidate")


main()
