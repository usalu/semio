import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

H = dict(region="hoop-galv", material="galv-steel")
C = dict(region="crossbar-galv", material="galv-steel")
R = 0.02415
CX = 0.35085

def strip(id, r, top=0.8, z=3, tone=0.45, role="body"):
    c = top - r
    k = 0.5523 * r
    d = f"M {-r} 0 L {-r} {c:.5f} C {-r} {c + k:.5f} {-k:.5f} {c + r:.5f} 0 {c + r:.5f} C {k:.5f} {c + r:.5f} {r} {c + k:.5f} {r} {c:.5f} L {r} 0 Z"
    return pth(id, "hoop", d, z=z, tone=tone, **H)

def hline(id, bone, y, r, z=4, reg=H, tone=0.3):
    return poly(id, bone, [(-r, y), (r, y)], role="detail", z=z, tone=tone, closed=False, **reg)

def vline(id, bone, x, r, z=4, reg=H, tone=0.3):
    return poly(id, bone, [(x, -r), (x, r)], role="detail", z=z, tone=tone, closed=False, **reg)

views = {}

r5s = 0.034
views[500] = (
    [strip("hoop-end-on", r5s, tone=0.5)],
    [cap("hoop-top", "hoop", -0.34, 0, 0.34, 0, r5s, r5s, tone=0.5, **H)],
    {"silhouette": ("Front and plan are one bar of 0.068 m, the hoop seen end-on and from above, widened by 0.02 m from the true 0.048 m tube so it stays printable and inside the dimension tolerance.",
                    "Vorder- und Draufsicht sind je ein Balken von 0,068 m, der Bügel von vorn und von oben, gegenüber dem wahren Rohr von 0,048 m um 0,02 m verbreitert, damit er druckbar bleibt und in der Maßtoleranz liegt."),
     "omitted": ("Crossbar, welds and leg rings are dropped in front and plan; the crossbar hides behind the near leg and below the top tube.",
                 "Querholm, Schweißnähte und Beinringe entfallen in Vorder- und Draufsicht; der Querholm verschwindet hinter dem nahen Bein und unter dem oberen Rohr.")})

views[200] = (
    [strip("hoop-end-on", R), rect("crossbar-end-on", "crossbar", -R, 0.4 - R, 2 * R, 2 * R, z=3.2, tone=0.45, **C)],
    [cap("hoop-top", "hoop", -CX, 0, CX, 0, R, R, tone=0.45, **H),
     vline("bend-tangent-top-0", "hoop", -0.1708, R), vline("bend-tangent-top-1", "hoop", 0.1708, R)],
    {"silhouette": ("Front: the hoop tube end-on at its true 48.3 mm with the cross rail as a square joint on it; plan: one capsule of true tube width with the two bend tangents.",
                    "Vorderansicht: das Bügelrohr von vorn in wahrer Stärke von 48,3 mm mit dem Querholm als quadratischem Stoß; Draufsicht: eine Kapsel in wahrer Rohrstärke mit den beiden Biegetangenten."),
     "omitted": ("Leg rings and ground collars in front and plan.", "Beinringe und Bodenmanschetten in Vorder- und Draufsicht.")})

views[100] = (
    [strip("hoop-end-on", R), rect("crossbar-end-on", "crossbar", -R, 0.4 - R, 2 * R, 2 * R, z=3.2, tone=0.45, **C),
     hline("bend-tangent-low", "hoop", 0.59585, R), hline("ground-collar", "hoop", 0.004, R)],
    [cap("hoop-top", "hoop", -CX, 0, CX, 0, R, R, tone=0.45, **H),
     vline("bend-tangent-top-0", "hoop", -0.1708, R), vline("bend-tangent-top-1", "hoop", 0.1708, R),
     ell("leg-dot-0", "hoop", -CX, 0, 0.011, 0.011, z=4, tone=0.3, **H), ell("leg-dot-1", "hoop", CX, 0, 0.011, 0.011, z=4, tone=0.3, **H)],
    {"silhouette": ("Front: the end-on tube with cross rail joint, bend tangent and ground collar; plan: the bar with the two bend tangents and the two leg dots.",
                    "Vorderansicht: das Rohr von vorn mit Querholmstoß, Biegetangente und Bodenmanschette; Draufsicht: der Balken mit den beiden Biegetangenten und den zwei Beinpunkten."),
     "retained": ("The leg dots mark the two legs set into the footing.", "Die Beinpunkte markieren die beiden einbetonierten Beine.")})

views[50] = (
    [strip("hoop-end-on", R), rect("crossbar-end-on", "crossbar", -R, 0.4 - R, 2 * R, 2 * R, z=3.2, tone=0.45, **C),
     hline("bend-tangent-low", "hoop", 0.59585, R), hline("bend-tangent-top", "hoop", 0.7517, R), hline("ground-collar", "hoop", 0.004, R),
     hline("weld-collar-low", "crossbar", 0.4 - R + 0.005, R, reg=C), hline("weld-collar-high", "crossbar", 0.4 + R - 0.005, R, reg=C),
     ell("crossbar-bore", "crossbar", 0, 0.4, 0.0138, 0.0138, z=4.1, tone=0.3, filled=False, **C)],
    [cap("hoop-top", "hoop", -CX, 0, CX, 0, R, R, tone=0.45, **H),
     vline("bend-tangent-top-0", "hoop", -0.1708, R), vline("bend-tangent-top-1", "hoop", 0.1708, R),
     vline("bend-tangent-low-0", "hoop", -0.32665, R), vline("bend-tangent-low-1", "hoop", 0.32665, R),
     ell("leg-ring-0", "hoop", -CX, 0, 0.0138, 0.0138, z=4, tone=0.3, filled=False, **H), ell("leg-ring-1", "hoop", CX, 0, 0.0138, 0.0138, z=4, tone=0.3, filled=False, **H),
     ell("leg-dot-0", "hoop", -CX, 0, 0.006, 0.006, z=4.1, tone=0.2, **H), ell("leg-dot-1", "hoop", CX, 0, 0.006, 0.006, z=4.1, tone=0.2, **H)],
    {"silhouette": ("Front: the end-on tube with bend tangents, ground collar and the cross rail with both weld collars and its open bore; plan: the capsule bar with all four bend tangents and the two leg rings with their centre dots.",
                    "Vorderansicht: das Rohr von vorn mit Biegetangenten, Bodenmanschette und dem Querholm mit beiden Schweißmanschetten und seiner offenen Bohrung; Draufsicht: der Kapselbalken mit allen vier Biegetangenten und den zwei Beinringen mit ihren Mittelpunkten."),
     "retained": ("True 48.3 mm tube width in both views.", "Wahre Rohrstärke von 48,3 mm in beiden Ansichten.")})

apply("bicycle-stand", views)
