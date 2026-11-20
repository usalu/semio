"""🛢️ Round 3 hot-water cylinder: states every solid of every scale (1:50, 1:100, 1:200, 1:500) and list (body, thermal, structure) by hand and writes `volumes/appliances/hot-water-cylinder.json`.

Usage: python3 r3-hot-water-cylinder-model.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-hot-water-cylinder-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
SOURCE = os.path.join(HERE, "volumes", "appliances", "hot-water-cylinder.json")
TEXTS = os.path.join(HERE, "r3-hot-water-cylinder-texts.json")

ZT = {"zone-4": 0.9, "zone-3": 0.88, "zone-2": 0.86, "zone-1": 0.84}
TL4 = K.SHELL_LO - K.ORIGIN["zone-4"]
TL1 = K.SHELL_HI - K.ORIGIN["zone-1"]
HD = K.head_depth()


def bottom_outer(nk, nc):
    """⬇️ Outer bottom head of zone-4 from the rim down to the apex (axial, radial)."""
    return [(TL4 - h, r) for r, h in K.head(0.0, nk, nc)]


def bottom_inner(nk, nc):
    """⬇️ Inner bottom head of zone-4 from the apex up to the rim."""
    return [(TL4 - h, r) for r, h in reversed(K.head(K.TANK_S, nk, nc))]


def top_outer(nk, nc):
    """⬆️ Outer top head of zone-1 from the rim up to the apex."""
    return [(TL1 + h, r) for r, h in K.head(0.0, nk, nc)]


def top_inner(nk, nc):
    """⬆️ Inner top head of zone-1 from the apex down to the rim."""
    return [(TL1 + h, r) for r, h in reversed(K.head(K.TANK_S, nk, nc))]


def steel(zone, nk=4, nc=4, s=K.TANK_S):
    """🛢️ The enamelled steel vessel inside one quarter as a closed loop of its real 3 mm wall: lower shell and bottom head in zone-4, plain shell in zone-3 and zone-2, upper shell and top head in zone-1."""
    ro, ri = K.TANK_RO, K.TANK_RO - s
    if zone == "zone-4":
        return K.loop([(K.ZONE_H, ro), (TL4, ro)] + bottom_outer(nk, nc)[1:] + bottom_inner(nk, nc)[:-1] + [(TL4, ri), (K.ZONE_H, ri)])
    if zone == "zone-1":
        return K.loop([(0, ro), (TL1, ro)] + top_outer(nk, nc)[1:] + top_inner(nk, nc)[:-1] + [(TL1, ri), (0, ri)])
    return K.loop([(0, ro), (K.ZONE_H, ro), (K.ZONE_H, ri), (0, ri)])


def foam(zone):
    """🧽 The 60 mm PU foam of one quarter around the vessel; zone-4 includes the bottom insulation under the head, zone-1 the top insulation over it."""
    ro, rj = K.TANK_RO, K.JACKET_R
    if zone == "zone-4":
        return [(0, 0.290), (0.003, 0.297), (0.010, rj), (K.ZONE_H, rj), (K.ZONE_H, ro), (TL4, ro)] + bottom_outer(4, 4)[1:]
    if zone == "zone-1":
        return list(reversed(top_outer(4, 4)))[:-1] + [(TL1, ro), (0, ro), (0, rj), (K.ZONE_H, rj), (K.ZONE_H, 0)]
    return K.loop([(0, ro), (K.ZONE_H, ro), (K.ZONE_H, rj), (0, rj)])


def jacket(zone, edge):
    """🧥 The body of one quarter as seen: the closed cylinder of the jacket skin (zone-4 with its rolled bottom edge)."""
    if zone == "zone-4":
        return [(0, K.JACKET_R - edge), (edge * 0.3, K.JACKET_R - edge * 0.3), (edge, K.JACKET_R), (K.ZONE_H, K.JACKET_R)]
    return [(0, K.JACKET_R), (K.ZONE_H, K.JACKET_R)]


def dome(n, a=K.CAP_A, c=K.CAP_C):
    """🔘 The domed ABS cover as (axial, radial) from the rim to the apex."""
    return [(h, r) for r, h in K.ellipse(a, c, n)]


def cap_shell(n, t):
    """🔘 The ABS cover skin of thickness t as a closed loop."""
    outer = dome(n)
    inner = list(reversed(dome(n, K.CAP_A - t, K.CAP_C - t)))
    return K.loop(outer + inner)


def plume_patch(n, half):
    """🌫️ The outlet patch of the warm-air plume: the crown of the cover within r 0.2 m, a skin of 2 x half around the cover surface (D2: on the surface, never in the air)."""
    def arc(a, c):
        rs = [0.2 * (1 - i / n) for i in range(n + 1)]
        return [(c * math.sqrt(max(1 - (r / a) ** 2, 0)), r) for r in rs]
    return K.loop(arc(K.CAP_A + half, K.CAP_C + half) + list(reversed(arc(K.CAP_A - half, K.CAP_C - half))))


def stub_body(sid, side, y, length=0.045, r=0.012, scale=None):
    """🔩 A brass connection nipple from inside the jacket to its tip at |x| 0.33."""
    return K.rev(sid, "connections", "brass", "body", 0.7, [(0, r), (length, r)], origin=(side * (0.33 - length), y, 0), axis=(side, 0, 0), lateralScale=scale)


def nut(sid, side, y):
    """⬡ The union nut at the tip of a nipple (34 mm hexagon, 12 mm thick)."""
    a, b = (-0.33, -0.318) if side < 0 else (0.318, 0.33)
    return K.solid("extrude", sid, "connections", "brass", "body", 0.45, profile=K.hexagon(y, 0.0, 0.019), plane="yz", **{"from": a, "to": b})


def foot_profile():
    """🦶 A levelling foot: 50 mm pad with a chamfered top, M16 screw, 36 mm welded socket reaching into the plinth."""
    return [(0, 0.025), (0.006, 0.025), (0.010, 0.019), (0.010, 0.008), (0.028, 0.008), (0.028, 0.018), (0.045, 0.018)]


def seam_bead(bone, sid, y0, y1):
    """🧵 The vertical lock seam of the jacket skin on the -z side at x 0.22 (texture)."""
    x = 0.22
    f = (K.JACKET_R + 0.0005) / K.JACKET_R
    z = K.jacket_point(x)
    return K.solid("capsule", sid, bone, "foam-jacket", "detail", 0.4, p0=[K.r5(x * f), K.r5(y0), K.r5(z * f)], p1=[K.r5(x * f), K.r5(y1), K.r5(z * f)], r0=0.0015, r1=0.0015, texture=True)


def tick(sid, ang):
    """➖ A tick of the thermometer scale on the glass (texture)."""
    a = math.radians(ang)
    p0 = (0.034 * math.sin(a), K.DIAL_Y + 0.034 * math.cos(a))
    p1 = (0.042 * math.sin(a), K.DIAL_Y + 0.042 * math.cos(a))
    return K.solid("capsule", sid, "zone-2", "foam-jacket", "detail", 0.4, p0=[K.r5(p0[0]), K.r5(p0[1]), -0.3085], p1=[K.r5(p1[0]), K.r5(p1[1]), -0.3085], r0=0.0008, r1=0.0008, texture=True)


def body50():
    """🎨 1:50 body: three levelling feet, plinth, four jacket quarters, domed cover with its seam, five nipples with union nuts, hot-water outlet, thermometer, type plate, inspection cover, vertical skin seam."""
    out = []
    fx, fz = K.FEET[0]
    out.append(K.rev("foot-l", "base", "powder-steel", "body", 0.35, foot_profile(), origin=(fx, 0, fz)))
    fx, fz = K.FEET[1]
    out.append(K.rev("foot-r", "base", "powder-steel", "body", 0.35, foot_profile(), origin=(fx, 0, fz), repeat={"count": 2, "translation": [0, 0, K.r5(2 * -fz)]}))
    out.append(K.rev("base-ring", "base", "powder-steel", "body", 0.4, [(0, 0.272), (0.008, 0.28), (0.025, 0.28)], origin=(0, 0.04, 0)))
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev(z, z, "foam-jacket", "body", ZT[z], jacket(z, 0.010)))
    out.append(K.rev("cap", "cap", "cap-plastic", "body", 0.78, dome(16)))
    ys = 0.0565
    rs = K.CAP_A * math.sqrt(1 - (ys / K.CAP_C) ** 2)
    nr, ny = rs / K.CAP_A ** 2, ys / K.CAP_C ** 2
    nl = math.hypot(nr, ny)
    out.append(K.rev("cap-seam", "cap", "cap-plastic", "detail", 0.4, K.bead(ys + 0.001 * ny / nl, rs + 0.001 * nr / nl, 0.0015), texture=True))
    for sid, side, y in K.STUBS:
        out.append(stub_body(sid, side, y))
        out.append(nut(sid.replace("stub", "nut"), side, y))
    out.append(K.rev("hot-out", "connections", "brass", "body", 0.7, [(0, 0.015), (0.048, 0.015)], origin=(K.HOT_X, 1.43, 0)))
    out.append(K.rev("dial-bezel", "zone-2", "powder-steel", "detail", 0.95, [(0, 0.05), (0.019, 0.05), (0.023, 0.046)], origin=(0, K.DIAL_Y, -0.285), axis=(0, 0, -1)))
    out.append(K.rev("dial", "zone-2", "window-glass", "glazing", 0.95, K.loop([(0.023, 0.0), (0.023, 0.044), (0.0245, 0.044), (0.0245, 0.0)]), origin=(0, K.DIAL_Y, -0.285), axis=(0, 0, -1), texture=True))
    out += [tick("tick-cold", -70), tick("tick-mid", 0), tick("tick-hot", 70)]
    out.append(K.solid("capsule", "needle", "needle", "window-glass", "detail", 0.2, p0=[0, 0, -0.309], p1=[0, 0.034, -0.309], r0=0.0012, r1=0.0012, texture=True))
    out.append(K.rev("label", "zone-3", "foam-jacket", "detail", 0.4, K.loop([(0.085, 0.3), (0.185, 0.3), (0.185, 0.3015), (0.085, 0.3015)]), sweepRad=0.68, transform={"rotation": [0, 0.57728, 0, 0.81654]}, texture=True))
    out.append(K.rev("inspection-cover", "zone-4", "cap-plastic", "detail", 0.8, [(0, 0.08), (0.03, 0.08), (0.036, 0.074), (0.04, 0.06)], origin=(0, 0.17, -0.27), axis=(0, 0, -1)))
    for z, y0, y1 in (("zone-4", 0.012, K.ZONE_H), ("zone-3", 0, K.ZONE_H), ("zone-2", 0, K.ZONE_H), ("zone-1", 0, K.ZONE_H)):
        out.append(seam_bead(z, "seam-" + z[-1], y0, y1))
    return out


def thermal50():
    """🌡️ 1:50 thermal: per quarter the nested layers, innermost first: the 3 mm enamelled steel wall of the vessel as a real shell and the foam jacket as the closed quarter it bounds (the product's outer area has no C31 cavity flood, so a 60 mm foam annulus would count its inner face against the steel); the cover; the exposed parts of the nipples, the nuts and the outlet; the plume outlet patch on the crown of the cover."""
    out = []
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev("th-steel-" + z[-1], z, "enamelled-steel", "body", 0.5, steel(z), region=z, layer="tank-wall"))
        out.append({"id": "th-jacket-" + z[-1], "of": z, "region": z, "layer": "jacket", "sectionFaces": ["-v", "+v"]})
    out.append({"id": "th-cap", "of": "cap", "region": "top-cap", "layer": "shell", "sectionFaces": ["-v"]})
    for sid, side, y in K.STUBS:
        out.append(K.rev("th-" + sid, "connections", "brass", "body", 0.7, [(0, 0.012), (0.03, 0.012)], origin=(side * 0.30, y, 0), axis=(side, 0, 0), region="connections", layer="tube", sectionFaces=["+u" if side < 0 else "-u"]))
        out.append({"id": "th-" + sid.replace("stub", "nut"), "of": sid.replace("stub", "nut"), "region": "connections", "layer": "tube"})
    y0 = 1.38 + K.CAP_C * math.sqrt(1 - ((K.HOT_X + 0.015) / K.CAP_A) ** 2)
    out.append(K.rev("th-hot-out", "connections", "brass", "body", 0.7, [(0, 0.015), (1.478 - y0, 0.015)], origin=(K.HOT_X, y0, 0), region="connections", layer="tube", sectionFaces=["-v"]))
    out.append(K.rev("th-top-plume", "cap", "cap-plastic", "detail", 0.2, plume_patch(8, 0.004), region="top-cap", tags=["top-plume"]))
    return out


def collar():
    """🧱 The rigid PU top insulation between the upper shell course and the cover rim: bonded to the shell, it is what carries the cover (6 kg) into the vessel."""
    lo = K.SHELL_HI - 0.01 - K.ORIGIN["cap"]
    return K.rev("cover-collar", "cap", "foam-jacket", "body", 0.8, K.loop([(lo, K.TANK_RO - 0.005), (0.0, K.TANK_RO - 0.005), (0.0, K.JACKET_R - 0.004), (lo, K.JACKET_R - 0.004)]), member="cap", part="member")


def tube(sid, bone, material, origin, axis, length, ro, ri, member, part="member"):
    """🧪 A hollow tube of real wall as a closed loop along an axis."""
    return K.rev(sid, bone, material, "body", 0.45, K.loop([(0, ro), (length, ro), (length, ri), (0, ri)]), origin=origin, axis=axis, member=member, part=part)


def structure50():
    """🏗️ 1:50 structure: the pressure vessel (four 3 mm shell courses, Kloepper heads), girth welds, three welded legs with levelling screws, contact pads at the two support anchors, nipples welded into the shell, the cover skin, a mass at the centre of each quarter, the needle pivot."""
    out = []
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev("shell-" + z[-1], z, "enamelled-steel", "body", 0.6, steel(z), member=z, part="member"))
    out.append(K.rev("cover-skin", "cap", "cap-plastic", "body", 0.75, cap_shell(12, K.CAP_SKIN), member="cap", part="member"))
    out.append(K.rev("weld-bottom", "zone-4", "enamelled-steel", "detail", 0.3, K.bead(TL4, K.TANK_RO + 0.001, 0.004), member="zone-4", part="joint"))
    out.append(K.rev("weld-top", "zone-1", "enamelled-steel", "detail", 0.3, K.bead(TL1, K.TANK_RO + 0.001, 0.004), member="zone-1", part="joint"))
    rt = 0.20
    yt = K.SHELL_LO - K.head_height(rt) + 0.004
    out.append(K.solid("capsule", "legs", "base", "powder-steel", "body", 0.45, p0=[-rt, K.r5(yt), 0], p1=[-K.FOOT_PITCH, 0.045, 0], r0=0.012, r1=0.012, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="member"))
    out.append(K.solid("ellipsoid", "leg-welds", "base", "powder-steel", "detail", 0.3, radii=[0.016, 0.01, 0.016], transform={"translation": [-rt, K.r5(yt), 0]}, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="joint"))
    out.append(K.solid("capsule", "screws", "base", "powder-steel", "body", 0.45, p0=[-K.FOOT_PITCH, 0.008, 0], p1=[-K.FOOT_PITCH, 0.045, 0], r0=0.008, r1=0.008, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="member"))
    fx, fz = K.FEET[0]
    out.append(K.rev("support-foot-left", "base", "powder-steel", "body", 0.3, [(0, 0.025), (0.008, 0.025)], origin=(fx, 0, fz), member="base", part="support", anchor="foot-left"))
    fx, fz = K.FEET[1]
    out.append(K.rev("support-foot-right", "base", "powder-steel", "body", 0.3, [(0, 0.025), (0.008, 0.025)], origin=(fx, 0, fz), repeat={"count": 2, "translation": [0, 0, K.r5(-2 * fz)]}, member="base", part="support", anchor="foot-right"))
    for sid, side, y in K.STUBS:
        n = sid[-1]
        out.append(tube("pipe-" + n, "connections", "brass", (side * (K.TANK_RO - 0.005), y, 0), (side, 0, 0), 0.33 - K.TANK_RO + 0.005, 0.012, 0.0085, "connections"))
        out.append(K.rev("weld-pipe-" + n, "connections", "enamelled-steel", "detail", 0.3, K.bead(0.009, 0.0135, 0.0042), origin=(side * (K.TANK_RO - 0.005), y, 0), axis=(side, 0, 0), member="connections", part="joint"))
    yh = K.SHELL_HI + K.head_height(K.HOT_X) - 0.006
    out.append(tube("pipe-hot", "connections", "brass", (K.HOT_X, yh, 0), (0, 1, 0), 1.478 - yh, 0.015, 0.011, "connections"))
    out.append(K.rev("weld-pipe-hot", "connections", "enamelled-steel", "detail", 0.3, K.bead(0.01, 0.0165, 0.0042), origin=(K.HOT_X, yh, 0), member="connections", part="joint"))
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.solid("ellipsoid", "mass-" + z[-1], z, "enamelled-steel", "detail", 0.6, radii=[0.03, 0.03, 0.03], transform={"translation": [0, 0.165, 0]}, member=z, part="mass"))
    out.append(K.solid("ellipsoid", "needle-pivot", "needle", "brass", "detail", 0.45, radii=[0.005, 0.005, 0.006], transform={"translation": [0, 0, -0.302]}, member="needle", part="joint"))
    out.append(collar())
    return out


def body100():
    """🎨 1:100 body: feet and plinth merged into a base drum, four quarters, cover, five nipples with the nut merged (24 x 40 mm), outlet, thermometer as a shallow dome."""
    out = [K.rev("feet", "base", "powder-steel", "body", 0.35, [(0, 0.235), (0.015, 0.25), (0.065, 0.25)])]
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev(z, z, "foam-jacket", "body", ZT[z], jacket(z, 0.015)))
    out.append(K.rev("cap", "cap", "cap-plastic", "body", 0.78, dome(8)))
    for sid, side, y in K.STUBS:
        out.append(stub_body(sid, side, y, scale=1.667))
    out.append(K.rev("hot-out", "connections", "brass", "body", 0.7, [(0, 0.015), (0.048, 0.015)], origin=(K.HOT_X, 1.43, 0), lateralScale=1.333))
    out.append(K.solid("ellipsoid", "dial", "zone-2", "powder-steel", "body", 0.95, radii=[0.05, 0.05, 0.02], transform={"translation": [0, K.DIAL_Y, -0.29]}))
    return out


def thermal100():
    """🌡️ 1:100 thermal: the layers merged into one solid per quarter (references to the jacket bodies, the cuts between quarters are section faces), the cover, the connections, the plume patch."""
    out = [{"id": "th-" + z, "of": z, "region": z, "layer": "jacket", "sectionFaces": ["-v", "+v"]} for z in ("zone-4", "zone-3", "zone-2", "zone-1")]
    out.append({"id": "th-cap", "of": "cap", "region": "top-cap", "layer": "shell", "sectionFaces": ["-v"]})
    for sid, side, y in K.STUBS:
        out.append({"id": "th-" + sid, "of": sid, "region": "connections", "layer": "tube"})
    out.append({"id": "th-hot-out", "of": "hot-out", "region": "connections", "layer": "tube"})
    out.append(K.rev("th-top-plume", "cap", "cap-plastic", "detail", 0.2, plume_patch(5, 0.008), region="top-cap", tags=["top-plume"]))
    return out


def structure100():
    """🏗️ 1:100 structure: vessel courses with heads of fewer facets, cover skin, two girth welds, three legs with screws merged, two contact pads, nipples as bars, four masses."""
    out = []
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev("shell-" + z[-1], z, "enamelled-steel", "body", 0.6, steel(z, 2, 3), member=z, part="member"))
    out.append(K.rev("cover-skin", "cap", "cap-plastic", "body", 0.75, cap_shell(8, K.CAP_SKIN), member="cap", part="member"))
    out.append(K.rev("weld-bottom", "zone-4", "enamelled-steel", "detail", 0.3, K.bead(TL4, K.TANK_RO, 0.0078, 4), member="zone-4", part="joint"))
    out.append(K.rev("weld-top", "zone-1", "enamelled-steel", "detail", 0.3, K.bead(TL1, K.TANK_RO, 0.0078, 4), member="zone-1", part="joint"))
    rt = 0.20
    yt = K.SHELL_LO - K.head_height(rt) + 0.004
    out.append(K.solid("capsule", "legs", "base", "powder-steel", "body", 0.45, p0=[-rt, K.r5(yt), 0], p1=[-K.FOOT_PITCH, 0.01, 0], r0=0.0125, r1=0.0125, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="member"))
    fx, fz = K.FEET[0]
    out.append(K.rev("support-foot-left", "base", "powder-steel", "body", 0.3, [(0, 0.03), (0.016, 0.03)], origin=(fx, 0, fz), member="base", part="support", anchor="foot-left"))
    fx, fz = K.FEET[1]
    out.append(K.rev("support-foot-right", "base", "powder-steel", "body", 0.3, [(0, 0.03), (0.016, 0.03)], origin=(fx, 0, fz), repeat={"count": 2, "translation": [0, 0, K.r5(-2 * fz)]}, member="base", part="support", anchor="foot-right"))
    for sid, side, y in K.STUBS:
        out.append(K.rev("pipe-" + sid[-1], "connections", "brass", "body", 0.45, [(0, 0.012), (0.33 - K.TANK_RO + 0.005, 0.012)], origin=(side * (K.TANK_RO - 0.005), y, 0), axis=(side, 0, 0), member="connections", part="member"))
    yh = K.SHELL_HI + K.head_height(K.HOT_X) - 0.006
    out.append(K.rev("pipe-hot", "connections", "brass", "body", 0.45, [(0, 0.015), (1.478 - yh, 0.015)], origin=(K.HOT_X, yh, 0), member="connections", part="member"))
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.solid("ellipsoid", "mass-" + z[-1], z, "enamelled-steel", "detail", 0.6, radii=[0.03, 0.03, 0.03], transform={"translation": [0, 0.165, 0]}, member=z, part="mass"))
    out.append(collar())
    return out


def body200():
    """🎨 1:200 body: base drum, lower and upper half of the jacket, cover, outlet, one nipple per side (50 mm)."""
    return [
        K.rev("feet", "base", "powder-steel", "body", 0.35, [(0, 0.25), (0.065, 0.25)]),
        K.rev("lower", "zone-4", "foam-jacket", "body", 0.9, [(0, 0.3), (2 * K.ZONE_H, 0.3)]),
        K.rev("upper", "zone-2", "foam-jacket", "body", 0.86, [(0, 0.3), (2 * K.ZONE_H, 0.3)]),
        K.rev("cap", "cap", "cap-plastic", "body", 0.78, dome(6)),
        K.rev("hot-out", "connections", "brass", "body", 0.7, [(0, 0.025), (0.06, 0.025)], origin=(0.20278, 1.42, 0)),
        K.rev("side-l", "connections", "brass", "body", 0.7, [(0, 0.025), (0.05, 0.025)], origin=(-0.28, 1.19875, 0), axis=(-1, 0, 0)),
        K.rev("side-r", "connections", "brass", "body", 0.7, [(0, 0.025), (0.05, 0.025)], origin=(0.28, 0.92375, 0), axis=(1, 0, 0)),
    ]


def slices(prefix, n_cap):
    """🌡️ Coarse thermal: one solid cylinder slice per quarter (layers merged, the cuts between the quarters and the bottom are section faces), the cover as a solid dome."""
    out = [K.rev(prefix + z, z, "foam-jacket", "body", ZT[z], [(0, 0.3), (K.ZONE_H, 0.3)], region=z, layer="jacket", sectionFaces=["-v", "+v"]) for z in ("zone-4", "zone-3", "zone-2", "zone-1")]
    out.append({"id": prefix + "cap", "of": "cap", "region": "top-cap", "layer": "shell", "sectionFaces": ["-v"]})
    return out


def thermal200():
    """🌡️ 1:200 thermal: slices, cover, the three nipples, the plume patch."""
    out = slices("th-", 6)
    out += [{"id": "th-" + s, "of": s, "region": "connections", "layer": "tube"} for s in ("hot-out", "side-l", "side-r")]
    out.append(K.rev("th-top-plume", "cap", "cap-plastic", "detail", 0.2, plume_patch(3, 0.016), region="top-cap", tags=["top-plume"]))
    return out


def structure200():
    """🏗️ 1:200 structure: vessel courses with three-point heads, cover skin, three legs, two pads, three nipple bars, four masses."""
    out = []
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev("shell-" + z[-1], z, "enamelled-steel", "body", 0.6, steel(z, 1, 2), member=z, part="member"))
    out.append(K.rev("cover-skin", "cap", "cap-plastic", "body", 0.75, cap_shell(5, K.CAP_SKIN), member="cap", part="member"))
    rt = 0.20
    yt = K.SHELL_LO - K.head_height(rt) + 0.004
    out.append(K.solid("capsule", "legs", "base", "powder-steel", "body", 0.45, p0=[-rt, K.r5(yt), 0], p1=[-K.FOOT_PITCH, 0.02, 0], r0=0.016, r1=0.016, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="member"))
    fx, fz = K.FEET[0]
    out.append(K.solid("box", "support-foot-left", "base", "powder-steel", "body", 0.3, size=[0.06, 0.03, 0.06], transform={"translation": [fx, 0.015, fz]}, member="base", part="support", anchor="foot-left"))
    fx, fz = K.FEET[1]
    out.append(K.solid("box", "support-foot-right", "base", "powder-steel", "body", 0.3, size=[0.06, 0.03, 0.06], transform={"translation": [fx, 0.015, fz]}, repeat={"count": 2, "translation": [0, 0, K.r5(-2 * fz)]}, member="base", part="support", anchor="foot-right"))
    out.append(K.rev("pipe-l", "connections", "brass", "body", 0.45, [(0, 0.016), (0.095, 0.016)], origin=(-0.235, 1.19875, 0), axis=(-1, 0, 0), member="connections", part="member"))
    out.append(K.rev("pipe-r", "connections", "brass", "body", 0.45, [(0, 0.016), (0.095, 0.016)], origin=(0.235, 0.92375, 0), axis=(1, 0, 0), member="connections", part="member"))
    yh = K.SHELL_HI + K.head_height(0.20278) - 0.006
    out.append(K.rev("pipe-hot", "connections", "brass", "body", 0.45, [(0, 0.016), (1.48 - yh, 0.016)], origin=(0.20278, yh, 0), member="connections", part="member"))
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.solid("ellipsoid", "mass-" + z[-1], z, "enamelled-steel", "detail", 0.6, radii=[0.035, 0.035, 0.035], transform={"translation": [0, 0.165, 0]}, member=z, part="mass"))
    out.append(collar())
    return out


def body500():
    """🎨 1:500 body: base drum 0.12 m (drawn at the 75 mm floor), lower and upper jacket, cover of four facets, outlet and side nipples as 110 mm blocks."""
    return [
        K.rev("feet", "base", "powder-steel", "body", 0.35, [(0, 0.25), (0.12, 0.25)]),
        K.rev("lower", "zone-4", "foam-jacket", "body", 0.9, [(0, 0.3), (2 * K.ZONE_H, 0.3)]),
        K.rev("upper", "zone-2", "foam-jacket", "body", 0.86, [(0, 0.3), (2 * K.ZONE_H, 0.3)]),
        K.rev("cap", "cap", "cap-plastic", "body", 0.78, dome(4)),
        K.solid("box", "hot-out", "connections", "brass", "body", 0.7, size=[0.11, 0.11, 0.11], transform={"translation": [0.20278, 1.425, 0]}),
        K.solid("box", "side-l", "connections", "brass", "body", 0.7, size=[0.11, 0.11, 0.11], transform={"translation": [-0.275, 1.19875, 0]}),
        K.solid("box", "side-r", "connections", "brass", "body", 0.7, size=[0.11, 0.11, 0.11], transform={"translation": [0.275, 0.92375, 0]}),
    ]


def thermal500():
    """🌡️ 1:500 thermal: slices, cover, the three blocks, the plume patch at the 75 mm floor."""
    out = slices("th-", 4)
    out += [{"id": "th-" + s, "of": s, "region": "connections", "layer": "tube"} for s in ("hot-out", "side-l", "side-r")]
    out.append(K.rev("th-top-plume", "cap", "cap-plastic", "detail", 0.2, plume_patch(2, 0.038), region="top-cap", tags=["top-plume"]))
    return out


def structure500():
    """🏗️ 1:500 structure: vessel courses with flat-faceted heads, cover skin and collar, three 76 mm legs, two pads, one connection bar, four masses."""
    out = []
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.rev("shell-" + z[-1], z, "enamelled-steel", "body", 0.6, steel(z, 1, 1), member=z, part="member"))
    out.append(K.rev("cover-skin", "cap", "cap-plastic", "body", 0.75, cap_shell(3, K.CAP_SKIN), member="cap", part="member"))
    rt = 0.19
    yt = K.SHELL_LO - K.head_height(rt) + 0.01
    out.append(K.solid("capsule", "legs", "base", "powder-steel", "body", 0.45, p0=[-rt, K.r5(yt), 0], p1=[-K.FOOT_PITCH, 0.06, 0], r0=0.038, r1=0.038, repeat={"count": 3, "rotationAxis": [0, 1, 0], "rotationRad": K.r5(2 * math.pi / 3)}, member="base", part="member"))
    fx, fz = K.FEET[0]
    out.append(K.solid("box", "support-foot-left", "base", "powder-steel", "body", 0.3, size=[0.075, 0.075, 0.075], transform={"translation": [fx, 0.0375, fz]}, member="base", part="support", anchor="foot-left"))
    fx, fz = K.FEET[1]
    out.append(K.solid("box", "support-foot-right", "base", "powder-steel", "body", 0.3, size=[0.075, 0.075, 0.075], transform={"translation": [fx, 0.0375, fz]}, repeat={"count": 2, "translation": [0, 0, K.r5(-2 * fz)]}, member="base", part="support", anchor="foot-right"))
    yh = K.SHELL_HI + K.head_height(0.20278) - 0.006
    out.append(K.solid("box", "pipe-hot", "connections", "brass", "body", 0.45, size=[0.075, K.r5(1.48 - yh), 0.075], transform={"translation": [0.20278, K.r5((1.48 + yh) / 2), 0]}, member="connections", part="member"))
    for z in ("zone-4", "zone-3", "zone-2", "zone-1"):
        out.append(K.solid("ellipsoid", "mass-" + z[-1], z, "enamelled-steel", "detail", 0.6, radii=[0.04, 0.04, 0.04], transform={"translation": [0, 0.165, 0]}, member=z, part="mass"))
    out.append(collar())
    return out


def main():
    texts = json.load(open(TEXTS, encoding="utf-8"))["volumes"]
    build = {"50": (body50, thermal50, structure50), "100": (body100, thermal100, structure100), "200": (body200, thermal200, structure200), "500": (body500, thermal500, structure500)}
    volumes = {}
    for den, (b, t, s) in build.items():
        volumes[den] = {"status": "authored", "denominator": int(den), "body": b(), "thermal": t(), "structure": s(), "decisions": texts[den]}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": volumes}, f, indent=1, ensure_ascii=False)
        f.write("\n")
    for den, v in volumes.items():
        print(den, {k: len(v[k]) for k in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
