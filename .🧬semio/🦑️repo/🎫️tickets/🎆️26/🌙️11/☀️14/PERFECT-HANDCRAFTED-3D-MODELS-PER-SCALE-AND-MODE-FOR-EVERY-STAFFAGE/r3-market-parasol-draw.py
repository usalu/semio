"""✏️ The market parasol's 2D drawings of every scale and view, redrawn by hand as the true orthographic views of the corrected linkage (ruling E1): clevis foot, two-diameter mast with coupling collar and crank, crown hub with top cap, runner, eight 45-degree ribs with tip caps, eight stretchers and eight flat gores as extruded solids on their ribs.

See ./r3-objects-market-parasol.md and ./design.md (C9, C13, C24).
"""
import math

ALU = {"region": "frame-alu", "material": "aluminium-anodised"}
CONC = {"region": "base-concrete", "material": "concrete"}
CLOTH = {"region": "canopy-top", "material": "polyester-canvas"}


def r6(x):
    return round(x, 6)


def pts(seq):
    return [[r6(a), r6(b)] for a, b in seq]


def shape(sid, bone, role, z, tone, geom, mat, filled=True, pen=None, **extra):
    """🧩 One shape with the common fields in the catalog's member order."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s.update(mat)
    if pen is not None:
        s["penMm"] = pen
    s.update(extra)
    return s


def rect(x, y, w, h, r=None):
    g = {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}
    if r:
        g["r"] = r
    return g


def poly(points, closed=True):
    return {"kind": "poly", "points": pts(points), "closed": closed}


def path(d):
    return {"kind": "path", "d": d}


def ellipse(cx, cy, rx, ry=None):
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry if ry is not None else rx)}


def member(x0, x1, section, y=0.0, z=0.0):
    return {"kind": "member", "from": {"x": r6(x0), "y": r6(y), "z": r6(z)}, "to": {"x": r6(x1), "y": r6(y), "z": r6(z)}, "section": section}


def arc_pts(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def d_of(rings):
    return " ".join("M " + " L ".join(f"{r6(x)} {r6(y)}" for x, y in ring) + " Z" for ring in rings)


def gore_profile(k, r):
    """🔺 The gore of rib r in its rib's (u, w) plane: apex, own fabric corner, the next rib's fabric corner (open pose)."""
    fr = k.OPEN[r]["frame"]
    lf = k.RIB_L - 0.02
    far = k.to_local(fr, [0, 0, 0], k.mul(k.OPEN[k.nxt(r)]["dir"], lf))
    return [(0.0, 0.0), (lf, 0.0), (far[0], far[2])], far


def gore_shape(k, r, z, v0, v1, pen):
    prof, _ = gore_profile(k, r)
    return shape(f"panel-{r}", f"rib-{r}", "body", z, 0.82, poly(prof), CLOTH, pen=pen, solid={"kind": "extruded", "sweep": "v", "z0": r6(v0), "z1": r6(v1)})


def depth_order(k, view):
    """🔢 Ribs sorted far to near for a viewpoint (elevation from -z, front from +x, plan from +y)."""
    def near(r):
        a = math.radians(k.AZ[r] + 22.5)
        x, z = math.cos(a), -math.sin(a)
        return -z if view == "elevation" else x
    return sorted(k.RIBS, key=near)


def canopy(k, view, v0, v1, pen, z0=2.0):
    order = depth_order(k, view) if view != "plan" else k.RIBS
    return [gore_shape(k, r, round(z0 + 0.01 * i, 3), v0, v1, pen) for i, r in enumerate(order)]


def ribs(k, view, section, z, tips=None, pen=None):
    out = []
    for r in k.RIBS:
        out.append(shape(f"rib-{r}-rod", f"rib-{r}", "body", z, 0.35, member(0.03, k.RIB_L - 0.03, section), ALU, pen=pen))
        if tips:
            out.append(shape(f"rib-{r}-tip", f"rib-{r}", "detail", round(z + 0.05, 3), 0.2, member(k.RIB_L - 0.032, k.RIB_L, {"kind": "circle", "d": tips}), ALU))
    return out


def stretchers(k, d, z, pen=None):
    return [shape(f"stretch-{r}", f"stretch-{r}", "body", z, 0.4, member(0.04, k.STRETCH_L - 0.012, {"kind": "circle", "d": d}), ALU, pen=pen) for r in k.RIBS]


def mast_elev(k, lower, upper, collar=True):
    """📏 Mast outline in the mast frame (u up from the pivot, v = -x): lower tube, coupling, upper tube."""
    top = k.MAST_L
    if not collar:
        return poly([(0.03, -lower), (top, -lower), (top, lower), (0.03, lower)])
    return poly([(0.03, -lower), (0.79, -lower), (0.79, -upper), (top, -upper), (top, upper), (0.79, upper), (0.79, lower), (0.03, lower)])


def mast_front(k, lower, upper, foot):
    top = k.CROWN_Y
    return poly([(-foot, 0.236), (foot, 0.236), (foot, 0.31), (lower, 0.31), (lower, 1.05), (upper, 1.05), (upper, top), (-upper, top), (-upper, 1.05), (-lower, 1.05), (-lower, 0.31), (-foot, 0.31)])


def hub_rings(cap_r, hub_r, scale):
    """👑 Crown hub with its top cap and finial (crown frame, y up from the crown)."""
    if scale == 50:
        cap = [(-hub_r, -0.05), (hub_r, -0.05), (hub_r, 0.012), (cap_r, 0.012)] + arc_pts(0, 0.012, cap_r, 0, 90, 8)[1:-1] + [(0.0, 0.064)]
        right = cap
        left = [(-x, y) for x, y in reversed(right)]
        return [right + left[1:-1]]
    ring = [(-hub_r, -0.05), (hub_r, -0.05), (hub_r, 0.012), (cap_r, 0.012), (cap_r * 0.6, 0.05), (0.012, 0.064), (0.012, 0.1125), (-0.012, 0.1125), (-0.012, 0.064), (-cap_r * 0.6, 0.05), (-cap_r, 0.012), (-hub_r, 0.012)]
    return [ring]


def finial_d():
    return "M -0.008 0.06 L 0.008 0.06 L 0.009 0.074 C 0.018 0.078 0.019 0.1 0.011 0.108 C 0.006 0.1135 -0.006 0.1135 -0.011 0.108 C -0.019 0.1 -0.018 0.078 -0.009 0.074 Z"


def runner_d(dy=0.0):
    ring = [(-0.032, -0.045), (0.032, -0.045), (0.032, 0.03), (0.036, 0.036), (0.036, 0.045), (-0.036, 0.045), (-0.036, 0.036), (-0.032, 0.03)]
    return d_of([[(x, y + dy) for x, y in ring]])


def clevis_elev():
    cheek = [(-0.035, 0.215), (0.035, 0.215)] + arc_pts(0, 0.26, 0.035, 0, 180, 12)
    return d_of([cheek])


def s50(k):
    pens = {"gore": 0.35}
    el = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.2, 0, 0.4, 0.2, 0.012), CONC, pen=0.5),
        shape("base-seam", "base", "detail", 1.1, 0.35, poly([(-0.188, 0.188), (0.188, 0.188)], False), CONC, filled=False),
        shape("base-grit", "base", "texture", 1.15, 0.3, path("M -0.15 0.06 L -0.12 0.07 M 0.05 0.04 L 0.09 0.05 M -0.04 0.11 L 0.0 0.1"), CONC, filled=False),
        shape("base-plate", "base", "body", 1.2, 0.35, rect(-0.14, 0.2, 0.28, 0.015, 0.003), ALU),
        shape("base-bolts", "base", "detail", 1.25, 0.3, path(d_of([[(x - 0.0095, 0.215), (x + 0.0095, 0.215), (x + 0.0095, 0.223), (x - 0.0095, 0.223)] for x in (-0.11, 0.11)])), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, mast_elev(k, 0.024, 0.020), ALU, pen=0.5),
        shape("mast-seam", "mast", "texture", 1.51, 0.3, poly([(0.12, 0.012), (0.72, 0.012)], False), ALU, filled=False),
        shape("mast-tilt-collar", "mast", "detail", 1.55, 0.25, rect(0.755, -0.034, 0.11, 0.068, 0.006), ALU),
        shape("mast-collar-knob", "mast", "detail", 1.56, 0.2, ellipse(0.81, 0.0, 0.009), ALU),
        shape("mast-winch", "mast", "detail", 1.55, 0.3, rect(1.075, 0.016, 0.09, 0.034, 0.006), ALU),
        shape("mast-crank", "mast", "detail", 1.56, 0.25, poly([(1.116, 0.05), (1.124, 0.05), (1.124, 0.083), (1.078, 0.083), (1.078, 0.074), (1.116, 0.074)]), ALU),
        shape("mast-crank-knob", "mast", "detail", 1.57, 0.2, rect(1.062, 0.07, 0.016, 0.045, 0.006), ALU),
        shape("base-socket", "base", "body", 1.6, 0.3, path(clevis_elev()), ALU),
        shape("mast-knuckle", "mast", "detail", 1.7, 0.2, ellipse(0, 0, 0.011), ALU),
        shape("runner-collar", "runner", "body", 1.55, 0.35, path(runner_d()), ALU),
    ]
    el += stretchers(k, 0.011, 1.4)
    el += ribs(k, "elevation", {"kind": "rect", "w": 0.01, "h": 0.016}, 1.45, tips=0.018)
    el += canopy(k, "elevation", 0.008, 0.016, pens["gore"])
    el += [
        shape("crown-hub", "crown", "body", 2.5, 0.35, path(d_of(hub_rings(0.055, 0.042, 50))), ALU, pen=0.35),
        shape("crown-finial", "crown", "body", 2.6, 0.3, path(finial_d()), ALU),
    ]
    fr = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.15, 0, 0.3, 0.2, 0.012), CONC, pen=0.5),
        shape("base-seam", "base", "detail", 1.1, 0.35, poly([(-0.138, 0.188), (0.138, 0.188)], False), CONC, filled=False),
        shape("base-plate", "base", "body", 1.2, 0.35, rect(-0.14, 0.2, 0.28, 0.015, 0.003), ALU),
        shape("base-bolts", "base", "detail", 1.25, 0.3, path(d_of([[(x - 0.0095, 0.215), (x + 0.0095, 0.215), (x + 0.0095, 0.223), (x - 0.0095, 0.223)] for x in (-0.11, 0.11)])), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, mast_front(k, 0.024, 0.020, 0.023), ALU, pen=0.5, followTip=True),
        shape("base-socket", "base", "body", 1.6, 0.3, path(d_of([[(-0.035, 0.215), (0.035, 0.215), (0.035, 0.223), (-0.035, 0.223)], [(-0.036, 0.223), (-0.028, 0.223), (-0.028, 0.295), (-0.036, 0.295)], [(0.028, 0.223), (0.036, 0.223), (0.036, 0.295), (0.028, 0.295)]])), ALU),
        shape("mast-knuckle", "mast", "detail", 1.7, 0.2, path(d_of([[(-0.042, 0.249), (-0.036, 0.249), (-0.036, 0.254), (0.036, 0.254), (0.036, 0.249), (0.042, 0.249), (0.042, 0.271), (0.036, 0.271), (0.036, 0.266), (-0.036, 0.266), (-0.036, 0.271), (-0.042, 0.271)]])), ALU),
        shape("mast-tilt-collar", "mast", "detail", 1.55, 0.25, rect(-0.034, 1.015, 0.068, 0.11, 0.006), ALU),
        shape("mast-collar-knob", "mast", "detail", 1.56, 0.2, rect(-0.061, 1.061, 0.027, 0.018, 0.006), ALU),
        shape("runner-collar", "runner", "body", 1.55, 0.35, path(runner_d(k.CROWN_Y - k.RUNNER_OPEN_H)), ALU),
    ]
    fr += stretchers(k, 0.011, 1.4)
    fr += ribs(k, "front", {"kind": "rect", "w": 0.01, "h": 0.016}, 1.45, tips=0.018)
    fr += canopy(k, "front", 0.008, 0.016, pens["gore"])
    fr += [
        shape("crown-hub", "crown", "body", 2.5, 0.35, path(d_of([[(x, y + k.CROWN_Y) for x, y in hub_rings(0.055, 0.042, 50)[0]]])), ALU, pen=0.35),
        shape("crown-finial", "crown", "body", 2.6, 0.3, path(shift_d(finial_d(), k.CROWN_Y)), ALU),
    ]
    pl = canopy(k, "plan", 0.008, 0.016, pens["gore"])
    pl += [shape(f"rib-{r}-tip", f"rib-{r}", "detail", 2.4, 0.2, member(k.RIB_L - 0.032, k.RIB_L, {"kind": "circle", "d": 0.018}), ALU) for r in k.RIBS]
    pl += [
        shape("crown-hub", "crown", "body", 2.5, 0.35, ellipse(0, 0, 0.055), ALU, pen=0.35),
        shape("crown-finial", "crown", "body", 2.6, 0.3, ellipse(0, 0, 0.0185), ALU),
    ]
    return {"elevation": el, "front": fr, "plan": pl}


def shift_d(d, dy):
    """↕️ A path string moved by dy (absolute M/L/C commands only)."""
    tok = d.split(" ")
    out, i, coord = [], 0, 0
    for t in tok:
        if t.isalpha():
            out.append(t)
            coord = 0
            continue
        v = float(t)
        out.append(str(r6(v + dy)) if coord % 2 == 1 else t)
        coord += 1
    return " ".join(out)


def s100(k):
    el = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.2, 0, 0.4, 0.2, 0.012), CONC),
        shape("base-plate", "base", "body", 1.2, 0.35, rect(-0.14, 0.2, 0.28, 0.015), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, mast_elev(k, 0.022, 0.022, collar=False), ALU),
        shape("mast-tilt-collar", "mast", "detail", 1.55, 0.25, rect(0.755, -0.034, 0.11, 0.068), ALU),
        shape("mast-winch", "mast", "detail", 1.55, 0.3, poly([(1.075, 0.016), (1.165, 0.016), (1.165, 0.05), (1.128, 0.05), (1.128, 0.086), (1.06, 0.086), (1.06, 0.115), (1.044, 0.115), (1.044, 0.07), (1.112, 0.07), (1.112, 0.05), (1.075, 0.05)]), ALU),
        shape("base-socket", "base", "body", 1.6, 0.3, path(clevis_elev()), ALU),
        shape("runner-collar", "runner", "body", 1.55, 0.35, rect(-0.036, -0.045, 0.072, 0.09), ALU),
    ]
    el += stretchers(k, 0.012, 1.4)
    el += ribs(k, "elevation", {"kind": "rect", "w": 0.01, "h": 0.016}, 1.45)
    el += canopy(k, "elevation", 0.008, 0.023, 0.3)
    el += [shape("crown-hub", "crown", "body", 2.5, 0.35, path(d_of(hub_rings(0.055, 0.042, 100))), ALU)]
    fr = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.15, 0, 0.3, 0.2, 0.012), CONC),
        shape("base-plate", "base", "body", 1.2, 0.35, rect(-0.14, 0.2, 0.28, 0.015), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, poly([(-0.023, 0.236), (0.023, 0.236), (0.023, 0.31), (0.022, 0.31), (0.022, k.CROWN_Y), (-0.022, k.CROWN_Y), (-0.022, 0.31), (-0.023, 0.31)]), ALU, followTip=True),
        shape("base-socket", "base", "body", 1.6, 0.3, rect(-0.042, 0.215, 0.084, 0.08), ALU),
        shape("mast-tilt-collar", "mast", "detail", 1.55, 0.25, rect(-0.034, 1.015, 0.068, 0.11), ALU),
        shape("runner-collar", "runner", "body", 1.55, 0.35, rect(-0.036, k.CROWN_Y - k.RUNNER_OPEN_H - 0.045, 0.072, 0.09), ALU),
    ]
    fr += stretchers(k, 0.012, 1.4)
    fr += ribs(k, "front", {"kind": "rect", "w": 0.01, "h": 0.016}, 1.45)
    fr += canopy(k, "front", 0.008, 0.023, 0.3)
    fr += [shape("crown-hub", "crown", "body", 2.5, 0.35, path(d_of([[(x, y + k.CROWN_Y) for x, y in hub_rings(0.055, 0.042, 100)[0]]])), ALU)]
    pl = canopy(k, "plan", 0.008, 0.023, 0.3)
    pl += [shape("crown-hub", "crown", "body", 2.5, 0.35, ellipse(0, 0, 0.055), ALU)]
    return {"elevation": el, "front": fr, "plan": pl}


def s200(k):
    el = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.2, 0, 0.4, 0.2), CONC),
        shape("base-socket", "base", "body", 1.6, 0.3, rect(-0.04, 0.2, 0.08, 0.1), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, mast_elev(k, 0.022, 0.022, collar=False), ALU),
        shape("runner-collar", "runner", "body", 1.55, 0.35, rect(-0.036, -0.045, 0.072, 0.09), ALU),
    ]
    el += canopy(k, "elevation", -0.008, 0.022, 0.25)
    el += [shape("crown-hub", "crown", "body", 2.5, 0.35, poly([(-0.045, -0.05), (0.045, -0.05), (0.045, 0.012), (0.03, 0.06), (0.015, 0.1125), (-0.015, 0.1125), (-0.03, 0.06), (-0.045, 0.012)]), ALU)]
    fr = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.15, 0, 0.3, 0.2), CONC),
        shape("base-socket", "base", "body", 1.6, 0.3, rect(-0.042, 0.2, 0.084, 0.1), ALU),
        shape("mast", "mast", "body", 1.5, 0.45, poly([(-0.022, 0.26), (0.022, 0.26), (0.022, k.CROWN_Y), (-0.022, k.CROWN_Y)]), ALU, followTip=True),
        shape("runner-collar", "runner", "body", 1.55, 0.35, rect(-0.036, k.CROWN_Y - k.RUNNER_OPEN_H - 0.045, 0.072, 0.09), ALU),
    ]
    fr += canopy(k, "front", -0.008, 0.022, 0.25)
    fr += [shape("crown-hub", "crown", "body", 2.5, 0.35, poly([(x, y + k.CROWN_Y) for x, y in [(-0.045, -0.05), (0.045, -0.05), (0.045, 0.012), (0.03, 0.06), (0.015, 0.1125), (-0.015, 0.1125), (-0.03, 0.06), (-0.045, 0.012)]]), ALU)]
    pl = canopy(k, "plan", -0.008, 0.022, 0.25)
    pl += [shape("crown-hub", "crown", "body", 2.5, 0.35, ellipse(0, 0, 0.055), ALU)]
    return {"elevation": el, "front": fr, "plan": pl}


def s500(k):
    el = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.2, 0, 0.4, 0.2), CONC),
        shape("mast", "mast", "body", 1.5, 0.45, poly([(-0.06, 0), (k.MAST_L, 0)], False), ALU, filled=False),
    ]
    el += canopy(k, "elevation", -0.03, 0.045, 0.18)
    fr = [
        shape("base-block", "base", "body", 1, 0.62, rect(-0.15, 0, 0.3, 0.2), CONC),
        shape("mast", "mast", "body", 1.5, 0.45, poly([(0, 0.2), (0, k.CROWN_Y)], False), ALU, filled=False, followTip=True),
    ]
    fr += canopy(k, "front", -0.03, 0.045, 0.18)
    hub = [(-0.045, -0.05), (0.045, -0.05), (0.045, 0.06), (0.02, 0.1125), (-0.02, 0.1125), (-0.045, 0.06)]
    el += [shape("crown-hub", "crown", "body", 2.5, 0.35, poly(hub), ALU)]
    fr += [shape("crown-hub", "crown", "body", 2.5, 0.35, poly([(x, y + k.CROWN_Y) for x, y in hub]), ALU)]
    pl = canopy(k, "plan", -0.03, 0.045, 0.18)
    return {"elevation": el, "front": fr, "plan": pl}


def geometry(k):
    """🗺️ Geometry of all four scales."""
    return {"50": s50(k), "100": s100(k), "200": s200(k), "500": s500(k)}
