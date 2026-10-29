import sys, math; sys.path.insert(0, ".")
from lib import *
doc = load("objects"); a = asset(doc, "market-parasol")
CF, FA, BC = ("canopy-fabric", "polyester-canvas"), ("frame-alu", "aluminium-anodised"), ("base-concrete", "concrete")
d2r = math.pi / 180
LT, LS, RP = 1.26, 0.55, 0.50          # true rib length, stretcher length, stretcher attachment on rib
PHI0, PHIF = -15.0, -85.0
PLAN = [11.25 + 45 * k for k in range(8)]
NAMES = ["rib-right", "rib-b", "rib-c", "rib-d", "rib-left", "rib-f", "rib-g", "rib-h"]
def cth(k): return math.cos(PLAN[k] * d2r)
def proj(k, phi): return (LT * math.cos(phi * d2r) * cth(k), LT * math.sin(phi * d2r))
def alpha(k, phi):
    x, y = proj(k, phi); return math.degrees(math.atan2(y, x))
def hrun(phi):
    c, s = math.cos(phi * d2r), math.sin(phi * d2r)
    return RP * s * -1 + math.sqrt(LS ** 2 - (RP * c) ** 2) if False else (-RP * s) + math.sqrt(LS ** 2 - (RP * c) ** 2) * -1 * -1
def hrun(phi):
    c, s = math.cos(phi * d2r), math.sin(phi * d2r)
    return -RP * s + math.sqrt(LS ** 2 - (RP * c) ** 2)
def stretch_angle(side, phi):
    c = cth(0) if side == "right" else cth(4)
    px, py = RP * math.cos(phi * d2r) * c, RP * math.sin(phi * d2r)
    return math.degrees(math.atan2(py + hrun(phi), px))
H0 = hrun(PHI0)
def pose_state(phi):
    st = {"runner": {"dx": round(H0 - hrun(phi), 5)}}
    for k, n in enumerate(NAMES): st[n] = {"angleDeg": round(alpha(k, phi) - alpha(k, PHI0), 4)}
    for side in ("right", "left"): st[f"stretch-{side}"] = {"angleDeg": round(stretch_angle(side, phi) - stretch_angle(side, PHI0), 4)}
    return st

bones = [b for b in a["rig"]["bones"] if b["id"] in ("base", "mast")]
B = {b["id"]: b for b in bones}
B["mast"].update(lengthM=2.15)
bones.append({"id": "crown", "parent": "mast", "title": T("Crown hub", "Schirmkrone"), "x": 2.15, "y": 0, "angleDeg": -90, "lengthM": 0.05, "joint": {"type": "fixed"}})
bones.append({"id": "runner", "parent": "mast", "title": T("Sliding runner", "Gleitring (Läufer)"), "x": round(2.15 - H0, 5), "y": 0, "angleDeg": -90, "lengthM": 0.05, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.74, "maxM": 0.1}, "mass": {"kg": 0.4, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
rib_titles = {"rib-right": ("Rib at 11 deg (near right)", "Speiche bei 11 Grad (rechts)"), "rib-left": ("Rib at 191 deg (left)", "Speiche bei 191 Grad (links)")}
for k, n in enumerate(NAMES):
    L = math.hypot(*proj(k, PHI0))
    en, de = rib_titles.get(n, (f"Rib at {PLAN[k]:.0f} deg", f"Speiche bei {PLAN[k]:.0f} Grad"))
    bones.append({"id": n, "parent": "crown", "title": T(en, de), "x": 0, "y": 0, "angleDeg": round(alpha(k, PHI0), 4), "lengthM": round(L, 4), "joint": {"type": "revolute", "minDeg": -100, "maxDeg": 100}, "mass": {"kg": 0.6875, "comX": round(L * 0.5, 4), "comY": 0, "distribution": "uniform", "source": "authored-estimate"}})
for side, en, de in (("right", "right", "rechts"), ("left", "left", "links")):
    ang = stretch_angle(side, PHI0)
    bones.append({"id": f"stretch-{side}", "parent": "runner", "title": T(f"Stretcher {en}", f"Spreize {de}"), "x": 0, "y": 0, "angleDeg": round(ang, 4), "lengthM": LS, "joint": {"type": "revolute", "minDeg": -100, "maxDeg": 100}})
a["rig"]["bones"] = bones
for an in a["anchors"]:
    if an["id"] == "canopy-centre": an.update(bone="crown", x=0, y=-0.1)

g = G()
g.add("base-block", "base", "body", 0, 1, rect(-0.2, 0, 0.4, 0.2, 0.01), *BC, tone=0.62)
g.add("base-seam", "base", "detail", 1, 1.1, poly([(-0.2, 0.15), (0.2, 0.15)], False), *BC, filled=False, tone=0.35)
g.add("base-plate", "base", "body", 1, 1.2, rect(-0.14, 0.2, 0.28, 0.015, 0.002), *FA, tone=0.35)
g.add("base-socket", "base", "body", 1, 1.3, path("M -0.045 0.215 L -0.035 0.30 L 0.035 0.30 L 0.045 0.215 Z"), *FA, tone=0.3)
for k, x in enumerate((-0.11, 0.11)):
    g.add(f"base-bolt-{k}", "base", "detail", 3, 1.4, ell(x, 0.2075, 0.008, 0.0075), *FA, tone=0.15)
g.add("base-grit", "base", "texture", 2, 1.2, path("M -0.15 0.06 L -0.12 0.07 M 0.05 0.04 L 0.09 0.05 M -0.04 0.11 L 0.0 0.1"), *BC, filled=False, tone=0.3)
g.add("mast", "mast", "body", 0, 3, cap(0.0, 0, 2.15, 0, 0.026, 0.021), *FA, tone=0.45)
g.add("mast-knuckle", "mast", "detail", 1, 3.2, ell(0.0, 0, 0.04, 0.04), *FA, tone=0.3)
g.add("mast-tilt-collar", "mast", "detail", 1, 3.3, rect(0.82, -0.034, 0.11, 0.068, 0.008), *FA, tone=0.25)
g.add("mast-crank", "mast", "detail", 2, 3.4, path("M 1.18 0.021 L 1.18 0.075 L 1.14 0.095"), *FA, filled=False, tone=0.3)
g.add("mast-crank-knob", "mast", "detail", 3, 3.5, ell(1.135, 0.097, 0.012, 0.012), *FA, tone=0.2)
g.add("mast-seam", "mast", "texture", 2, 3.1, poly([(0.4, 0.012), (0.78, 0.012)], False), *FA, filled=False, tone=0.3)
g.add("crown-hub", "crown", "body", 0, 3.5, rect(-0.045, -0.06, 0.09, 0.14, 0.02), *FA, tone=0.35)
g.add("crown-finial", "crown", "body", 1, 3.6, path("M -0.012 0.08 L -0.012 0.1 C -0.01 0.115 0.01 0.115 0.012 0.1 L 0.012 0.08 Z"), *FA, tone=0.3)
g.add("runner-collar", "runner", "body", 0, 3.5, rect(-0.035, -0.04, 0.07, 0.08, 0.02), *FA, tone=0.35)
g.add("runner-detent", "runner", "detail", 2, 3.6, poly([(-0.035, 0.0), (0.035, 0.0)], False), *FA, filled=False, tone=0.3)
for side in ("right", "left"):
    g.add(f"stretch-{side}", f"stretch-{side}", "body", 1, 2.4, cap(0, 0, LS, 0, 0.006, 0.006), *FA, tone=0.4)
    g.add(f"stretch-{side}-eye", f"stretch-{side}", "detail", 3, 2.5, ell(LS, 0, 0.008, 0.008), *FA, tone=0.2)
for k, n in enumerate(NAMES):
    L = math.hypot(*proj(k, PHI0)); main = n in ("rib-right", "rib-left")
    g.add(f"{n}-rod", n, "body", 0 if main else 1, 2.5, cap(0, 0, L, 0, 0.011, 0.007), *FA, tone=0.35)
    g.add(f"{n}-tip", n, "detail", 2, 2.6, ell(L, 0, 0.013, 0.013), *FA, tone=0.2)
    s = 1 if cth(k) >= 0 else -1
    # hem scallop ears either side of the tip, bitten upward between ribs (in rib-local frame: +y is above the rib for right-going, below for left-going)
    th = alpha(k, PHI0) * d2r; ex = 0.10
    hx, hy = (math.cos(0), 0)
    for sg, tag in ((1, "a"), (-1, "b")):
        wx, wy = L * math.cos(th) + sg * ex, L * math.sin(th)
        cx, cy = L * math.cos(th) + sg * ex * 0.5, L * math.sin(th) + 0.055
        sx, sy = L * math.cos(th), L * math.sin(th)
        def loc(px, py):
            c, s_ = math.cos(-th), math.sin(-th); return (px * c - py * s_, px * s_ + py * c)
        p0, p1, p2 = loc(sx, sy), loc(cx, cy), loc(wx, wy)
        g.add(f"{n}-scallop-{tag}", n, "detail", 2, 2.7, path(f"M {p0[0]:.4f} {p0[1]:.4f} Q {p1[0]:.4f} {p1[1]:.4f} {p2[0]:.4f} {p2[1]:.4f}"), *CF, filled=False, tone=0.3)
# fabric crescents on the two main ribs (top profile of the dome), seam lines and folds
for n, k in (("rib-right", 0), ("rib-left", 4)):
    L = math.hypot(*proj(k, PHI0)); b = 0.115
    g.add(f"panel-{n[4:]}", n, "body", 0, 2, path(f"M 0 0 C {0.28*L:.4f} {b:.4f} {0.72*L:.4f} {b*0.9:.4f} {L:.4f} 0 Z" if k == 0 else f"M 0 0 C {0.28*L:.4f} {-b:.4f} {0.72*L:.4f} {-b*0.9:.4f} {L:.4f} 0 Z"), *CF, tone=0.82)
    sg = 1 if k == 0 else -1
    g.add(f"panel-{n[4:]}-seam", n, "detail", 1, 2.1, path(f"M {0.3*L:.4f} {sg*0.05:.4f} C {0.5*L:.4f} {sg*0.085:.4f} {0.75*L:.4f} {sg*0.07:.4f} {0.92*L:.4f} {sg*0.02:.4f}"), *CF, filled=False, tone=0.3)
    g.add(f"panel-{n[4:]}-fold", n, "texture", 2, 2.2, path(f"M {0.15*L:.4f} {sg*0.03:.4f} C {0.3*L:.4f} {sg*0.06:.4f} {0.45*L:.4f} {sg*0.075:.4f} {0.6*L:.4f} {sg*0.07:.4f}"), *CF, filled=False, tone=0.3)
a["geometry"]["elevation"] = g.items

for p in a["poses"]:
    st = {"open-still": {}, "tilted-to-sun": {}, "furled": None}[p["id"]]
    bo = {}
    if p["id"] == "furled": bo = pose_state(PHIF)
    if p["id"] == "tilted-to-sun": bo = {"mast": {"angleDeg": -8}}
    p["bones"] = bo
    for l in p["loads"]:
        l.update(bone="crown", x=0, y=-0.1)
flap = pose_state(PHI0 + 4)
ck = a["clips"][0]["keys"]
ck[2]["bones"] = flap
for p in a["poses"]:
    if p["id"] == "furled":
        p["description"] = T("Ribs folded down along the mast; the runner has slid down by 0.65 m and the fabric hangs bundled along the ribs.", "Speichen entlang des Masts nach unten geklappt; der Läufer ist 0,65 m abgerutscht und der Stoff hängt gebündelt an den Speichen.")
a["structure"]["assumptions"][0] = T("Base is a fixed socket; the canopy wind loads act at the crown hub as a quasi-static design state.", "Der Fuß ist eine feste Hülse; die Windlasten am Schirm wirken an der Krone als quasistatischer Bemessungszustand.")
a["summary"] = T("A tilting market parasol with a concrete base, an aluminium mast, an eight-rib canopy (11 degrees plan offset so all eight ribs show in elevation), fabric scallops and a sliding runner whose stretchers fold the ribs down along the mast.", "Ein neigbarer Marktschirm mit Betonfuß, Aluminiummast, achtspeichigem Schirm (11 Grad Planversatz, damit alle acht Speichen im Aufriss sichtbar sind), Stoffbögen und einem Läufer, dessen Spreizen die Speichen am Mast nach unten klappen.")
a["structure"]["assumptions"].append(T("Ribs other than the two main ribs are drawn as their foreshortened elevation projections with a rigid length; they are exact for the open canopy and approximate (shorter than true) when folded.", "Die übrigen Speichen sind als verkürzte Aufrissprojektionen mit starrer Länge gezeichnet; sie sind für den offenen Schirm exakt und im gefalteten Zustand näherungsweise (kürzer als wahr)."))
save("objects", doc)
print(H0, hrun(PHIF), pose_state(PHIF))
