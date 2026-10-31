"""diesel-generator generator (400 kVA silenced 20 ft container genset, side elevation, discharge hood at +x, origin mid-length on the ground)."""
import math
from common import *

HX = 3.03
CH = 2.59
CAP0 = 3.47

rig = Rig("container")
rig.add("container", None, 0, 0, 0, 6.06, joint_fixed(), "Container with engine, alternator, radiator and fuel base tank", "Container mit Motor, Generator, Kühler und Kraftstoff-Grundtank", 6554, (0, 1.0))
rig.add("silencer", "container", 0, 0, 0, 2.2, joint_fixed(), "Roof silencer", "Dachschalldämpfer", 350, (-0.9, 2.86))
rig.add("stack", "container", -1.2, 3.14, 90, 0.33, joint_fixed(), "Exhaust stack", "Abgasrohr", 90, (0.18, 0))
rig.add("cap-flap", "container", -1.05, CAP0, 180, 0.3, joint_rev(-100, 0), "Rain cap flap on the stack", "Regenklappe auf dem Abgasrohr", 6, (0.15, 0))
rig.add("outlet-hood", "container", 0, 0, 0, 1.13, joint_fixed(), "Radiator discharge hood", "Kühlerluft-Austrittshaube", 160, (2.5, 2.9))
rig.add("outlet-flap", "container", HX, 3.25, -90, 0.63, joint_rev(0, 80), "Discharge weather flap", "Wetterklappe des Luftaustritts", 40, (0.3, 0))
G = Geo(rig)
BW = rig.W


def lw(bone, pts):
    return [ap(BW[bone], p) for p in pts]


def rrect(x0, y0, x1, y1, r, n=4):
    out = []
    for cx, cy, a0 in ((x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180), (x1 - r, y0 + r, 270)):
        out += arc(cx, cy, r, a0, a0 + 90, n)
    return out


def silencer_poly(coarse):
    return rrect(-2.0, 2.59, 0.2, 3.14, 0.22 if coarse else 0.27, 3 if coarse else 5)


def shapes(den):
    s = []
    A = s.append
    if den == 500:
        A(G.rect("container-body", "container", "body", 2, -HX, 0.0, 2 * HX, CH, tone=0.5, regions=["enclosure-walls", "roof", "inlet-louvres", "skid-tank"], material="container-steel"))
        A(G.poly("silencer", "silencer", "body", 2.5, silencer_poly(True), tone=0.35, region="silencer", material="exhaust-steel"))
        A(G.rect("exhaust-stack", "stack", "body", 2.6, -1.3, 3.1, 0.2, 0.37, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
        A(G.poly("rain-cap", "cap-flap", "body", 2.7, lw("cap-flap", [(0, -0.1), (0.3, -0.1), (0.3, 0.0), (0, 0.0)]), tone=0.3, region="rain-cap", material="galvanised-steel"))
        A(G.poly("discharge-hood", "outlet-hood", "body", 2.5, [(1.9, CH), (HX, CH), (HX, 3.25), (2.2, 3.25), (1.9, 2.95)], tone=0.4, region="discharge-hood", material="container-steel"))
        A(G.poly("outlet-flap", "outlet-flap", "body", 2.6, lw("outlet-flap", [(0, -0.1), (0.63, -0.1), (0.63, 0.0), (0, 0.0)]), tone=0.3, region="outlet-flap", material="galvanised-steel"))
        return s
    A(G.rect("skid-tank", "container", "body", 1.9, -HX, 0.0, 2 * HX, 0.2, tone=0.3, region="skid-tank", material="container-steel"))
    A(G.rect("wall", "container", "body", 2, -HX, 0.2, 2 * HX, CH - 0.2, tone=0.55, region="enclosure-walls", material="container-steel"))
    A(G.rect("roof-band", "container", "body", 2.1, -HX, CH - 0.1, 2 * HX, 0.1, tone=0.4, region="roof", material="container-steel"))
    for k, (x0, x1) in enumerate(((-2.8, -1.5), (-0.2, 1.1))):
        A(G.rect(f"inlet-louvre-{k + 1}", "container", "detail", 2.2, x0, 0.55, x1 - x0, 1.5, tone=0.25, region="inlet-louvres", material="aluminium", hatch="louvre" if den <= 100 else None))
    A(G.rect("door-line", "container", "detail", 2.2, 1.5, 0.45, 0.75, 1.7, tone=0.5, filled=False, region="enclosure-walls", material="container-steel"))
    A(G.poly("silencer", "silencer", "body", 2.5, silencer_poly(False), tone=0.35, region="silencer", material="exhaust-steel"))
    A(G.rect("exhaust-stack", "stack", "body", 2.6, -1.3, 3.1, 0.2, 0.37, tone=0.2, region="exhaust-stack", material="exhaust-steel"))
    A(G.poly("rain-cap", "cap-flap", "body", 2.7, lw("cap-flap", [(0, -0.1), (0.3, -0.1), (0.3, 0.0), (0, 0.0)]), tone=0.3, region="rain-cap", material="galvanised-steel"))
    A(G.poly("discharge-hood", "outlet-hood", "body", 2.5, [(1.9, CH), (HX, CH), (HX, 3.25), (2.2, 3.25), (1.9, 2.95)], tone=0.4, region="discharge-hood", material="container-steel"))
    A(G.poly("outlet-flap", "outlet-flap", "body", 2.6, lw("outlet-flap", [(0, -0.06), (0.63, -0.06), (0.63, 0.0), (0, 0.0)]), tone=0.3, region="outlet-flap", material="galvanised-steel"))
    if den >= 100:
        A(G.rect("door-line-2", "container", "detail", 2.2, 2.35, 0.45, 0.5, 1.7, tone=0.5, filled=False, region="enclosure-walls", material="container-steel"))
        A(G.rect("silencer-clamp", "silencer", "detail", 2.55, -1.6, 2.59, 0.12, 0.55, tone=0.2, region="silencer", material="exhaust-steel"))
        A(G.rect("silencer-clamp-2", "silencer", "detail", 2.55, -0.5, 2.59, 0.12, 0.55, tone=0.2, region="silencer", material="exhaust-steel"))
        A(G.rect("corner-casting-l", "container", "detail", 1.95, -HX, 0.0, 0.25, 0.2, tone=0.2, region="skid-tank", material="container-steel"))
        A(G.rect("corner-casting-r", "container", "detail", 1.95, HX - 0.25, 0.0, 0.25, 0.2, tone=0.2, region="skid-tank", material="container-steel"))
    if den == 50:
        A(G.rect("door-handle", "container", "detail", 2.3, 1.55, 1.2, 0.06, 0.3, tone=0.2, region="enclosure-walls", material="container-steel"))
        A(G.rect("fuel-fill", "container", "detail", 2.3, -2.9, 0.05, 0.4, 0.1, tone=0.2, region="skid-tank", material="container-steel"))
        A(G.rect("lifting-eye", "container", "detail", 2.3, 2.5, CH, 0.3, 0.07, tone=0.2, region="roof", material="container-steel"))
        A(G.rect("silencer-flange", "silencer", "detail", 2.55, 0.0, 2.59, 0.08, 0.55, tone=0.2, region="silencer", material="exhaust-steel"))
    return s
