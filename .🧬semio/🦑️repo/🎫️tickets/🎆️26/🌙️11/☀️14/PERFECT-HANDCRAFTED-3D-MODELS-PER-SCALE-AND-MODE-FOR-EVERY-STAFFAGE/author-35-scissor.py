import importlib.util, os, math, json, glob
here = os.path.dirname(os.path.abspath(__file__))
os.environ["FRAMES"] = os.path.join(here, "🗑️generated", "author-35", "scissor.frames.json")
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-04-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("scissor-lift")
F = V.frames
SC = dict(material="scissor-steel", role="body")
CH = dict(material="chassis-paint", role="body")
DK = dict(material="deck-steel", role="body")
RL = dict(material="rail-steel", role="body")
RU = dict(material="rubber", role="body")
SCALES = (50, 100, 200)
T = {50: 0.008, 100: 0.015, 200: 0.03}
ARM = {50: (0.34, 0.25, 0.06), 100: (0.34, 0.22, 0.08), 200: (0.33, 0.15, 0.12)}


def W(bone, p):
    f = F[bone]
    return [f["o"][k] + p[0] * f["u"][k] + p[1] * f["v"][k] + p[2] * f["w"][k] for k in range(3)]


def AX(bone):
    f = F[bone]
    return [f["u"], f["v"], f["w"]]


def LB(id, bone, c, size, **m):
    return V.box(id, bone, W(bone, c), size, axes=AX(bone), **m)


def LC(id, bone, a, b, r, **m):
    return V.cyl(id, bone, W(bone, a), W(bone, b), r, **m)


def LX(id, bone, plane, profile, a0, a1, holes=None, rep=None, **m):
    d = V._meta(id, bone, "extrude", m)
    d["profile"] = [lib.rv(p) for p in profile]
    if holes:
        d["holes"] = [[lib.rv(p) for p in h] for h in holes]
    d["plane"] = plane
    d["from"], d["to"] = lib.r5(min(a0, a1)), lib.r5(max(a0, a1))
    if rep:
        d.update(V._repeat(bone, rep))
    return V.put(d)


def rect(y0, y1, z0, z1):
    return [(y0, z0), (y1, z0), (y1, z1), (y0, z1)]


def shell(prefix, bone, c, size, t, rep=None, **m):
    sx, sy, sz = size
    cx, cy, cz = c
    LB(f"{prefix}-bottom", bone, [cx, cy - sy / 2 + t / 2, cz], [sx, t, sz], **m)
    LB(f"{prefix}-top", bone, [cx, cy + sy / 2 - t / 2, cz], [sx, t, sz], **m)
    LB(f"{prefix}-side", bone, [cx, cy, cz + sz / 2 - t / 2], [sx, sy - 2 * t, t], rep={"count": 2, "step": [0, 0, -(sz - t)]}, **m)
    LB(f"{prefix}-end", bone, [cx + sx / 2 - t / 2, cy, cz], [t, sy - 2 * t, sz - 2 * t], rep={"count": 2, "step": [-(sx - t), 0, 0]}, **m)


def chassis(scale):
    V.at(scale, "body")
    t = T[scale]
    prof = [(-0.925, 0.14), (-0.9, 0.1), (0.9, 0.1), (0.925, 0.14), (0.925, 0.45), (-0.925, 0.45)]
    LX("chassis-core", "chassis", "xy", prof, -0.245, 0.245, holes=[[(-0.62, 0.17), (-0.17, 0.17), (-0.17, 0.43), (-0.62, 0.43)], [(-0.08, 0.17), (0.32, 0.17), (0.32, 0.43), (-0.08, 0.43)]], tone=0.45, **CH)
    wells = [(-0.82, -0.28), (0.28, 0.82)]
    pts = [(-0.925, -0.405)]
    for a, b in wells:
        pts += [(a, -0.405), (a, -0.245), (b, -0.245), (b, -0.405)]
    pts += [(0.925, -0.405), (0.925, 0.405)]
    for a, b in reversed(wells):
        pts += [(b, 0.405), (b, 0.245), (a, 0.245), (a, 0.405)]
    pts += [(-0.925, 0.405)]
    LX("chassis-deck", "chassis", "zx", [(z, x) for x, z in pts], 0.45 - max(t, 0.01), 0.45, tone=0.45, **CH)
    LB("battery-box", "chassis", [-0.395, 0.285, 0], [0.45, 0.23, 0.4], tone=0.3, material="chassis-paint", role="body")
    LB("pump-box", "chassis", [0.12, 0.285, 0], [0.4, 0.23, 0.4], tone=0.35, material="chassis-paint", role="body")
    e = {50: 0.01, 100: 0.015, 200: 0.03}[scale]
    LC("motor-vent-outlet", "chassis", [0.925, 0.3, 0], [0.925 + e, 0.3, 0], 0.05 if scale == 50 else 0.06, tone=0.9, material="chassis-paint", role="body", tags=["motor-vent"])
    LC("battery-plume-outlet", "chassis", [-0.4, 0.45, 0], [-0.4, 0.45 + e, 0], 0.15, tone=0.9, material="chassis-paint", role="body", tags=["battery-plume"])


def wheels(scale):
    V.at(scale, "body")
    for nm, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        if scale == 50:
            prof = [[0.0, 0.0], [0.0, 0.21], [0.02, 0.255], [0.05, 0.27], [0.11, 0.27], [0.14, 0.255], [0.16, 0.21], [0.16, 0.0]]
            for sg, zc in (("l", 0.245), ("r", -0.405)):
                V.rev(f"tyre-{nm}-{sg}", bone, W(bone, [0, 0, zc]), [0, 0, 1], prof, tone=0.12, **RU)
        else:
            for sg, zc in (("l", 0.245), ("r", -0.405)):
                LC(f"tyre-{nm}-{sg}", bone, [0, 0, zc], [0, 0, zc + 0.16], 0.27, tone=0.12, **RU)
        LC(f"hub-{nm}", bone, [0, 0, 0.265], [0, 0, 0.385], 0.11, rep={"count": 2, "step": [0, 0, -0.65]}, tone=0.6, **CH)
        LC(f"axle-{nm}", bone, [0, 0, -0.245], [0, 0, 0.245], 0.03, tone=0.3, **CH)


def arms(scale):
    V.at(scale, "body")
    ca, cb, aw = ARM[scale]
    for i in range(1, 6):
        for nm, c in (("a", ca), ("b", cb)):
            bone = f"{nm}{i}"
            LB(f"arm-{bone}", bone, [0.85, 0, c], [1.84, 0.12, aw], rep={"count": 2, "step": [0, 0, -2 * c]}, tone=0.4 if nm == "a" else 0.5, **SC)
    pr = {50: 0.015, 100: 0.025, 200: 0.03}[scale]
    cr = {50: 0.03, 100: 0.045, 200: 0.055}[scale]
    z0, z1 = cb - aw / 2, ca + aw / 2
    side = {"count": 2, "step": [0, 0, -(z0 + z1)]}
    for i in range(1, 6):
        LC(f"pivot-{i}", f"a{i}", [0.852, 0, z0], [0.852, 0, z1], cr, rep=side, tone=0.1, **SC)
    if scale <= 100:
        for i in range(1, 6):
            LC(f"pin-left-{i}", f"a{i}", [0, 0, z0], [0, 0, z1], pr, rep=side, tone=0.1, **SC)
            LC(f"pin-right-{i}", f"b{i}", [0, 0, z0], [0, 0, z1], pr, rep=side, tone=0.1, **SC)
        LC("pin-top-left", "b5", [1.7, 0, z0], [1.7, 0, z1], pr, rep=side, tone=0.1, **SC)
        LC("pin-top-right", "a5", [1.7, 0, z0], [1.7, 0, z1], pr, rep=side, tone=0.1, **SC)


def platform(scale):
    V.at(scale, "body")
    t = T[scale]
    LB("deck", "platform", [0.85, 0.05, 0], [1.85, 0.1, 0.74], tone=0.5, **DK)
    LB("deck-ext", "deck-ext", [-0.45, 0.05, 0], [0.9, 0.08, 0.7], tone=0.55, **DK)
    rw = 0.07 if scale == 200 else 0.05
    levels = (("top", 1.165), ("mid", 0.655)) if scale == 200 else (("top", 1.175), ("mid", 0.645), ("mid2", 0.925))
    zc = 0.335 if scale == 200 else 0.345
    xe = (-0.04, 1.74) if scale == 200 else (-0.05, 1.75)
    for nm, y in levels:
        LB(f"rail-{nm}-side", "platform", [0.85, y, zc], [1.85, rw, rw], rep={"count": 2, "step": [0, 0, -2 * zc]}, tone=0.3, **RL)
        LB(f"rail-{nm}-end", "platform", [xe[0], y, 0], [rw, rw, 2 * zc - rw], rep={"count": 2, "step": [xe[1] - xe[0], 0, 0]}, tone=0.3, **RL)
    LB("post", "platform", [xe[0], 0.65, zc], [rw, 1.1, rw], rep={"count": 2, "step": [xe[1] - xe[0], 0, 0]}, tone=0.3, **RL)
    LB("post-side", "platform", [xe[0], 0.65, -zc], [rw, 1.1, rw], rep={"count": 2, "step": [xe[1] - xe[0], 0, 0]}, tone=0.3, **RL)
    if scale == 50:
        LB("post-mid", "platform", [0.875, 0.625, 0.345], [0.05, 1.05, 0.05], rep={"count": 2, "step": [0, 0, -0.69]}, tone=0.3, **RL)
    tb = max(t, 0.008)
    th = 0.2 if scale == 200 else 0.15
    LB("toe-side", "platform", [0.85, 0.1 + th / 2, 0.37 - tb / 2], [1.85, th, tb], rep={"count": 2, "step": [0, 0, -(0.74 - tb)]}, tone=0.3, **RL)
    LB("toe-end", "platform", [-0.075 + tb / 2, 0.1 + th / 2, 0], [tb, th, 0.74 - 2 * tb], rep={"count": 2, "step": [1.85 - tb, 0, 0]}, tone=0.3, **RL)
    if scale == 50:
        LB("control-box", "platform", [1.51, 0.875, -0.3575], [0.22, 0.25, 0.095], tone=0.15, material="chassis-paint", role="body")


def region_of(i):
    for pre, reg in (("arm-", "arms"), ("pivot", "arms"), ("pin-", "arms"), ("chassis", "chassis"), ("battery-box", "battery-box"), ("battery-plume", "battery-box"), ("pump-box", "motor-pump"), ("motor-vent", "motor-pump"), ("tyre", "tyres"), ("hub", "tyres"), ("axle", "chassis"), ("deck", "deck")):
        if i.startswith(pre):
            return reg
    return "rails"


def derive(scale):
    body = V.data[scale]["body"]
    ids = {b["id"] for b in body}
    V.at(scale, "thermal")
    for b in body:
        V.ref("th-" + b["id"], b["id"], region=region_of(b["id"]), **({"tags": b["tags"]} if "tags" in b else {}))
    V.at(scale, "structure")

    def mem(id, of, bone, part="member"):
        if of in ids:
            V.ref(f"{part}-{id}", of, member=bone, part=part)

    tt = T[scale]
    ca, cb, aw = ARM[scale]
    for i in range(1, 6):
        for nm, c in (("a", ca), ("b", cb)):
            outer = rect(-0.06, 0.06, c - aw / 2, c + aw / 2)
            hole = rect(-0.06 + tt, 0.06 - tt, c - aw / 2 + tt, c + aw / 2 - tt) if (aw / 2 - tt) > 0.004 else None
            LX(f"member-arm-{nm}{i}", f"{nm}{i}", "yz", outer, -0.07, 1.77, holes=[hole] if hole else None, rep={"count": 2, "step": [0, 0, -2 * c]}, tone=0.4, member=f"{nm}{i}", part="member", **SC)
        mem(f"pivot-{i}", f"pivot-{i}", f"a{i}", "joint")
        if scale == 50:
            mem(f"pin-left-{i}", f"pin-left-{i}", f"a{i}", "joint")
            mem(f"pin-right-{i}", f"pin-right-{i}", f"b{i}", "joint")
    if scale == 50:
        mem("pin-top-left", "pin-top-left", "b5", "joint")
        mem("pin-top-right", "pin-top-right", "a5", "joint")
    for n in ("chassis-core", "chassis-deck"):
        mem(n, n, "chassis")
    cd = max(tt, 0.01)
    LB("member-deck-bottom", "platform", [0.85, cd / 2, 0], [1.85, cd, 0.74], tone=0.5, member="platform", part="member", **DK)
    LB("member-deck-top", "platform", [0.85, 0.1 - cd / 2, 0], [1.85, cd, 0.74], tone=0.5, member="platform", part="member", **DK)
    LB("member-deck-side", "platform", [0.85, 0.05, 0.37 - cd / 2], [1.85, 0.1 - 2 * cd, cd], rep={"count": 2, "step": [0, 0, -(0.74 - cd)]}, tone=0.5, member="platform", part="member", **DK)
    mem("deck-ext", "deck-ext", "deck-ext")
    for nm in ("f", "r"):
        mem(f"axle-{nm}", f"axle-{nm}", f"wheel-{nm}")
        if scale < 200:
            mem(f"hub-{nm}", f"hub-{nm}", f"wheel-{nm}", "joint")
    LB("member-slide-track", "slide", [-0.25, -0.025, 0], [0.5, 0.03, 0.2], tone=0.4, member="slide", part="member", **CH)
    mc = {50: [0.7, 0.04, 0.3], 100: [0.68, 0.05, 0.3], 200: [0.66, 0.06, 0.3]}[scale]
    V.box("mass-chassis", "chassis", [0, 0.13, 0], mc, tone=0.4, member="chassis", part="mass", **CH)
    mp = {50: [1.6, 0.03, 0.6], 100: [1.55, 0.03, 0.58], 200: [1.5, 0.03, 0.56]}[scale]
    LB("mass-platform", "platform", [0.85, 0.05, 0], mp, tone=0.4, member="platform", part="mass", **DK)


for sc in SCALES:
    chassis(sc)
    wheels(sc)
    arms(sc)
    platform(sc)

for sc in SCALES:
    derive(sc)
EN = {
 50: ("Detailed model of the lift stowed: chassis core with two pockets for the battery and pump boxes and a top plate with four wheel arches, four revolved tyres with rounded shoulders and axles, hubs and an axle tube per wheel; ten scissor arms of 0.12 x 0.06 m (outer pair at +-0.34 m, inner pair at +-0.25 m), each as one solid with a copy on the far side, pivot pins at the centre crossing and at both ends; platform deck as a 1.85 x 0.1 x 0.74 m solid with the sliding extension plate inside it, guard rails on three levels with corner and mid posts, toe boards and the control box outside the rail. Emitters are a 0.1 m outlet disc at the chassis end and a 0.3 m plume disc on the battery pocket.",
      "Thermal: every body solid is referenced by the region its 2D shape names; regions have one layer each, so no shells; the battery and pump boxes stand in the pockets so that their faces are exposed. Structure: the arms as hollow tubes (8 mm wall) inside the arm solids, the hollow deck tray (four plates) inside the deck solid, chassis core and top plate, the wheel axles and a slide track under the chassis for the sliding arm end are members; pivots, pins and hubs are joints; chassis and platform have a mass plate; the asset declares no supports."),
 100: ("Redrawn against 1:100: arms 0.08 m wide (outer pair at +-0.34 m, inner pair at +-0.22 m), pivots as 0.09 m tubes, pins and hubs as drawn, rails of 0.05 m without mid posts or control box; tyres plain cylinders; the arm tubes of the structure have 15 mm walls.",
       "Thermal and structure derived from the body; the structure drops the pins and keeps the pivots, axles, hubs and the slide track."),
 200: ("Redrawn against 1:200: arms 0.12 m wide (outer pair at +-0.33 m, inner pair at +-0.15 m), pivots 0.11 m, no end pins, rails and posts 0.07 m on two levels, toe board 0.2 m high, no control box, no hubs; chassis and deck as at 1:100.",
        "Thermal and structure derived from the body; the structure keeps the arm tubes (30 mm walls), the pivots, chassis, deck and axles."),
}
DE = {
 50: ("Detailmodell der Hebebühne im eingefahrenen Zustand: Fahrgestellkern mit zwei Taschen für Batterie- und Pumpenkasten und einer Deckplatte mit vier Radausschnitten, vier gedrehte Reifen mit gerundeten Schultern und Achsen, Naben und ein Achsrohr je Rad; zehn Scherenarme von 0,12 x 0,06 m (äußeres Paar bei +-0,34 m, inneres Paar bei +-0,25 m), jeder ein Körper mit Kopie auf der anderen Seite, Gelenkbolzen am Mittelkreuz und an beiden Enden; Plattformdeck als Körper 1,85 x 0,1 x 0,74 m mit der Ausziehplatte darin, Geländer in drei Höhen mit Eck- und Mittelpfosten, Fußleisten und der Steuerkasten außen am Geländer. Emitter sind eine Auslassscheibe von 0,1 m am Fahrgestellende und eine Fahnenscheibe von 0,3 m auf der Batterietasche.",
      "Thermik: Jeder Körper ist durch die Region seiner 2D-Form verwiesen; die Regionen haben je eine Schicht, daher keine Schalen; Batterie- und Pumpenkasten stehen in den Taschen, damit ihre Flächen frei liegen. Tragwerk: die Arme als Hohlrohre (8 mm Wand) in den Armkörpern, die Hohlwanne des Decks (vier Platten) im Deckkörper, Fahrgestellkern und Deckplatte, die Radachsen und eine Gleitbahn unter dem Fahrgestell für das gleitende Armende sind Stäbe; Drehpunkte, Bolzen und Naben sind Gelenke; Fahrgestell und Plattform haben eine Massenplatte; das Asset nennt keine Auflager."),
 100: ("Neu gezeichnet gegen 1:100: Arme 0,08 m breit (äußeres Paar bei +-0,34 m, inneres Paar bei +-0,22 m), Drehpunkte als Rohre von 0,09 m, Bolzen und Naben wie gezeichnet, Geländer 0,05 m ohne Mittelpfosten und Steuerkasten; Reifen einfache Zylinder; die Armrohre des Tragwerks haben 15 mm Wand.",
        "Thermik und Tragwerk aus dem Körper abgeleitet; das Tragwerk lässt die Bolzen weg und behält Drehpunkte, Achsen, Naben und die Gleitbahn."),
 200: ("Neu gezeichnet gegen 1:200: Arme 0,12 m breit (äußeres Paar bei +-0,33 m, inneres Paar bei +-0,15 m), Drehpunkte 0,11 m, keine Endbolzen, Geländer und Pfosten 0,07 m in zwei Höhen, Fußleiste 0,2 m hoch, kein Steuerkasten, keine Naben; Fahrgestell und Deck wie bei 1:100.",
        "Thermik und Tragwerk aus dem Körper abgeleitet; das Tragwerk behält die Armrohre (30 mm Wand), die Drehpunkte, Fahrgestell, Deck und Achsen."),
}
for sc in SCALES:
    for k in (0, 1):
        V.decide(sc, EN[sc][k], DE[sc][k])
V.write(unavailable=(500,))
