import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1996", "m95.diaphragm-ratio", "EN 1998-1 4.3.1(4)", "u_max / u_rigid", 1.1, 1.1, "1", "Rigid-diaphragm criterion as coded by the artifact: the diaphragm is rigid while its largest in-plane displacement stays within 1.10 times the displacement of the rigid-diaphragm assumption (the 10 % of EN 1998-1 4.3.1(4)); recalled, not checked against the print, no national value recorded.", part="1-1")
save(d)
print(len(d))
