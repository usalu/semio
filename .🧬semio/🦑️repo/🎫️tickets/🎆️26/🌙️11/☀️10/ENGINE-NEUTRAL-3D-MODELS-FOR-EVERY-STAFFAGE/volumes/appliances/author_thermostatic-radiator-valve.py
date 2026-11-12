import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

S = [
    revolve("body", "body", "brass", "body", 0.6, "body",
            [(-0.075, 0.0085), (-0.052, 0.0085), (-0.052, 0.012), (-0.042, 0.012), (-0.042, 0.019), (0, 0.019), (0, 0.017), (0.012, 0.017), (0.012, 0)],
            axis=(0, 1, 0)),
    revolve("collar", "head", "abs-plastic", "body", 0.8, "head",
            [(0.019, 0.027), (0.0175, 0.0295), (0.0155, 0.03), (-0.0485, 0.03), (-0.0515, 0.0285), (-0.052, 0.026)], axis=(0, 0, 1)),
    box("index", "head", "abs-plastic", "detail", 0.12, "head", (0.011, 0.0085, 0.0012), at=(0, 0.02475, -0.0526), maxd=50),
    revolve("knob", "knob", "abs-plastic", "body", 0.95, "knob",
            [(-0.052, 0.0195), (-0.0535, 0.02), (-0.0715, 0.02), (-0.0735, 0.0185), (-0.0745, 0.0155)], axis=(0, 0, 1), maxd=50),
    box("pointer", "knob", "abs-plastic", "detail", 0.1, "knob", (0.011, 0.0045, 0.0015), at=(0, 0.00575, -0.07525), maxd=50),
]
D = [dec("The brass body is one revolve about the vertical axis (pipe stub, union nut, hex body, neck), because front and elevation draw the same outline; the head sits on its neck.",
         "Der Messingkörper ist ein einziger Rotationskörper um die Hochachse (Rohrstutzen, Überwurfmutter, Sechskant, Hals), weil Aufriss und Ansicht dieselbe Kontur zeigen; der Kopf sitzt auf dem Hals."),
     dec("Head collar and setting knob are revolves about the lateral axis w (the screw axis of the head); the knob turns about it in every pose and clip.",
         "Kopfring und Einstellknopf sind Rotationskörper um die Querachse w (die Schraubenachse des Kopfes); der Knopf dreht sich in jeder Pose und jedem Clip um sie."),
     dec("Declared depth 0.090 m but the authored front and plan shapes span 0.095 m (collar 71 mm plus knob 24 mm); the body follows the shapes, the volume ends 5.6 % deeper than the declared depth. The unmaterialled collar groove line is not modelled.",
         "Deklarierte Tiefe 0,090 m, die gezeichneten Ansichts- und Grundrissformen messen aber 0,095 m (Ring 71 mm plus Knopf 24 mm); der Körper folgt den Formen, das Volumen ist 5,6 % tiefer als deklariert. Die material- und regionlose Ringnut-Linie wird nicht modelliert.")]
write("thermostatic-radiator-valve", S, D)
