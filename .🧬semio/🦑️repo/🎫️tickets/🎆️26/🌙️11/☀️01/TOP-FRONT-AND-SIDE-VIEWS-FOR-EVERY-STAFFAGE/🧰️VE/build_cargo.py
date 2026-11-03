"""cargo-bike: front and plan views at 1:500, 1:200, 1:100, 1:50. Adds view bones arm-n and arm-f at the crank."""
from lib import *

AID = "cargo-bike"
doc = load(AID)
a = doc["asset"]
strip_viewpoints(a)

TY, FR, BX, CG, FN, TP, BP, DM, BR = "tyres", "frame", "box-wood", "cargo", "fenders", "touch-parts", "battery-pack", "drive-motor", "brakes"
BW = 0.325

# ================================================================ 1:50
front = [
    S("tyre-r", "wheel-r", "body", 1, rect(-0.03, 0, 0.06, 0.66, 0.025), 0.1, region=TY, material="rubber"),
    S("rear-fender", "frame", "detail", 1.1, rect(-0.04, 0.48, 0.08, 0.205, 0.015), 0.2, region=FN, material="plastic"),
    S("rear-rack", "frame", "detail", 1.2, rect(-0.15, 0.78, 0.3, 0.02), 0.3, region=FR, material="aluminium"),
    S("rear-light", "frame", "detail", 1.3, rect(-0.03, 0.78, 0.06, 0.06, 0.006), 0.7, region=FN, material="plastic"),
    S("seat-tube", "frame", "body", 1.4, rect(-0.02, 0.304, 0.04, 0.746, 0.01), 0.35, region=FR, material="aluminium"),
    S("saddle", "frame", "body", 3.2, rect(-0.085, 1.043, 0.17, 0.062, 0.025), 0.15, region=TP, material="plastic"),
    S("main-beam", "frame", "body", 1.5, rect(-0.03, 0.29, 0.06, 0.14, 0.012), 0.35, region=FR, material="aluminium"),
    S("motor-housing", "motor", "detail", 1.6, rect(-0.07, 0.25, 0.14, 0.14, 0.01), 0.3, region=DM, material="motor-case"),
    S("crate-lower", "crate", "body", 2.5, rect(-0.26, 0.37, 0.52, 0.28, 0.02), 0.55, region=CG, material="plastic"),
    S("crate-upper", "crate", "body", 2.5, rect(-0.25, 0.65, 0.5, 0.28, 0.02), 0.62, region=CG, material="plastic"),
    S("box-shell", "box", "body", 3, rect(-BW, 0.35, 2 * BW, 0.65, 0.03), 0.7, region=BX, material="plywood"),
    S("box-planks", "box", "detail", 3.1, poly([(-BW, 0.55), (BW, 0.55)], False), 0.45, filled=False, pen=0.18),
    S("box-planks-2", "box", "detail", 3.1, poly([(-BW, 0.75), (BW, 0.75)], False), 0.45, filled=False, pen=0.18),
    S("bar-stem", "bar", "body", 3.3, rect(-0.013, 0.607, 0.026, 0.456, 0.006), 0.35, region=FR, material="aluminium"),
    S("handlebar", "bar", "body", 3.4, rect(-0.3, 1.06, 0.6, 0.03, 0.012), 0.3, region=TP, material="plastic"),
    S("grip-l", "bar", "body", 3.5, rect(0.2, 1.045, 0.1, 0.091, 0.03), 0.15, region=TP, material="plastic"),
    S("grip-r", "bar", "body", 3.5, rect(-0.3, 1.045, 0.1, 0.091, 0.03), 0.15, region=TP, material="plastic"),
    S("steer-fork", "fork", "body", 5, rect(-0.065, 0.24, 0.13, 0.4, 0.01), 0.35, region=FR, material="aluminium"),
    S("tyre-f", "wheel-f", "body", 5.5, rect(-0.03, 0, 0.06, 0.51, 0.025), 0.1, region=TY, material="rubber"),
    S("front-fender", "fork", "detail", 5.6, rect(-0.04, 0.397, 0.08, 0.143, 0.015), 0.2, region=FN, material="plastic"),
    S("hub-f", "wheel-f", "detail", 5.7, rect(-0.065, 0.225, 0.13, 0.06, 0.006), 0.45, region=BR, material="steel-disc"),
    S("head-lamp", "fork", "detail", 6, rect(-0.04, 0.59, 0.08, 0.06, 0.02), 0.9, region=FR, material="plastic"),
    S("arm-n", "arm-n", "detail", 3.6, rect(-0.14, 0.306, 0.025, 0.028, 0.003, True), 0.3, region=FR, material="alloy-rim"),
    S("arm-f", "arm-f", "detail", 1.7, rect(0.115, 0.306, 0.025, 0.028, 0.003, True), 0.3, region=FR, material="alloy-rim"),
    S("pedal-a", "arm-n", "detail", 3.7, rect(-0.2, 0.308, 0.1, 0.024, 0.004, True), 0.15, region=FR, material="plastic"),
    S("pedal-b", "arm-f", "detail", 1.8, rect(0.1, 0.31, 0.1, 0.02, 0.004, True), 0.15, region=FR, material="plastic"),
    S("stand-leg-l", "stand", "detail", 3.1, rect(0.11, -0.055, 0.02, 0.355, 0.006, True), 0.3, region=FR, material="aluminium"),
    S("stand-leg-r", "stand", "detail", 3.1, rect(-0.13, -0.055, 0.02, 0.355, 0.006, True), 0.3, region=FR, material="aluminium"),
    S("stand-foot", "stand", "detail", 3.1, rect(-0.15, -0.059, 0.3, 0.034, 0.012, True), 0.15, region=FR, material="plastic"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 1, rect(-0.33, -0.03, 0.66, 0.06, 0.012), 0.1, region=TY, material="rubber"),
    S("rear-fender", "frame", "detail", 1.1, rect(-0.307, -0.04, 0.629, 0.08, 0.02), 0.2, region=FN, material="plastic"),
    S("rear-rack", "frame", "detail", 1.2, rect(-0.27, -0.15, 0.57, 0.3, 0.01), 0.3, region=FR, material="aluminium"),
    S("rear-light", "frame", "detail", 1.3, rect(-0.3, -0.03, 0.04, 0.06, 0.006), 0.7, region=FN, material="plastic"),
    S("chainstay-l", "frame", "body", 1.4, cap(0, 0.06, 0.55, 0.03, 0.012), 0.35, region=FR, material="aluminium"),
    S("chainstay-r", "frame", "body", 1.4, cap(0, -0.06, 0.55, -0.03, 0.012), 0.35, region=FR, material="aluminium"),
    S("seatstay-l", "frame", "body", 1.45, cap(0, 0.06, 0.34, 0.02, 0.01), 0.35, region=FR, material="aluminium"),
    S("seatstay-r", "frame", "body", 1.45, cap(0, -0.06, 0.34, -0.02, 0.01), 0.35, region=FR, material="aluminium"),
    S("seat-tube", "frame", "body", 1.5, cap(0.286, 0, 0.566, 0, 0.02), 0.35, region=FR, material="aluminium"),
    S("battery-case", "battery", "detail", 3.4, rect(0.363, -0.06, 0.204, 0.12, 0.012), 0.2, region=BP, material="battery-case"),
    S("saddle", "frame", "body", 3.2, poly([(0.158, -0.085), (0.3, -0.09), (0.4, -0.04), (0.43, -0.02), (0.43, 0.02), (0.4, 0.04), (0.3, 0.09), (0.158, 0.085)]), 0.15, region=TP, material="plastic"),
    S("main-beam", "frame", "body", 1.5, cap(0.52, 0, 1.81, 0, 0.02), 0.35, region=FR, material="aluminium"),
    S("motor-housing", "motor", "detail", 1.6, rect(0.48, -0.07, 0.14, 0.14, 0.01), 0.3, region=DM, material="motor-case"),
    S("arm-n", "arm-n", "detail", 3.6, cap(0.55, -0.1, 0.72, -0.1, 0.014, follow=True), 0.3, region=FR, material="alloy-rim"),
    S("arm-f", "arm-f", "detail", 1.7, cap(0.55, 0.1, 0.38, 0.1, 0.014, follow=True), 0.3, region=FR, material="alloy-rim"),
    S("pedal-a", "arm-n", "detail", 3.7, rect(0.67, -0.185, 0.1, 0.09, 0.004), 0.15, region=FR, material="plastic"),
    S("pedal-b", "arm-f", "detail", 1.8, rect(0.36, 0.095, 0.06, 0.09, 0.004), 0.15, region=FR, material="plastic"),
    S("steer-rod", "fork", "detail", 2, cap(0.74, 0, 1.85, 0, 0.0135), 0.35, region=FR, material="aluminium"),
    S("crate", "crate", "body", 3.5, rect(1.15, -0.26, 0.5, 0.52, 0.025), 0.5, region=CG, material="plastic"),
    S("crate-slots", "crate", "detail", 3.6, poly([(1.3, -0.15), (1.5, -0.15)], False), 0.3, filled=False, pen=0.13),
    S("crate-slots-2", "crate", "detail", 3.6, poly([(1.3, 0.15), (1.5, 0.15)], False), 0.3, filled=False, pen=0.13),
    S("box-shell", "box", "body", 3, rect(0.95, -BW, 1.0, 2 * BW, 0.03), 0.7, region=BX, material="plywood"),
    S("box-rim", "box", "detail", 3.1, rect(0.98, -0.295, 0.94, 0.59, 0.02), 0.5, filled=False, region=BX, material="plywood", pen=0.18),
    S("bar-stem", "bar", "body", 3.3, cap(0.74, -0.2, 0.74, 0.2, 0.013), 0.35, region=FR, material="aluminium"),
    S("handlebar-l", "bar", "body", 3.4, cap(0.74, 0.2, 0.6, 0.27, 0.014), 0.3, region=TP, material="plastic"),
    S("handlebar-r", "bar", "body", 3.4, cap(0.74, -0.2, 0.6, -0.27, 0.014), 0.3, region=TP, material="plastic"),
    S("grip-l", "bar", "body", 3.5, cap(0.6, 0.27, 0.6, 0.285, 0.02), 0.15, region=TP, material="plastic"),
    S("grip-r", "bar", "body", 3.5, cap(0.6, -0.27, 0.6, -0.285, 0.02), 0.15, region=TP, material="plastic"),
    S("fork-leg-l", "fork", "body", 5, cap(1.85, 0.06, 2.1, 0.06, 0.012), 0.35, region=FR, material="aluminium"),
    S("fork-leg-r", "fork", "body", 5, cap(1.85, -0.06, 2.1, -0.06, 0.012), 0.35, region=FR, material="aluminium"),
    S("fork-crown", "fork", "body", 5.1, rect(1.83, -0.07, 0.04, 0.14, 0.008), 0.35, region=FR, material="aluminium"),
    S("tyre-f", "wheel-f", "body", 5.5, rect(1.845, -0.03, 0.51, 0.06, 0.012), 0.1, region=TY, material="rubber"),
    S("front-fender", "fork", "detail", 5.6, rect(1.853, -0.04, 0.494, 0.08, 0.02), 0.2, region=FN, material="plastic"),
    S("hub-f", "wheel-f", "detail", 5.7, rect(2.07, -0.065, 0.06, 0.13, 0.006), 0.45, region=BR, material="steel-disc"),
    S("head-lamp", "fork", "detail", 6, rect(1.91, -0.04, 0.08, 0.08, 0.025), 0.9, region=FR, material="plastic"),
    S("stand-leg-l", "stand", "detail", 3.1, cap(-0.08, 0.12, -0.11, 0.12, 0.01, 0.018, True), 0.3, region=FR, material="aluminium"),
    S("stand-leg-r", "stand", "detail", 3.1, cap(-0.08, -0.12, -0.11, -0.12, 0.01, 0.018, True), 0.3, region=FR, material="aluminium"),
]
apply_views(a, "50", front, plan, {
    "silhouette": ("Front: front wheel, steering fork, box with crate behind, handlebar, rear wheel, saddle, rack and stand legs; top: both wheels, box with crate, rack, saddle, frame spine, stays, motor, cranks, steering rod, handlebar, fork, fenders and stand legs.",
                   "Vorn: Vorderrad, Lenkgabel, Kasten mit Kiste dahinter, Lenker, Hinterrad, Sattel, Gepäckträger und Ständerbeine; oben: beide Räder, Kasten mit Kiste, Gepäckträger, Sattel, Rahmenrückgrat, Streben, Motor, Kurbeln, Lenkstange, Lenker, Gabel, Schutzbleche und Ständerbeine."),
    "overlays": ("Front and top shapes carry the regions of the side view. The crate follows the crate bone (loading slide and drop), the stand legs and the crank arms are bound with followTip, the crank arms through the added view bones arm-n and arm-f.",
                 "Vorder- und Draufsichtformen tragen die Regionen der Seitenansicht. Die Kiste folgt dem Knochen crate (Verschieben und Absenken), Ständerbeine und Kurbelarme sind mit followTip gebunden, die Kurbelarme über die ergänzten Ansichtsknochen arm-n und arm-f."),
})

# ================================================================ 1:100
front = [
    S("tyre-r", "wheel-r", "body", 1, rect(-0.03, 0, 0.06, 0.66, 0.025), 0.3, region=TY, material="rubber"),
    S("rear-fender", "frame", "detail", 1.1, rect(-0.04, 0.5, 0.08, 0.228, 0.015), 0.4, region=FN, material="plastic"),
    S("rear-rack", "frame", "detail", 1.2, rect(-0.15, 0.78, 0.3, 0.08), 0.4, regions=[FR, FN], material="aluminium"),
    S("seat-tube", "frame", "body", 1.4, rect(-0.02, 0.3, 0.04, 0.77, 0.01), 0.4, region=FR, material="aluminium"),
    S("saddle", "frame", "body", 3.2, rect(-0.085, 1.045, 0.17, 0.06, 0.025), 0.2, region=TP, material="plastic"),
    S("motor-housing", "motor", "detail", 1.6, rect(-0.07, 0.25, 0.14, 0.14), 0.3, region=DM, material="motor-case"),
    S("crate", "crate", "body", 2.5, rect(-0.26, 0.43, 0.52, 0.48), 0.5, region=CG, material="plastic"),
    S("box-shell", "box", "body", 3, rect(-BW, 0.35, 2 * BW, 0.65, 0.03), 0.6, region=BX, material="plywood"),
    S("bar-stem", "bar", "body", 3.3, rect(-0.013, 0.62, 0.026, 0.44), 0.4, region=FR, material="aluminium"),
    S("handlebar", "bar", "body", 3.4, rect(-0.3, 1.04, 0.6, 0.08, 0.03), 0.2, region=TP, material="plastic"),
    S("steer-fork", "fork", "body", 5, rect(-0.065, 0.24, 0.13, 0.4, 0.01), 0.4, region=FR, material="aluminium"),
    S("tyre-f", "wheel-f", "body", 5.5, rect(-0.03, 0, 0.06, 0.51, 0.025), 0.3, regions=[TY, BR], material="rubber"),
    S("front-fender", "fork", "detail", 5.6, rect(-0.04, 0.418, 0.08, 0.165, 0.015), 0.4, region=FN, material="plastic"),
    S("head-lamp", "fork", "detail", 6, rect(-0.04, 0.59, 0.08, 0.06, 0.02), 0.8, region=FR, material="plastic"),
    S("pedal-a", "arm-n", "detail", 3.7, rect(-0.2, 0.306, 0.1, 0.028, 0.004, True), 0.3, region=FR, material="alloy-rim"),
    S("pedal-b", "arm-f", "detail", 1.8, rect(0.1, 0.306, 0.1, 0.028, 0.004, True), 0.3, region=FR, material="alloy-rim"),
    S("stand-leg-l", "stand", "detail", 3.1, rect(0.11, -0.042, 0.02, 0.342, None, True), 0.4, region=FR, material="aluminium"),
    S("stand-leg-r", "stand", "detail", 3.1, rect(-0.13, -0.042, 0.02, 0.342, None, True), 0.4, region=FR, material="aluminium"),
]
plan = [
    S("tyre-r", "wheel-r", "body", 1, rect(-0.33, -0.03, 0.66, 0.06), 0.3, region=TY, material="rubber"),
    S("rear-fender", "frame", "detail", 1.1, rect(-0.344, -0.04, 0.705, 0.08, 0.02), 0.4, region=FN, material="plastic"),
    S("rear-rack", "frame", "detail", 1.2, rect(-0.27, -0.15, 0.57, 0.3), 0.4, regions=[FR], material="aluminium"),
    S("rear-stays", "frame", "body", 1.4, poly([(0, 0.06), (0.55, 0.03), (0.55, -0.03), (0, -0.06)]), 0.4, region=FR, material="aluminium"),
    S("seat-tube", "frame", "body", 1.5, cap(0.286, 0, 0.566, 0, 0.02), 0.4, region=FR, material="aluminium"),
    S("battery-case", "battery", "detail", 3.4, rect(0.4, -0.06, 0.06, 0.12), 0.3, region=BP, material="battery-case"),
    S("saddle", "frame", "body", 3.2, poly([(0.18, -0.085), (0.3, -0.09), (0.4, -0.04), (0.4, 0.04), (0.3, 0.09), (0.18, 0.085)]), 0.2, region=TP, material="plastic"),
    S("main-beam", "frame", "body", 1.5, cap(0.52, 0, 1.66, 0, 0.02), 0.4, region=FR, material="aluminium"),
    S("motor-housing", "motor", "detail", 1.6, rect(0.48, -0.07, 0.14, 0.14), 0.3, region=DM, material="motor-case"),
    S("crank-arms", "arm-n", "detail", 3.6, cap(0.55, -0.1, 0.72, -0.1, 0.014, follow=True), 0.5, region=FR, material="alloy-rim"),
    S("crank-arms-f", "arm-f", "detail", 1.7, cap(0.55, 0.1, 0.38, 0.1, 0.014, follow=True), 0.5, region=FR, material="alloy-rim"),
    S("steer-rod", "fork", "detail", 2, cap(0.74, 0, 1.85, 0, 0.0135), 0.4, region=FR, material="aluminium"),
    S("crate", "crate", "body", 3.5, rect(1.15, -0.26, 0.45, 0.52), 0.45, region=CG, material="plastic"),
    S("box-shell", "box", "body", 3, rect(0.95, -BW, 1.0, 2 * BW), 0.6, region=BX, material="plywood"),
    S("bar-stem", "bar", "body", 3.3, cap(0.74, -0.2, 0.74, 0.2, 0.013), 0.4, region=FR, material="aluminium"),
    S("handlebar-l", "bar", "body", 3.4, cap(0.74, 0.2, 0.6, 0.285, 0.017), 0.2, region=TP, material="plastic"),
    S("handlebar-r", "bar", "body", 3.4, cap(0.74, -0.2, 0.6, -0.285, 0.017), 0.2, region=TP, material="plastic"),
    S("fork", "fork", "body", 5, poly([(1.85, 0.07), (2.1, 0.07), (2.1, 0.05), (1.87, 0.05), (1.87, -0.05), (2.1, -0.05), (2.1, -0.07), (1.85, -0.07)]), 0.4, region=FR, material="aluminium"),
    S("tyre-f", "wheel-f", "body", 5.5, rect(1.845, -0.03, 0.51, 0.06), 0.3, regions=[TY, BR], material="rubber"),
    S("front-fender", "fork", "detail", 5.6, rect(1.816, -0.04, 0.568, 0.08, 0.02), 0.4, region=FN, material="plastic"),
    S("head-lamp", "fork", "detail", 6, rect(1.91, -0.04, 0.08, 0.08, 0.025), 0.8, region=FR, material="plastic"),
    S("stand-leg-l", "stand", "detail", 3.1, cap(-0.08, 0.12, -0.11, 0.12, 0.01, 0.018, True), 0.4, region=FR, material="aluminium"),
    S("stand-leg-r", "stand", "detail", 3.1, cap(-0.08, -0.12, -0.11, -0.12, 0.01, 0.018, True), 0.4, region=FR, material="aluminium"),
]
apply_views(a, "100", front, plan, {
    "silhouette": ("Front and top views are articulated masses: wheels, fenders, box with crate, frame spine, saddle, motor, handlebar with steering rod, fork and stand legs.",
                   "Vorder- und Draufsicht sind gegliederte Massen: Räder, Schutzbleche, Kasten mit Kiste, Rahmenrückgrat, Sattel, Motor, Lenker mit Lenkstange, Gabel und Ständerbeine."),
    "merged": ("Both crate halves, the grips and handlebar, and the fork legs with the crown are single shapes in front and top.",
               "Beide Kistenhälften, Griffe und Lenker sowie Gabelholme mit Krone sind vorn und oben je eine Form."),
})

# ================================================================ 1:200
front = [
    S("wheel-r", "wheel-r", "body", 1, rect(-0.035, 0, 0.07, 0.69, 0.025), 0.3, regions=[TY, FN, BR], material="rubber"),
    S("rear-column", "frame", "body", 1.4, rect(-0.15, 0.3, 0.3, 0.77), 0.4, regions=[FR, TP, FN], material="aluminium"),
    S("crate", "crate", "body", 2.5, rect(-0.26, 0.37, 0.52, 0.56), 0.5, region=CG, material="plastic"),
    S("box-shell", "box", "body", 3, rect(-BW, 0.35, 2 * BW, 0.65), 0.6, regions=[BX, BP, DM], material="plywood"),
    S("bars", "bar", "body", 3.4, rect(-0.3, 1.04, 0.6, 0.08, 0.03), 0.2, regions=[TP, FR], material="plastic"),
    S("wheel-f", "wheel-f", "body", 5.5, rect(-0.04, 0, 0.08, 0.58, 0.03), 0.3, regions=[TY, BR, FN], material="rubber"),
    S("fork", "fork", "body", 5, rect(-0.065, 0.24, 0.13, 0.4, 0.01), 0.4, region=FR, material="aluminium"),
    S("stand", "stand", "detail", 3.1, rect(-0.13, -0.042, 0.26, 0.342, None, True), 0.4, region=FR, material="aluminium"),
]
plan = [
    S("wheel-r", "wheel-r", "body", 1, rect(-0.344, -0.04, 0.705, 0.08, 0.015), 0.3, regions=[TY, FN, BR], material="rubber"),
    S("rear-frame", "frame", "body", 1.4, poly([(-0.27, -0.15), (0.3, -0.15), (0.566, -0.05), (1.66, -0.02), (1.66, 0.02), (0.566, 0.05), (0.3, 0.15), (-0.27, 0.15)]), 0.4, regions=[FR, TP, FN, DM, BP], material="aluminium"),
    S("crate", "crate", "body", 3.5, rect(1.15, -0.26, 0.5, 0.52), 0.5, region=CG, material="plastic"),
    S("box-shell", "box", "body", 3, rect(0.95, -BW, 1.0, 2 * BW), 0.6, region=BX, material="plywood"),
    S("bars", "bar", "body", 3.4, poly([(0.78, -0.2), (0.78, 0.2), (0.6, 0.3), (0.58, 0.28), (0.7, 0.0), (0.58, -0.28), (0.6, -0.3)]), 0.2, regions=[TP, FR], material="plastic"),
    S("fork", "fork", "body", 5, poly([(1.85, 0.07), (2.1, 0.07), (2.1, 0.05), (1.87, 0.05), (1.87, -0.05), (2.1, -0.05), (2.1, -0.07), (1.85, -0.07)]), 0.4, region=FR, material="aluminium"),
    S("wheel-f", "wheel-f", "body", 5.5, rect(1.816, -0.04, 0.539, 0.08, 0.015), 0.3, regions=[TY, BR, FN], material="rubber"),
    S("stand-l", "stand", "detail", 3.1, cap(-0.08, 0.12, -0.11, 0.12, 0.012, 0.02, True), 0.4, region=FR, material="aluminium"),
    S("stand-r", "stand", "detail", 3.1, cap(-0.08, -0.12, -0.11, -0.12, 0.012, 0.02, True), 0.4, region=FR, material="aluminium"),
]
apply_views(a, "200", front, plan, {
    "silhouette": ("Front and top views keep the main masses: two wheel strips, box with crate, rider column or frame, handlebar, fork and stand.",
                   "Vorder- und Draufsicht behalten die Hauptmassen: zwei Radstreifen, Kasten mit Kiste, Fahrersäule beziehungsweise Rahmen, Lenker, Gabel und Ständer."),
    "merged": ("Fenders and discs belong to the wheel strips; battery and motor belong to the frame (top) or are hidden behind the box (front); the pedals are dropped.",
               "Schutzbleche und Scheiben gehören zu den Radstreifen; Akku und Motor gehören zum Rahmen (oben) oder liegen vorn hinter dem Kasten; die Pedale entfallen."),
})

# ================================================================ 1:500
front = [
    S("wheel-r", "wheel-r", "body", 1, rect(-0.04, 0, 0.08, 0.66, 0.03), 0.3, regions=[TY, BR], material="rubber"),
    S("box-shell", "box", "body", 3, rect(-BW, 0.35, 2 * BW, 0.65), 0.6, regions=[BX, CG], material="plywood"),
    S("bars-column", "bar", "body", 3.4, poly([(-0.3, 1.04), (-0.02, 1.04), (-0.02, 0.62), (0.02, 0.62), (0.02, 1.04), (0.3, 1.04), (0.3, 1.12), (-0.3, 1.12)]), 0.3, regions=[FR, TP, BP, DM], material="aluminium"),
    S("wheel-f", "wheel-f", "body", 5.5, rect(-0.04, 0, 0.08, 0.51, 0.03), 0.3, regions=[TY, BR, FN], material="rubber"),
    S("stand", "stand", "body", 3.1, rect(-0.13, -0.042, 0.26, 0.342, None, True), 0.4, region=FR, material="aluminium"),
]
plan = [
    S("wheel-r", "wheel-r", "body", 1, rect(-0.33, -0.04, 0.66, 0.08), 0.3, regions=[TY, BR], material="rubber"),
    S("frame-spine", "frame", "body", 1.4, poly([(-0.27, -0.12), (0.3, -0.12), (0.566, -0.03), (1.66, -0.03), (1.66, 0.03), (0.566, 0.03), (0.3, 0.12), (-0.27, 0.12)]), 0.4, regions=[FR, TP, FN, DM, BP], material="aluminium"),
    S("box-shell", "box", "body", 3, rect(0.95, -BW, 1.0, 2 * BW), 0.6, region=BX, material="plywood"),
    S("crate", "crate", "body", 3.5, rect(1.15, -0.26, 0.5, 0.52), 0.5, region=CG, material="plastic"),
    S("bars", "bar", "body", 3.4, poly([(0.78, -0.2), (0.78, 0.2), (0.6, 0.3), (0.58, 0.26), (0.7, 0.0), (0.58, -0.26), (0.6, -0.3)]), 0.3, region=TP, material="plastic"),
    S("wheel-f", "wheel-f", "body", 5.5, rect(1.845, -0.04, 0.51, 0.08, 0.015), 0.3, regions=[TY, BR, FN], material="rubber"),
    S("stand", "stand", "body", 3.1, cap(-0.08, 0.12, -0.11, 0.12, 0.03, 0.03, True), 0.4, region=FR, material="aluminium"),
]
apply_views(a, "500", front, plan, {
    "silhouette": ("Front and top views are composed silhouettes of a few filled masses: wheels, box with crate, frame spine, handlebar and stand.",
                   "Vorder- und Draufsicht sind komponierte Silhouetten aus wenigen gefüllten Massen: Räder, Kasten mit Kiste, Rahmenrückgrat, Lenker und Ständer."),
    "thinElements": ("Frame tubes, stays, fork and steering rod are widened to the spine (top) or the handlebar column (front); the stand is one shape about 0.06 m wide in plan.",
                     "Rahmenrohre, Streben, Gabel und Lenkstange sind zum Rückgrat (oben) beziehungsweise zur Lenkersäule verbreitert; der Ständer ist oben eine Form von etwa 0,06 m Breite."),
})

CRN = cap(0, 0, 0.17, 0, 0.014)
CRF = cap(0, 0, -0.17, 0, 0.014)
STD = cap(0, 0, 0.3433, 0, 0.01)
PA = rect(0.12, -0.012, 0.1, 0.024, 0.004)
PB = rect(-0.19, -0.01, 0.06, 0.02, 0.004)
ARMS = {"arm-n": ("crank", ext(CRN, -0.115, -0.085)), "arm-f": ("crank", ext(CRF, 0.085, 0.115)),
        "pedal-a": ("crank", ext(PA, -0.19, -0.1)), "pedal-b": ("crank", ext(PB, 0.1, 0.19))}
LEGS = {"stand-leg-l": ("stand", ext(STD, 0.11, 0.13)), "stand-leg-r": ("stand", ext(STD, -0.13, -0.11)),
        "stand-foot": ("stand", ext(cap(0.3, 0, 0.3433, 0, 0.017), -0.15, 0.15))}
f50 = dict(ARMS)
f50.update(LEGS)
p50 = dict(ARMS)
p50.update(LEGS)
p50.pop("stand-foot")
f100 = {"pedal-a": ARMS["pedal-a"], "pedal-b": ARMS["pedal-b"], "stand-leg-l": LEGS["stand-leg-l"], "stand-leg-r": LEGS["stand-leg-r"]}
p100 = {"crank-arms": ARMS["arm-n"], "crank-arms-f": ARMS["arm-f"], "stand-leg-l": LEGS["stand-leg-l"], "stand-leg-r": LEGS["stand-leg-r"]}
f200 = {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.015), -0.13, 0.13))}
p200 = {"stand-l": ("stand", ext(cap(0, 0, 0.3433, 0, 0.012), 0.11, 0.13)), "stand-r": ("stand", ext(cap(0, 0, 0.3433, 0, 0.012), -0.13, -0.11))}
apply_rules(a, {"50": {"front": f50, "plan": p50}, "100": {"front": f100, "plan": p100}, "200": {"front": f200, "plan": p200},
                "500": {"front": {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.02), -0.13, 0.13))}, "plan": {"stand": ("stand", ext(cap(0, 0, 0.3433, 0, 0.03), 0.09, 0.15))}}})

save(AID, doc)
for s in ("50", "100", "200", "500"):
    print(report(a, s))
