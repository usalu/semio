"""🪗️ Writes the candidate asset document `🔣️.json` of `used-accordion` (a used 120-bass piano accordion in its open hard case) from the hand-chosen
parts of `author-geometry.py` and the physical scenario below. The one authoritative script: run `python3 author-accordion.py` to regenerate.

@see ../../brief.md
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("geometry", os.path.join(HERE, "author-geometry.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)
spec_t = importlib.util.spec_from_file_location("text", os.path.join(HERE, "author-text.py"))
TX = importlib.util.module_from_spec(spec_t)
spec_t.loader.exec_module(TX)
TEXT = TX.TEXT

R4 = lambda v: round(v + 0.0, 5)
SCALES = (50, 100, 200)


def t(en, de):
    """🌍️ A bilingual text."""
    return {"en": en, "de": de}


def local(bone, x, y, z):
    """📐️ World rest point to the bone-local frame (every bone rests unrotated)."""
    ox, oy, oz = G.ORIGINS[bone]
    return (R4(x - ox), R4(y - oy), R4(z - oz))


def centre(p):
    """🎯️ World centre of a part box."""
    return tuple((a + b) / 2 for a, b in (p["x"], p["y"], p["z"]))


def size(p):
    """📏️ World extents of a part box."""
    return tuple(R4(b - a) for a, b in (p["x"], p["y"], p["z"]))


def solid_of(p):
    """🧊️ The body solid of one part, bone-local."""
    base = {"id": p["id"], "bone": p["bone"], "material": p["material"], "role": p["role"], "tone": p["tone"]}
    for flag in ("sheet", "texture", "slender"):
        if p.get(flag):
            base[flag] = True
    kind = p["kind"]
    if kind == "box":
        sx, sy, sz = size(p)
        cx, cy, cz = centre(p)
        if p.get("repeat") and not p.get("black"):
            cy = p["y"][0] + sy / 2
        s = {**base, "kind": "box", "size": [sx, sy, sz], "transform": {"translation": list(local(p["bone"], cx, cy, cz))}}
        if p.get("r"):
            s["cornerRadius"] = p["r"]
        if p.get("repeat"):
            s["repeat"] = {"count": p["repeat"], "translation": [0, G.WHITE_PITCH, 0]}
        return s
    if kind == "cylinder":
        bx, by, bz = p["base"]
        axis = [0, 1, 0] if p["axis"] == "y" else [1, 0, 0]
        s = {**base, "kind": "revolve", "profile": [[0, p["radius"]], [p["length"], p["radius"]]], "axis": axis, "origin": list(local(p["bone"], bx, by, bz))}
        if p.get("repeat"):
            s["repeat"] = {"count": p["repeat"], "translation": [0, G.BUTTON_PITCH, 0]}
        return s
    if kind == "grip":
        ox, oy, oz = G.ORIGINS[p["bone"]]
        yc = 0.536
        return {**base, "kind": "capsule", "p0": [R4(-0.045 - ox), R4(yc - oy), 0], "p1": [R4(0.045 - ox), R4(yc - oy), 0], "r0": 0.009, "r1": 0.009, "lateralScale": 1.4}
    if kind == "arch":
        ox, oy, oz = G.ORIGINS[p["bone"]]
        return {**base, "kind": "extrude", "plane": "xy", "profile": arch_profile(), "from": R4(p["z"][0] - oz), "to": R4(p["z"][1] - oz)}
    if kind == "keyboard":
        ox, oy, oz = G.ORIGINS[p["bone"]]
        return {**base, "kind": "extrude", "plane": "zx", "profile": [[w, u] for u, w in keyboard_profile()], "from": R4(p["y"][0] - oy), "to": R4(p["y"][1] - oy)}
    if kind == "strap":
        ox, oy, oz = G.ORIGINS[p["bone"]]
        return {**base, "kind": "extrude", "plane": "xy", "profile": strap_profile(p), "from": R4(p["z"][0] - oz), "to": R4(p["z"][1] - oz)}
    if kind == "frame":
        ox, oy, oz = G.ORIGINS[p["bone"]]
        (x0, x1), (y0, y1), w = p["x"], p["y"], p["wall"]
        outer = [[R4(x0 - ox), R4(y0 - oy)], [R4(x1 - ox), R4(y0 - oy)], [R4(x1 - ox), R4(y1 - oy)], [R4(x0 - ox), R4(y1 - oy)]]
        hole = [[R4(x0 + w - ox), R4(y0 + w - oy)], [R4(x0 + w - ox), R4(y1 - w - oy)], [R4(x1 - w - ox), R4(y1 - w - oy)], [R4(x1 - w - ox), R4(y0 + w - oy)]]
        return {**base, "kind": "extrude", "plane": "xy", "profile": outer, "holes": [hole], "from": R4(p["z"][0] - oz), "to": R4(p["z"][1] - oz)}
    raise ValueError(kind)


def arch_profile():
    """🌉️ The handle arch of 1:200 in the handle frame (u, v)."""
    return [[0, 0], [0.02, 0], [0.02, 0.017], [0.11, 0.017], [0.11, 0], [0.13, 0], [0.13, 0.035], [0, 0.035]]


def keyboard_profile():
    """🎹️ The keyboard section (u, w) of the treble frame: key bed, white keys and the black-key ridge."""
    return [(-0.105, -0.090), (-0.105, 0.035), (-0.113, 0.035), (-0.113, 0.004), (-0.140, 0.004), (-0.140, -0.038), (-0.125, -0.038), (-0.125, -0.088), (-0.113, -0.088), (-0.113, -0.090)]


def strap_profile(p):
    """🧤️ The bass strap seen from the side (u, v of the bass frame): two end blocks and the 8 mm band standing 17 mm off the end face."""
    ox, oy, _ = G.ORIGINS[p["bone"]]
    x0, x1 = p["x"][0] - ox, p["x"][1] - ox
    y0, y1 = p["y"][0] - oy, p["y"][1] - oy
    pts = [(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y1 - 0.025), (x1 - 0.008, y1 - 0.025), (x1 - 0.008, y0 + 0.025), (x0, y0 + 0.025)]
    return [[R4(a), R4(b)] for a, b in pts]


def depth(p, view):
    """🎨️ Paint order: nearer to the viewer paints later (elevation from -z, front from +x, plan from +y)."""
    if view == "elevation":
        return round(5 - 10 * p["z"][0], 4)
    if view == "front":
        return round(5 + 10 * p["x"][1], 4)
    return round(5 + 10 * p["y"][1], 4)


def shape(p, view, geometry, **extra):
    """✏️ One filled body shape of a part."""
    s = {"id": f"{view[0]}-{p['id']}", "bone": p["bone"], "role": p["role"] if p["role"] != "body" or not p.get("texture") else "detail", "z": depth(p, view), "filled": True, "tone": p["tone"], **geometry}
    if p.get("region"):
        s["region"] = p["region"]
    s["material"] = p["material"]
    s.update(extra)
    return s


def rect(x, y, w, h, r=None):
    """⬛️ A rect geometry."""
    g = {"kind": "rect", "x": R4(x), "y": R4(y), "w": R4(w), "h": R4(h)}
    if r:
        g["r"] = r
    return g


def poly(points, closed=True):
    """🔷️ A poly geometry."""
    return {"kind": "poly", "points": [[R4(a), R4(b)] for a, b in points], "closed": closed}


def lid_solid(p):
    """🚪️ The `solid` placement of a lid part: the side profile swept along the lid's lateral axis over its rest span."""
    return {"kind": "extruded", "sweep": "w", "z0": R4(p["z"][0]), "z1": R4(p["z"][1])}


def shapes_of(p, scale):
    """🖊️ The three 2D drawings of one part at its scale (rest pose: elevation bone-local, front and plan world)."""
    ox, oy, oz = G.ORIGINS[p["bone"]]
    (x0, x1), (y0, y1), (z0, z1) = p["x"], p["y"], p["z"]
    out = {"elevation": [], "front": [], "plan": []}
    kind = p["kind"]
    if p["bone"] == "lid":
        prof = rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)
        for view in out:
            q = dict(p, tone=0.42) if view == "plan" and p["id"] == "lid-panel" else p
            out[view].append(shape(q, view, prof, solid=lid_solid(p)))
        return out
    if p.get("draw") is False or not p.get("body", True):
        return out
    if kind == "drawing":
        out["elevation"].append(shape(dict(p, region="case-lining", tone=0.42, material="case-plush", z=(0.117, z1)), "elevation", rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)))
        if scale == 100:
            w = p["inner"]
            out["elevation"].append({"id": "e-case-opening", "bone": p["bone"], "role": "detail", "z": depth(dict(p, z=(0.117, z1)), "elevation") + 0.001, "filled": False, "tone": 0.2, **rect(w, w, x1 - x0 - 2 * w, y1 - y0 - 2 * w)})
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, y1 - y0)))
        out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0)))
        return out
    if p.get("texture"):
        out["elevation"].append(shape(p, "elevation", rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)))
        return out
    if p["id"] == "grille":
        out["elevation"].append(shape(p, "elevation", rect(x0 - ox, y0 - oy, x1 - x0, y1 - y0)))
        return out
    if kind == "box":
        r = p.get("r") if scale < 200 else None
        if p.get("black"):
            yb = y0 + (len(p["black"]) - 1) * G.WHITE_PITCH + (y1 - y0)
            out["elevation"].append(shape(dict(p, role="detail"), "elevation", rect(x0 - ox, y0 - oy, x1 - x0, yb - y0)))
            out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, yb - y0)))
            out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0)))
            return out
        yb = y1
        if p.get("repeat"):
            yb = y0 + (p["repeat"] - 1) * G.WHITE_PITCH + (y1 - y0)
        out["elevation"].append(shape(p, "elevation", rect(x0 - ox, y0 - oy, x1 - x0, yb - y0, r)))
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, yb - y0, r)))
        out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0, r)))
        return out
    if kind == "cylinder":
        bx, by, bz = p["base"]
        rr, L = p["radius"], p["length"]
        if p["axis"] == "y":
            out["elevation"].append(shape(p, "elevation", rect(bx - rr - ox, by - oy, 2 * rr, L)))
            out["front"].append(shape(p, "front", rect(bz - rr, by, 2 * rr, L)))
            out["plan"].append(shape(p, "plan", {"kind": "ellipse", "cx": R4(bx), "cy": R4(bz), "rx": rr, "ry": rr}))
            return out
        n = p.get("repeat", 1)
        ytop = by + (n - 1) * G.BUTTON_PITCH
        out["elevation"].append(shape(p, "elevation", rect(bx - ox, by - rr - oy, L, ytop - by + 2 * rr)))
        out["plan"].append(shape(p, "plan", rect(bx, bz - rr, L, 2 * rr)))
        if n == 1 or scale == 50:
            for k in range(n):
                q = dict(p, id=f"{p['id']}-{k + 1}" if n > 1 else p["id"])
                q["role"] = "detail"
                out["front"].append(shape(q, "front", {"kind": "ellipse", "cx": R4(bz), "cy": R4(by + k * G.BUTTON_PITCH), "rx": rr, "ry": rr}))
        return out
    if kind == "grip":
        yc, rr = 0.536, 0.009
        out["elevation"].append(shape(p, "elevation", {"kind": "capsule", "x0": R4(-0.045 - ox), "y0": R4(yc - oy), "x1": R4(0.045 - ox), "y1": R4(yc - oy), "r0": rr, "r1": rr}))
        out["front"].append(shape(p, "front", {"kind": "ellipse", "cx": R4((z0 + z1) / 2), "cy": yc, "rx": R4((z1 - z0) / 2), "ry": rr}))
        out["plan"].append(shape(p, "plan", rect(-0.054, z0, 0.108, z1 - z0, R4((z1 - z0) / 2))))
        return out
    if kind == "arch":
        out["elevation"].append(shape(p, "elevation", poly(arch_profile())))
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, y1 - y0)))
        out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0)))
        return out
    if kind == "keyboard" and scale > 50:
        out["elevation"].append(shape(p, "elevation", rect(-0.140, y0 - oy, 0.035, y1 - y0)))
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, y1 - y0)))
        out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0)))
        return out
    if kind == "keyboard":
        q = dict(p, id="keyboard-white", tone=0.82)
        out["elevation"].append(shape(q, "elevation", rect(-0.125, y0 - oy, 0.020, y1 - y0)))
        q = dict(p, id="keyboard-black", tone=0.1, z=(-0.038, 0.004))
        out["elevation"].append(shape(q, "elevation", rect(-0.140, y0 - oy, 0.015, y1 - y0)))
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, y1 - y0)))
        out["plan"].append(shape(p, "plan", poly([(u + ox, w) for u, w in keyboard_profile()])))
        return out
    if kind == "strap":
        out["elevation"].append(shape(p, "elevation", poly([tuple(v) for v in strap_profile(p)])))
        out["front"].append(shape(p, "front", rect(z0, y0, z1 - z0, y1 - y0)))
        out["plan"].append(shape(p, "plan", rect(x0, z0, x1 - x0, z1 - z0)))
        return out
    raise ValueError(kind)


def details(scale):
    """🖋️ Detail strokes and marks of use that the parts do not carry: bellows folds and worn corners, key joints, scuffs, the old dealer sticker."""
    el, fr, pl = [], [], []
    if scale == 200:
        return el, fr, pl
    folds = (0.004, 0.026, 0.048, 0.070, 0.092) if scale == 50 else (0.004, 0.048, 0.092)
    for k in range(1, G.SEGMENTS + 1):
        inset = 0.001 * (G.SEGMENTS - k)
        bone = f"bellows-{k}"
        oy = G.ORIGINS[bone][1]
        ya, yb = 0.034 + inset - oy, 0.477 - inset - oy
        zb = round(5 - 10 * (-0.089 + inset), 4) + 0.001
        for n, u in enumerate(folds):
            el.append({"id": f"e-fold-{k}-{n + 1}", "bone": bone, "role": "detail", "z": zb, "filled": False, "tone": 0.62, "kind": "poly", "points": [[u, R4(ya)], [u, R4(yb)]], "closed": False})
            if scale == 50 and n == 0:
                for end, (va, vb) in (("low", (ya, ya + 0.012)), ("high", (yb - 0.012, yb))):
                    el.append({"id": f"e-worn-corner-{k}-{n + 1}-{end}", "bone": bone, "role": "detail", "z": zb + 0.001, "filled": True, "tone": 0.5, "kind": "rect", "x": R4(u - 0.004), "y": R4(va), "w": 0.008, "h": R4(vb - va), "region": "bellows", "material": "bellows-card"})
    if scale == 50:
        ox, oy, _ = G.ORIGINS["treble"]
        for i in range(1, 24):
            v = G.WHITE_V0 + i * G.WHITE_PITCH - 0.00025 - oy
            el.append({"id": f"e-key-joint-{i}", "bone": "treble", "role": "detail", "z": round(5 - 10 * -0.088, 4) + 0.002, "filled": False, "tone": 0.3, "kind": "poly", "points": [[-0.125, R4(v)], [-0.113, R4(v)]], "closed": False})
        el.append({"id": "e-grille-frame", "bone": "treble", "role": "detail", "z": round(5 - 10 * -0.098, 4) + 0.002, "filled": False, "tone": 0.3, "kind": "rect", "x": -0.095, "y": R4(0.073 - oy), "w": 0.073, "h": 0.365, "r": 0.004})
        for n, (u, v0, v1) in enumerate(((-0.08, 0.10, 0.17), (-0.06, 0.25, 0.33), (-0.04, 0.12, 0.36))):
            el.append({"id": f"e-grille-bar-{n + 1}", "bone": "treble", "role": "detail", "z": round(5 - 10 * -0.098, 4) + 0.003, "filled": False, "tone": 0.35, "kind": "poly", "points": [[u, R4(v0 - oy)], [u, R4(v1 - oy)]], "closed": False})
        bx, by, _ = G.ORIGINS["bass"]
        for n, pts in enumerate((((0.02, 0.36), (0.05, 0.40), (0.07, 0.39)), ((0.09, 0.10), (0.11, 0.14)), ((0.03, 0.20), (0.06, 0.21), (0.08, 0.25)))):
            el.append({"id": f"e-crazing-{n + 1}", "bone": "bass", "role": "detail", "z": round(5 - 10 * -0.095, 4) + 0.002, "filled": False, "tone": 0.6, "kind": "poly", "points": [[a, R4(b - by)] for a, b in pts], "closed": False})
        cx = G.ORIGINS["case"][0]
        for n, (u, v) in enumerate(((0.01, 0.03), (0.49, 0.47))):
            el.append({"id": f"e-case-scuff-{n + 1}", "bone": "case", "role": "detail", "z": round(5 - 10 * -0.11, 4) + 0.002, "filled": False, "tone": 0.7, "kind": "poly", "points": [[u, v], [u, R4(v + 0.02)]], "closed": False})
        fr.append({"id": "f-dealer-sticker", "bone": "case", "role": "detail", "z": round(5 + 10 * 0.25, 4) + 0.002, "filled": True, "tone": 0.92, "kind": "rect", "x": 0.03, "y": 0.30, "w": 0.06, "h": 0.045, "region": "case-shell", "material": "case-tolex"})
        fr.append({"id": "f-sticker-text", "bone": "case", "role": "detail", "z": round(5 + 10 * 0.25, 4) + 0.003, "filled": False, "tone": 0.3, "kind": "poly", "points": [[0.038, 0.33], [0.082, 0.33]], "closed": False})
        pl.append({"id": "p-lid-plush-edge", "bone": "lid", "role": "detail", "z": round(5 + 10 * 0.025, 4) + 0.002, "filled": False, "tone": 0.6, "kind": "rect", "x": 0.012, "y": -0.0125, "w": 0.476, "h": 0.025, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.608, "z1": -0.122}})
    return el, fr, pl


PENS = {50: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 100: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, 200: {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}


def representation(scale, parts):
    """🗺️ The authored representation of one scale."""
    geo = {"elevation": [], "front": [], "plan": []}
    for p in parts:
        for view, shapes in shapes_of(p, scale).items():
            geo[view].extend(shapes)
    el, fr, pl = details(scale)
    geo["elevation"] += el
    geo["front"] += fr
    geo["plan"] += pl
    for view in geo:
        geo[view].sort(key=lambda s: s["z"])
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": geo, "decisions": TEXT["representation"][scale]}


def structure_list(scale, parts):
    """🏗️ The structural idealisation of one scale: members along the bones, the movable joints and the masses; every support is a pose contact at its anchor, switched by poses and clips, so `structure.supports` stays empty."""
    f = G.FEATURE[scale]
    ids = {p["id"] for p in parts}
    out = []
    member_parts = {"case": [i for i in ("case-bottom", "case-top", "case-end-treble", "case-end-bass", "case-frame", "case-back") if i in ids], "lid": ["lid-panel"], "handle": [i for i in ("handle-grip", "handle-lug-treble", "handle-lug-bass", "handle-block") if i in ids], "treble": ["treble-box"], **{f"bellows-{k}": [f"bellows-segment-{k}"] for k in range(1, G.SEGMENTS + 1)}, "bass": ["bass-box"]}
    for bone, pids in member_parts.items():
        for pid in pids:
            out.append({"id": f"st-{pid}", "of": pid, "member": bone, "part": "member"})
    out.append({"id": "joint-lid-hinge", "kind": "capsule", "bone": "lid", "material": "corner-steel", "role": "body", "tone": 0.5, "p0": [0.01, 0, 0], "p1": [0.49, 0, 0], "r0": 0.004, "r1": 0.004, "slender": True, "member": "lid", "part": "joint"})
    ox, oy, oz = G.ORIGINS["treble"]
    lo, hi = (0.010, G.Y_BOX0) if scale == 50 else (0.010, G.Y_FOOT) if scale == 100 else (G.Y_FOOT, G.Y_FOOT)
    out.append({"id": "joint-case-seat", "kind": "box", "bone": "treble", "material": "case-plush", "role": "body", "tone": 0.5, "size": [max(0.03, 2 * f), R4(max(hi - lo, f)), 0.14], "transform": {"translation": [-0.055, R4((lo + hi) / 2 - oy), 0]}, "member": "treble", "part": "joint"})
    for k in range(1, G.SEGMENTS + 1):
        out.append({"id": f"joint-bellows-{k}", "kind": "box", "bone": f"bellows-{k}", "material": "bellows-card", "role": "body", "tone": 0.5, "size": [2 * f, 0.03, 0.03], "transform": {"translation": [f, 0, 0]}, "member": f"bellows-{k}", "part": "joint"})
    out.append({"id": "joint-bass", "kind": "box", "bone": "bass", "material": "celluloid", "role": "body", "tone": 0.5, "size": [2 * f, 0.03, 0.03], "transform": {"translation": [f, 0, 0]}, "member": "bass", "part": "joint"})
    for b in BONES:
        m = b.get("mass")
        if m is None:
            continue
        s = max(f, 0.012)
        out.append({"id": f"mass-{b['id']}", "kind": "box", "bone": b["id"], "material": "celluloid", "role": "body", "tone": 0.5, "size": [s, s, s], "transform": {"translation": [m["comX"], m["comY"], MASS_W.get(b["id"], 0)]}, "member": b["id"], "part": "mass"})
    return out


def volume(scale, parts):
    """🧊️ The authored volume of one scale."""
    body = [solid_of(p) for p in parts if p["kind"] != "drawing" and p.get("body", True)]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in parts if p["kind"] != "drawing" and p.get("body", True) and p.get("thermal", True)]
    thermal += [{**solid_of(dict(p, id=f"th-{p['id']}")), "region": p["region"]} for p in parts if not p.get("body", True)]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure_list(scale, parts), "decisions": TEXT["volume"][scale]}


def bone(bid, parent, title, x, y, length, joint, kg=None, com=(0, 0), z=None):
    """🦴️ One rig bone resting unrotated."""
    b = {"id": bid, "parent": parent, "title": title, "x": R4(x), "y": R4(y), "angleDeg": 0, "lengthM": length, "joint": joint}
    if z is not None:
        b["z"] = R4(z)
    if kg is not None:
        b["mass"] = {"kg": kg, "comX": R4(com[0]), "comY": R4(com[1]), "distribution": "point", "source": "authored-estimate", "reference": "richter-1990"}
    return b


O = G.ORIGINS
BONES = [
    bone("case", None, t("Hard case tub with lining", "Koffer-Unterteil mit Auskleidung"), O["case"][0], 0, 0.5, {"type": "fixed"}, 3.9, (0.25, 0.22)),
    bone("lid", "case", t("Front lid on its piano hinge", "Vorderer Deckel am Klavierband"), 0, 0.0125, 0.5, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -90, "twistMaxDeg": 0}, 1.3, (0.25, 0), z=O["lid"][2]),
    bone("handle", "case", t("Carrying handle", "Tragegriff"), O["handle"][0] - O["case"][0], G.CASE_TOP, 0.13, {"type": "fixed"}, 0.1, (0.065, 0.02), z=O["handle"][2]),
    bone("treble", "case", t("Treble box with keyboard and shoulder straps", "Diskantteil mit Tastatur und Schulterriemen"), O["treble"][0] - O["case"][0], G.Y_MID, 0.105, {"type": "prismatic", "axisDeg": 36.869897645844, "minM": 0, "maxM": 1.0}, 5.6, (-0.055, 0.0)),
    *[bone(f"bellows-{k}", "treble" if k == 1 else f"bellows-{k - 1}", t(f"Bellows segment {k} of 4 (five folds)", f"Balgabschnitt {k} von 4 (fünf Falten)"), 0 if k == 1 else G.PITCH_CLOSED, 0, 0.03, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.08}, 0.175, (0.055, 0.0)) for k in range(1, G.SEGMENTS + 1)],
    bone("bass", "bellows-4", t("Bass box with 120 buttons and bass strap", "Bassteil mit 120 Knöpfen und Bassriemen"), G.PITCH_CLOSED, 0, 0.14, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.08}, 3.9, (0.07, 0.0)),
]
MASS_W = {"case": 0.0125, "lid": -0.2425}

FEET = [("case-foot-treble-front", 0.02, -0.09), ("case-foot-treble-back", 0.02, 0.115), ("case-foot-bass-front", 0.48, -0.09), ("case-foot-bass-back", 0.48, 0.115)]
ANCHORS = [
    *[{"id": i, "bone": "case", "x": u, "y": 0.0, "z": w, "kind": "contact", "title": t(f"Case corner on the floor ({i.split('-')[2]} end, {i.split('-')[3]})", f"Kofferecke auf dem Boden ({'Diskant' if 'treble' in i else 'Bass'}seite, {'vorn' if 'front' in i else 'hinten'})"), "support": "contact", "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.5} for i, u, w in FEET],
    {"id": "lid-rest", "bone": "lid", "x": 0.25, "y": -0.0125, "z": -0.46, "kind": "contact", "title": t("Far edge of the open lid on the floor", "Vordere Kante des offenen Deckels auf dem Boden"), "support": "contact", "footprintLengthM": 0.5, "footprintWidthM": 0.03, "frictionCoefficient": 0.5},
    {"id": "handle-grip", "bone": "handle", "x": 0.065, "y": 0.035, "z": 0.0, "kind": "support", "title": t("Hand on the handle (carrying)", "Hand am Griff (Tragen)"), "support": "pin"},
    {"id": "thigh", "bone": "bass", "x": 0.08, "y": R4(G.Y_FOOT - G.Y_MID), "z": 0.0, "kind": "contact", "title": t("Bass foot on the player's left thigh", "Bassfuß auf dem linken Oberschenkel"), "support": "contact", "footprintLengthM": 0.06, "footprintWidthM": 0.12, "frictionCoefficient": 0.6},
    {"id": "strap-hook-right", "bone": "treble", "x": -0.0755, "y": R4(G.Y_BOX1 + 0.010 - G.Y_MID), "z": 0.085, "kind": "support", "title": t("Right shoulder strap at its top hook", "Rechter Schulterriemen am oberen Haken"), "support": "pin"},
    {"id": "strap-hook-left", "bone": "treble", "x": -0.0255, "y": R4(G.Y_BOX1 + 0.010 - G.Y_MID), "z": 0.085, "kind": "support", "title": t("Left shoulder strap at its top hook", "Linker Schulterriemen am oberen Haken"), "support": "pin"},
    {"id": "keys-press", "bone": "treble", "x": -0.137, "y": 0.05, "z": -0.04, "kind": "load", "title": t("Right-hand fingers on the keys", "Finger der rechten Hand auf den Tasten")},
    {"id": "bass-strap-hand", "bone": "bass", "x": 0.165, "y": 0.0, "z": 0.0575, "kind": "load", "title": t("Left hand under the bass strap", "Linke Hand unter dem Bassriemen")},
]
SUPPORTS = [a["id"] for a in ANCHORS if a["kind"] in ("contact", "support")]

REGIONS = ["case-shell", "case-lid", "case-lining", "treble-casing", "grille", "keyboard", "bellows", "bass-casing", "bass-buttons", "straps"]


def main():
    """🖨️ Assemble and write the document."""
    scale_parts = {s: G.parts_of(s) for s in SCALES}
    asset = {
        "id": "used-accordion",
        "category": "objects",
        "title": t("Used Piano Accordion", "Gebrauchtes Akkordeon"),
        "summary": TEXT["summary"],
        "tags": ["used", "instrument", "music", "second-hand", "folk", "keyboard", "accordion", "free-reed", "bellows", "celluloid", "hard-case"],
        "provenance": TEXT["provenance"],
        "dimensions": dimensions(scale_parts[50]),
        "materials": TEXT["materials"],
        "rig": {"kind": "mechanical", "root": "case", "bones": BONES},
        "anchors": ANCHORS,
        "representations": {**{str(s): representation(s, scale_parts[s]) for s in SCALES}, "500": TEXT["unavailable500"]},
        "symbols": [],
        "poses": TEXT["poses"](SUPPORTS),
        "clips": TEXT["clips"](SUPPORTS),
        "thermal": TEXT["thermal"],
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": TEXT["structureAssumptions"], "clinical": False},
        "volumes": {**{str(s): volume(s, scale_parts[s]) for s in SCALES}, "500": {"status": "unavailable", "denominator": 500}},
        "presentation": {"defaultPose": "stored", "defaultClip": "bellows-cycle", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: asset[k] for k in order}}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as out:
        json.dump(doc, out, ensure_ascii=False, indent=1)
        out.write("\n")
    print("[DEBUG] dimensions", asset["dimensions"], "solids", {s: [len(asset["volumes"][str(s)][l]) for l in ("body", "thermal", "structure")] for s in SCALES})


def dimensions(parts):
    """🧰️ The envelope of the rest pose from the 1:50 parts, and the total mass of the bones."""
    ext = [(min(p[a][0] for p in parts), max(p[a][1] for p in parts)) for a in ("x", "y", "z")]
    return {"heightM": R4(ext[1][1] - ext[1][0]), "widthM": R4(ext[0][1] - ext[0][0]), "depthM": R4(ext[2][1] - ext[2][0]), "massKg": round(sum(b["mass"]["kg"] for b in BONES), 3)}


if __name__ == "__main__":
    main()
