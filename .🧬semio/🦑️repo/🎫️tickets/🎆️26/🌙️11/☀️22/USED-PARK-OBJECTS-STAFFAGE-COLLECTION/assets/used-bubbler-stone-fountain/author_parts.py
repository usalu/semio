"""🧱️ Solids and 2D drawings of `used-bubbler-stone-fountain` for the four preset scales, all derived from the numbers of `author_geometry`.

Body solids are written in the rest frame of their bone; drawings are authored in world coordinates of the rest pose and mapped into the bone frame for the side elevation.
"""
import math

from author_geometry import (BUBBLE, BUBBLE_500, FRAME, HANDLE, HINGE, HOSE_R, LEAVES, OUTLET_R, OUTLET_U, OUTLET_V, PI, PUMP_BASE, PUMP_L, PUMP_TOP, PUMP_W,
                             R_HAUNCH_IN, R_HOLE, R_IN, R_OUT, R_PLATE, R_SETT_IN, R_SETT_OUT, SLAB_T, STONE, STONE_100, STONE_200, STONE_500, X_BORE, X_LID, X_STONE,
                             Y_FLOOR, Y_GRAVEL, Y_HAUNCH, Y_RIM, Y_SHAFT, Y_SLAB_TOP, Y_TOP, Y_WATER, band, circle, ellipse, hose_path, leaf_outline, loc, pts, r6,
                             stone_front, stone_plan, stone_side, superellipse)

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
ROT_UP = [0.5, 0.5, 0.5, 0.5]


def solid(kind, id, bone, material, role, tone, **geom):
    """🧊️ A volume solid."""
    s = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(geom)
    return s


def rev(id, bone, material, role, tone, profile, axis=(1, 0, 0), origin=(0, 0, 0), **extra):
    """🌀️ A revolve solid."""
    return solid("revolve", id, bone, material, role, tone, profile=pts(profile), axis=list(axis), origin=[r6(c) for c in origin], **extra)


def ext(id, bone, material, role, tone, profile, plane, a, b, holes=None, **extra):
    """🧱️ An extrude solid."""
    s = solid("extrude", id, bone, material, role, tone, profile=pts(profile), plane=plane, **{"from": r6(a), "to": r6(b)})
    if holes:
        s["holes"] = [pts(h) for h in holes]
    s.update(extra)
    return s


def cap(id, bone, material, role, tone, p0, p1, r, **extra):
    """💊️ A capsule solid."""
    return solid("capsule", id, bone, material, role, tone, p0=[r6(c) for c in p0], p1=[r6(c) for c in p1], r0=r6(r), r1=r6(r), **extra)


def box(id, bone, material, role, tone, size, at, **extra):
    """📦️ A box solid."""
    return solid("box", id, bone, material, role, tone, size=[r6(c) for c in size], transform={"translation": [r6(c) for c in at]}, **extra)


def ell(id, bone, material, radius, at, **extra):
    """⚫️ A mass marker."""
    return solid("ellipsoid", id, bone, material, "body", 0.5, radii=[r6(radius)] * 3, transform={"translation": [r6(c) for c in at]}, **extra)


def loop(u0, u1, r0, r1):
    """⭕️ A closed rectangular revolve loop."""
    return [(u0, r0), (u1, r0), (u1, r1), (u0, r1), (u0, r0)]


def stone_loft(id, sections, material="granite-boulder", tone=0.55):
    """🪨️ The boulder as a loft of horizontal superelliptic sections on the stone bone (u up, v = -x, w = z)."""
    secs = [{"at": [r6(h), r6(-dx), 0.0], "rx": r6(rx), "ry": r6(rz), "exponent": n, "rotation": ROT_UP} for h, dx, rx, rz, n in sections]
    return solid("loft", id, "stone", material, "body", tone, sections=secs, capStart=True, capEnd=True)


def hose_loft(id, slender=True):
    """🧵️ The PE riser hose as a round loft along its centre line on the stone bone."""
    secs = []
    for x, y in hose_path():
        u, v = loc("stone", x, y)
        secs.append({"at": [r6(u), r6(v), 0.0], "rx": HOSE_R, "ry": HOSE_R})
    s = solid("loft", id, "stone", "pe-hose", "detail", 0.15, sections=secs, capStart=True, capEnd=True)
    if slender:
        s["slender"] = True
    return s


def zx(poly):
    """🔁️ World plan points (x, z) as an extrude profile (w, u) of a bone at abs angle 0 with its origin at x = ox."""
    return [(z, x) for x, z in poly]


def shaft_cup():
    """🥣️ The shaft cup profile (u up from the base, radius)."""
    return [(0, 0), (0, R_OUT), (Y_RIM - Y_SHAFT, R_OUT), (Y_RIM - Y_SHAFT, R_IN), (Y_FLOOR - Y_SHAFT, R_IN), (Y_FLOOR - Y_SHAFT, 0)]


def water_profile():
    """💧️ The reservoir water in the shaft."""
    return [(Y_FLOOR - Y_SHAFT, 0), (Y_FLOOR - Y_SHAFT, R_IN), (Y_WATER - Y_SHAFT, R_IN), (Y_WATER - Y_SHAFT, 0)]


def slab_solid(id="slab"):
    """🟫️ The precast cover slab DN 1200 with its eccentric service opening."""
    return ext(id, "slab", "concrete", "body", 0.6, circle(0.0, R_OUT, R_OUT, 96), "zx", -SLAB_T / 2, SLAB_T / 2, holes=[circle(0.0, R_OUT + X_LID, R_HOLE, 64)])


def gravel_disc(id, y0, r=R_OUT):
    """🪨️ The gravel layer over the slab with the openings for the service lid and the boulder base."""
    holes = [circle(0.0, X_LID, 0.245, 64), ellipse(0.0, X_STONE, 0.25, 0.34, 48)]
    return ext(id, "basin", "gravel", "body", 0.72, circle(0.0, 0.0, r, 96), "zx", y0 - Y_GRAVEL, 0.0, holes=holes)


def basin_ring(id, material, tone, r0, r1, y0, y1, **extra):
    """⭕️ A ring of the basin (gravel edge, setts, haunch) about the vertical axis of the basin bone."""
    return rev(id, "basin", material, "body", tone, loop(y0 - Y_GRAVEL, y1 - Y_GRAVEL, r0, r1), axis=(0, 1, 0), **extra)


def frame_solid(profile, id="lid-frame"):
    """🔲️ The galvanised lid frame in the slab opening (slab bone, axis vertical through the lid centre)."""
    o = loc("slab", X_LID, Y_RIM + SLAB_T / 2)
    prof = [(y - (Y_RIM + SLAB_T / 2), r) for r, y in profile]
    return rev(id, "slab", "steel-galv", "body", 0.3, prof + [prof[0]], axis=(0, 1, 0), origin=(o[0], 0, 0))


def lid_disc(id, r, top, bottom, tone=0.35):
    """🛡️ A lid disc on the lid bone (u from the hinge towards the stone, v down) between two world heights."""
    v0, v1 = -(top - HINGE[1]), -(bottom - HINGE[1])
    return rev(id, "lid", "steel-galv", "body", tone, [(v0, 0), (v0, r), (v1, r), (v1, 0)], axis=(0, 1, 0), origin=(HINGE[0] - X_LID, 0, 0))


def handle_profile():
    """🪝️ The D-handle of the pump in its bone plane (u up, v = -x): outer bar and the grip opening."""
    u0, u1, half, bar, _ = HANDLE
    outer = [(u0, -half), (u1 - 0.02, -half), (u1, -half + 0.02), (u1, half - 0.02), (u1 - 0.02, half), (u0, half)]
    hole = [(u0 + 0.004, -half + bar), (u1 - bar, -half + bar), (u1 - bar, half - bar), (u0 + 0.004, half - bar)]
    return outer, hole


def handle_solid(id="pump-handle"):
    """🪝️ The carrying handle of the pump, a 20 mm thick D-bar on the housing top."""
    outer, hole = handle_profile()
    w = HANDLE[4]
    return ext(id, "pump", "stainless", "detail", 0.35, outer, "xy", -w, w, holes=[hole])


def pump_solids(scale):
    """🔧️ The submersible pump: strainer base, housing, outlet stub and handle (merged at the coarse scales)."""
    if scale == "50":
        return [box("pump-base", "pump", "stainless", "body", 0.4, (PUMP_BASE, PUMP_L, PUMP_W), (PUMP_BASE / 2, 0, 0), cornerRadius=0.006),
                box("pump-housing", "pump", "stainless", "body", 0.5, (PUMP_TOP - PUMP_BASE, PUMP_L - 0.02, PUMP_W - 0.01), ((PUMP_BASE + PUMP_TOP) / 2, 0, 0), cornerRadius=0.02),
                cap("pump-outlet", "pump", "stainless", "detail", 0.35, (OUTLET_U, PUMP_L / 2 - 0.02, 0), (OUTLET_U, OUTLET_V - OUTLET_R, 0), OUTLET_R), handle_solid()]
    if scale == "100":
        return [box("pump-housing", "pump", "stainless", "body", 0.5, (PUMP_TOP, PUMP_L, PUMP_W), (PUMP_TOP / 2, 0, 0), cornerRadius=0.015),
                cap("pump-outlet", "pump", "stainless", "detail", 0.35, (OUTLET_U, PUMP_L / 2 - 0.02, 0), (OUTLET_U, OUTLET_V - OUTLET_R, 0), OUTLET_R), handle_solid()]
    return [box("pump-housing", "pump", "stainless", "body", 0.5, (PUMP_TOP, PUMP_L, PUMP_W), (PUMP_TOP / 2, 0, 0))]


def bubble_solid(profile, id="bubble"):
    """💧️ The water dome bubbling out of the bore."""
    return rev(id, "bubble", "water", "glazing", 0.9, profile)


def leaves():
    """🍂️ Four fallen lime leaves lying on the gravel (declared thin sheets)."""
    out = []
    for k, (x, z, deg) in enumerate(LEAVES):
        out.append(ext(f"leaf-{k}", "basin", "leaf-litter", "detail", 0.45, zx(leaf_outline(x, z, deg)), "zx", 0.0, 0.0012, sheet=True))
    return out


def limescale():
    """🦪️ The limescale crust round the outlet on the boulder top (texture)."""
    u = Y_TOP - Y_SLAB_TOP
    return rev("limescale", "stone", "limescale", "texture", 0.95, loop(-0.001, 0.002, 0.044, 0.068), origin=(u, -(X_BORE - X_STONE), 0), texture=True)


def body(scale):
    """🧊️ The body solids of one scale."""
    b = [rev("shaft-ring", "shaft", "concrete", "body", 0.6, shaft_cup()), rev("water", "shaft", "water", "glazing", 0.88, water_profile()), slab_solid()]
    if scale == "50":
        b += [frame_solid(FRAME), lid_disc("lid-plate", R_PLATE, -0.020, -0.028),
              rev("lid-rib", "lid", "steel-galv", "detail", 0.3, loop(0.004, 0.034, 0.150, 0.160), axis=(0, 1, 0), origin=(HINGE[0] - X_LID, 0, 0)),
              cap("lid-hinge", "lid", "steel-galv", "detail", 0.25, (0, 0, -0.04), (0, 0, 0.04), 0.009)]
    elif scale == "100":
        b += [frame_solid([(0.185, -0.130), (0.215, -0.130), (0.215, -0.020), (0.185, -0.020)]), lid_disc("lid-plate", R_PLATE, -0.020, -0.045)]
    elif scale == "200":
        b += [lid_disc("lid-plate", R_HOLE, -0.020, -0.051)]
    else:
        b += [lid_disc("lid-plate", R_HOLE, -0.020, -0.096)]
    b.append(stone_loft("stone", {"50": STONE, "100": STONE_100, "200": STONE_200, "500": STONE_500}[scale]))
    b.append(hose_loft("hose"))
    b.append(bubble_solid(BUBBLE_500 if scale == "500" else BUBBLE))
    b += pump_solids(scale)
    if scale == "500":
        b.append(gravel_disc("gravel", -0.096, R_SETT_IN))
    else:
        b.append(gravel_disc("gravel", -0.050 if scale in ("50", "100") else -0.051))
        b.append(basin_ring("gravel-edge", "gravel", 0.72, R_OUT, R_SETT_IN, -0.10, Y_GRAVEL))
    b.append(basin_ring("setts", "granite-setts", 0.45, R_SETT_IN, R_SETT_OUT, -0.10, 0.0))
    b.append(basin_ring("haunch", "concrete", 0.65, R_HAUNCH_IN, R_SETT_OUT, Y_HAUNCH, -0.10))
    if scale == "50":
        b += leaves() + [limescale()]
    return b


REGION_OF = {"shaft-ring": "civil-concrete", "water": "reservoir-water", "slab": "civil-concrete", "lid-frame": "lid-steel", "lid-plate": "lid-steel", "lid-rib": "lid-steel", "lid-hinge": "lid-steel",
             "stone": "granite-stone", "hose": "riser-hose", "bubble": "bubble-water", "pump-base": "pump-motor", "pump-housing": "pump-motor", "pump-outlet": "pump-motor",
             "pump-handle": "pump-motor", "gravel": "gravel-bed", "gravel-edge": "gravel-bed", "setts": "edging-setts", "haunch": "civil-concrete"}
HIDDEN = {"reservoir-water", "riser-hose", "pump-motor"}


def thermal(scale, bodies):
    """🌡️ The thermal list: body surfaces on their regions, the hidden regions as solids of their own, the evaporation outlet patch on the boulder top."""
    out = []
    for s in bodies:
        region = REGION_OF.get(s["id"])
        if region is None:
            continue
        if region in HIDDEN:
            out.append(dict(s, id="th-" + s["id"], region=region))
        else:
            out.append({"id": "th-" + s["id"], "of": s["id"], "region": region})
    t = FEATURE[scale]
    u = Y_TOP - Y_SLAB_TOP
    out.append(rev("th-evaporation", "stone", "granite-boulder", "detail", 0.9, [(u - t, 0.0), (u - t, 0.06), (u, 0.06), (u, 0.0)], origin=(0, -(X_BORE - X_STONE), 0), region="granite-stone", tags=["evaporation"], texture=True))
    return out


def structure(scale, bodies, masses):
    """🏗️ The load path of one scale: shaft ring on the soil, bearing joint, cover slab, mortar bed, boulder; lid on its frame with the hinge; pump on its foot; three masses."""
    f = FEATURE[scale]
    ids = {s["id"] for s in bodies}
    mr = {"50": 0.015, "100": 0.02, "200": 0.03, "500": 0.0375}[scale]
    o = loc("slab", 0.0, Y_RIM)
    st = [{"id": "member-shaft", "of": "shaft-ring", "member": "shaft", "part": "member"},
          rev("joint-bearing", "slab", "concrete", "body", 0.5, loop(-SLAB_T / 2, -SLAB_T / 2 + f, R_IN, R_OUT), axis=(0, 1, 0), origin=(o[0], 0, 0), member="slab", part="joint"),
          {"id": "member-slab", "of": "slab", "member": "slab", "part": "member"},
          ext("joint-bed", "stone", "mortar", "body", 0.5, ellipse(0.0, 0.0, 0.30, 0.22, 48), "yz", 0.0, f, member="stone", part="joint"),
          {"id": "member-stone", "of": "stone", "member": "stone", "part": "member"},
          {"id": "member-lid", "of": "lid-plate", "member": "lid", "part": "member"}]
    if "lid-frame" in ids:
        st.append({"id": "joint-frame", "of": "lid-frame", "member": "lid", "part": "joint"})
    if "lid-hinge" in ids:
        st.append({"id": "joint-hinge", "of": "lid-hinge", "member": "lid", "part": "joint"})
    else:
        st.append(box("joint-hinge", "lid", "steel-galv", "body", 0.3, (max(f, 0.012), max(f, 0.012), 0.08), (0.0, 0.0, 0.0), member="lid", part="joint"))
    st.append({"id": "member-pump", "of": "pump-housing", "member": "pump", "part": "member"})
    if "pump-base" in ids:
        st.append({"id": "joint-pump-base", "of": "pump-base", "member": "pump", "part": "joint"})
    else:
        st.append(box("joint-pump-base", "pump", "stainless", "body", 0.4, (f, PUMP_L, PUMP_W), (f / 2, 0, 0), member="pump", part="joint"))
    for bone, (u, v) in masses.items():
        st.append(ell(f"mass-{bone}", bone, {"stone": "granite-boulder", "lid": "steel-galv", "pump": "stainless"}[bone], mr, (u, v, 0.0), member=bone, part="mass"))
    return st


class Views:
    """🖼️ Collects the side elevation (section A-A), front and plan shapes of one scale; points are given in world coordinates of the rest pose."""

    def __init__(self, pens):
        self.e, self.f, self.p = [], [], []
        self.pens = pens

    @staticmethod
    def shape(kind, id, bone, role, z, tone, region=None, material=None, filled=True, **geom):
        """✏️ A 2D shape."""
        s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
        if isinstance(region, list):
            s["regions"] = region
        elif region:
            s["region"] = region
        if material:
            s["material"] = material
        s["kind"] = kind
        for k, v in geom.items():
            s[k] = pts(v) if k == "points" else (r6(v) if isinstance(v, float) else v)
        return s

    def poly(self, view, id, bone, role, z, tone, points, region=None, material=None, closed=True, filled=True, **kw):
        """🔺️ A polygon; elevation points are mapped into the bone frame."""
        p = [loc(bone, x, y) for x, y in points] if view == "e" else points
        getattr(self, view).append(self.shape("poly", id, bone, role, z, tone, region, material, filled and closed, points=p, closed=closed, **kw))

    def rect(self, view, id, bone, role, z, tone, x0, y0, x1, y1, region=None, material=None, **kw):
        """▭️ A rectangle as a polygon."""
        self.poly(view, id, bone, role, z, tone, [(x0, y0), (x1, y0), (x1, y1), (x0, y1)], region, material, **kw)

    def ring(self, view, id, bone, role, z, tone, cx, cy, r0, r1, region=None, material=None, n=96, **kw):
        """⭕️ An annulus as a path with a hole (plan or front coordinates)."""
        def d(points):
            return "M " + " L ".join(f"{r6(x)} {r6(y)}" for x, y in points) + " Z"
        s = self.shape("path", id, bone, role, z, tone, region, material, True, d=d(circle(cx, cy, r1, n)) + " " + d(list(reversed(circle(cx, cy, r0, n)))), **kw)
        getattr(self, view).append(s)

    def stroke(self, view, id, bone, role, z, tone, points, region=None, **kw):
        """〰️ An open stroke."""
        self.poly(view, id, bone, role, z, tone, points, region, None, closed=False, filled=False, **kw)


def mirror(profile, x0, y0):
    """🪞️ A closed outline from a half profile (axial up, radial) about a vertical axis at x0, base at y0."""
    return [(x0 + r, y0 + u) for u, r in profile] + [(x0 - r, y0 + u) for u, r in reversed(profile)]


def lid_shapes(v, scale, z):
    """🛡️ The service lid in its own frame: elevation cut, front and plan by its revolved solid."""
    r, top, bottom = {"50": (R_PLATE, -0.020, -0.028), "100": (R_PLATE, -0.020, -0.045), "200": (R_HOLE, -0.020, -0.051), "500": (R_HOLE, -0.020, -0.096)}[scale]
    v0, v1 = -(top - HINGE[1]), -(bottom - HINGE[1])
    uc = HINGE[0] - X_LID
    half = [(uc, v0), (uc + r, v0), (uc + r, v1), (uc, v1)]
    sol = {"kind": "revolved", "axisBone": "lid", "axis": "v", "axisX": r6(-uc), "axisZ": 0, "mirror": True}
    hatch = {"hatch": "chequer"} if scale in ("50", "100") else {}
    v.poly("e", "lid-plate", "lid", "body", z, 0.35, [(X_LID - r, top), (X_LID + r, top), (X_LID + r, bottom), (X_LID - r, bottom)], "lid-steel", "steel-galv")
    for view in ("f", "p"):
        getattr(v, view).append(v.shape("poly", "lid-plate", "lid", "body", z, 0.35, "lid-steel", "steel-galv", True, points=half, closed=True, solid=sol, **hatch))
    if scale == "50":
        rib = [(uc + 0.150, 0.004), (uc + 0.160, 0.004), (uc + 0.160, 0.034), (uc + 0.150, 0.034)]
        for view in ("f",):
            getattr(v, view).append(v.shape("poly", "lid-rib", "lid", "detail", z - 0.05, 0.3, "lid-steel", "steel-galv", True, points=rib, closed=True, solid=sol))
        v.rect("e", "lid-rib-far", "lid", "detail", z - 0.06, 0.5, X_LID - 0.15, -0.058, X_LID + 0.15, -0.028, "lid-steel", "steel-galv")
        for s in (-1, 1):
            v.rect("e", f"lid-rib-{'f' if s > 0 else 'r'}", "lid", "detail", z - 0.05, 0.3, X_LID + s * 0.150, -0.058, X_LID + s * 0.160, -0.028, "lid-steel", "steel-galv")
        v.stroke("e", "lid-hinge", "lid", "detail", z + 0.1, 0.25, circle(HINGE[0], HINGE[1], 0.009, 16) + [circle(HINGE[0], HINGE[1], 0.009, 16)[0]], "lid-steel")
        v.stroke("p", "lid-hinge", "lid", "detail", z + 0.1, 0.25, [(HINGE[0], -0.04), (HINGE[0], 0.04)], "lid-steel")
        for k, zz in enumerate((-0.09, 0.09)):
            v.stroke("p", f"lid-key-{k}", "lid", "texture", z + 0.2, 0.1, [(X_LID - 0.112, zz), (X_LID - 0.088, zz)], "lid-steel")
        v.stroke("p", "lid-rust", "lid", "texture", z + 0.2, 0.6, [(X_LID + 0.12, -0.11), (X_LID + 0.15, -0.07), (X_LID + 0.16, -0.02)], "lid-steel")
        v.stroke("p", "lid-gum", "lid", "texture", z + 0.2, 0.15, [(X_LID - 0.028, 0.06), (X_LID - 0.012, 0.064), (X_LID - 0.02, 0.052)], "lid-steel")


def views(scale, pens):
    """🖼️ The 2D drawings of one scale: side elevation as section A-A through the bore, front elevation and plan."""
    v = Views(pens)
    fine = scale in ("50", "100")
    texture = pens["texture"] is not None
    detail = pens["detail"] is not None
    sec = {"50": STONE, "100": STONE_100, "200": STONE_200, "500": STONE_500}[scale]
    concrete = {"hatch": "concrete"} if fine else {}
    granite = {"hatch": "granite"} if fine else {}
    gravel = {"hatch": "gravel"} if fine else {}
    if scale == "500":
        for s in (-1, 1):
            v.rect("e", f"edging-cut-{'f' if s > 0 else 'r'}", "basin", "body", 0.8, 0.35, s * R_HAUNCH_IN, Y_HAUNCH, s * R_SETT_OUT, 0.0, ["edging-setts", "civil-concrete"], "granite-setts")
    else:
        v.rect("e", "haunch", "basin", "body", 0.5, 0.75, -R_SETT_OUT, Y_HAUNCH, R_SETT_OUT, -0.10, "civil-concrete", "concrete")
        for s in (-1, 1):
            v.rect("e", f"haunch-cut-{'f' if s > 0 else 'r'}", "basin", "body", 0.6, 0.45, s * R_HAUNCH_IN, Y_HAUNCH, s * R_SETT_OUT, -0.10, "civil-concrete", "concrete", **concrete)
        if fine:
            v.rect("e", "setts", "basin", "body", 0.7, 0.62, -R_SETT_IN, -0.10, R_SETT_IN, 0.0, "edging-setts", "granite-setts")
        for s in (-1, 1):
            v.rect("e", f"setts-cut-{'f' if s > 0 else 'r'}", "basin", "body", 0.8, 0.35, s * R_SETT_IN, -0.10, s * R_SETT_OUT, 0.0, "edging-setts", "granite-setts", **({"hatch": "setts"} if fine else {}))
    v.rect("e", "gravel", "basin", "body", 0.9, 0.72, -R_SETT_IN, -0.10 if scale != "500" else -0.096, R_SETT_IN, Y_GRAVEL, "gravel-bed", "gravel", **gravel)
    v.rect("e", "shaft-inner", "shaft", "body", 1.0, 0.82, -R_IN, Y_FLOOR, R_IN, Y_RIM, "civil-concrete", "concrete")
    v.rect("e", "water", "shaft", "glazing" if pens["glazing"] else "body", 1.1, 0.9, -R_IN, Y_FLOOR, R_IN, Y_WATER, "reservoir-water", "water")
    v.poly("e", "shaft-ring", "shaft", "body", 1.2, 0.45, [(-R_OUT, Y_SHAFT), (R_OUT, Y_SHAFT), (R_OUT, Y_RIM), (R_IN, Y_RIM), (R_IN, Y_FLOOR), (-R_IN, Y_FLOOR), (-R_IN, Y_RIM), (-R_OUT, Y_RIM)], "civil-concrete", "concrete", **concrete)
    v.rect("e", "slab-far", "slab", "body", 1.3, 0.7, X_LID - R_HOLE, Y_RIM, X_LID + R_HOLE, Y_SLAB_TOP, "civil-concrete", "concrete")
    v.rect("e", "slab-cut-r", "slab", "body", 1.4, 0.45, -R_OUT, Y_RIM, X_LID - R_HOLE, Y_SLAB_TOP, "civil-concrete", "concrete", **concrete)
    v.rect("e", "slab-cut-f", "slab", "body", 1.4, 0.45, X_LID + R_HOLE, Y_RIM, R_OUT, Y_SLAB_TOP, "civil-concrete", "concrete", **concrete)
    g_top = -0.050 if scale in ("50", "100") else -0.051
    if scale != "500":
        v.rect("e", "gravel-top-r", "basin", "body", 1.5, 0.72, -R_OUT, g_top, X_LID - 0.245, Y_GRAVEL, "gravel-bed", "gravel", **gravel)
        v.rect("e", "gravel-top-f", "basin", "body", 1.5, 0.72, X_LID + 0.245, g_top, R_OUT, Y_GRAVEL, "gravel-bed", "gravel", **gravel)
    if scale == "50":
        v.rect("e", "frame-far", "slab", "detail", 1.6, 0.55, X_LID - 0.200, -0.130, X_LID + 0.200, -0.020, "lid-steel", "steel-galv")
        for s in (-1, 1):
            v.poly("e", f"frame-cut-{'f' if s > 0 else 'r'}", "slab", "body", 1.7, 0.25, [(X_LID + s * r, y) for r, y in FRAME], "lid-steel", "steel-galv")
    elif scale == "100":
        v.rect("e", "frame-far", "slab", "detail", 1.6, 0.55, X_LID - 0.185, -0.130, X_LID + 0.185, -0.020, "lid-steel", "steel-galv")
        for s in (-1, 1):
            v.rect("e", f"frame-cut-{'f' if s > 0 else 'r'}", "slab", "body", 1.7, 0.25, X_LID + s * 0.185, -0.130, X_LID + s * 0.215, -0.020, "lid-steel", "steel-galv")
    lid_shapes(v, scale, 1.8)
    pump_shapes(v, scale)
    bub = BUBBLE_500 if scale == "500" else BUBBLE
    v.poly("e", "bubble", "bubble", "glazing" if pens["glazing"] else "body", 2.0, 0.92, mirror(bub, X_BORE, Y_TOP), "bubble-water", "water")
    v.poly("e", "stone", "stone", "body", 2.2, 0.55, stone_side(sec), "granite-stone", "granite-boulder", **granite)
    hose = hose_path()
    if fine:
        v.poly("e", "hose", "stone", "detail", 2.4, 0.15, band(hose, HOSE_R), "riser-hose", "pe-hose")
    else:
        v.stroke("e", "hose", "stone", "body", 2.4, 0.15, hose, "riser-hose")
    if scale == "50":
        stone_traces_50(v)
    elif scale == "100":
        v.stroke("e", "limescale-run", "stone", "texture", 2.6, 0.95, [at(0.79, 0.85), at(0.62, 0.87), at(0.40, 0.90)], "granite-stone")
        v.stroke("e", "moss-rear", "stone", "texture", 2.6, 0.3, [at(0.05, -0.92), at(0.20, -0.94), at(0.36, -0.92)], "granite-stone")
    front(v, scale, sec, concrete, granite, fine, texture)
    plan(v, scale, sec, granite, gravel, fine, texture, detail)
    return v


def pump_shapes(v, scale):
    """🔧️ The submersible pump: cut in the elevation, hidden inside the shaft in front and plan until it is lifted."""
    x0, x1 = X_LID - PUMP_L / 2, X_LID + PUMP_L / 2
    v.rect("e", "pump-housing", "pump", "body", 1.9, 0.5, x0, Y_FLOOR, x1, Y_FLOOR + PUMP_TOP, "pump-motor", "stainless")
    v.rect("f", "pump-housing", "pump", "body", 0.2, 0.5, -PUMP_W / 2, Y_FLOOR, PUMP_W / 2, Y_FLOOR + PUMP_TOP, "pump-motor", "stainless")
    v.rect("p", "pump-housing", "pump", "body", 0.2, 0.5, x0, -PUMP_W / 2, x1, PUMP_W / 2, "pump-motor", "stainless")
    if scale in ("50", "100"):
        y = Y_FLOOR + OUTLET_U
        v.rect("e", "pump-outlet", "pump", "detail", 1.95, 0.35, X_LID - OUTLET_V, y - OUTLET_R, x0 + 0.02, y + OUTLET_R, "pump-motor", "stainless")
        v.rect("p", "pump-outlet", "pump", "detail", 0.25, 0.35, X_LID - OUTLET_V, -OUTLET_R, x0 + 0.02, OUTLET_R, "pump-motor", "stainless")
        outer, hole = handle_profile()
        ring = lambda q: "M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in q) + " Z"
        v.e.append(v.shape("path", "pump-handle", "pump", "detail", 1.96, 0.35, "pump-motor", "stainless", True, d=ring(outer) + " " + ring(list(reversed(hole)))))
        u0, u1, half, bar, w = HANDLE
        v.rect("f", "pump-handle", "pump", "detail", 0.25, 0.35, -w, Y_FLOOR + u0, w, Y_FLOOR + u1, "pump-motor", "stainless")
        v.rect("p", "pump-handle", "pump", "detail", 0.26, 0.35, X_LID - half, -w, X_LID + half, w, "pump-motor", "stainless")
    if scale == "50":
        v.stroke("e", "pump-seam", "pump", "detail", 1.93, 0.3, [(x0, Y_FLOOR + PUMP_BASE), (x1, Y_FLOOR + PUMP_BASE)], "pump-motor", penMm=0.13)
        for k in range(7):
            x = x0 + 0.03 + 0.03 * k
            v.stroke("e", f"strainer-{k}", "pump", "texture", 1.93, 0.2, [(x, Y_FLOOR + 0.01), (x, Y_FLOOR + 0.03)], "pump-motor")
        v.stroke("e", "pump-scratch", "pump", "texture", 1.97, 0.7, [(X_LID + 0.03, Y_FLOOR + 0.07), (X_LID + 0.07, Y_FLOOR + 0.12)], "pump-motor")


def at(h, f, view="e"):
    """📍️ A point on the 1:50 boulder at height h above its base and the fraction f of its half width (elevation: towards +x; front: towards +z)."""
    for (h0, d0, a0, b0, n0), (h1, d1, a1, b1, n1) in zip(STONE, STONE[1:]):
        if h0 <= h <= h1:
            t = (h - h0) / (h1 - h0)
            d, a, b = d0 + t * (d1 - d0), a0 + t * (a1 - a0), b0 + t * (b1 - b0)
            return (X_STONE + d + f * a, Y_SLAB_TOP + h) if view == "e" else (f * b, Y_SLAB_TOP + h)
    raise ValueError(h)


def pat(fx, fz):
    """📍️ A plan point on the boulder: fractions of its largest half length and width about the centre of its girth."""
    h, d, a, b, n = STONE[4]
    return (X_STONE + d + fx * a, fz * b)


def stone_traces_50(v):
    """🦠️ The traces of use and weather on the 1:50 boulder: limescale crust and runs, moss and algae on the shaded rear flank, a quartz vein, the water film, the algae rim in the shaft."""
    v.stroke("e", "limescale-crust", "stone", "texture", 2.6, 0.95, [at(0.792, -0.75), at(0.80, -0.35), at(0.80, 0.35), at(0.792, 0.75)], "granite-stone")
    for k, run in enumerate([[at(0.79, 0.85), at(0.70, 0.86), at(0.55, 0.88), at(0.38, 0.90), at(0.22, 0.92)],
                             [at(0.78, 0.45), at(0.66, 0.50), at(0.50, 0.58), at(0.34, 0.62)],
                             [at(0.79, -0.6), at(0.70, -0.66), at(0.60, -0.72)]]):
        v.stroke("e", f"limescale-run-{k}", "stone", "texture", 2.6, 0.95, run, "granite-stone")
    for k, run in enumerate([[at(0.03, -0.93), at(0.14, -0.95), at(0.26, -0.95)], [at(0.34, -0.93), at(0.44, -0.92), at(0.52, -0.88)], [at(0.06, -0.82), at(0.18, -0.85)]]):
        v.stroke("e", f"moss-{k}", "stone", "texture", 2.6, 0.3, run, "granite-stone")
    v.stroke("e", "quartz-vein", "stone", "detail", 2.5, 0.8, [at(0.08, -0.8), at(0.22, -0.4), at(0.30, 0.0), at(0.45, 0.4), at(0.50, 0.75)], "granite-stone", penMm=0.13)
    v.stroke("e", "water-film", "stone", "texture", 2.6, 0.9, [at(0.79, 0.3), at(0.765, 0.55), at(0.72, 0.72), at(0.64, 0.82)], "granite-stone")
    v.stroke("e", "splash", "bubble", "texture", 2.1, 0.9, [(X_BORE - 0.03, Y_TOP + 0.066), (X_BORE - 0.035, Y_TOP + 0.075)], "bubble-water")
    v.stroke("e", "water-line", "shaft", "texture", 1.15, 0.6, [(-R_IN, Y_WATER), (R_IN, Y_WATER)], "reservoir-water")
    v.stroke("e", "algae-rim", "shaft", "texture", 1.25, 0.3, [(-R_IN, Y_WATER + 0.02), (-R_IN + 0.05, Y_WATER + 0.02)], "reservoir-water")


def front(v, scale, sec, concrete, granite, fine, texture):
    """🧭️ The front elevation, seen from ahead: below the ground the haunch, setts, slab and shaft, above it the boulder with its bubble and, when opened, the lid."""
    v.rect("f", "shaft-ring", "shaft", "body", 0.5, 0.6, -R_OUT, Y_SHAFT, R_OUT, Y_RIM, "civil-concrete", "concrete", **concrete)
    v.rect("f", "slab", "slab", "body", 0.6, 0.55, -R_OUT, Y_RIM, R_OUT, Y_SLAB_TOP, "civil-concrete", "concrete", **concrete)
    v.poly("f", "bubble", "bubble", "glazing" if v.pens["glazing"] else "body", 0.9, 0.92, mirror(BUBBLE_500 if scale == "500" else BUBBLE, 0.0, Y_TOP), "bubble-water", "water")
    v.poly("f", "stone", "stone", "body", 1.0, 0.55, stone_front(sec), "granite-stone", "granite-boulder", **granite)
    if scale == "500":
        v.rect("f", "edging", "basin", "body", 2.3, 0.45, -R_SETT_OUT, Y_HAUNCH, R_SETT_OUT, 0.0, ["edging-setts", "civil-concrete"], "granite-setts")
    else:
        v.rect("f", "haunch", "basin", "body", 2.2, 0.7, -R_SETT_OUT, Y_HAUNCH, R_SETT_OUT, -0.10, "civil-concrete", "concrete", **concrete)
        v.rect("f", "setts", "basin", "body", 2.3, 0.45, -R_SETT_OUT, -0.10, R_SETT_OUT, 0.0, "edging-setts", "granite-setts")
    if fine:
        n = 24 if scale == "50" else 12
        for k in range(1, n):
            a = PI * k / n
            z = R_SETT_OUT * math.cos(a)
            v.stroke("f", f"sett-joint-{k}", "basin", "detail", 2.4, 0.3, [(z, -0.10), (z, 0.0)], "edging-setts", penMm=0.1)
    if scale == "50":
        for k, run in enumerate([[at(0.10, 0.80, "f"), at(0.28, 0.86, "f"), at(0.45, 0.82, "f")], [at(0.06, 0.62, "f"), at(0.20, 0.70, "f")]]):
            v.stroke("f", f"moss-{k}", "stone", "texture", 1.2, 0.3, run, "granite-stone")
        for k, run in enumerate([[at(0.79, 0.1, "f"), at(0.66, 0.15, "f"), at(0.50, 0.22, "f")], [at(0.785, -0.3, "f"), at(0.65, -0.38, "f"), at(0.48, -0.45, "f")]]):
            v.stroke("f", f"limescale-run-{k}", "stone", "texture", 1.2, 0.95, run, "granite-stone")
        v.stroke("f", "quartz-vein", "stone", "detail", 1.15, 0.8, [at(0.18, -0.85, "f"), at(0.30, -0.3, "f"), at(0.40, 0.3, "f"), at(0.46, 0.8, "f")], "granite-stone", penMm=0.13)
    elif scale == "100":
        v.stroke("f", "moss", "stone", "texture", 1.2, 0.3, [at(0.10, 0.80, "f"), at(0.28, 0.86, "f"), at(0.45, 0.82, "f")], "granite-stone")


def plan(v, scale, sec, granite, gravel, fine, texture, detail):
    """🗺️ The plan seen from above: setts ring, gravel bed with leaves, lid frame and lid, the boulder with its top and the bubble."""
    v.ring("p", "setts", "basin", "body", 1.0, 0.45, 0.0, 0.0, R_SETT_IN, R_SETT_OUT, "edging-setts", "granite-setts")
    v.poly("p", "gravel", "basin", "body", 1.1, 0.72, circle(0.0, 0.0, R_SETT_IN, 96), "gravel-bed", "gravel", **gravel)
    if fine:
        n = 60 if scale == "50" else 30
        for k in range(n):
            a = 2 * PI * k / n
            v.stroke("p", f"sett-joint-{k}", "basin", "detail", 1.05, 0.3, [(R_SETT_IN * math.cos(a), R_SETT_IN * math.sin(a)), (R_SETT_OUT * math.cos(a), R_SETT_OUT * math.sin(a))], "edging-setts", penMm=0.1)
    if scale == "50":
        v.ring("p", "lid-frame", "slab", "body", 1.5, 0.3, X_LID, 0.0, 0.188, 0.245, "lid-steel", "steel-galv", n=64)
        for k, (x, z, deg) in enumerate(LEAVES):
            v.poly("p", f"leaf-{k}", "basin", "detail", 1.2, 0.45, leaf_outline(x, z, deg), None, "leaf-litter")
        for k, run in enumerate([[(-0.10, 0.36), (0.02, 0.40), (0.12, 0.34)], [(-0.74, -0.30), (-0.78, -0.12)]]):
            v.stroke("p", f"algae-{k}", "basin", "texture", 1.2, 0.3, run, "gravel-bed")
    elif scale == "100":
        v.ring("p", "lid-frame", "slab", "body", 1.5, 0.3, X_LID, 0.0, 0.185, 0.215, "lid-steel", "steel-galv", n=48)
    v.poly("p", "stone", "stone", "body", 2.0, 0.55, stone_plan(sec, 96 if fine else 48), "granite-stone", "granite-boulder", **granite)
    if detail:
        for k, idx in enumerate((5, 7) if scale in ("50", "100") else (3,)):
            h, dx, rx, rz, n = sec[min(idx, len(sec) - 2)]
            v.poly("p", f"stone-contour-{k}", "stone", "detail", 2.1, 0.5, superellipse(X_STONE + dx, 0.0, rx, rz, n, 64), "granite-stone", filled=False, penMm=0.1)
    if scale in ("50", "100"):
        h, dx, rx, rz, n = sec[-1]
        v.poly("p", "stone-top", "stone", "detail", 2.2, 0.6, superellipse(X_STONE + dx, 0.0, rx, rz, n, 48), "granite-stone", filled=False, penMm=0.13 if scale == "50" else 0.1)
        v.poly("p", "outlet-cup", "stone", "detail", 2.3, 0.92, circle(X_BORE, 0.0, 0.042, 32), "bubble-water", "water")
    if scale == "50":
        v.ring("p", "limescale-crust", "stone", "texture", 2.25, 0.95, X_BORE, 0.0, 0.044, 0.068, "granite-stone", "limescale", n=32)
        for k, run in enumerate([[pat(-0.88, -0.35), pat(-0.94, 0.0), pat(-0.88, 0.35)], [pat(-0.75, -0.6), pat(-0.83, -0.4)], [pat(-0.78, 0.55), pat(-0.68, 0.68)]]):
            v.stroke("p", f"moss-{k}", "stone", "texture", 2.3, 0.3, run, "granite-stone")
        for k, rr in enumerate((0.055, 0.075)):
            v.poly("p", f"ripple-{k}", "stone", "texture", 2.35, 0.9, circle(X_BORE, 0.0, rr, 24), "granite-stone", filled=False)
        v.stroke("p", "quartz-vein", "stone", "detail", 2.3, 0.8, [pat(-0.75, -0.55), pat(-0.25, -0.3), pat(0.25, -0.05), pat(0.7, 0.2)], "granite-stone", penMm=0.13)
    v.poly("p", "bubble", "bubble", "glazing" if v.pens["glazing"] else "body", 0.9, 0.92, circle(X_BORE, 0.0, 0.045 if scale == "500" else 0.042, 24), "bubble-water", "water")
