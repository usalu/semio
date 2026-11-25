"""💧️ Ops writer of `evaporative-humidifier`: the contact `base-contact`, one anchor at the middle of the base for the two runners the unit stands on, 20 mm above the floor line of the casing at 1:50 and 1:100, becomes one contact per runner (D45) with its own support pad at every scale; the structural assumption says so: `python3 make/evaporative-humidifier.py` writes `ops/evaporative-humidifier.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("evaporative-humidifier")
part = lambda name, x, en, de: {"id": f"runner-{name}-contact", "bone": "casing", "x": x, "y": 0, "kind": "contact", "title": {"en": en, "de": de}, "support": "contact", "footprintLengthM": 0.04, "footprintWidthM": 0.4, "frictionCoefficient": 0.4}
parts = [part("inlet", 0.06, "Inlet-side base runner on the surface", "Kufe der Eintrittsseite auf der Unterlage"), part("outlet", 0.44, "Outlet-side base runner on the surface", "Kufe der Austrittsseite auf der Unterlage")]
text = dict(a["structure"]["assumptions"][0])
text["en"] = text["en"].replace("a unilateral contact of 0.42 m by 0.4 m carries the weight", "one unilateral contact of 0.04 m by 0.4 m under each runner carries the weight")
text["de"] = text["de"].replace("ein einseitiger Kontakt von 0,42 m mal 0,4 m trägt das Gewicht", "je ein einseitiger Kontakt von 0,04 m mal 0,4 m unter jeder Kufe trägt das Gewicht")
write("evaporative-humidifier", [
    {"op": "contact-split", "anchor": "base-contact", "parts": parts, "pads": [{"x": 0.06, "lengthM": 0.04}, {"x": 0.44, "lengthM": 0.04}]},
    put(["structure", "assumptions", 0], text),
])
