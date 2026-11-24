"""💡️ Writes the repair ops of `used-bollard-light`: the plan of 1:500, a 1 mm disc, keeps the socket and the head and drops the sand ring (50 mm, 0.1 mm on paper) and the flange ring (40 mm, 0.08 mm), which 1:200 still prints."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
write("used-bollard-light", [
    {"op": "shape-remove", "scale": "500", "view": "plan", "id": "sand-fill"},
    {"op": "shape-remove", "scale": "500", "view": "plan", "id": "flange"},
    {"op": "note", "scale": "500", "decision": "omitted", "en": "Plan: the sand ring and the flange ring (0.1 and 0.08 mm on paper).", "de": "Draufsicht: der Sandring und der Flanschring (0,1 und 0,08 mm auf dem Papier)."},
])
