"""🌱 Round 3, appliances/circulation-pump: the root corrections (dimensions, rig, anchors, materials, texts, areas) and the 2D drawings of every scale and view, drawn by hand as the true orthographic views of that scale's body (ruling E1). Reads the catalog asset and the volumes source, writes the candidate `🗑️generated/r3-circulation-pump/🔣️.json`. Running it again on the corrected catalog gives the same candidate.

Usage: python3 r3-circulation-pump-roots.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-circulation-pump-kit.py"))
k = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(k)

CATALOG = os.path.normpath(os.path.join(HERE, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/circulation-pump/🔣️.json"))
SOURCE = os.path.join(HERE, "volumes", "appliances", "circulation-pump.json")
OUT = os.path.join(HERE, "🗑️generated", "r3-circulation-pump", "🔣️.json")
A = k.AXIS_Y
IRON, ALU, PA, GLASS, CU, BRASS, STEEL = "cast-iron", "aluminium", "polyamide", "window-glass", "copper", "brass", "steel"
REGION = {IRON: "casing", CU: "casing", BRASS: "casing", ALU: "motor", STEEL: "motor", PA: "terminal-box", GLASS: "rotor-window"}


def text(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def shape(id, bone, z, tone, geometry, material=None, role="body", filled=True, region="auto", regions=None):
    """✏️ One 2D shape: the common members in the order of the file, then its geometry."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    if regions:
        out["regions"] = regions
    elif filled and region == "auto" and material:
        out["region"] = REGION[material]
    elif region not in ("auto", None):
        out["region"] = region
    if material:
        out["material"] = material
    return out


def rect(x0, y0, x1, y1, r=None):
    """▭ A rectangle between two corners."""
    out = {"kind": "rect", "x": k.r5(min(x0, x1)), "y": k.r5(min(y0, y1)), "w": k.r5(abs(x1 - x0)), "h": k.r5(abs(y1 - y0))}
    if r:
        out["r"] = r
    return out


def circle(cx, cy, r):
    """◯ A circle."""
    return {"kind": "ellipse", "cx": k.r5(cx), "cy": k.r5(cy), "rx": r, "ry": r}


def poly(points, closed=True):
    """⬠ A polygon or a stroke."""
    return {"kind": "poly", "points": [k.pt(p) for p in points], "closed": closed}


def sym(profile, flip=False):
    """🪞 The outline of an axisymmetric profile [(axial, radial)] seen from the side: upper edge out, lower edge back."""
    pts = [(a, r) for a, r in profile] + [(a, -r) for a, r in reversed(profile)]
    return [(-a, r) for a, r in pts] if flip else pts


def hb(points):
    """🚿 Pipe-axis coordinates into the elevation frame of the housing bone."""
    return [(x + k.XH, y) for x, y in points]


def tb(points):
    """📦 World coordinates (x along the pipe, y above the shaft) into the elevation frame of the terminal bone (u up, v to the left)."""
    return [(y - k.TY, -x) for x, y in points]


def tb_rect(x0, y0, x1, y1, r=None):
    """📦 A rectangle given in world coordinates, in the frame of the terminal bone."""
    (a0, b0), (a1, b1) = tb([(x0, y0), (x1, y1)])
    return rect(a0, b0, a1, b1, r)


def volute_side(sections, cy):
    """🐚 The volute seen across the shaft: (w, offset + radius) along the top, back down the rounded back and along the bottom."""
    return [(w, cy + r) for w, r in sections] + [(w, cy - r) for w, r in reversed(sections)]


def swap(points):
    """🔁 (a, b) to (b, a)."""
    return [(b, a) for a, b in points]


def up(points, dy=A, dx=0.0):
    """⬆️ Shifted points."""
    return [(x + dx, y + dy) for x, y in points]


def nut_outline(half, chamfer=True):
    """🔩 The union nut seen across the pipe: its length by the given half height, with the end chamfers."""
    x0, x1 = k.X_NUT
    if not chamfer:
        return [(x0, half), (x1, half), (x1, -half), (x0, -half)]
    c = half * 0.93
    return [(x0, c), (x0 + 0.0015, half), (x1 - 0.0015, half), (x1, c), (x1, -c), (x1 - 0.0015, -half), (x0 + 0.0015, -half), (x0, -c)]


def mirror_x(points):
    """🪞 Mirrored about the shaft plane."""
    return [(-x, y) for x, y in points]


def neck_outline(profile, x_from=0.040):
    """🏺 A port neck seen across the pipe from x_from out to where the nut covers the thread."""
    pts = [(x_from, profile[0][1] + (profile[1][1] - profile[0][1]) * (x_from - profile[0][0]) / (profile[1][0] - profile[0][0]))] + [p for p in profile[1:] if p[0] < 0.079]
    if profile[-1][0] > 0.079:
        pts.append((0.079, pts[-1][1]))
    return sym(pts)


def pipe_parts(scale, frame, cy, half_nut, edge_y):
    """🚿 Tails, necks, unions and the nut edges of one side view (elevation or plan), both sides."""
    out = []
    neck = k.NECK_50 if scale == 50 else k.NECK_100
    for name, side in (("l", -1), ("r", 1)):
        flip = (lambda p: mirror_x(p)) if side < 0 else (lambda p: p)
        place = lambda p: frame(up(flip(p), cy))
        out.append(shape(f"stub-{name}", "housing", 3.02, 0.7, poly(place([(0.095, k.R_STUB), (k.X_END, k.R_STUB), (k.X_END, -k.R_STUB), (0.095, -k.R_STUB)])), CU))
        out.append(shape(f"neck-{name}", "housing", 2.98, 0.42, poly(place(neck_outline(neck))), IRON))
        out.append(shape(f"union-{name}", "housing", 3.1, 0.5, poly(place(nut_outline(half_nut, False))), BRASS))
        if scale == 50:
            for suffix, y in zip(("", "-low"), edge_y):
                out.append(shape(f"nut-edge-{name}{suffix}", "housing", 3.15, 0.3, poly(place([(k.X_NUT[0] + 0.0008, y), (k.X_NUT[1] - 0.0008, y)]), False), role="detail", filled=False))
    return out


def gland_outline(scale):
    """🔌 The cable gland seen from the side, pointing to -x from the left box wall, about its own axis."""
    profile = k.GLAND_50[1:] if scale == 50 else [(0.0, 0.010), (0.017, 0.010)]
    profile = [(0.0, profile[0][1])] + profile if scale == 50 else profile
    return [(-0.030 - a, r) for a, r in sym(profile)]


def elevation(scale):
    """👁️ The view along the motor shaft (from -z): bone-local frames."""
    s = pipe_parts(scale, hb, 0.0, k.R_HEX, (0.015, -0.015)) if scale < 200 else [shape("pipe", "housing", 2.95, 0.6, rect(k.XH - k.X_END, -0.022, k.XH + k.X_END, 0.022), BRASS)]
    merged = ["casing", "motor", "rotor-window"] if scale == 200 else None
    s.append(shape("casing", "housing", 3, 0.42, circle(k.XH + k.CX, k.CY, k.R_VOL), IRON, regions=merged))
    if scale == 200:
        s.append(shape("terminal-box", "terminal", 2.9, 0.78, tb_rect(-0.031, 0.030, 0.031, k.BOX_TOP), PA))
        return s
    if scale == 50:
        s.append(shape("terminal-neck", "terminal", 3.25, 0.7, tb_rect(-0.028, 0.028, 0.028, 0.044), PA))
        s.append(shape("motor-flange", "motor", 3.28, 0.55, rect(-k.FL, -k.FL, k.FL, k.FL, k.FL_R), ALU))
    else:
        s.append(shape("terminal-box", "terminal", 3.25, 0.78, tb_rect(-0.031, 0.028, 0.031, k.BOX_TOP, 0.004), PA))
        s.append(shape("flange", "motor", 3.28, 0.55, rect(-k.FL, -k.FL, k.FL, k.FL, k.FL_R), ALU))
    s.append(shape("motor-head", "motor", 3.3, 0.62, circle(0, 0, k.R_STATOR), ALU))
    if scale == 100:
        s.append(shape("window", "motor", 3.4, 0.18, circle(0, 0, k.R_PANE), GLASS))
        s.append(shape("gland", "terminal", 3.2, 0.4, poly(tb(up(gland_outline(100), 0.052))), PA))
        return s
    s.append(shape("motor-ring", "motor", 3.31, 0.5, circle(0, 0, 0.038), role="detail", filled=False))
    for i, (sx, sy) in enumerate(((1, 1), (-1, 1), (-1, -1), (1, -1)), 1):
        s.append(shape(f"screw-{i}", "motor", 3.32, 0.3, circle(sx * k.SCREW, sy * k.SCREW, 0.005), STEEL, role="detail"))
    s.append(shape("bezel", "motor", 3.39, 0.5, circle(0, 0, k.R_BEZEL), ALU, role="detail"))
    s.append(shape("window", "motor", 3.4, 0.18, circle(0, 0, k.R_PANE), GLASS))
    s.append(shape("gland", "terminal", 3.45, 0.4, poly(tb(up(gland_outline(50), 0.052))), PA, role="detail"))
    s.append(shape("terminal-box", "terminal", 3.5, 0.78, tb_rect(-0.030, 0.042, 0.030, 0.062, 0.004), PA))
    s.append(shape("terminal-lid", "terminal", 3.505, 0.82, tb_rect(-0.031, 0.061, 0.031, k.BOX_TOP, 0.004), PA))
    s.append(shape("lid-seam", "terminal", 3.51, 0.3, poly(tb([(-0.031, 0.061), (0.031, 0.061)]), False), role="detail", filled=False))
    s.append(shape("led", "terminal", 3.52, 0.95, circle(0.068 - k.TY, 0, 0.0045), PA, role="detail"))
    s.append(shape("pointer", "rotor", 3.6, 0.95, poly([(0, -0.006), (0.024, 0), (0, 0.006)]), PA, role="detail", region=None))
    s.append(shape("tail", "rotor", 3.6, 0.7, poly([(0, 0.0045), (-0.011, 0), (0, -0.0045)]), PA, role="detail", region=None))
    s.append(shape("hub", "rotor", 3.7, 0.35, circle(0, 0, 0.0055), PA, role="detail", region=None))
    return s


def front(scale):
    """👁️ The view along the pipe (from +x): paper x = world z, paper y = height."""
    volute = {50: k.VOLUTE_50, 100: k.VOLUTE_100, 200: k.VOLUTE_200}[scale]
    if scale == 200:
        body = [(k.W_FRONT, A - k.R_STATOR), (k.W_HOUSING, A - k.R_STATOR)] + [(w, A + k.CY - r) for w, r in volute] + [(w, A + k.CY + r) for w, r in reversed(volute)] + [(k.W_FRONT, A + k.CY + k.R_VOL)]
        return [
            shape("terminal-box", "terminal", 2.9, 0.78, rect(k.TW - 0.030, A + 0.030, k.TW + 0.034, A + k.BOX_TOP), PA),
            shape("body", "housing", 3, 0.42, poly(body), IRON, regions=["casing", "motor", "rotor-window"]),
            shape("pipe", "housing", 3.8, 0.6, circle(k.WP, A, 0.022), BRASS),
        ]
    s = [shape("casing", "housing", 3, 0.42, poly(up(volute_side(volute, k.CY))), IRON)]
    if scale == 50:
        s.append(shape("flange", "housing", 3.05, 0.38, rect(k.W_HOUSING, A - k.FL, k.W_FLANGE, A + k.FL), IRON))
        s.append(shape("terminal-neck", "terminal", 3.25, 0.7, rect(k.TW - 0.028, A + 0.028, k.TW + 0.028, A + 0.044), PA))
        s.append(shape("motor-flange", "motor", 3.28, 0.55, rect(k.W_MOTOR, A - k.FL, k.W_HOUSING, A + k.FL), ALU))
        s.append(shape("motor-head", "motor", 3.3, 0.62, poly(up(sym(HEAD_50))), ALU))
        s.append(shape("motor-ring", "motor", 3.31, 0.5, poly([(-0.056, A - k.R_STATOR), (-0.056, A + k.R_STATOR)], False), role="detail", filled=False))
        for name, sy in (("top", 1), ("bottom", -1)):
            s.append(shape(f"screw-{name}", "motor", 3.32, 0.3, rect(-0.006, A + sy * k.SCREW - 0.005, 0.0, A + sy * k.SCREW + 0.005), STEEL, role="detail"))
        s.append(shape("terminal-box", "terminal", 3.5, 0.78, rect(k.TW - 0.033, A + 0.042, k.TW + 0.033, A + 0.062, 0.004), PA))
        s.append(shape("terminal-lid", "terminal", 3.505, 0.82, rect(k.TW - 0.034, A + 0.061, k.TW + 0.034, A + k.BOX_TOP, 0.004), PA))
        s.append(shape("lid-seam", "terminal", 3.51, 0.3, poly([(k.TW - 0.034, A + 0.061), (k.TW + 0.034, A + 0.061)], False), role="detail", filled=False))
        s.append(shape("union", "housing", 3.8, 0.5, poly([(k.WP + z, A + y) for y, z in k.hexagon()]), BRASS))
        s.append(shape("stub", "housing", 3.85, 0.7, circle(k.WP, A, k.R_STUB), CU))
        s.append(shape("bore", "housing", 3.9, 0.12, circle(k.WP, A, k.R_BORE), region=None))
        return s
    s.append(shape("terminal-box", "terminal", 3.25, 0.78, rect(k.TW - 0.030, A + 0.028, k.TW + 0.034, A + k.BOX_TOP, 0.004), PA))
    s.append(shape("flange", "motor", 3.28, 0.55, rect(k.W_MOTOR, A - k.FL, k.W_FLANGE, A + k.FL), ALU))
    s.append(shape("motor-head", "motor", 3.3, 0.62, poly(up(sym(k.STATOR_100[:-1] + [(0.0, k.R_STATOR)]))), ALU))
    s.append(shape("union", "housing", 3.8, 0.5, poly([(k.WP + z, A + y) for y, z in k.hexagon()]), BRASS))
    s.append(shape("stub", "housing", 3.85, 0.7, circle(k.WP, A, k.R_STUB), CU))
    return s


def plan(scale):
    """👁️ The view from above: paper x = world x, paper y = world z."""
    volute = {50: k.VOLUTE_50, 100: k.VOLUTE_100, 200: k.VOLUTE_200}[scale]
    top = lambda pts: swap(up(pts, 0.0))
    if scale == 200:
        body = [(-k.R_STATOR, k.W_FRONT), (k.R_STATOR, k.W_FRONT), (k.R_STATOR, k.W_HOUSING)] + [(k.CX + r, w) for w, r in volute] + [(k.CX - r, w) for w, r in reversed(volute)] + [(-k.R_STATOR, k.W_HOUSING)]
        return [
            shape("pipe", "housing", 2.9, 0.6, rect(-k.X_END, k.WP - 0.022, k.X_END, k.WP + 0.022), BRASS),
            shape("body", "housing", 3, 0.42, poly(body), IRON, regions=["casing", "motor", "rotor-window"]),
            shape("terminal-box", "terminal", 3.5, 0.78, rect(-0.031, k.TW - 0.030, 0.031, k.TW + 0.034), PA),
        ]
    s = pipe_parts(scale, lambda p: p, k.WP, k.R_HEX * 0.8660254, (0.0,))
    s.append(shape("casing", "housing", 3, 0.42, poly([(k.CX + r, w) for w, r in volute] + [(k.CX - r, w) for w, r in reversed(volute)]), IRON))
    if scale == 50:
        s.append(shape("flange", "housing", 3.05, 0.38, rect(-k.FL, k.W_HOUSING, k.FL, k.W_FLANGE), IRON))
        s.append(shape("motor-flange", "motor", 3.28, 0.55, rect(-k.FL, k.W_MOTOR, k.FL, k.W_HOUSING), ALU))
        s.append(shape("motor-head", "motor", 3.3, 0.62, poly(swap(sym(HEAD_50))), ALU))
        s.append(shape("motor-ring", "motor", 3.31, 0.5, poly([(-k.R_STATOR, -0.056), (k.R_STATOR, -0.056)], False), role="detail", filled=False))
        for name, sx in (("l", -1), ("r", 1)):
            s.append(shape(f"screw-{name}", "motor", 3.32, 0.3, rect(sx * k.SCREW - 0.005, -0.006, sx * k.SCREW + 0.005, 0.0), STEEL, role="detail"))
        s.append(shape("gland", "terminal", 3.45, 0.4, poly(up(gland_outline(50), k.TW)), PA, role="detail"))
        s.append(shape("terminal-box", "terminal", 3.5, 0.78, rect(-0.030, k.TW - 0.033, 0.030, k.TW + 0.033, 0.004), PA))
        s.append(shape("terminal-lid", "terminal", 3.505, 0.82, rect(-0.031, k.TW - 0.034, 0.031, k.TW + 0.034, 0.004), PA))
        return s
    s.append(shape("flange", "motor", 3.28, 0.55, rect(-k.FL, k.W_MOTOR, k.FL, k.W_FLANGE), ALU))
    s.append(shape("motor-head", "motor", 3.3, 0.62, poly(swap(sym(k.STATOR_100[:-1] + [(0.0, k.R_STATOR)]))), ALU))
    s.append(shape("gland", "terminal", 3.45, 0.4, poly(up(gland_outline(100), k.TW)), PA))
    s.append(shape("terminal-box", "terminal", 3.5, 0.78, rect(-0.031, k.TW - 0.030, 0.031, k.TW + 0.034, 0.004), PA))
    return s


HEAD_50 = [(k.W_FRONT, 0.0295), (-0.066, k.R_BEZEL), (k.W_STATOR, k.R_BEZEL)] + k.STATOR_50[:-1] + [(0.0, k.R_STATOR)]
TEXTS = json.load(open(os.path.join(HERE, "r3-circulation-pump-texts.json"), encoding="utf-8"))


def roots(asset):
    """🌱 Every correction outside `volumes`."""
    asset["summary"] = TEXTS["summary"]
    refs = [r for r in asset["provenance"]["references"] if r["id"] != "grundfos-ups-25-60-180"]
    refs.insert(1, {"id": "grundfos-ups-25-60-180", "citation": TEXTS["citation"]})
    asset["provenance"]["references"] = refs
    asset["provenance"]["notes"] = TEXTS["notes"]
    asset["dimensions"].update(heightM=0.126, widthM=0.22, depthM=0.134)
    materials = [m for m in asset["materials"] if m["id"] != STEEL]
    materials.append({"id": STEEL, "title": text("Zinc-plated steel screw", "Verzinkte Stahlschraube"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 480, "source": "literature", "reference": "incropera-2018"})
    asset["materials"] = materials
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    bones["housing"].update(x=-k.XH, y=A, lengthM=2 * k.XH, title=text("Pump housing along the pipe axis, from the left to the right support", "Pumpengehäuse auf der Rohrachse, vom linken zum rechten Lager"))
    bones["housing"]["mass"].update(comX=k.XH, comY=0)
    bones["motor"].update(x=k.XH, y=0, lengthM=k.R_STATOR, title=text("Wet-rotor motor head on the shaft axis (the bone spans its radius)", "Nassläufer-Motorkopf auf der Wellenachse (der Knochen spannt seinen Radius)"))
    bones["terminal"].update(x=0, y=k.TY, angleDeg=90, lengthM=k.r5(k.BOX_TOP - k.TY))
    bones["terminal"]["mass"].update(comX=0.016, comY=0)
    bones["rotor"].update(lengthM=0.024)
    anchors = {a["id"]: a for a in asset["anchors"]}
    anchors["pipe-left"].update(x=0, y=0, z=k.WP, footprintLengthM=0.008, title=text("Left pipe tail (pin)", "Linkes Einlegeteil (Festlager)"))
    anchors["pipe-right"].update(x=2 * k.XH, y=0, z=k.WP, footprintLengthM=0.008, title=text("Right pipe tail (roller)", "Rechtes Einlegeteil (Rollenlager)"))
    anchors["motor-top"].update(x=k.r5(k.BOX_TOP - k.TY), y=0, z=k.TW)
    for scale in (50, 100, 200):
        rep = asset["representations"][str(scale)]
        rep["geometry"] = {"elevation": elevation(scale), "plan": plan(scale), "front": front(scale)}
        rep["decisions"] = TEXTS["decisions"][str(scale)]
    asset["representations"]["500"]["reason"] = TEXTS["reason500"]
    regions = {r["id"]: r for r in asset["thermal"]["regions"]}
    for rid, area in TEXTS["areas"].items():
        regions[rid]["areaM2"] = area
    regions["casing"]["characteristicLengthM"] = 0.102
    regions["motor"]["characteristicLengthM"] = 0.084
    regions["casing"]["title"] = text("Pump housing, unions and pipe tails", "Pumpengehäuse, Verschraubungen und Einlegeteile")
    asset["thermal"]["emitters"][0]["title"] = text("Warm air rising from the motor head past the terminal box", "Warme Luft, die vom Motorkopf am Klemmenkasten vorbei aufsteigt")
    asset["structure"]["assumptions"] = TEXTS["structure"]
    return asset


def main():
    document = json.load(open(CATALOG, encoding="utf-8"))
    document["asset"] = roots(document["asset"])
    document["asset"]["volumes"] = json.load(open(SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(document, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print("written", OUT)


if __name__ == "__main__":
    main()
