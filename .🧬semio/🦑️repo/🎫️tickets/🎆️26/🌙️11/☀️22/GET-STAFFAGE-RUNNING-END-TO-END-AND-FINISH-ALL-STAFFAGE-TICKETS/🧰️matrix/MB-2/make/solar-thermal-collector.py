"""☀️ Writes the repair ops of `solar-thermal-collector`: the lumped base contact, which lay on the centre line 0.42 m from either base rail, becomes one contact per rail at its real lateral place (D45), with the supports, the contacts of poses and clips and the support pads following. Front and plan of 1:50 and 1:100 stay one drawing (allowlisted, D47)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("solar-thermal-collector")
contact = lambda sid, z, en, de: {"id": sid, "bone": "stand", "x": 0.75, "y": 0, "z": z, "kind": "contact", "title": {"en": en, "de": de}, "support": "contact", "footprintLengthM": 1.5, "footprintWidthM": 0.029, "frictionCoefficient": 0.4}
ops = [
    {"op": "contact-split", "anchor": "base-contact", "parts": [contact("base-contact-right", -0.4375, "Right base rail on the surface", "Rechte Fußschiene auf der Unterlage"), contact("base-contact-left", 0.4375, "Left base rail on the surface", "Linke Fußschiene auf der Unterlage")]},
    {"op": "set", "path": ["structure", "assumptions", 0], "value": {"en": "The 40 kg collector with its stand rests on two base rails: two unilateral contacts of 1.5 m by 0.029 m, 0.875 m apart, carry the weight; wind and snow loads are not part of the asset.", "de": "Der 40 kg schwere Kollektor mit Gestell ruht auf zwei Fußschienen: zwei einseitige Kontakte von 1,5 m mal 0,029 m im Abstand von 0,875 m tragen das Gewicht; Wind- und Schneelasten gehören nicht zum Asset."}},
]
write("solar-thermal-collector", ops)
