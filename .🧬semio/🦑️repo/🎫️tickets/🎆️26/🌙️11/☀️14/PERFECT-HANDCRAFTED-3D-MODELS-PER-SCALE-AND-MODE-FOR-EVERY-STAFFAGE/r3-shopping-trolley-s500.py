"""🛒 Round 3 shopping trolley 1:500 (feature 75 mm): the translucent basket planes with the rim, the chassis as two slender bent rods straight from the rail end to the handle corner with the front cross rod, four castor blocks 75 mm wide, the gate panel, the handle bar."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ZT, ZW, PP, PU = "zinc-tube", "zinc-wire", "pp-grip", "pu-tyre"
TUBE_TONE, MESH_TONE = 0.62, 0.6
SIDES = (("l", 1), ("r", -1))
CORNER = k.SIGN_CORNER


def C(p):
    return k.local("chassis", p)


def chassis():
    out = []
    for n, s in SIDES:
        pts = [(k.RAIL_X_FRONT, k.RAIL_Y, s * k.RAIL_Z_FRONT), (CORNER[0], CORNER[1], s * k.RAIL_Z_REAR), (k.SIGN_TOP[0], k.SIGN_TOP[1], s * k.RAIL_Z_REAR)]
        out += k.polyline("frame-%s" % n, "chassis", ZT, "body", TUBE_TONE, [C(p) for p in pts], k.TUBE_R, slender=True)
    out.append(k.capsule("front-cross", "chassis", ZT, "body", TUBE_TONE, C((k.RAIL_X_FRONT, k.RAIL_Y, -k.RAIL_Z_FRONT)), C((k.RAIL_X_FRONT, k.RAIL_Y, k.RAIL_Z_FRONT)), 0.0095, slender=True))
    for pos in ("rear", "front"):
        c = k.CASTORS[pos]
        xs = c["xs"]
        xw = xs - k.TRAIL
        arc = [(xw + k.WHEEL_R * k.math.cos(a), k.WHEEL_R + k.WHEEL_R * k.math.sin(a)) for a in [k.math.radians(d) for d in range(150, 400, 20)]]
        prof = [C(p)[:2] for p in [(xs + 0.04, k.PLATE_Y[1]), (xs - 0.04, k.PLATE_Y[1]), (xs - 0.04, 0.11)] + arc + [(xs + 0.04, 0.11)]]
        for n, s in SIDES:
            zb = s * min(c["zs"], 0.2475)
            out.append(k.extrude("castor-%s-%s" % (pos, n), "chassis", PU, "body", 0.18, prof, "xy", zb - 0.0375, zb + 0.0375))
    return out


def basket():
    frl, ffl, rfl, rrl = k.side_corners(1)
    frr, ffr, rfr, rrr = k.side_corners(-1)
    return [
        k.sheet("mesh-side-l", "basket", ZW, "glazing", MESH_TONE, [frl, ffl, rfl, rrl], 0.001),
        k.sheet("mesh-side-r", "basket", ZW, "glazing", MESH_TONE, [frr, ffr, rfr, rrr], 0.001),
        k.sheet("mesh-front", "basket", ZW, "glazing", MESH_TONE, [ffr, ffl, rfl, rfr], 0.001),
        k.sheet("mesh-floor", "basket", ZW, "glazing", MESH_TONE, [frr, frl, ffl, ffr], 0.001),
    ] + k.polyline("rim", "basket", ZW, "detail", 0.5, [rrl, rfl, rfr, rrr, rrl], k.RIM_WR, slender=True)


def gate():
    ut, ub = 0.015, 0.322
    ht, hb = k.gate_half(ut), k.gate_half(ub)
    ring = [[ut, 0, ht], [ub, 0, hb], [ub, 0, -hb], [ut, 0, -ht]]
    return [k.sheet("gate-mesh", "gate", ZW, "glazing", MESH_TONE, ring, 0.001), k.capsule("gate-frame", "gate", ZW, "detail", 0.5, [ut, 0, 0], [ub, 0, 0], k.GATE_FRAME_WR, slender=True)]


def handle():
    x = k.local("handle", k.SIGN_TOP)[0]
    return [k.capsule("handle-bar", "handle", ZT, "body", TUBE_TONE, [x, 0, -k.RAIL_Z_REAR], [x, 0, k.RAIL_Z_REAR], 0.013, slender=True)]


def body():
    return chassis() + basket() + gate() + handle()


def thermal_only():
    """🌡️ Thermal-only surfaces the 1:500 drawing no longer shows: the grip sleeve as a slender o27 rod over the bar, and the mesh and gate wires merged into o24, o18 and o20 bundles in the mesh planes."""
    make = lambda i, a, b, r: k.capsule(i, "basket", ZW, "detail", 0.55, a, b, r, slender=True)
    out = [k.capsule("eq-grip", "handle", PP, "body", 0.4, [k.local("handle", k.SIGN_TOP)[0], 0, -0.2725], [k.local("handle", k.SIGN_TOP)[0], 0, 0.2725], 0.0135, slender=True)]
    out += k.bundles("eq", make, (0.06, 0.16, 0.26), (0.10, 0.18, 0.26), 4, 0.012, 0.009, 0.012)
    for n, s in SIDES:
        fr, ff, rf, rr = k.side_corners(s)
        out.append(make("eq-floor-edge-%s" % n, fr, ff, 0.012))
    for j, u in enumerate((0.11, 0.22)):
        h = k.gate_half(u) - 0.004
        out.append(k.capsule("eq-gate-%d" % (j + 1), "gate", ZW, "detail", 0.55, [u, 0, -h], [u, 0, h], 0.01, slender=True))
    return out


def region_of(sid):
    if sid.startswith(("gate-frame", "eq-gate")):
        return "gate-wire"
    if sid.startswith("castor-"):
        return "wheels"
    if sid.startswith("eq-grip"):
        return "handle-grip"
    if sid.startswith(("frame-", "front-cross", "handle-bar")):
        return "frame-tube"
    return "basket-wire"


def thermal(solids):
    refs = [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in solids if s["role"] != "glazing"]
    return refs + [dict(s, region=region_of(s["id"])) for s in thermal_only()]


def structure(solids):
    out = []
    for s in solids:
        sid = s["id"]
        member = {"frame": "chassis", "front": "chassis", "castor": "chassis", "rim": "basket", "gate": "gate", "handle": "handle"}.get(sid.split("-")[0])
        if member and s["role"] != "glazing":
            out.append({"id": "st-" + sid, "of": sid, "member": member, "part": "member"})
    out.append(dict(k.capsule("joint-gate-hinge", "gate", ZW, "detail", 0.5, [0.0, 0, -0.22], [0.0, 0, 0.22], 0.01, slender=True), member="gate", part="joint"))
    for pos in ("rear", "front"):
        c = k.CASTORS[pos]
        for n, s in SIDES:
            anchor = "wheel-%s-%s" % (pos, "left" if s > 0 else "right")
            out.append(dict(k.box("support-%s" % anchor, "chassis", PU, "ground", 0.3, [0.075, 0.075, 0.075], C((c["xs"] - k.TRAIL, 0.0375, s * min(c["zs"], 0.2475)))), member="chassis", part="support", anchor=anchor))
    return out


def masses(com):
    out = []
    for bone, (p, r) in com.items():
        if bone in ("basket", "handle"):
            out.append(dict(k.ellipsoid("mass-%s" % bone, bone, ZT, "detail", 0.5, [0.0375, 0.0375, 0.0375], p), member=bone, part="mass"))
    return out + k.castor_masses(lambda i, p, r: dict(k.ellipsoid(i, "chassis", ZT, "detail", 0.5, [r, r, r], p), member="chassis", part="mass"), 0.0375)


DECISIONS = [
    {"en": "1:500 (feature 75 mm): the basket is its four translucent mesh planes with the rim; the gate its panel with one frame line; the chassis two slender o25 rods from the o19 front cross rod along the rails and straight up to the handle corner 12 mm behind the grip axis (bends and arms are below the feature); each castor one 75 mm block of wheel, fork and plate; the handle one slender bar.", "de": "1:500 (kleinstes Detail 75 mm): der Korb besteht aus seinen vier durchscheinenden Gitterebenen mit dem Rand; die Klappe aus ihrer Tafel mit einer Rahmenlinie; das Fahrgestell aus zwei schlanken Stäben o25 vom vorderen Querstab o19 die Holme entlang und gerade hinauf zur Griffecke 12 mm hinter der Griffachse (Bögen und Arme liegen unter dem Detailmaß); jede Rolle ist ein Block von 75 mm aus Rad, Gabel und Platte; der Griff eine schlanke Stange."},
    {"en": "Merged: struts, tray, brackets, coin lock and caps are dropped (below 75 mm or hidden under the basket).", "de": "Zusammengefasst: Stützen, Ablage, Laschen, Pfandschloss und Kappen entfallen (unter 75 mm oder unter dem Korb verborgen)."},
    {"en": "Thermal: rods, rim, gate frame and castor blocks by reference, plus a thermal-only slender o27 grip sleeve over the bar and wire bundles that keep the mesh and gate surface. Structure: the rods, rim, gate frame, castor blocks and handle bar as members, the hinge joint, 75 mm contact blocks at the wheel anchors, castor, basket and handle masses.", "de": "Thermik: Stäbe, Rand, Klappenrahmen und Rollenblöcke als Verweise, dazu eine rein thermische schlanke Griffhülse o27 über der Stange und Drahtbündel, welche die Gitter- und Klappenoberfläche erhalten. Tragwerk: die Stäbe, Rand, Klappenrahmen, Rollenblöcke und Griffstange als Stäbe, das Scharniergelenk, Kontaktblöcke von 75 mm an den Radankern, Massen der Rollen, des Korbs und des Griffs."},
]


def volume(com=None):
    b = body()
    return {"status": "authored", "denominator": 500, "body": b, "thermal": thermal(b), "structure": structure(b) + masses(com or {}), "decisions": DECISIONS}
