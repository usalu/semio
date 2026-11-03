import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1997", "7.7.lateral-deflection-limit", "7.7", "y_lim", 0.010, None, "m", "Head deflection limit of a transversely loaded pile, 10 mm as requested by the en1997 v4 owner; no normative value is known and the national value is open.", part="1", keep=True)
put(d, "1997", "9.seismic.gamma-sliding", "EN 1998-5 7.3.2", "gamma_Rh,E", 1.0, None, "", "Seismic sliding partial factor 1.0 as requested by the en1997 v4 owner; EN 1998-5 leaves the seismic gamma_M/gamma_R to the National Annex.", part="1", keep=True)
put(d, "1997", "9.blum-embedment-increase", "9.7", "t/t0", 1.2, None, "", "Increase of the equilibrium embedment of a cantilever or Blum fixed-earth wall for the assumed rotation about the toe (EAB, 20 %); not checked against the print.", part="1", keep=True)
save(d)
