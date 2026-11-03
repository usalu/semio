import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
EV = "Written from memory by the en1993 artifact agent; NOT double-keyed from the print edition. Only a human verification against EN 1993-1-8:2005 may flip the status."
rows = [
 ("v-440.1", "7.4.2", "Table 7.11 N_1,Rd chord face", "N_1,Rd = 8.9 k_n f_y0 t_0^2 sqrt(gamma) (b_1+b_2+h_1+h_2)/(4 b_0 sin(theta_1))/gamma_M5, N_2,Rd = N_1,Rd sin(theta_1)/sin(theta_2); k_n = 1 (tension) else min(1.3-0.4 n/beta, 1)", "Table 7.11"),
 ("v-440.2", "7.4.2", "Table 7.11 N_i,Rd brace effective width", "N_i,Rd = f_yi t_i (2 h_i - 4 t_i + b_i + b_eff)/gamma_M5, b_eff = 10/(b_0/t_0) f_y0 t_0/(f_yi t_i) b_i <= b_i", "Table 7.11"),
 ("v-440.3", "7.4.1", "Table 7.8 validity range of the K/N gap joint formulas", "beta >= 0.35, 15 <= b_0/t_0 <= 35, b_i/t_i <= 35, 0.5(1-beta) <= g/b_0 <= 1.5(1-beta), g >= t_1+t_2 (the code encodes the older 15..35 slenderness window; the Table 7.8 window must be re-keyed)", "Table 7.8"),
 ("v-440.4", "7.4.2", "Table 7.11 k_n chord stress function", "k_n = 1.0 for n >= 0, k_n = 1.3 - 0.4 n/beta <= 1.0 for n < 0, n = sigma_0,Ed/f_y0", "Table 7.11 note"),
]
for k, cl, sym, note, loc_ in rows:
    put(d, "1993", k, cl, sym, None, None, "", note, part="1-8", locator=loc_)
    d["ndp.en1993." + k]["source"]["document"] = "EN 1993-1-8:2005"
    d["ndp.en1993." + k]["evidence"] = EV
put(d, "1993", "v-123.1", "6.3.4", "alpha_cr,op estimate", None, None, "", "alpha_cr,op estimated as 1/(N_Ed/N_cr,z + M_y,Ed/M_cr) (conservative interpolation; the standard asks for the minimum amplifier of the in-plane/out-of-plane combined buckling mode from a second-order or eigenvalue analysis)", part="1-1", locator="6.3.4(2)")
d["ndp.en1993.v-123.1"]["evidence"] = "Engineering estimate of the artifact (no table or normative formula); keep unverified until a buckling eigenvalue analysis replaces it."
put(d, "1993", "v-113b.1", "6.3.2.2", "W_y class dependent", None, None, "", "W_y = W_pl,y (class 1 and 2), W_el,y (class 3), W_eff,y (class 4) in the LTB resistance M_b,Rd = chi_LT W_y f_y/gamma_M1", part="1-1", locator="6.3.2.1(3)")
d["ndp.en1993.v-113b.1"]["evidence"] = "Formula reading of 6.3.2.1(3); no NDP value. Kept unverified until a human confirms the LT curve selection for the section family."
put(d, "1993", "v-420.2", "6.2.7.2", "group effect of bolt rows", None, None, "", "Effective lengths of rows as part of a group (end row cp = pi m + p, nc = 2m + 0.625e + 0.5p; inner row cp = 2p, nc = p; first row below the flange as part of a group) and the group T-stub of consecutive rows for 6.2.7.2(7)", part="1-8", locator="Tables 6.4 and 6.6, 6.2.7.2(7)")
d["ndp.en1993.v-420.2"]["source"]["document"] = "EN 1993-1-8:2005"
d["ndp.en1993.v-420.2"]["evidence"] = EV
put(d, "1993", "v-026.2", "5.2.1", "Eq. 5.2 validity", None, None, "", "Eq. 5.2 (Horne) accepted only within 5 percent of the eigenvalue alpha_cr of the modelled portal; the conditions of 5.2.1(4)B (roof slope up to 26 degrees, beam axial force N_Ed up to 0.09 N_cr) are not encoded", part="1-1", locator="5.2.1(4)B")
d["ndp.en1993.v-026.2"]["evidence"] = "The 5 percent threshold is an engineering tolerance of the artifact derived from the oracle spread (PyNite P-Delta and scipy eigen), not a normative value."
save(d)
print([r[0] for r in rows])
