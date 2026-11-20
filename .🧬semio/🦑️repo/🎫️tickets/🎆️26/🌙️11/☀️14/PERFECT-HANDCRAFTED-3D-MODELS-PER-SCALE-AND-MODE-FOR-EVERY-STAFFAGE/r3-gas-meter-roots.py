"""🧯 Round 3, appliances/gas-meter: the root fixes outside `volumes`.

The asset was a 0.24 x 0.27 x 0.17 m box of "die-cast aluminium with a bead seam" carrying 110 mm stubs and 1.95 kg: the size of a 2 litre meter with the connections and mass of the compact one. It becomes the compact G4 meter of the Elster BK-G4 data sheet (110 mm spacing, 193 x 221 x 157 mm, 1.9 kg, folded sheet-steel housing, 1.2 litres), 0.245 m high with the unions. Dimensions, materials, rig (pointer bone along its needle), anchors (the meter hangs on its unions), poses, clips, thermal regions (hidden measuring unit), statics text and the 2D drawings of all scales and views are rewritten by hand to the body of `r3-gas-meter-model.py` (ruling E1).

Usage: python3 r3-gas-meter-roots.py            builds the candidate 🗑️generated/r3-gas-meter/🔣️.json (volumes from the source) and validates it
       python3 r3-gas-meter-roots.py --apply    also writes the changed members into the catalog file and the manifest entry
"""
import copy
import importlib.util
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAFFAGE = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"))
CATALOG = os.path.join(STAFFAGE, "🗂️catalog/🔌️appliances/gas-meter/🔣️.json")
MANIFEST = os.path.join(STAFFAGE, "🗂️catalog/🔣️.json")
SOURCE = os.path.join(HERE, "volumes/appliances/gas-meter.json")
OUT = os.path.join(HERE, "🗑️generated/r3-gas-meter")
CANDIDATE = os.path.join(OUT, "🔣️.json")
NS = {}
exec(open(os.path.join(HERE, "volumes-insert.py"), encoding="utf-8").read().split("def main")[0], NS)
spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-gas-meter-kit.py"))
K = importlib.util.module_from_spec(spec)
spec.loader.exec_module(K)
TEXTS = json.load(open(os.path.join(HERE, "r3-gas-meter-roots-texts.json"), encoding="utf-8"))

ST, ABS, PIPE = "sheet-steel", "abs-plastic", "steel-pipe"
HEIGHT, WIDTH, DEPTH = 0.245, 0.193, 0.157
AREA = {"shell": 0.2, "index": 0.034, "pipes": 0.014, "measuring-unit": 0.147}


def n6(v):
    """🔢 A coordinate as the drawings carry it."""
    return round(v + 0.0, 4)


def common(sid, role, z, tone, bone, filled):
    """🏷️ The leading members of a shape in the order of the catalog."""
    return {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}


def bind(s, region, material, regions=None):
    """🔗 The trailing region and material binding of a shape."""
    if regions:
        s["regions"] = regions
    elif region:
        s["region"] = region
    if material:
        s["material"] = material
    return s


def rect(sid, role, z, tone, x, y, w, h, region, material, r=None, bone="body", filled=True):
    """▭ A rectangle."""
    s = common(sid, role, z, tone, bone, filled)
    s.update({"kind": "rect", "x": n6(x), "y": n6(y), "w": n6(w), "h": n6(h)})
    if r:
        s["r"] = r
    return bind(s, region, material)


def poly(sid, role, z, tone, points, region, material, closed=True, bone="body", filled=True, regions=None):
    """⬠ A polygon or an open stroke."""
    s = common(sid, role, z, tone, bone, filled)
    s.update({"kind": "poly", "points": [[n6(a), n6(b)] for a, b in points], "closed": closed})
    return bind(s, region, material, regions)


def ellipse(sid, role, z, tone, cx, cy, rx, region, material, bone="body"):
    """⬭ A circle."""
    s = common(sid, role, z, tone, bone, True)
    s.update({"kind": "ellipse", "cx": n6(cx), "cy": n6(cy), "rx": n6(rx), "ry": n6(rx)})
    return bind(s, region, material)


def shell_outline(half, upper, steps):
    """🫙 The true outline of one pressed shell seen square on: seam edge, drafted wall, quarter-circle shoulder, flat crown."""
    y_s, sign = (K.YST, 1) if upper else (K.YSB, -1)
    right = [(half, 0.0), (half - K.DRAFT, y_s)] + [(half - K.DRAFT - i, y) for y, i in K.shoulder(y_s, sign, steps)]
    pts = right + [(-x, y) for x, y in reversed(right)]
    return pts if upper else list(reversed(pts))


def housing_outline(half, steps):
    """🧴 The outline of the merged housing of 1:100: straight walls at the bead width, shoulders in `steps` chords."""
    top = [(half - i, y) for y, i in K.shoulder(K.YST, 1, steps)]
    low = [(half - i, y) for y, i in K.shoulder(K.YSB, -1, steps)]
    right = list(reversed(low)) + [(half, K.YSB), (half, K.YST)] + top
    return right + [(-x, y) for x, y in reversed(right)]


def stub_outline(x0, y0=0.093):
    """🚰 A stub with its brazed boss, from inside the crown to the sealing face."""
    b, s = K.BOSS_R, K.STUB_R
    return [(x0 - b, y0), (x0 + b, y0), (x0 + b, 0.100), (x0 + s, 0.100), (x0 + s, K.Y_STUB_TOP), (x0 - s, K.Y_STUB_TOP), (x0 - s, 0.100), (x0 - b, 0.100)]


def plan_outline(rx, rz, count):
    """🥚 The plan outline of a pressing as (x, z) points."""
    return [(x, z) for z, x in K.superellipse(rx, rz, count=count)]


def rounded(x0, y0, x1, y1, r, n=3):
    """▢ A rounded rectangle as points, counter-clockwise from the lower right corner."""
    return K.rrect(x0, y0, x1, y1, r, n)


def needle_points(width, tail, tip):
    """🧭 The pointer blade along its bone."""
    return [(-tail, -width / 2), (tip, -0.001), (tip, 0.001), (-tail, width / 2)]


def elevation50():
    """📐 1:50 elevation: the meter face."""
    bw, _, bh = K.BEAD
    cw, ch, cy = K.COUNTER
    dw, dh, pitch, x0 = K.DIGIT
    out = []
    for k, x in ((1, -K.XS), (2, K.XS)):
        out += [
            poly(f"stub-{k}", "body", 2.9, 0.5, stub_outline(x), "pipes", PIPE),
            rect(f"union-{k}", "body", 3, 0.4, x - K.NUT_R, K.NUT_Y0, 2 * K.NUT_R, K.NUT_Y1 - K.NUT_Y0, "pipes", PIPE, r=0.0015),
            poly(f"union-{k}-edge-a", "detail", 3.05, 0.15, [(x - K.NUT_R / 2, K.NUT_Y0), (x - K.NUT_R / 2, K.NUT_Y1)], "pipes", PIPE, closed=False, filled=False),
            poly(f"union-{k}-edge-b", "detail", 3.05, 0.15, [(x + K.NUT_R / 2, K.NUT_Y0), (x + K.NUT_R / 2, K.NUT_Y1)], "pipes", PIPE, closed=False, filled=False),
            rect(f"tail-{k}", "body", 2.95, 0.5, x - K.TAIL_R, K.NUT_Y1, 2 * K.TAIL_R, K.Y_TOP - K.NUT_Y1, "pipes", PIPE),
        ]
    out += [
        poly("lower-shell", "body", 3, 0.72, shell_outline(K.A, False, 3), "shell", ST),
        poly("upper-shell", "body", 3.01, 0.76, shell_outline(K.A, True, 3), "shell", ST),
        rect("seam", "detail", 3.1, 0.5, -bw, -bh, 2 * bw, 2 * bh, "shell", ST, r=0.002),
        rect("index", "body", 3.2, 0.9, -K.IX, K.IY0, 2 * K.IX, K.IY1 - K.IY0, "index", ABS, r=K.IR),
        rect("index-window", "detail", 3.22, 0.3, -K.WIN[0], K.WIN[1], 2 * K.WIN[0], K.WIN[2] - K.WIN[1], "index", ABS, r=0.003, filled=False),
        rect("counter", "body", 3.25, 0.1, -cw / 2, cy - ch / 2, cw, ch, "index", ABS, r=0.001),
    ]
    for k in range(8):
        out.append(rect(f"digit-{k + 1}", "detail", 3.3, 0.92 if k < 5 else 0.55, x0 + k * pitch - dw / 2, cy - dh / 2, dw, dh, "index", ABS))
    cx, cyd, rd = K.DIAL
    out.append(ellipse("dial", "body", 3.3, 0.97, cx, cyd, rd, "index", ABS))
    for k, (dx, dy) in enumerate(((1, 0), (0, 1), (-1, 0), (0, -1))):
        out.append(poly(f"tick-{k + 1}", "detail", 3.45, 0.2, [(cx + 0.013 * dx, cyd + 0.013 * dy), (cx + 0.017 * dx, cyd + 0.017 * dy)], "index", ABS, closed=False, filled=False))
    out += [
        poly("needle", "body", 3.5, 0.1, needle_points(0.005, 0.004, 0.0155), "index", ABS, bone="pointer"),
        ellipse("pivot", "detail", 3.55, 0.1, 0, 0, 0.004, "index", ABS, bone="pointer"),
        rect("plate", "body", 3.2, 0.88, -K.PLATE[0], K.PLATE[1], 2 * K.PLATE[0], K.PLATE[2] - K.PLATE[1], "shell", ST, r=0.001),
        poly("plate-line-1", "detail", 3.3, 0.25, [(-0.045, -0.047), (0.045, -0.047)], "shell", ST, closed=False, filled=False),
        poly("plate-line-2", "detail", 3.3, 0.25, [(-0.045, -0.062), (0.025, -0.062)], "shell", ST, closed=False, filled=False),
    ]
    return out


def front50():
    """📐 1:50 front: the meter side-on, index to the left."""
    _, bd, bh = K.BEAD
    af = K.NUT_R * 0.866
    return [
        poly("stub", "body", 2.9, 0.5, stub_outline(0.0), "pipes", PIPE),
        rect("union", "body", 3, 0.4, -af, K.NUT_Y0, 2 * af, K.NUT_Y1 - K.NUT_Y0, "pipes", PIPE, r=0.0015),
        poly("union-edge", "detail", 3.05, 0.15, [(0, K.NUT_Y0), (0, K.NUT_Y1)], "pipes", PIPE, closed=False, filled=False),
        rect("tail", "body", 2.95, 0.5, -K.TAIL_R, K.NUT_Y1, 2 * K.TAIL_R, K.Y_TOP - K.NUT_Y1, "pipes", PIPE),
        poly("lower-shell", "body", 3, 0.72, shell_outline(K.B, False, 3), "shell", ST),
        poly("upper-shell", "body", 3.01, 0.76, shell_outline(K.B, True, 3), "shell", ST),
        rect("seam", "detail", 3.1, 0.5, -bd, -bh, 2 * bd, 2 * bh, "shell", ST, r=0.002),
        rect("index", "body", 3.2, 0.9, K.IZ_FRONT, K.IY0, -0.059 - K.IZ_FRONT, K.IY1 - K.IY0, "index", ABS),
        poly("index-edge", "detail", 3.3, 0.3, [(K.IZ_FACE, K.IY0), (K.IZ_FACE, K.IY1)], "index", ABS, closed=False, filled=False),
    ]


def plan50():
    """📐 1:50 plan: the seam bead as outline, the shell edge inside it, the index head in front, hexagonal nuts, tailpieces and bores, the flow arrow."""
    bw, bd, _ = K.BEAD
    out = [
        poly("housing", "body", 3, 0.76, plan_outline(bw, bd, 32), "shell", ST),
        poly("bead", "detail", 3.1, 0.3, plan_outline(K.A, K.B, 32), "shell", ST, filled=False),
        rect("index", "body", 3.2, 0.9, -K.IX, K.IZ_FRONT, 2 * K.IX, -0.060 - K.IZ_FRONT, "index", ABS),
        poly("index-edge", "detail", 3.3, 0.3, [(-K.IX, K.IZ_FACE), (K.IX, K.IZ_FACE)], "index", ABS, closed=False, filled=False),
        poly("flow-arrow", "detail", 3.15, 0.45, [(-0.02, -0.002), (0.008, -0.002), (0.008, -0.006), (0.02, 0.0), (0.008, 0.006), (0.008, 0.002), (-0.02, 0.002)], "shell", ST),
    ]
    for k, x in ((1, -K.XS), (2, K.XS)):
        out += [
            poly(f"union-{k}", "body", 3, 0.4, [(xx, z) for z, xx in K.hexagon(0, x, K.NUT_R)], "pipes", PIPE),
            ellipse(f"stub-{k}", "body", 3.1, 0.5, x, 0, K.TAIL_R, "pipes", PIPE),
            ellipse(f"bore-{k}", "body", 3.2, 0.12, x, 0, K.TAIL_BORE, None, None),
        ]
    return out


def elevation100():
    """📐 1:100 elevation."""
    bw = K.BEAD[0]
    cw, ch, cy = K.COUNTER
    out = []
    for k, x in ((1, -K.XS), (2, K.XS)):
        out += [
            rect(f"stub-{k}", "body", 2.9, 0.5, x - K.STUB_R, 0.093, 2 * K.STUB_R, K.Y_TOP - 0.093, "pipes", PIPE),
            rect(f"union-{k}", "body", 3, 0.4, x - 0.02, K.NUT_Y0, 0.04, K.NUT_Y1 - K.NUT_Y0, "pipes", PIPE),
        ]
    return out + [
        poly("housing", "body", 3, 0.74, housing_outline(bw, 3), "shell", ST),
        rect("index", "body", 3.2, 0.9, -K.IX, K.IY0, 2 * K.IX, K.IY1 - K.IY0, "index", ABS, r=0.005),
        rect("counter", "body", 3.25, 0.1, -cw / 2, cy - ch / 2, cw, ch, "index", ABS),
        ellipse("dial", "body", 3.3, 0.97, K.DIAL[0], K.DIAL[1], K.DIAL[2], "index", ABS),
        poly("needle", "body", 3.5, 0.1, needle_points(0.005, 0.004, 0.0155), "index", ABS, bone="pointer"),
        rect("plate", "body", 3.2, 0.88, -K.PLATE[0], K.PLATE[1], 2 * K.PLATE[0], K.PLATE[2] - K.PLATE[1], "shell", ST),
    ]


def front100():
    """📐 1:100 front."""
    bd = K.BEAD[1]
    return [
        rect("stub", "body", 2.9, 0.5, -K.STUB_R, 0.093, 2 * K.STUB_R, K.Y_TOP - 0.093, "pipes", PIPE),
        rect("union", "body", 3, 0.4, -0.02, K.NUT_Y0, 0.04, K.NUT_Y1 - K.NUT_Y0, "pipes", PIPE),
        poly("housing", "body", 3, 0.74, housing_outline(bd, 3), "shell", ST),
        rect("index", "body", 3.2, 0.9, K.IZ_FRONT, K.IY0, 0.024, K.IY1 - K.IY0, "index", ABS),
    ]


def plan100():
    """📐 1:100 plan."""
    bw, bd, _ = K.BEAD
    return [
        poly("housing", "body", 3, 0.74, plan_outline(bw, bd, 24), "shell", ST),
        rect("index", "body", 3.2, 0.9, -K.IX, K.IZ_FRONT, 2 * K.IX, 0.024, "index", ABS),
        ellipse("union-1", "body", 3, 0.4, -K.XS, 0, 0.02, "pipes", PIPE),
        ellipse("union-2", "body", 3, 0.4, K.XS, 0, 0.02, "pipes", PIPE),
    ]


def elevation200():
    """📐 1:200 elevation."""
    bw = K.BEAD[0]
    return [
        rect("stub-1", "body", 2.9, 0.45, -K.XS - 0.02, 0.093, 0.04, K.Y_TOP - 0.093, "pipes", PIPE),
        rect("stub-2", "body", 2.9, 0.45, K.XS - 0.02, 0.093, 0.04, K.Y_TOP - 0.093, "pipes", PIPE),
        rect("housing", "body", 3, 0.74, -bw, K.YB, 2 * bw, K.YT - K.YB, "shell", ST, r=K.R),
        rect("index", "body", 3.2, 0.9, -K.IX, K.IY0, 2 * K.IX, K.IY1 - K.IY0, "index", ABS),
        rect("counter", "body", 3.25, 0.1, -0.052, 0.048, 0.104, 0.022, "index", ABS),
        ellipse("dial", "body", 3.3, 0.97, K.DIAL[0], K.DIAL[1], 0.02, "index", ABS),
        poly("needle", "body", 3.5, 0.1, needle_points(0.006, 0.005, 0.017), "index", ABS, bone="pointer"),
    ]


def front200():
    """📐 1:200 front: housing and index step in one outline, one pin."""
    bd = K.BEAD[1]
    ring = rounded(-bd, K.YB, bd, K.YT, K.R)
    step = [(-bd, K.IY1), (K.IZ_FRONT, K.IY1), (K.IZ_FRONT, K.IY0), (-bd, K.IY0)]
    out = ring[:12] + step + ring[12:]
    return [
        rect("stub", "body", 2.9, 0.45, -0.02, 0.093, 0.04, K.Y_TOP - 0.093, "pipes", PIPE),
        poly("housing", "body", 3, 0.74, out, None, ST, regions=["shell", "index"]),
    ]


def plan200():
    """📐 1:200 plan: one outline of housing and index strip."""
    bw, bd, _ = K.BEAD
    ring = rounded(-bw, -bd, bw, bd, K.R)
    strip = [(-K.IX, -bd), (-K.IX, K.IZ_FRONT), (K.IX, K.IZ_FRONT), (K.IX, -bd)]
    return [poly("meter", "body", 3, 0.74, ring + strip, None, ST, regions=["shell", "index"])]


def text_of(key):
    """🗣️ A bilingual text of the texts file."""
    return copy.deepcopy(TEXTS[key])


def roots(asset):
    """🌱 The corrected asset (everything but `volumes`)."""
    a = copy.deepcopy(asset)
    a["summary"] = text_of("summary")
    refs = a["provenance"]["references"]
    refs[0]["citation"] = "DIN EN 1359:2017 Gas meters - Diaphragm gas meters (G4 size: maximum flow 6 cubic metres per hour, minimum flow 0.04 cubic metres per hour, index with roller counter and test element)."
    if not any(r["id"] == "elster-bk-g4-compact" for r in refs):
        refs.insert(1, {"id": "elster-bk-g4-compact", "citation": "Elster-Instromet, BK-G4 compact diaphragm meter for residential installations, data sheet BK-G4 DS Rev. 0 (two-pipe, top in and top out: connection spacing 110 mm, height 221 mm, width 193 mm, depth 157 mm, thread M30 x 2, folded steel housing, 1.9 kg, cyclic volume 1.2 litres, 0.04 to 6 cubic metres per hour); Elster GmbH, BK-G4 and BK-G4T data sheet 2012 (pressed housing powder coated pale grey RAL 7035, 0.5 bar).", "url": "https://add-furnace.com/product-catalog/elster-thailand/bk-g4-gas-meter-manual.pdf"})
    a["provenance"]["notes"] = text_of("notes")
    a["dimensions"].update({"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH})
    mats = a["materials"]
    mats[0] = {"id": ST, "title": {"en": "Deep-drawn sheet steel, powder coated pale grey", "de": "Tiefgezogenes Stahlblech, hellgrau pulverbeschichtet"}, "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 480, "source": "authored-estimate", "reference": "incropera-2018"}
    if not any(m["id"] == "polyester-gf" for m in mats):
        mats.append({"id": "polyester-gf", "title": {"en": "Glass-fibre reinforced polyester of the measuring unit", "de": "Glasfaserverstärkter Polyester des Messwerks"}, "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1500, "conductivityWmK": 0.3, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "incropera-2018"})
    body, pointer = a["rig"]["bones"]
    body["lengthM"] = 0.0965
    body["mass"]["comY"] = K.COM_Y
    pointer.update({"y": K.DIAL[1], "angleDeg": 90, "lengthM": 0.0155})
    a["anchors"] = [
        {"id": "union-inlet", "bone": "body", "x": -K.XS, "y": K.Y_STUB_TOP, "kind": "support", "title": {"en": "Inlet union on the meter bar", "de": "Einlassverschraubung am Zählerbügel"}, "support": "pin", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.03},
        {"id": "union-outlet", "bone": "body", "x": K.XS, "y": K.Y_STUB_TOP, "kind": "support", "title": {"en": "Outlet union on the meter bar", "de": "Auslassverschraubung am Zählerbügel"}, "support": "roller", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.4},
    ]
    for pose in a["poses"]:
        pose["contacts"] = ["union-inlet", "union-outlet"]
        regions = pose["thermal"]["regions"]
        regions["shell"] = {"exposedFraction": 1, "sunlitFraction": 0}
        regions["measuring-unit"] = {"exposedFraction": 1, "sunlitFraction": 0}
    for clip in a["clips"]:
        for c, anchor in zip(clip["contacts"], ("union-inlet", "union-outlet")):
            c["anchor"] = anchor
    th = a["thermal"]
    shell, index, pipes = th["regions"][:3]
    shell.update({"title": {"en": "Sheet-steel housing and data plate", "de": "Stahlblechgehäuse und Datenschild"}, "material": ST, "areaM2": AREA["shell"], "characteristicLengthM": 0.195, "heatCapacityJM2K": 3770})
    shell["layers"][0].update({"title": {"en": "Powder-coated sheet steel 1 mm", "de": "Pulverbeschichtetes Stahlblech 1 mm"}, "material": ST, "thicknessM": 0.001, "resistanceM2KW": 2e-05, "heatCapacityJM2K": 3770})
    index.update({"areaM2": AREA["index"], "characteristicLengthM": 0.13})
    pipes.update({"areaM2": AREA["pipes"], "characteristicLengthM": 0.03, "heatCapacityJM2K": 15300})
    pipes["layers"][0].update({"title": {"en": "Steel stub wall 4 mm with gas inside", "de": "Stahlstutzenwand 4 mm mit Gas innen"}, "thicknessM": 0.004, "resistanceM2KW": 8e-05, "heatCapacityJM2K": 15300})
    th["regions"] = [shell, index, pipes, {
        "id": "measuring-unit", "title": {"en": "Measuring unit with diaphragm chambers, valves and outlet duct", "de": "Messwerk mit Membrankammern, Ventilen und Auslasskanal"},
        "kind": "surface", "model": "passive", "material": "polyester-gf", "areaM2": AREA["measuring-unit"], "characteristicLengthM": 0.15, "geometry": "vertical-plate", "skyViewFactor": 0,
        "layers": [{"id": "wall", "title": {"en": "Moulded wall 3 mm", "de": "Formteilwand 3 mm"}, "kind": "polymer", "material": "polyester-gf", "thicknessM": 0.003, "resistanceM2KW": 0.01, "heatCapacityJM2K": 5400, "source": "authored-estimate", "reference": "incropera-2018"}],
        "heatCapacityJM2K": 5400, "source": "authored-estimate", "reference": "incropera-2018", "hidden": text_of("hidden"),
    }]
    th["assumptions"] = [th["assumptions"][0], text_of("thermal-assumption")]
    a["structure"]["supports"] = ["union-inlet", "union-outlet"]
    a["structure"]["assumptions"] = [text_of("structure-assumption")]
    rep = a["representations"]
    rep["50"]["geometry"] = {"elevation": elevation50(), "plan": plan50(), "front": front50()}
    rep["100"]["geometry"] = {"elevation": elevation100(), "plan": plan100(), "front": front100()}
    rep["200"]["geometry"] = {"elevation": elevation200(), "plan": plan200(), "front": front200()}
    for scale in ("50", "100", "200"):
        rep[scale]["decisions"] = text_of("decisions-" + scale)
    rep["500"]["reason"] = text_of("reason-500")
    rep["500"]["limitingFeature"] = text_of("limiting-500")
    rep["500"]["limitingSizeM"] = 0.005
    return a


def js_numbers(text):
    """🔢 Writes exponent numbers the way the catalog files carry them (JavaScript: plain decimals down to 1e-6)."""
    def repl(m):
        v = float(m.group(0))
        if v != 0 and abs(v) >= 1e-6 and abs(v) < 1e21:
            return ("%.15f" % v).rstrip("0").rstrip(".")
        return m.group(0)
    return re.sub(r"-?\d+(?:\.\d+)?e[+-]\d+", repl, text)


def indent2(value):
    """🪜 The two-space style with the asset's member indentation."""
    return js_numbers(json.dumps(value, indent=2, ensure_ascii=False)).replace("\n", "\n    ")


def build():
    """🏗️ Writes and validates the candidate document."""
    doc = json.load(open(CATALOG, encoding="utf-8"))
    new = roots(doc["asset"])
    new["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    doc["asset"] = new
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    bun = sorted(os.popen("ls ~/Library/Caches/semio/runtimes/bun/*/bun").read().split())[0]
    r = subprocess.run([bun, "r3-validate.ts", "--file", CANDIDATE], cwd=HERE, capture_output=True, text=True)
    print(r.stdout[-6000:], r.stderr[-2000:])
    return doc, r.returncode


def apply(doc):
    """📝 Writes the changed members into the catalog file in one write (the new volumes need the new anchors and regions) and the manifest entry."""
    want = doc["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    have = json.loads(text)["asset"]
    changed = []
    for key in want:
        if have.get(key) == want[key]:
            continue
        start, end = NS["member_range"](text, "    ", key)
        old = text[start:end]
        style = indent2 if key != "volumes" and indent2(have[key]) in old else (lambda v: js_numbers(NS["dump"](v, 2)))
        text = text[:start] + "    " + json.dumps(key) + ": " + style(want[key]) + text[end:]
        changed.append(key)
    got = json.loads(text)["asset"]
    if got != want:
        raise SystemExit("written asset differs from the candidate")
    if list(got) != list(have):
        raise SystemExit("member order changed")
    open(CATALOG, "w", encoding="utf-8").write(text)
    m = open(MANIFEST, encoding="utf-8").read()
    i = m.index('"id": "gas-meter"')
    j = m.index('"completion"', i)
    entry = m[i:j]
    entry = re.sub(r'"heightM": [0-9.]+', f'"heightM": {HEIGHT}', entry, count=1)
    entry = re.sub(r'"widthM": [0-9.]+', f'"widthM": {WIDTH}', entry, count=1)
    k = entry.index('"distinction": ')
    head, tail = entry[:k], entry[k:]
    for lang in ("en", "de"):
        tail = re.sub(r'"' + lang + r'": "(?:[^"\\\\]|\\\\.)*"', lambda _m: '"' + lang + '": ' + json.dumps(TEXTS["distinction"][lang], ensure_ascii=False), tail, count=1)
    entry = head + tail
    probe = json.loads("{" + entry.rstrip().rstrip(",") + "}")
    if probe["distinction"] != TEXTS["distinction"] or probe["heightM"] != HEIGHT or probe["widthM"] != WIDTH:
        raise SystemExit("manifest entry did not round-trip")
    m = m[:i] + entry + m[j:]
    json.loads(m)
    open(MANIFEST, "w", encoding="utf-8").write(m)
    print("written", CATALOG, "members:", ", ".join(changed), "; manifest entry updated")


if __name__ == "__main__":
    doc, code = build()
    if "--apply" in sys.argv:
        if code != 0:
            raise SystemExit("candidate INVALID: not applied")
        apply(doc)
