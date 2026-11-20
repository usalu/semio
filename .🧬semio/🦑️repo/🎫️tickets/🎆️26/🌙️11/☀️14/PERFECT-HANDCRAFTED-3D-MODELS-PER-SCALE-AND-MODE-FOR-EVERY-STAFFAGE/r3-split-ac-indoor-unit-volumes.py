"""🧊 Round 3, appliances/split-ac-indoor-unit: every solid of every authored scale and list, stated by hand; writes `volumes/appliances/split-ac-indoor-unit.json`.

Usage: python3 r3-split-ac-indoor-unit-volumes.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geo", os.path.join(HERE, "r3-split-ac-indoor-unit-geo.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)

SOURCE = os.path.join(HERE, "volumes", "appliances", "split-ac-indoor-unit.json")
T = 0.002
Z = 0.423


def solid(id, kind, bone="unit", material="abs-white", role="body", tone=0.9, **fields):
    """🧩 One solid with the common members first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(fields)
    return out


def box(id, lo, hi, **kw):
    """📦 A box from its lower and upper corner."""
    size = [g.r6(hi[i] - lo[i]) for i in range(3)]
    centre = [g.r6((hi[i] + lo[i]) / 2) for i in range(3)]
    return solid(id, "box", size=size, transform={"translation": centre}, **kw)


def ext(id, profile, z0, z1, **kw):
    """➡️ An extrude of a side profile along w."""
    return solid(id, "extrude", profile=[g.pt(p) for p in profile], plane="xy", **{"from": g.r6(z0), "to": g.r6(z1)}, **kw)


def th(s, region, layer=None, faces=None, tags=None):
    """🌡️ A thermal solid from a body-like solid."""
    s = dict(s)
    s["region"] = region
    if layer:
        s["layer"] = layer
    if faces:
        s["sectionFaces"] = faces
    if tags:
        s["tags"] = tags
    return s


def st(s, member, part, anchor=None):
    """🏗️ A structural solid from a body-like solid."""
    s = dict(s)
    s["member"] = member
    s["part"] = part
    if anchor:
        s["anchor"] = anchor
    return s


def ref(id, of, **kw):
    """🔗 A reference to a body solid."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def fan_axis(id, profile, **kw):
    """🌀 A revolve about the fan axis (w through the fan centre); axial coordinate = w."""
    return solid(id, "revolve", profile=[[g.r6(a), g.r6(r)] for a, r in profile], axis=[0, 0, 1], origin=[g.FAN[0], g.FAN[1], 0.0], **kw)


def pin(id, bone, x, y, z0, z1, r, **kw):
    """📌 A cylinder along w through (x, y)."""
    return solid(id, "revolve", bone=bone, profile=[[g.r6(z0), 0.0], [g.r6(z0), g.r6(r)], [g.r6(z1), g.r6(r)], [g.r6(z1), 0.0]], axis=[0, 0, 1], origin=[g.r6(x), g.r6(y), 0.0], **kw)


def ball(id, bone, p, r, **kw):
    """⚫ A sphere."""
    return solid(id, "ellipsoid", bone=bone, radii=[r, r, r], transform={"translation": [g.r6(p[0]), g.r6(p[1]), 0.0]}, **kw)


def end_caps(profile, bevel, z_bevel):
    """🔚 The two side covers: a flat slab and a rounded (tapered) rim per side."""
    if bevel is None:
        return [ext("end-cap", profile, g.INNER, g.HALF, repeat={"count": 2, "translation": [0, 0, -(g.INNER + g.HALF)]})]
    out = []
    if z_bevel > g.INNER:
        out.append(ext("end-cap", profile, g.INNER, z_bevel, repeat={"count": 2, "translation": [0, 0, -(g.INNER + z_bevel)]}))
    out.append(ext("end-cap-rim-l", profile, z_bevel, g.HALF, taperTo=[1, bevel]))
    out.append(ext("end-cap-rim-r", profile, -g.HALF, -z_bevel, taperTo=[bevel, 1]))
    return out


COIL = [(0.19, 0.1), (0.19, 0.222), (0.11, 0.276), (0.03, 0.205), (0.03, 0.17), (0.055, 0.17), (0.055, 0.198), (0.11, 0.247), (0.165, 0.21), (0.165, 0.1)]
COIL_100 = [(0.19, 0.1), (0.19, 0.222), (0.11, 0.276), (0.03, 0.2), (0.03, 0.17), (0.058, 0.17), (0.11, 0.244), (0.163, 0.207), (0.163, 0.1)]
COIL_200 = [(0.19, 0.1), (0.19, 0.225), (0.11, 0.276), (0.03, 0.2), (0.03, 0.16), (0.11, 0.235), (0.16, 0.2), (0.16, 0.1)]
LOUVRE = [(0.103, 0.024), (0.103, 0.045), (0.115, 0.058), (0.155, 0.063), (0.172, 0.05), (0.15, 0.03), (0.12, 0.018)]
VANE_COM = (0.033, -0.005)
COM = (0.1, 0.15)


def shell(scale):
    """🐚 The 2 mm ABS skin of the casing as panels whose inside faces are marked (the product has no C31 cavity flood)."""
    corner = g.arc(g.D - g.CORNER, g.H - g.CORNER, g.CORNER, 0, 90, 10)
    bezel = g.arc(g.C[0], g.C[1], g.R, g.LIP_FRONT_DEG, 0, 6)
    under = [(0.0, 0.0)] + g.arc(g.C[0], g.C[1], g.R, -90, g.LIP_REAR_DEG, 4)
    duct = g.duct(0.105)
    kw = dict(material="abs-white", tone=0.9)
    out = [th(box("th-back", (0, T, -Z), (T, g.H - T, Z), **kw), "casing", "shell", ["+u"])]
    if scale == 50:
        out += [
            th(box("th-top-rear", (0, g.H - T, -Z), (g.GRILLE[0], g.H, Z), **kw), "casing", "shell", ["-v"]),
            th(box("th-top-front", (g.GRILLE[1], g.H - T, -Z), (g.D - g.CORNER, g.H, Z), **kw), "casing", "shell", ["-v"]),
            th(box("th-grille-end", (g.GRILLE[0], g.H - T, -Z), (g.GRILLE[1], g.H, -g.INNER), repeat={"count": 2, "translation": [0, 0, Z + g.INNER]}, **kw), "casing", "shell", ["-v"]),
            th(ext("th-corner", g.band(corner, T), -Z, Z, **kw), "casing", "shell", ["-u", "-v"]),
            th(box("th-panel", (g.D - T, g.PANEL[1], -Z), (g.D, g.PANEL[2], Z), **kw), "casing", "shell", ["-u"]),
            th(ext("th-bezel", g.band(bezel + [(g.D, g.PANEL[1])], T), -Z, Z, **kw), "casing", "shell", ["-u"]),
            th(ext("th-underside", g.band(under, T), -Z, Z, **kw), "casing", "shell", ["+v"]),
            th(ext("th-duct-rear", g.band(duct[:5], T), -g.INNER, g.INNER, **kw), "casing", "shell", ["-u"]),
            th(ext("th-duct-roof", g.band(duct[4:], T), -g.INNER, g.INNER, **kw), "casing", "shell", ["+v"]),
            th(ext("th-flap-end", g.band(g.arc(g.C[0], g.C[1], g.R, g.LIP_REAR_DEG, g.LIP_FRONT_DEG, 5), T), -Z, -g.FLAP, repeat={"count": 2, "translation": [0, 0, Z + g.FLAP]}, **kw), "casing", "shell", ["-u", "+v"]),
        ]
    elif scale == 100:
        front = bezel + [(g.D, g.H - g.CORNER)] + corner[1:]
        out += [
            th(box("th-top-rear", (0, g.H - T, -Z), (g.GRILLE[0], g.H, Z), **kw), "casing", "shell", ["-v"]),
            th(box("th-top-front", (g.GRILLE[1], g.H - T, -Z), (g.D - g.CORNER, g.H, Z), **kw), "casing", "shell", ["-v"]),
            th(ext("th-front", g.band(front, T), -Z, Z, **kw), "casing", "shell", ["-u", "-v"]),
            th(ext("th-underside", g.band(under, T), -Z, Z, **kw), "casing", "shell", ["+v"]),
            th(ext("th-duct", g.band(g.duct(0.098, True), T), -g.INNER, g.INNER, **kw), "casing", "shell", ["-u", "+v"]),
        ]
    else:
        front = g.arc(g.C[0], g.C[1], g.R, g.LIP_FRONT_DEG, 0, 10) + [(g.D, g.H - g.CORNER)] + corner[1:]
        out += [
            th(box("th-top", (0, g.H - T, -Z), (g.D - g.CORNER, g.H, Z), **kw), "casing", "shell", ["-v"]),
            th(ext("th-front", g.band(front, T), -Z, Z, **kw), "casing", "shell", ["-u", "-v"]),
            th(ext("th-underside", g.band(under, T), -Z, Z, **kw), "casing", "shell", ["+v"]),
        ]
    out += [
        th(ext("th-end-l", g.outer_d(10), Z, g.HALF, **kw), "casing", "shell", ["-w"]),
        th(ext("th-end-r", g.outer_d(10), -g.HALF, -Z, **kw), "casing", "shell", ["+w"]),
    ]
    return out


def jet(scale):
    """💨 The outlet patch of the cold jet in the discharge mouth (D2)."""
    r0 = {50: 0.1, 100: 0.093, 200: 0.078}[scale]
    return th(ext("th-cold-jet", g.band(g.arc(g.C[0], g.C[1], r0 + 0.005, -79, -41, 6), 0.005), -0.385, 0.385, material="abs-grey", role="detail", tone=0.4), "casing", None, ["+u", "-u", "+v", "-v", "+w", "-w"], ["cold-jet"])


def scale50():
    """🔬 1:50: every assembly a 7.5 mm feature can show."""
    body = [
        ext("casing", g.casing_profile(4, 0.105), -g.INNER, g.INNER, tone=0.9),
        ext("front-panel", [(g.PANEL[0], g.PANEL[1] + 0.0015), (g.PANEL[0] + 0.0015, g.PANEL[1]), (g.D, g.PANEL[1]), (g.D, g.PANEL[2]), (g.PANEL[0] + 0.0015, g.PANEL[2]), (g.PANEL[0], g.PANEL[2] - 0.0015)], -g.INNER, g.INNER, tone=0.94),
        *end_caps(g.outer_d(4), 0.92, 0.41),
        box("filter", (g.GRILLE[0], g.GRILLE_FLOOR, -g.INNER), (g.GRILLE[1], g.GRILLE_FLOOR + 0.0015, g.INNER), material="abs-grey", role="texture", tone=0.3, texture=True),
        box("grille-bar", (0.03675, g.GRILLE_FLOOR, -g.INNER), (0.04425, 0.2885, g.INNER), material="abs-grey", role="detail", tone=0.8, repeat={"count": 5, "translation": [0.021, 0, 0]}),
        box("grille-rib", (g.GRILLE[0], g.GRILLE_FLOOR, -0.36375), (g.GRILLE[1], 0.2885, -0.35625), material="abs-grey", role="detail", tone=0.8, repeat={"count": 9, "translation": [0, 0, 0.09]}),
        ext("louvre", LOUVRE, -0.361, -0.359, material="abs-grey", role="detail", tone=0.6, texture=True, repeat={"count": 17, "translation": [0, 0, 0.045]}),
        ext("vane", g.vane_profile(0.008, 0.004), -g.FLAP, g.FLAP, bone="vane", material="abs-grey", tone=0.86),
        box("display-window", (0.2085, 0.12, 0.32), (0.2108, 0.155, 0.38), material="display", role="glazing", tone=0.12),
    ]
    thermal = shell(50) + [
        th(box("th-grille", (g.GRILLE[0], 0.2865, -g.INNER), (g.GRILLE[1], 0.2885, g.INNER), material="abs-grey", tone=0.5), "intake-grille", "shell", ["-v"]),
        ref("th-vane", "vane", region="discharge-vane", layer="shell"),
        th(ext("th-coil", COIL, -0.36, 0.36, material="aluminium-fin", tone=0.7), "evaporator", "fins"),
        th(ext("th-coil-sheet", COIL, 0.36, 0.363, material="aluminium-fin", tone=0.55, repeat={"count": 2, "translation": [0, 0, -0.723]}), "evaporator", "fins"),
        th(fan_axis("th-fan-cage", [(-0.33, 0.038), (0.33, 0.038), (0.33, 0.05), (-0.33, 0.05), (-0.33, 0.038)], material="san-gf", tone=0.35), "fan", "blades"),
        th(fan_axis("th-fan-disc", [(-0.3315, 0.0), (-0.3285, 0.0), (-0.3285, 0.05), (-0.3315, 0.05), (-0.3315, 0.0)], material="san-gf", tone=0.3, repeat={"count": 9, "translation": [0, 0, 0.0825]}), "fan", "blades"),
        th(fan_axis("th-fan-motor", [(-0.388, 0.0), (-0.388, 0.03), (-0.338, 0.03), (-0.338, 0.0)], material="san-gf", tone=0.2), "fan", "blades"),
        jet(50),
    ]
    structure = [
        st(ext("member-wall-plate", [(0, 0.03), (0.01, 0.03), (0.01, 0.0315), (0.0015, 0.0315), (0.0015, 0.2705), (0.012, 0.2705), (0.012, 0.26), (0.0135, 0.26), (0.0135, 0.272), (0, 0.272)], -0.32, 0.32, material="galvanised-steel", tone=0.6), "unit", "member"),
        st(ext("member-chassis", [(0.0015, 0.002), (0.095, 0.002), (0.095, 0.005), (0.0045, 0.005), (0.0045, 0.283), (0.03, 0.283), (0.03, 0.286), (0.0015, 0.286)], -0.385, 0.385, tone=0.8), "unit", "member"),
        st(ext("member-side-frame", g.scaled(g.outer_d(8), 0.97), 0.385, 0.395, holes=[[[0.035, 0.18], [0.17, 0.18], [0.17, 0.262], [0.035, 0.262]], [[0.03, 0.03], [0.07, 0.03], [0.07, 0.15], [0.03, 0.15]]], tone=0.75, repeat={"count": 2, "translation": [0, 0, -0.78]}), "unit", "member"),
        st(ext("member-drain-pan", [(0.155, 0.08), (0.197, 0.08), (0.197, 0.105), (0.193, 0.105), (0.193, 0.084), (0.159, 0.084), (0.159, 0.105), (0.155, 0.105)], -0.385, 0.385, tone=0.7), "unit", "member"),
        ref("member-vane", "vane", member="vane", part="member"),
        st(pin("joint-vane", "vane", 0, 0, g.FLAP, 0.4, 0.004, material="abs-grey", role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.792]}), "vane", "joint"),
        st(pin("support-wall-hook", "unit", 0.012, 0.27, -0.05, 0.05, 0.004, material="galvanised-steel", role="ground", tone=0.3), "unit", "support", "wall-hook"),
        st(pin("support-wall-rest", "unit", 0.004, 0.02, -0.05, 0.05, 0.004, material="abs-white", role="ground", tone=0.3), "unit", "support", "wall-rest"),
        st(ball("mass-unit", "unit", COM, 0.015, material="abs-white", role="detail", tone=0.3), "unit", "mass"),
        st(ball("mass-vane", "vane", VANE_COM, 0.0075, material="abs-grey", role="detail", tone=0.3), "vane", "mass"),
    ]
    return body, thermal, structure


def scale100():
    """🔭 1:100: 15 mm feature; grille bars, filter, louvres and the duct detail merge."""
    body = [
        ext("casing", g.casing_profile(8, 0.098, recess=False, pocket=(0.195, g.PANEL[1], g.PANEL[2]), simple=True), -g.INNER, g.INNER, tone=0.9),
        ext("front-panel", [(0.195, g.PANEL[1]), (g.D, g.PANEL[1]), (g.D, g.PANEL[2]), (0.195, g.PANEL[2])], -g.INNER, g.INNER, tone=0.94),
        ext("end-cap-l", g.outer_d(8), g.INNER, g.HALF, taperTo=[1, 0.93]),
        ext("end-cap-r", g.outer_d(8), -g.HALF, -g.INNER, taperTo=[0.93, 1]),
        box("grille-field", (g.GRILLE[0], g.H - 0.0005, -g.INNER), (g.GRILLE[1], g.H + 0.001, g.INNER), material="abs-grey", role="texture", tone=0.35, texture=True),
        ext("vane", g.vane_profile(0.015, 0.01, 6), -g.FLAP, g.FLAP, bone="vane", material="abs-grey", tone=0.86),
        box("display-window", (0.2075, 0.12, 0.32), (0.211, 0.155, 0.38), material="display", role="glazing", tone=0.12),
    ]
    thermal = shell(100) + [
        th(box("th-grille", (g.GRILLE[0], g.H - T, -g.INNER), (g.GRILLE[1], g.H, g.INNER), material="abs-grey", tone=0.5), "intake-grille", "shell", ["-v"]),
        th(box("th-grille-end", (g.GRILLE[0], g.H - T, -Z), (g.GRILLE[1], g.H, -g.INNER), material="abs-white", tone=0.9, repeat={"count": 2, "translation": [0, 0, Z + g.INNER]}), "casing", "shell", ["-v"]),
        ref("th-vane", "vane", region="discharge-vane", layer="shell"),
        th(ext("th-coil", COIL_100, -0.363, 0.363, material="aluminium-fin", tone=0.7), "evaporator", "fins"),
        th(fan_axis("th-fan", [(-0.388, 0.0), (-0.388, 0.03), (-0.33, 0.03), (-0.33, 0.05), (0.33, 0.05), (0.33, 0.0)], material="san-gf", tone=0.35), "fan", "blades"),
        jet(100),
    ]
    structure = [
        st(ext("member-wall-plate", [(0, 0.03), (0.015, 0.03), (0.015, 0.033), (0.003, 0.033), (0.003, 0.257), (0.015, 0.257), (0.015, 0.272), (0, 0.272)], -0.32, 0.32, material="galvanised-steel", tone=0.6), "unit", "member"),
        st(ext("member-chassis", [(0.003, 0.002), (0.095, 0.002), (0.095, 0.017), (0.018, 0.017), (0.018, 0.286), (0.003, 0.286)], -0.38, 0.38, tone=0.8), "unit", "member"),
        st(ext("member-side-frame", g.scaled(g.outer_d(10), 0.96), 0.38, 0.395, holes=[[[0.04, 0.185], [0.165, 0.185], [0.165, 0.258], [0.04, 0.258]]], tone=0.75, repeat={"count": 2, "translation": [0, 0, -0.775]}), "unit", "member"),
        ref("member-vane", "vane", member="vane", part="member"),
        st(pin("joint-vane", "vane", 0, 0, 0.385, 0.4, 0.0075, material="abs-grey", role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.785]}), "vane", "joint"),
        st(pin("support-wall-hook", "unit", 0.0075, 0.265, -0.05, 0.05, 0.0075, material="galvanised-steel", role="ground", tone=0.3), "unit", "support", "wall-hook"),
        st(pin("support-wall-rest", "unit", 0.0075, 0.02, -0.05, 0.05, 0.0075, material="abs-white", role="ground", tone=0.3), "unit", "support", "wall-rest"),
        st(ball("mass-unit", "unit", COM, 0.015, material="abs-white", role="detail", tone=0.3), "unit", "mass"),
        st(ball("mass-vane", "vane", VANE_COM, 0.0075, material="abs-grey", role="detail", tone=0.3), "vane", "mass"),
    ]
    return body, thermal, structure


def scale200():
    """🗺️ 1:200: 30 mm feature; the casing is one D with square side covers, the flap a 30 mm blade."""
    body = [
        ext("casing", g.casing_profile(10, 0.083, recess=False, pocket=None, simple=True), -g.INNER, g.INNER, tone=0.9),
        *end_caps(g.outer_d(10), None, None),
        box("grille-field", (g.GRILLE[0], g.H - 0.0005, -g.INNER), (g.GRILLE[1], g.H + 0.001, g.INNER), material="abs-grey", role="texture", tone=0.35, texture=True),
        ext("vane", g.vane_profile(0.03, 0.03, 8), -g.FLAP, g.FLAP, bone="vane", material="abs-grey", tone=0.86),
    ]
    thermal = shell(200) + [
        th(box("th-grille", (g.GRILLE[0], g.H, -g.INNER), (g.GRILLE[1], g.H + T, g.INNER), material="abs-grey", tone=0.5), "intake-grille", "shell", ["-v"]),
        ref("th-vane", "vane", region="discharge-vane", layer="shell"),
        th(ext("th-coil", COIL_200, -0.363, 0.363, material="aluminium-fin", tone=0.7), "evaporator", "fins"),
        th(fan_axis("th-fan", [(-0.385, 0.0), (-0.385, 0.05), (0.33, 0.05), (0.33, 0.0)], material="san-gf", tone=0.35), "fan", "blades"),
        jet(200),
    ]
    structure = [
        st(box("member-back", (0.0, 0.002, -0.38), (0.03, 0.286, 0.38), material="galvanised-steel", tone=0.7), "unit", "member"),
        st(ext("member-side-frame", g.scaled(g.outer_d(12), 0.95), 0.365, 0.395, holes=[[[0.045, 0.19], [0.16, 0.19], [0.16, 0.25], [0.045, 0.25]]], tone=0.75, repeat={"count": 2, "translation": [0, 0, -0.76]}), "unit", "member"),
        ref("member-vane", "vane", member="vane", part="member"),
        st(pin("joint-vane", "vane", 0, 0, 0.37, 0.4, 0.015, material="abs-grey", role="detail", tone=0.4, repeat={"count": 2, "translation": [0, 0, -0.77]}), "vane", "joint"),
        st(pin("support-wall-hook", "unit", 0.015, 0.265, -0.05, 0.05, 0.015, material="galvanised-steel", role="ground", tone=0.3), "unit", "support", "wall-hook"),
        st(pin("support-wall-rest", "unit", 0.015, 0.02, -0.05, 0.05, 0.015, material="abs-white", role="ground", tone=0.3), "unit", "support", "wall-rest"),
        st(ball("mass-unit", "unit", COM, 0.015, material="abs-white", role="detail", tone=0.3), "unit", "mass"),
    ]
    return body, thermal, structure


DECISIONS = {
    50: [
        {"en": "Body: the casing is the side profile of a 2.5 kW high-wall unit pushed over 0.79 m, with the recessed top grille (five 7.5 mm bars, nine ribs, the dark filter under them), the pocket of the 10 mm front panel and the discharge mouth behind the flap; two side covers with a rounded rim close the ends; the flap is a curved 8 mm to 4 mm blade on the casing radius that turns about its rear edge, with 17 swing louvres in the mouth and the display window on the panel.", "de": "Körper: Das Gehäuse ist das Seitenprofil eines 2,5-kW-Wandgeräts, über 0,79 m ausgezogen, mit dem vertieften Ansauggitter oben (fünf Stäbe von 7,5 mm, neun Rippen, darunter der dunkle Filter), der Tasche der 10 mm dicken Frontblende und dem Ausblasmund hinter der Klappe; zwei Seitendeckel mit gerundetem Rand schließen die Enden; die Klappe ist ein gewölbtes Blatt von 8 bis 4 mm auf dem Gehäuseradius, das sich um seine Hinterkante dreht, dazu 17 Schwenklamellen im Mund und das Anzeigefenster auf der Blende."},
        {"en": "Thermal: the 2 mm ABS skin is built from panels (back, top strips, rounded top edge, front panel, lower bezel, underside, the walls of the discharge mouth, side covers) whose inside faces are section faces; the grille is its own plate, the flap its body solid; hidden inside are the folded evaporator coil of 25 mm with its two tube sheets and the cross-flow fan (blade cage, nine support discs, motor); the cold jet leaves through an outlet patch in the mouth.", "de": "Thermik: Die 2 mm dicke ABS-Haut besteht aus Platten (Rückwand, Deckstreifen, gerundete Oberkante, Frontblende, untere Blende, Unterseite, Wände des Ausblasmunds, Seitendeckel), deren Innenflächen Schnittflächen sind; das Gitter ist eine eigene Platte, die Klappe ihr Körper; verborgen liegen innen der gefaltete Verdampfer von 25 mm mit zwei Rohrböden und der Querstromlüfter (Schaufelkäfig, neun Stützscheiben, Motor); der kalte Strahl tritt durch eine Auslassfläche im Mund aus."},
        {"en": "Structure: the load path runs from the galvanised wall plate (folded top hook and bottom lip) into the ABS chassis (back wall and base), the two side frames (moulded plates with lightening openings above the fan and behind it) that carry coil, fan bearing and motor, and the drain-pan beam; the flap is its own member on two pivot pins; the hook is a pin and the lower rest a roller against the wall; masses sit at the centre of mass of the unit and of the flap.", "de": "Tragwerk: Der Lastpfad läuft von der verzinkten Wandplatte (gekanteter Haken oben, Lippe unten) in das ABS-Chassis (Rückwand und Boden), die zwei Seitenrahmen (Formteile mit Aussparungen über und hinter dem Lüfter), die Verdampfer, Lüfterlager und Motor tragen, und den Kondensatwannen-Träger; die Klappe ist ein eigener Stab auf zwei Drehzapfen; der Haken ist ein Festlager, die untere Anlage ein Rollenlager gegen die Wand; Massen sitzen im Schwerpunkt des Geräts und der Klappe."},
    ],
    100: [
        {"en": "Body: a new abstraction for a 15 mm feature: the casing with a 15 mm front panel and a plain discharge mouth, two tapered side covers, the grille as a dark field on the top (bars, ribs and filter merged), the flap as a 15 mm blade and the display window; the louvres are omitted.", "de": "Körper: eine neue Abstraktion für ein kleinstes Merkmal von 15 mm: das Gehäuse mit einer 15 mm dicken Frontblende und einem einfachen Ausblasmund, zwei verjüngte Seitendeckel, das Gitter als dunkles Feld oben (Stäbe, Rippen und Filter verschmolzen), die Klappe als 15 mm dickes Blatt und das Anzeigefenster; die Schwenklamellen entfallen."},
        {"en": "Thermal: the skin is merged into five panels and two side covers (front, rounded edge and lower bezel are one band; the mouth one wall), the grille a plate; inside, the evaporator is one folded band and the fan one drum with its motor; the outlet patch sits in the mouth.", "de": "Thermik: Die Haut ist zu fünf Platten und zwei Seitendeckeln zusammengefasst (Front, gerundete Kante und untere Blende als ein Band, der Mund als eine Wand), das Gitter eine Platte; innen ist der Verdampfer ein gefaltetes Band und der Lüfter eine Trommel mit Motor; die Auslassfläche sitzt im Mund."},
        {"en": "Structure: wall plate with 15 mm folds, chassis back and base 15 mm, two 15 mm side frames, the flap on 15 mm pins; the drain-pan beam merges into the side frames; hook pin, wall roller and the two masses.", "de": "Tragwerk: Wandplatte mit Kantungen von 15 mm, Chassis-Rückwand und -Boden 15 mm, zwei Seitenrahmen von 15 mm, die Klappe auf Zapfen von 15 mm; der Kondensatwannen-Träger geht in den Seitenrahmen auf; Hakenlager, Wandrolle und die zwei Massen."},
    ],
    200: [
        {"en": "Body: a new abstraction for a 30 mm feature: one D-shaped casing with square side covers, the flap as a 30 mm blade that opens a plain slot and the grille as a dark field; panel, display and the rounded rims are below the feature and merge into the casing.", "de": "Körper: eine neue Abstraktion für ein kleinstes Merkmal von 30 mm: ein D-förmiges Gehäuse mit eckigen Seitendeckeln, die Klappe als 30 mm dickes Blatt, das einen einfachen Schlitz öffnet, und das Gitter als dunkles Feld; Blende, Anzeige und gerundete Ränder liegen unter dem Merkmal und gehen im Gehäuse auf."},
        {"en": "Thermal: the skin is a back, a top, one front band, the underside and the side covers; the discharge mouth is not modelled; the evaporator is one folded band, the fan with its motor one cylinder, the grille a plate and the outlet patch sits behind the flap.", "de": "Thermik: Die Haut besteht aus Rückwand, Deckplatte, einem Frontband, Unterseite und Seitendeckeln; der Ausblasmund wird nicht modelliert; der Verdampfer ist ein gefaltetes Band, Lüfter und Motor ein Zylinder, das Gitter eine Platte, und die Auslassfläche sitzt hinter der Klappe."},
        {"en": "Structure: wall plate and chassis merge into one 30 mm back member, two 30 mm side frames, the flap on 30 mm pins, the hook pin and the wall roller at 30 mm and the mass of the unit; the flap's own mass merges into it.", "de": "Tragwerk: Wandplatte und Chassis verschmelzen zu einem Rückenstab von 30 mm, zwei Seitenrahmen von 30 mm, die Klappe auf Zapfen von 30 mm, Hakenlager und Wandrolle mit 30 mm und die Masse des Geräts; die Masse der Klappe geht darin auf."},
    ],
}


def main():
    """🚀 Builds every authored scale and writes the source."""
    volumes = {}
    for scale, build in ((50, scale50), (100, scale100), (200, scale200)):
        body, thermal, structure = build()
        volumes[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": DECISIONS[scale]}
    volumes["500"] = {"status": "unavailable", "denominator": 500}
    json.dump({"volumes": volumes}, open(SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written", SOURCE, {k: [len(v.get(x, [])) for x in ("body", "thermal", "structure")] for k, v in volumes.items()})


if __name__ == "__main__":
    main()
