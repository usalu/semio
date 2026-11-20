"""🧊 Round 3, machinery/evaporative-humidifier: every solid of the body, thermal and structure lists of 1:50, 1:100 and 1:200, stated by hand.

Usage: imported by `r3-evaporative-humidifier-build.py` (`volumes()`).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("humidifier_geo", os.path.join(HERE, "r3-evaporative-humidifier-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL = "galvanised-steel"
PP = "pp-plastic"
WATER = "water"
PAD = "cellulose"
R6 = lambda v: round(v, 6)


def solid(id, kind, bone, material, tone, role="body", **kw):
    """🧩 One solid with the common members first."""
    out = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    out.update(kw)
    return out


def box(id, bone, material, tone, cx, cy, cz, sx, sy, sz, **kw):
    """📦 A box given by its world rest centre and size."""
    u, v = G.loc(bone, cx, cy)
    return solid(id, "box", bone, material, tone, size=[R6(sx), R6(sy), R6(sz)], transform={"translation": [u, v, R6(cz)]}, **kw)


def ext_xy(id, bone, material, tone, pts, z0, z1, **kw):
    """🧱 A world (x, y) profile pushed along z."""
    return solid(id, "extrude", bone, material, tone, profile=[G.loc(bone, x, y) for x, y in pts], plane="xy", **{"from": R6(z0), "to": R6(z1)}, **kw)


def ext_yz(id, bone, material, tone, pts, x0, x1, holes=None, **kw):
    """🧱 A world (y, z) profile pushed along x."""
    ox, oy = G.RIG[bone]
    out = solid(id, "extrude", bone, material, tone, profile=[[R6(y - oy), z] for y, z in pts], plane="yz", **{"from": R6(x0 - ox), "to": R6(x1 - ox)})
    if holes:
        out["holes"] = [[[R6(y - oy), z] for y, z in hole] for hole in holes]
    out.update(kw)
    return out


def ext_zx(id, bone, material, tone, pts, y0, y1, holes=None, **kw):
    """🧱 A world (z, x) profile pushed along y."""
    ox, oy = G.RIG[bone]
    out = solid(id, "extrude", bone, material, tone, profile=[[z, R6(x - ox)] for z, x in pts], plane="zx", **{"from": R6(y0 - oy), "to": R6(y1 - oy)})
    if holes:
        out["holes"] = [[[z, R6(x - ox)] for z, x in hole] for hole in holes]
    out.update(kw)
    return out


def rev(id, bone, material, tone, profile, origin, axis, **kw):
    """🌀 A revolve of an [axial, radial] profile about an axis through a world rest point."""
    u, v = G.loc(bone, origin[0], origin[1])
    return solid(id, "revolve", bone, material, tone, profile=profile, axis=axis, origin=[u, v, origin[2]], **kw)


def ref(id, of, **kw):
    """🔗 A reference to a body solid of the same scale."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def ring(y0, y1, z0, z1):
    """⬜ A (y, z) rectangle."""
    return [[y0, z0], [y1, z0], [y1, z1], [y0, z1]]


def runners(s):
    """🛤️ Two base runners along z: rectangular hollow sections 40 x 20 x 2 mm with a plastic end plug in each end at 1:50; solid bars of that section at 1:100."""
    out = []
    w, h = G.RUNNER_W / 2, G.BASE
    for name, x in (("inlet", -G.RUNNER_X), ("outlet", G.RUNNER_X)):
        if s == 50:
            out.append(ext_xy(f"base-runner-{name}", "casing", STEEL, 0.25, G.rect(x - w, 0.0, x + w, h), -G.HZ, G.HZ, holes=[[G.loc("casing", a, b) for a, b in G.rect(x - w + 0.002, 0.002, x + w - 0.002, h - 0.002)]]))
            out.append(box(f"runner-plug-{name}", "casing", PP, 0.15, x, h / 2, -G.HZ + 0.005, 2 * w - 0.004, h - 0.004, 0.0075, repeat={"count": 2, "translation": [0, 0, 2 * G.HZ - 0.01]}))
        else:
            out.append(ext_xy(f"base-runner-{name}", "casing", STEEL, 0.25, G.rect(x - w, 0.0, x + w, h), -G.HZ, G.HZ))
    return out


def casing(s):
    """🗄️ The duct-section casing of 1 mm sheet (real thickness, hollow profile) and its two connection frames whose bottom bars are the tank end walls."""
    t, hz, top = G.SHEET, G.HZ, G.TOP
    f = {50: 0.01, 100: 0.015, 200: 0.03}[s]
    y0 = G.BASE if s != 200 else 0.03
    out = []
    if s == 50:
        shell = [[y0, -hz], [y0, hz], [top, hz], [top, -hz], [top - t, -hz], [top - t, hz - t], [y0 + t, hz - t], [y0 + t, -hz]]
        out.append(ext_yz("casing-shell", "casing", STEEL, 0.35, shell, -G.HX, G.HX))
        cover = [[y0 + t, -hz], [y0 + t, -hz + 0.01], [y0 + 2 * t, -hz + 0.01], [y0 + 2 * t, -hz + t], [top - 2 * t, -hz + t], [top - 2 * t, -hz + 0.01], [top - t, -hz + 0.01], [top - t, -hz]]
        out.append(ext_yz("service-cover", "casing", STEEL, 0.4, cover, -G.HX + f, G.HX - f))
        out.append(rev("cover-fastener-low", "casing", STEEL, 0.15, [[-0.006, 0.0], [0.004, 0.0], [0.004, 0.011], [-0.006, 0.011]], [-0.2, 0.1, -hz], [0, 0, -1], repeat={"count": 2, "translation": [0.4, 0, 0]}))
        out.append(rev("cover-fastener-high", "casing", STEEL, 0.15, [[-0.006, 0.0], [0.004, 0.0], [0.004, 0.011], [-0.006, 0.011]], [-0.2, 0.52, -hz], [0, 0, -1], repeat={"count": 2, "translation": [0.4, 0, 0]}))
    else:
        out.append(ext_yz("casing-shell", "casing", STEEL, 0.35, ring(y0, top, -hz, hz), -G.HX, G.HX, holes=[ring(y0 + t, top - t, -hz + t, hz - t)]))
    near = -hz if s == 50 else -hz + t
    outer = ring(y0 + t, top - t, near, hz - t)
    hole = ring(G.OPEN_Y[0], G.OPEN_Y[1], -G.OPEN_Z, G.OPEN_Z)
    out.append(ext_yz("end-frame-inlet", "casing", STEEL, 0.3, outer, -G.HX, -G.HX + f, holes=[hole]))
    out.append(ext_yz("end-frame-outlet", "casing", STEEL, 0.3, outer, G.HX - f, G.HX, holes=[hole]))
    if s == 200:
        out.insert(0, box("base", "casing", STEEL, 0.25, 0.0, 0.015, 0.0, 2 * G.HX, 0.03, 2 * hz))
    return out


def fittings():
    """🔩 1:50 only: the bulkhead fittings of the make-up water inlet and the tank drain on the far side wall."""
    return [
        rev("water-inlet-fitting", "casing", STEEL, 0.2, [[-0.004, 0.0], [0.0035, 0.0], [0.0035, 0.015], [-0.004, 0.015]], [G.FLOAT[0], 0.075, G.HZ], [0, 0, 1]),
        rev("drain-fitting", "casing", STEEL, 0.2, [[-0.004, 0.0], [0.0035, 0.0], [0.0035, 0.02], [-0.004, 0.02]], [0.19, 0.045, G.HZ], [0, 0, 1]),
    ]


def water(s):
    """💧 The tank water: 6 L are 32 mm over the tank floor between the tank walls (generalised to a 40 mm slab at 1:200)."""
    if s == 200:
        return [box("water", "water", WATER, 0.45, 0.0, 0.05, 0.0, 0.44, 0.04, 2 * (G.HZ - G.SHEET))]
    y0 = G.BASE + G.SHEET
    f = {50: 0.01, 100: 0.015}[s]
    return [box("water", "water", WATER, 0.45, 0.0, (y0 + G.LEVEL) / 2, 0.0, 2 * (G.HX - f), G.LEVEL - y0, 2 * (G.HZ - G.SHEET))]


def pad_support(s):
    """🪜 The two rails that carry the cassette over the tank and stand with their vertical leg in the water: 20 x 30 mm angles of 2 mm steel at 1:50, bars at 1:100."""
    if s == 200:
        return []
    out = []
    hz = G.HZ - G.SHEET
    for name, x in (("inlet", -G.RAIL_X), ("outlet", G.RAIL_X)):
        if s == 50:
            pts = [[x - 0.01, 0.05], [x - 0.008, 0.05], [x - 0.008, 0.078], [x + 0.01, 0.078], [x + 0.01, 0.08], [x - 0.01, 0.08]]
            out.append(ext_xy(f"pad-rail-{name}", "casing", STEEL, 0.3, pts, -hz, hz))
        else:
            out.append(box(f"pad-rail-{name}", "casing", STEEL, 0.3, x, 0.065, 0.0, 0.02, 0.03, 2 * hz))
    return out


def bulkhead(s):
    """🚧 The bypass seal on the inlet side: a bulkhead frame from the water to the top panel with a window on the pad face, and the reveal that joins the window to the cassette."""
    hz = G.HZ - G.SHEET
    t = G.FEATURE[s]
    window = ring(G.WINDOW_Y[0], G.WINDOW_Y[1], -G.OPEN_Z, G.OPEN_Z)
    low = {50: 0.045, 100: 0.04, 200: 0.07}[s]
    out = [ext_yz("bulkhead", "casing", STEEL, 0.3, ring(low, G.TOP - G.SHEET, -hz, hz), G.BULKHEAD_X - t, G.BULKHEAD_X, holes=[window])]
    if s == 50:
        out.append(ext_yz("pad-reveal", "casing", STEEL, 0.32, ring(0.0875, 0.4775, -0.1775, 0.1775), G.BULKHEAD_X, -G.PAD_X - 0.002, holes=[window]))
    if s == 100:
        out.append(ext_yz("pad-reveal", "casing", STEEL, 0.32, ring(0.08, 0.48, -0.185, 0.185), G.BULKHEAD_X, -G.PAD_X, holes=[window]))
    return out


def pad(s):
    """🧽 The cross-fluted cellulose pad, 0.2 m deep in the air direction: vertical flutes on both faces (24 at 1:50, 12 at 1:100, a plain block at 1:200) and, at 1:50, the two side channels of its cassette."""
    y0, y1 = G.PAD_Y
    if s == 200:
        return [box("pad", "pad", PAD, 0.7, -0.01, 0.275, 0.0, 0.22, 0.41, 2 * (G.HZ - G.SHEET))]
    out = [ext_zx("pad", "pad", PAD, 0.7, G.flutes(24, 0.006) if s == 50 else G.flutes(12, 0.008), y0, y1)]
    if s == 50:
        z, x = G.PAD_Z, G.PAD_X
        far = [[z - 0.015, -x - 0.002], [z + 0.001, -x - 0.002], [z + 0.001, x + 0.002], [z - 0.015, x + 0.002], [z - 0.015, x + 0.001], [z, x + 0.001], [z, -x - 0.001], [z - 0.015, -x - 0.001]]
        out.append(ext_zx("cassette-far", "pad", STEEL, 0.32, far, y0, y1))
        out.append(ext_zx("cassette-near", "pad", STEEL, 0.32, [[-a, b] for a, b in far][::-1], y0, y1))
    return out


def trough(s):
    """🚿 The polypropylene distribution trough on the cassette: a 3 mm channel with an anti-splash rim and two end caps at 1:50; a floor slab under a 3 mm wall ring at 1:100 and 1:200; the water film in it."""
    x, z, w = G.TROUGH_X, G.TROUGH_Z, G.TROUGH_WALL
    y0, y1 = G.TROUGH_Y
    if s == 50:
        channel = [[-x, y0], [x, y0], [x, y1], [x - 0.009, y1], [x - 0.009, y1 - w], [x - w, y1 - w], [x - w, y0 + w], [-x + w, y0 + w], [-x + w, y1 - w], [-x + 0.009, y1 - w], [-x + 0.009, y1], [-x, y1]]
        far = [[y0, z - w], [y0, z], [y1, z], [y1, z - 0.012], [y1 - w, z - 0.012], [y1 - w, z - w]]
        return [
            ext_xy("trough", "trough", PP, 0.4, channel, -z + w, z - w),
            ext_yz("trough-end-far", "trough", PP, 0.4, far, -x, x),
            ext_yz("trough-end-near", "trough", PP, 0.4, [[a, -b] for a, b in far][::-1], -x, x),
            box("film", "pad", WATER, 0.85, 0.0, y0 + w + 0.005, 0.0, 2 * (x - w), 0.01, 2 * (z - w)),
        ]
    f = G.FEATURE[s]
    y1 = y0 + f + (0.035 if s == 100 else 0.03)
    if s == 200:
        z = G.HZ - G.SHEET
    out = [
        box("trough-floor", "trough", PP, 0.4, 0.0, y0 + f / 2, 0.0, 2 * x, f, 2 * z),
        ext_zx("trough-walls", "trough", PP, 0.4, [[-z, -x], [z, -x], [z, x], [-z, x]], y0 + f, y1, holes=[[[-z + w, -x + w], [-z + w, x - w], [z - w, x - w], [z - w, -x + w]]]),
    ]
    if s == 100:
        out.append(box("film", "pad", WATER, 0.85, 0.0, y0 + f + 0.0075, 0.0, 2 * (x - w), 0.015, 2 * (z - w)))
    return out


def pump(s):
    """⛽ The submersible recirculation pump in the downstream part of the tank with its riser hose to the trough, and the float valve of the make-up water in the upstream part."""
    if s == 200:
        return []
    x, z = G.PUMP
    floor = G.BASE + G.SHEET
    path = G.hose_path(s)
    hose = solid("riser-hose", "loft", "casing", PP, 0.3, role="detail", slender=True, sections=[{"at": [*G.loc("casing", px, py), z], "rx": G.HOSE_R, "ry": G.HOSE_R} for px, py in path], capStart=True, capEnd=True)
    if s == 50:
        body = [[0.0, 0.0], [0.0, 0.034], [0.012, 0.034], [0.012, 0.03], [0.062, 0.03], [0.07, 0.022], [0.07, 0.011], [0.088, 0.011], [0.088, 0.0]]
        fx, fy, fz = G.FLOAT
        return [
            rev("pump", "casing", PP, 0.25, body, [x, floor, z], [0, 1, 0]),
            hose,
            solid("float-valve", "capsule", "casing", PP, 0.3, p0=[*G.loc("casing", fx, 0.075), G.HZ - G.SHEET - 0.011], p1=[*G.loc("casing", fx, 0.075), 0.16], r0=0.011, r1=0.011),
            box("float-lever", "casing", PP, 0.3, fx, 0.0735, 0.125, 0.0075, 0.0075, 0.07),
            solid("float-ball", "ellipsoid", "casing", PP, 0.6, radii=[G.FLOAT_R] * 3, transform={"translation": [*G.loc("casing", fx, fy), fz]}),
        ]
    fx, fy, fz = G.FLOAT
    return [
        rev("pump", "casing", PP, 0.25, [[0.0, 0.0], [0.0, 0.033], [0.085, 0.033], [0.085, 0.0]], [x, floor, z], [0, 1, 0]),
        hose,
        solid("float-ball", "ellipsoid", "casing", PP, 0.6, radii=[G.FLOAT_R] * 3, transform={"translation": [*G.loc("casing", fx, fy), fz]}),
    ]


def drops(s):
    """💦 The drops on the prismatic drip bone: nine at 1:50 (a row on the inlet face, a row inside the pad on the section plane, a row on the outlet face), five at 1:100 (the middle row and one on each face)."""
    if s == 50:
        r = [0.01, 0.013, 0.01]
        return [solid(f"drop-row-{k + 1}", "ellipsoid", "drip", WATER, 0.85, radii=r, transform={"translation": [x, 0.0, -G.DROP_Z]}, repeat={"count": 3, "translation": [0, 0, G.DROP_Z]}) for k, x in enumerate((-G.DROP_X, 0.0, G.DROP_X))]
    if s == 100:
        r = [0.014, 0.0182, 0.014]
        return [
            solid("drop-row", "ellipsoid", "drip", WATER, 0.85, radii=r, transform={"translation": [0.0, 0.0, -G.DROP_Z]}, repeat={"count": 3, "translation": [0, 0, G.DROP_Z]}),
            solid("drop-face", "ellipsoid", "drip", WATER, 0.85, radii=r, transform={"translation": [-G.DROP_X - 0.004, 0.0, 0.0]}, repeat={"count": 2, "translation": [2 * G.DROP_X + 0.008, 0, 0]}),
        ]
    return []


def body(s):
    """🧍 The body list of scale s."""
    out = ([] if s == 200 else runners(s)) + casing(s)
    if s == 50:
        out += fittings()
    return out + water(s) + pad_support(s) + bulkhead(s) + pad(s) + trough(s) + pump(s) + drops(s)


def panels(s):
    """🌡️ The four casing panels as 1 mm skins of the casing layer; their inner faces look at the process air of the duct run and are marked (C25 a, C31)."""
    t, hz, top = G.SHEET, G.HZ, G.TOP
    y0 = G.BASE if s != 200 else 0.03
    mid, h = (y0 + top) / 2, top - y0 - 2 * t
    out = [
        box("th-casing-top", "casing", STEEL, 0.35, 0.0, top - t / 2, 0.0, 2 * G.HX, t, 2 * hz, sectionFaces=["-v"]),
        box("th-casing-bottom", "casing", STEEL, 0.35, 0.0, y0 + t / 2, 0.0, 2 * G.HX, t, 2 * hz, sectionFaces=["+v"]),
        box("th-casing-far", "casing", STEEL, 0.35, 0.0, mid, hz - t / 2, 2 * G.HX, h, t, sectionFaces=["-w"]),
        box("th-casing-cover", "casing", STEEL, 0.4, 0.0, mid, -hz + t / 2, 2 * G.HX, h, t, sectionFaces=["+w"]),
    ]
    for d in out:
        d["region"] = "casing"
        d["layer"] = "casing-skin"
    return out


OUTSIDE = {"base", "cover-fastener-low", "cover-fastener-high", "water-inlet-fitting", "drain-fitting"}
REGION = {"pad": "pad", "film": "pad", "drop-row-1": "pad", "drop-row-2": "pad", "drop-row-3": "pad", "drop-row": "pad", "drop-face": "pad", "trough": "trough", "trough-end-far": "trough", "trough-end-near": "trough", "trough-floor": "trough", "trough-walls": "trough", "riser-hose": "trough", "water": "sump"}


def thermal(s, bodies):
    """🌡️ The thermal list: the casing as its four outer skins, connection frames (both faces lie against the duct flanges or look at the process air), runners and outside fittings; pad, film and drops; trough and riser hose; tank water; the vent emitter as an outlet disc flush in the outlet frame (D2). Parts inside the airway that belong to no region stay out."""
    out = panels(s)
    if s != 200:
        for name, x in (("inlet", -G.RUNNER_X), ("outlet", G.RUNNER_X)):
            out.append(box(f"th-base-runner-{name}", "casing", STEEL, 0.25, x, G.BASE / 2, 0.0, G.RUNNER_W, G.BASE, 2 * G.HZ, region="casing"))
    for b in bodies:
        if b["id"] in OUTSIDE:
            out.append(ref("th-" + b["id"], b["id"], region="casing"))
        elif b["id"].startswith("end-frame"):
            out.append(ref("th-" + b["id"], b["id"], region="casing", sectionFaces=["-u", "+u"]))
        elif b["id"] in REGION:
            out.append(ref("th-" + b["id"], b["id"], region=REGION[b["id"]]))
    f = G.FEATURE[s]
    disc = rev("th-outlet-air", "casing", PAD, 0.9, [[-f, 0.0], [0.0, 0.0], [0.0, G.OUTLET_R], [-f, G.OUTLET_R]], [G.HX, G.OUTLET_Y, 0.0], [1, 0, 0], tags=["outlet-air"])
    disc["region"] = "pad"
    return out + [disc]


def st(d, member, part, **kw):
    """🏗️ Turns a solid into a structural solid."""
    d = dict(d)
    d["member"] = member
    d["part"] = part
    d.update(kw)
    return d


def structure(s):
    """🏗️ The load path: pad and trough on the cassette, the cassette on two rails that span between the side walls, the water on the tank floor, the casing as its four folded longitudinal edges between the two connection frames, the base runners on the floor; supports under the runners, masses at the centres of mass."""
    hz = G.HZ
    e = {50: 0.02, 100: 0.03, 200: 0.045}[s]
    f = G.FEATURE[s]
    y0 = G.BASE if s != 200 else 0.03
    out = []
    if s == 200:
        out.append(ref("st-base", "base", member="water", part="member"))
    else:
        out += [ref("st-base-runner-inlet", "base-runner-inlet", member="casing", part="member"), ref("st-base-runner-outlet", "base-runner-outlet", member="casing", part="member")]
        out.append(st(box("st-tank-floor", "water", STEEL, 0.35, 0.0, y0 + f / 2, 0.0, 2 * G.HX, f, 2 * hz), "water", "member"))
    out += [
        st(box("st-edge-low", "casing", STEEL, 0.35, 0.0, y0 + e / 2, -hz + e / 2, 2 * G.HX, e, e, repeat={"count": 2, "translation": [0, 0, 2 * hz - e]}), "casing", "member"),
        st(box("st-edge-high", "casing", STEEL, 0.35, 0.0, G.TOP - e / 2, -hz + e / 2, 2 * G.HX, e, e, repeat={"count": 2, "translation": [0, 0, 2 * hz - e]}), "casing", "member"),
        ref("st-end-frame-inlet", "end-frame-inlet", member="casing", part="member"),
        ref("st-end-frame-outlet", "end-frame-outlet", member="casing", part="member"),
    ]
    if s != 200:
        out += [ref("st-pad-rail-inlet", "pad-rail-inlet", member="casing", part="member"), ref("st-pad-rail-outlet", "pad-rail-outlet", member="casing", part="member")]
    if s == 50:
        out += [ref("st-cassette-far", "cassette-far", member="pad", part="member"), ref("st-cassette-near", "cassette-near", member="pad", part="member")]
        out += [ref("st-trough", "trough", member="trough", part="member")]
    else:
        post = {100: 0.015, 200: 0.03}[s]
        zc = G.PAD_Z + 0.0075 if s == 100 else 0.184
        out.append(st(box("st-cassette-post", "pad", STEEL, 0.32, 0.0, (G.PAD_Y[0] + G.PAD_Y[1]) / 2, -zc, 2 * G.PAD_X, G.PAD_Y[1] - G.PAD_Y[0], post, repeat={"count": 2, "translation": [0, 0, 2 * zc]}), "pad", "member"))
        out += [ref("st-trough-floor", "trough-floor", member="trough", part="member")]
    seat = {50: 0.0075, 100: 0.015, 200: 0.03}[s]
    if s == 200:
        out.append(st(box("st-pad-seat", "pad", STEEL, 0.5, 0.0, 0.085, 0.0, 2 * G.PAD_X, seat, 2 * G.PAD_Z), "pad", "joint"))
    else:
        out.append(st(box("st-pad-seat", "pad", STEEL, 0.5, -G.RAIL_X, G.PAD_Y[0] + seat / 2, 0.0, 0.02, seat, 2 * G.PAD_Z, repeat={"count": 2, "translation": [2 * G.RAIL_X, 0, 0]}), "pad", "joint"))
        out.append(st(box("st-rail-fixing", "casing", STEEL, 0.5, -G.RAIL_X, 0.065, -hz + 0.01, 0.02, 0.03, 0.02, repeat={"count": 2, "translation": [2 * G.RAIL_X, 0, 0]}), "casing", "joint"))
        out.append(st(box("st-rail-fixing-far", "casing", STEEL, 0.5, -G.RAIL_X, 0.065, hz - 0.01, 0.02, 0.03, 0.02, repeat={"count": 2, "translation": [2 * G.RAIL_X, 0, 0]}), "casing", "joint"))
    zs = G.PAD_Z if s == 50 else (G.PAD_Z + 0.0075 if s == 100 else 0.184)
    out.append(st(box("st-trough-seat", "trough", PP, 0.5, 0.0, G.TROUGH_Y[0] + seat / 2, -zs, 2 * G.PAD_X, seat, seat, repeat={"count": 2, "translation": [0, 0, 2 * zs]}), "trough", "joint"))
    wr = {50: [0.11, 0.014, 0.1], 100: [0.11, 0.014, 0.1], 200: [0.1, 0.018, 0.09]}[s]
    wy = 0.037 if s != 200 else 0.05
    out.append(st(solid("mass-water", "ellipsoid", "water", WATER, 0.45, radii=wr, transform={"translation": [*G.loc("water", 0.0, wy), 0.0]}), "water", "mass"))
    if s == 50:
        out.append(st(solid("mass-pad", "ellipsoid", "pad", PAD, 0.7, radii=[0.03, 0.06, 0.1], transform={"translation": [*G.loc("pad", 0.0, 0.28), 0.0]}), "pad", "mass"))
    cr = {50: 0.045, 100: 0.05, 200: 0.055}[s]
    out.append(st(solid("mass-casing", "ellipsoid", "casing", STEEL, 0.35, radii=[cr] * 3, transform={"translation": [*G.loc("casing", 0.0, 0.26), 0.0]}), "casing", "mass"))
    if s == 200:
        out.append(st(box("base-contact-pad", "casing", STEEL, 0.4, 0.0, f / 2, 0.0, 0.42, f, 2 * hz), "casing", "support", anchor="base-contact"))
    else:
        out.append(st(box("base-contact-pad", "casing", STEEL, 0.4, -G.RUNNER_X, f / 2, 0.0, G.RUNNER_W, f, 2 * hz, repeat={"count": 2, "translation": [2 * G.RUNNER_X, 0, 0]}), "casing", "support", anchor="base-contact"))
    return out


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    50: [
        T("Detailed model of the real humidifier section, air flowing along x: a duct-section casing of 1 mm galvanised sheet (top, far side and floor folded as one profile, the near side a removable service cover with four quarter-turn fasteners) between two 30 mm connection frames whose bottom bars are the tank end walls, on two base runners of rectangular hollow section 40 by 20 by 2 mm with plastic end plugs; the lower 70 mm are the tank with 32 mm of water (6 L), a submersible pump with its riser hose, a float valve with lever and ball, and the fittings of the make-up water and the drain on the far wall.", "Detailliertes Modell des realen Befeuchterteils, Luftrichtung entlang x: ein Kanalgehäuse aus 1 mm verzinktem Blech (Deckel, hintere Seite und Boden als ein gekantetes Profil, die vordere Seite ein abnehmbarer Revisionsdeckel mit vier Schnellverschlüssen) zwischen zwei 30-mm-Anschlussrahmen, deren untere Stäbe die Stirnwände der Wanne sind, auf zwei Kufen aus Rechteckrohr 40 mal 20 mal 2 mm mit Kunststoffstopfen; die unteren 70 mm sind die Wanne mit 32 mm Wasser (6 l), einer Tauchpumpe mit Steigschlauch, einem Schwimmerventil mit Hebel und Kugel sowie den Verschraubungen von Frischwasser und Ablauf an der hinteren Wand."),
        T("The cellulose pad is 0.36 m wide, 0.40 m high and 0.20 m deep in the air direction, with 24 vertical flutes of 15 mm pitch on its inlet and outlet faces (two real 7 mm sheets per flute); it stands in two cassette side channels on two angle rails that span between the side walls and stand in the water; a bulkhead frame on the inlet side with a reveal to the cassette closes the bypass around the pad and dips into the water; the polypropylene trough of 3 mm with an anti-splash rim and two end caps lies on the cassette and holds a 10 mm water film; nine drops ride the prismatic drip bone: a row on each pad face and a row inside the pad on the section plane.", "Die Cellulosematte ist 0,36 m breit, 0,40 m hoch und in Luftrichtung 0,20 m tief, mit 24 senkrechten Wellen von 15 mm Teilung auf Ein- und Austrittsfläche (zwei reale 7-mm-Lagen je Welle); sie steht in zwei seitlichen Kassettenprofilen auf zwei Winkelschienen, die zwischen den Seitenwänden spannen und im Wasser stehen; ein Schottrahmen auf der Eintrittsseite mit Laibung zur Kassette schließt den Bypass um die Matte und taucht ins Wasser; die 3 mm starke Polypropylenrinne mit Spritzrand und zwei Endkappen liegt auf der Kassette und hält einen 10 mm hohen Wasserfilm; neun Tropfen sitzen auf dem Schubknochen drip: je eine Reihe auf beiden Mattenflächen und eine Reihe in der Matte auf der Schnittebene."),
        T("Thermal: the casing is its four outer skins of 1 mm (the inner faces look at the process air and are section faces), the connection frames (both faces lie against the duct run), the runners as closed bars, fasteners and fittings; the pad block, the film and the drops are the wet pad region (an exchange area, coverage only); trough, end caps and riser hose are the trough region; the tank water is the sump region; the vent emitter is an outlet disc of 0.3 m flush in the outlet frame. Bulkhead, rails, cassette, pump and float valve lie inside the airway or the water and belong to no region.", "Thermik: Das Gehäuse besteht aus seinen vier 1 mm dicken Außenhäuten (die Innenflächen sehen die Prozessluft und sind Schnittflächen), den Anschlussrahmen (beide Flächen liegen am Kanal an), den Kufen als geschlossenen Stäben, Verschlüssen und Verschraubungen; Mattenblock, Film und Tropfen sind die nasse Mattenregion (Austauschfläche, nur Abdeckung); Rinne, Endkappen und Steigschlauch sind die Rinnenregion; das Wannenwasser ist die Wannenregion; der Lüftungsemitter ist eine 0,3 m große Auslassscheibe bündig im Austrittsrahmen. Schott, Schienen, Kassette, Pumpe und Schwimmerventil liegen im Luftweg oder im Wasser und gehören zu keiner Region."),
        T("Structure: the pad sits in its two cassette channels (members of the pad) on seat joints over the two rails, the rails are fixed to both side walls, the trough channel spans the cassette on two seat joints, the tank floor carries the water, the casing is its four folded longitudinal edges between the two connection frames, and the two runners stand on contact pads; masses of water, pad and casing sit at their centres of mass.", "Tragwerk: Die Matte sitzt in ihren beiden Kassettenprofilen (Stäbe der Matte) auf Auflagergelenken über den zwei Schienen, die Schienen sind an beiden Seitenwänden befestigt, das Rinnenprofil spannt auf zwei Auflagern über die Kassette, der Wannenboden trägt das Wasser, das Gehäuse besteht aus seinen vier gekanteten Längskanten zwischen den beiden Anschlussrahmen, und die zwei Kufen stehen auf Kontaktflächen; die Massen von Wasser, Matte und Gehäuse sitzen in ihren Schwerpunkten."),
    ],
    100: [
        T("Redrawn for 1:100 (15 mm feature): the casing as one closed 1 mm shell (the service cover merged), two connection frames 15 mm deep, two solid runner bars of 40 by 20 mm, the tank water, two rail bars of 20 by 30 mm, the bulkhead as a 15 mm frame with a 15 mm reveal ring (the cassette channels merged into it), the pad with 12 flutes of 30 mm pitch, the trough as a 15 mm floor slab under a 3 mm wall ring with a 15 mm film, the pump as a plain cylinder with its riser hose, the float ball, and five drops (the row on the section plane and one on each pad face). Fasteners, fittings, float valve and lever are omitted.", "Für 1:100 neu gezeichnet (Merkmal 15 mm): das Gehäuse als eine geschlossene 1-mm-Hülle (der Revisionsdeckel geht darin auf), zwei 15 mm tiefe Anschlussrahmen, zwei volle Kufenstäbe von 40 mal 20 mm, das Wannenwasser, zwei Schienenstäbe von 20 mal 30 mm, das Schott als 15-mm-Rahmen mit 15-mm-Laibungsring (die Kassettenprofile gehen darin auf), die Matte mit 12 Wellen von 30 mm Teilung, die Rinne als 15 mm starke Bodenplatte unter einem 3-mm-Wandring mit 15 mm Film, die Pumpe als glatter Zylinder mit Steigschlauch, die Schwimmerkugel und fünf Tropfen (die Reihe auf der Schnittebene und je einer auf beiden Mattenflächen). Verschlüsse, Verschraubungen, Schwimmerventil und Hebel entfallen."),
        T("Thermal: the four casing skins, frames and runners; pad, film and drops; trough floor, wall ring and hose; tank water; the outlet disc is 15 mm deep. Structure: runners, tank floor, four 30 mm longitudinal edges, connection frames, rails with their fixings, two cassette posts beside the pad, the trough floor on its seats, the masses of water and of casing with pad, and the contact pads.", "Thermik: die vier Gehäusehäute, Rahmen und Kufen; Matte, Film und Tropfen; Rinnenboden, Wandring und Schlauch; Wannenwasser; die Auslassscheibe ist 15 mm tief. Tragwerk: Kufen, Wannenboden, vier 30-mm-Längskanten, Anschlussrahmen, Schienen mit ihren Befestigungen, zwei Kassettenpfosten neben der Matte, der Rinnenboden auf seinen Auflagern, die Massen von Wasser und von Gehäuse mit Matte sowie die Kontaktflächen."),
    ],
    200: [
        T("Generalised for 1:200 (30 mm feature): a 30 mm base slab (runners and tank floor merged), the casing shell above it, two connection frames 30 mm deep, the tank water as a 40 mm slab, the pad as one plain block from side wall to side wall that includes rails, reveal, cassette and the 20 mm side gaps, the bulkhead as a 30 mm frame, and the trough as a 30 mm floor slab under a 30 mm wall ring. Flutes, film, drops, pump, hose and float are omitted.", "Für 1:200 verallgemeinert (Merkmal 30 mm): eine 30 mm starke Bodenplatte (Kufen und Wannenboden gehen darin auf), darüber die Gehäusehülle, zwei 30 mm tiefe Anschlussrahmen, das Wannenwasser als 40-mm-Platte, die Matte als ein glatter Block von Seitenwand zu Seitenwand, der Schienen, Laibung, Kassette und die 20-mm-Seitenspalte einschließt, das Schott als 30-mm-Rahmen und die Rinne als 30 mm starke Bodenplatte unter einem 30 mm hohen Wandring. Wellen, Film, Tropfen, Pumpe, Schlauch und Schwimmer entfallen."),
        T("Thermal: the four casing skins, base and frames, the pad block, the trough parts, the water slab and a 30 mm outlet disc. Structure: the base slab under the water, four 45 mm longitudinal edges, the connection frames, two 30 mm cassette posts on one seat, the trough floor on its seats, the masses of water and casing, and one contact pad.", "Thermik: die vier Gehäusehäute, Bodenplatte und Rahmen, der Mattenblock, die Rinnenteile, die Wasserplatte und eine 30 mm tiefe Auslassscheibe. Tragwerk: die Bodenplatte unter dem Wasser, vier 45-mm-Längskanten, die Anschlussrahmen, zwei 30-mm-Kassettenpfosten auf einem Auflager, der Rinnenboden auf seinen Auflagern, die Massen von Wasser und Gehäuse sowie eine Kontaktfläche."),
    ],
}


def scale_volume(s):
    """📐 One authored scale."""
    b = body(s)
    return {"status": "authored", "denominator": s, "body": b, "thermal": thermal(s, b), "structure": structure(s), "decisions": DECISIONS[s]}


def volumes():
    """📦 The `volumes` member."""
    return {"50": scale_volume(50), "100": scale_volume(100), "200": scale_volume(200), "500": {"status": "unavailable", "denominator": 500}}
