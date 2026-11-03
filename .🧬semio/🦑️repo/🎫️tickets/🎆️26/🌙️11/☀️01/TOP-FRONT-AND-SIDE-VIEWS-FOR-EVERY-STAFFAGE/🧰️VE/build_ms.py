"""motor-scooter: front and plan views at 1:500, 1:200, 1:100, 1:50."""
from lib import *

AID = "motor-scooter"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

TY, BD, SA, HL, SE, EN, EX, BRK = "tyres", "body-panels", "steer-assembly", "headlamp", "seat", "engine", "exhaust-system", "brake-disc"


def sym(pts):
    """Half outline (x, y>0) from rear to front -> closed symmetric polygon."""
    return pts + [(x, -y) for x, y in reversed(pts)]


BODY_FRONT = [(-0.21, 0.26), (0.21, 0.26), (0.2, 0.5), (0.16, 0.9), (0.13, 1.0), (-0.13, 1.0), (-0.16, 0.9), (-0.2, 0.5)]
HALF_TOP = [(-0.945, 0.0), (-0.93, 0.1), (-0.88, 0.16), (-0.7, 0.19), (-0.4, 0.19), (-0.25, 0.2), (-0.05, 0.2), (0.2, 0.2), (0.35, 0.17), (0.5, 0.12), (0.56, 0.08)]
BODY_TOP = sym(HALF_TOP)

# ================================================================ 1:50
front = [
    S("tyre-r", "wheel-r", "body", 1.5, rect(-0.06, 0, 0.12, 0.46, 0.045), 0.1, region=TY, material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_FRONT), 0.55, region=BD, material="paint"),
    S("stand-leg-l", "stand", "body", 2.1, rect(0.09, 0.3, 0.02, 0.012, 0.004, True), 0.4, region=BD, material="paint"),
    S("stand-leg-r", "stand", "body", 2.1, rect(-0.11, 0.3, 0.02, 0.012, 0.004, True), 0.4, region=BD, material="paint"),
    S("fork-l", "steer", "body", 3, rect(0.062, 0.23, 0.032, 0.77, 0.01), 0.4, region=SA, material="paint"),
    S("fork-r", "steer", "body", 3, rect(-0.094, 0.23, 0.032, 0.77, 0.01), 0.4, region=SA, material="paint"),
    S("cowl", "steer", "body", 3.6, rect(-0.17, 0.93, 0.34, 0.22, 0.07), 0.5, region=SA, material="paint"),
    S("headlamp", "steer", "body", 3.8, rect(-0.065, 0.995, 0.13, 0.09, 0.04), 0.9, region=HL, material="glass"),
    S("handlebar", "steer", "body", 3.7, rect(-0.37, 1.045, 0.74, 0.035, 0.015), 0.4, region=SA, material="paint"),
    S("grip-l", "steer", "body", 3.9, rect(0.29, 1.03, 0.08, 0.065, 0.028), 0.1, region=SA, material="rubber"),
    S("grip-r", "steer", "body", 3.9, rect(-0.37, 1.03, 0.08, 0.065, 0.028), 0.1, region=SA, material="rubber"),
    S("fender-f", "steer", "body", 3.5, rect(-0.065, 0.314, 0.13, 0.191, 0.04), 0.55, region=BD, material="paint"),
    S("tyre-f", "wheel-f", "body", 4, rect(-0.05, 0, 0.1, 0.46, 0.04), 0.1, region=TY, material="rubber"),
    S("hub-f", "wheel-f", "body", 4.2, rect(-0.075, 0.185, 0.15, 0.09, 0.01), 0.7, region=BRK, material="alloy"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 1.5, rect(-0.9, -0.06, 0.46, 0.12, 0.03), 0.1, region=TY, material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_TOP), 0.55, region=BD, material="paint"),
    S("seat-l", "seat", "body", 2.4, cap(-0.22, 0.1, -0.78, 0.1, 0.055, follow=True), 0.1, region=SE, material="seat-vinyl"),
    S("seat-c", "seat", "body", 2.4, cap(-0.22, 0.0, -0.78, 0.0, 0.055, follow=True), 0.1, region=SE, material="seat-vinyl"),
    S("seat-r", "seat", "body", 2.4, cap(-0.22, -0.1, -0.78, -0.1, 0.055, follow=True), 0.1, region=SE, material="seat-vinyl"),
    S("muffler", "swing", "body", 2.5, cap(-0.89, -0.15, -0.5, -0.15, 0.055), 0.4, region=EX, material="exhaust-steel"),
    S("tail-pipe", "swing", "body", 2.5, cap(-0.958, -0.15, -0.818, -0.15, 0.022), 0.45, region=EX, material="exhaust-steel"),
    S("engine-cover", "swing", "body", 2.4, rect(-0.54, -0.215, 0.22, 0.06, 0.02), 0.35, region=EN, material="engine-metal"),
    S("tail-lamp", "frame", "body", 2.7, rect(-0.955, -0.07, 0.03, 0.14, 0.008), 0.3, region=HL, material="glass"),
    S("stand-leg-l", "stand", "body", 1.2, cap(-0.1, 0.11, -0.435, 0.11, 0.012, follow=True), 0.4, region=BD, material="paint"),
    S("stand-leg-r", "stand", "body", 1.2, cap(-0.1, -0.11, -0.435, -0.11, 0.012, follow=True), 0.4, region=BD, material="paint"),
    S("fork-l", "steer", "body", 1.4, cap(0.42, 0.085, 0.67, 0.085, 0.014), 0.4, region=SA, material="paint"),
    S("fork-r", "steer", "body", 1.4, cap(0.42, -0.085, 0.67, -0.085, 0.014), 0.4, region=SA, material="paint"),
    S("tyre-f", "wheel-f", "body", 1.5, rect(0.44, -0.05, 0.46, 0.1, 0.03), 0.1, region=TY, material="rubber"),
    S("fender-f", "steer", "body", 2.3, rect(0.412, -0.065, 0.483, 0.13, 0.03), 0.55, region=BD, material="paint"),
    S("cowl", "steer", "body", 2.6, rect(0.33, -0.18, 0.22, 0.36, 0.06), 0.5, region=SA, material="paint"),
    S("headlamp", "steer", "body", 2.8, rect(0.43, -0.065, 0.06, 0.13, 0.02), 0.9, region=HL, material="glass"),
    S("handlebar", "steer", "body", 2.7, cap(0.4, -0.3, 0.4, 0.3, 0.018), 0.4, region=SA, material="paint"),
    S("grip-l", "steer", "body", 2.9, cap(0.4, 0.31, 0.4, 0.34, 0.03), 0.1, region=SA, material="rubber"),
    S("grip-r", "steer", "body", 2.9, cap(0.4, -0.31, 0.4, -0.34, 0.03), 0.1, region=SA, material="rubber"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: front tyre, fender, fork legs, hub, leg shield, cowl with headlamp, handlebar with grips and stand legs, rear tyre peeking beside; top: tyres, body with seat, exhaust, engine cover, cowl, fork, fender, handlebar and stand.",
                   "Vorn: Vorderreifen, Schutzblech, Gabelholme, Nabe, Beinschild, Verkleidung mit Scheinwerfer, Lenker mit Griffen und Ständerbeine, daneben der Hinterreifen; oben: Reifen, Karosserie mit Sitzbank, Auspuff, Motorabdeckung, Verkleidung, Gabel, Schutzblech, Lenker und Ständer."),
    "overlays": ("Front and top shapes carry the regions of the side view. The seat is drawn as three followTip capsules so it shortens when it opens; the stand legs are followTip rects (front) and capsules (top). Seat, engine and tail lamp are hidden behind the leg shield in the front view, the rear tyre is covered by the bodywork in plan.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Die Sitzbank besteht aus drei followTip-Kapseln und verkürzt sich beim Öffnen; die Ständerbeine sind followTip-Rechtecke (vorn) und -Kapseln (oben). Sitzbank, Motor und Rücklicht liegen vorn hinter dem Beinschild, der Hinterreifen liegt oben unter der Karosserie."),
})

# ================================================================ 1:100
front = [
    S("tyre-r", "wheel-r", "body", 1.5, rect(-0.06, 0, 0.12, 0.46, 0.045), 0.15, region=TY, material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_FRONT), 0.55, region=BD, material="paint"),
    S("stand-leg-l", "stand", "body", 2.1, rect(0.09, 0.3, 0.02, 0.012, None, True), 0.4, region=BD, material="paint"),
    S("stand-leg-r", "stand", "body", 2.1, rect(-0.11, 0.3, 0.02, 0.012, None, True), 0.4, region=BD, material="paint"),
    S("fork-l", "steer", "body", 3, rect(0.062, 0.23, 0.032, 0.77), 0.4, region=SA, material="paint"),
    S("fork-r", "steer", "body", 3, rect(-0.094, 0.23, 0.032, 0.77), 0.4, region=SA, material="paint"),
    S("cowl", "steer", "body", 3.6, rect(-0.17, 0.93, 0.34, 0.22, 0.07), 0.5, regions=[SA], material="paint"),
    S("headlamp", "steer", "body", 3.8, rect(-0.065, 0.995, 0.13, 0.09, 0.04), 0.9, region=HL, material="glass"),
    S("handlebar", "steer", "body", 3.7, rect(-0.37, 1.03, 0.74, 0.065, 0.028), 0.3, region=SA, material="paint"),
    S("fender-f", "steer", "body", 3.5, rect(-0.065, 0.314, 0.13, 0.191, 0.04), 0.55, region=BD, material="paint"),
    S("tyre-f", "wheel-f", "body", 4, rect(-0.05, 0, 0.1, 0.46, 0.04), 0.15, regions=[TY, BRK], material="rubber"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 1.5, rect(-0.9, -0.06, 0.46, 0.12), 0.15, regions=[TY, BRK], material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_TOP), 0.55, region=BD, material="paint"),
    S("seat-l", "seat", "body", 2.4, cap(-0.22, 0.07, -0.78, 0.07, 0.085, follow=True), 0.1, region=SE, material="seat-vinyl"),
    S("seat-r", "seat", "body", 2.4, cap(-0.22, -0.07, -0.78, -0.07, 0.085, follow=True), 0.1, region=SE, material="seat-vinyl"),
    S("muffler", "swing", "body", 2.5, cap(-0.89, -0.15, -0.5, -0.15, 0.055), 0.4, regions=[EX, EN], material="exhaust-steel"),
    S("stand-leg-l", "stand", "body", 1.2, cap(-0.1, 0.11, -0.435, 0.11, 0.012, follow=True), 0.4, region=BD, material="paint"),
    S("stand-leg-r", "stand", "body", 1.2, cap(-0.1, -0.11, -0.435, -0.11, 0.012, follow=True), 0.4, region=BD, material="paint"),
    S("fork", "steer", "body", 1.4, poly([(0.42, 0.1), (0.67, 0.1), (0.67, 0.07), (0.45, 0.07), (0.45, -0.07), (0.67, -0.07), (0.67, -0.1), (0.42, -0.1)]), 0.4, region=SA, material="paint"),
    S("tyre-f", "wheel-f", "body", 1.5, rect(0.44, -0.05, 0.46, 0.1), 0.15, regions=[TY, BRK], material="rubber"),
    S("fender-f", "steer", "body", 2.3, rect(0.412, -0.065, 0.483, 0.13, 0.03), 0.55, region=BD, material="paint"),
    S("cowl", "steer", "body", 2.6, rect(0.33, -0.18, 0.22, 0.36, 0.06), 0.5, region=SA, material="paint"),
    S("headlamp", "steer", "body", 2.8, rect(0.43, -0.065, 0.06, 0.13, 0.02), 0.9, region=HL, material="glass"),
    S("handlebar", "steer", "body", 2.7, cap(0.4, -0.34, 0.4, 0.34, 0.03), 0.3, region=SA, material="paint"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top as articulated masses: tyres, fender, fork, leg shield or body, seat, exhaust with engine, cowl with headlamp, handlebar and stand.",
                   "Vorder- und Draufsicht als gegliederte Massen: Reifen, Schutzblech, Gabel, Beinschild beziehungsweise Karosserie, Sitzbank, Auspuff mit Motor, Verkleidung mit Scheinwerfer, Lenker und Ständer."),
    "merged": ("Grips and handlebar tube, hub and tyre, muffler with engine cover and tail pipe, and tail lamp with the body are merged in front and top.",
               "Griffe und Lenkerrohr, Nabe und Reifen, Auspuff mit Motorabdeckung und Endrohr sowie Rücklicht und Karosserie sind vorn und oben zusammengefasst."),
})

# ================================================================ 1:200
front = [
    S("wheel-r", "wheel-r", "body", 1.5, rect(-0.06, 0, 0.12, 0.46, 0.04), 0.2, regions=[TY, BRK], material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_FRONT), 0.55, regions=[BD, EN, EX, SE], material="paint"),
    S("fork", "steer", "body", 3, rect(-0.094, 0.23, 0.188, 0.77), 0.3, region=SA, material="paint"),
    S("cowl-bar", "steer", "body", 3.6, poly([(-0.37, 1.03), (-0.17, 1.03), (-0.17, 0.93), (0.17, 0.93), (0.17, 1.03), (0.37, 1.03), (0.37, 1.095), (0.17, 1.095), (0.17, 1.15), (-0.17, 1.15), (-0.17, 1.095), (-0.37, 1.095)]), 0.5, regions=[SA, HL], material="paint"),
    S("wheel-f", "wheel-f", "body", 4, rect(-0.065, 0, 0.13, 0.505, 0.04), 0.2, regions=[TY, BRK, BD], material="rubber"),
    S("stand", "stand", "body", 2.1, rect(-0.11, 0.3, 0.22, 0.012, None, True), 0.4, region=BD, material="paint"),
]
plan = [
    S("wheel-r", "wheel-r", "body", 1.5, rect(-0.9, -0.06, 0.46, 0.12), 0.2, regions=[TY, BRK], material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_TOP), 0.55, regions=[BD, EN, EX], material="paint"),
    S("seat", "seat", "body", 2.4, cap(-0.22, 0.0, -0.8, 0.0, 0.15, 0.12, True), 0.1, region=SE, material="seat-vinyl"),
    S("stand-l", "stand", "body", 1.2, cap(-0.1, 0.11, -0.435, 0.11, 0.014, follow=True), 0.4, region=BD, material="paint"),
    S("stand-r", "stand", "body", 1.2, cap(-0.1, -0.11, -0.435, -0.11, 0.014, follow=True), 0.4, region=BD, material="paint"),
    S("fork", "steer", "body", 1.4, rect(0.42, -0.1, 0.25, 0.2), 0.3, region=SA, material="paint"),
    S("wheel-f", "wheel-f", "body", 1.5, rect(0.44, -0.065, 0.46, 0.13, 0.03), 0.2, regions=[TY, BRK, BD], material="rubber"),
    S("cowl", "steer", "body", 2.6, rect(0.33, -0.18, 0.22, 0.36, 0.06), 0.5, regions=[SA, HL], material="paint"),
    S("handlebar", "steer", "body", 2.7, cap(0.4, -0.34, 0.4, 0.34, 0.03), 0.3, region=SA, material="paint"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top keep the main masses: two wheels, body with seat, fork, cowl with handlebar and the stand.",
                   "Vorder- und Draufsicht behalten die Hauptmassen: zwei Räder, Karosserie mit Sitzbank, Gabel, Verkleidung mit Lenker und den Ständer."),
    "merged": ("Fender and hub are part of the wheel shapes; engine, exhaust and tail lamp are part of the body; the headlamp is part of the cowl.",
               "Schutzblech und Nabe gehören zu den Radformen; Motor, Auspuff und Rücklicht zur Karosserie; der Scheinwerfer zur Verkleidung."),
})

# ================================================================ 1:500
front = [
    S("wheel-r", "wheel-r", "body", 1.5, rect(-0.06, 0, 0.12, 0.46, 0.04), 0.25, regions=[TY, BRK], material="rubber"),
    S("body", "frame", "body", 2, poly(BODY_FRONT), 0.55, regions=[BD, EN, EX, SE], material="paint"),
    S("cowl-bar", "steer", "body", 3.6, poly([(-0.37, 1.03), (-0.17, 1.03), (-0.17, 0.93), (0.17, 0.93), (0.17, 1.03), (0.37, 1.03), (0.37, 1.095), (0.17, 1.095), (0.17, 1.15), (-0.17, 1.15), (-0.17, 1.095), (-0.37, 1.095)]), 0.5, regions=[SA, HL, BD], material="paint"),
    S("wheel-f", "wheel-f", "body", 4, rect(-0.065, 0, 0.13, 0.46, 0.04), 0.25, regions=[TY, BRK], material="rubber"),
]
plan = [
    S("body", "frame", "body", 2, poly(BODY_TOP), 0.55, regions=[BD, EN, EX, HL], material="paint"),
    S("seat", "seat", "body", 2.4, cap(-0.22, 0.0, -0.8, 0.0, 0.15, 0.12, True), 0.15, region=SE, material="seat-vinyl"),
    S("wheel-f", "wheel-f", "body", 1.5, rect(0.44, -0.065, 0.46, 0.13, 0.03), 0.25, regions=[TY, BRK, BD], material="rubber"),
    S("cowl", "steer", "body", 2.6, poly([(0.33, -0.18), (0.55, -0.18), (0.55, -0.034), (0.45, -0.034), (0.45, 0.034), (0.55, 0.034), (0.55, 0.18), (0.33, 0.18)]), 0.5, regions=[SA, HL], material="paint"),
    S("handlebar", "steer", "body", 2.7, cap(0.4, -0.34, 0.4, 0.34, 0.035), 0.3, region=SA, material="paint"),
    S("wheel-r", "wheel-r", "body", 1.5, rect(-0.9, -0.06, 0.46, 0.12), 0.25, regions=[TY, BRK], material="rubber"),
]
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top are composed silhouettes of a few masses: wheels, body with seat, cowl and handlebar; the stand is omitted.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten aus wenigen Massen: Räder, Karosserie mit Sitzbank, Verkleidung und Lenker; der Ständer entfällt."),
})

STD = cap(0, 0, 0.335, 0, 0.012)
LEGF = {"stand-leg-l": ("stand", ext(STD, 0.095, 0.125)), "stand-leg-r": ("stand", ext(STD, -0.125, -0.095))}


def seat(z, r=0.05):
    return ("seat", ext(rect(0, -0.12, 0.58, 0.195, r), -z, z))


apply_rules(a, {
    "50": {"front": LEGF, "plan": dict(LEGF, **{"seat-l": seat(0.155), "seat-c": None, "seat-r": None})},
    "100": {"front": LEGF, "plan": dict(LEGF, **{"seat-l": seat(0.16), "seat-r": None})},
    "200": {"front": {"stand": ("stand", ext(STD, -0.125, 0.125))}, "plan": {"stand-l": LEGF["stand-leg-l"], "stand-r": LEGF["stand-leg-r"], "seat": seat(0.15, 0.06)}},
    "500": {"plan": {"seat": seat(0.15, 0.06)}},
})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
