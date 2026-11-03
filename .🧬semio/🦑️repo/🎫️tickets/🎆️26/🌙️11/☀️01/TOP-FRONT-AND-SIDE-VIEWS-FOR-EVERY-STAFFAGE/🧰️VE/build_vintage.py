"""vintage-car-1929: front and plan views (replaces the old plan) at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "vintage-car-1929"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

BP, RF, HD, FN, GL, IN, RD, EB, EX, TY, BR = "body-panels", "roof", "hood", "fenders", "glazing", "interior", "radiator", "engine-bay", "exhaust-system", "tyres", "brakes"
TR, TW = 0.67, 0.115


def symx(half):
    return half + [(-x, y) for x, y in reversed(half)]


def symy(half):
    return half + [(x, -y) for x, y in reversed(half)]


def both(f):
    return [f(1, "l"), f(-1, "r")]


def mirror_poly(pts, sgn):
    return [(x, sgn * y) for x, y in pts]


def tyres_front(tone, rr, hubs=True, z=4):
    out = []
    for side, sgn in (("l", 1), ("r", -1)):
        x0 = TR - TW / 2 if sgn > 0 else -TR - TW / 2
        out.append(S(f"tyre-r{side}", "wheel-r", "body", z - 0.1, rect(x0, 0, TW, 0.74, rr), tone, region=TY, material="rubber"))
        out.append(S(f"tyre-f{side}", "wheel-f", "body", z, rect(x0, 0, TW, 0.74, rr), tone, region=TY, material="rubber"))
        if hubs:
            hx = TR + TW / 2 if sgn > 0 else -TR - TW / 2 - 0.04
            out.append(S(f"hub-f{side}", "wheel-f", "detail", 5.2, rect(hx, 0.305, 0.04, 0.13, 0.015), 0.8, region=TY, material="nickel-plate"))
    return out


FENDER_F = [(0.5, 0.45), (0.85, 0.45), (0.85, 0.7), (0.8, 0.82), (0.68, 0.86), (0.55, 0.84), (0.5, 0.75)]
FENDER_PF = [(0.96, 0.5), (0.96, 0.78), (1.1, 0.85), (1.7, 0.85), (2.0, 0.8), (2.14, 0.65), (2.14, 0.5)]
FENDER_PR = [(-0.98, 0.55), (-0.98, 0.8), (-1.1, 0.85), (-1.8, 0.85), (-2.0, 0.78), (-2.03, 0.6), (-2.03, 0.55)]
TUB_HALF = [(-1.99, 0.0), (-1.99, 0.5), (-1.9, 0.62), (0.6, 0.62), (0.6, 0.0)]
ROOF_HALF = [(-1.7, 0.0), (-1.7, 0.55), (-1.55, 0.6), (0.5, 0.6), (0.62, 0.5), (0.62, 0.0)]

# ================================================================ 1:50
front = tyres_front(0.08, 0.05) + [
    S("frame", "chassis", "body", 1, rect(-0.4, 0.35, 0.8, 0.06), 0.18, region=BP, material="exhaust-steel"),
    S("silencer", "chassis", "detail", 0.9, rect(-0.4, 0.195, 0.2, 0.15, 0.06), 0.4, region=EX, material="exhaust-steel"),
    S("running-board-l", "chassis", "body", 1.5, rect(0.55, 0.4, 0.28, 0.06), 0.42, region=FN, material="paint"),
    S("running-board-r", "chassis", "body", 1.5, rect(-0.83, 0.4, 0.28, 0.06), 0.42, region=FN, material="paint"),
    S("tub", "chassis", "body", 2, rect(-0.62, 0.52, 1.24, 0.62, 0.03), 0.6, region=BP, material="paint"),
    S("cabin", "chassis", "body", 2.1, rect(-0.6, 1.14, 1.2, 0.5, 0.05), 0.58, region=BP, material="paint"),
    S("roof-slab", "chassis", "detail", 2.4, rect(-0.62, 1.58, 1.24, 0.12, 0.04), 0.78, region=RF, material="fabric-roof"),
    S("hood", "chassis", "body", 2.3, rect(-0.34, 0.58, 0.68, 0.625, 0.05), 0.62, region=HD, material="paint"),
    S("louvre-l", "chassis", "detail", 2.5, rect(0.3, 0.96, 0.04, 0.17, 0.01), 0.12, region=EB, material="engine-metal"),
    S("louvre-r", "chassis", "detail", 2.5, rect(-0.34, 0.96, 0.04, 0.17, 0.01), 0.12, region=EB, material="engine-metal"),
    S("windscreen", "chassis", "glazing", 6, rect(-0.52, 1.21, 1.04, 0.39), 0.85, region=GL, material="glass"),
    S("windscreen-post", "chassis", "detail", 6.1, poly([(0.0, 1.21), (0.0, 1.6)], False), 0.5, filled=False, region=GL, material="glass", pen=0.18),
    S("radiator-shell", "chassis", "body", 2.6, rect(-0.22, 0.56, 0.44, 0.7, 0.03), 0.78, region=RD, material="nickel-plate"),
    S("radiator-neck", "chassis", "detail", 2.7, rect(-0.018, 1.242, 0.036, 0.076), 0.78, region=RD, material="nickel-plate"),
    S("radiator-cap", "chassis", "detail", 2.8, rect(-0.05, 1.303, 0.1, 0.064, 0.02), 0.78, region=RD, material="nickel-plate"),
    S("spare-tyre", "chassis", "body", 3.6, rect(-0.615, 0.47, TW, 0.62, 0.05), 0.08, region=TY, material="rubber"),
    S("fender-l", "chassis", "body", 3, poly(FENDER_F), 0.5, region=FN, material="paint"),
    S("fender-r", "chassis", "body", 3, poly(mirror_poly(FENDER_F, 1)[:0] or [(-x, y) for x, y in FENDER_F]), 0.5, region=FN, material="paint"),
    S("lamp-stalk-l", "chassis", "detail", 3.1, rect(0.535, 0.785, 0.03, 0.13), 0.5, region=FN, material="nickel-plate"),
    S("lamp-stalk-r", "chassis", "detail", 3.1, rect(-0.565, 0.785, 0.03, 0.13), 0.5, region=FN, material="nickel-plate"),
    S("headlamp-bowl-l", "chassis", "body", 3.2, ell(0.55, 0.99, 0.11, 0.11), 0.74, region=FN, material="nickel-plate"),
    S("headlamp-bowl-r", "chassis", "body", 3.2, ell(-0.55, 0.99, 0.11, 0.11), 0.74, region=FN, material="nickel-plate"),
    S("headlamp-lens-l", "chassis", "glazing", 3.3, ell(0.55, 0.99, 0.085, 0.085), 0.88, region=GL, material="glass"),
    S("headlamp-lens-r", "chassis", "glazing", 3.3, ell(-0.55, 0.99, 0.085, 0.085), 0.88, region=GL, material="glass"),
    S("bumper-front-upper", "chassis", "body", 3.4, rect(-0.7, 0.64, 1.4, 0.044, 0.015), 0.8, region=BP, material="nickel-plate"),
    S("bumper-front-lower", "chassis", "body", 3.4, rect(-0.7, 0.378, 1.4, 0.044, 0.015), 0.8, region=BP, material="nickel-plate"),
]
plan = [
    S("frame-l", "chassis", "body", 1, cap(-1.98, 0.38, 2.08, 0.38, 0.03), 0.18, region=BP, material="exhaust-steel"),
    S("frame-r", "chassis", "body", 1, cap(-1.98, -0.38, 2.08, -0.38, 0.03), 0.18, region=BP, material="exhaust-steel"),
    S("tail-pipe", "chassis", "detail", 0.9, cap(-1.85, -0.35, -2.067, -0.35, 0.0375), 0.45, region=EX, material="exhaust-steel"),
    S("running-board-l", "chassis", "body", 1.5, rect(-1.0, 0.62, 2.0, 0.18, 0.02), 0.42, region=FN, material="paint"),
    S("running-board-r", "chassis", "body", 1.5, rect(-1.0, -0.8, 2.0, 0.18, 0.02), 0.42, region=FN, material="paint"),
    S("tub", "chassis", "body", 2, poly(symy(TUB_HALF)), 0.6, region=BP, material="paint"),
    S("hood", "chassis", "body", 2.3, rect(0.62, -0.34, 1.16, 0.68, 0.06), 0.62, region=HD, material="paint"),
    S("louvre-l", "chassis", "detail", 2.5, rect(0.9, 0.26, 0.65, 0.05, 0.015), 0.12, region=EB, material="engine-metal"),
    S("louvre-r", "chassis", "detail", 2.5, rect(0.9, -0.31, 0.65, 0.05, 0.015), 0.12, region=EB, material="engine-metal"),
    S("roof-slab", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.78, region=RF, material="fabric-roof"),
    S("windscreen", "chassis", "glazing", 6.1, rect(0.47, -0.52, 0.105, 1.04), 0.85, region=GL, material="glass"),
    S("radiator-shell", "chassis", "body", 2.6, rect(1.78, -0.22, 0.1, 0.44, 0.02), 0.78, region=RD, material="nickel-plate"),
    S("radiator-cap", "chassis", "detail", 2.8, ell(1.83, 0, 0.03, 0.04), 0.78, region=RD, material="nickel-plate"),
    S("spare-tyre", "chassis", "body", 3.6, rect(0.55, -0.615, 0.62, TW, 0.05), 0.08, region=TY, material="rubber"),
    S("fender-fl", "chassis", "body", 3, poly(FENDER_PF), 0.5, region=FN, material="paint"),
    S("fender-fr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PF]), 0.5, region=FN, material="paint"),
    S("fender-rl", "chassis", "body", 3, poly(FENDER_PR), 0.5, region=FN, material="paint"),
    S("fender-rr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PR]), 0.5, region=FN, material="paint"),
    S("headlamp-bowl-l", "chassis", "body", 3.2, ell(1.78, 0.55, 0.105, 0.11), 0.74, region=FN, material="nickel-plate"),
    S("headlamp-bowl-r", "chassis", "body", 3.2, ell(1.78, -0.55, 0.105, 0.11), 0.74, region=FN, material="nickel-plate"),
    S("headlamp-lens-l", "chassis", "glazing", 3.3, ell(1.866, 0.55, 0.018, 0.085), 0.88, region=GL, material="glass"),
    S("headlamp-lens-r", "chassis", "glazing", 3.3, ell(1.866, -0.55, 0.018, 0.085), 0.88, region=GL, material="glass"),
    S("bracket-front-l", "chassis", "detail", 3.3, cap(2.06, 0.3, 2.19, 0.3, 0.02), 0.6, region=BP, material="exhaust-steel"),
    S("bracket-front-r", "chassis", "detail", 3.3, cap(2.06, -0.3, 2.19, -0.3, 0.02), 0.6, region=BP, material="exhaust-steel"),
    S("bumper-front", "chassis", "body", 3.4, rect(2.198, -0.7, 0.044, 1.4, 0.015), 0.8, region=BP, material="nickel-plate"),
    S("bracket-rear-l", "chassis", "detail", 3.3, cap(-2.09, 0.3, -1.96, 0.3, 0.02), 0.6, region=BP, material="exhaust-steel"),
    S("bracket-rear-r", "chassis", "detail", 3.3, cap(-2.09, -0.3, -1.96, -0.3, 0.02), 0.6, region=BP, material="exhaust-steel"),
    S("bumper-rear", "chassis", "body", 3.4, rect(-2.142, -0.7, 0.044, 1.4, 0.015), 0.8, region=BP, material="nickel-plate"),
    S("number-plate", "chassis", "detail", 7, rect(-2.015, -0.2, 0.03, 0.4), 0.9, region=BP, material="paint"),
    S("tail-lamp", "chassis", "detail", 7, rect(-2.03, 0.35, 0.05, 0.1, 0.01), 0.3, region=BP, material="paint"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: tyres with hub caps, frame, running boards, body with cabin, hood, folding-top slab, windscreen with post, nickel radiator shell with cap, side-mounted spare, fenders, headlamps on stalks and the two bumper bars; top: frame rails, running boards, body, hood with side louvres, top, windscreen, radiator, spare, fenders, headlamps, bumpers, plate and exhaust end.",
                   "Vorn: Reifen mit Radkappen, Rahmen, Trittbretter, Aufbau mit Kabine, Haube, Verdeckplatte, Windschutzscheibe mit Pfosten, vernickelter Kühlerschale mit Kappe, seitlich montiertem Reserverad, Kotflügeln, Scheinwerfern auf Stielen und den beiden Stoßstangenbändern; oben: Rahmenholme, Trittbretter, Aufbau, Haube mit seitlichen Lüftungsschlitzen, Verdeck, Windschutzscheibe, Kühler, Reserverad, Kotflügel, Scheinwerfer, Stoßstangen, Kennzeichen und Auspuffende."),
    "overlays": ("Front and top shapes carry the regions of the side view; the total width of 1.70 m is the fender width. The wheels are drawn in front view and hidden under the fenders in plan, so the wheel poses read in the front view.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht; die Gesamtbreite von 1,70 m ist die Kotflügelbreite. Die Räder sind vorn gezeichnet und oben unter den Kotflügeln verdeckt, so lesen sich die Radposen in der Vorderansicht."),
})

# ================================================================ 1:100
front = tyres_front(0.1, 0.05, True) + [
    S("frame", "chassis", "body", 1, rect(-0.4, 0.35, 0.8, 0.06), 0.18, region=BP, material="exhaust-steel"),
    S("silencer", "chassis", "detail", 0.9, rect(-0.4, 0.195, 0.2, 0.15, 0.06), 0.4, region=EX, material="exhaust-steel"),
    S("running-board-l", "chassis", "body", 1.5, rect(0.55, 0.4, 0.28, 0.06), 0.42, region=FN, material="paint"),
    S("running-board-r", "chassis", "body", 1.5, rect(-0.83, 0.4, 0.28, 0.06), 0.42, region=FN, material="paint"),
    S("tub", "chassis", "body", 2, rect(-0.62, 0.52, 1.24, 0.62, 0.03), 0.6, region=BP, material="paint"),
    S("cabin", "chassis", "body", 2.1, rect(-0.6, 1.14, 1.2, 0.5, 0.05), 0.58, region=BP, material="paint"),
    S("roof-slab", "chassis", "detail", 2.4, rect(-0.62, 1.58, 1.24, 0.12, 0.04), 0.78, region=RF, material="fabric-roof"),
    S("hood", "chassis", "body", 2.3, rect(-0.34, 0.58, 0.68, 0.625, 0.05), 0.62, regions=[HD, EB], material="paint"),
    S("windscreen", "chassis", "glazing", 6, rect(-0.52, 1.21, 1.04, 0.39), 0.85, region=GL, material="glass"),
    S("radiator-shell", "chassis", "body", 2.6, rect(-0.22, 0.56, 0.44, 0.82, 0.03), 0.78, region=RD, material="nickel-plate"),
    S("spare-tyre", "chassis", "body", 3.6, rect(-0.615, 0.47, TW, 0.62, 0.05), 0.08, region=TY, material="rubber"),
    S("fender-l", "chassis", "body", 3, poly(FENDER_F), 0.5, region=FN, material="paint"),
    S("fender-r", "chassis", "body", 3, poly([(-x, y) for x, y in FENDER_F]), 0.5, region=FN, material="paint"),
    S("headlamp-l", "chassis", "body", 3.2, ell(0.55, 0.99, 0.11, 0.11), 0.74, regions=[FN, GL], material="nickel-plate"),
    S("headlamp-r", "chassis", "body", 3.2, ell(-0.55, 0.99, 0.11, 0.11), 0.74, regions=[FN, GL], material="nickel-plate"),
    S("bumper-front", "chassis", "body", 3.4, rect(-0.7, 0.378, 1.4, 0.306, 0.02), 0.8, region=BP, material="nickel-plate"),
]
plan = [
    S("frame-l", "chassis", "body", 1, cap(-1.98, 0.38, 2.08, 0.38, 0.03), 0.18, region=BP, material="exhaust-steel"),
    S("frame-r", "chassis", "body", 1, cap(-1.98, -0.38, 2.08, -0.38, 0.03), 0.18, region=BP, material="exhaust-steel"),
    S("tail-pipe", "chassis", "detail", 0.9, cap(-1.85, -0.35, -2.067, -0.35, 0.0375), 0.45, region=EX, material="exhaust-steel"),
    S("running-board-l", "chassis", "body", 1.5, rect(-1.0, 0.62, 2.0, 0.18), 0.42, region=FN, material="paint"),
    S("running-board-r", "chassis", "body", 1.5, rect(-1.0, -0.8, 2.0, 0.18), 0.42, region=FN, material="paint"),
    S("tub", "chassis", "body", 2, poly(symy(TUB_HALF)), 0.6, region=BP, material="paint"),
    S("hood", "chassis", "body", 2.3, rect(0.62, -0.34, 1.16, 0.68, 0.06), 0.62, regions=[HD, EB], material="paint"),
    S("roof-slab", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.78, region=RF, material="fabric-roof"),
    S("windscreen", "chassis", "glazing", 6.1, rect(0.47, -0.52, 0.105, 1.04), 0.85, region=GL, material="glass"),
    S("radiator-shell", "chassis", "body", 2.6, rect(1.78, -0.22, 0.1, 0.44, 0.02), 0.78, region=RD, material="nickel-plate"),
    S("spare-tyre", "chassis", "body", 3.6, rect(0.55, -0.615, 0.62, TW, 0.05), 0.08, region=TY, material="rubber"),
    S("fender-fl", "chassis", "body", 3, poly(FENDER_PF), 0.5, region=FN, material="paint"),
    S("fender-fr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PF]), 0.5, region=FN, material="paint"),
    S("fender-rl", "chassis", "body", 3, poly(FENDER_PR), 0.5, region=FN, material="paint"),
    S("fender-rr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PR]), 0.5, region=FN, material="paint"),
    S("headlamp-l", "chassis", "body", 3.2, ell(1.78, 0.55, 0.105, 0.11), 0.74, regions=[FN, GL], material="nickel-plate"),
    S("headlamp-r", "chassis", "body", 3.2, ell(1.78, -0.55, 0.105, 0.11), 0.74, regions=[FN, GL], material="nickel-plate"),
    S("bumper-front", "chassis", "body", 3.4, rect(2.198, -0.7, 0.044, 1.4, 0.015), 0.8, region=BP, material="nickel-plate"),
    S("bumper-rear", "chassis", "body", 3.4, rect(-2.142, -0.7, 0.044, 1.4, 0.015), 0.8, region=BP, material="nickel-plate"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: tyres, frame, running boards, body, hood, top, windscreen, radiator, spare, fenders, headlamps and bumpers.",
                   "Vorder- und Draufsicht als gegliederte Massen: Reifen, Rahmen, Trittbretter, Aufbau, Haube, Verdeck, Windschutzscheibe, Kühler, Reserverad, Kotflügel, Scheinwerfer und Stoßstangen."),
    "merged": ("Lamp bowl and lens, both bumper bars and the hood with its louvres are single shapes.",
               "Lampentopf mit Linse, beide Stoßstangenbänder und die Haube mit ihren Lüftungsschlitzen sind jeweils eine Form."),
})

# ================================================================ 1:200
front = tyres_front(0.12, 0.0, False) + [
    S("frame", "chassis", "body", 1, rect(-0.4, 0.35, 0.8, 0.06), 0.2, regions=[BP, EX], material="exhaust-steel"),
    S("body", "chassis", "body", 2, poly([(-0.62, 0.52), (0.62, 0.52), (0.62, 1.14), (0.6, 1.58), (0.62, 1.7), (-0.62, 1.7), (-0.6, 1.58), (-0.62, 1.14)]), 0.6, regions=[BP, RF, IN], material="paint"),
    S("hood-radiator", "chassis", "body", 2.5, rect(-0.34, 0.56, 0.68, 0.7, 0.04), 0.62, regions=[HD, RD, EB], material="paint"),
    S("windscreen", "chassis", "glazing", 6, rect(-0.52, 1.21, 1.04, 0.39), 0.85, region=GL, material="glass"),
    S("spare-tyre", "chassis", "body", 3.6, rect(-0.615, 0.47, TW, 0.62, 0.05), 0.12, region=TY, material="rubber"),
    S("fender-l", "chassis", "body", 3, poly(FENDER_F), 0.5, regions=[FN, GL], material="paint"),
    S("fender-r", "chassis", "body", 3, poly([(-x, y) for x, y in FENDER_F]), 0.5, regions=[FN, GL], material="paint"),
    S("bumper-front", "chassis", "body", 3.4, rect(-0.7, 0.378, 1.4, 0.306), 0.8, region=BP, material="nickel-plate"),
]
plan = [
    S("frame-pair", "chassis", "body", 1, poly([(-1.98, -0.41), (2.08, -0.41), (2.08, 0.41), (-1.98, 0.41)]), 0.2, regions=[BP, EX], material="exhaust-steel"),
    S("running-board-l", "chassis", "body", 1.5, rect(-1.0, 0.62, 2.0, 0.18), 0.42, region=FN, material="paint"),
    S("running-board-r", "chassis", "body", 1.5, rect(-1.0, -0.8, 2.0, 0.18), 0.42, region=FN, material="paint"),
    S("tub", "chassis", "body", 2, poly(symy(TUB_HALF)), 0.6, regions=[BP, IN], material="paint"),
    S("hood", "chassis", "body", 2.3, rect(0.62, -0.34, 1.26, 0.68, 0.06), 0.62, regions=[HD, EB, RD], material="paint"),
    S("roof-slab", "chassis", "body", 2.4, poly(symy(ROOF_HALF)), 0.78, region=RF, material="fabric-roof"),
    S("windscreen", "chassis", "glazing", 6.1, rect(0.47, -0.52, 0.105, 1.04), 0.85, region=GL, material="glass"),
    S("spare-tyre", "chassis", "body", 3.6, rect(0.55, -0.615, 0.62, TW), 0.12, region=TY, material="rubber"),
    S("fender-fl", "chassis", "body", 3, poly(FENDER_PF), 0.5, regions=[FN, GL], material="paint"),
    S("fender-fr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PF]), 0.5, regions=[FN, GL], material="paint"),
    S("fender-rl", "chassis", "body", 3, poly(FENDER_PR), 0.5, region=FN, material="paint"),
    S("fender-rr", "chassis", "body", 3, poly([(x, -y) for x, y in FENDER_PR]), 0.5, region=FN, material="paint"),
    S("bumper-front", "chassis", "body", 3.4, rect(2.198, -0.7, 0.044, 1.4), 0.8, region=BP, material="nickel-plate"),
    S("bumper-rear", "chassis", "body", 3.4, rect(-2.142, -0.7, 0.044, 1.4), 0.8, region=BP, material="nickel-plate"),
    S("tail-pipe", "chassis", "body", 3.5, cap(-1.85, -0.35, -2.067, -0.35, 0.0375), 0.4, region=EX, material="exhaust-steel"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep main masses only: tyre blocks, frame, body with top, hood with radiator, windscreen, spare, fenders and bumpers.",
                   "Vorder- und Draufsicht behalten nur die Hauptmassen: Reifenblöcke, Rahmen, Aufbau mit Verdeck, Haube mit Kühler, Windschutzscheibe, Reserverad, Kotflügel und Stoßstangen."),
    "merged": ("Radiator, hood and engine bay are one shape; headlamps are part of the fenders; the louvres, hub caps and lamp stalks are dropped.",
               "Kühler, Haube und Motorraum sind eine Form; die Scheinwerfer gehören zu den Kotflügeln; Lüftungsschlitze, Radkappen und Lampenstiele entfallen."),
})

# ================================================================ 1:500
front = tyres_front(0.15, 0.0, False) + [
    S("underbody", "chassis", "body", 1, rect(-0.7, 0.378, 1.4, 0.3), 0.25, regions=[EX, BP], material="exhaust-steel"),
    S("silhouette", "chassis", "body", 2, poly([(-0.85, 0.5), (0.85, 0.5), (0.85, 0.85), (0.62, 0.9), (0.62, 1.6), (0.64, 1.7), (-0.64, 1.7), (-0.62, 1.6), (-0.62, 0.9), (-0.85, 0.85)]), 0.55, regions=[BP, HD, FN, RD, EB, RF], material="paint"),
    S("windows", "chassis", "glazing", 6, rect(-0.52, 1.22, 1.04, 0.33), 0.85, region=GL, material="glass"),
]
plan = [
    S("underbody", "chassis", "body", 1, rect(-2.145, -0.7, 4.39, 1.4), 0.25, region=EX, material="exhaust-steel"),
    S("silhouette", "chassis", "body", 2, poly(symy([(-2.03, 0.0), (-2.03, 0.6), (-2.0, 0.78), (-1.8, 0.85), (-1.1, 0.85), (-0.98, 0.62), (0.96, 0.62), (1.1, 0.85), (1.7, 0.85), (2.0, 0.8), (2.14, 0.65), (2.14, 0.0)])), 0.55, regions=[BP, HD, FN, RD, EB, RF], material="paint"),
    S("windows", "chassis", "glazing", 6, rect(0.47, -0.52, 0.105, 1.04), 0.85, region=GL, material="glass"),
    S("bumper-front", "chassis", "body", 3.4, rect(2.145, -0.7, 0.1, 1.4), 0.8, region=BP, material="nickel-plate"),
    S("bumper-rear", "chassis", "body", 3.4, rect(-2.145, -0.7, 0.1, 1.4), 0.8, region=BP, material="nickel-plate"),
]
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes: one body outline with fenders, a glass strip, tyre blocks and in plan the two bumpers; the wheels are hidden under the fenders in plan.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten: ein Aufbauumriss mit Kotflügeln, ein Glasstreifen, Reifenblöcke und oben die beiden Stoßstangen; die Räder liegen oben unter den Kotflügeln."),
    "thinElements": ("The bumper bars are widened to 0.1 m in plan; the front bumper bars are merged with the underbody band in front view.",
                     "Die Stoßstangen sind oben auf 0,1 m verbreitert; die Stoßstangenbänder sind vorn mit dem Unterbodenband zusammengefasst."),
})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
