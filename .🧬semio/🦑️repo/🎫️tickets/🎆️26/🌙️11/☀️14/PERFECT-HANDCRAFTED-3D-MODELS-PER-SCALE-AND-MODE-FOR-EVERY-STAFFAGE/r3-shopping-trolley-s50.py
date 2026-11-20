"""🧺 Round 3 shopping trolley 1:50 (feature 7.5 mm): every wire of the basket, gate and crate tray, the o25 chassis tubes with their bends, four complete swivel castors, the handle with grip, caps and coin lock."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("trolley_kit", os.path.join(T, "r3-shopping-trolley-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

ZT, ZW, PP, PU = "zinc-tube", "zinc-wire", "pp-grip", "pu-tyre"
TUBE_TONE, WIRE_TONE, MESH_TONE = 0.62, 0.55, 0.6
SIDES = (("l", 1), ("r", -1))


def C(p):
    return k.local("chassis", p)


def H(p):
    return k.local("handle", p)


def chassis():
    out = []
    for n, s in SIDES:
        pts = [(k.RAIL_X_FRONT, k.RAIL_Y, s * k.RAIL_Z_FRONT), (k.RAIL_X_REAR, k.RAIL_Y, s * k.RAIL_Z_REAR)] + [(x, y, s * k.RAIL_Z_REAR) for x, y in k.BEND] + [(k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR)]
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
    out = [
        k.box("castor-plate-%s" % tag, "chassis", ZT, "body", 0.7, [0.07, 0.008, 0.06], C((xs, 0.151, zs)), radius=0.003),
        k.revolve("castor-head-%s" % tag, "chassis", ZT, "body", 0.5, [[0.0, 0.0], [0.0, 0.026], [0.003, 0.0285], [0.010, 0.0285], [0.013, 0.026], [0.013, 0.0]], [0, 1, 0], C((xs, k.HEAD_Y[0], zs))),
        k.box("castor-crown-%s" % tag, "chassis", ZT, "body", 0.6, [0.05, 0.008, 0.052], C((xs - 0.002, 0.138, zs)), radius=0.002),
        k.capsule("castor-axle-%s" % tag, "chassis", ZT, "detail", 0.8, C((xw, k.WHEEL_R, zs - 0.029)), C((xw, k.WHEEL_R, zs + 0.029)), 0.005),
        k.revolve("tyre-%s" % tag, "chassis", PU, "body", 0.18, k.tyre_profile(), [0, 0, 1], C((xw, k.WHEEL_R, zs))),
        k.revolve("hub-%s" % tag, "chassis", PU, "detail", 0.78, k.hub_profile(), [0, 0, 1], C((xw, k.WHEEL_R, zs))),
    ]
    prof = [C((x, y))[:2] for x, y in k.fork_profile(xs)]
    for leg, a, b in (("o", 0.0185, 0.026), ("i", -0.026, -0.0185)):
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
            out.append(k.capsule("tray-post-%s-%s" % (end, n), "chassis", ZW, "detail", WIRE_TONE, C((x, y, s * z)), C((x, top, s * z)), 0.00375))
    for i in range(13):
        t = (i + 1) / 14.0
        out.append(k.capsule("tray-wire-%d" % (i + 1), "chassis", ZW, "detail", WIRE_TONE, C((xr, y + 0.004, -zr + 2 * zr * t)), C((xf, y + 0.004, -zf + 2 * zf * t)), 0.002, slender=True))
    out.append(k.sheet("tray-mesh", "chassis", ZW, "glazing", MESH_TONE, [C(p) for p in ring[:4]], 0.001))
    return out


def basket():
    out = []
    L, Rt = k.side_corners(1), k.side_corners(-1)
    frl, ffl, rfl, rrl = L
    frr, ffr, rfr, rrr = Rt
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
        for i, x in enumerate(k.picket_xs(1)):
            lo, hi = k.side_vertical(s, x)
            out.append(k.capsule("picket-%s-%d" % (n, i + 1), "basket", ZW, "detail", WIRE_TONE, lo, hi, k.MESH_WR, slender=True))
    for j, y in enumerate((0.07, 0.14, 0.21, 0.28)):
        rl, fl = k.side_horizontal(1, y)
        rr_, fr_ = k.side_horizontal(-1, y)
        out.append(k.capsule("wire-h-%d-l" % (j + 1), "basket", ZW, "detail", WIRE_TONE, rl, fl, k.MESH_WR, slender=True))
        out.append(k.capsule("wire-h-%d-f" % (j + 1), "basket", ZW, "detail", WIRE_TONE, fl, fr_, k.MESH_WR, slender=True))
        out.append(k.capsule("wire-h-%d-r" % (j + 1), "basket", ZW, "detail", WIRE_TONE, fr_, rr_, k.MESH_WR, slender=True))
    for i in range(13):
        z = -0.18 + 0.03 * i
        lo, hi = k.front_vertical(z)
        out.append(k.capsule("picket-f-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, lo, hi, k.MESH_WR, slender=True))
    for i in range(13):
        a, b = k.floor_long(i, 13, 0.025)
        out.append(k.capsule("floor-long-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, a, b, k.MESH_WR, slender=True))
    for i, x in enumerate((0.1, 0.2, 0.3, 0.4, 0.5)):
        h, y = k.floor_half(x), k.floor_y(x) - 0.0045
        out.append(k.capsule("floor-cross-%d" % (i + 1), "basket", ZW, "detail", WIRE_TONE, [x, y, -h], [x, y, h], k.MESH_WR, slender=True))
    return out


def gate():
    out = []
    ut, ub = 0.015, 0.322
    ht, hb = k.gate_half(ut), k.gate_half(ub)
    ring = [[ut, 0, ht], [ub, 0, hb], [ub, 0, -hb], [ut, 0, -ht], [ut, 0, ht]]
    out += k.polyline("gate-frame", "gate", ZW, "detail", 0.5, ring, k.GATE_FRAME_WR, slender=True)
    out.append(k.sheet("gate-mesh", "gate", ZW, "glazing", MESH_TONE, ring[:4], 0.001))
    for i, w in enumerate((-0.17, 0.0, 0.17)):
        out.append(k.capsule("gate-eye-%d" % (i + 1), "gate", ZW, "detail", 0.75, [-0.002, 0, w], [0.015, 0, w], 0.0065))
    n = 20
    for i in range(n):
        t = -1 + 2 * (i + 0.5) / n
        out.append(k.capsule("gate-wire-v-%d" % (i + 1), "gate", ZW, "detail", WIRE_TONE, [ut, 0, t * (ht - 0.004)], [ub, 0, t * (hb - 0.004)], k.GATE_WIRE_WR, slender=True))
    for j, u in enumerate((0.12, 0.22)):
        h = k.gate_half(u)
        out.append(k.capsule("gate-wire-h-%d" % (j + 1), "gate", ZW, "detail", WIRE_TONE, [u, 0.003, -h], [u, 0.003, h], k.GATE_WIRE_WR, slender=True))
    return out


def handle():
    out = []
    for n, s in SIDES:
        pts = [(k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR)] + [(x, y, s * k.RAIL_Z_REAR) for x, y in k.ARM] + [(k.GRIP_C[0], k.GRIP_C[1], s * k.RAIL_Z_REAR)]
        out += k.polyline("arm-%s" % n, "handle", ZT, "body", TUBE_TONE, [H(p) for p in pts], k.TUBE_R)
        out.append(k.capsule("cap-%s" % n, "handle", PP, "detail", 0.4, [0, 0, s * 0.25], [0, 0, s * 0.267], k.CAP_R))
    out.append(k.capsule("cross-tube", "handle", ZT, "body", TUBE_TONE, [0, 0, -k.RAIL_Z_REAR], [0, 0, k.RAIL_Z_REAR], k.TUBE_R))
    out.append(k.revolve("grip", "handle", PP, "body", 0.4, [[-0.24, 0.0], [-0.24, 0.0155], [-0.238, k.GRIP_R], [0.238, k.GRIP_R], [0.24, 0.0155], [0.24, 0.0]], [0, 0, 1], [0, 0, 0]))
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
    if sid.startswith(("rim-", "floor-", "corner-", "picket-", "wire-h-", "tray-")):
        return "basket-wire"
    return None


def thermal(solids):
    out = []
    for s in solids:
        if s["role"] == "glazing":
            continue
        out.append({"id": "th-" + s["id"], "of": s["id"], "region": region_of(s["id"])})
    return out


def structure(solids):
    ids = [s["id"] for s in solids]
    out = []

    def ref(sid, member, part="member"):
        out.append({"id": "st-" + sid, "of": sid, "member": member, "part": part})

    for sid in ids:
        if sid.startswith(("frame-", "strut-", "bracket-", "front-cross", "castor-plate", "castor-head", "castor-crown", "castor-fork", "castor-axle", "hub-", "tyre-")):
            ref(sid, "chassis")
        elif sid.startswith(("rim-", "floor-frame", "corner-")):
            ref(sid, "basket")
        elif sid.startswith("gate-frame"):
            ref(sid, "gate")
        elif sid.startswith(("arm-", "cross-tube")):
            ref(sid, "handle")
        elif sid.startswith("gate-eye"):
            ref(sid, "gate", "joint")
    for n, s in SIDES:
        for tag, p, r in (("rim", (k.upright_x(0.875), 0.875, s * k.RAIL_Z_REAR), 0.012), ("floor", (k.upright_x(0.54), 0.54, s * k.RAIL_Z_REAR), 0.012), ("strut", (0.262, 0.562, s * 0.186), 0.0095)):
            out.append(dict(k.ellipsoid("joint-basket-%s-%s" % (tag, n), "chassis", ZT, "detail", 0.5, [r, r, r], C(p)), member="basket", part="joint"))
        out.append(dict(k.ellipsoid("joint-handle-%s" % n, "handle", ZT, "detail", 0.5, [0.0125, 0.0125, 0.0125], H((k.UPRIGHT_TOP[0], k.UPRIGHT_TOP[1], s * k.RAIL_Z_REAR))), member="handle", part="joint"))
    for pos in ("rear", "front"):
        for n, s in SIDES:
            c = k.CASTORS[pos]
            xw, zs = c["xs"] - k.TRAIL, s * c["zs"]
            anchor = "wheel-%s-%s" % (pos, "left" if s > 0 else "right")
            out.append(dict(k.box("support-%s" % anchor, "chassis", PU, "ground", 0.3, [0.03, 0.0075, 0.03], C((xw, 0.00375, zs))), member="chassis", part="support", anchor=anchor))
    return out


def masses(com):
    out = []
    for bone, (p, r) in com.items():
        if bone != "chassis":
            out.append(dict(k.ellipsoid("mass-%s" % bone, bone, ZT, "detail", 0.5, [r, r, r], p), member=bone, part="mass"))
    return out + k.castor_masses(lambda i, p, r: dict(k.ellipsoid(i, "chassis", ZT, "detail", 0.5, [r, r, r], p), member="chassis", part="mass"), 0.016)


DECISIONS = [
    {"en": "1:50 (feature 7.5 mm) shows the trolley as built: every wire of the welded basket (28 pickets per side at 30 mm, 13 on the sloping front wall, four horizontal hoops at 70 mm, 13 floor wires on five cross wires, o8 rim and o6 frame and corner wires), the gate panel with 20 o4 wires, two cross wires and three hinge eyes on the rear rim bar, and the crate tray on the rails with its lips. The mesh planes are thin glazing sheets so the basket reads translucent as the 2D draws it.", "de": "1:50 (kleinstes Detail 7,5 mm) zeigt den Wagen wie gebaut: jeden Draht des geschweißten Korbs (28 Stäbe je Seite im Abstand 30 mm, 13 an der schrägen Stirnwand, vier waagrechte Bügel im Abstand 70 mm, 13 Bodendrähte auf fünf Querdrähten, Randdraht o8 sowie Rahmen- und Eckdrähte o6), die Klappe mit 20 Drähten o4, zwei Querdrähten und drei Scharnierösen am hinteren Randstab und die Kistenablage auf den Holmen mit ihren Kanten. Die Gitterebenen sind dünne Verglasungsflächen, damit der Korb durchscheinend wirkt, wie ihn die 2D-Zeichnung zeigt."},
    {"en": "The chassis is two o25 tubes per side bent from the lower rail up into the push-frame uprights (9 degrees back lean), a front cross tube and two o20 front struts under the basket floor; the rails converge from 545 to 390 mm so the next chassis nests, and the rear stays open at the bottom. Each of the four swivel castors has its top plate, ball-race head, crown, two pressed fork legs (thickened to the 7.5 mm feature), axle, o125 x 32 tyre with rounded tread and nylon hub, trailing 35 mm.", "de": "Das Fahrgestell besteht je Seite aus einem Rohr o25, das vom unteren Holm in die Schiebebügel hochgebogen ist (9 Grad nach hinten geneigt), einem vorderen Querrohr und zwei vorderen Stützen o20 unter dem Korbboden; die Holme laufen von 545 auf 390 mm zusammen, damit der nächste Wagen einfährt, hinten unten bleibt das Gestell offen. Jede der vier Lenkrollen hat Kopfplatte, Kugellagerkopf, Gabelbrücke, zwei gepresste Gabelschenkel (auf das Detailmaß 7,5 mm verdickt), Achse, Rad o125 x 32 mit gerundeter Lauffläche und Nylonnabe, 35 mm nachlaufend."},
    {"en": "Thermal: every wire, tube, castor part, tyre, hub, grip, cap and the coin lock is its region's own surface (references), the glazing mesh planes are no surface. Structure: the bent tubes, struts, brackets and castors carry the chassis, rim ring, floor frame and corner wires the basket, the gate frame and its eyes the gate, the handle tube and arms the handle; joints at the basket fixings, the handle bends and the hinge; contact pads at the four wheel anchors; four mass points.", "de": "Thermik: jeder Draht, jedes Rohr, jedes Rollenteil, Reifen, Nabe, Griff, Kappe und das Pfandschloss ist die Oberfläche seiner Zone (Verweise), die Gitterebenen sind keine Oberfläche. Tragwerk: die gebogenen Rohre, Stützen, Laschen und Rollen tragen das Fahrgestell, Randring, Bodenrahmen und Eckdrähte den Korb, der Klappenrahmen mit Ösen die Klappe, Griffrohr und Arme den Griff; Gelenke an den Korbbefestigungen, den Griffbögen und dem Scharnier; Kontaktflächen an den vier Radankern; vier Massenpunkte."},
]


def volume(com=None):
    b = body()
    return {"status": "authored", "denominator": 50, "body": b, "thermal": thermal(b), "structure": structure(b) + masses(com or {}), "decisions": DECISIONS}
