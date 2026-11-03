import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "patio-heater"
BASE = dict(region="base-housing", material="powder-steel")
BASE_R = dict(region="base-housing")
POLE = dict(region="pole-stainless", material="stainless-brushed")
POLE_R = dict(region="pole-stainless")
HEAD = dict(region="emitter-head", material="stainless-hot")
HEAD_R = dict(region="emitter-head")
REF = dict(region="reflector", material="alu-reflector")
REF_R = dict(region="reflector")
KNOB = dict(region="control", material="plastic-black")
POLE_Y = 0.62
HEAD_Y = 1.98
REF_Y = 2.06


def base(scale):
    s = orig_shape(A, scale, "elevation", "base")
    return pth("base", "base", s["d"], z=3, tone=0.6, **BASE)


def reflector(scale):
    s = orig_shape(A, scale, "elevation", "reflector")
    return pth("reflector", "reflector", shift_path(s["d"], 0, REF_Y), z=3.2, tone=0.5, **REF)


def hl(id, bone, y, r, reg, z=3.4, tone=0.3):
    return poly(id, bone, [(-r, y), (r, y)], role="detail", z=z, tone=tone, closed=False, **reg)


def vl(id, bone, x, y0, y1, reg, z=3.3, tone=0.3):
    return poly(id, bone, [(x, y0), (x, y1)], role="detail", z=z, tone=tone, closed=False, **reg)


def radial(id, bone, deg, r0, r1, z=3.45, tone=0.4):
    a = math.radians(deg)
    return poly(id, bone, [(r0 * math.cos(a), r0 * math.sin(a)), (r1 * math.cos(a), r1 * math.sin(a))], role="detail", z=z, tone=tone, closed=False, **REF_R)


views = {}

views[500] = (
    [base(500), rect("pole", "pole", -0.07, POLE_Y, 0.14, 1.36, z=3, tone=0.45, **POLE),
     rect("head", "burner-head", -0.13, HEAD_Y - 0.02, 0.26, 0.14, z=2.9, tone=0.8, **HEAD),
     reflector(500)],
    [ell("reflector", "reflector", 0, 0, 0.4, 0.4, z=3.2, tone=0.5, **REF)],
    {"silhouette": ("Front: the same turned profile as the side view (base, pole, burner head, reflector dome), all of it round; plan: the reflector disc of 0.80 m that hides everything below it.",
                    "Vorderansicht: dasselbe gedrehte Profil wie in der Seitenansicht (Sockel, Mast, Brennerkopf, Reflektorkuppel), alles rund; Draufsicht: die Reflektorscheibe von 0,80 m, die alles darunter verdeckt."),
     "omitted": ("The control knob is merged into the base in front; base, pole and burner head are hidden under the reflector in plan.",
                 "Der Drehknopf geht in der Vorderansicht im Sockel auf; Sockel, Mast und Brennerkopf sind in der Draufsicht unter dem Reflektor verdeckt.")})

views[200] = (
    [base(200), rect("pole", "pole", -0.035, POLE_Y, 0.07, 1.36, z=3, tone=0.45, **POLE),
     rect("head", "burner-head", -0.13, HEAD_Y, 0.26, 0.12, z=2.9, tone=0.8, **HEAD),
     reflector(200), ell("knob", "knob", 0, 0.46, 0.03, 0.03, z=3.5, tone=0.2, **KNOB)],
    [ell("reflector", "reflector", 0, 0, 0.4, 0.4, z=3.2, tone=0.5, **REF),
     ell("dome-ring", "reflector", 0, 0, 0.22, 0.22, z=3.4, tone=0.3, role="detail", filled=False, **REF_R)],
    {"silhouette": ("Front: turned base, pole, burner head and reflector with the round control knob on the base; plan: the reflector disc with the dome contour ring.",
                    "Vorderansicht: gedrehter Sockel, Mast, Brennerkopf und Reflektor mit dem runden Drehknopf am Sockel; Draufsicht: die Reflektorscheibe mit dem Kuppelkonturring."),
     "omitted": ("The knob pointer is not drawn in front (the knob turns about the lateral axis); door seam and base are hidden or on the side.",
                 "Der Knopfzeiger wird in der Vorderansicht nicht gezeichnet (der Knopf dreht um die Querachse); Türfuge und Sockel liegen an der Seite oder sind verdeckt.")})

views[100] = (
    [base(100), rect("pole", "pole", -0.03, POLE_Y, 0.06, 1.36, z=3, tone=0.45, **POLE),
     hl("pole-joint", "pole", POLE_Y + 0.68, 0.03, POLE_R),
     rect("head", "burner-head", -0.13, HEAD_Y, 0.26, 0.12, z=2.9, tone=0.8, r=0.02, **HEAD),
     vl("guard-rod-0", "burner-head", -0.08, HEAD_Y + 0.005, HEAD_Y + 0.075, HEAD_R), vl("guard-rod-1", "burner-head", 0.0, HEAD_Y + 0.005, HEAD_Y + 0.075, HEAD_R), vl("guard-rod-2", "burner-head", 0.08, HEAD_Y + 0.005, HEAD_Y + 0.075, HEAD_R),
     reflector(100), hl("dome-rim", "reflector", REF_Y + 0.02, 0.4, REF_R),
     hl("vent-slots", "base", 0.57, 0.08, BASE_R),
     ell("knob", "knob", 0, 0.46, 0.028, 0.028, z=3.5, tone=0.2, **KNOB)],
    [ell("reflector", "reflector", 0, 0, 0.4, 0.4, z=3.2, tone=0.5, **REF),
     ell("dome-ring", "reflector", 0, 0, 0.22, 0.22, z=3.4, tone=0.3, role="detail", filled=False, **REF_R),
     ell("top-cap", "reflector", 0, 0, 0.04, 0.04, z=3.5, tone=0.2, **REF_R)],
    {"silhouette": ("Front: the turned silhouette with pole joint, burner head and three guard rods, dome rim, vent slots and the control knob; plan: reflector disc with the dome contour ring and the top cap.",
                    "Vorderansicht: die gedrehte Silhouette mit Mastfuge, Brennerkopf und drei Schutzstäben, Kuppelrand, Lüftungsschlitzen und Drehknopf; Draufsicht: Reflektorscheibe mit Kuppelkonturring und Kappe."),
     "omitted": ("The knob pointer and the side door are not drawn in front.", "Knopfzeiger und seitliche Tür werden in der Vorderansicht nicht gezeichnet.")})

views[50] = (
    [base(50),
     rect("wheel-l", "base", 0.11, 0.01, 0.02, 0.08, z=2.8, tone=0.9, **dict(region="base-housing", material="plastic-black")),
     rect("wheel-r", "base", -0.13, 0.01, 0.02, 0.08, z=2.8, tone=0.9, **dict(region="base-housing", material="plastic-black")),
     hl("base-lip", "base", 0.045, 0.17, BASE_R),
     hl("vent-slot-0", "base", 0.55, 0.07, BASE_R), hl("vent-slot-1", "base", 0.58, 0.07, BASE_R), hl("vent-slot-2", "base", 0.61, 0.07, BASE_R),
     rect("pole", "pole", -0.03, POLE_Y, 0.06, 1.36, z=3, tone=0.45, **POLE),
     hl("pole-joint", "pole", POLE_Y + 0.68, 0.03, POLE_R),
     vl("pole-highlight", "pole", -0.012, POLE_Y + 0.08, POLE_Y + 1.28, POLE_R, z=3.5),
     rect("head", "burner-head", -0.13, HEAD_Y, 0.26, 0.12, z=2.9, tone=0.8, r=0.02, hatch="mesh", **HEAD)]
    + [vl(f"guard-rod-{i}", "burner-head", x, HEAD_Y + 0.01, HEAD_Y + 0.075, HEAD_R) for i, x in enumerate((-0.1, -0.05, 0.0, 0.05, 0.1))]
    + [reflector(50),
       pth("dome-bead", "reflector", shift_path("M -0.395 0.025 C -0.3 0.045 0.3 0.045 0.395 0.025", 0, REF_Y), z=3.4, tone=0.5, filled=False, **REF_R),
       ell("top-cap", "reflector", 0, 0.175, 0.03, 0.012, z=3.5, tone=0.2, **REF_R),
       ell("knob", "knob", 0, 0.46, 0.03, 0.03, z=3.5, tone=0.2, **KNOB),
       ell("knob-ring", "knob", 0, 0.46, 0.038, 0.038, z=3.45, tone=0.3, role="detail", filled=False, **KNOB)],
    [ell("reflector", "reflector", 0, 0, 0.4, 0.4, z=3.2, tone=0.5, **REF),
     ell("dome-bead", "reflector", 0, 0, 0.37, 0.37, z=3.4, tone=0.5, role="detail", filled=False, **REF_R),
     ell("dome-ring", "reflector", 0, 0, 0.22, 0.22, z=3.4, tone=0.3, role="detail", filled=False, **REF_R)]
    + [radial(f"seam-{i}", "reflector", 45 * i, 0.05, 0.4) for i in range(8)]
    + [ell("top-cap", "reflector", 0, 0, 0.05, 0.05, z=3.5, tone=0.2, **REF_R)],
    {"silhouette": ("Front: the full turned silhouette with wheels, lip, three vent slots, pole joint and highlight, the meshed burner head with five guard rods, the dome bead and top cap, and the knob with its ring; plan: the reflector disc with bead ring, contour ring, eight radial panel seams and the top cap.",
                    "Vorderansicht: die volle gedrehte Silhouette mit Rädern, Lippe, drei Lüftungsschlitzen, Mastfuge und Lichtkante, dem vergitterten Brennerkopf mit fünf Schutzstäben, Kuppelwulst und Kappe sowie dem Drehknopf mit Ring; Draufsicht: die Reflektorscheibe mit Wulstring, Konturring, acht radialen Blechnähten und der Kappe."),
     "omitted": ("Knob pointer and the side door are not drawn in front; everything under the reflector is hidden in plan.",
                 "Knopfzeiger und seitliche Tür werden in der Vorderansicht nicht gezeichnet; alles unter dem Reflektor ist in der Draufsicht verdeckt.")})

apply(A, views)
