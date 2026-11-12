import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *

S = [
    box("housing", "housing", "abs-plastic", "body", 0.9, "casing", (0.09, 0.09, 0.015), at=(0, 0, 0.006), corner=0.0072),
    box("display", "housing", "lcd-glass", "glazing", 0.12, "display", (0.036, 0.021, 0.0016), at=(0, 0.0265, -0.0023), maxd=50),
    ellipsoid("led", "housing", "led-lens", "detail", 0.35, "led", (0.0035, 0.0035, 0.002), at=(0.0315, 0.0265, -0.0015), maxd=50),
    revolve("dial", "dial", "abs-plastic", "body", 0.62, "dial",
            [(-0.0015, 0.0205), (-0.0035, 0.021), (-0.0112, 0.021), (-0.0123, 0.0195), (-0.0125, 0.0165), (-0.0125, 0)], axis=(0, 0, 1)),
    box("pointer", "dial", "abs-plastic", "detail", 0.95, "dial", (0.011, 0.0145, 0.002), at=(0, 0.01175, -0.0125), maxd=50),
]
D = [dec("The housing is a rounded box 15 mm deep; its wall side is +z, the dial and the display face -z, as in the plan view.",
         "Das Gehäuse ist ein abgerundeter Quader von 15 mm Tiefe; die Wandseite liegt bei +z, Knopf und Anzeige blicken nach -z, wie im Grundriss."),
     dec("The set-point dial is a revolve about the lateral axis w with a rounded rim, so it turns about its own axis in every pose; the pointer is a ridge on its face (1:50 only).",
         "Der Sollwertknopf ist ein Rotationskörper um die Querachse w mit gerundetem Rand, sodass er sich in jeder Pose um seine eigene Achse dreht; der Zeiger ist ein Steg auf seiner Stirnfläche (nur 1:50)."),
     dec("Display and LED have no front or plan outline; their depth is a 1.6 mm glass plate and a half-sunk lens, both only drawn at 1:50.",
         "Anzeige und LED haben keinen Aufriss- oder Grundrissumriss; ihre Tiefe ist eine 1,6 mm dicke Glasplatte und eine halb versenkte Linse, beide nur im Maßstab 1:50.")]
write("room-thermostat", S, D)
