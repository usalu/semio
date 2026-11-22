"""🍖 Charcoal grill, author pass 1: the lid top as on the real kettle — grip on the crown on two straps, vent dial 24 degrees off the crown toward the wheels.

Usage: python3 rework-charcoal-grill-lid-top.py [--source] [--catalog]
Without flags it only writes the candidate document to 🗑️generated/r3-charcoal-grill/🔣️.json.
All numbers are lid-bone coordinates (x from the rear rim point, y up from the rim plane, z lateral) and are chosen by hand.
It starts from the state of 2026-11-22 00:50 (grip in front of the rim, dial on the crown) and refuses to run twice.
"""
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/objects/charcoal-grill.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/charcoal-grill/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated/r3-charcoal-grill/🔣️.json")

CX, CY, RHO = 0.2925, -0.05246, 0.29446
CROWN_Y = CY + RHO
VENT_DEG = 24.0
FULL_DEG, FADE_DEG = 10.0, 72.0
GRIP_Y, GRIP_RX, GRIP_RY, GRIP_HALF = 0.281, 0.015, 0.011, 0.08
GRIP_TOP = GRIP_Y + GRIP_RY
STRAP_Z, STRAP_W, STRAP_Y0 = 0.058, 0.018, 0.232
STRAP_T = {"50": 0.008, "100": 0.015}
POST_200 = 0.03
BLOCK_500 = (0.08, 0.208, 0.16)
FIN = {"50": (0.011, 0.008), "100": (0.015, 0.015)}
INNER_INSET = {"500": 0.0015}
LID_COM = (0.287, 0.115)


def scalar(value):
    return not isinstance(value, (dict, list))


def inline(value):
    if isinstance(value, list):
        return all(scalar(item) for item in value) or (
            all(isinstance(item, list) and all(scalar(leaf) for leaf in item) for item in value) and len(json.dumps(value)) <= 160
        )
    return False


def dump(value, depth):
    """🧵 The catalog layout of `volumes-insert.py`, so untouched members stay byte-identical."""
    pad, inner = "  " * depth, "  " * (depth + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [inner + json.dumps(key, ensure_ascii=False) + ": " + dump(item, depth + 1) for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        if inline(value):
            return json.dumps(value, ensure_ascii=False)
        return "[\n" + ",\n".join(inner + dump(item, depth + 1) for item in value) + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


def r5(x):
    return round(x, 5)


def weight(polar_deg):
    if polar_deg <= FULL_DEG:
        return 1.0
    if polar_deg >= FADE_DEG:
        return 0.0
    s = (FADE_DEG - polar_deg) / (FADE_DEG - FULL_DEG)
    return s * s * (3 - 2 * s)


def slide(vertex, inset=0.0):
    """🌐 Turns a dome point about the lateral axis through the sphere centre, fully under the dial and fading to nothing toward the rim, so the three vent holes travel with the dial and both shell faces stay on their spheres."""
    dx, dy, dz = vertex[0] - CX, vertex[1] - CY, vertex[2]
    radius = math.sqrt(dx * dx + dy * dy + dz * dz)
    if radius < 0.2 or dy <= 0:
        return vertex
    polar = math.degrees(math.acos(max(-1.0, min(1.0, dy / radius))))
    a = math.radians(VENT_DEG * weight(polar))
    if a == 0:
        return vertex
    k = 1 - inset * min(1.0, (FADE_DEG - polar) / 10) / radius if radius < RHO - 0.0004 else 1.0
    return [round(CX + k * (dx * math.cos(a) - dy * math.sin(a)), 7), round(CY + k * (dx * math.sin(a) + dy * math.cos(a)), 7), round(k * vertex[2], 7)]


def box(sid, material, role, tone, size, at, corner):
    return {"id": sid, "kind": "box", "bone": "lid", "material": material, "role": role, "tone": tone, "size": [r5(v) for v in size], "cornerRadius": corner, "transform": {"translation": [r5(v) for v in at]}}


def grip(stations, ry_scale):
    sections = [{"at": [CX, GRIP_Y, z], "rx": rx, "ry": r5(ry * ry_scale), "exponent": 2.6} for z, rx, ry in stations]
    return {"id": "lid-handle", "kind": "loft", "bone": "lid", "material": "wood-handle", "role": "body", "tone": 0.55, "sections": sections, "capStart": True, "capEnd": True}


def strap(side, sign, thickness):
    height = GRIP_Y - STRAP_Y0
    return box("lid-bracket-" + side, "steel-black", "detail", 0.25, (STRAP_W, height, thickness), (CX, STRAP_Y0 + height / 2, sign * STRAP_Z), min(0.003, thickness / 3))


def lid_top(scale):
    scale_ry = GRIP_RY / 0.0132
    if scale == "50":
        stations = [(-0.08, 0.008, 0.007), (-0.076, 0.0118, 0.0102), (-0.068, 0.0145, 0.0125), (-0.04, 0.015, 0.013), (0, 0.0152, 0.0132), (0.04, 0.015, 0.013), (0.068, 0.0145, 0.0125), (0.076, 0.0118, 0.0102), (0.08, 0.008, 0.007)]
        return [grip(stations, scale_ry), strap("left", 1, STRAP_T["50"]), strap("right", -1, STRAP_T["50"])]
    if scale == "100":
        stations = [(-0.08, 0.008, 0.007), (-0.068, 0.0145, 0.0125), (0, 0.0152, 0.0132), (0.068, 0.0145, 0.0125), (0.08, 0.008, 0.007)]
        return [grip(stations, scale_ry), strap("left", 1, STRAP_T["100"]), strap("right", -1, STRAP_T["100"])]
    if scale == "200":
        bar = box("lid-handle", "wood-handle", "body", 0.55, (POST_200, POST_200, 2 * GRIP_HALF), (CX, GRIP_TOP - POST_200 / 2, 0), 0.008)
        height = GRIP_TOP - POST_200 / 2 - STRAP_Y0
        posts = [box("lid-bracket-" + side, "steel-black", "detail", 0.25, (POST_200, height, POST_200), (CX, STRAP_Y0 + height / 2, sign * 0.055), 0.004) for side, sign in (("left", 1), ("right", -1))]
        return [bar] + posts
    width, y0, length = BLOCK_500
    return [box("lid-handle", "wood-handle", "body", 0.55, (width, GRIP_TOP - y0, length), (CX, (GRIP_TOP + y0) / 2, 0), 0.012)]


def rework_volumes(volumes):
    for scale, v in volumes.items():
        old = next(s for s in v["body"] if s["id"] == "lid-handle")
        if old.get("kind") == "box" or abs(old["sections"][0]["at"][0] - 0.625) > 1e-6:
            raise SystemExit("already applied at 1:" + scale)
        for name in ("body", "thermal"):
            for solid in v[name]:
                if solid.get("kind") == "polyhedron" and solid.get("bone") == "lid":
                    solid["vertices"] = [slide(p, INNER_INSET.get(scale, 0.0)) for p in solid["vertices"]]
        new = lid_top(scale)
        ids = {s["id"] for s in new}
        drop = {"lid-handle", "lid-bracket-left", "lid-bracket-right"}
        at = next(i for i, s in enumerate(v["body"]) if s["id"] == "lid-handle")
        v["body"] = [s for s in v["body"][:at] if s["id"] not in drop] + new + [s for s in v["body"][at:] if s["id"] not in drop]
        gone = drop - ids
        if scale in FIN:
            fin = next(s for s in v["body"] if s["id"] == "vent-fin")
            fin["size"], fin["transform"]["translation"] = [FIN[scale][0], 0.07, FIN[scale][1]], [r5(0.007 + FIN[scale][0] / 2), 0, 0]
        else:
            v["body"] = [s for s in v["body"] if s["id"] != "vent-fin"]
            gone.add("vent-fin")
        for name in ("thermal", "structure"):
            v[name] = [s for s in v[name] if s.get("of") not in gone]
        for solid in v["structure"]:
            if solid["id"] == "mass-lid":
                solid["transform"]["translation"] = [LID_COM[0], LID_COM[1], 0]
        coarse(scale, v)
        v["decisions"] = [d for d in v["decisions"] if "crown" not in d["en"]]
        for d in v["decisions"]:
            d["en"] = d["en"].replace("one front leg on a plastic foot under the lid handle", "one front leg on a plastic foot").replace("the wooden lid handle on two flat brackets, the 80 mm vent dial", "the wooden lid handle on the crown of the dome, the 80 mm vent dial beside it")
            d["de"] = d["de"].replace("ein vorderes Bein auf Kunststofffuß unter dem Deckelgriff", "ein vorderes Bein auf Kunststofffuß").replace("der hölzerne Deckelgriff auf zwei Flachhaltern, die Lüftungsscheibe von 80 mm", "der hölzerne Deckelgriff auf dem Scheitel der Kuppel, die Lüftungsscheibe von 80 mm daneben")
        v["decisions"].append(LID_TEXT[scale])
        if scale in COARSE_TEXT:
            v["decisions"].append(COARSE_TEXT[scale])


RESTS = ["grate-rest-1", "grate-rest-2", "grate-rest-3", "fuel-rest-1", "fuel-rest-2", "fuel-rest-3"]
HANGERS = ["pan-hanger-front", "pan-hanger-left", "pan-hanger-right"]
SIDE_HANDLES = ["side-handle-left", "side-handle-right"]
SIDE_BRACKETS = ["side-bracket-left-a", "side-bracket-left-b", "side-bracket-right-a", "side-bracket-right-b"]
BOWL_RHO = 0.2904
DAMPER_R = 0.06
COARSE = {
    "100": {"thick": {**{k: 0.012 for k in RESTS + HANGERS}, "damper-lever": 0.015}, "drop": ["vent-skirt"], "plate": 0.0045, "dial": None},
    "200": {"thick": {**{k: 0.03 for k in HANGERS + SIDE_HANDLES}, "damper-lever": 0.03}, "drop": RESTS + SIDE_BRACKETS + ["damper-knob", "vent-skirt"], "plate": 0.0045, "dial": (-0.003, 0.022), "bracket": 0.03},
    "500": {"thick": {k: 0.034 for k in HANGERS + SIDE_HANDLES}, "drop": RESTS + SIDE_BRACKETS + ["damper-knob", "damper-lever", "vent-skirt"], "plate": 0.0045, "dial": (0.0, 0.003), "bracket": 0.034, "bed": 1.9},
}


def thicken(solid, t):
    """📏 Brings every thickness of a rod, bar or block up to the feature of the scale and leaves its axis and length alone."""
    h = t / 2
    if solid["kind"] == "capsule":
        solid["r0"], solid["r1"] = max(solid["r0"], h), max(solid["r1"], h)
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["rx"], section["ry"] = max(section["rx"], h), max(section["ry"], h)
    elif solid["kind"] == "box":
        solid["size"] = [max(v, t) for v in solid["size"]]
        if "cornerRadius" in solid:
            solid["cornerRadius"] = min(solid["cornerRadius"], t / 3)
    elif solid["kind"] == "ellipsoid":
        solid["radii"] = [max(v, h) for v in solid["radii"]]
    else:
        raise SystemExit("cannot thicken " + solid["id"])


def damper_plate(old, t):
    """🥏 The bottom damper as the flat 4.5 mm sheet disc it is, touching the bowl at its lowest point."""
    profile = [[0, 0], [0, DAMPER_R], [t, DAMPER_R], [t, 0]]
    return {"id": old["id"], "kind": "revolve", "bone": old["bone"], "material": old["material"], "role": old["role"], "tone": old["tone"], "profile": profile, "axis": [1, 0, 0], "origin": [0, 0, 0]}


def dial_cap(old, a0, a1):
    return {"id": old["id"], "kind": "revolve", "bone": old["bone"], "material": old["material"], "role": old["role"], "tone": old["tone"], "profile": [[a0, 0], [a0, 0.04], [a1, 0.04], [a1, 0]] if a1 - a0 < 0.01 else [[a0, 0], [a0, 0.04], [a1 - 0.004, 0.04], [a1, 0.036], [a1, 0]], "axis": [1, 0, 0], "origin": [0, 0, 0]}


def coarse(scale, v):
    """🔭 The coarser lists modelled for their own feature: sub-feature fittings are thickened to the feature where they stay readable, merged into one block where they do not, and left out where they are hidden inside the bowl."""
    plan = COARSE.get(scale)
    if plan is None:
        return
    body = v["body"]
    for solid in body:
        if solid["id"] in plan["thick"]:
            thicken(solid, plan["thick"][solid["id"]])
        if solid["id"] == "damper-plate":
            body[body.index(solid)] = damper_plate(solid, plan["plate"])
        if solid["id"] == "vent-dial":
            if plan["dial"] is None:
                solid["from"], solid["to"] = -0.003, 0.012
            else:
                body[body.index(solid)] = dial_cap(solid, *plan["dial"])
        if solid["id"] == "charcoal-bed" and "bed" in plan:
            solid["profile"] = [[r5(a * plan["bed"]), r] for a, r in solid["profile"]] if solid["axis"][1] == 0 else solid["profile"]
            if solid["axis"][1] != 0:
                low = min(a for a, r in solid["profile"])
                solid["profile"] = [[r5(low + (a - low) * plan["bed"]), r] for a, r in solid["profile"]]
    drop = set(plan["drop"])
    if "bracket" in plan:
        t = plan["bracket"]
        for side, sign in (("left", 1), ("right", -1)):
            at = next(i for i, s in enumerate(body) if s["id"] == "side-bracket-" + side + "-a")
            block = box("side-bracket-" + side + "-a", "steel-black", "detail", 0.25, (0.09, t, t), (0, 0.705, sign * 0.3), 0.005)
            block["bone"] = "bowl"
            body[at] = block
        drop -= {"side-bracket-left-a", "side-bracket-right-a"}
    v["body"] = [s for s in body if s["id"] not in drop]
    for name in ("thermal", "structure"):
        v[name] = [s for s in v[name] if s.get("of") not in drop]


COARSE_TEXT = {
    "100": {
        "en": "Fittings at the 15 mm feature: grate rests and pan hangers are 12 mm (the thinnest stroke drawn on the bowl is 10 mm), the damper is its flat 4.5 mm sheet disc of 120 mm (a sheet, not a lump) with a 15 mm lever, the vent dial is one 15 mm plate seated in the dome with its three holes.",
        "de": "Beschläge für das 15-mm-Merkmal: Rostauflagen und Wannenhalter sind 12 mm stark (der dünnste am Kessel gezeichnete Strich ist 10 mm), der Schieber ist seine flache Blechscheibe von 4,5 mm und 120 mm Durchmesser (ein Blech, kein Klotz) mit einem Hebel von 15 mm, die Lüftungsscheibe ist eine in die Kuppel eingelassene Platte von 15 mm mit ihren drei Löchern.",
    },
    "200": {
        "en": "Fittings at the 30 mm feature: the side handles are 30 mm bars on one 30 mm block each, the pan hangs on three 30 mm arms, the damper is its flat 4.5 mm sheet disc with a 30 mm lever, the vent dial a 25 mm cap; the rests inside the bowl, the knob and the vent holes are left out.",
        "de": "Beschläge für das 30-mm-Merkmal: Die Seitengriffe sind Stäbe von 30 mm auf je einem Block von 30 mm, die Wanne hängt an drei Armen von 30 mm, der Schieber ist seine flache Blechscheibe von 4,5 mm mit einem Hebel von 30 mm, die Lüftungsscheibe eine Kappe von 25 mm; die Auflagen im Kessel, der Knauf und die Lüftungslöcher entfallen.",
    },
    "500": {
        "en": "Fittings at the 75 mm feature: side handles and their blocks are 34 mm (the thinnest stroke drawn on the bowl is 33 mm), the pan hangs on three 34 mm arms, the damper is its flat 4.5 mm sheet disc of 120 mm, the vent dial its 3 mm sheet disc of 80 mm on the dome, the charcoal a bed 76 mm deep; rests, lever, knob and vent holes are left out.",
        "de": "Beschläge für das 75-mm-Merkmal: Seitengriffe und ihre Blöcke sind 34 mm stark (der dünnste am Kessel gezeichnete Strich ist 33 mm), die Wanne hängt an drei Armen von 34 mm, der Schieber ist seine flache Blechscheibe von 4,5 mm und 120 mm Durchmesser, die Lüftungsscheibe ihre Blechscheibe von 3 mm und 80 mm auf der Kuppel, die Kohle ein 76 mm tiefes Bett; Auflagen, Hebel, Knauf und Lüftungslöcher entfallen.",
    },
}

LID_TEXT = {
    "50": {
        "en": "Lid top as on the real kettle: the wooden grip (160 x 30 x 22 mm) lies across the crown of the dome, 28 mm clear of the enamel, on two 18 x 8 mm steel straps 116 mm apart that end inside the grip and inside the shell. The vent dial sits 24 degrees (123 mm along the dome) off the crown toward the wheels and is tilted with the dome; the three 12 mm holes of the shell lie under the holes of the dial. The grip is the highest point at 1.03 m.",
        "de": "Deckeloberseite wie am echten Kugelgrill: Der Holzgriff (160 x 30 x 22 mm) liegt quer über dem Scheitel der Kuppel, 28 mm über der Emaille, auf zwei Stahllaschen von 18 x 8 mm im Abstand von 116 mm, die im Griff und in der Schale enden. Die Lüftungsscheibe sitzt 24 Grad (123 mm auf der Kuppel) neben dem Scheitel zur Radseite hin und ist mit der Kuppel geneigt; die drei 12-mm-Löcher der Schale liegen unter den Löchern der Scheibe. Der Griff ist mit 1,03 m der höchste Punkt.",
    },
    "100": {
        "en": "Lid top at the 15 mm feature: the wooden grip across the crown with the same outline in five stations, its two straps thickened to 15 mm, the vent dial 24 degrees off the crown toward the wheels with a 15 mm grip fin.",
        "de": "Deckeloberseite für das 15-mm-Merkmal: der Holzgriff quer über dem Scheitel mit gleichem Umriss in fünf Stationen, seine beiden Laschen auf 15 mm verstärkt, die Lüftungsscheibe 24 Grad neben dem Scheitel zur Radseite hin mit einer Griffrippe von 15 mm.",
    },
    "200": {
        "en": "Lid top at the 30 mm feature: the grip is a 30 mm bar of 160 mm across the crown on two 30 mm posts, the vent dial a cap 24 degrees off the crown toward the wheels without the fin.",
        "de": "Deckeloberseite für das 30-mm-Merkmal: Der Griff ist ein 30-mm-Stab von 160 mm quer über dem Scheitel auf zwei Pfosten von 30 mm, die Lüftungsscheibe eine Kappe 24 Grad neben dem Scheitel zur Radseite hin ohne Rippe.",
    },
    "500": {
        "en": "Lid top at the 75 mm feature: grip and straps are one block of 80 x 84 x 160 mm on the crown, the vent dial is its 80 mm cap without the fin, 24 degrees off the crown toward the wheels.",
        "de": "Deckeloberseite für das 75-mm-Merkmal: Griff und Laschen sind ein Block von 80 x 84 x 160 mm auf dem Scheitel, die Lüftungsscheibe ist ihre Kappe von 80 mm ohne Rippe, 24 Grad neben dem Scheitel zur Radseite hin.",
    },
}


def oval(cx, cy, rx, ry, n=16, exponent=2.6):
    points = []
    for k in range(n):
        a = 2 * math.pi * k / n
        c, s = math.cos(a), math.sin(a)
        points.append([r5(cx + rx * math.copysign(abs(c) ** (2 / exponent), c)), r5(cy + ry * math.copysign(abs(s) ** (2 / exponent), s))])
    return points


def rounded(x0, y0, x1, y1, r):
    k = r * 0.2929
    return [[r5(a), r5(b)] for a, b in ((x1, y1 - r), (x1 - k, y1 - k), (x1 - r, y1), (x0 + r, y1), (x0 + k, y1 - k), (x0, y1 - r), (x0, y0 + r), (x0 + k, y0 + k), (x0 + r, y0), (x1 - r, y0), (x1 - k, y0 + k), (x1, y0 + r))]


def rework_drawings(asset):
    half = STRAP_W / 2
    for scale, rep in asset["representations"].items():
        if scale in ("50", "100"):
            handle = oval(CX, GRIP_Y, GRIP_RX, GRIP_RY)
            bracket = rounded(CX - half, STRAP_Y0 + 0.004, CX + half, GRIP_Y, 0.002)
            t = STRAP_T[scale] / 2
            spans = {"handle-bracket-l": (STRAP_Z - t, STRAP_Z + t), "handle-bracket-r": (-STRAP_Z - t, -STRAP_Z + t)}
        elif scale == "200":
            handle = rounded(CX - POST_200 / 2, STRAP_Y0 + 0.004, CX + POST_200 / 2, GRIP_TOP, 0.008)
            bracket, spans = None, {}
        else:
            width, y0, _ = BLOCK_500
            handle = rounded(CX - width / 2, y0 + 0.004, CX + width / 2, GRIP_TOP, 0.012)
            bracket, spans = None, {}
        for view, shapes in rep["geometry"].items():
            for shape in shapes:
                if shape["id"] == "handle":
                    shape["points"] = handle
                elif shape["id"].startswith("handle-bracket"):
                    shape["points"] = bracket
                    if "solid" in shape:
                        shape["solid"]["z0"], shape["solid"]["z1"] = (r5(v) for v in spans[shape["id"]])
        d = rep["decisions"]
        d["silhouette"]["en"] = d["silhouette"]["en"].replace("the wooden lid handle on two rod brackets", "the wooden lid handle across the crown on two straps, the vent dial beside it toward the wheels").replace("the lid handle on its brackets", "the lid handle on its straps on the crown")
        d["silhouette"]["de"] = d["silhouette"]["de"].replace("der hölzerne Deckelgriff auf zwei Rundstabhaltern", "der hölzerne Deckelgriff quer über dem Scheitel auf zwei Laschen, die Lüftungsscheibe daneben zur Radseite hin")


def rework_roots(asset):
    a = math.radians(VENT_DEG)
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "lid-vent":
            bone["x"], bone["y"], bone["angleDeg"] = r5(CX - RHO * math.sin(a)), r5(CY + RHO * math.cos(a)), 90 + VENT_DEG
            bone["title"] = {"en": "Lid vent dial (turns about the dome normal, 24 degrees off the crown)", "de": "Deckel-Lüftungsscheibe (dreht um die Kuppelnormale, 24 Grad neben dem Scheitel)"}
        if bone["id"] == "lid":
            bone["mass"]["comX"], bone["mass"]["comY"] = LID_COM
    asset["dimensions"]["heightM"], asset["dimensions"]["widthM"] = 1.03, 0.6
    rework_drawings(asset)


def loose(value):
    """🔎 A pattern that finds a JSON value in the file whatever its line layout is."""
    return r"\s*".join(re.escape(token) for token in re.findall(r'"(?:[^"\\\\]|\\\\.)*"|[\[\]{},:]|[^\s\[\]{},:"]+', json.dumps(value, ensure_ascii=False)))


def splice(text, before, after):
    """✂️ Writes only the changed values into the catalog text: `dimensions` and the two bones in their own layout, the changed 2D point lists, bracket spans and decision sentences in place."""
    for key in ("dimensions",):
        old = "\n    " + json.dumps(key) + ": " + dump(before[key], 2)
        if text.count(old) != 1:
            raise SystemExit("member layout of " + key + " is not the expected one")
        text = text.replace(old, "\n    " + json.dumps(key) + ": " + dump(after[key], 2))
    for old_bone, new_bone in zip(before["rig"]["bones"], after["rig"]["bones"]):
        if old_bone != new_bone:
            old = dump(old_bone, 4)
            if text.count(old) != 1:
                raise SystemExit("bone layout of " + old_bone["id"] + " is not the expected one")
            text = text.replace(old, dump(new_bone, 4))
    start = text.index('\n    "representations"')
    end = text.index('\n    "symbols"')
    head, body, tail = text[:start], text[start:end], text[end:]
    for scale, rep in before["representations"].items():
        new_rep = after["representations"][scale]
        at = body.index('\n      ' + json.dumps(scale) + ': {')
        later = [body.index('\n      ' + json.dumps(other) + ': {') for other in before["representations"]]
        stop = min([i for i in later if i > at] + [len(body)])
        part = body[at:stop]
        for view, shapes in rep["geometry"].items():
            for old_shape, new_shape in zip(shapes, new_rep["geometry"][view]):
                if old_shape == new_shape:
                    continue
                part, n = re.subn(loose(old_shape["points"]), lambda m: json.dumps(new_shape["points"]), part)
                if "solid" in old_shape and old_shape["solid"] != new_shape["solid"]:
                    part, m = re.subn(loose(old_shape["solid"]), lambda m: json.dumps(new_shape["solid"], ensure_ascii=False), part)
        for key in ("en", "de"):
            old_text, new_text = json.dumps(rep["decisions"]["silhouette"][key], ensure_ascii=False), json.dumps(new_rep["decisions"]["silhouette"][key], ensure_ascii=False)
            if old_text != new_text:
                part = part.replace(old_text, new_text)
        body = body[:at] + part + body[stop:]
    return head + body + tail


def main():
    source = json.load(open(SOURCE, encoding="utf-8"))
    rework_volumes(source["volumes"])
    text = open(CATALOG, encoding="utf-8").read()
    print("layout identical", dump(json.loads(text), 0) + "\n" == text or dump(json.loads(text), 0) == text)
    document = json.loads(text)
    rework_roots(document["asset"])
    document["asset"]["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(dump(document, 0) + "\n")
    print("candidate", CANDIDATE)
    if "--source" in sys.argv:
        json.dump(source, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        print("source written")
    if "--catalog" in sys.argv:
        text = open(CATALOG, encoding="utf-8").read()
        before = json.loads(text)
        after = json.loads(text)
        rework_roots(after["asset"])
        text = splice(text, before["asset"], after["asset"])
        if json.loads(text) != after:
            raise SystemExit("splice does not reproduce the reworked document; nothing written")
        open(CATALOG, "w", encoding="utf-8").write(text)
        print("catalog roots written")


main()
