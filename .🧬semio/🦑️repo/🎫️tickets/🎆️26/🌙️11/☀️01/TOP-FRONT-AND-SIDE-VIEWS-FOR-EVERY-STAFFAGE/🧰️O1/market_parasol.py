import sys, os, copy
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "market-parasol"
ALU = dict(region="frame-alu", material="aluminium-anodised")
ALU_R = dict(region="frame-alu")
CON = dict(region="base-concrete", material="concrete")
HEM = "M 0 0 C 0.3465 0.115 0.8909 0.1035 1.2374 0 L 1.3339 0.0264 Z"


def rev(sid, scale, plan=False, new=None, axis="crown", mirror=True, path=None, ring=False, **over):
    s = copy.deepcopy(orig_shape(A, scale, "elevation", sid))
    if path is not None:
        s["d"] = path
    s["id"] = new or sid
    s["solid"] = {"kind": "revolved", "axisBone": axis, "axisX": 0.0}
    if not plan:
        s["solid"]["mirror"] = mirror
    if ring:
        s["filled"] = False
        s["role"] = "detail"
    s.update(over)
    s.pop("hatch", None) if False else None
    return s


def vrect(id, bone, w, y0, y1, z=3, tone=0.45, follow=False, r=None, **reg):
    s = rect(id, bone, -w, y0, 2 * w, y1 - y0, z=z, tone=tone, followTip=follow, **reg)
    if r: s["r"] = r
    return s


def trap(id, bone, w0, y0, w1, y1, z=3, tone=0.45, **reg):
    return poly(id, bone, [(-w0, y0), (w0, y0), (w1, y1), (-w1, y1)], z=z, tone=tone, followTip=True, **reg)


def rib_plan(id, bone, sign, r0, r1, z=2.5, tone=0.35, **over):
    return cap(id, bone, 0, 0, sign * 1.1935, 0, r0, r1, z=z, tone=tone, follow=True, **ALU, **over)


views = {}

# ----------------------------------------------------------------- 1:500
views[500] = (
    [rect("base-block", "base", -0.1995, 0, 0.4, 0.2, z=1, tone=0.62, **CON),
     poly("mast", "mast", [(0, 0.2), (0, 2.4605)], role="body", z=3, tone=0.45, closed=False, followTip=True, **ALU),
     rev("panel-right", 500, new="canopy", z=2)],
    [rev("panel-right", 500, plan=True, new="canopy", z=2)],
    {"silhouette": ("Front and plan are solids of revolution about the mast axis: the front shows the canopy profile mirrored about the mast, the plan the canopy disc, both opening, tilting and furling with the rib.",
                    "Vorder- und Draufsicht sind Rotationskörper um die Mastachse: die Vorderansicht zeigt das gespiegelte Schirmprofil, die Draufsicht die Schirmscheibe, beide öffnen, neigen und schließen sich mit der Rippe."),
     "omitted": ("The concrete base block, the mast line in front and everything under the canopy in plan are drawn only as far as they show; ribs, hub and stretchers are merged into the canopy.",
                 "Betonfuß und Mastlinie erscheinen in der Vorderansicht, in der Draufsicht ist alles unter dem Schirm verdeckt; Rippen, Nabe und Spreizen gehen im Schirm auf.")})

# ----------------------------------------------------------------- 1:200
views[200] = (
    [rect("base-block", "base", -0.2, 0, 0.4, 0.2, z=1, tone=0.62, **CON),
     vrect("mast", "mast", 0.025, 0.2, 2.46, follow=True, z=3, tone=0.45, **ALU),
     rect("crown-hub", "crown", -0.07, 2.305, 0.14, 0.09, z=3.5, tone=0.35, **ALU),
     rev("panel-right", 200, new="canopy", z=2),
     rev("rib-right-rod", 200, new="rib-front", z=2.5, penMm=0.28),
     rev("rib-d-rod", 200, new="rib-oblique", z=2.5, penMm=0.28),
     rev("stretch-right", 200, axis="runner", new="stretcher", z=2.4)],
    [rev("panel-right", 200, plan=True, new="canopy", z=2),
     rib_plan("rib-right", "rib-right", 1, 0.011, 0.007), rib_plan("rib-left", "rib-left", -1, 0.011, 0.007),
     ell("crown-hub", "crown", 0, 0, 0.07, 0.07, z=3.5, tone=0.35, **ALU)],
    {"silhouette": ("Front: base block, mast, crown hub and the canopy as a solid of revolution with its front rib, an oblique rib and the stretcher, all following the opening and the tilt; plan: the canopy disc with the two in-plane ribs and the crown hub.",
                    "Vorderansicht: Fußblock, Mast, Kronennabe und der Schirm als Rotationskörper mit seiner Frontrippe, einer Schrägrippe und der Spreize, alle mit Öffnung und Neigung; Draufsicht: die Schirmscheibe mit den zwei Rippen in der Ebene und der Kronennabe."),
     "omitted": ("Mast, runner and stretchers are hidden under the canopy in plan; the other ribs are not drawn in plan because their plan angle cannot follow the folding.",
                 "Mast, Läufer und Spreizen sind in der Draufsicht unter dem Schirm verdeckt; die übrigen Rippen werden in der Draufsicht nicht gezeichnet, weil ihr Grundrisswinkel dem Zusammenfalten nicht folgen kann.")})

# ----------------------------------------------------------------- 1:100
views[100] = (
    [rect("base-block", "base", -0.2, 0, 0.4, 0.2, z=1, tone=0.62, r=0.01, **CON),
     pth("base-socket", "base", "M -0.045 0.215 L -0.035 0.3 L 0.035 0.3 L 0.045 0.215 Z", z=1.3, tone=0.3, **ALU),
     vrect("mast", "mast", 0.025, 0.2, 2.45, follow=True, z=3, tone=0.45, **ALU),
     rect("crown-hub", "crown", -0.07, 2.305, 0.14, 0.09, z=3.5, tone=0.35, **ALU),
     rect("runner-collar", "runner", -0.035, 1.917, 0.07, 0.08, z=3.5, tone=0.35, **ALU),
     rev("panel-right", 100, new="canopy", path=HEM, z=2),
     rev("rib-right-rod", 100, new="rib-front", z=2.5),
     rev("rib-b-rod", 100, new="rib-oblique-b", z=2.5),
     rev("rib-d-rod", 100, new="rib-oblique-d", z=2.5),
     rev("rib-right-scallop-a", 100, new="scallop-a", z=2.7, ring=False),
     rev("rib-right-scallop-b", 100, new="scallop-b", z=2.7, ring=False),
     rev("stretch-right", 100, axis="runner", new="stretcher", z=2.4)],
    [rev("panel-right", 100, plan=True, new="canopy", path=HEM, z=2),
     rev("rib-right-scallop-a", 100, plan=True, new="hem-ring", z=2.7, ring=True),
     rib_plan("rib-right", "rib-right", 1, 0.011, 0.007), rib_plan("rib-left", "rib-left", -1, 0.011, 0.007),
     ell("crown-hub", "crown", 0, 0, 0.07, 0.07, z=3.5, tone=0.35, **ALU)],
    {"silhouette": ("Front: base block with socket, mast, crown hub, runner collar and the canopy as a solid of revolution with scalloped hem, front rib, two oblique ribs and the stretcher; plan: canopy disc with hem ring, the two in-plane ribs and the crown hub.",
                    "Vorderansicht: Fußblock mit Hülse, Mast, Kronennabe, Läuferkragen und der Schirm als Rotationskörper mit Zackensaum, Frontrippe, zwei Schrägrippen und der Spreize; Draufsicht: Schirmscheibe mit Saumring, die zwei Rippen in der Ebene und die Kronennabe."),
     "retained": ("Canopy, ribs, stretcher and runner collar are bound to their own bones, so opening, furling and tilting read in front and plan.",
                  "Schirm, Rippen, Spreize und Läuferkragen sind an ihre eigenen Knochen gebunden, Öffnen, Schließen und Neigen sind in Vorder- und Draufsicht ablesbar.")})

# ----------------------------------------------------------------- 1:50
views[50] = (
    [rect("base-block", "base", -0.2, 0, 0.4, 0.2, z=1, tone=0.62, r=0.01, penMm=0.35, **CON),
     poly("base-seam", "base", [(-0.2, 0.15), (0.2, 0.15)], role="detail", z=1.1, tone=0.35, closed=False, **CON),
     rect("base-plate", "base", -0.14, 0.2, 0.28, 0.015, z=1.2, tone=0.35, r=0.002, **ALU),
     pth("base-socket", "base", "M -0.045 0.215 L -0.035 0.3 L 0.035 0.3 L 0.045 0.215 Z", z=1.3, tone=0.3, **ALU),
     trap("mast", "mast", 0.026, 0.2, 0.021, 2.35, z=3, tone=0.45, penMm=0.416, **ALU),
     ell("mast-knuckle", "mast", 0, 0.2, 0.04, 0.04, z=3.2, tone=0.3, **ALU),
     rect("mast-tilt-collar", "mast", -0.034, 1.02, 0.068, 0.11, z=3.3, tone=0.25, followTip=True, r=0.008, **ALU),
     ell("mast-crank-knob", "mast", 0, 1.335, 0.012, 0.012, z=3.5, tone=0.2, **ALU),
     rect("crown-hub", "crown", -0.045, 2.29, 0.09, 0.14, z=3.5, tone=0.35, r=0.02, penMm=0.35, **ALU),
     pth("crown-finial", "crown", "M -0.012 2.43 L -0.012 2.45 C -0.01 2.465 0.01 2.465 0.012 2.45 L 0.012 2.43 Z", z=3.6, tone=0.3, **ALU),
     rect("runner-collar", "runner", -0.035, 1.917, 0.07, 0.08, z=3.5, tone=0.35, r=0.02, **ALU),
     poly("runner-detent", "runner", [(-0.035, 1.957), (0.035, 1.957)], role="detail", z=3.6, tone=0.3, closed=False, **ALU),
     rev("panel-right", 50, new="canopy", path=HEM, z=2, penMm=0.5),
     rev("panel-right-seam", 50, new="canopy-seam", z=2.1),
     rev("panel-right-fold", 50, new="canopy-fold", z=2.2),
     rev("rib-right-rod", 50, new="rib-front", z=2.5),
     rev("rib-right-tip", 50, new="rib-front-tip", z=2.6),
     rev("rib-b-rod", 50, new="rib-oblique-b", z=2.5),
     rev("rib-b-tip", 50, new="rib-oblique-b-tip", z=2.6),
     rev("rib-d-rod", 50, new="rib-oblique-d", z=2.5),
     rev("rib-d-tip", 50, new="rib-oblique-d-tip", z=2.6),
     rev("rib-right-scallop-a", 50, new="scallop-a", z=2.7),
     rev("rib-right-scallop-b", 50, new="scallop-b", z=2.7),
     rev("stretch-right", 50, axis="runner", new="stretcher", z=2.4)],
    [rev("panel-right", 50, plan=True, new="canopy", path=HEM, z=2, penMm=0.5),
     rev("panel-right-seam", 50, plan=True, new="canopy-seam", z=2.1, ring=True),
     rev("panel-right-fold", 50, plan=True, new="canopy-fold", z=2.2, ring=True),
     rev("rib-right-scallop-a", 50, plan=True, new="hem-ring", z=2.7, ring=True),
     rib_plan("rib-right", "rib-right", 1, 0.011, 0.007, penMm=0.4), rib_plan("rib-left", "rib-left", -1, 0.011, 0.007),
     ell("crown-hub", "crown", 0, 0, 0.045, 0.045, z=3.5, tone=0.35, penMm=0.35, **ALU),
     ell("crown-finial", "crown", 0, 0, 0.012, 0.012, z=3.6, tone=0.3, **ALU)],
    {"silhouette": ("Front: base with plate and socket, the tapered mast with tilt collar and crank knob, crown hub with finial, runner with detent and the canopy as a solid of revolution with seam, fold, scalloped hem, the front rib and two oblique ribs with tips and the stretcher; plan: canopy disc with seam, fold and hem rings, the two in-plane ribs and the crown hub with finial.",
                    "Vorderansicht: Fuß mit Platte und Hülse, der verjüngte Mast mit Neigungsmuffe und Kurbelknauf, Kronennabe mit Spitze, Läufer mit Raste und der Schirm als Rotationskörper mit Naht, Falte, Zackensaum, Frontrippe und zwei Schrägrippen mit Spitzen sowie der Spreize; Draufsicht: Schirmscheibe mit Naht-, Falten- und Saumring, die zwei Rippen in der Ebene und die Kronennabe mit Spitze."),
     "omitted": ("Base grit, mast seam and the other four ribs are not drawn in front and plan.",
                 "Betonkörnung, Mastnaht und die übrigen vier Rippen werden in Vorder- und Draufsicht nicht gezeichnet.")})

apply(A, views)
