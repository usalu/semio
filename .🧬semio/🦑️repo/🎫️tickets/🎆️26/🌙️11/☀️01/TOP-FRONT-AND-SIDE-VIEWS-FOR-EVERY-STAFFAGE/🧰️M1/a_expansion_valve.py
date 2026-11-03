import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

DOME_FRONT = "M -0.025 0.05 C -0.025 0.068 -0.014 0.078 0 0.078 C 0.014 0.078 0.025 0.068 0.025 0.05 Z"

# Front: lateral x (asset left = +x), world y up. Plan: forward x, lateral y (left = +y).
# The sensing bulb sits 18 mm to the left of the sagittal plane so that it does not hide the outlet end in the front view.
front50 = [
    poly("capillary", "head", "detail", 3.6, 0.1, [(0, 0.072), (0.011, 0.088), (0.02, 0.082), (0.02, 0.05)], closed=False, region="capillary", material="copper"),
    rect("bulb", "bulb", "body", 3.8, 0.5, 0.008, 0.015, 0.02, 0.035, region="bulb", material="copper"),
    rect("stem", "stem", "body", 3.3, 0.3, -0.01, 0.078, 0.02, 0.012, region="head", material="brass"),
    path("head", "head", "body", 3.4, 0.55, DOME_FRONT, region="head", material="brass"),
    ellipse("outlet", "body", "body", 3.7, 0.4, 0, 0.019, 0.009, 0.009, region="outlet", material="copper"),
    rect("body", "body", "body", 3.5, 0.5, -0.03, 0, 0.06, 0.05, region="body", material="brass"),
]
plan50 = [
    poly("capillary", "head", "detail", 3.7, 0.1, [(0.018, 0), (0.05, 0.012), (0.09, 0.02), (0.108, 0.02)], closed=False, region="capillary", material="copper"),
    ellipse("bulb", "bulb", "body", 3.3, 0.5, 0.108, 0.018, 0.01, 0.01, region="bulb", material="copper"),
    ellipse("stem", "stem", "body", 3.9, 0.3, 0, 0, 0.01, 0.01, region="head", material="brass"),
    ellipse("head", "head", "body", 3.6, 0.55, 0, 0, 0.025, 0.025, region="head", material="brass"),
    rect("inlet", "body", "body", 3.3, 0.4, -0.078, -0.009, 0.046, 0.018, region="inlet", material="copper"),
    rect("outlet", "body", "body", 3.3, 0.4, 0.032, -0.009, 0.046, 0.018, region="outlet", material="copper"),
    rect("body", "body", "body", 3.5, 0.5, -0.032, -0.03, 0.064, 0.06, region="body", material="brass"),
]
front100 = [
    rect("bulb", "bulb", "body", 3.8, 0.5, 0.006, 0.015, 0.024, 0.035, regions=["bulb", "capillary"], material="copper"),
    path("head", "head", "body", 3.4, 0.55, DOME_FRONT, regions=["head"], material="brass"),
    ellipse("outlet", "body", "body", 3.7, 0.4, 0, 0.019, 0.011, 0.011, region="outlet", material="copper"),
    rect("body", "body", "body", 3.5, 0.5, -0.03, 0, 0.06, 0.05, region="body", material="brass"),
]
plan100 = [
    ellipse("bulb", "bulb", "body", 3.3, 0.5, 0.108, 0.018, 0.012, 0.012, regions=["bulb", "capillary"], material="copper"),
    ellipse("head", "head", "body", 3.6, 0.55, 0, 0, 0.025, 0.025, regions=["head"], material="brass"),
    rect("inlet", "body", "body", 3.3, 0.4, -0.078, -0.011, 0.046, 0.022, region="inlet", material="copper"),
    rect("outlet", "body", "body", 3.3, 0.4, 0.032, -0.011, 0.046, 0.022, region="outlet", material="copper"),
    rect("body", "body", "body", 3.5, 0.5, -0.032, -0.03, 0.064, 0.06, region="body", material="brass"),
]

apply("expansion-valve", {
    "50": {"front": front50, "plan": plan50, "dec": {
        "retained": (
            "Front view: body block, diaphragm dome, adjusting stem, the outlet end seen head-on, the bulb offset 18 mm to the left and the capillary looping down to it; the stem drops with the pose. Plan: body, round dome with stem, both connections, bulb circle and capillary route.",
            "Vorderansicht: Ventilblock, Membrankuppe, Einstellspindel, der Austritt in der Stirnansicht, der Fühler 18 mm nach links versetzt und die Kapillare, die zu ihm hinabläuft; die Spindel senkt sich mit der Pose. Grundriss: Körper, runde Kuppe mit Spindel, beide Anschlüsse, Fühlerkreis und Kapillarverlauf."),
        "omitted": (
            "In the front view the inlet stays hidden behind the body block; the body is drawn with its lateral size of 60 mm, not 64 mm.",
            "In der Vorderansicht bleibt der Eintritt hinter dem Ventilblock verdeckt; der Körper ist mit dem Quermaß von 60 mm gezeichnet, nicht mit 64 mm."),
    }},
    "100": {"front": front100, "plan": plan100, "dec": {
        "retained": (
            "Front view: body block, dome, outlet circle and one bulb block; plan: body, dome circle, two connection stubs and the bulb circle.",
            "Vorderansicht: Ventilblock, Kuppe, Austrittskreis und ein Fühlerblock; Grundriss: Körper, Kuppenkreis, zwei Anschlussstutzen und der Fühlerkreis."),
        "merged": (
            "In both views the capillary joins the bulb (regions bulb and capillary).",
            "In beiden Ansichten geht die Kapillare im Fühler auf (Regionen Fühler und Kapillare)."),
        "omitted": (
            "Stem and capillary line in front and plan; the inlet is hidden in the front view.",
            "Spindel und Kapillarlinie in Vorderansicht und Grundriss; der Eintritt ist in der Vorderansicht verdeckt."),
    }},
})
