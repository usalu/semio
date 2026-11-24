"""♨️ Writes the repair ops of `plate-heat-exchanger`: the lumped base contact, which lay 30 mm under the casing between the two base rails, becomes one contact per leg of the two rails (inverted channels of 40 mm with 2 mm legs, drawn so in the section of 1:50) at its real place (D45), with the supports, the contacts of poses and clips and the support pads following. Seen from above the bodies of 1:100 and 1:200 show nothing that differs, so their plan stays one drawing (allowlisted, D47)."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import write
LEGS = (
    ("rear-outer", 0.041, 0.06, "Outer leg of the rear base rail on the surface", "Äußerer Schenkel der hinteren Sockelschiene auf der Unterlage"),
    ("rear-inner", 0.079, 0.06, "Inner leg of the rear base rail on the surface", "Innerer Schenkel der hinteren Sockelschiene auf der Unterlage"),
    ("front-inner", 0.421, 0.44, "Inner leg of the front base rail on the surface", "Innerer Schenkel der vorderen Sockelschiene auf der Unterlage"),
    ("front-outer", 0.459, 0.44, "Outer leg of the front base rail on the surface", "Äußerer Schenkel der vorderen Sockelschiene auf der Unterlage"),
)
write("plate-heat-exchanger", [
    {"op": "contact-split", "anchor": "base-contact", "parts": [{"id": f"base-contact-{name}", "bone": "frame", "x": x, "y": 0, "kind": "contact", "title": {"en": en, "de": de}, "support": "contact", "footprintLengthM": 0.002, "footprintWidthM": 0.5, "frictionCoefficient": 0.4} for name, x, _, en, de in LEGS], "pads": [{"x": pad, "lengthM": 0.04} for _, _, pad, _, _ in LEGS]},
    {"op": "set", "path": ["structure", "assumptions", 0], "value": {"en": "The 18 kg exchanger stands on the four 2 mm legs of its two base rails (inverted channels of 40 mm): four unilateral line contacts of 0.5 m carry the weight.", "de": "Der 18 kg schwere Übertrager steht auf den vier 2 mm starken Schenkeln seiner zwei Sockelschienen (umgekehrte U-Profile von 40 mm): vier einseitige Linienkontakte von 0,5 m tragen das Gewicht."}},
])
