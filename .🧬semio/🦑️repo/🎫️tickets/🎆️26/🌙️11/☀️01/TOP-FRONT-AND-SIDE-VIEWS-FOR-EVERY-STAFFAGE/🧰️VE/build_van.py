"""delivery-van: front and plan views at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "delivery-van"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

BP, RF, BN, SD, RD, CG, EB, EX, TY, BR = "body-panels", "roof", "bonnet", "slider-door", "rear-doors", "cab-glazing", "engine-bay", "exhaust-system", "tyres", "brakes"
W = 0.91
FRONT_HALF = [(0.0, 0.356), (0.9, 0.356), (0.91, 0.5), (0.91, 1.1), (0.88, 1.8), (0.83, 1.97), (0.78, 1.99), (0.0, 1.99)]


def symx(half):
    return half + [(-x, y) for x, y in reversed(half)]


def symy(half):
    return half + [(x, -y) for x, y in reversed(half)]


PLAN_HALF = [(-2.67, 0.0), (-2.67, 0.82), (-2.6, W), (1.8, W), (2.35, 0.88), (2.6, 0.8), (2.67, 0.65), (2.67, 0.0)]
BONNET_HALF = [(1.8, 0.0), (1.8, 0.88), (2.35, 0.86), (2.6, 0.79), (2.67, 0.65), (2.67, 0.0)]


def tyres_front(z, fr, rear, pen_tone, rr=0.07):
    out = []
    for side, x0 in (("l", 0.74), ("r", -0.94)):
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z - 0.1, rect(x0, 0, 0.2, 0.712, rr), pen_tone, region=TY, material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(x0, 0, 0.2, 0.712, rr), pen_tone, region=TY, material="rubber"))
    return out


def tyres_plan(z, tone, fillet=0.02, merge=None):
    out = []
    for side, sgn in (("l", 1), ("r", -1)):
        y0 = 0.74 if sgn > 0 else -0.94
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z, rect(-1.956, y0, 0.712, 0.2, fillet), tone, regions=merge or [TY], material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(1.344, y0, 0.712, 0.2, fillet), tone, regions=merge or [TY], material="rubber"))
    return out


# ================================================================ 1:50
front = tyres_front(4, True, True, 0.1) + [
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.65, region=BP, material="paint"),
    S("roof-cap", "chassis", "body", 2.2, rect(-0.8, 1.93, 1.6, 0.06, 0.02), 0.75, region=RF, material="paint"),
    S("bonnet-top", "chassis", "body", 2.3, rect(-0.86, 0.95, 1.72, 0.17, 0.04), 0.6, region=BN, material="paint"),
    S("bumper", "chassis", "body", 2.4, rect(-0.9, 0.36, 1.8, 0.18, 0.05), 0.45, region=BP, material="paint"),
    S("grille", "chassis", "detail", 2.5, rect(-0.46, 0.6, 0.92, 0.28, 0.03), 0.3, region=EB, material="engine-metal"),
    S("headlamp-l", "chassis", "glazing", 7, rect(0.55, 0.68, 0.3, 0.16, 0.04), 0.9, material="glass"),
    S("headlamp-r", "chassis", "glazing", 7, rect(-0.85, 0.68, 0.3, 0.16, 0.04), 0.9, material="glass"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.84, 1.13), (0.84, 1.13), (0.79, 1.93), (-0.79, 1.93)]), 0.88, region=CG, material="glass"),
    S("mirror-l", "chassis", "detail", 7, rect(0.91, 1.19, 0.085, 0.22, 0.03), 0.3, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(-0.995, 1.19, 0.085, 0.22, 0.03), 0.3, material="plastic-trim"),
]
plan = tyres_plan(1, 0.1) + [
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.65, region=BP, material="paint"),
    S("slider-panel", "slider", "body", 2.5, rect(-0.8, -W, 1.05, 0.05), 0.6, region=SD, material="paint"),
    S("slider-track", "chassis", "body", 2.45, rect(-1.7, -W - 0.035, 2.0, 0.04), 0.4, region=SD, material="paint"),
    S("rear-leaf-r", "rear-door", "body", 1.5, rect(-2.67, -W, 0.95, 0.05), 0.6, region=RD, material="paint"),
    S("rear-leaf-l", "rear-door", "body", 1.5, rect(-2.67, W - 0.05, 0.95, 0.05), 0.6, region=RD, material="paint"),
    S("roof", "chassis", "body", 2.2, rect(-2.6, -0.86, 3.74, 1.72, 0.05), 0.75, region=RF, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(1.14, -0.84), (1.8, -0.8), (1.8, 0.8), (1.14, 0.84)]), 0.88, region=CG, material="glass"),
    S("bonnet", "chassis", "body", 2.3, poly(symy(BONNET_HALF)), 0.6, region=BN, material="paint"),
    S("headlamp-l", "chassis", "glazing", 7, rect(2.5, 0.5, 0.14, 0.28, 0.03), 0.9, material="glass"),
    S("headlamp-r", "chassis", "glazing", 7, rect(2.5, -0.78, 0.14, 0.28, 0.03), 0.9, material="glass"),
    S("tail-lamp-l", "chassis", "detail", 7, rect(-2.67, 0.7, 0.05, 0.14, 0.01), 0.3, material="plastic-trim"),
    S("tail-lamp-r", "chassis", "detail", 7, rect(-2.67, -0.84, 0.05, 0.14, 0.01), 0.3, material="plastic-trim"),
    S("mirror-l", "chassis", "detail", 7, rect(1.91, 0.91, 0.14, 0.085, 0.03), 0.3, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(1.91, -0.995, 0.14, 0.085, 0.03), 0.3, material="plastic-trim"),
    S("exhaust-tip", "chassis", "detail", 2.6, rect(-2.66, -0.35, 0.3, 0.08, 0.03), 0.4, region=EX, material="exhaust-steel"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: cab and cargo body outline with bumper, grille, bonnet, headlamps, windscreen, roof cap, mirrors and the four tyres behind each other; top: roof, windscreen, bonnet, body outline, lamps, mirrors, the slider and both rear door leaves and the tyre treads beside the body.",
                   "Vorn: Umriss von Fahrerhaus und Laderaum mit Stoßfänger, Kühlergrill, Haube, Scheinwerfern, Windschutzscheibe, Dachkappe, Spiegeln und den vier Reifen hintereinander; oben: Dach, Windschutzscheibe, Haube, Karosserieumriss, Leuchten, Spiegel, Schiebetür und beide Hecktürflügel und die Reifenlaufflächen neben dem Aufbau."),
    "overlays": ("Front and top shapes carry the regions of the side view. The slider and the two rear door leaves are bound to their prismatic bones and sit under the roof outline while closed; opened, they slide out beside and behind the body exactly as in the side view.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Schiebetür und beide Hecktürflügel sind an ihre prismatischen Knochen gebunden und liegen geschlossen unter dem Dachumriss; geöffnet gleiten sie wie in der Seitenansicht neben und hinter den Aufbau."),
})

# ================================================================ 1:100
front = tyres_front(4, True, True, 0.15) + [
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.65, region=BP, material="paint"),
    S("roof-cap", "chassis", "body", 2.2, rect(-0.8, 1.93, 1.6, 0.06, 0.02), 0.75, region=RF, material="paint"),
    S("bonnet-top", "chassis", "body", 2.3, rect(-0.86, 0.95, 1.72, 0.17, 0.04), 0.6, region=BN, material="paint"),
    S("grille", "chassis", "detail", 2.5, rect(-0.7, 0.6, 1.4, 0.28, 0.04), 0.3, regions=[EB, BP], material="engine-metal"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.84, 1.13), (0.84, 1.13), (0.79, 1.93), (-0.79, 1.93)]), 0.88, region=CG, material="glass"),
    S("mirror-l", "chassis", "detail", 7, rect(0.91, 1.19, 0.085, 0.22, 0.03), 0.3, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(-0.995, 1.19, 0.085, 0.22, 0.03), 0.3, material="plastic-trim"),
]
plan = tyres_plan(1, 0.15) + [
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.65, region=BP, material="paint"),
    S("slider-panel", "slider", "body", 2.5, rect(-0.8, -W, 1.05, 0.05), 0.6, region=SD, material="paint"),
    S("slider-track", "chassis", "body", 2.45, rect(-1.7, -W - 0.035, 2.0, 0.04), 0.4, region=SD, material="paint"),
    S("rear-leaf-r", "rear-door", "body", 1.5, rect(-2.67, -W, 0.95, 0.05), 0.6, region=RD, material="paint"),
    S("rear-leaf-l", "rear-door", "body", 1.5, rect(-2.67, W - 0.05, 0.95, 0.05), 0.6, region=RD, material="paint"),
    S("roof", "chassis", "body", 2.2, rect(-2.6, -0.86, 3.74, 1.72, 0.05), 0.75, regions=[RF, EX], material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(1.14, -0.84), (1.8, -0.8), (1.8, 0.8), (1.14, 0.84)]), 0.88, region=CG, material="glass"),
    S("bonnet", "chassis", "body", 2.3, poly(symy(BONNET_HALF)), 0.6, regions=[BN, EB], material="paint"),
    S("headlamp-l", "chassis", "glazing", 7, rect(2.5, 0.5, 0.14, 0.28, 0.03), 0.9, material="glass"),
    S("headlamp-r", "chassis", "glazing", 7, rect(2.5, -0.78, 0.14, 0.28, 0.03), 0.9, material="glass"),
    S("mirror-l", "chassis", "detail", 7, rect(1.91, 0.91, 0.14, 0.085, 0.03), 0.3, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(1.91, -0.995, 0.14, 0.085, 0.03), 0.3, material="plastic-trim"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: tyres, body outline, roof, bonnet, windscreen, mirrors, slider and both rear door leaves.",
                   "Vorder- und Draufsicht als gegliederte Massen: Reifen, Karosserieumriss, Dach, Haube, Windschutzscheibe, Spiegel, Schiebetür und beide Hecktürflügel."),
    "merged": ("Bumper, grille and headlamps are one front face; the exhaust tip and the engine bay belong to the roof and bonnet shapes in plan.",
               "Stoßfänger, Grill und Scheinwerfer sind eine Frontfläche; Endrohr und Motorraum gehören oben zu den Dach- und Haubenformen."),
})

# ================================================================ 1:200
front = tyres_front(4, True, True, 0.2, 0.05) + [
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.65, regions=[BP, RF, BN, EB, EX], material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.84, 1.13), (0.84, 1.13), (0.79, 1.93), (-0.79, 1.93)]), 0.88, region=CG, material="glass"),
]
plan = tyres_plan(1, 0.2, 0, [TY, BR]) + [
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.65, regions=[BP, RF, BN, EB, EX], material="paint"),
    S("slider-panel", "slider", "body", 2.5, rect(-0.8, -W, 1.05, 0.05), 0.55, region=SD, material="paint"),
    S("rear-leaf-r", "rear-door", "body", 1.5, rect(-2.67, -W, 0.95, 0.05), 0.55, region=RD, material="paint"),
    S("rear-leaf-l", "rear-door", "body", 1.5, rect(-2.67, W - 0.05, 0.95, 0.05), 0.55, region=RD, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(1.14, -0.84), (1.8, -0.8), (1.8, 0.8), (1.14, 0.84)]), 0.88, region=CG, material="glass"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep main masses only: tyre blocks, one body shape, the windscreen and, in plan, the slider and both rear door leaves as strips.",
                   "Vorder- und Draufsicht behalten nur die Hauptmassen: Reifenblöcke, eine Karosserieform, die Windschutzscheibe und oben Schiebetür und beide Hecktürflügel als Streifen."),
    "merged": ("Roof, bonnet, bumper, grille and lamps are one body shape; mirrors are dropped.",
               "Dach, Haube, Stoßfänger, Grill und Leuchten sind eine Karosserieform; die Spiegel entfallen."),
})

# ================================================================ 1:500
front = tyres_front(4, True, True, 0.25, 0.04) + [
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.65, regions=[BP, RF, BN, EB, EX], material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.84, 1.13), (0.84, 1.13), (0.79, 1.93), (-0.79, 1.93)]), 0.88, region=CG, material="glass"),
]
plan = [
    S("tyres-r", "wheel-r", "body", 1, rect(-1.956, -0.94, 0.712, 1.88, 0), 0.25, regions=[TY, BR], material="rubber"),
    S("tyres-f", "wheel-f", "body", 1, rect(1.344, -0.94, 0.712, 1.88, 0), 0.25, regions=[TY, BR], material="rubber"),
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.65, regions=[BP, RF, BN, EB, EX], material="paint"),
    S("slider-panel", "slider", "body", 2.5, rect(-0.8, -W, 1.05, 0.05), 0.55, region=SD, material="paint"),
    S("rear-leaf-r", "rear-door", "body", 1.5, rect(-2.67, -W, 0.95, 0.05), 0.55, region=RD, material="paint"),
    S("rear-leaf-l", "rear-door", "body", 1.5, rect(-2.67, W - 0.05, 0.95, 0.05), 0.55, region=RD, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(1.14, -0.84), (1.8, -0.8), (1.8, 0.8), (1.14, 0.84)]), 0.88, region=CG, material="glass"),
]
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes: one body outline with windscreen, tyre blocks beside it, and in plan the slider and both rear door leaves as strips.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten: ein Karosserieumriss mit Windschutzscheibe, daneben Reifenblöcke, oben Schiebetür und beide Hecktürflügel als Streifen."),
})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
