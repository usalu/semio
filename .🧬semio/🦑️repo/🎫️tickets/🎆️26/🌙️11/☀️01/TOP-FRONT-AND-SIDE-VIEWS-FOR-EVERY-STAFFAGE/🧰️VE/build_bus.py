"""city-bus: front and plan views at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "city-bus"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

BP, RF, GL, DR, BT, HV, EA, TY, BR = "body-panels", "roof", "glazing", "doors", "battery-pack", "hvac-unit", "e-axle", "tyres", "brakes"
W = 1.275
LEAVES = (("door-m-aft", 6.35), ("door-m-fwd", 6.9), ("door-f-aft", 10.35), ("door-f-fwd", 10.9))
FRONT_HALF = [(0.0, 0.3), (1.26, 0.3), (1.275, 0.5), (1.275, 2.85), (1.2, 2.98), (0.0, 2.98)]


def symx(half):
    return half + [(-x, y) for x, y in reversed(half)]


def tyres_front(tone, rr, z=4):
    out = []
    for side, x0 in (("l", 0.9), ("r", -1.2)):
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z - 0.1, rect(x0, 0, 0.3, 0.96, rr), tone, region=TY, material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(x0, 0, 0.3, 0.96, rr), tone, region=TY, material="rubber"))
    return out


def door_plan(z_open, z_leaf, tone_open=0.06, tone_leaf=0.5, thick=0.05, glass=True):
    out = [
        S("door-opening-m", "body", "body", z_open, rect(6.35, -W, 1.1, thick), tone_open, material="interior-trim"),
        S("door-opening-f", "body", "body", z_open, rect(10.35, -W, 1.1, thick), tone_open, material="interior-trim"),
    ]
    for bone, x0 in LEAVES:
        out.append(S("leaf-" + bone[5:], bone, "body", z_leaf, rect(x0, -W, 0.55, thick), tone_leaf, region=DR, material="door-leaf"))
    return out


# ================================================================ 1:50
front = tyres_front(0.1, 0.1) + [
    S("body-shell", "body", "body", 2, poly(symx(FRONT_HALF)), 0.7, region=BP, material="paint"),
    S("roof-cap", "body", "detail", 2.1, rect(-1.2, 2.88, 2.4, 0.1, 0.04), 0.8, region=RF, material="paint"),
    S("battery-pack", "battery", "body", 2.2, rect(-1.1, 2.98, 2.2, 0.14, 0.03), 0.55, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 2.3, rect(-1.0, 2.98, 2.0, 0.16, 0.03), 0.6, region=HV, material="hvac-housing"),
    S("bumper", "body", "detail", 2.4, rect(-1.275, 0.3, 2.55, 0.3, 0.06), 0.5, region=BP, material="paint"),
    S("destination-sign", "body", "glazing", 6, rect(-0.95, 2.72, 1.9, 0.14, 0.02), 0.8, region=GL, material="glass"),
    S("windscreen-l", "body", "glazing", 6, poly([(0.03, 1.2), (1.22, 1.2), (1.18, 2.68), (0.03, 2.68)]), 0.75, region=GL, material="glass"),
    S("windscreen-r", "body", "glazing", 6, poly([(-0.03, 1.2), (-1.22, 1.2), (-1.18, 2.68), (-0.03, 2.68)]), 0.75, region=GL, material="glass"),
    S("headlamp-l", "body", "glazing", 7, rect(0.8, 0.62, 0.35, 0.22, 0.04), 0.9, material="glass"),
    S("headlamp-r", "body", "glazing", 7, rect(-1.15, 0.62, 0.35, 0.22, 0.04), 0.9, material="glass"),
    S("mirror-l", "body", "detail", 7, rect(1.2, 2.1, 0.075, 0.38, 0.03), 0.3, material="paint"),
    S("mirror-r", "body", "detail", 7, rect(-1.275, 2.1, 0.075, 0.38, 0.03), 0.3, material="paint"),
]
plan = door_plan(2.5, 3) + [
    S("body-shell", "body", "body", 2, rect(0, -W, 12, 2 * W, 0.25), 0.7, region=BP, material="paint"),
    S("roof-panel", "body", "detail", 2.1, rect(0.06, -1.2, 11.64, 2.4, 0.15), 0.8, region=RF, material="paint"),
    S("battery-pack", "battery", "body", 2.2, rect(1.0, -1.05, 6.3, 2.1, 0.08), 0.55, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 2.2, rect(8.0, -1.0, 2.5, 2.0, 0.08), 0.6, region=HV, material="hvac-housing"),
    S("windscreen", "body", "glazing", 6, poly([(11.73, -1.2), (11.98, -1.15), (11.98, 1.15), (11.73, 1.2)]), 0.75, region=GL, material="glass"),
    S("headlamp-l", "body", "glazing", 7, rect(11.94, 0.8, 0.06, 0.35, 0.02), 0.9, material="glass"),
    S("headlamp-r", "body", "glazing", 7, rect(11.94, -1.15, 0.06, 0.35, 0.02), 0.9, material="glass"),
    S("tail-lamp-l", "body", "detail", 7, rect(0.0, 0.8, 0.06, 0.35, 0.02), 0.7, material="glass"),
    S("tail-lamp-r", "body", "detail", 7, rect(0.0, -1.15, 0.06, 0.35, 0.02), 0.7, material="glass"),
    S("mirror-l", "body", "detail", 7, rect(11.77, 1.2, 0.155, 0.075, 0.03), 0.3, material="paint"),
    S("mirror-r", "body", "detail", 7, rect(11.77, -1.275, 0.155, 0.075, 0.03), 0.3, material="paint"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: body outline with bumper, destination sign, split windscreen, headlamps, mirrors, the roof battery and HVAC boxes and the four tyres; top: roof with battery and HVAC boxes, windscreen, lamps, mirrors and the four door leaves as strips on the near side.",
                   "Vorn: Aufbauumriss mit Stoßfänger, Zielanzeige, geteilter Windschutzscheibe, Scheinwerfern, Spiegeln, den Dachkästen für Batterie und Klimagerät und den vier Reifen; oben: Dach mit Batterie- und Klimakasten, Windschutzscheibe, Leuchten, Spiegel und die vier Türflügel als Streifen auf der Nahseite."),
    "overlays": ("Front and top shapes carry the regions of the side view. The four door leaves are bound to their prismatic bones, so they slide apart over the dark door opening exactly as in the side view; the wheels are hidden under the body in plan and therefore not drawn there.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Die vier Türflügel sind an ihre prismatischen Knochen gebunden und gleiten wie in der Seitenansicht über der dunklen Türöffnung auseinander; die Räder liegen oben unter dem Aufbau und werden dort nicht gezeichnet."),
})

# ================================================================ 1:100
front = tyres_front(0.15, 0.1) + [
    S("body-shell", "body", "body", 2, poly(symx(FRONT_HALF)), 0.62, region=BP, material="paint"),
    S("roof-cap", "body", "detail", 2.1, rect(-1.2, 2.88, 2.4, 0.1, 0.04), 0.5, region=RF, material="paint"),
    S("battery-pack", "battery", "body", 2.2, rect(-1.1, 2.98, 2.2, 0.14, 0.03), 0.45, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 2.3, rect(-1.0, 2.98, 2.0, 0.16, 0.03), 0.5, region=HV, material="hvac-housing"),
    S("bumper", "body", "detail", 2.4, rect(-1.275, 0.3, 2.55, 0.3, 0.06), 0.45, region=BP, material="paint"),
    S("destination-sign", "body", "glazing", 6, rect(-0.95, 2.72, 1.9, 0.14, 0.02), 0.8, region=GL, material="glass"),
    S("windscreen-l", "body", "glazing", 6, poly([(0.04, 1.2), (1.22, 1.2), (1.18, 2.66), (0.04, 2.66)]), 0.8, region=GL, material="glass"),
    S("windscreen-r", "body", "glazing", 6, poly([(-0.04, 1.2), (-1.22, 1.2), (-1.18, 2.66), (-0.04, 2.66)]), 0.8, region=GL, material="glass"),
    S("headlamps", "body", "detail", 7, rect(-1.15, 0.62, 2.3, 0.22, 0.04), 0.85, material="glass"),
    S("mirror-l", "body", "detail", 7, rect(1.2, 2.1, 0.075, 0.38, 0.03), 0.3, material="paint"),
    S("mirror-r", "body", "detail", 7, rect(-1.275, 2.1, 0.075, 0.38, 0.03), 0.3, material="paint"),
]
plan = door_plan(2.5, 3, 0.05, 0.75, 0.06) + [
    S("body-shell", "body", "body", 2, rect(0, -W, 12, 2 * W, 0.25), 0.62, region=BP, material="paint"),
    S("roof-panel", "body", "detail", 2.1, rect(0.06, -1.2, 11.64, 2.4, 0.15), 0.5, region=RF, material="paint"),
    S("battery-pack", "battery", "body", 3, rect(1.0, -1.05, 6.3, 2.1, 0.08), 0.45, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 3, rect(8.0, -1.0, 2.5, 2.0, 0.08), 0.5, region=HV, material="hvac-housing"),
    S("windscreen", "body", "glazing", 6, poly([(11.73, -1.2), (11.98, -1.15), (11.98, 1.15), (11.73, 1.2)]), 0.85, region=GL, material="glass"),
    S("mirror-l", "body", "detail", 7, rect(11.77, 1.2, 0.155, 0.075, 0.03), 0.3, material="paint"),
    S("mirror-r", "body", "detail", 7, rect(11.77, -1.275, 0.155, 0.075, 0.03), 0.3, material="paint"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: body outline, roof cap, battery and HVAC boxes, windscreen halves, destination sign, mirrors, tyres and, in plan, the door leaves.",
                   "Vorder- und Draufsicht als gegliederte Massen: Aufbauumriss, Dachkappe, Batterie- und Klimakasten, Windschutzscheibenhälften, Zielanzeige, Spiegel, Reifen und oben die Türflügel."),
    "merged": ("Front: the two windscreen halves, the destination sign and both mirrors are drawn, the headlamps are one bar; top: the mirrors are drawn, the lamps are dropped.",
               "Vorn: die beiden Scheibenhälften, die Zielanzeige und beide Spiegel sind gezeichnet, die Scheinwerfer sind ein Balken; oben sind die Spiegel gezeichnet, die Leuchten entfallen."),
})

# ================================================================ 1:200
front = tyres_front(0.2, 0.08) + [
    S("body-shell", "body", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, EA], material="paint"),
    S("roof-boxes", "battery", "body", 2.2, rect(-1.1, 2.98, 2.2, 0.16, 0.03), 0.5, regions=[BT, HV], material="battery-housing"),
    S("windscreen", "body", "glazing", 6, poly([(-1.22, 1.2), (1.22, 1.2), (1.18, 2.84), (-1.18, 2.84)]), 0.85, region=GL, material="glass"),
]
plan = door_plan(2.5, 3, 0.05, 0.75, 0.07) + [
    S("body-shell", "body", "body", 2, rect(0, -W, 12, 2 * W, 0.25), 0.62, regions=[BP, RF, GL, EA], material="paint"),
    S("battery-pack", "battery", "body", 3, rect(1.0, -1.05, 6.3, 2.1, 0.08), 0.45, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 3, rect(8.0, -1.0, 2.5, 2.0, 0.08), 0.5, region=HV, material="hvac-housing"),
    S("windscreen", "body", "glazing", 6, poly([(11.73, -1.2), (11.98, -1.15), (11.98, 1.15), (11.73, 1.2)]), 0.85, region=GL, material="glass"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep main masses only: one body shape, the roof boxes, the windscreen, tyre blocks and the four door leaves as strips.",
                   "Vorder- und Draufsicht behalten nur die Hauptmassen: eine Aufbauform, die Dachkästen, die Windschutzscheibe, Reifenblöcke und die vier Türflügel als Streifen."),
    "merged": ("Roof cap, bumper, lamps and mirrors are part of the body shape; battery and HVAC boxes are one roof block in front view.",
               "Dachkappe, Stoßfänger, Leuchten und Spiegel gehören zur Aufbauform; Batterie- und Klimakasten sind vorn ein Dachblock."),
})

# ================================================================ 1:500
front = tyres_front(0.25, 0.05) + [
    S("body-shell", "body", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, EA], material="paint"),
    S("roof-boxes", "battery", "body", 2.2, rect(-1.1, 2.98, 2.2, 0.17, 0.03), 0.45, regions=[BT, HV], material="battery-housing"),
    S("windscreen", "body", "glazing", 6, poly([(-1.22, 1.2), (1.22, 1.2), (1.18, 2.84), (-1.18, 2.84)]), 0.85, region=GL, material="glass"),
]
plan = [
    S("body-shell", "body", "body", 2, rect(0, -W, 12, 2 * W, 0.25), 0.62, regions=[BP, RF, GL, EA], material="paint"),
    S("door-opening-m", "body", "body", 2.5, rect(6.35, -W, 1.1, 0.1), 0.1, region=DR, material="door-leaf"),
    S("door-opening-f", "body", "body", 2.5, rect(10.35, -W, 1.1, 0.1), 0.1, region=DR, material="door-leaf"),
    S("roof-boxes-r", "battery", "body", 3, rect(1.0, -1.05, 6.3, 2.1, 0.08), 0.45, region=BT, material="battery-housing"),
    S("hvac-unit", "hvac", "body", 3, rect(8.0, -1.0, 2.5, 2.0, 0.08), 0.5, region=HV, material="hvac-housing"),
    S("windscreen", "body", "glazing", 6, poly([(11.73, -1.2), (11.98, -1.15), (11.98, 1.15), (11.73, 1.2)]), 0.85, region=GL, material="glass"),
]
for bone, x0 in LEAVES:
    plan.append(S("leaf-" + bone[5:], bone, "body", 3.1, rect(x0, -W, 0.55, 0.1), 0.7, region=DR, material="door-leaf"))
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes: one body outline with the roof boxes and windscreen, tyre blocks and in plan the two door openings with their four leaves.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten: ein Aufbauumriss mit Dachkästen und Windschutzscheibe, Reifenblöcke und oben die beiden Türöffnungen mit ihren vier Flügeln."),
    "thinElements": ("The door leaf strips of the plan are widened to 0.1 m (0.2 mm on paper); in reality the sliding leaves lie in a 0.05 m wall.",
                     "Die Türflügelstreifen der Draufsicht sind auf 0,1 m (0,2 mm auf Papier) verbreitert; in Wirklichkeit liegen die Schiebeflügel in einer 0,05 m dicken Wand."),
})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
