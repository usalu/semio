"""🏖️ Writes the repair ops of `used-sandpit-digger`: the front of 1:50 gets the drying checks of the two old larch courses (three cracks 0.5 to 0.6 m long, 10 to 12 mm on paper, drawn with the texture pen), as the side elevation of 1:50 has them and 1:100 has fewer; the fresh front board stays clean."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("used-sandpit-digger")
base = shape(a, 50, "elevation", "check-right-4")
CHECKS = {
    "check-front-1": [[-1.5, 0.0912], [-1.4083, 0.0874], [-1.3167, 0.0921], [-1.225, 0.0889], [-1.1333, 0.0935], [-1.0417, 0.0897], [-0.95, 0.0918]],
    "check-front-2": [[0.9, 0.2204], [1.0, 0.2251], [1.1, 0.2218], [1.2, 0.2262], [1.3, 0.2227], [1.4, 0.2259], [1.5, 0.2213]],
    "check-front-3": [[1.05, 0.0489], [1.1333, 0.0522], [1.2167, 0.0478], [1.3, 0.0531], [1.3833, 0.0494], [1.4667, 0.0517], [1.55, 0.0483]],
}
write("used-sandpit-digger", [{"op": "shape-add", "scale": "50", "view": "front", "after": "graffiti-front-0", "shape": like(base, id=sid, points=points)} for sid, points in CHECKS.items()] + [
    {"op": "note", "scale": "50", "decision": "retained", "en": "Front: three drying checks in the two old courses.", "de": "Vorderansicht: drei Trockenrisse in den zwei alten Lagen."},
])
