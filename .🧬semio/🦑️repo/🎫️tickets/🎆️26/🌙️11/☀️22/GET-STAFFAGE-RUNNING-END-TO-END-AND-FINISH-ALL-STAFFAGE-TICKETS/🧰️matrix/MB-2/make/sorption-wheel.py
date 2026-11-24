"""🎡️ Writes the repair ops of `sorption-wheel`: the casing band seen from above and from ahead was one rectangle at every scale. At 1:50 its body is three strips (near band, the 16 mm belt strip, far band), so the plan and the front draw the two seams of the belt strip (0.32 mm apart on paper); at 1:100 the strip is 0.16 mm wide and is one line; 1:200 keeps the plain band."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("sorption-wheel")
base = like(shape(a, 50, "front", "gearhead-joint"), drop=("points", "closed", "kind"), id="belt-strip-edges", z=3.3)
line = lambda scale, **geometry: like(base, **geometry)
write("sorption-wheel", [
    {"op": "shape-add", "scale": "50", "view": "plan", "after": "casing", "shape": like(base, kind="path", d="M -0.31 0.022 L 0.31 0.022 M -0.31 0.038 L 0.31 0.038")},
    {"op": "shape-add", "scale": "50", "view": "front", "after": "casing", "shape": like(base, kind="path", d="M 0.022 0.235125 L 0.022 0.62 M 0.038 0.235125 L 0.038 0.62")},
    {"op": "note", "scale": "50", "decision": "retained", "en": "Top and front view: the two seams of the 16 mm belt strip in the casing band.", "de": "Draufsicht und Vorderansicht: die zwei Nähte des 16 mm breiten Riemenstreifens im Gehäuseband."},
    {"op": "shape-add", "scale": "100", "view": "plan", "after": "casing", "shape": like(base, id="belt-strip-line", kind="poly", points=[[-0.31, 0.03], [0.31, 0.03]], closed=False)},
    {"op": "shape-add", "scale": "100", "view": "front", "after": "casing", "shape": like(base, id="belt-strip-line", kind="poly", points=[[0.03, 0.235125], [0.03, 0.62]], closed=False)},
    {"op": "note", "scale": "100", "decision": "retained", "en": "Top and front view: the belt strip of the casing band as one line (16 mm, 0.16 mm on paper).", "de": "Draufsicht und Vorderansicht: der Riemenstreifen des Gehäusebands als eine Linie (16 mm, 0,16 mm auf dem Papier)."},
])
