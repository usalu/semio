"""city-car: front and plan views (replaces the old plan) at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "city-car"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

BP, RF, BN, TG, GL, IN, EB, EX, TY, BR = "body-panels", "roof", "bonnet", "tailgate", "glazing", "interior", "engine-bay", "exhaust-system", "tyres", "brakes"
TR = 0.72
FRONT_HALF = [(0.0, 0.2), (0.72, 0.2), (0.76, 0.35), (0.76, 0.9), (0.72, 1.05), (0.66, 1.4), (0.6, 1.495), (0.0, 1.495)]
PLAN_HALF = [(-1.95, 0.0), (-1.95, 0.55), (-1.93, 0.68), (-1.8, 0.74), (-1.5, 0.76), (-0.8, 0.76), (0.8, 0.76), (1.3, 0.75), (1.7, 0.7), (1.9, 0.55), (1.951, 0.3), (1.951, 0.0)]
ROOF_HALF = [(0.31, 0.0), (0.3, 0.55), (0.08, 0.63), (-0.5, 0.655), (-1.2, 0.64), (-1.45, 0.6), (-1.5, 0.0)]


def symx(half):
    return half + [(-x, y) for x, y in reversed(half)]


def symy(half):
    return half + [(x, -y) for x, y in reversed(half)]


def tyres_front(tone, rr, z=4, wide=False):
    out = []
    w = 0.2025 if wide else 0.185
    for side, x0 in (("l", TR - 0.0925), ("r", -TR - 0.0925 - (0.0175 if wide else 0))):
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z - 0.1, rect(x0, 0, w, 0.6, rr), tone, region=TY, material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(x0, 0, w, 0.6, rr), tone, region=TY, material="rubber"))
    return out


def tyres_plan(tone, rr, z=1, regions=None, wide=False):
    out = []
    w = 0.2025 if wide else 0.185
    for side, y0 in (("l", TR - 0.0925), ("r", -TR - 0.0925 - (0.0175 if wide else 0))):
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z, rect(-1.5, y0, 0.6, w, rr), tone, regions=regions or [TY], material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(0.95, y0, 0.6, w, rr), tone, regions=regions or [TY], material="rubber"))
    return out


def bonnet_plan(n, r):
    ys = [(-(n - 1) / 2 + i) * (2 * 0.68 - 2 * r) / (n - 1) for i in range(n)]
    return [S(f"bonnet-{i}", "bonnet", "body", 2.3, cap(0.95 + r, y, 1.864 - r, y, r, follow=True), 0.62, region=BN, material="paint") for i, y in enumerate(ys)]


def hatch_plan(n, r):
    ys = [(-(n - 1) / 2 + i) * (2 * 0.7 - 2 * r) / (n - 1) for i in range(n)]
    return [S(f"hatch-{i}", "hatch", "body", 2.3, cap(-1.5 - r, y, -1.931 + r, y, r, follow=True), 0.62, region=TG, material="paint") for i, y in enumerate(ys)]


# ================================================================ 1:50
front = tyres_front(0.1, 0.08) + [
    S("silencer", "chassis", "detail", 0.8, rect(-0.45, 0.055, 0.2, 0.15, 0.05), 0.4, region=EX, material="exhaust-steel"),
    S("hatch-panel", "hatch", "body", 1.5, rect(-0.58, 0.838, 1.16, 0.602, 0.05, True), 0.62, region=TG, material="paint"),
    S("hatch-glass", "hatch", "glazing", 1.6, rect(-0.48, 1.0, 0.96, 0.34, 0.04, True), 0.85, region=GL, material="glass"),
    S("engine-bay", "chassis", "detail", 1.8, rect(-0.62, 0.75, 1.24, 0.22, 0.03), 0.12, region=EB, material="engine-metal"),
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.62, region=BP, material="paint"),
    S("roof-band", "chassis", "detail", 2.1, rect(-0.6, 1.43, 1.2, 0.067, 0.02), 0.5, region=RF, material="paint"),
    S("bonnet-panel", "bonnet", "body", 2.3, rect(-0.7, 0.799, 1.4, 0.171, 0.03, True), 0.62, region=BN, material="paint"),
    S("bumper", "chassis", "detail", 2.4, rect(-0.75, 0.2, 1.5, 0.17, 0.04), 0.45, region=BP, material="paint"),
    S("grille", "chassis", "detail", 2.5, rect(-0.4, 0.48, 0.8, 0.15, 0.03), 0.3, region=EB, material="engine-metal"),
    S("seat-front-headrest-l", "chassis", "detail", 3, rect(0.2, 1.1, 0.18, 0.22, 0.05), 0.3, region=IN, material="interior-trim"),
    S("seat-front-headrest-r", "chassis", "detail", 3, rect(-0.38, 1.1, 0.18, 0.22, 0.05), 0.3, region=IN, material="interior-trim"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.7, 1.01), (0.7, 1.01), (0.6, 1.43), (-0.6, 1.43)]), 0.85, region=GL, material="glass"),
    S("headlamp-l", "chassis", "glazing", 7, rect(0.45, 0.66, 0.28, 0.12, 0.04), 0.85, region=GL, material="glass"),
    S("headlamp-r", "chassis", "glazing", 7, rect(-0.73, 0.66, 0.28, 0.12, 0.04), 0.85, region=GL, material="glass"),
    S("mirror-l", "chassis", "detail", 7, rect(0.76, 1.05, 0.08, 0.1, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(-0.84, 1.05, 0.08, 0.1, 0.03), 0.3, region=BP, material="plastic-trim"),
]
plan = tyres_plan(0.1, 0.03) + [
    S("engine-bay", "chassis", "detail", 1.8, rect(1.0, -0.6, 0.85, 1.2, 0.05), 0.12, region=EB, material="engine-metal"),
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.62, region=BP, material="paint"),
    S("roof", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.5, region=RF, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(0.9, -0.7), (0.9, 0.7), (0.31, 0.6), (0.31, -0.6)]), 0.85, region=GL, material="glass"),
] + bonnet_plan(5, 0.14) + hatch_plan(5, 0.14) + [
    S("mirror-l", "chassis", "detail", 7, rect(0.76, 0.76, 0.12, 0.08, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(0.76, -0.84, 0.12, 0.08, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("headlamp-l", "chassis", "glazing", 7, rect(1.76, 0.5, 0.17, 0.2, 0.04), 0.85, region=GL, material="glass"),
    S("headlamp-r", "chassis", "glazing", 7, rect(1.76, -0.7, 0.17, 0.2, 0.04), 0.85, region=GL, material="glass"),
    S("tail-lamp-l", "chassis", "detail", 7, rect(-1.96, 0.5, 0.07, 0.22, 0.01), 0.7, region=TG, material="plastic-trim"),
    S("tail-lamp-r", "chassis", "detail", 7, rect(-1.96, -0.72, 0.07, 0.22, 0.01), 0.7, region=TG, material="plastic-trim"),
    S("tail-pipe", "chassis", "detail", 7, cap(-1.92, -0.3, -1.945, -0.3, 0.025), 0.45, region=EX, material="exhaust-steel"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: tyres, body outline with bumper, grille, bonnet band, headlamps, windscreen with seat headrests, roof band and mirrors; the tailgate panel and the engine bay show behind and under the closed parts. Top: roof, windscreen, bonnet and tailgate, lamps, mirrors and the tyre treads beside the body.",
                   "Vorn: Reifen, Karosserieumriss mit Stoßfänger, Grill, Haubenband, Scheinwerfern, Windschutzscheibe mit Kopfstützen, Dachband und Spiegeln; Heckklappe und Motorraum zeigen sich hinter und unter den geschlossenen Teilen. Oben: Dach, Windschutzscheibe, Haube und Heckklappe, Leuchten, Spiegel und die Reifenlaufflächen neben dem Aufbau."),
    "overlays": ("Front and top shapes carry the regions of the side view; the total width of 1.68 m includes the mirrors. Bonnet and tailgate are followTip rects (front: they rise above the roof when open) and five followTip capsules each (top: they shorten and reveal the engine bay), so the open poses read in every view.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht; die Gesamtbreite von 1,68 m schließt die Spiegel ein. Haube und Heckklappe sind followTip-Rechtecke (vorn: sie steigen geöffnet über das Dach) und je fünf followTip-Kapseln (oben: sie verkürzen sich und geben den Motorraum frei), so lesen sich die geöffneten Posen in jeder Ansicht."),
})

# ================================================================ 1:100
front = tyres_front(0.15, 0.08) + [
    S("silencer", "chassis", "detail", 0.8, rect(-0.45, 0.055, 0.2, 0.15, 0.05), 0.4, region=EX, material="exhaust-steel"),
    S("hatch-panel", "hatch", "body", 1.5, rect(-0.58, 0.838, 1.16, 0.602, 0.05, True), 0.62, region=TG, material="paint"),
    S("hatch-glass", "hatch", "glazing", 1.6, rect(-0.48, 1.0, 0.96, 0.34, 0.04, True), 0.85, region=GL, material="glass"),
    S("engine-bay", "chassis", "detail", 1.8, rect(-0.62, 0.75, 1.24, 0.22, 0.03), 0.15, region=EB, material="engine-metal"),
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, IN], material="paint"),
    S("roof-band", "chassis", "detail", 2.1, rect(-0.6, 1.43, 1.2, 0.067, 0.02), 0.5, region=RF, material="paint"),
    S("bonnet-panel", "bonnet", "body", 2.3, rect(-0.7, 0.799, 1.4, 0.171, 0.03, True), 0.62, region=BN, material="paint"),
    S("grille", "chassis", "detail", 2.5, rect(-0.45, 0.48, 0.9, 0.15, 0.03), 0.3, region=EB, material="engine-metal"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.7, 1.01), (0.7, 1.01), (0.6, 1.43), (-0.6, 1.43)]), 0.85, region=GL, material="glass"),
    S("headlamps", "chassis", "glazing", 7, rect(-0.73, 0.66, 1.46, 0.12, 0.04), 0.85, region=GL, material="glass"),
    S("mirror-l", "chassis", "detail", 7, rect(0.76, 1.05, 0.08, 0.1, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(-0.84, 1.05, 0.08, 0.1, 0.03), 0.3, region=BP, material="plastic-trim"),
]
plan = tyres_plan(0.15, 0.03) + [
    S("engine-bay", "chassis", "detail", 1.8, rect(1.0, -0.6, 0.85, 1.2, 0.05), 0.15, region=EB, material="engine-metal"),
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.62, regions=[BP, EX], material="paint"),
    S("roof", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.5, region=RF, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(0.9, -0.7), (0.9, 0.7), (0.31, 0.6), (0.31, -0.6)]), 0.85, region=GL, material="glass"),
] + bonnet_plan(3, 0.23) + hatch_plan(4, 0.17) + [
    S("mirror-l", "chassis", "detail", 7, rect(0.76, 0.76, 0.12, 0.08, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("mirror-r", "chassis", "detail", 7, rect(0.76, -0.84, 0.12, 0.08, 0.03), 0.3, region=BP, material="plastic-trim"),
    S("tail-pipe", "chassis", "detail", 7, cap(-1.92, -0.3, -1.945, -0.3, 0.025), 0.45, region=EX, material="exhaust-steel"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: tyres, body outline, roof, windscreen, grille and headlamp bar, bonnet and tailgate, mirrors.",
                   "Vorder- und Draufsicht als gegliederte Massen: Reifen, Karosserieumriss, Dach, Windschutzscheibe, Grill und Scheinwerferband, Haube und Heckklappe, Spiegel."),
    "merged": ("Both headlamps and the bumper are one bar; the bonnet and the tailgate are three followTip capsules each in plan; seats and lamps are dropped.",
               "Beide Scheinwerfer und der Stoßfänger sind ein Balken; Haube und Heckklappe sind oben je drei followTip-Kapseln; Sitze und Leuchten entfallen."),
})

# ================================================================ 1:200
front = tyres_front(0.2, 0.06) + [
    S("hatch-panel", "hatch", "body", 1.5, rect(-0.58, 0.838, 1.16, 0.602, None, True), 0.62, regions=[TG, GL], material="paint"),
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, EB, EX, IN], material="paint"),
    S("bonnet-panel", "bonnet", "body", 2.3, rect(-0.7, 0.799, 1.4, 0.171, None, True), 0.55, region=BN, material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(-0.7, 1.01), (0.7, 1.01), (0.6, 1.43), (-0.6, 1.43)]), 0.85, region=GL, material="glass"),
]
plan = tyres_plan(0.2, 0.0, 1, [TY, BR]) + [
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.62, regions=[BP, RF, EB, EX], material="paint"),
    S("windscreen", "chassis", "glazing", 6, poly([(0.9, -0.7), (0.9, 0.7), (0.31, 0.6), (0.31, -0.6)]), 0.85, region=GL, material="glass"),
    S("roof", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.5, region=RF, material="paint"),
] + bonnet_plan(3, 0.23) + hatch_plan(3, 0.2)
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep main masses only: tyre blocks, one body shape, windscreen and roof, bonnet and tailgate; mirrors and lamps are dropped, so the width is the body plus tyres.",
                   "Vorder- und Draufsicht behalten nur die Hauptmassen: Reifenblöcke, eine Karosserieform, Windschutzscheibe und Dach, Haube und Heckklappe; Spiegel und Leuchten entfallen, die Breite ist Aufbau plus Reifen."),
    "merged": ("Grille, bumper, lamps and engine bay are part of the body shape; the tailgate glass is part of the tailgate shape.",
               "Grill, Stoßfänger, Leuchten und Motorraum gehören zur Karosserieform; das Heckscheibenglas gehört zur Heckklappenform."),
})

# ================================================================ 1:500
front = tyres_front(0.25, 0.04, wide=True) + [
    S("hatch-panel", "hatch", "body", 1.5, rect(-0.58, 0.838, 1.16, 0.602, None, True), 0.62, regions=[TG, GL], material="paint"),
    S("body-shell", "chassis", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, EB, EX], material="paint"),
    S("bonnet-panel", "bonnet", "body", 2.3, rect(-0.7, 0.799, 1.4, 0.171, None, True), 0.55, region=BN, material="paint"),
    S("greenhouse", "chassis", "glazing", 6, poly([(-0.7, 1.01), (0.7, 1.01), (0.6, 1.43), (-0.6, 1.43)]), 0.85, region=GL, material="glass"),
]
plan = tyres_plan(0.25, 0.0, 1, [TY, BR], wide=True) + [
    S("body-shell", "chassis", "body", 2, poly(symy(PLAN_HALF)), 0.62, regions=[BP, RF, EB, EX], material="paint"),
    S("greenhouse", "chassis", "glazing", 6, poly([(0.9, -0.7), (0.9, 0.7), (0.31, 0.6), (-1.4, 0.6), (-1.4, -0.6), (0.31, -0.6)]), 0.85, region=GL, material="glass"),
] + bonnet_plan(3, 0.23) + hatch_plan(3, 0.2)
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes: body, glass area, tyre blocks, bonnet and tailgate as followTip shapes.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten: Aufbau, Glasfläche, Reifenblöcke, Haube und Heckklappe als followTip-Formen."),
})

BON = poly([(-0.05, -0.07), (0.9, -0.07), (0.94, 0.03), (-0.05, 0.02)])
HAT = poly([(-0.05, -0.05), (0.8, -0.05), (0.8, 0.03), (0.74, 0.095), (-0.05, 0.08)])
HGL = poly([(0.03, -0.03), (0.45, -0.015), (0.45, 0.05), (0.03, 0.055)])


def plan_rules(nb, nh):
    t = {"bonnet-0": ("bonnet", ext(BON, -0.68, 0.68)), "hatch-0": ("hatch", ext(HAT, -0.7, 0.7))}
    t.update({f"bonnet-{i}": None for i in range(1, nb)})
    t.update({f"hatch-{i}": None for i in range(1, nh)})
    return t


front_rules = {"bonnet-panel": ("bonnet", ext(BON, -0.7, 0.7)), "hatch-panel": ("hatch", ext(HAT, -0.58, 0.58)), "hatch-glass": ("hatch", ext(HGL, -0.48, 0.48))}
front_rules_500 = dict(front_rules, **{"hatch-panel": ("hatch", ext(poly([(-0.05, -0.07), (0.8, -0.07), (0.8, 0.05), (0.74, 0.115), (-0.05, 0.1)]), -0.58, 0.58))})
apply_rules(a, {"50": {"front": front_rules, "plan": plan_rules(5, 5)}, "100": {"front": front_rules, "plan": plan_rules(3, 4)},
                "200": {"front": front_rules, "plan": plan_rules(3, 3)}, "500": {"front": front_rules_500, "plan": plan_rules(3, 3)}})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
