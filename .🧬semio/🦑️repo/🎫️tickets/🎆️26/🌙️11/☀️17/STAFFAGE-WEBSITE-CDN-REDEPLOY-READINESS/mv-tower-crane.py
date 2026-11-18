"""Tower crane 1:200 and 1:100 elevation: the drawn jib chord rectangles (0.14 m) and rail rectangles (0.12 m) are centred on the volume chord centre lines (y = +0.8 and -0.8 m); they were 0.06 m too far inside, and a slender volume member is widened to the drawn width around its own centre line. Applied to 1:200 in a first pass and to 1:100 in a second."""
from mvlib import *
d = D("🏗️machinery/tower-crane")
for sc in (200, 100):
    for sid, y in (("jib-chord-l", 0.73), ("jib-chord-r", 0.73), ("jib-rail-l", -0.86), ("jib-rail-r", -0.86)):
        d.set(d.shape(sc, "elevation", sid), "y", y)
d.commit()
