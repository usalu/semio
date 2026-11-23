"""🦯️ Person with white cane, three-views sweep (clip constant-contact-walk at toe-off, 1:500): the one boot per foot of 1:500 carries the toe, so the side elevation tips it about the ankle, while the front boot was a plate at one depth and only rose. The front boots of 1:500 become extruded solids of the boot's side profile over the width the front drawing had, so the front elevation shows the same boot as the side elevation and the 1:500 volume. `python3 tv1-person-with-white-cane.py [target json]`."""
import sys

import tv1_edit as e

doc = e.Doc(e.catalog_file("person-with-white-cane"))
PROFILE = [[-0.0078, -0.107], [0.2195, 0.0356], [0.1984, 0.0692], [0.0914, 0.0378], [-0.0229, 0.0267], [-0.0611, -0.0221]]

for side, span in (("l", (0.0373, 0.133)), ("r", (-0.1327, -0.037))):
    path, shape = doc.shape(500, "front", f"boot-{side}", f"foot-{side}")
    doc.set(path, {**shape, "points": PROFILE, "solid": {"kind": "extruded", "sweep": "w", "z0": span[0], "z1": span[1]}})

print(doc.save(sys.argv[1] if len(sys.argv) > 1 else None), "edits")
