"""🌀️ Ops writer of `scroll-compressor`: the contact `base-contact`, one anchor under the middle of the shell for the four rubber grommets the compressor stands on, 22 mm off the drawn feet at 1:50, becomes one contact per grommet at its place (D45); the support of every scale, a reference to the four repeated grommets, becomes one pad per grommet (the grommet solid written out at its place), and the structural assumption says so: `python3 make/scroll-compressor.py` writes `ops/scroll-compressor.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("scroll-compressor")
R = 0.09525
FEET = [("front-left", R, R, "Front left", "vorn links"), ("front-right", R, -R, "Front right", "vorn rechts"), ("back-right", -R, -R, "Back right", "hinten rechts"), ("back-left", -R, R, "Back left", "hinten links")]
parts = [{"id": f"grommet-{name}", "bone": "shell", "x": x, "y": 0, "z": z, "kind": "contact", "title": {"en": f"{en} grommet on the surface", "de": f"Gummitülle {de} auf der Unterlage"}, "support": "contact", "footprintLengthM": 0.068, "footprintWidthM": 0.068, "frictionCoefficient": 0.4} for name, x, z, en, de in FEET]
ops = []
for scale, volume in a["volumes"].items():
    if "structure" not in volume:
        continue
    grommets = next(s for s in volume["body"] if s["id"] == "grommets")
    reference = next((s for s in volume["structure"] if s["id"] == "contact-grommets"), None)
    pads = {f"grommet-{name}-pad" for name, *_ in FEET}
    kept = [s for s in volume["structure"] if s["id"] != "contact-grommets" and s["id"] not in pads]
    member = reference["member"] if reference is not None else next(s for s in volume["structure"] if s["id"] in pads)["member"]
    new = [dict({k: v for k, v in grommets.items() if k != "repeat"}, id=f"grommet-{name}-pad", origin=[x, 0, z], member=member, part="support", anchor=f"grommet-{name}") for name, x, z, *_ in FEET]
    at = next((i for i, s in enumerate(volume["structure"]) if s["id"] == "contact-grommets" or s["id"] in pads), len(kept))
    ops.append(put(["volumes", scale, "structure"], kept[:at] + new + kept[at:]))
text = dict(a["structure"]["assumptions"][0])
text["en"] = text["en"].replace("a unilateral contact of 0.23 m by 0.23 m carries the weight", "one unilateral contact of 68 mm diameter under each grommet carries the weight")
text["de"] = text["de"].replace("ein einseitiger Kontakt von 0,23 m mal 0,23 m trägt das Gewicht", "je ein einseitiger Kontakt von 68 mm Durchmesser unter jeder Tülle trägt das Gewicht")
ops += [{"op": "contact-split", "anchor": "base-contact", "parts": parts}, put(["structure", "assumptions", 0], text)]
write("scroll-compressor", ops)
