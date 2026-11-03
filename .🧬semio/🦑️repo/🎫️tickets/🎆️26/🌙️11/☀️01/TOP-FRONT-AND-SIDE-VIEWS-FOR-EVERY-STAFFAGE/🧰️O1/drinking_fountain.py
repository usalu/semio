import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "drinking-fountain"
PED = dict(region="pedestal-cast", material="cast-iron")
BAS = dict(region="basin-steel", material="stainless-basin")
BUB = dict(region="bubbler-brass", material="chrome-brass")
JET = dict(region="water-jet")
BASIN_Y = 0.74


def turned(id, scale, bone, sid, dy, z, tone, mat, hatch=None, **extra):
    s = orig_shape(A, scale, "elevation", sid)
    d = shift_path(s["d"], 0, dy)
    kw = dict(mat)
    if hatch: kw["hatch"] = hatch
    return pth(id, bone, d, z=z, tone=tone, **kw)


def hline(id, bone, y, r, z=3.5, tone=0.3, reg=PED):
    return poly(id, bone, [(-r, y), (r, y)], role="detail", z=z, tone=tone, closed=False, **reg)


views = {}

views[500] = (
    [turned("pedestal", 500, "pedestal", "pedestal", 0, 3, 0.6, PED),
     turned("basin", 500, "basin", "basin", BASIN_Y, 3.2, 0.5, BAS),
     rect("bubbler-column", "bubbler", -0.03, 0.855, 0.06, 0.145, z=3.3, tone=0.7, **BUB),
     poly("jet", "jet", [(0, 0.989), (0, 0.87)], role="body", z=3.5, tone=0.5, closed=False, penMm=0.13, **JET)],
    [ell("basin", "basin", 0, 0, 0.2, 0.2, z=3.2, tone=0.5, **BAS),
     cap("bubbler", "bubbler", -0.14, 0, -0.07, 0, 0.03, 0.03, z=3.3, tone=0.7, **BUB)],
    {"silhouette": ("Front: the turned pedestal and basin profile as in the side view with the bubbler column in front of the bowl and the jet as a vertical stroke; plan: the basin disc with the bubbler head.",
                    "Vorderansicht: das gedrehte Sockel- und Beckenprofil wie in der Seitenansicht mit der Spendersäule vor der Schale und dem Strahl als senkrechtem Strich; Draufsicht: die Beckenscheibe mit dem Spenderkopf."),
     "omitted": ("The pedestal is covered by the basin in plan, the button and the jet are not drawn in plan (the jet stroke would show in every pose).",
                 "Der Sockel ist in der Draufsicht vom Becken verdeckt, Taster und Strahl werden in der Draufsicht nicht gezeichnet (der Strahlstrich würde in jeder Pose erscheinen).")})

views[200] = (
    [turned("pedestal", 200, "pedestal", "pedestal", 0, 3, 0.6, PED),
     turned("basin", 200, "basin", "basin", BASIN_Y, 3.2, 0.5, BAS),
     rect("bubbler-column", "bubbler", -0.03, 0.855, 0.06, 0.145, z=3.3, tone=0.7, **BUB),
     hline("collar", "pedestal", 0.55, 0.115),
     poly("jet", "jet", [(0, 0.989), (0, 0.87)], role="body", z=3.5, tone=0.5, closed=False, penMm=0.13, **JET)],
    [ell("basin", "basin", 0, 0, 0.2, 0.2, z=3.2, tone=0.5, **BAS),
     cap("bubbler", "bubbler", -0.14, 0, -0.07, 0, 0.03, 0.03, z=3.3, tone=0.7, **BUB)],
    {"silhouette": ("Front: turned pedestal with its collar, the basin, the bubbler column and the jet; plan: basin disc with the bubbler head capsule.",
                    "Vorderansicht: gedrehter Sockel mit Kragen, Becken, Spendersäule und Strahl; Draufsicht: Beckenscheibe mit der Kapsel des Spenderkopfs."),
     "omitted": ("Button and jet are not drawn in plan.", "Taster und Strahl werden in der Draufsicht nicht gezeichnet.")})

views[100] = (
    [turned("pedestal", 100, "pedestal", "pedestal", 0, 3, 0.6, PED),
     turned("basin", 100, "basin", "basin", BASIN_Y, 3.2, 0.5, BAS),
     hline("rim", "basin", 0.84, 0.19, reg=BAS),
     rect("bubbler-column", "bubbler", -0.027, 0.855, 0.054, 0.145, z=3.3, tone=0.7, **BUB),
     ell("nozzle", "bubbler", 0, 0.958, 0.02, 0.02, z=3.45, tone=0.3, filled=False, **BUB),
     ell("button", "button", 0, 0.66, 0.02, 0.025, z=3.4, tone=0.8, **BUB),
     hline("collar-0", "pedestal", 0.3, 0.12), hline("collar-1", "pedestal", 0.55, 0.12),
     poly("jet", "jet", [(0, 0.989), (0, 0.87)], role="body", z=3.5, tone=0.5, closed=False, penMm=0.18, **JET)],
    [ell("basin", "basin", 0, 0, 0.2, 0.2, z=3.2, tone=0.5, **BAS),
     ell("rim", "basin", 0, 0, 0.19, 0.19, z=3.5, tone=0.3, role="detail", filled=False, **BAS),
     cap("bubbler", "bubbler", -0.14, 0, -0.07, 0, 0.03, 0.025, z=3.3, tone=0.7, **BUB),
     ell("nozzle", "bubbler", -0.05, 0, 0.016, 0.016, z=3.45, tone=0.3, filled=False, **BUB)],
    {"silhouette": ("Front: turned pedestal and basin with the rim line, the bubbler column with its nozzle ring, the push button on the front of the pedestal and the jet; plan: basin disc with rim ring, the bubbler head and its nozzle ring.",
                    "Vorderansicht: gedrehter Sockel und Becken mit Randlinie, Spendersäule mit Düsenring, Taster an der Sockelvorderseite und Strahl; Draufsicht: Beckenscheibe mit Randring, der Spenderkopf und sein Düsenring."),
     "omitted": ("The pushed button sits below the basin and the jet is not drawn in plan.", "Der gedrückte Taster liegt unter dem Becken und der Strahl wird in der Draufsicht nicht gezeichnet.")})

views[50] = (
    [turned("pedestal", 50, "pedestal", "pedestal", 0, 3, 0.6, PED, hatch="flutes"),
     hline("collar-0", "pedestal", 0.2, 0.132258), hline("collar-1", "pedestal", 0.42, 0.118065), hline("collar-2", "pedestal", 0.62, 0.105161),
     ell("bolt-0", "pedestal", -0.1792, 0.02, 0.008, 0.008, z=3.6, tone=0.2, **PED), ell("bolt-1", "pedestal", 0.1792, 0.02, 0.008, 0.008, z=3.6, tone=0.2, **PED),
     turned("basin", 50, "basin", "basin", BASIN_Y, 3.2, 0.5, BAS),
     hline("rim", "basin", 0.84, 0.19, reg=BAS),
     pth("rim-bead", "basin", shift_path("M -0.205 0.125 C -0.2 0.132 0.2 0.132 0.205 0.125", 0, BASIN_Y), z=3.6, tone=0.5, filled=False, **BAS),
     rect("bubbler-column", "bubbler", -0.027, 0.855, 0.054, 0.145, z=3.3, tone=0.7, **BUB),
     hline("bubbler-collar", "bubbler", 0.9, 0.027, z=3.5, tone=0.3, reg=BUB),
     ell("nozzle-ring", "bubbler", 0, 0.958, 0.02, 0.02, z=3.45, tone=0.3, filled=False, **BUB),
     ell("nozzle-bore", "bubbler", 0, 0.958, 0.008, 0.008, z=3.5, tone=0.2, **BUB),
     ell("button", "button", 0, 0.66, 0.02, 0.025, z=3.4, tone=0.8, **BUB),
     ell("button-ring", "button", 0, 0.66, 0.027, 0.032, z=3.35, tone=0.3, filled=False, **BUB),
     poly("jet", "jet", [(0, 0.989), (0, 0.87)], role="body", z=3.5, tone=0.5, closed=False, penMm=0.25, **JET),
     ell("drop-0", "jet", 0, 0.915, 0.007, 0.007, z=2.7, tone=0.3, **JET)],
    [ell("basin", "basin", 0, 0, 0.2, 0.2, z=3.2, tone=0.5, **BAS),
     ell("rim", "basin", 0, 0, 0.19, 0.19, z=3.5, tone=0.3, role="detail", filled=False, **BAS),
     ell("rim-bead", "basin", 0, 0, 0.2, 0.2, z=3.6, tone=0.5, role="detail", filled=False, **BAS),
     ell("drain", "basin", 0.04, 0, 0.022, 0.022, z=3.4, tone=0.2, role="detail", **BAS),
     cap("bubbler", "bubbler", -0.14, 0, -0.07, 0, 0.03, 0.025, z=3.3, tone=0.7, **BUB),
     ell("bubbler-collar", "bubbler", -0.14, 0, 0.018, 0.018, z=3.5, tone=0.3, role="detail", filled=False, **BUB),
     ell("nozzle-ring", "bubbler", -0.05, 0, 0.016, 0.016, z=3.45, tone=0.3, filled=False, **BUB),
     ell("nozzle-bore", "bubbler", -0.05, 0, 0.007, 0.007, z=3.5, tone=0.2, **BUB)],
    {"silhouette": ("Front: fluted pedestal with three collars and base bolts, the basin with rim line and bead, the bubbler column with collar and nozzle ring and bore, the push button with its escutcheon ring and the jet with a drop; plan: the basin with rim ring, bead, drain and the bubbler with collar and nozzle.",
                    "Vorderansicht: kannelierter Sockel mit drei Kragen und Fußschrauben, das Becken mit Randlinie und Wulst, die Spendersäule mit Kragen sowie Düsenring und Bohrung, der Taster mit Blendenring und der Strahl mit einem Tropfen; Draufsicht: das Becken mit Randring, Wulst, Abfluss und der Spender mit Kragen und Düse."),
     "omitted": ("The pedestal lies under the basin and the jet is not drawn in plan, because it would show in every pose.",
                 "Der Sockel liegt unter dem Becken und der Strahl wird in der Draufsicht nicht gezeichnet, weil er in jeder Pose erschiene.")})

apply(A, views)
