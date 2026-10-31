"""scissor-lift generator (19 ft electric class: 5 scissor stages, platform height 5.8 m, side elevation, origin mid-chassis on the ground)."""
import math
from common import *

L = 1.7
NST = 5
H0 = 0.45
XA = -0.85
WRAD = 0.27
TH0 = math.degrees(math.asin(0.55 / (NST * L)))
TH_MAX = 38.1

rig = Rig("chassis")
rig.add("chassis", None, 0, 0, 0, 1.85, joint_fixed(), "Chassis with battery, motor-pump unit and drive hubs", "Fahrgestell mit Batterie, Motor-Pumpen-Einheit und Antriebsnaben", 760, (0, 0.28))
rig.add("wheel-f", "chassis", 0.55, WRAD, 0, WRAD, joint_rev(), "Front wheel", "Vorderrad", 45, (0, 0))
rig.add("wheel-r", "chassis", -0.55, WRAD, 0, WRAD, joint_rev(), "Rear wheel", "Hinterrad", 45, (0, 0))
rig.add("A1", "chassis", XA, H0, TH0, L, joint_rev(0, 45), "Scissor arm A1 (fixed foot)", "Scherenarm A1 (fester Fuß)", 50, (L / 2, 0))
rig.add("slide", "chassis", XA + L * math.cos(math.radians(TH0)), H0, 0, 0.3, joint_pris(0, -0.6, 0.0), "Rolling foot of arm B1", "Rollenfuß des Arms B1", 20, (0, 0))
rig.add("B1", "slide", 0, 0, 180 - TH0, L, joint_rev(-45, 0), "Scissor arm B1 (rolling foot)", "Scherenarm B1 (Rollenfuß)", 50, (L / 2, 0))
for k in range(2, NST + 1):
    rig.add(f"A{k}", f"B{k - 1}", L, 0, 2 * TH0 - 180, L, joint_rev(0, 90), f"Scissor arm A{k}", f"Scherenarm A{k}", 50, (L / 2, 0))
    rig.add(f"B{k}", f"A{k - 1}", L, 0, 180 - 2 * TH0, L, joint_rev(-90, 0), f"Scissor arm B{k}", f"Scherenarm B{k}", 50, (L / 2, 0))
rig.add("platform", f"B{NST}", L, 0, -(180 - TH0), 1.85, joint_rev(0, 45), "Work platform with guard rails", "Arbeitsbühne mit Geländer", 340, (0.85, 0.5))
rig.add("deck-ext", "platform", 0.9, 0, 0, 0.9, joint_pris(0, 0, 0.9), "Sliding deck extension", "Ausschiebbare Bühnenverlängerung", 40, (0.45, 0.03))
G = Geo(rig)
BW = rig.W


def lw(bone, pts):
    return [ap(BW[bone], p) for p in pts]


def arm_poly(w):
    return [(-0.07, -w / 2), (L + 0.07, -w / 2), (L + 0.07, w / 2), (-0.07, w / 2)]


def arms(den):
    s = []
    wd = {200: 0.12, 100: 0.12, 50: 0.12}[den]
    for i, bid in enumerate(["A1", "B1"] + [f"{c}{k}" for k in range(2, NST + 1) for c in "AB"]):
        k = int(bid[1:])
        s.append(G.poly(f"arm-{bid.lower()}", bid, "body", 3 + 0.05 * k + (0.02 if bid[0] == "B" else 0), lw(bid, arm_poly(wd)), tone=0.4 if bid[0] == "A" else 0.5, region="arms", material="scissor-steel"))
        if den >= 100:
            zz = 3.5 + 0.05 * k + (0.02 if bid[0] == "B" else 0)
            s.append(G.ell(f"pin-{bid.lower()}", bid, "detail", zz, *ap(BW[bid], (L / 2, 0)), 0.045 if den == 100 else 0.055, tone=0.1, region="arms", material="scissor-steel"))
    return s


def chassis(den):
    s = []
    A = s.append
    A(G.poly("chassis-body", "chassis", "body", 2, [(-0.925, 0.14), (-0.9, 0.1), (0.9, 0.1), (0.925, 0.14), (0.925, 0.45), (-0.925, 0.45)], tone=0.45, region="chassis", material="chassis-paint"))
    A(G.rect("battery-cover", "chassis", "detail", 2.2, -0.62, 0.17, 0.45, 0.23, tone=0.3, region="battery-box", material="chassis-paint"))
    A(G.rect("pump-cover", "chassis", "detail", 2.2, -0.08, 0.17, 0.4, 0.23, tone=0.35, region="motor-pump", material="chassis-paint"))
    for b, x in (("wheel-f", 0.55), ("wheel-r", -0.55)):
        A(G.ell(f"tyre-{b[-1]}", b, "body", 4, x, WRAD, WRAD, tone=0.12, region="tyres", material="rubber"))
        if den >= 100:
            A(G.ell(f"hub-{b[-1]}", b, "detail", 4.2, x, WRAD, 0.11, tone=0.6, region="tyres", material="chassis-paint"))
    return s


def platform(den):
    s = []
    A = s.append
    A(G.poly("deck-ext", "deck-ext", "body", 2.9, lw("deck-ext", [(-0.9, 0.01), (0.0, 0.01), (0.0, 0.09), (-0.9, 0.09)]), tone=0.55, region="deck", material="deck-steel"))
    A(G.poly("deck", "platform", "body", 3.6, lw("platform", [(-0.075, 0.0), (1.775, 0.0), (1.775, 0.1), (-0.075, 0.1)]), tone=0.5, region="deck", material="deck-steel"))
    t = 0.07 if den == 200 else 0.05
    rails = [("top-rail", -0.075, 1.2 - t, 1.85, t), ("mid-rail", -0.075, 0.62, 1.85, t), ("toe-board", -0.075, 0.1, 1.85, 0.2 if den == 200 else 0.15), ("post-rear", -0.075, 0.1, t, 1.1), ("post-front", 1.775 - t, 0.1, t, 1.1)]
    for rid, x, y, w, h in rails:
        A(G.poly(rid, "platform", "body" if den == 200 else "detail", 3.7, lw("platform", [(x, y), (x + w, y), (x + w, y + h), (x, y + h)]), tone=0.3, region="rails", material="rail-steel"))
    if den == 100 or den == 50:
        A(G.poly("mid-rail-2", "platform", "detail", 3.7, lw("platform", [(-0.075, 0.9), (1.775, 0.9), (1.775, 0.9 + t), (-0.075, 0.9 + t)]), tone=0.3, region="rails", material="rail-steel"))
    if den == 50:
        A(G.poly("post-mid", "platform", "detail", 3.7, lw("platform", [(0.85, 0.1), (0.85 + t, 0.1), (0.85 + t, 1.15), (0.85, 1.15)]), tone=0.3, region="rails", material="rail-steel"))
        A(G.poly("control-box", "platform", "detail", 3.8, lw("platform", [(1.4, 0.75), (1.62, 0.75), (1.62, 1.0), (1.4, 1.0)]), tone=0.15, region="rails", material="chassis-paint"))
    return s


def shapes(den):
    return arms(den) + chassis(den) + platform(den)


def bones_for(th, ext=0.0, wheel=0.0):
    d = math.radians(th)
    D = th - TH0
    out = {"A1": {"angleDeg": D}, "slide": {"dx": -L * (math.cos(math.radians(TH0)) - math.cos(d))}, "B1": {"angleDeg": -D}, "platform": {"angleDeg": D}}
    for k in range(2, NST + 1):
        out[f"A{k}"] = {"angleDeg": 2 * D}
        out[f"B{k}"] = {"angleDeg": -2 * D}
    if ext:
        out["deck-ext"] = {"dx": ext}
    if wheel:
        out["wheel-f"] = {"angleDeg": wheel}
        out["wheel-r"] = {"angleDeg": wheel}
    return {k: {kk: round(vv, 4) for kk, vv in v.items()} for k, v in out.items()}
