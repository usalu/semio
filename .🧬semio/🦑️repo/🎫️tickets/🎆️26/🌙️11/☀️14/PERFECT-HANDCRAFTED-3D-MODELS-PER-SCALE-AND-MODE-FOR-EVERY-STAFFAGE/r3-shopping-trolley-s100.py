"""🛒 Round 3 shopping trolley 1:100 (feature 15 mm): the basket mesh at 60 mm with wires merged in pairs (o9, the same surface), two hoops, the chassis tubes with one bend each, castors as plate block, crown, two legs, tyre and hub, the tray with seven wires, the handle with grip, caps and coin lock."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ZT, ZW, PP, PU = "zinc-tube", "zinc-wire", "pp-grip", "pu-tyre"
TUBE_TONE, WIRE_TONE, MESH_TONE = 0.62, 0.55, 0.6
SIDES = (("l", 1), ("r", -1))
WR = 0.0045
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
        out += k.polyline("frame-%s" % n, "chassis", ZT, "body", TUBE_TONE, [C(p) for p in pts], k.TUBE_R)
        out.append(k.capsule("strut-%s" % n, "chassis", ZT, "body", TUBE_TONE, C((0.30, 0.18, s * 0.1987)), C((0.262, 0.562, s * 0.186)), k.STRUT_R))
        out.append(k.capsule("bracket-rim-%s" % n, "chassis", ZT, "detail", TUBE_TONE, C((k.upright_x(0.875), 0.875, s * k.RAIL_Z_REAR)), C((-0.391, 0.875, s * 0.255)), 0.0055))
        out.append(k.capsule("bracket-floor-%s" % n, "chassis", ZT, "detail", TUBE_TONE, C((k.upright_x(0.54), 0.54, s * k.RAIL_Z_REAR)), C((-0.357, 0.54, s * 0.21)), 0.005))
    out.append(k.capsule("front-cross", "chassis", ZT, "body", TUBE_TONE, C((k.RAIL_X_FRONT, k.RAIL_Y, -k.RAIL_Z_FRONT)), C((k.RAIL_X_FRONT, k.RAIL_Y, k.RAIL_Z_FRONT)), k.TUBE_R))
    return out


def castor(pos, n, s):
    c = k.CASTORS[pos]
    xs, zs = c["xs"], s * c["zs"]
    xw = xs - k.TRAIL
    tag = "%s-%s" % (pos, n)
    prof = [C((x, y))[:2] for x, y in k.fork_profile(xs)]
    out = [
        k.box("castor-block-%s" % tag, "chassis", ZT, "body", 0.6, [0.07, k.PLATE_Y[1] - k.HEAD_Y[0], 0.06], C((xs, (k.PLATE_Y[1] + k.HEAD_Y[0]) / 2, zs)), radius=0.004),
        k.box("castor-crown-%s" % tag, "chassis", ZT, "body", 0.6, [0.05, 0.008, 0.052], C((xs - 0.002, 0.138, zs)), radius=0.002),
        k.revolve("tyre-%s" % tag, "chassis", PU, "body", 0.18, [[-0.016, k.TYRE_IN], [-0.016, 0.057], [-0.011, k.WHEEL_R], [0.011, k.WHEEL_R], [0.016, 0.057], [0.016, k.TYRE_IN]], [0, 0, 1], C((xw, k.WHEEL_R, zs))),
        k.revolve("hub-%s" % tag, "chassis", PU, "detail", 0.78, [[-0.0185, 0.0], [-0.0185, 0.016], [-0.0125, 0.02], [-0.0125, 0.0485], [0.0125, 0.0485], [0.0125, 0.02], [0.0185, 0.016], [0.0185, 0.0]], [0, 0, 1], C((xw, k.WHEEL_R, zs))),
    ]
    for leg, a, b in (("o", 0.018, 0.026), ("i", -0.026, -0.018)):
        out.append(k.extrude("castor-fork-%s-%s" % (tag, leg), "chassis", ZT, "body", 0.6, prof, "xy", zs + a, zs + b))
    return out


def tray():
    out = []
    xr, xf = k.TRAY_X
    zr, zf = k.rail_z(xr), k.rail_z(xf)
    y, top = k.TRAY_Y, k.TRAY_Y + k.TRAY_LIP
    ring = [(xr, y, zr), (xf, y, zf), (xf, y, -zf), (xr, y, -zr), (xr, y, zr)]
    out += k.polyline("tray-frame", "chassis", ZW, "detail", WIRE_TONE, [C(p) for p in ring], 0.003, slender=True)
    for end, x, z in (("front", xf, zf - 0.01), ("rear", xr, zr - 0.01)):
        out.append(k.capsule("tray-lip-%s" % end, "chassis", ZW, "detail", WIRE_TONE, C((x, top, -z)), C((x, top, z)), 0.003, slender=True))
        for n, s in SIDES:
            out.append(k.capsule("tray-post-%s-%s" % (end, n), "chassis", ZW, "detail", WIRE_TONE, C((x, y, s * z)), C((x, top, s * z)), 0.004))
    for i in range(7):
        t = (i + 1) / 8.0
        out.append(k.capsule("tray-wire-%d" % (i + 1), "chassis", ZW, "detail", WIRE_TONE, C((xr, y + 0.004, -zr + 2 * zr * t)), C((xf, y + 0.004, -zf + 2 * zf * t)), 0.0037, slender=True))
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
        for i, x in enumerate(k.picket_xs(2, 0.185)):
            lo, hi = k.side_vertical(s, x)
            out.append(k.capsule("picket-%s-%d" % (n, i + 1), "basket", ZW, "detail", WIRE_TONE, lo, hi, WR, slender=True))
    for j, y in enumerate((0.105, 0.21)):
        rl, fl = k.side_horizontal(1, y)
        rr_, fr_ = k.side_horizontal(-1, y)
        out += k.polyline("wire-h-%d" % (j + 1), "basket", ZW, "detail", WIRE_TONE, [rl, fl, fr_, rr_], WR, slender=True)
    for i in range(7):
        z = -0.18 + 0.06 * i
        lo, hi = k.front_vertical(z)
        out.append(k.capsule("picket-f-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, lo, hi, WR, slender=True))
    for i in range(7):
        a, b = k.floor_long(i, 7, 0.025)
        out.append(k.capsule("floor-long-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, a, b, WR, slender=True))
    for i, x in enumerate((0.155, 0.31, 0.465)):
        h, y = k.floor_half(x), k.floor_y(x) - 0.0075
        out.append(k.capsule("floor-cross-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, [x, y, -h], [x, y, h], WR, slender=True))
    return out


def gate():
    out = []
    ut, ub = 0.015, 0.322
    ht, hb = k.gate_half(ut), k.gate_half(ub)
    ring = [[ut, 0, ht], [ub, 0, hb], [ub, 0, -hb], [ut, 0, -ht], [ut, 0, ht]]
    out += k.polyline("gate-frame", "gate", ZW, "detail", 0.5, ring, k.GATE_FRAME_WR, slender=True)
    out.append(k.sheet("gate-mesh", "gate", ZW, "glazing", MESH_TONE, ring[:4], 0.001))
    for i, w in enumerate((-0.17, 0.17)):
        out.append(k.capsule("gate-eye-%d" % (i + 1), "gate", ZW, "detail", 0.75, [-0.002, 0, w], [0.015, 0, w], 0.0075))
    for i in range(10):
        t = -1 + 2 * (i + 0.5) / 10
        out.append(k.capsule("gate-wire-v-%d" % (i + 1), "gate", ZW, "detail", WIRE_TONE, [ut, 0, t * (ht - 0.006)], [ub, 0, t * (hb - 0.006)], 0.004, slender=True))
    h = k.gate_half(0.17)
    out.append(k.capsule("gate-wire-h-1", "gate", ZW, "detail", WIRE_TONE, [0.17, 0.004, -h], [0.17, 0.004, h], 0.004, slender=True))
    return out


def handle():
    out = []
    for n, s in SIDES:
        pts = [(k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR), (ARM[0], ARM[1], s * k.RAIL_Z_REAR), (k.GRIP_C[0], k.GRIP_C[1], s * k.RAIL_Z_REAR)]
        out += k.polyline("arm-%s" % n, "handle", ZT, "body", TUBE_TONE, [H(p) for p in pts], k.TUBE_R)
        out.append(k.capsule("cap-%s" % n, "handle", PP, "detail", 0.4, [0, 0, s * 0.25], [0, 0, s * 0.267], k.CAP_R))
    out.append(k.capsule("cross-tube", "handle", ZT, "body", TUBE_TONE, [0, 0, -k.RAIL_Z_REAR], [0, 0, k.RAIL_Z_REAR], k.TUBE_R))
    out.append(k.capsule("grip", "handle", PP, "body", 0.4, [0, 0, -0.2325], [0, 0, 0.2325], k.GRIP_R))
    out.append(k.box("coin-lock", "handle", PP, "detail", 0.35, [0.06, 0.05, 0.10], [0.028, -0.012, 0], radius=0.008))
    return out


def body():
    out = chassis()
    for pos in ("rear", "front"):
        for n, s in SIDES:
            out += castor(pos, n, s)
    return out + tray() + basket() + gate() + handle()


def region_of(sid):
    if sid.startswith(("gate-wire", "gate-frame", "gate-eye")):
        return "gate-wire"
    if sid.startswith(("tyre-", "hub-")):
        return "wheels"
    if sid.startswith(("grip", "cap-", "coin-lock")):
        return "handle-grip"
    if sid.startswith(("frame-", "strut-", "bracket-", "front-cross", "castor-", "arm-", "cross-tube")):
        return "frame-tube"
    return "basket-wire"


def thermal(solids):
    return [{"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])} for s in solids if s["role"] != "glazing"]


def structure(solids):
    out = []
    for s in solids:
        sid = s["id"]
        member = None
        if sid.startswith(("frame-", "strut-", "front-cross", "castor-block", "castor-crown", "castor-fork", "hub-", "tyre-")):
            member = "chassis"
        elif sid.startswith(("rim-", "floor-frame", "corner-")):
            member = "basket"
        elif sid.startswith("gate-frame"):
            member = "gate"
        elif sid.startswith(("arm-", "cross-tube")):
            member = "handle"
        if member:
            out.append({"id": "st-" + sid, "of": sid, "member": member, "part": "member"})
        if sid.startswith("gate-eye"):
            out.append({"id": "st-" + sid, "of": sid, "member": "gate", "part": "joint"})
    for n, s in SIDES:
        for tag, p in (("rim", (k.upright_x(0.875), 0.875, s * k.RAIL_Z_REAR)), ("floor", (k.upright_x(0.54), 0.54, s * k.RAIL_Z_REAR)), ("strut", (0.262, 0.562, s * 0.186))):
            out.append(dict(k.ellipsoid("joint-basket-%s-%s" % (tag, n), "chassis", ZT, "detail", 0.5, [0.0115, 0.0115, 0.0115], C(p)), member="basket", part="joint"))
        out.append(dict(k.ellipsoid("joint-handle-%s" % n, "handle", ZT, "detail", 0.5, [0.0125, 0.0125, 0.0125], H((k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR))), member="handle", part="joint"))
    for pos in ("rear", "front"):
        c = k.CASTORS[pos]
        for n, s in SIDES:
            anchor = "wheel-%s-%s" % (pos, "left" if s > 0 else "right")
            out.append(dict(k.box("support-%s" % anchor, "chassis", PU, "ground", 0.3, [0.03, 0.015, 0.03], C((c["xs"] - k.TRAIL, 0.0075, s * c["zs"]))), member="chassis", part="support", anchor=anchor))
    return out


def masses(com):
    out = []
    for bone, (p, r) in com.items():
        if bone != "chassis":
            out.append(dict(k.ellipsoid("mass-%s" % bone, bone, ZT, "detail", 0.5, [max(r, 0.0075)] * 3, p), member=bone, part="mass"))
    return out + k.castor_masses(lambda i, p, r: dict(k.ellipsoid(i, "chassis", ZT, "detail", 0.5, [r, r, r], p), member="chassis", part="mass"), 0.015)


DECISIONS = [
    {"en": "1:100 (feature 15 mm): the basket keeps its form, rim, floor frame and corner wires; the mesh is drawn at 60 mm, so two o4.5 wires of 1:50 are merged into one o9 wire of the same surface (side and front pickets, two hoops, seven floor wires on three cross wires); the gate has ten o8 wires, one cross wire and two eyes; the crate tray seven wires.", "de": "1:100 (kleinstes Detail 15 mm): der Korb behält Form, Rand-, Bodenrahmen- und Eckdrähte; das Gitter ist im Abstand 60 mm gezeichnet, daher sind je zwei Drähte o4,5 von 1:50 zu einem Draht o9 gleicher Oberfläche zusammengefasst (Seiten- und Stirnstäbe, zwei Bügel, sieben Bodendrähte auf drei Querdrähten); die Klappe hat zehn Drähte o8, einen Querdraht und zwei Ösen; die Kistenablage sieben Drähte."},
    {"en": "Chassis tubes o25 with one bend at the rail end and one at the handle; castor head and plate are one block, the crown and both fork legs stay, tyre and hub are simplified profiles, the axle is dropped; the grip is a capsule.", "de": "Fahrgestellrohre o25 mit je einem Bogen am Holmende und am Griff; Rollenkopf und Platte sind ein Block, Gabelbrücke und beide Gabelschenkel bleiben, Reifen und Nabe sind vereinfachte Profile, die Achse entfällt; der Griff ist eine Kapsel."},
    {"en": "Thermal: every wire, tube, castor part, tyre, hub, grip, cap and the coin lock is its region's surface; the mesh planes are none. Structure: tubes, struts, castors, rim, floor frame, corner wires, gate frame and handle tube as members, joints at the six basket fixings, the handle bends and the gate eyes, contact pads at the wheel anchors, castor, basket, gate and handle masses.", "de": "Thermik: jeder Draht, jedes Rohr, Rollenteil, Reifen, Nabe, Griff, Kappe und das Pfandschloss ist die Oberfläche seiner Zone; die Gitterebenen sind keine. Tragwerk: Rohre, Stützen, Rollen, Rand, Bodenrahmen, Eckdrähte, Klappenrahmen und Griffrohr als Stäbe, Gelenke an den sechs Korbbefestigungen, den Griffbögen und den Klappenösen, Kontaktflächen an den Radankern, Massen der Rollen, des Korbs, der Klappe und des Griffs."},
]


def volume(com=None):
    b = body()
    return {"status": "authored", "denominator": 100, "body": b, "thermal": thermal(b), "structure": structure(b) + masses(com or {}), "decisions": DECISIONS}
