import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "charcoal-grill"
LID = dict(region="lid-shell", material="enamel-steel")
LID5 = dict(regions=["lid-shell", "vents-alu"], material="enamel-steel")
BOWL = dict(region="bowl-shell", material="enamel-steel")
BOWL5 = dict(regions=["bowl-shell", "ash-catcher"], material="enamel-steel")
ASH = dict(region="ash-catcher", material="alu-cast")
LEG = dict(region="legs-steel", material="steel-black")
WOOD = dict(region="handle-wood", material="wood-handle")
VENT = dict(region="vents-alu", material="alu-cast")
HINGE_X = -0.2925
RIM_Y = 0.74



def bez(p0, p1, p2, p3, t):
    u = 1 - t
    return (u**3 * p0[0] + 3 * u * u * t * p1[0] + 3 * u * t * t * p2[0] + t**3 * p3[0], u**3 * p0[1] + 3 * u * u * t * p1[1] + 3 * u * t * t * p2[1] + t**3 * p3[1])


def dome_local(apex, n=6):
    right = [bez((0, 0), (0, 0.14), (0.1225, apex), (0.2925, apex), i / n) for i in range(n + 1)]
    left = [(0.585 - x, y) for x, y in reversed(right[:-1])]
    return [(0, -0.005)] + right + [(0.2925 + (0.2925 - x), y) for x, y in reversed(right[:-1])] + [(0.585, -0.005)]


SOLID = {"kind": "revolved", "axisBone": "lid-vent", "axisX": 0.0, "mirror": True}


def handle_x(id, scale, z=3.5, tone=0.5, **kw):
    if scale == 50:
        return pth(id, "lid", orig_shape(A, 50, "elevation", "handle")["d"], z=z, tone=tone, solid={"kind": "extruded", "z0": -0.03, "z1": 0.03}, hatch="grain", **WOOD, **kw)
    return rect(id, "lid", 0.5475, 0.08, 0.13, 0.06, z=z, tone=tone, solid={"kind": "extruded", "z0": -0.03, "z1": 0.03}, **WOOD, **kw)


def rim_ring(id, **kw):
    return poly(id, "lid", [(0.0225, 0), (0.5625, 0)], role="detail", z=3.6, tone=0.3, closed=False, solid={**SOLID, "mirror": False}, region="lid-shell", **kw)



def lid_front(id, apex, tone, reg, z=3.2, **kw):
    return poly(id, "lid", dome_local(apex), z=z, tone=tone, solid=SOLID, **reg, **kw)


def lid_plan(id, apex, tone, reg, z=3.2, **kw):
    return poly(id, "lid", dome_local(apex), z=z, tone=tone, solid={**SOLID, "mirror": False}, **reg, **kw)


def hline(id, bone, y, x0, x1, reg, z=3.6, tone=0.3, follow=False, **kw):
    return poly(id, bone, [(x0, y), (x1, y)], role="detail", z=z, tone=tone, closed=False, followTip=follow, **reg, **kw)


def turned(id, scale, sid, z, tone, reg, **kw):
    s = orig_shape(A, scale, "elevation", sid)
    return pth(id, s["bone"], s["d"], z=z, tone=tone, **reg, **kw)


views = {}

# ------------------------------------------------------------------ 1:500
front500 = [
    poly("leg-fl", "bowl", [(0.14, 0), (0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    poly("leg-fr", "bowl", [(-0.14, 0), (-0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    pth("bowl", "bowl", "M -0.2925 0.735 C -0.2925 0.6 -0.18 0.46 0 0.46 C 0.18 0.46 0.2925 0.6 0.2925 0.735 Z", z=3, tone=0.3, penMm=0.256, **BOWL5),
    lid_front("lid", 0.22, 0.3, LID5),
    rect("side-handle-l", "bowl", 0.2925, 0.71, 0.04, 0.04, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.3325, 0.71, 0.04, 0.04, z=3.4, tone=0.5, **WOOD),
]
plan500 = [
    ell("bowl", "bowl", 0, 0, 0.2925, 0.2925, z=3, tone=0.3, penMm=0.256, **BOWL5),
    rect("side-handle-l", "bowl", -0.04, 0.2925, 0.08, 0.04, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.04, -0.3325, 0.08, 0.04, z=3.4, tone=0.5, **WOOD),
    lid_plan("lid", 0.22, 0.3, LID5),
    handle_x("handle", 500, penMm=0.25),
]
views[500] = (front500, plan500, {
    "silhouette": ("Front: the bowl profile, the lid dome that lifts with the lid and the two leg pairs as single strokes; plan: the bowl disc, the lid disc that swings back with the lid and the handle stroke.",
                   "Vorderansicht: das Schalenprofil, die Deckelkuppel, die mit dem Deckel hochgeht, und die beiden Beinpaare als Einzelstriche; Draufsicht: die Schalenscheibe, die Deckelscheibe, die mit dem Deckel zurückschwingt, und der Griffstrich."),
    "omitted": ("Wheels, vent dial and ash catcher are merged into the bowl and lid masses in front and plan.",
                "Räder, Lüftungsscheibe und Aschefänger gehen in der Vorder- und Draufsicht in den Schalen- und Deckelmassen auf.")})

# ------------------------------------------------------------------ 1:200
front200 = [
    poly("leg-fl", "bowl", [(0.14, 0), (0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    poly("leg-fr", "bowl", [(-0.14, 0), (-0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    turned("bowl", 200, "bowl", 3, 0.3, BOWL),
    pth("ash-catcher", "bowl", "M -0.12 0.5 L -0.12 0.43 L 0.12 0.43 L 0.12 0.5 Z", z=2.9, tone=0.6, **ASH),
    rect("side-handle-l", "bowl", 0.2925, 0.715, 0.04, 0.03, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.3325, 0.715, 0.04, 0.03, z=3.4, tone=0.5, **WOOD),
    lid_front("lid", 0.2, 0.3, LID),
    rect("vent-dial", "lid-vent", -0.04, 0.935, 0.08, 0.035, z=3.4, tone=0.7, **VENT),
    handle_x("handle", 200),
    hline("rim-line", "bowl", 0.74, -0.2925, 0.2925, dict(region="bowl-shell"), z=3.6),
]
plan200 = [
    ell("bowl", "bowl", 0, 0, 0.2925, 0.2925, z=3, tone=0.3, **BOWL),
    ell("rim-ring", "bowl", 0, 0, 0.2925, 0.2925, z=3.6, tone=0.3, role="detail", filled=False, region="bowl-shell"),
    rect("side-handle-l", "bowl", -0.04, 0.2925, 0.08, 0.04, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.04, -0.3325, 0.08, 0.04, z=3.4, tone=0.5, **WOOD),
    lid_plan("lid", 0.2, 0.3, LID),
    ell("vent-dial", "lid-vent", 0, 0, 0.04, 0.04, z=3.4, tone=0.7, **VENT),
    handle_x("handle", 200),
]
views[200] = (front200, plan200, {
    "silhouette": ("Front: legs, bowl profile with ash catcher and rim line, the lid dome that rises with the lid, vent dial and handle; plan: bowl disc with rim ring and two side handles, lid disc with vent dial and handle.",
                   "Vorderansicht: Beine, Schalenprofil mit Aschefänger und Randlinie, die Deckelkuppel, die mit dem Deckel steigt, Lüftungsscheibe und Griff; Draufsicht: Schalenscheibe mit Randring und zwei Seitengriffen, Deckelscheibe mit Lüftungsscheibe und Griff."),
    "omitted": ("Wheels, feet and the ash catcher are not drawn in plan (hidden under the bowl or too small).",
                "Räder, Füße und der Aschefänger werden in der Draufsicht nicht gezeichnet (unter der Schale verdeckt oder zu klein).")})

# ------------------------------------------------------------------ 1:100
front100 = [
    poly("leg-fl", "bowl", [(0.14, 0.012), (0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    poly("leg-fr", "bowl", [(-0.14, 0.012), (-0.1, 0.58)], role="body", z=2.5, tone=0.3, closed=False, region="legs-steel", penMm=0.13),
    rect("wheel-l", "bowl", 0.13, 0, 0.02, 0.1, z=2.3, tone=0.9, **LEG),
    rect("wheel-r", "bowl", -0.15, 0, 0.02, 0.1, z=2.3, tone=0.9, **LEG),
    hline("leg-ring", "bowl", 0.22, -0.125, 0.125, dict(region="legs-steel"), z=2.4),
    turned("bowl", 100, "bowl", 3, 0.3, BOWL),
    turned("ash-catcher", 100, "ash-catcher", 2.9, 0.6, ASH),
    rect("side-handle-l", "bowl", 0.2925, 0.715, 0.03, 0.03, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.3225, 0.715, 0.03, 0.03, z=3.4, tone=0.5, **WOOD),
    lid_front("lid", 0.2, 0.3, LID),
    rect("vent-dial", "lid-vent", -0.04, 0.935, 0.08, 0.035, z=3.4, tone=0.7, **VENT),
    handle_x("handle", 100),
    hline("rim-line", "bowl", 0.74, -0.2925, 0.2925, dict(region="bowl-shell"), z=3.6),
]
plan100 = [
    ell("bowl", "bowl", 0, 0, 0.2925, 0.2925, z=3, tone=0.3, **BOWL),
    ell("rim-ring", "bowl", 0, 0, 0.28, 0.28, z=3.6, tone=0.3, role="detail", filled=False, region="bowl-shell"),
    rect("side-handle-l", "bowl", -0.035, 0.2925, 0.07, 0.03, z=3.4, tone=0.5, **WOOD),
    rect("side-handle-r", "bowl", -0.035, -0.3225, 0.07, 0.03, z=3.4, tone=0.5, **WOOD),
    rect("wheel-fl", "bowl", 0.22, 0.13, 0.1, 0.025, z=2.3, tone=0.9, **LEG),
    rect("wheel-fr", "bowl", 0.22, -0.155, 0.1, 0.025, z=2.3, tone=0.9, **LEG),
    ell("foot-bl", "bowl", -0.27, 0.14, 0.015, 0.015, z=2.4, tone=0.3, **LEG),
    ell("foot-br", "bowl", -0.27, -0.14, 0.015, 0.015, z=2.4, tone=0.3, **LEG),
    lid_plan("lid", 0.2, 0.3, LID),
    rim_ring("lid-rim-ring"),
    ell("vent-dial", "lid-vent", 0, 0, 0.04, 0.04, z=3.4, tone=0.7, **VENT),
    ell("vent-hole", "lid-vent", 0, 0, 0.012, 0.012, z=3.5, tone=0.1, **VENT),
    handle_x("handle", 100),
]
views[100] = (front100, plan100, {
    "silhouette": ("Front: leg pairs with wheels and ring, the bowl profile with ash catcher and side handles, the lid dome with vent dial and handle that rise with the lid; plan: bowl disc with rim ring, side handles, wheels and feet at the rim, the lid disc with ring, vent dial and handle.",
                   "Vorderansicht: Beinpaare mit Rädern und Ring, das Schalenprofil mit Aschefänger und Seitengriffen, die Deckelkuppel mit Lüftungsscheibe und Griff, die mit dem Deckel steigen; Draufsicht: Schalenscheibe mit Randring, Seitengriffen, Rädern und Füßen am Rand, die Deckelscheibe mit Ring, Lüftungsscheibe und Griff."),
    "retained": ("The lid, its handle and its vent dial are bound to the lid bones, so front and plan follow every lid pose and clip.",
                 "Deckel, Griff und Lüftungsscheibe sind an die Deckelknochen gebunden, Vorder- und Draufsicht folgen daher jeder Deckelpose und jedem Clip.")})

# ------------------------------------------------------------------ 1:50
ash50 = orig_shape(A, 50, "elevation", "ash-catcher")
front50 = [
    cap("leg-fl", "bowl", 0.14, 0.012, 0.1, 0.58, 0.012, 0.012, z=2.5, tone=0.3, **LEG),
    cap("leg-fr", "bowl", -0.14, 0.012, -0.1, 0.58, 0.012, 0.012, z=2.5, tone=0.3, **LEG),
    rect("wheel-l", "bowl", 0.13, 0, 0.02, 0.1, z=2.3, tone=0.9, **LEG),
    rect("wheel-r", "bowl", -0.15, 0, 0.02, 0.1, z=2.3, tone=0.9, **LEG),
    rect("wheel-hub-l", "bowl", 0.135, 0.034, 0.01, 0.032, z=2.4, tone=0.2, **LEG),
    rect("wheel-hub-r", "bowl", -0.145, 0.034, 0.01, 0.032, z=2.4, tone=0.2, **LEG),
    hline("leg-ring", "bowl", 0.22, -0.125, 0.125, dict(region="legs-steel"), z=2.4),
    turned("bowl", 50, "bowl", 3, 0.3, BOWL),
    pth("ash-catcher", "bowl", ash50["d"], z=2.9, tone=0.6, **ASH),
    pth("damper", "ash-damper", "M -0.05 0.495 L 0.05 0.495 L 0.05 0.47 L -0.05 0.47 Z", z=3.0, tone=0.8, region="vents-alu"),
    rect("side-handle-l", "bowl", 0.2925, 0.715, 0.03, 0.03, z=3.4, tone=0.5, hatch="grain", **WOOD),
    rect("side-handle-r", "bowl", -0.3225, 0.715, 0.03, 0.03, z=3.4, tone=0.5, hatch="grain", **WOOD),
    lid_front("lid", 0.2, 0.3, LID),
    rect("vent-dial", "lid-vent", -0.04, 0.935, 0.08, 0.035, z=3.4, tone=0.7, r=0.006, **VENT),
    ell("vent-hole-0", "lid-vent", -0.022, 0.9525, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    ell("vent-hole-1", "lid-vent", 0.0, 0.9525, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    ell("vent-hole-2", "lid-vent", 0.022, 0.9525, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    handle_x("handle", 50),
    hline("rim-line", "bowl", 0.74, -0.2925, 0.2925, dict(region="bowl-shell"), penMm=0.246),
    poly("bowl-highlight", "bowl", [(-0.25, 0.68), (-0.17, 0.585), (-0.08, 0.535)], role="detail", z=3.6, tone=0.3, closed=False, region="bowl-shell"),
]
plan50 = [
    ell("bowl", "bowl", 0, 0, 0.2925, 0.2925, z=3, tone=0.3, **BOWL),
    ell("rim-ring", "bowl", 0, 0, 0.28, 0.28, z=3.6, tone=0.3, role="detail", filled=False, region="bowl-shell", penMm=0.246),
    ell("ash-vent-ring", "ash-damper", 0, 0, 0.05, 0.05, z=3.0, tone=0.8, role="detail", filled=False, region="vents-alu"),
    rect("side-handle-l", "bowl", -0.035, 0.2925, 0.07, 0.03, z=3.4, tone=0.5, hatch="grain", **WOOD),
    rect("side-handle-r", "bowl", -0.035, -0.3225, 0.07, 0.03, z=3.4, tone=0.5, hatch="grain", **WOOD),
    rect("wheel-fl", "bowl", 0.22, 0.13, 0.1, 0.025, z=2.3, tone=0.9, **LEG),
    rect("wheel-fr", "bowl", 0.22, -0.155, 0.1, 0.025, z=2.3, tone=0.9, **LEG),
    ell("wheel-hub-fl", "bowl", 0.27, 0.1425, 0.01, 0.01, z=2.4, tone=0.2, **LEG),
    ell("wheel-hub-fr", "bowl", 0.27, -0.1425, 0.01, 0.01, z=2.4, tone=0.2, **LEG),
    ell("foot-bl", "bowl", -0.27, 0.14, 0.015, 0.015, z=2.4, tone=0.3, **LEG),
    ell("foot-br", "bowl", -0.27, -0.14, 0.015, 0.015, z=2.4, tone=0.3, **LEG),
    lid_plan("lid", 0.2, 0.3, LID),
    rim_ring("lid-rim-ring", penMm=0.246),
    ell("vent-dial", "lid-vent", 0, 0, 0.04, 0.04, z=3.4, tone=0.7, **VENT),
    ell("vent-hole-0", "lid-vent", -0.022, 0, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    ell("vent-hole-1", "lid-vent", 0.0, 0, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    ell("vent-hole-2", "lid-vent", 0.022, 0, 0.006, 0.006, z=3.5, tone=0.1, **VENT),
    handle_x("handle", 50),
]
views[50] = (front50, plan50, {
    "silhouette": ("Front: both leg pairs as tubes, wheels with hubs, ring, bowl with ash catcher, ash damper, side handles with grain, the lid dome with rim, vent dial and holes, and the grained handle, all lifting with the lid; plan: bowl with rim ring, ash vent ring, side handles, wheels, feet, the lid disc with rim ring, vent dial with three holes and the grained handle.",
                   "Vorderansicht: beide Beinpaare als Rohre, Räder mit Naben, Ring, Schale mit Aschefänger, Ascheschieber, Seitengriffe mit Maserung, die Deckelkuppel mit Rand, Lüftungsscheibe und Löchern sowie der gemaserte Griff, alle mit dem Deckel steigend; Draufsicht: Schale mit Randring, Aschelüftungsring, Seitengriffen, Rädern, Füßen, die Deckelscheibe mit Randring, Lüftungsscheibe mit drei Löchern und der gemaserte Griff."),
    "retained": ("Lid dome, handle, vent dial and the lid disc follow every lid pose and clip through the lid bone itself.",
                 "Deckelkuppel, Griff, Lüftungsscheibe und Deckelscheibe folgen jeder Deckelpose und jedem Clip über den Deckelknochen selbst.")})

apply(A, views, drop=("lid-axis", "lid-handle"), strip={"retained": ("through two massless lid reference bones.", "über zwei masselose Deckel-Bezugsknochen.")})
