import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
EV = "Written from memory by the en1993 V-id agent; NOT double-keyed from the print edition. Only a human verification against the cited EN 1993 part may flip the status."
rows = [
 ("v-010.1", "3.2.1", "Table 3.1 nominal f_y and f_u (t <= 40 mm)", "EN 10025-2 hot rolled, nominal thickness up to 40 mm: S235 f_y 235 / f_u 360, S275 275 / 430, S355 355 / 510, S450 440 / 550 MPa; thicker products take the lower rows of Table 3.1, which are not encoded", "1-1", "Table 3.1"),
 ("v-011.1", "3.2.2", "Ductility f_u/f_y", "f_u/f_y >= 1.10, elongation at failure >= 15 %, epsilon_u >= 15 epsilon_y; only the ratio f_u/f_y is evaluated because the material row carries no elongation", "1-1", "3.2.2(1)"),
 ("v-013.1", "3.2.6", "Design values of constants", "E = 210000 MPa, G = E/(2(1+nu)) rounded to 81000 MPa, nu = 0.3; the 1 percent agreement tolerance of the check is an engineering value of the artifact", "1-1", "3.2.6(1)"),
 ("v-024.1", "5.3.2(6)", "Bow imperfection inclusion rule", "Member imperfections e_0 are included in the global analysis where lambda_bar > 0.5 sqrt(A f_y/N_Ed) and at least one end joint is moment resistant; e_0/L from Table 5.1 (registered separately); the check assumes a moment-resistant joint at one end", "1-1", "5.3.2(6)"),
 ("v-050.1", "6.2.6(3)", "Shear area A_v of rolled and welded I sections", "A_v = A - 2 b t_f + (t_w + 2 r) t_f >= eta h_w t_w (rolled I), eta sum(h_w t_w) (welded I), eta = 1.2 for S460 and below; the 3 percent agreement tolerance against the stored shear area is an engineering value of the artifact", "1-1", "6.2.6(3)"),
 ("v-091.1", "7.2.3", "Floor natural frequency limit", "f_1 = pi/(2 L^2) sqrt(E I/m) >= 3 Hz (recommended lower limit); m from the permanent member actions of a simply supported beam under uniform load (w = 8 M/L^2); the limit is an NDP, the National Annex may set another value", "1-1", "7.2.3"),
 ("v-101.1", "6.3.1.4", "Torsional buckling length and curve", "Torsional buckling length l_T taken as the minor-axis buckling length; buckling curve of the z-z axis (6.3.1.4(2)); N_cr,T = (G I_t + pi^2 E I_w/l_T^2)/i_0^2 with i_0^2 = i_y^2 + i_z^2 for doubly symmetric sections", "1-1", "6.3.1.4"),
 ("v-112.1", "6.3.2.4(1)B", "Simplified LTB method k_fl and lambda_c0", "lambda_c0 = lambda_LT,0 + 0.1 (lambda_LT,0 = 0.4 recommended), k_c from Table 6.6, k_fl = 1.10 recommended; the check uses the compression flange plus 1/6 of the web as equivalent flange for double symmetric class 1 and 2 sections", "1-1", "6.3.2.4"),
 ("v-402.1", "3.6", "Punching shear of bolt heads and nuts", "B_p,Rd = 0.6 pi d_m t_p f_u/gamma_M2 with d_m the mean of across-points and across-flats of head or nut; across-flats approximated by 1.5 d (ISO 4014 for M16 to M24), across-points by 1.155 times that", "1-8", "Table 3.4"),
 ("v-405.1", "3.10.2", "Block tearing geometry", "Concentric loading V_eff,1,Rd = f_u A_nt/gamma_M2 + f_y A_nv/(sqrt(3) gamma_M0); the net tension face runs between the outer bolt lines ((n_2 - 1)(p_2 - d_0) t), the net shear faces along both outer bolt lines (2 (e_1 + (n_1 - 1) p_1 - (n_1 - 1/2) d_0) t); one bolt column: one shear face and the edge distance e_2 for the tension face", "1-8", "3.10.2"),
 ("v-413.1", "4.10", "Effective width of a plate welded to an unstiffened flange", "b_eff = t_w + 2 s + 7 k t_f with k = (t_f/t_p)(f_y,f/f_y,p) <= 1, b_eff <= b; the joint must be stiffened if b_eff < (f_y,p/f_u,p) b_p; s = sqrt(2) times the weld throat", "1-8", "4.10"),
 ("v-607.1", "4.2.3.3", "Lateral-torsional buckling in fire", "M_b,fi,t,Rd = chi_LT,fi W_pl,y k_y,theta,com f_y/gamma_M,fi with lambda_LT,theta,com = lambda_LT sqrt(k_y,theta,com/k_E,theta,com), alpha = 0.65 sqrt(235/f_y), gamma_M,fi = 1.0; the compression flange temperature is taken equal to theta_a; M_fi,Ed = eta_fi M_Ed with eta_fi = 0.65 as in the existing buckling check", "1-2", "4.2.3.3"),
 ("v-608.1", "4.2.3.5", "Bending and axial compression in fire", "Eq. 4.21a with k_y = 1 - mu_y N_fi,Ed/(chi_y,fi A k_y,theta f_y/gamma_M,fi) <= 3, mu_y = (1.2 beta_M,y - 3) lambda_y,theta + 0.44 beta_M,y + 0.29 <= 0.8, lambda_y,theta <= 1.1, beta_M,y = 1.3 (uniform load) and k_LT = 1 - mu_LT N_fi,Ed/(chi_z,fi A k_y,theta f_y/gamma_M,fi) <= 1, mu_LT = 0.15 lambda_z,theta beta_M,LT - 0.15 <= 0.9 (strong-axis moment only)", "1-2", "4.2.3.5"),
]
for k, cl, sym, note, part, loc_ in rows:
    put(d, "1993", k, cl, sym, None, None, "", note, part=part, locator=loc_)
    d["ndp.en1993." + k]["evidence"] = EV
save(d)
print([r[0] for r in rows])
