import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
UNV = "recalled, NOT checked against the printed standard (the repo holds no copy); verification is a human double-key"
def row(key, clause, symbol, v, note, evidence, part="1-1", doc=None):
    i = put(d, "1999", key, clause, symbol, v, v, "", note, part=part)
    d[i]["source"]["document"] = doc or "DIN EN 1999-1-1/NA"
    d[i]["specRef"] = "wave3c en1999"
    d[i]["evidence"] = evidence
row("table-8-2.p2-over-d0", "EN 1999-1-1 Table 8.2 (minimum spacing p2 perpendicular to the load)", "p2/d0", 2.4,
    "Minimum spacing p2 of fasteners perpendicular to the load direction as a multiple of the hole diameter d0 (p2 >= 2.4 d0)",
    f"Value p2 >= 2.4 d0 recalled from the printed Table 8.2 while the e1/e2 (1.2 d0) and p1 (2.2 d0) entries are the other minima of the same table; {UNV}. Reviewer: compare the p2 entry of Table 8.2 (and any footnote on staggered rows) with the print.")
CK = "Crawford and Kulak 1971 (Eccentrically loaded bolted connections, ASCE J. Struct. Div.) as tabulated in the AISC Steel Construction Manual, Part 7 (instantaneous centre of rotation); this is common practice for ductile bolt groups, EN 1999-1-1 8.5 gives no instantaneous-centre rule"
row("fastener-ic.curve-mu", "EN 1999-1-1 8.5 (fastener group, instantaneous-centre method; common practice, not an EN 1999 clause)", "mu = 10*Delta_max", 3.4,
    "Load-deformation curve of one fastener R/R_ult = (1 - exp(-mu*Delta/Delta_max))^k with the deformation Delta proportional to the distance from the instantaneous centre: exponent factor mu = 10 /in * 0.34 in",
    f"Source {CK}: R = R_ult (1 - e^(-10 Delta))^0.55 with Delta_max = 0.34 in for the farthest fastener, hence mu = 3.4; the curve is for 3/4 in steel bolts and is applied to aluminium fastener groups by analogy; {UNV}.")
row("fastener-ic.curve-exponent", "EN 1999-1-1 8.5 (fastener group, instantaneous-centre method; common practice, not an EN 1999 clause)", "k", 0.55,
    "Exponent k of the fastener load-deformation curve R/R_ult = (1 - exp(-mu*Delta/Delta_max))^k used by the instantaneous-centre method",
    f"Source {CK}: R = R_ult (1 - e^(-10 Delta))^0.55; {UNV}.")
save(d)
print(len(d))
