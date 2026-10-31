"""heat-pump-outdoor-unit generator (12 kW class monobloc, side elevation, discharge to +x, origin at the centre of the footprint on the ground)."""
import math
from common import *

HW = 0.225
H = 1.45
FX = 0.26

rig = Rig("unit")
rig.add("unit", None, 0, 0, 0, 0.45, joint_fixed(), "Casing with fan, compressor, coil and feet", "Gehäuse mit Lüfter, Verdichter, Wärmetauscher und Füßen", 141, (0, 0.7))
rig.add("vane", "unit", FX, 0.8, 0, 0.22, joint_rev(-25, 25), "Discharge air guide vane", "Luftleitlamelle am Ausblas", 2, (0.11, 0))
rig.add("lid", "unit", -HW - 0.01, H, 0, 0.47, joint_rev(0, 100), "Service lid hinged at the rear top edge", "Wartungsdeckel, hinten oben angeschlagen", 7, (0.23, 0))
G = Geo(rig)
BW = rig.W


def lw(bone, pts):
    return [ap(BW[bone], p) for p in pts]


def shapes(den):
    s = []
    A = s.append
    if den == 500:
        A(G.poly("casing", "unit", "body", 2, [(-HW, 0), (HW, 0), (HW, 0.45), (FX, 0.5), (FX, 1.1), (HW, 1.15), (HW, 1.35), (-HW, 1.35)], tone=0.5, regions=["casing", "coil-guard", "compressor-casing", "base-tray", "fan-guard", "feet"], material="powder-coat"))
        A(G.poly("lid", "lid", "body", 2.5, lw("lid", [(0, -0.1), (0.47, -0.1), (0.47, 0.0), (0, 0.0)]), tone=0.35, region="top-lid", material="powder-coat"))
        A(G.poly("vane", "vane", "body", 2.6, lw("vane", [(-0.02, -0.06), (0.22, -0.06), (0.22, 0.06), (-0.02, 0.06)]), tone=0.3, region="discharge-vane", material="powder-coat"))
        return s
    th = 0.05 if den == 200 else 0.04
    if den == 200:
        A(G.rect("casing", "unit", "body", 2, -HW, 0.0, 2 * HW, 1.41, tone=0.55, regions=["casing", "feet"], material="powder-coat"))
    else:
        A(G.rect("feet", "unit", "body", 1.5, -0.17, 0.0, 0.34, 0.09, tone=0.2, region="feet", material="rubber"))
        A(G.rect("casing", "unit", "body", 2, -HW, 0.08, 2 * HW, 1.33, tone=0.55, region="casing", material="powder-coat"))
    A(G.poly("compressor-panel", "unit", "body", 2.2, [(-HW, 0.08), (HW, 0.08), (HW, 0.32), (-HW, 0.32)], tone=0.4, region="compressor-casing", material="powder-coat"))
    A(G.rect("coil-guard", "unit", "detail", 2.3, -0.13, 0.42, 0.22, 0.86, tone=0.25, region="coil-guard", material="aluminium", hatch="fins" if den >= 100 else None))
    A(G.poly("fan-guard", "unit", "body", 2.2, [(HW - 0.01, 0.45), (FX, 0.5), (FX, 1.1), (HW - 0.01, 1.15)], tone=0.3, region="fan-guard", material="galvanised-steel"))
    A(G.rect("base-tray", "unit", "body", 2.1, -HW, 0.08, 2 * HW, 0.07, tone=0.3, region="base-tray", material="galvanised-steel"))
    A(G.poly("lid", "lid", "body", 2.5, lw("lid", [(0, -th), (0.47, -th), (0.47, 0.0), (0, 0.0)]), tone=0.35, region="top-lid", material="powder-coat"))
    A(G.poly("vane", "vane", "body", 2.6, lw("vane", [(-0.02, -th / 2 - 0.005), (0.22, -th / 2 - 0.005), (0.22, th / 2 + 0.005), (-0.02, th / 2 + 0.005)]), tone=0.3, region="discharge-vane", material="powder-coat"))
    if den >= 100:
        A(G.rect("service-panel-line", "unit", "detail", 2.4, 0.0, 0.45, 0.12, 0.8, tone=0.5, filled=False, region="casing", material="powder-coat"))
    if den == 50:
        A(G.rect("panel-handle", "unit", "detail", 2.5, 0.1, 0.8, 0.04, 0.1, tone=0.2, region="casing", material="powder-coat"))
    return s
