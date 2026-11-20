"""🛒 Round 3 shopping trolley 1:200 (feature 30 mm): the translucent basket with its edge wires and two hoops, the chassis as slender o25 tubes, castors as wheel discs under one housing block each, the crate tray as a plane in its frame, the gate panel, the handle with grip bar and coin lock."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ZT, ZW, PP, PU = "zinc-tube", "zinc-wire", "pp-grip", "pu-tyre"
TUBE_TONE, WIRE_TONE, MESH_TONE = 0.62, 0.55, 0.6
SIDES = (("l", 1), ("r", -1))
BEND = (-0.333, 0.20)
ARM = (-0.478, 0.972)


def C(p):
    return k.local("chassis", p)


def H(p):
    return k.local("handle", p)


def chassis():
    out = []
    for n, s in SIDES:
        pts = [(k.RAIL_X_FRONT, k.RAIL_Y, s * k.RAIL_Z_FRONT), (k.RAIL_X_REAR, k.RAIL_Y, s * k.RAIL_Z_REAR), (BEND[0], BEND[1], s * k.RAIL_Z_REAR), (k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR)]
        segs = k.polyline("frame-%s" % n, "chassis", ZT, "body", TUBE_TONE, [C(p) for p in pts], k.TUBE_R)
        segs[0]["slender"] = segs[2]["slender"] = True
        out += [segs[0], dict(segs[1], r0=0.015, r1=0.015), segs[2]]
        out.append(k.capsule("strut-%s" % n, "chassis", ZT, "body", TUBE_TONE, C((0.30, 0.18, s * 0.1987)), C((0.262, 0.562, s * 0.186)), 0.009, slender=True))
    out.append(k.capsule("front-cross", "chassis", ZT, "body", TUBE_TONE, C((k.RAIL_X_FRONT, k.RAIL_Y, -k.RAIL_Z_FRONT)), C((k.RAIL_X_FRONT, k.RAIL_Y, k.RAIL_Z_FRONT)), 0.015))
    return out


def castor(pos, n, s):
    c = k.CASTORS[pos]
    xs, zs = c["xs"], s * c["zs"]
    xw = xs - k.TRAIL
    tag = "%s-%s" % (pos, n)
    block = [C(p)[:2] for p in [(xs + 0.035, k.PLATE_Y[1]), (xs - 0.035, k.PLATE_Y[1]), (xs - 0.035, k.HEAD_Y[0]), (xw - 0.015, k.WHEEL_R), (xw + 0.015, k.WHEEL_R), (xs + 0.02, k.HEAD_Y[0]), (xs + 0.035, k.HEAD_Y[0])]]
    return [
        k.revolve("wheel-%s" % tag, "chassis", PU, "body", 0.18, [[-0.016, 0.0], [-0.016, 0.057], [-0.011, k.WHEEL_R], [0.011, k.WHEEL_R], [0.016, 0.057], [0.016, 0.0]], [0, 0, 1], C((xw, k.WHEEL_R, zs))),
        k.extrude("castor-%s" % tag, "chassis", ZT, "body", 0.6, block, "xy", zs - 0.03, zs + 0.03),
    ]


def tray():
    xr, xf = k.TRAY_X
    zr, zf = k.rail_z(xr), k.rail_z(xf)
    y, top = k.TRAY_Y, k.TRAY_Y + k.TRAY_LIP
    ring = [(xr, y, zr), (xf, y, zf), (xf, y, -zf), (xr, y, -zr), (xr, y, zr)]
    out = k.polyline("tray-frame", "chassis", ZW, "detail", WIRE_TONE, [C(p) for p in ring], 0.003, slender=True)
    for end, x, z in (("front", xf, zf - 0.01), ("rear", xr, zr - 0.01)):
        out.append(k.capsule("tray-lip-%s" % end, "chassis", ZW, "detail", WIRE_TONE, C((x, top, -z)), C((x, top, z)), 0.003, slender=True))
    out.append(k.sheet("tray-mesh", "chassis", ZW, "glazing", MESH_TONE, [C(p) for p in ring[:4]], 0.001))
    return out


def basket():
    out = []
    frl, ffl, rfl, rrl = k.side_corners(1)
    frr, ffr, rfr, rrr = k.side_corners(-1)
    out.append(k.sheet("mesh-side-l", "basket", ZW, "glazing", MESH_TONE, [frl, ffl, rfl, rrl], 0.001))
    out.append(k.sheet("mesh-side-r", "basket", ZW, "glazing", MESH_TONE, [frr, ffr, rfr, rrr], 0.001))
    out.append(k.sheet("mesh-front", "basket", ZW, "glazing", MESH_TONE, [ffr, ffl, rfl, rfr], 0.001))
    out.append(k.sheet("mesh-floor", "basket", ZW, "glazing", MESH_TONE, [frr, frl, ffl, ffr], 0.001))
    out += k.polyline("rim", "basket", ZW, "detail", 0.5, [rrl, rfl, rfr, rrr, rrl], k.RIM_WR, slender=True)
    out += k.polyline("floor-frame", "basket", ZW, "detail", WIRE_TONE, [frl, ffl, ffr, frr, frl], k.FRAME_WR, slender=True)
    for n, s in SIDES:
        fr, ff, rf, rr = k.side_corners(s)
        out.append(k.capsule("corner-rear-%s" % n, "basket", ZW, "detail", WIRE_TONE, fr, rr, k.FRAME_WR, slender=True))
        out.append(k.capsule("corner-front-%s" % n, "basket", ZW, "detail", WIRE_TONE, ff, rf, k.FRAME_WR, slender=True))
    for j, y in enumerate((0.105, 0.21)):
        rl, fl = k.side_horizontal(1, y)
        rr_, fr_ = k.side_horizontal(-1, y)
        out += k.polyline("wire-h-%d" % (j + 1), "basket", ZW, "detail", WIRE_TONE, [rl, fl, fr_, rr_], 0.003, slender=True)
    return out


def mesh_equivalent():
    """🌡️ Thermal-only wire bundles in the mesh planes: the 1:50 mesh surface the drawing no longer shows, as o18 horizontals and floor bundles, o12 tray and o16 gate bundles."""
    make = lambda i, a, b, r: k.capsule(i, "basket", ZW, "detail", WIRE_TONE, a, b, r, slender=True)
    out = k.bundles("eq", make, (0.05, 0.13, 0.17, 0.25), (0.07, 0.14, 0.21, 0.28), 4, 0.009, 0.009, 0.009)
    xr, xf = k.TRAY_X
    for i in range(4):
        t = (i + 0.5) / 4.0
        zr, zf = k.rail_z(xr) - 0.02, k.rail_z(xf) - 0.02
        out.append(k.capsule("eq-tray-%d" % (i + 1), "chassis", ZW, "detail", WIRE_TONE, k.local("chassis", (xr, k.TRAY_Y, -zr + 2 * zr * t)), k.local("chassis", (xf, k.TRAY_Y, -zf + 2 * zf * t)), 0.006, slender=True))
    for j, u in enumerate((0.08, 0.16, 0.24)):
        h = k.gate_half(u) - 0.004
        out.append(k.capsule("eq-gate-%d" % (j + 1), "gate", ZW, "detail", WIRE_TONE, [u, 0, -h], [u, 0, h], 0.008, slender=True))
    return out


def gate():
    ut, ub = 0.015, 0.322
    ht, hb = k.gate_half(ut), k.gate_half(ub)
    ring = [[ut, 0, ht], [ub, 0, hb], [ub, 0, -hb], [ut, 0, -ht], [ut, 0, ht]]
    return k.polyline("gate-frame", "gate", ZW, "detail", 0.5, ring, k.GATE_FRAME_WR, slender=True) + [k.sheet("gate-mesh", "gate", ZW, "glazing", MESH_TONE, ring[:4], 0.001)]


def handle():
    out = []
    for n, s in SIDES:
        pts = [(k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR), (ARM[0], ARM[1], s * k.RAIL_Z_REAR), (k.GRIP_C[0], k.GRIP_C[1], s * k.RAIL_Z_REAR)]
        out += k.polyline("arm-%s" % n, "handle", ZT, "body", TUBE_TONE, [H(p) for p in pts], 0.015)
    out.append(k.capsule("grip", "handle", PP, "body", 0.4, [0, 0, -k.RAIL_Z_REAR], [0, 0, k.RAIL_Z_REAR], k.GRIP_R))
    out.append(k.box("coin-lock", "handle", PP, "body", 0.35, [0.06, 0.05, 0.10], [0.028, -0.012, 0], radius=0.008))
    return out


def body():
    out = chassis()
    for pos in ("rear", "front"):
        for n, s in SIDES:
            out += castor(pos, n, s)
    return out + tray() + basket() + gate() + handle()


def region_of(sid):
    if sid.startswith(("gate-frame", "eq-gate")):
        return "gate-wire"
    if sid.startswith("wheel-"):
        return "wheels"
    if sid.startswith(("grip", "coin-lock")):
        return "handle-grip"
    if sid.startswith(("frame-", "strut-", "front-cross", "castor-", "arm-")):
        return "frame-tube"
    return "basket-wire"


def thermal(solids):
    refs = [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in solids if s["role"] != "glazing"]
    return refs + [dict(s, region=region_of(s["id"])) for s in mesh_equivalent()]


def structure(solids):
    out = []
    for s in solids:
        sid = s["id"]
        member = None
        if sid.startswith(("frame-", "strut-", "front-cross", "castor-", "wheel-")):
            member = "chassis"
        elif sid.startswith(("rim-", "floor-frame", "corner-")):
            member = "basket"
        elif sid.startswith("gate-frame"):
            member = "gate"
        elif sid.startswith(("arm-", "grip")):
            member = "handle"
        if member:
            out.append({"id": "st-" + sid, "of": sid, "member": member, "part": "member"})
    out.append(dict(k.capsule("joint-gate-hinge", "gate", ZW, "detail", 0.5, [0.0, 0, -0.2], [0.0, 0, 0.2], 0.008, slender=True), member="gate", part="joint"))
    for n, s in SIDES:
        for tag, p in (("rim", (k.upright_x(0.875), 0.875, s * k.RAIL_Z_REAR)), ("strut", (0.262, 0.562, s * 0.186))):
            out.append(dict(k.ellipsoid("joint-basket-%s-%s" % (tag, n), "chassis", ZT, "detail", 0.5, [0.015, 0.015, 0.015], C(p)), member="basket", part="joint"))
    for pos in ("rear", "front"):
        c = k.CASTORS[pos]
        for n, s in SIDES:
            anchor = "wheel-%s-%s" % (pos, "left" if s > 0 else "right")
            out.append(dict(k.box("support-%s" % anchor, "chassis", PU, "ground", 0.3, [0.03, 0.03, 0.03], C((c["xs"] - k.TRAIL, 0.015, s * c["zs"]))), member="chassis", part="support", anchor=anchor))
    return out


def masses(com):
    out = []
    for bone, (p, r) in com.items():
        if bone != "chassis":
            out.append(dict(k.ellipsoid("mass-%s" % bone, bone, ZT, "detail", 0.5, [0.015, 0.015, 0.015], p), member=bone, part="mass"))
    return out + k.castor_masses(lambda i, p, r: dict(k.ellipsoid(i, "chassis", ZT, "detail", 0.5, [r, r, r], p), member="chassis", part="mass"), 0.02)


DECISIONS = [
    {"en": "1:200 (feature 30 mm): the basket is its translucent mesh planes outlined by the o8 rim, the o6 floor frame and corner wires and two hoops; pickets and floor wires are no longer drawn. The gate is its panel in its frame, the crate tray a plane in its frame with two lips.", "de": "1:200 (kleinstes Detail 30 mm): der Korb besteht aus seinen durchscheinenden Gitterebenen, umrissen vom Randdraht o8, Bodenrahmen und Eckdrähten o6 und zwei Bügeln; Stäbe und Bodendrähte sind nicht mehr gezeichnet. Die Klappe ist ihre Tafel im Rahmen, die Kistenablage eine Ebene im Rahmen mit zwei Kanten."},
    {"en": "The chassis tubes and struts are slender o25 and o18 rods (the rear bend and the front cross tube thickened to the 30 mm feature); each castor is a o125 x 32 wheel under one 60 mm housing block (plate, head, crown and fork merged); the handle is two o30 arms, one o35 grip bar over caps and tube, and the coin lock.", "de": "Fahrgestellrohre und Stützen sind schlanke Stäbe o25 und o18 (der hintere Bogen und das vordere Querrohr auf das Detailmaß 30 mm verdickt); jede Rolle ist ein Rad o125 x 32 unter einem Gehäuseblock von 60 mm (Platte, Kopf, Brücke und Gabel zusammengefasst); der Griff besteht aus zwei Armen o30, einer Griffstange o35 über Kappen und Rohr und dem Pfandschloss."},
    {"en": "Thermal: the drawn wires, tubes, wheels, castor blocks, grip and coin lock by reference, plus thermal-only wire bundles that keep the mesh surface the drawing omits (the 1:50 pickets, hoops, floor and tray wires merged into o18 bundles along the walls and floor and o12 tray bundles, the gate wires into three o16 bundles). Structure: tubes, struts, castors and wheels, rim, floor frame, corner wires, gate frame and handle as members, the hinge and four basket fixings as joints, contact pads, masses.", "de": "Thermik: die gezeichneten Drähte, Rohre, Räder, Rollenblöcke, Griff und Pfandschloss als Verweise, dazu rein thermische Drahtbündel, welche die von der Zeichnung weggelassene Gitteroberfläche erhalten (die Stäbe, Bügel, Boden- und Ablagedrähte von 1:50 zu Bündeln o18 entlang Wänden und Boden und Ablagebündeln o12, die Klappendrähte zu drei Bündeln o16). Tragwerk: Rohre, Stützen, Rollen und Räder, Rand, Bodenrahmen, Eckdrähte, Klappenrahmen und Griff als Stäbe, Scharnier und vier Korbbefestigungen als Gelenke, Kontaktflächen, Massen."},
]


def volume(com=None):
    b = body()
    return {"status": "authored", "denominator": 200, "body": b, "thermal": thermal(b), "structure": structure(b) + masses(com or {}), "decisions": DECISIONS}
