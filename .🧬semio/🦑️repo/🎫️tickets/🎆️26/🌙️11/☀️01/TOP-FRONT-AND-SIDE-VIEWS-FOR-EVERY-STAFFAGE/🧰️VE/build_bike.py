"""city-bicycle: front and plan views at 1:200, 1:100, 1:50 (1:500 unavailable). Adds view bones bars, fork-leg, arm-n, arm-f."""
from lib import *

AID = "city-bicycle"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)




TY = ["tyres", "rims-spokes"]
FT, ST, MG, DR, HB, LP = "frame-tubes", "steer-assembly", "mudguards", "drivetrain", "hub-brakes", "lamps"


def lr(f):
    return [f(1, "l"), f(-1, "r")]


# ================================================================ 1:50
front = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.02, 0, 0.04, 0.69, 0.018), 0.1, regions=TY, material="rubber"),
    S("hub-r", "wheel-r", "detail", 4.8, rect(-0.05, 0.335, 0.1, 0.02, 0.004), 0.7, region=HB, material="chrome"),
    S("fender-r", "frame", "body", 3.5, rect(-0.032, 0.6, 0.064, 0.101, 0.01), 0.55, region=MG, material="paint"),
    S("rack", "frame", "body", 3, rect(-0.09, 0.795, 0.18, 0.02, 0.004), 0.3, region=FT, material="paint"),
    S("tail-lamp", "frame", "body", 5, rect(-0.03, 0.17, 0.06, 0.06, 0.006), 0.3, region=LP, material="plastic"),
    S("down-tube", "frame", "body", 2.8, rect(-0.02, 0.3, 0.04, 0.5, 0.008), 0.3, region=FT, material="paint"),
    S("seat-tube", "frame", "body", 2.9, rect(-0.0125, 0.3, 0.025, 0.69, 0.006), 0.3, region=FT, material="paint"),
    S("bb-shell", "frame", "body", 3, rect(-0.04, 0.285, 0.08, 0.03, 0.006), 0.3, region=FT, material="paint"),
    S("chaincase", "frame", "body", 3.6, rect(-0.095, 0.225, 0.05, 0.16, 0.012), 0.6, region=DR, material="paint"),
    S("saddle", "frame", "body", 5, rect(-0.085, 0.985, 0.17, 0.055, 0.022), 0.2, region="saddle", material="saddle-skin"),
    S("tyre-f", "wheel-f", "body", 4, rect(-0.02, 0, 0.04, 0.69, 0.018), 0.1, regions=TY, material="rubber"),
    S("fender-f", "wheel-f", "body", 4.5, rect(-0.032, 0.195, 0.064, 0.506, 0.01), 0.55, region=MG, material="paint"),
    S("hub-f", "wheel-f", "detail", 4.8, rect(-0.05, 0.335, 0.1, 0.02, 0.004), 0.7, region=HB, material="chrome"),
    S("fork-l", "fork-leg", "body", 3.0, rect(0.038, 0.345, 0.014, 0.455, 0.005, True), 0.3, region=ST, material="paint"),
    S("fork-r", "fork-leg", "body", 3.0, rect(-0.052, 0.345, 0.014, 0.455, 0.005, True), 0.3, region=ST, material="paint"),
    S("fork-crown", "steer", "body", 3.1, rect(-0.052, 0.78, 0.104, 0.02, 0.005), 0.3, region=ST, material="paint"),
    S("head-tube", "frame", "body", 3.2, rect(-0.0225, 0.8, 0.045, 0.132, 0.008), 0.3, region=FT, material="paint"),
    S("stem-low", "steer", "body", 3.1, rect(-0.0125, 0.932, 0.025, 0.08, 0.004), 0.3, region=ST, material="paint"),
    S("stem-high", "bars", "body", 3.2, rect(-0.02, 1.0, 0.04, 0.088, 0.006), 0.3, region=ST, material="paint"),
    S("handlebar", "bars", "body", 3.3, rect(-0.31, 1.045, 0.62, 0.025, 0.011), 0.3, region=ST, material="paint"),
    S("grip-l", "bars", "body", 3.4, rect(0.22, 1.035, 0.09, 0.03, 0.014), 0.1, region=ST, material="rubber"),
    S("grip-r", "bars", "body", 3.4, rect(-0.31, 1.035, 0.09, 0.03, 0.014), 0.1, region=ST, material="rubber"),
    S("head-lamp", "steer", "body", 5, rect(-0.045, 0.905, 0.09, 0.07, 0.03), 0.85, region=LP, material="plastic"),
    S("arm-n", "arm-n", "detail", 6, rect(-0.088, 0.288, 0.02, 0.024, 0.003, True), 0.3, region=DR, material="paint"),
    S("arm-f", "arm-f", "detail", 2, rect(0.068, 0.288, 0.02, 0.024, 0.003, True), 0.3, region=DR, material="paint"),
    S("pedal-n", "pedal-n", "detail", 6.5, rect(-0.185, 0.287, 0.1, 0.025, 0.004), 0.2, region=DR, material="chrome"),
    S("pedal-f", "pedal-f", "detail", 1.5, rect(0.085, 0.287, 0.1, 0.025, 0.004), 0.2, region=DR, material="chrome"),
    S("stand-l", "stand", "detail", 3, rect(0.068, 0.318, 0.014, 0.012, 0.002, True), 0.3, region=FT, material="paint"),
    S("stand-r", "stand", "detail", 3, rect(-0.082, 0.318, 0.014, 0.012, 0.002, True), 0.3, region=FT, material="paint"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.925, -0.02, 0.69, 0.04, 0.01), 0.1, regions=TY, material="rubber"),
    S("hub-r", "wheel-r", "detail", 4.8, rect(-0.6, -0.05, 0.04, 0.1, 0.004), 0.7, region=HB, material="chrome"),
    S("fender-r", "frame", "body", 4.5, rect(-0.936, -0.03, 0.679, 0.06, 0.015), 0.55, region=MG, material="paint"),
    S("rack", "frame", "body", 3, rect(-0.84, -0.09, 0.54, 0.18, 0.006), 0.3, region=FT, material="paint"),
    S("tail-lamp", "frame", "body", 5, rect(-0.96, -0.03, 0.045, 0.06, 0.006), 0.3, region=LP, material="plastic"),
    S("chainstay-l", "frame", "body", 3, cap(-0.1, 0.035, -0.58, 0.055, 0.01), 0.3, region=FT, material="paint"),
    S("chainstay-r", "frame", "body", 3, cap(-0.1, -0.035, -0.58, -0.055, 0.01), 0.3, region=FT, material="paint"),
    S("seatstay-l", "frame", "body", 3.05, cap(-0.285, 0.02, -0.58, 0.055, 0.008), 0.3, region=FT, material="paint"),
    S("seatstay-r", "frame", "body", 3.05, cap(-0.285, -0.02, -0.58, -0.055, 0.008), 0.3, region=FT, material="paint"),
    S("seat-tube", "frame", "body", 3.1, cap(-0.348, 0, -0.1, 0, 0.016), 0.3, region=FT, material="paint"),
    S("down-tube", "frame", "body", 3.1, cap(-0.1, 0, 0.347, 0, 0.02), 0.3, region=FT, material="paint"),
    S("head-tube", "frame", "body", 3.2, rect(0.299, -0.025, 0.048, 0.05, 0.008), 0.3, region=FT, material="paint"),
    S("chaincase", "frame", "body", 3.6, rect(-0.58, -0.1, 0.555, 0.06, 0.02), 0.6, region=DR, material="paint"),
    S("saddle", "frame", "body", 5, poly([(-0.49, -0.075), (-0.35, -0.085), (-0.25, -0.04), (-0.19, -0.02), (-0.19, 0.02), (-0.25, 0.04), (-0.35, 0.085), (-0.49, 0.075)]), 0.2, region="saddle", material="saddle-skin"),
    S("tyre-f", "wheel-f", "body", 4, rect(0.215, -0.02, 0.69, 0.04, 0.01), 0.1, regions=TY, material="rubber"),
    S("hub-f", "wheel-f", "detail", 4.8, rect(0.54, -0.05, 0.04, 0.1, 0.004), 0.7, region=HB, material="chrome"),
    S("fender-f", "wheel-f", "body", 4.5, rect(0.252, -0.03, 0.663, 0.06, 0.015), 0.55, region=MG, material="paint"),
    S("fork-l", "fork-leg", "body", 3.0, cap(0.347, 0.045, 0.56, 0.045, 0.007, follow=True), 0.3, region=ST, material="paint"),
    S("fork-r", "fork-leg", "body", 3.0, cap(0.347, -0.045, 0.56, -0.045, 0.007, follow=True), 0.3, region=ST, material="paint"),
    S("fork-crown", "steer", "body", 3.1, rect(0.33, -0.052, 0.03, 0.104, 0.006), 0.3, region=ST, material="paint"),
    S("stem-head", "steer", "body", 3.1, cap(0.299, 0, 0.17, 0, 0.0125), 0.3, region=ST, material="paint"),
    S("stem-bar", "bars", "body", 3.2, cap(0.04, 0, 0.2, 0, 0.0125), 0.3, region=ST, material="paint"),
    S("handlebar-l", "bars", "body", 3.3, poly([(0.04, 0.0), (0.04, 0.18), (-0.02, 0.26)], False), 0.3, filled=False, region=ST, material="paint", pen=0.5),
    S("handlebar-r", "bars", "body", 3.3, poly([(0.04, 0.0), (0.04, -0.18), (-0.02, -0.26)], False), 0.3, filled=False, region=ST, material="paint", pen=0.5),
    S("grip-l", "bars", "body", 3.4, cap(-0.02, 0.26, -0.03, 0.292, 0.018), 0.1, region=ST, material="rubber"),
    S("grip-r", "bars", "body", 3.4, cap(-0.02, -0.26, -0.03, -0.292, 0.018), 0.1, region=ST, material="rubber"),
    S("head-lamp", "steer", "body", 5, rect(0.4, -0.04, 0.09, 0.08, 0.03), 0.85, region=LP, material="plastic"),
    S("arm-n", "arm-n", "detail", 6, cap(-0.1, -0.07, 0.07, -0.07, 0.011, follow=True), 0.3, region=DR, material="paint"),
    S("arm-f", "arm-f", "detail", 2, cap(-0.1, 0.07, -0.27, 0.07, 0.011, follow=True), 0.3, region=DR, material="paint"),
    S("pedal-n", "pedal-n", "detail", 6.5, rect(0.02, -0.165, 0.1, 0.09, 0.004), 0.2, region=DR, material="chrome"),
    S("pedal-f", "pedal-f", "detail", 1.5, rect(-0.32, 0.075, 0.1, 0.09, 0.004), 0.2, region=DR, material="chrome"),
    S("stand-l", "stand", "detail", 3, cap(-0.42, 0.075, -0.1, 0.075, 0.008, follow=True), 0.3, region=FT, material="paint"),
    S("stand-r", "stand", "detail", 3, cap(-0.42, -0.075, -0.1, -0.075, 0.008, follow=True), 0.3, region=FT, material="paint"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: two tyres behind each other, fork, head tube, stem, swept handlebar with grips, saddle, rack, fenders, lamps, chaincase, cranks, pedals and stand legs; top: wheels, fenders, frame spine, stays, fork, handlebar, saddle, rack, lamps, chaincase, cranks, pedals and stand.",
                   "Vorn: zwei hintereinanderliegende Reifen, Gabel, Steuerrohr, Vorbau, geschwungener Lenker mit Griffen, Sattel, Gepäckträger, Schutzbleche, Leuchten, Kettenkasten, Kurbeln, Pedale und Ständerbeine; oben: Räder, Schutzbleche, Rahmenrückgrat, Streben, Gabel, Lenker, Sattel, Gepäckträger, Leuchten, Kettenkasten, Kurbeln, Pedale und Ständer."),
    "overlays": ("Front and top shapes carry the regions of the side view. View bones fork-leg, bars, arm-n and arm-f (fixed, no mass) let followTip stretch the fork legs, crank arms and stand legs and move the handlebar with the steering, braking and pedalling poses.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Die Ansichtsknochen fork-leg, bars, arm-n und arm-f (fest, ohne Masse) lassen followTip Gabelholme, Kurbelarme und Ständerbeine strecken und den Lenker beim Lenken, Bremsen und Treten mitbewegen."),
})

# ================================================================ 1:100
front = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.02, 0, 0.04, 0.69, 0.018), 0.15, regions=TY, material="rubber"),
    S("fender-r", "frame", "body", 3.5, rect(-0.032, 0.6, 0.064, 0.101, 0.01), 0.55, region=MG, material="paint"),
    S("rack", "frame", "body", 3, rect(-0.09, 0.795, 0.18, 0.02), 0.3, region=FT, material="paint"),
    S("tail-lamp", "frame", "body", 5, rect(-0.03, 0.17, 0.06, 0.06), 0.3, region=LP, material="plastic"),
    S("frame-column", "frame", "body", 2.9, rect(-0.02, 0.3, 0.04, 0.69, 0.008), 0.3, region=FT, material="paint"),
    S("chaincase", "frame", "body", 3.6, rect(-0.095, 0.225, 0.05, 0.16, 0.012), 0.6, region=DR, material="paint"),
    S("saddle", "frame", "body", 5, rect(-0.085, 0.985, 0.17, 0.055, 0.022), 0.2, region="saddle", material="saddle-skin"),
    S("tyre-f", "wheel-f", "body", 4, rect(-0.02, 0, 0.04, 0.69, 0.018), 0.15, regions=TY, material="rubber"),
    S("fender-f", "wheel-f", "body", 4.5, rect(-0.032, 0.195, 0.064, 0.506, 0.01), 0.55, region=MG, material="paint"),
    S("hubs", "wheel-f", "detail", 4.8, rect(-0.05, 0.335, 0.1, 0.02), 0.7, regions=[HB], material="chrome"),
    S("fork-l", "fork-leg", "body", 3.0, rect(0.038, 0.345, 0.014, 0.455, None, True), 0.3, region=ST, material="paint"),
    S("fork-r", "fork-leg", "body", 3.0, rect(-0.052, 0.345, 0.014, 0.455, None, True), 0.3, region=ST, material="paint"),
    S("head-tube", "frame", "body", 3.2, rect(-0.0225, 0.8, 0.045, 0.132), 0.3, region=FT, material="paint"),
    S("stem", "bars", "body", 3.2, rect(-0.0125, 0.932, 0.025, 0.156), 0.3, region=ST, material="paint"),
    S("handlebar", "bars", "body", 3.3, rect(-0.31, 1.04, 0.62, 0.03, 0.012), 0.2, region=ST, material="paint"),
    S("head-lamp", "steer", "body", 5, rect(-0.045, 0.905, 0.09, 0.07, 0.03), 0.85, region=LP, material="plastic"),
    S("pedal-n", "pedal-n", "detail", 6.5, rect(-0.185, 0.287, 0.1, 0.025), 0.2, region=DR, material="chrome"),
    S("pedal-f", "pedal-f", "detail", 1.5, rect(0.085, 0.287, 0.1, 0.025), 0.2, region=DR, material="chrome"),
    S("stand-l", "stand", "detail", 3, rect(0.066, 0.318, 0.016, 0.012, None, True), 0.3, region=FT, material="paint"),
    S("stand-r", "stand", "detail", 3, rect(-0.082, 0.318, 0.016, 0.012, None, True), 0.3, region=FT, material="paint"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 4, rect(-0.925, -0.02, 0.69, 0.04), 0.15, regions=TY + [HB], material="rubber"),
    S("fender-r", "frame", "body", 4.5, rect(-0.936, -0.03, 0.679, 0.06, 0.015), 0.55, region=MG, material="paint"),
    S("rack", "frame", "body", 3, rect(-0.84, -0.09, 0.54, 0.18), 0.3, region=FT, material="paint"),
    S("tail-lamp", "frame", "body", 5, rect(-0.96, -0.03, 0.045, 0.06), 0.3, region=LP, material="plastic"),
    S("chainstays", "frame", "body", 3, poly([(-0.1, 0.035), (-0.58, 0.055), (-0.58, -0.055), (-0.1, -0.035)]), 0.3, region=FT, material="paint"),
    S("spine", "frame", "body", 3.1, cap(-0.348, 0, 0.347, 0, 0.018), 0.3, region=FT, material="paint"),
    S("chaincase", "frame", "body", 3.6, rect(-0.58, -0.1, 0.555, 0.06, 0.02), 0.6, region=DR, material="paint"),
    S("saddle", "frame", "body", 5, poly([(-0.49, -0.075), (-0.35, -0.085), (-0.25, -0.04), (-0.19, -0.02), (-0.19, 0.02), (-0.25, 0.04), (-0.35, 0.085), (-0.49, 0.075)]), 0.2, region="saddle", material="saddle-skin"),
    S("tyre-f", "wheel-f", "body", 4, rect(0.215, -0.02, 0.69, 0.04), 0.15, regions=TY + [HB], material="rubber"),
    S("fender-f", "wheel-f", "body", 4.5, rect(0.252, -0.03, 0.655, 0.06, 0.015), 0.55, region=MG, material="paint"),
    S("fork-l", "fork-leg", "body", 3.0, cap(0.347, 0.045, 0.56, 0.045, 0.007, follow=True), 0.3, region=ST, material="paint"),
    S("fork-r", "fork-leg", "body", 3.0, cap(0.347, -0.045, 0.56, -0.045, 0.007, follow=True), 0.3, region=ST, material="paint"),
    S("stem-head", "steer", "body", 3.1, cap(0.299, 0, 0.17, 0, 0.0125), 0.3, region=ST, material="paint"),
    S("stem-bar", "bars", "body", 3.2, cap(0.04, 0, 0.2, 0, 0.0125), 0.3, region=ST, material="paint"),
    S("handlebar-l", "bars", "body", 3.3, cap(0.04, 0.18, -0.02, 0.26, 0.0125), 0.3, region=ST, material="paint"),
    S("handlebar-r", "bars", "body", 3.3, cap(0.04, -0.18, -0.02, -0.26, 0.0125), 0.3, region=ST, material="paint"),
    S("crossbar", "bars", "body", 3.3, cap(0.04, -0.18, 0.04, 0.18, 0.0125), 0.3, region=ST, material="paint"),
    S("grip-l", "bars", "body", 3.4, cap(-0.02, 0.26, -0.03, 0.292, 0.018), 0.1, region=ST, material="rubber"),
    S("grip-r", "bars", "body", 3.4, cap(-0.02, -0.26, -0.03, -0.292, 0.018), 0.1, region=ST, material="rubber"),
    S("head-lamp", "steer", "body", 5, rect(0.4, -0.04, 0.09, 0.08, 0.03), 0.85, region=LP, material="plastic"),
    S("arm-n", "arm-n", "detail", 6, cap(-0.1, -0.07, 0.07, -0.07, 0.011, follow=True), 0.3, region=DR, material="paint"),
    S("arm-f", "arm-f", "detail", 2, cap(-0.1, 0.07, -0.27, 0.07, 0.011, follow=True), 0.3, region=DR, material="paint"),
    S("pedal-n", "pedal-n", "detail", 6.5, rect(0.02, -0.165, 0.1, 0.09), 0.2, region=DR, material="chrome"),
    S("pedal-f", "pedal-f", "detail", 1.5, rect(-0.32, 0.075, 0.1, 0.09), 0.2, region=DR, material="chrome"),
    S("stand-l", "stand", "detail", 3, cap(-0.42, 0.075, -0.1, 0.075, 0.008, follow=True), 0.3, region=FT, material="paint"),
    S("stand-r", "stand", "detail", 3, cap(-0.42, -0.075, -0.1, -0.075, 0.008, follow=True), 0.3, region=FT, material="paint"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top drawn as articulated masses: tyres, fenders, frame column or spine, fork, handlebar, saddle, rack, lamps, chaincase, pedals and stand.",
                   "Vorder- und Draufsicht als gegliederte Massen: Reifen, Schutzbleche, Rahmensäule beziehungsweise Rückgrat, Gabel, Lenker, Sattel, Gepäckträger, Leuchten, Kettenkasten, Pedale und Ständer."),
    "merged": ("Front: down tube, seat tube and bottom bracket are one column; hubs are one detail bar. Top: both chainstays and the seatstays are one wedge.",
               "Vorn: Unterrohr, Sattelrohr und Tretlager sind eine Säule; die Naben sind ein Detailbalken. Oben: beide Kettenstreben und die Sitzstreben sind ein Keil."),
})

# ================================================================ 1:200
front = [
    S("wheel-r", "wheel-r", "body", 4, rect(-0.025, 0, 0.05, 0.7, 0.02), 0.2, regions=TY + [MG, HB], material="rubber"),
    S("frame-column", "frame", "body", 3, rect(-0.025, 0.225, 0.05, 0.765), 0.3, regions=[FT, DR], material="paint"),
    S("saddle-rack", "frame", "body", 5, rect(-0.09, 0.8, 0.18, 0.24, 0.02), 0.3, regions=["saddle", FT, LP], material="saddle-skin"),
    S("wheel-f", "wheel-f", "body", 4, rect(-0.03, 0, 0.06, 0.7, 0.02), 0.2, regions=TY + [MG, HB], material="rubber"),
    S("fork", "fork-leg", "body", 3, rect(-0.052, 0.345, 0.104, 0.455, None, True), 0.3, region=ST, material="paint"),
    S("steer-column", "bars", "body", 3.2, rect(-0.02, 0.8, 0.04, 0.288), 0.3, regions=[ST, FT, LP], material="paint"),
    S("handlebar", "bars", "body", 3.3, rect(-0.31, 1.04, 0.62, 0.03, 0.012), 0.2, region=ST, material="rubber"),
    S("stand", "stand", "detail", 3, rect(-0.082, 0.318, 0.164, 0.012, None, True), 0.3, region=FT, material="paint"),
]
plan = [
    S("wheel-r", "wheel-r", "body", 4, rect(-0.936, -0.03, 0.701, 0.06), 0.2, regions=TY + [MG, HB], material="rubber"),
    S("rear-frame", "frame", "body", 3, poly([(-0.58, -0.055), (-0.1, -0.035), (0.347, -0.02), (0.347, 0.02), (-0.1, 0.035), (-0.58, 0.055)]), 0.3, regions=[FT, DR], material="paint"),
    S("saddle-rack", "frame", "body", 5, rect(-0.97, -0.09, 0.78, 0.18, 0.02), 0.3, regions=["saddle", LP], material="saddle-skin"),
    S("wheel-f", "wheel-f", "body", 4, rect(0.215, -0.03, 0.69, 0.06), 0.2, regions=TY + [MG, HB], material="rubber"),
    S("fork", "fork-leg", "body", 3, cap(0.347, 0.045, 0.56, 0.045, 0.012, follow=True), 0.3, region=ST, material="paint"),
    S("fork-r", "fork-leg", "body", 3, cap(0.347, -0.045, 0.56, -0.045, 0.012, follow=True), 0.3, region=ST, material="paint"),
    S("steer-column", "steer", "body", 3.2, cap(0.299, 0, 0.17, 0, 0.0125), 0.3, regions=[ST, FT, LP], material="paint"),
    S("handlebar", "bars", "body", 3.3, cap(0.04, -0.19, 0.04, 0.19, 0.0125), 0.2, region=ST, material="rubber"),
    S("sweep-l", "bars", "body", 3.3, cap(0.04, 0.19, -0.03, 0.292, 0.016), 0.2, region=ST, material="rubber"),
    S("sweep-r", "bars", "body", 3.3, cap(0.04, -0.19, -0.03, -0.292, 0.016), 0.2, region=ST, material="rubber"),
    S("stand", "stand", "detail", 3, cap(-0.42, 0.075, -0.1, 0.075, 0.01, follow=True), 0.3, region=FT, material="paint"),
    S("stand-r", "stand", "detail", 3, cap(-0.42, -0.075, -0.1, -0.075, 0.01, follow=True), 0.3, region=FT, material="paint"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top show only main masses: two wheel strips, a frame column or spine, saddle with rack, fork, steering column and one handlebar bar.",
                   "Vorder- und Draufsicht zeigen nur die Hauptmassen: zwei Radstreifen, eine Rahmensäule beziehungsweise ein Rückgrat, Sattel mit Gepäckträger, Gabel, Lenksäule und eine Lenkerstange."),
    "merged": ("Front and top: fenders and hubs are part of the wheel strips; chaincase and cranks are part of the frame; lamps are part of their column or rack.",
               "Vorn und oben: Schutzbleche und Naben gehören zu den Radstreifen; Kettenkasten und Kurbeln zum Rahmen; Leuchten zu ihrer Säule beziehungsweise zum Gepäckträger."),
})

STEM = cap(0, 0, -0.259, 0.133, 0.0125)
BAR = cap(-0.259, 0.133, -0.329, 0.108, 0.0125)
FORK = cap(0.048, -0.132, 0.261, -0.587, 0.007)
CRN = cap(0, 0, 0.17, 0, 0.012)
CRF = cap(0, 0, -0.17, 0, 0.012)
STD = cap(0, 0, 0.32, 0, 0.007)


def common(arms=True, arm_r=0.012):
    f = {"fork-l": ("steer", ext(FORK, 0.038, 0.052)), "fork-r": ("steer", ext(FORK, -0.052, -0.038)),
         "stand-l": ("stand", ext(STD, 0.068, 0.082)), "stand-r": ("stand", ext(STD, -0.082, -0.068))}
    if arms:
        f["arm-n"] = ("crank", ext(cap(0, 0, 0.17, 0, arm_r), -0.08, -0.06))
        f["arm-f"] = ("crank", ext(cap(0, 0, -0.17, 0, arm_r), 0.06, 0.08))
    return f


GRIP = ell(-0.329, 0.108, 0.016)
GRIPP = ell(-0.329, 0.108, 0.018)
SWEEP = cap(-0.259, 0.133, -0.309, 0.115, 0.0125)
f50 = common()
f50.update({"stem-low": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-high": None, "handlebar": ("steer", ext(BAR, -0.31, 0.31)),
            "grip-l": ("steer", ext(GRIP, 0.22, 0.31)), "grip-r": ("steer", ext(GRIP, -0.31, -0.22))})
p50 = common()
p50.update({"stem-head": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-bar": None,
            "handlebar-l": ("steer", ext(SWEEP, 0.0, 0.26), {"filled": True}), "handlebar-r": ("steer", ext(SWEEP, -0.26, 0.0), {"filled": True}),
            "grip-l": ("steer", ext(GRIPP, 0.26, 0.31)), "grip-r": ("steer", ext(GRIPP, -0.31, -0.26))})
f100 = common(False)
f100.update({"stem": ("steer", ext(STEM, -0.0125, 0.0125)), "handlebar": ("steer", ext(BAR, -0.31, 0.31))})
p100 = common(True, 0.017)
p100.update({"stem-head": ("steer", ext(STEM, -0.0125, 0.0125)), "stem-bar": None,
             "crossbar": ("steer", ext(ell(-0.259, 0.133, 0.0125), -0.18, 0.18)),
             "handlebar-l": ("steer", ext(SWEEP, 0.18, 0.26)), "handlebar-r": ("steer", ext(SWEEP, -0.26, -0.18)),
             "grip-l": ("steer", ext(GRIPP, 0.26, 0.31)), "grip-r": ("steer", ext(GRIPP, -0.31, -0.26))})
f200 = {"fork": ("steer", ext(FORK, -0.052, 0.052)), "steer-column": ("steer", ext(cap(0, -0.132, -0.259, 0.133, 0.02), -0.02, 0.02)),
        "handlebar": ("steer", ext(BAR, -0.31, 0.31)), "stand": ("stand", ext(cap(0, 0, 0.32, 0, 0.02), -0.082, 0.082))}
p200 = {"fork": ("steer", ext(FORK, 0.033, 0.057)), "fork-r": ("steer", ext(FORK, -0.057, -0.033)),
        "steer-column": ("steer", ext(cap(0, -0.132, -0.259, 0.133, 0.0125), -0.0125, 0.0125)),
        "handlebar": ("steer", ext(ell(-0.259, 0.133, 0.0125), -0.19, 0.19)),
        "sweep-l": ("steer", ext(cap(-0.259, 0.133, -0.329, 0.108, 0.016), 0.19, 0.31)), "sweep-r": ("steer", ext(cap(-0.259, 0.133, -0.329, 0.108, 0.016), -0.31, -0.19)),
        "stand": ("stand", ext(cap(0, 0, 0.32, 0, 0.02), 0.055, 0.095)), "stand-r": ("stand", ext(cap(0, 0, 0.32, 0, 0.02), -0.095, -0.055))}
apply_rules(a, {"50": {"front": f50, "plan": p50}, "100": {"front": f100, "plan": p100}, "200": {"front": f200, "plan": p200}})

save(AID, doc)
for s in ("50", "100", "200"):
    print(report(a, s))
