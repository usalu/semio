import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
d.pop("ndp.en1996.m95.q", None)
for system, lo, hi in (("unreinforced", 1.5, 2.5), ("confined", 2.0, 3.0), ("reinforced", 2.5, 3.0)):
    for bound, value in (("min", lo), ("max", hi)):
        put(d, "1996", f"m95.q.{system}.{bound}", "EN 1998-1 9.3 Table 9.1", f"q {bound} {system}", value, value, "", f"Behaviour factor range of the {system} masonry system, {bound} value as used by the artifact; not checked against the print of EN 1998-1 Table 9.1.", locator="M-95")
save(d)
