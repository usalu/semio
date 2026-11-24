"""🍄️ Writes the repair ops of `patio-heater`: seen from above the heater is the dome with its finial at 1:100 and at 1:200 alike. The plan of 1:100 gets the pressed stiffening rib of the dome as one circle of 0.44 m (4.4 mm on paper, drawn with the detail pen; 1:50 draws it with the bead and the nuts), which 1:200 leaves out."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("patio-heater")
rib = like(shape(a, 50, "plan", "dome-bead"), id="dome-rib", rx=0.22, ry=0.22)
write("patio-heater", [
    {"op": "shape-add", "scale": "100", "view": "plan", "after": "reflector", "shape": rib},
    {"op": "note", "scale": "100", "decision": "retained", "en": "Plan: the stiffening rib of the dome as one circle.", "de": "Draufsicht: die Versteifungssicke des Schirms als ein Kreis."},
])
