"""🪵️ Authors the staffage asset `used-writing-desk` from hand-chosen numbers and writes `🔣️.json` next to this file.

World frame of the staffage (x ahead = along the 1.20 m desk length towards the drawer pedestal, y up, z left = towards the wall; the user sits at -z).
Every part is stated once in world rest coordinates (the stored pose) and converted into the frame of its bone, so 2D shapes and 3D solids share one set of numbers.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
T = lambda en, de: {"en": en, "de": de}

BONES = {
    "ground": ((0.0, 0.0, 0.0), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
    "blanket": ((-0.62, 0.0, 0.0), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
    "leg-panel": ((-0.5575, 0.02, 0.0), ((0, 1, 0), (-1, 0, 0), (0, 0, 1))),
    "top": ((-0.5575, 0.735, 0.0), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
    "carcass": ((0.37, 0.735, 0.0), ((0, -1, 0), (1, 0, 0), (0, 0, 1))),
    "drawer-rail": ((0.37, 0.735, 0.0), ((0, 0, -1), (0, 1, 0), (1, 0, 0))),
    "drawer-bottom": ((0.1905, 0.095, -0.261), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
    "drawer-middle": ((0.1905, 0.420, -0.261), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
    "drawer-top": ((0.1905, 0.605, -0.261), ((1, 0, 0), (0, 1, 0), (0, 0, 1))),
}

R5 = lambda v: round(v + 0.0, 5)


def local(bone, p):
    """📍️ World rest point → bone-local (u, v, w)."""
    o, axes = BONES[bone]
    d = [p[i] - o[i] for i in range(3)]
    return tuple(R5(sum(axes[k][i] * d[i] for i in range(3))) for k in range(3))


def ldir(bone, d):
    """🧭️ World direction → bone-local direction."""
    axes = BONES[bone][1]
    return tuple(R5(sum(axes[k][i] * d[i] for i in range(3))) for k in range(3))


def box(sid, bone, mat, tone, x0, x1, y0, y1, z0, z1, role="body", r=None, **flags):
    """📦️ An axis-aligned world box as a box solid in the frame of its bone."""
    a, b = local(bone, (x0, y0, z0)), local(bone, (x1, y1, z1))
    size = [R5(abs(b[i] - a[i])) for i in range(3)]
    c = [R5((a[i] + b[i]) / 2) for i in range(3)]
    s = {"id": sid, "kind": "box", "bone": bone, "material": mat, "role": role, "tone": tone, "size": size, "transform": {"translation": c}}
    if r:
        s["cornerRadius"] = r
    s.update(flags)
    return s


def capsule(sid, bone, mat, tone, p0, p1, r, role="body", **flags):
    """💊️ A capsule between two world points."""
    s = {"id": sid, "kind": "capsule", "bone": bone, "material": mat, "role": role, "tone": tone, "p0": list(local(bone, p0)), "p1": list(local(bone, p1)), "r0": r, "r1": r}
    s.update(flags)
    return s


def extrude_plan(sid, bone, mat, tone, pts_xz, y0, y1, role="body", **flags):
    """🗺️ A plan outline (world x, z) pushed from height y0 to y1 on a bone whose frame is world aligned (plane zx: profile (w, u), along v)."""
    o = BONES[bone][0]
    prof = [[R5(z - o[2]), R5(x - o[0])] for x, z in pts_xz]
    if polygon_area(prof) < 0:
        prof.reverse()
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": prof, "plane": "zx", "from": R5(y0 - o[1]), "to": R5(y1 - o[1])}
    s.update(flags)
    return s


def extrude_end(sid, bone, mat, tone, pts_zy, x0, x1, role="body", **flags):
    """🪟️ An end outline (world z, y) pushed from x0 to x1 on the vertical leg-panel bone (u = y, v = -x, w = z; plane zx: profile (w, u), along v)."""
    prof = []
    for z, y in pts_zy:
        u, _, w = local(bone, (x0, y, z))
        prof.append([w, u])
    if polygon_area(prof) < 0:
        prof.reverse()
    va, vb = local(bone, (x0, 0.02, 0))[1], local(bone, (x1, 0.02, 0))[1]
    s = {"id": sid, "kind": "extrude", "bone": bone, "material": mat, "role": role, "tone": tone, "profile": prof, "plane": "zx", "from": min(va, vb), "to": max(va, vb)}
    s.update(flags)
    return s


def polygon_area(p):
    """📐️ Signed area of a ring."""
    return sum(p[i][0] * p[(i + 1) % len(p)][1] - p[(i + 1) % len(p)][0] * p[i][1] for i in range(len(p))) / 2


def solid_volume(s):
    """⚖️ Volume of one solid in m³ (with its repeat count)."""
    k = s["kind"]
    if k == "box":
        v = s["size"][0] * s["size"][1] * s["size"][2]
    elif k == "extrude":
        v = abs(polygon_area(s["profile"])) * (s["to"] - s["from"])
    elif k == "capsule":
        p0, p1, r = s["p0"], s["p1"], s["r0"]
        v = math.pi * r * r * math.dist(p0, p1) + 4 / 3 * math.pi * r ** 3
    else:
        v = 0.0
    return v * s.get("repeat", {}).get("count", 1)


def solid_centre(s):
    """🎯️ Local centre of one solid (first copy plus half the repeat travel)."""
    k = s["kind"]
    if k == "box":
        c = list(s["transform"]["translation"])
    elif k == "extrude":
        pr, a = s["profile"], polygon_area(s["profile"])
        cx = sum((pr[i][0] + pr[i - 1][0]) * (pr[i - 1][0] * pr[i][1] - pr[i][0] * pr[i - 1][1]) for i in range(len(pr))) / (6 * a)
        cy = sum((pr[i][1] + pr[i - 1][1]) * (pr[i - 1][0] * pr[i][1] - pr[i][0] * pr[i - 1][1]) for i in range(len(pr))) / (6 * a)
        c = [cy, (s["from"] + s["to"]) / 2, cx]
    elif k == "capsule":
        c = [(s["p0"][i] + s["p1"][i]) / 2 for i in range(3)]
    else:
        c = [0, 0, 0]
    rep = s.get("repeat")
    if rep:
        t = rep.get("translation", [0, 0, 0])
        c = [c[i] + t[i] * (rep["count"] - 1) / 2 for i in range(3)]
    return c


def rectE(sid, bone, x0, x1, y0, y1, z, tone, region, mat, role="body", filled=True, **extra):
    """🖼️ Elevation rect from a world (x, y) box, written in the bone's (u, v) frame."""
    a, b = local(bone, (x0, y0, 0)), local(bone, (x1, y1, 0))
    u0, u1 = sorted((a[0], b[0]))
    v0, v1 = sorted((a[1], b[1]))
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": u0, "y": v0, "w": R5(u1 - u0), "h": R5(v1 - v0)}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    s.update(extra)
    return s


def polyE(sid, bone, pts, z, tone, region, mat, role="detail", closed=False, filled=False, **extra):
    """✒️ Elevation polyline from world (x, y) points."""
    q = [[local(bone, (x, y, 0))[0], local(bone, (x, y, 0))[1]] for x, y in pts]
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": q, "closed": closed}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    s.update(extra)
    return s


def rectW(sid, bone, a0, a1, b0, b1, z, tone, region, mat, role="body", filled=True, **extra):
    """🗺️ Front (a = world z, b = world y) or plan (a = world x, b = world z) rect in world rest coordinates."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": R5(a0), "y": R5(b0), "w": R5(a1 - a0), "h": R5(b1 - b0)}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    s.update(extra)
    return s


def polyW(sid, bone, pts, z, tone, region, mat, role="detail", closed=False, filled=False, **extra):
    """〰️ Front or plan polyline in world rest coordinates."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "points": [[R5(a), R5(b)] for a, b in pts], "closed": closed}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    s.update(extra)
    return s


def ellW(sid, bone, cx, cy, rx, ry, z, tone, region, mat, role="texture", filled=False):
    """⭕️ Front or plan ellipse in world rest coordinates."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "ellipse", "cx": R5(cx), "cy": R5(cy), "rx": rx, "ry": ry}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    return s


def ellE(sid, bone, cx, cy, rx, ry, z, tone, region, mat, role="detail", filled=False):
    """🔘️ Elevation ellipse at a world (x, y) centre."""
    u, v, _ = local(bone, (cx, cy, 0))
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "ellipse", "cx": u, "cy": v, "rx": rx, "ry": ry}
    if region:
        s["region"] = region
    if mat:
        s["material"] = mat
    return s


VEN, TEAK, CHIP, HDF, CHROME, BRASS, FELT, LABEL = "teak-veneered-chipboard", "solid-teak", "chipboard-p2", "hardboard", "chrome-steel", "brass", "moving-blanket", "paper-label"
TOPR, CAR, FRO, BLA = "desk-top", "carcass", "drawer-fronts", "blanket"

DRAWERS = {
    "drawer-bottom": {"front": (0.083, 0.403), "box": (0.095, 0.365), "runner": (0.083, 0.095), "grip": (0.376, 0.394)},
    "drawer-middle": {"front": (0.406, 0.589), "box": (0.420, 0.570), "runner": (0.408, 0.420), "handle": 0.560},
    "drawer-top": {"front": (0.592, 0.732), "box": (0.605, 0.710), "runner": (0.593, 0.605), "grip": (0.705, 0.723)},
}
FX0, FX1 = 0.1905, 0.5495
CHIP_XZ = [(-0.572, -0.30), (0.60, -0.30), (0.60, 0.30), (-0.60, 0.30), (-0.60, -0.284), (-0.588, -0.291)]
TOP_XZ = [(-0.60, -0.30), (0.60, -0.30), (0.60, 0.30), (-0.60, 0.30)]


def body50():
    """🔍️ The 1:50 body: every part of the real desk at 7.5 mm."""
    s = [
        box("blanket", "blanket", FELT, 0.30, -0.62, 0.62, 0.0, 0.02, -0.32, 0.32, r=0.008),
        extrude_plan("top-board", "top", VEN, 0.62, CHIP_XZ, 0.735, 0.76),
        extrude_end("leg-panel", "leg-panel", VEN, 0.55, [(-0.28, 0.02), (-0.20, 0.02), (-0.20, 0.032), (0.20, 0.032), (0.20, 0.02), (0.28, 0.02), (0.28, 0.735), (-0.28, 0.735)], -0.570, -0.545),
        box("pedestal-side-inner", "carcass", VEN, 0.55, 0.170, 0.189, 0.02, 0.735, -0.28, 0.28),
        box("pedestal-side-outer", "carcass", VEN, 0.55, 0.551, 0.570, 0.02, 0.735, -0.28, 0.28),
        box("pedestal-back", "carcass", CHIP, 0.45, 0.189, 0.551, 0.064, 0.735, 0.272, 0.28),
        box("pedestal-bottom", "carcass", CHIP, 0.45, 0.189, 0.551, 0.064, 0.080, -0.261, 0.272),
        box("plinth", "carcass", VEN, 0.35, 0.189, 0.551, 0.02, 0.064, -0.240, -0.224),
        box("modesty-panel", "top", VEN, 0.45, -0.545, 0.170, 0.30, 0.735, 0.254, 0.270),
        box("price-sticker", "carcass", LABEL, 0.9, 0.570, 0.5705, 0.50, 0.54, 0.02, 0.08, role="texture", texture=True),
    ]
    for d, g in DRAWERS.items():
        ry0, ry1 = g["runner"]
        s.append(box(f"runner-{d[7:]}", "carcass", CHIP, 0.4, 0.189, 0.201, ry0, ry1, -0.25, 0.26, repeat={"count": 2, "translation": list(ldir("carcass", (0.35, 0, 0)))}))
        fy0, fy1 = g["front"]
        by0, by1 = g["box"]
        s.append(box(f"{d}-front", d, VEN, 0.5, FX0, FX1, fy0, fy1, -0.280, -0.261))
        s.append(box(f"{d}-side", d, CHIP, 0.6, 0.1915, 0.2035, by0, by1, -0.261, 0.26, repeat={"count": 2, "translation": [0.345, 0, 0]}))
        s.append(box(f"{d}-back", d, CHIP, 0.6, 0.2035, 0.5365, by0, by1, 0.248, 0.26))
        s.append(box(f"{d}-base", d, HDF, 0.65, 0.2035, 0.5365, by0, by0 + 0.004, -0.261, 0.248, sheet=True))
        if "grip" in g:
            gy0, gy1 = g["grip"]
            s.append(box(f"{d}-grip", d, TEAK, 0.3, 0.220, 0.520, gy0, gy1, -0.298, -0.280))
    hy = DRAWERS["drawer-middle"]["handle"]
    s.append(capsule("drawer-middle-handle-post", "drawer-middle", CHROME, 0.15, (0.306, hy, -0.280), (0.306, hy, -0.300), 0.005, repeat={"count": 2, "translation": [0.128, 0, 0]}))
    s.append(capsule("drawer-middle-handle-bar", "drawer-middle", CHROME, 0.15, (0.306, hy, -0.300), (0.434, hy, -0.300), 0.005))
    s.append(box("drawer-top-escutcheon", "drawer-top", BRASS, 0.25, 0.360, 0.380, 0.646, 0.674, -0.2815, -0.280, role="texture", texture=True))
    return s


def body100():
    """🔎️ The 1:100 body: drawers as fronts on solid boxes, no grips, runners or foot recess (15 mm)."""
    s = [
        box("blanket", "blanket", FELT, 0.30, -0.62, 0.62, 0.0, 0.02, -0.32, 0.32),
        extrude_plan("top-board", "top", VEN, 0.62, CHIP_XZ, 0.735, 0.76),
        box("leg-panel", "leg-panel", VEN, 0.55, -0.570, -0.545, 0.02, 0.735, -0.28, 0.28),
        box("pedestal-side", "carcass", VEN, 0.55, 0.170, 0.189, 0.02, 0.735, -0.28, 0.28, repeat={"count": 2, "translation": list(ldir("carcass", (0.381, 0, 0)))}),
        box("pedestal-back", "carcass", CHIP, 0.45, 0.189, 0.551, 0.064, 0.735, 0.265, 0.28),
        box("pedestal-bottom", "carcass", CHIP, 0.45, 0.189, 0.551, 0.064, 0.080, -0.261, 0.265),
        box("plinth", "carcass", VEN, 0.35, 0.189, 0.551, 0.02, 0.064, -0.240, -0.224),
        box("modesty-panel", "top", VEN, 0.45, -0.545, 0.170, 0.30, 0.735, 0.254, 0.270),
    ]
    for d, g in DRAWERS.items():
        fy0, fy1 = g["front"]
        by0, by1 = g["box"]
        s.append(box(f"{d}-front", d, VEN, 0.5, FX0, FX1, fy0, fy1, -0.280, -0.261))
        s.append(box(f"{d}-box", d, CHIP, 0.6, 0.1905, 0.5495, fy0 + 0.002, by1, -0.261, 0.26))
    return s


def body200():
    """🗜️ The 1:200 body: carcass shell with 30 mm generalised sides, back and plinth block, drawers as blocks (30 mm)."""
    s = [
        box("blanket", "blanket", FELT, 0.30, -0.62, 0.62, 0.0, 0.02, -0.32, 0.32),
        box("top-board", "top", VEN, 0.62, -0.60, 0.60, 0.735, 0.76, -0.30, 0.30),
        box("leg-panel", "leg-panel", VEN, 0.55, -0.570, -0.545, 0.02, 0.735, -0.28, 0.28),
        box("pedestal-side", "carcass", VEN, 0.55, 0.170, 0.200, 0.02, 0.735, -0.28, 0.28, repeat={"count": 2, "translation": list(ldir("carcass", (0.370, 0, 0)))}),
        box("pedestal-back", "carcass", CHIP, 0.45, 0.200, 0.540, 0.083, 0.735, 0.25, 0.28),
        box("pedestal-plinth", "carcass", VEN, 0.4, 0.200, 0.540, 0.02, 0.083, -0.28, 0.28),
        box("modesty-panel", "top", VEN, 0.45, -0.545, 0.170, 0.30, 0.735, 0.24, 0.27),
    ]
    for d, g in DRAWERS.items():
        fy0, fy1 = g["front"]
        s.append(box(f"{d}-block", d, VEN, 0.5, 0.200, 0.540, fy0, fy1, -0.280, 0.25))
    return s


def body500():
    """🧱️ The 1:500 body: top, leg panel, modesty panel, one pedestal block and the three drawer blocks sliding out of it (75 mm)."""
    s = [
        box("blanket", "blanket", FELT, 0.30, -0.62, 0.62, 0.0, 0.02, -0.32, 0.32),
        box("top-board", "top", VEN, 0.62, -0.60, 0.60, 0.735, 0.76, -0.30, 0.30),
        box("leg-panel", "leg-panel", VEN, 0.55, -0.570, -0.545, 0.02, 0.735, -0.28, 0.28),
        box("pedestal", "carcass", VEN, 0.5, 0.170, 0.570, 0.02, 0.735, -0.28, 0.28),
        box("modesty-panel", "top", VEN, 0.45, -0.545, 0.170, 0.30, 0.735, 0.245, 0.270),
    ]
    for d, g in DRAWERS.items():
        fy0, fy1 = g["front"]
        s.append(box(f"{d}-block", d, VEN, 0.5, FX0, FX1, fy0, fy1, -0.2815, 0.20))
    return s


def thermal_list(scale, body):
    """🌡️ Thermal list: references for desk top, carcass and blanket, own front solids that close the 3 mm shadow gaps of the drawer fronts."""
    reg = {"blanket": BLA, "top-board": TOPR}
    out = []
    for s in body:
        if s.get("texture"):
            continue
        i = s["id"]
        if i in reg:
            out.append({"id": f"th-{i}", "of": i, "region": reg[i]})
        elif i.startswith(("leg-panel", "pedestal", "plinth", "modesty")):
            out.append({"id": f"th-{i}", "of": i, "region": CAR})
    depth = {"50": 0.019, "100": 0.019, "200": 0.030, "500": 0.075}[scale]
    spans = {"drawer-bottom": (0.080, 0.4045), "drawer-middle": (0.4045, 0.5905), "drawer-top": (0.5905, 0.735)}
    xs = (0.189, 0.551) if scale in ("50", "100") else (0.200, 0.540)
    if scale == "500":
        xs = (FX0, FX1)
    for d, (y0, y1) in spans.items():
        if scale == "100":
            out.append({"id": f"th-{d}-front", "of": f"{d}-front", "region": FRO})
            continue
        t = box(f"th-{d}-front", d, VEN, 0.5, xs[0], xs[1], y0, y1, -0.280, -0.280 + depth)
        t["region"] = FRO
        out.append(t)
    if scale == "50":
        for d in ("drawer-bottom", "drawer-top"):
            out.append({"id": f"th-{d}-grip", "of": f"{d}-grip", "region": FRO})
        out.append({"id": "th-handle-post", "of": "drawer-middle-handle-post", "region": FRO})
        out.append({"id": "th-handle-bar", "of": "drawer-middle-handle-bar", "region": FRO})
    return out


def structure_list(scale, body, masses):
    """🏗️ Structure list: the panels and boards as members (they are the load path of a panel desk), connectors and runners as joints, one mass per bone; the supports are the contacts each pose switches on (floor under the blanket, feet, hands), so there is no permanent support solid."""
    f = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    member = {"blanket": "blanket", "top-board": "top", "modesty-panel": "top", "leg-panel": "leg-panel"}
    out = []
    for s in body:
        if s.get("texture"):
            continue
        i = s["id"]
        if i in member:
            out.append({"id": f"m-{i}", "of": i, "member": member[i], "part": "member"})
        elif i.startswith(("pedestal", "plinth")):
            out.append({"id": f"m-{i}", "of": i, "member": "carcass", "part": "member"})
        elif i.startswith("runner-"):
            out.append({"id": f"j-{i}", "of": i, "member": "drawer-" + i[7:], "part": "joint"})
        elif i.startswith("drawer-") and (i.endswith(("-side", "-base", "-box", "-block", "-back"))):
            out.append({"id": f"m-{i}", "of": i, "member": i.rsplit("-", 1)[0], "part": "member"})
    c = max(0.02, f)
    for sid, bone, x0, x1 in (("j-leg-top-connector", "top", -0.545, -0.545 + c), ("j-pedestal-top-connector", "carcass", 0.170 - c, 0.170)):
        k = box(sid, bone, CHIP, 0.3, x0, x1, 0.735 - c, 0.735, -0.20, 0.20, role="detail")
        k.update({"member": bone, "part": "joint"})
        out.append(k)
    if scale != "50":
        for d, g in DRAWERS.items():
            y0 = g["box"][0] if scale == "100" else g["front"][0]
            xa, xb = (0.189, 0.1905) if scale == "100" else (0.170, 0.200)
            if scale == "500":
                xa, xb = 0.170, FX0
            k = box(f"j-{d}-runner", d, CHIP, 0.3, xb - f / 2, xb + f / 2, y0, y0 + f, -0.25, -0.25 + 4 * f, role="detail")
            k.update({"member": d, "part": "joint"})
            out.append(k)
    for bone, (kg, cx, cy) in masses.items():
        r = max(0.012, f / 2)
        out.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": CHIP, "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [cx, cy, 0.0]}, "member": bone, "part": "mass"})
    return out


DENS = {VEN: 680, TEAK: 650, CHIP: 650, HDF: 850, CHROME: 7850, BRASS: 8500, FELT: 126, LABEL: 800}


def masses_of(body):
    """⚖️ Bone masses and local centres from section x density of the 1:50 solids."""
    acc = {}
    for s in body:
        m = solid_volume(s) * DENS[s["material"]]
        c = solid_centre(s)
        a = acc.setdefault(s["bone"], [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * c[0]
        a[2] += m * c[1]
    return {b: (round(a[0], 3), round(a[1] / a[0], 4), round(a[2] / a[0], 4)) for b, a in acc.items()}


def main():
    """🚀️ Builds the document and writes it."""
    b50, b100, b200, b500 = body50(), body100(), body200(), body500()
    masses = masses_of(b50)
    total = round(sum(m[0] for m in masses.values()), 2)
    print("[DEBUG] masses", masses, "total", total)
    import importlib.util
    spec = importlib.util.spec_from_file_location("meta", HERE / "author-meta.py")
    me = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(me)
    doc = me.meta()
    a = doc["asset"]
    a["dimensions"]["massKg"] = total
    for bone in a["rig"]["bones"]:
        if bone["id"] in masses:
            kg, cx, cy = masses[bone["id"]]
            bone["mass"] = {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate", "reference": "en-312-2010"}
    spec = importlib.util.spec_from_file_location("drawings", HERE / "author-drawings.py")
    dr = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(dr)
    dr.API.update({"rectE": rectE, "polyE": polyE, "rectW": rectW, "polyW": polyW, "ellW": ellW, "ellE": ellE, "local": local, "CHIP_XZ": CHIP_XZ, "DRAWERS": DRAWERS})
    a["representations"] = dr.representations()
    bodies = {"50": b50, "100": b100, "200": b200, "500": b500}
    a["volumes"] = {}
    for sc in ("500", "200", "100", "50"):
        a["volumes"][sc] = {"status": "authored", "denominator": int(sc), "body": bodies[sc], "thermal": thermal_list(sc, bodies[sc]), "structure": structure_list(sc, bodies[sc], masses), "decisions": dr.VOLUME_DECISIONS[sc]}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] written", HERE / "🔣️.json")


if __name__ == "__main__":
    main()
