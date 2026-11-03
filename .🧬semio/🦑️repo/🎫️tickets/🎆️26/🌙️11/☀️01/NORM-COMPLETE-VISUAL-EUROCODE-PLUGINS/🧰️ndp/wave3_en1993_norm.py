import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
MEM = "Written from memory by the W3-EN1993-NORM agent; NOT double-keyed from the print edition. Only a human verification against the cited document may flip the status."
MODEL = "Model constant of the cited literature, not an Eurocode value; EN 1993-1-8 3.12 prescribes no load-deformation law. Unverified artifact choice."
rows = [
 ("1-9.gamma-ff", "3(7)", "gamma_Ff", 1.0, "", "Partial factor on the fatigue loading; 1.0 as recommended", "EN 1993-1-9:2005", "3(7)", "1-9", MEM),
 ("1-9.stress-range.compression-factor", "5.4", "factor on the compressive part of a stress cycle", 0.6, "", "Compressive part of the stress cycle counted with 0.6 for non-welded and stress-relieved welded details (stress range sigma_max + 0.6 |sigma_min|)", "EN 1993-1-9:2005", "5.4", "1-9", MEM + " The clause number is not confirmed."),
 ("1-8.3.12.ic.mu", "3.12", "mu (instantaneous centre load-deformation law)", 10.0, "1/mm", "R = R_ult (1 - exp(-mu delta))^lambda of one bolt, delta in mm (Crawford and Kulak 1971)", "EN 1993-1-8:2005", "3.12", "1-8", MODEL),
 ("1-8.3.12.ic.lambda", "3.12", "lambda (instantaneous centre load-deformation law)", 0.55, "", "Exponent of the bolt load-deformation law R = R_ult (1 - exp(-mu delta))^lambda (Crawford and Kulak 1971)", "EN 1993-1-8:2005", "3.12", "1-8", MODEL),
 ("1-8.3.12.ic.delta-max", "3.12", "delta_max of the farthest bolt", 8.64, "mm", "Deformation of the farthest bolt at the ultimate state of the group (Crawford and Kulak 1971, 3/4 inch bolt)", "EN 1993-1-8:2005", "3.12", "1-8", MODEL),
 ("1-3.fastener.screw.bearing-equal", "8.3 Table 8.2", "bearing coefficient of a screw, t_2 = t_1", 3.2, "", "F_b,Rd = c sqrt(t^3 d) f_u,1/gamma_M2 for t_2 = t_1", "EN 1993-1-3:2006", "Table 8.2", "1-3", MEM),
 ("1-3.fastener.screw.bearing-thick", "8.3 Table 8.2", "bearing coefficient of a screw, t_2 >= 2.5 t_1", 3.2, "", "F_b,Rd = c t_1 d f_u,1/gamma_M2 for t_2 >= 2.5 t_1, linear interpolation in between", "EN 1993-1-3:2006", "Table 8.2", "1-3", MEM),
 ("1-3.fastener.screw.pull-over", "8.3 Table 8.3", "pull-over coefficient of a screw", 1.5, "", "F_p,Rd = c d_w t_1 f_u,1/gamma_M2", "EN 1993-1-3:2006", "Table 8.3", "1-3", MEM),
 ("1-3.fastener.screw.pull-out", "8.3 Table 8.3", "pull-out coefficient of a screw", 0.45, "", "F_o,Rd = c d t_2 f_u,2/gamma_M2", "EN 1993-1-3:2006", "Table 8.3", "1-3", MEM),
 ("1-3.fastener.rivet.bearing-equal", "8.3 Table 8.4", "bearing coefficient of a blind rivet, t_2 = t_1", 3.6, "", "F_b,Rd = c sqrt(t^3 d) f_u,1/gamma_M2 for t_2 = t_1", "EN 1993-1-3:2006", "Table 8.4", "1-3", MEM),
 ("1-3.fastener.rivet.bearing-thick", "8.3 Table 8.4", "bearing coefficient of a blind rivet, t_2 >= 2.5 t_1", 3.6, "", "F_b,Rd = c t_1 d f_u,1/gamma_M2 for t_2 >= 2.5 t_1, linear interpolation in between", "EN 1993-1-3:2006", "Table 8.4", "1-3", MEM),
 ("1-3.fastener.thickness-ratio", "8.3 Table 8.2", "t_2/t_1 upper end of the interpolation", 2.5, "", "Thickness ratio from which the thick-support bearing formula applies", "EN 1993-1-3:2006", "Table 8.2", "1-3", MEM),
 ("1-3.fastener.screw.e1-min", "8.3 Table 8.1", "e_1 / d of a screw", 3.0, "", "Minimum end distance in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.screw.e2-min", "8.3 Table 8.1", "e_2 / d of a screw", 1.5, "", "Minimum edge distance in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.screw.p1-min", "8.3 Table 8.1", "p_1 / d of a screw", 3.0, "", "Minimum pitch in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.screw.p2-min", "8.3 Table 8.1", "p_2 / d of a screw", 3.0, "", "Minimum gauge in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.rivet.e1-min", "8.3 Table 8.1", "e_1 / d of a blind rivet", 1.5, "", "Minimum end distance in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.rivet.e2-min", "8.3 Table 8.1", "e_2 / d of a blind rivet", 1.5, "", "Minimum edge distance in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.rivet.p1-min", "8.3 Table 8.1", "p_1 / d of a blind rivet", 3.0, "", "Minimum pitch in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fastener.rivet.p2-min", "8.3 Table 8.1", "p_2 / d of a blind rivet", 3.0, "", "Minimum gauge in units of d", "EN 1993-1-3:2006", "Table 8.1", "1-3", MEM),
 ("1-3.fya.k-roll", "3.2.2", "k of cold roll forming", 7.0, "", "f_ya = f_yb + (f_u - f_yb) k n t^2/A_g with k = 7 for cold roll forming", "EN 1993-1-3:2006", "3.2.2(3)", "1-3", MEM),
 ("1-3.fya.k-other", "3.2.2", "k of other forming", 5.0, "", "k = 5 for other forming methods", "EN 1993-1-3:2006", "3.2.2(3)", "1-3", MEM),
 ("1-3.fya.max-radius-ratio", "3.2.2", "r/t upper limit of an enhanced bend", 5.0, "", "Only 90 degree bends with an inner radius r <= 5 t count", "EN 1993-1-3:2006", "3.2.2(3)", "1-3", MEM),
 ("1-1.ductility.fu-fy", "3.2.2", "f_u/f_y lower limit", 1.10, "", "Ductility requirement f_u/f_y >= 1.10 for plastic global analysis", "EN 1993-1-1:2005", "3.2.2(1)", "1-1", MEM),
 ("1-1.ductility.elongation", "3.2.2", "elongation at failure lower limit", 0.15, "", "Ductility requirement elongation at failure >= 15 % (fraction)", "EN 1993-1-1:2005", "3.2.2(1)", "1-1", MEM),
 ("1-1.ductility.eps-u-factor", "3.2.2", "eps_u / eps_y lower limit", 15.0, "", "Ductility requirement eps_u >= 15 eps_y", "EN 1993-1-1:2005", "3.2.2(1)", "1-1", MEM),
]
ids = []
for key, clause, symbol, rec, unit, note, doc, loc_, part, ev in rows:
    i = put(d, "1993", key, clause, symbol, rec, rec, unit, note, part=part, locator=loc_)
    d[i]["source"]["document"] = doc
    d[i]["evidence"] = ev
    d[i]["specRef"] = "en1993-target.md (W3-EN1993-NORM)"
    ids.append(i)
save(d)
print(len(ids), "rows")
