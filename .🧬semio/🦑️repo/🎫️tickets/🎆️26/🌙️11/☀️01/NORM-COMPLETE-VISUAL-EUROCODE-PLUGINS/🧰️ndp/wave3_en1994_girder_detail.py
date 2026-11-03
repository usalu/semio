import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
put(d, "1994", "2.stud-edge-distance", "6.6.5", "e_D", 0.02, None, "m", "Minimum distance of a headed stud from the flange edge, 20 mm as written in the target specification (C-20 / C-54); the clause number and the national value are not checked against the print.", part="2")
put(d, "1994", "2.stud-cross-spacing", "6.6.5", "e_t/d", 2.5, None, "", "Minimum transverse stud spacing 2.5 d, 4 d in a solid slab, as written in the target specification (C-20); not checked against the print.", part="2")
put(d, "1994", "2.min-transverse-reinforcement", "6.6.6.3", "rho_t", 0.002, None, "", "Minimum transverse reinforcement 0.002 times the concrete section per length as written in the target specification (C-19); NA-DE value open.", part="2")
put(d, "1994", "2.end-plate-bearing", "6.6.5", "F_Rdu", None, None, "", "End plate bearing idealised as the partially loaded area of EN 1992-1-1 6.7 (A_c1 grown to three times the plate, limited to the slab, F_Rdu <= 3 f_cd A_c0); the EN 1994-2 rule for the end anchorage is not transcribed.", part="2")
save(d)
