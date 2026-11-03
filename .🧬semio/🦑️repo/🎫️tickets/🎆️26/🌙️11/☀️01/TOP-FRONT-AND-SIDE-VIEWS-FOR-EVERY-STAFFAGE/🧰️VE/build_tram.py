"""tram: front and plan views at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "tram"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

BP, RF, GL, DR, GW, TM, WS, BD, BR_, HV, PN = "body-panels", "roof", "glazing", "doors", "gangway", "traction-motors", "wheelsets", "brake-discs", "brake-resistor", "hvac-unit", "pantograph"
W = 1.2
FRONT_HALF = [(0.0, 0.28), (1.2, 0.28), (1.2, 3.0), (1.1, 3.4), (0.0, 3.4)]
END_HALF = [(0.45, 0.0), (0.45, W), (8.9, W), (9.3, 1.1), (9.45, 0.8), (9.45, 0.0)]
MID_HALF = [(-6.0, 0.0), (-6.0, 0.8), (-5.85, 1.1), (-5.5, W), (-0.45, W), (-0.45, 0.0)]
DOORS = (("door-a1", 3.5), ("door-a2", 4.15), ("door-b1", -4.0), ("door-b2", -3.35))
PAN_LOW, PAN_HIGH = (2.6, 3.55), (4.362, 4.262)


def symx(half):
    return half + [(-x, y) for x, y in reversed(half)]


def symy(half):
    return half + [(x, -y) for x, y in reversed(half)]


def wheels_front(tone, rr, z=1.5):
    return [
        S("wheel-l", "ws-f2", "body", z, rect(0.65, 0, 0.13, 0.6, rr), tone, region=WS, material="steel-wheel"),
        S("wheel-r", "ws-f2", "body", z, rect(-0.78, 0, 0.13, 0.6, rr), tone, region=WS, material="steel-wheel"),
    ]


def pantograph_front(w_head, pen_head, base=True, thick=0.06):
    out = [
        S("pan-lower", "pan-lower", "body", 3, rect(-thick / 2, 3.55, thick, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
        S("pan-upper", "pan-upper", "body", 3.1, rect(-thick / 2, 4.262, thick, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
        S("pan-head", "pan-head", "detail" if pen_head else "body", 3.2, rect(-w_head, 4.944, 2 * w_head, 0.06, 0.02), 0.2, region=PN, material="carbon"),
    ]
    if base:
        out.append(S("pan-base", "body-end", "detail", 3.3, rect(-0.4, 3.4, 0.8, 0.15, 0.02), 0.4, region=PN, material="pan-alloy"))
    return out


def pantograph_plan(r, w_head):
    return [
        S("pan-base", "body-end", "detail", 3.3, rect(2.3, -0.4, 0.6, 0.8, 0.03), 0.4, region=PN, material="pan-alloy"),
        S("pan-lower", "pan-lower", "body", 3, cap(2.6, 0, 4.362, 0, r, follow=True), 0.3, region=PN, material="pan-alloy"),
        S("pan-upper", "pan-upper", "body", 3.1, cap(4.362, 0, 2.6, 0, r, follow=True), 0.3, region=PN, material="pan-alloy"),
        S("pan-head", "pan-head", "detail", 3.2, rect(2.32, -w_head, 0.56, 2 * w_head, 0.03), 0.2, region=PN, material="carbon"),
    ]


def doors_plan(z_open, z_leaf, thick, tone_leaf=0.6):
    out = [
        S("door-opening-a", "body-end", "body", z_open, rect(3.5, -W, 1.3, thick), 0.06, material="interior-trim"),
        S("door-opening-b", "body-end", "body", z_open, rect(-4.0, -W, 1.3, thick), 0.06, material="interior-trim"),
    ]
    for bone, x0 in DOORS:
        out.append(S("leaf-" + bone[5:], bone, "body", z_leaf, rect(x0, -W, 0.65, thick), tone_leaf, region=DR, material="paint"))
    return out


# ================================================================ 1:50
front = wheels_front(0.2, 0.03) + [
    S("shell-end", "body-end", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF], material="paint"),
    S("hvac-box", "body-mid", "body", 2.5, rect(-0.9, 3.4, 1.8, 0.26, 0.04), 0.7, region=HV, material="paint"),
    S("resistor-box", "body-end", "body", 2.6, rect(-0.85, 3.4, 1.7, 0.24, 0.03), 0.35, region=BR_, material="resistor-steel"),
    S("windscreen", "body-end", "glazing", 6, poly([(-1.05, 1.25), (1.05, 1.25), (0.98, 2.75), (-0.98, 2.75)]), 0.88, region=GL, material="glass"),
    S("destination-sign", "body-end", "glazing", 6, rect(-0.7, 2.85, 1.4, 0.2, 0.03), 0.6, region=GL, material="glass"),
    S("headlamp-l", "body-end", "glazing", 7, rect(0.7, 0.85, 0.3, 0.25, 0.05), 0.9, material="glass"),
    S("headlamp-r", "body-end", "glazing", 7, rect(-1.0, 0.85, 0.3, 0.25, 0.05), 0.9, material="glass"),
] + pantograph_front(0.6, True)
plan = doors_plan(2.5, 3, 0.06) + [
    S("shell-end", "body-end", "body", 2, poly(symy(END_HALF)), 0.62, region=BP, material="paint"),
    S("shell-mid", "body-mid", "body", 2, poly(symy(MID_HALF)), 0.62, region=BP, material="paint"),
    S("gangway", "gangway", "body", 2.2, rect(-0.45, -1.0, 0.9, 2.0, 0.1), 0.2, region=GW, material="rubber"),
    S("roof-end", "body-end", "detail", 2.1, rect(0.6, -1.0, 8.4, 2.0, 0.15), 0.8, region=RF, material="paint"),
    S("roof-mid", "body-mid", "detail", 2.1, rect(-5.85, -1.0, 5.2, 2.0, 0.15), 0.8, region=RF, material="paint"),
    S("hvac-box", "body-mid", "body", 2.5, rect(-5.2, -0.9, 3.8, 1.8, 0.06), 0.7, region=HV, material="paint"),
    S("resistor-box", "body-end", "body", 2.6, rect(4.5, -0.9, 2.0, 1.8, 0.05), 0.35, region=BR_, material="resistor-steel"),
    S("windscreen", "body-end", "glazing", 6, poly([(9.1, -1.0), (9.35, -0.95), (9.35, 0.95), (9.1, 1.0)]), 0.88, region=GL, material="glass"),
    S("headlamp-l", "body-end", "glazing", 7, rect(9.39, 0.7, 0.06, 0.3, 0.02), 0.9, material="glass"),
    S("headlamp-r", "body-end", "glazing", 7, rect(9.39, -1.0, 0.06, 0.3, 0.02), 0.9, material="glass"),
] + pantograph_plan(0.03, 0.6)
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: wheels below the skirt, cab front with windscreen, destination sign and headlamps, the roof boxes and the single-arm pantograph with its collector; top: both car shells with the gangway, roof boxes, pantograph, windscreen, lamps and the four door leaves as strips.",
                   "Vorn: Räder unter der Schürze, Führerstandsfront mit Windschutzscheibe, Zielanzeige und Scheinwerfern, die Dachkästen und der Einholm-Stromabnehmer mit Schleifstück; oben: beide Wagenkästen mit Übergang, Dachkästen, Stromabnehmer, Windschutzscheibe, Leuchten und die vier Türflügel als Streifen."),
    "overlays": ("Front and top shapes carry the regions of the side view. The pantograph arms are followTip rects (front) and capsules (top) bound to their own bones, the collector head follows the upper arm tip, so the raised and the depot pose read in every view; the door leaves slide over the dark openings; wheels and motors are hidden by the skirt in front view and by the shells in plan.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Die Stromabnehmerarme sind followTip-Rechtecke (vorn) und -Kapseln (oben) an eigenen Knochen, das Schleifstück folgt der Spitze des Oberarms, so lesen sich die ausgefahrene und die Depotstellung in jeder Ansicht; die Türflügel gleiten über die dunklen Öffnungen; Räder und Motoren sind vorn durch die Schürze und oben durch die Wagenkästen verdeckt."),
})

# ================================================================ 1:100
front = wheels_front(0.2, 0.03) + [
    S("shell-end", "body-end", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF], material="paint"),
    S("hvac-box", "body-mid", "body", 2.5, rect(-0.9, 3.4, 1.8, 0.26, 0.04), 0.5, region=HV, material="paint"),
    S("resistor-box", "body-end", "body", 2.6, rect(-0.85, 3.4, 1.7, 0.24, 0.03), 0.4, region=BR_, material="resistor-steel"),
    S("windscreen", "body-end", "glazing", 6, poly([(-1.05, 1.25), (1.05, 1.25), (0.98, 3.05), (-0.98, 3.05)]), 0.85, region=GL, material="glass"),
    S("headlamps", "body-end", "detail", 7, rect(-1.0, 0.85, 2.0, 0.25, 0.05), 0.85, material="glass"),
] + pantograph_front(0.6, True, True, 0.07)
plan = doors_plan(2.5, 3, 0.07, 0.75) + [
    S("shell-end", "body-end", "body", 2, poly(symy(END_HALF)), 0.62, region=BP, material="paint"),
    S("shell-mid", "body-mid", "body", 2, poly(symy(MID_HALF)), 0.62, region=BP, material="paint"),
    S("gangway", "gangway", "body", 2.2, rect(-0.45, -1.0, 0.9, 2.0, 0.1), 0.25, region=GW, material="rubber"),
    S("roof", "body-end", "detail", 2.1, rect(0.6, -1.0, 8.4, 2.0, 0.15), 0.5, region=RF, material="paint"),
    S("hvac-box", "body-mid", "body", 2.5, rect(-5.2, -0.9, 3.8, 1.8, 0.06), 0.5, region=HV, material="paint"),
    S("resistor-box", "body-end", "body", 2.6, rect(4.5, -0.9, 2.0, 1.8, 0.05), 0.4, region=BR_, material="resistor-steel"),
    S("windscreen", "body-end", "glazing", 6, poly([(9.1, -1.0), (9.35, -0.95), (9.35, 0.95), (9.1, 1.0)]), 0.85, region=GL, material="glass"),
] + pantograph_plan(0.035, 0.6)
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: wheels, cab front, roof boxes, pantograph, shells with gangway, windscreen and the door leaves.",
                   "Vorder- und Draufsicht als gegliederte Massen: Räder, Führerstandsfront, Dachkästen, Stromabnehmer, Wagenkästen mit Übergang, Windschutzscheibe und die Türflügel."),
    "merged": ("Front: windscreen and destination sign are one glass shape and the headlamps one bar; top: lamps are dropped and both roof areas are one shape.",
               "Vorn: Windschutzscheibe und Zielanzeige sind eine Glasform und die Scheinwerfer ein Balken; oben entfallen die Leuchten und beide Dachflächen sind eine Form."),
})

# ================================================================ 1:200
front = wheels_front(0.25, 0.0) + [
    S("shell-end", "body-end", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, TM, BD], material="paint"),
    S("roof-boxes", "body-end", "body", 2.6, rect(-0.9, 3.4, 1.8, 0.26, 0.03), 0.45, regions=[HV, BR_], material="resistor-steel"),
    S("windscreen", "body-end", "glazing", 6, poly([(-1.05, 1.25), (1.05, 1.25), (0.98, 3.05), (-0.98, 3.05)]), 0.85, region=GL, material="glass"),
    S("pan-lower", "pan-lower", "body", 3, rect(-0.05, 3.55, 0.1, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
    S("pan-upper", "pan-upper", "body", 3.1, rect(-0.05, 4.262, 0.1, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
    S("pan-head", "pan-head", "body", 3.2, rect(-0.6, 4.934, 1.2, 0.07), 0.2, region=PN, material="carbon"),
]
plan = doors_plan(2.5, 3, 0.08, 0.75) + [
    S("shell-end", "body-end", "body", 2, poly(symy(END_HALF)), 0.62, regions=[BP, RF, GL], material="paint"),
    S("shell-mid", "body-mid", "body", 2, poly(symy(MID_HALF)), 0.62, regions=[BP, RF, GL], material="paint"),
    S("gangway", "gangway", "body", 2.2, rect(-0.45, -1.0, 0.9, 2.0, 0.1), 0.25, region=GW, material="rubber"),
    S("hvac-box", "body-mid", "body", 2.5, rect(-5.2, -0.9, 3.8, 1.8, 0.06), 0.5, region=HV, material="paint"),
    S("resistor-box", "body-end", "body", 2.6, rect(4.5, -0.9, 2.0, 1.8, 0.05), 0.4, region=BR_, material="resistor-steel"),
    S("pan-lower", "pan-lower", "body", 3, cap(2.6, 0, 4.362, 0, 0.05, follow=True), 0.3, region=PN, material="pan-alloy"),
    S("pan-upper", "pan-upper", "body", 3.1, cap(4.362, 0, 2.6, 0, 0.05, follow=True), 0.3, region=PN, material="pan-alloy"),
    S("pan-head", "pan-head", "body", 3.2, rect(2.32, -0.6, 0.56, 1.2, 0.03), 0.2, region=PN, material="carbon"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep main masses only: wheels, one cab shape with windscreen, the roof block, the pantograph as one arm and collector, and in plan both shells, gangway, boxes and the door leaves.",
                   "Vorder- und Draufsicht behalten nur die Hauptmassen: Räder, eine Führerstandsform mit Windschutzscheibe, den Dachblock, den Stromabnehmer als ein Arm mit Schleifstück und oben beide Wagenkästen, Übergang, Kästen und die Türflügel."),
    "merged": ("The pantograph arms are plain followTip rects without base; lamps and destination sign are dropped.",
               "Die Stromabnehmerarme sind schlichte followTip-Rechtecke ohne Sockel; Leuchten und Zielanzeige entfallen."),
})

# ================================================================ 1:500
front = wheels_front(0.25, 0.0) + [
    S("shell-end", "body-end", "body", 2, poly(symx(FRONT_HALF)), 0.62, regions=[BP, RF, TM, BD, HV, BR_], material="paint"),
    S("windscreen", "body-end", "glazing", 6, poly([(-1.05, 1.25), (1.05, 1.25), (0.98, 3.05), (-0.98, 3.05)]), 0.85, region=GL, material="glass"),
    S("pan-lower", "pan-lower", "body", 3, rect(-0.06, 3.55, 0.12, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
    S("pan-upper", "pan-upper", "body", 3.1, rect(-0.06, 4.262, 0.12, 0.712, None, True), 0.3, region=PN, material="pan-alloy"),
    S("pan-head", "pan-head", "body", 3.2, rect(-0.6, 4.924, 1.2, 0.08), 0.2, region=PN, material="carbon"),
]
plan = [
    S("shell-end", "body-end", "body", 2, poly(symy(END_HALF)), 0.62, regions=[BP, RF, GL, BR_], material="paint"),
    S("shell-mid", "body-mid", "body", 2, poly(symy(MID_HALF)), 0.62, regions=[BP, RF, GL, HV], material="paint"),
    S("gangway", "gangway", "body", 2.2, rect(-0.45, -1.0, 0.9, 2.0, 0.1), 0.3, region=GW, material="rubber"),
    S("door-opening-a", "body-end", "body", 2.5, rect(3.5, -W, 1.3, 0.1), 0.1, region=DR, material="paint"),
    S("door-opening-b", "body-end", "body", 2.5, rect(-4.0, -W, 1.3, 0.1), 0.1, region=DR, material="paint"),
    S("pan-lower", "pan-lower", "body", 3, cap(2.6, 0, 4.362, 0, 0.06, follow=True), 0.3, region=PN, material="pan-alloy"),
    S("pan-upper", "pan-upper", "body", 3.1, cap(4.362, 0, 2.6, 0, 0.06, follow=True), 0.3, region=PN, material="pan-alloy"),
    S("pan-head", "pan-head", "body", 3.2, rect(2.32, -0.6, 0.56, 1.2, 0.03), 0.2, region=PN, material="carbon"),
]
for bone, x0 in DOORS:
    plan.append(S("leaf-" + bone[5:], bone, "body", 3.3, rect(x0, -W, 0.65, 0.1), 0.7, region=DR, material="paint"))
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes: wheels, one body outline with windscreen, the pantograph as widened arm and collector, and in plan both shells, gangway, door openings and leaves.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten: Räder, ein Aufbauumriss mit Windschutzscheibe, der Stromabnehmer als verbreiterter Arm mit Schleifstück und oben beide Wagenkästen, Übergang, Türöffnungen und Flügel."),
    "thinElements": ("Pantograph arms are widened to 0.1 m (front) and 0.12 m (plan) and the door leaf strips to 0.1 m so that they stay above 0.2 mm on paper.",
                     "Stromabnehmerarme sind vorn auf 0,1 m und oben auf 0,12 m, die Türflügelstreifen auf 0,1 m verbreitert, damit sie auf Papier über 0,2 mm bleiben."),
})

def pan(r, hw, hr=0.03):
    return {"pan-lower": ("pan-lower", ext(cap(0, 0, 1.9, 0, r), -r, r)), "pan-upper": ("pan-upper", ext(cap(0, 0, 1.9, 0, r), -r, r)),
            "pan-head": ("pan-head", ext(cap(-0.28, 0, 0.28, 0, hr), -hw, hw))}


apply_rules(a, {"50": {"front": pan(0.03, 0.6), "plan": pan(0.03, 0.6)}, "100": {"front": pan(0.035, 0.6), "plan": pan(0.035, 0.6)},
                "200": {"front": pan(0.05, 0.6, 0.04), "plan": pan(0.05, 0.6, 0.04)}, "500": {"front": pan(0.06, 0.6), "plan": pan(0.06, 0.6)}})

for shape in a["representations"]["200"]["geometry"]["elevation"]:
    if shape["id"] == "pan-head":
        shape["r0"] = shape["r1"] = 0.04

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
