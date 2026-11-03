"""Reference for catalogue::crack_width (EN 1992-1-1 7.3.4 crack width of the hogging zone) with structuralcodes: python -I en1994-crack-width-oracle.py

Case of the Rust test: sigma_s = 240 MPa, bar 16 mm at s = 150 mm (A_s = 1340 mm2/m), cover to the bar surface c = 32 mm, h_c,eff = 100 mm, f_ctm = 2.9 MPa, E_cm = 33 GPa.
"""
import math

from structuralcodes.codes.ec2_2004 import eps_sm_eps_cm, sr_max_close

es, ecm, fct = 210e9, 33e9, 2.9e6
sigma, phi, c, rho = 240e6, 0.016, 0.032, 0.001340 / 0.1
eps = eps_sm_eps_cm(sigma, es / ecm, rho, 0.4, fct, es)
sr = sr_max_close(c * 1e3, phi * 1e3, rho, 0.8, 0.5, 3.4, 0.425) * 1e-3
print({"eps_sm_minus_eps_cm": eps, "s_r_max_m": sr, "w_k_m": eps * sr})
