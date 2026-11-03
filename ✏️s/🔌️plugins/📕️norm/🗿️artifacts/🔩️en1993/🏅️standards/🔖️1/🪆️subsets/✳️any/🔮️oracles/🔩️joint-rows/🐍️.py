"""🔩️ Oracle of the end plate component method (EN 1993-1-8 6.2, 6.3) for the two explicit rows of the showcase joint: the T-stub modes are recomputed here by evaluating the three failure mode formulas separately and taking the minimum, the plastic row distribution by explicit top-down capping, S_j,ini from the Table 6.11 coefficients. Group effects, the beam web in tension and the stiffened column web are not modelled (same scope as the Rust clause). No third-party library covers bolted end plate joints (see oracle-library-coverage). Run with `python3 -I`."""
import json
import math

E = 210000.0
GM0, GM1, GM2 = 1.0, 1.1, 1.25
col = dict(tfc=17.0, twc=10.0, dc=164.0, avc=2400.0, rc=21.0, fy=355.0, w=110.0, e=40.0, sigma=50.0, beta=1.0, tfb=10.7, ab=6.0)
plate = dict(tp=20.0, bp=200.0, fy=355.0)
bolt = dict(d=20.0, a_s=245.0, lb=50.0, fub=1000.0, n=2.0)
rows = [dict(kind="extended", lever=270.0, m=40.0, e=40.0, mx=40.0, ex=40.0, alpha=6.5), dict(kind="below", lever=180.0, m=40.0, e=40.0, mx=0.0, ex=0.0, alpha=6.5)]
M_BEAM_PL, M_COL_PL, M_ED = 110000.0, 200000.0, 80000.0
IB, LB = 8.356e-5 * 1e12, 6000.0


def tstub(l1, l2, tf, fy, m, n, ew, sum_ft, a_s, nb, lb):
    mp1, mp2 = 0.25 * l1 * tf**2 * fy / GM0, 0.25 * l2 * tf**2 * fy / GM0
    lb_star = 8.8 * m**3 * a_s * nb / (l1 * tf**3)
    prying = lb <= lb_star
    if prying:
        modes = [4 * mp1 / m, (2 * mp2 + n * sum_ft) / (m + n), sum_ft]
    else:
        modes = [2 * mp1 / m, math.inf, sum_ft]
    mode = modes.index(min(modes)) + 1
    return min(modes), mode


m_c = (col["w"] - col["twc"]) / 2 - 0.8 * col["rc"]
l_col = (min(2 * math.pi * m_c, 4 * m_c + 1.25 * col["e"]), 4 * m_c + 1.25 * col["e"])
sum_ft = bolt["n"] * 0.9 * bolt["fub"] * bolt["a_s"] / GM2
ew = 0.25 * 1.6 * bolt["d"]
col_ft, col_mode = tstub(l_col[0], l_col[1], col["tfc"], col["fy"], m_c, min(col["e"], 1.25 * m_c), ew, sum_ft, bolt["a_s"], bolt["n"], bolt["lb"])
b_eff_t = l_col[0] if col_mode == 1 else l_col[1]
x = (b_eff_t * col["twc"] / col["avc"]) ** 2
omega = 1.0 / math.sqrt(1 + 1.3 * x)
ft_wc = omega * b_eff_t * col["twc"] * col["fy"] / GM0
b_eff_c = col["tfb"] + 2 * math.sqrt(2) * col["ab"] + 5 * (col["tfc"] + col["rc"])
xc = (b_eff_c * col["twc"] / col["avc"]) ** 2
omega_c = 1.0 / math.sqrt(1 + 1.3 * xc)
kwc = 1.0 if col["sigma"] <= 0.7 * col["fy"] else 1.7 - col["sigma"] / col["fy"]
lam_p = 0.932 * math.sqrt(b_eff_c * col["dc"] * col["fy"] / (E * col["twc"] ** 2))
rho = 1.0 if lam_p <= 0.72 else (lam_p - 0.2) / lam_p**2
fc_wc = min(omega_c * kwc * rho * b_eff_c * col["twc"] * col["fy"] / GM1, omega_c * kwc * b_eff_c * col["twc"] * col["fy"] / GM0)
v_wp = 0.9 * col["fy"] * col["avc"] / (math.sqrt(3) * GM0) / col["beta"]
f_beam = M_BEAM_PL * 1e3 / rows[0]["lever"]
iso, ks, plate_modes = [], [], []
for r in rows:
    if r["kind"] == "extended":
        cp = min(2 * math.pi * r["mx"], math.pi * r["mx"] + col["w"], math.pi * r["mx"] + 2 * r["e"])
        nc = min(4 * r["mx"] + 1.25 * r["ex"], r["e"] + 2 * r["mx"] + 0.625 * r["ex"], 0.5 * plate["bp"], 0.5 * col["w"] + 2 * r["mx"] + 0.625 * r["ex"])
        m_use = r["mx"]
        n_p = min(r["e"], r["ex"])
    else:
        cp, nc = 2 * math.pi * r["m"], r["alpha"] * r["m"]
        m_use = r["m"]
        n_p = r["e"]
    ft_pl, mode_pl = tstub(min(cp, nc), nc, plate["tp"], plate["fy"], m_use, n_p, ew, sum_ft, bolt["a_s"], bolt["n"], bolt["lb"])
    plate_modes.append(mode_pl)
    iso.append(min(col_ft, ft_pl, ft_wc))
    k3 = 0.7 * b_eff_t * col["twc"] / col["dc"]
    k4 = 0.9 * l_col[0] * col["tfc"] ** 3 / m_c**3
    k5 = 0.9 * min(cp, nc) * plate["tp"] ** 3 / m_use**3
    k10 = 1.6 * bolt["a_s"] * bolt["n"] / bolt["lb"]
    ks.append(1.0 / (1 / k3 + 1 / k4 + 1 / k5 + 1 / k10))
cap = min(fc_wc, v_wp, f_beam)
forces, total = [], 0.0
for value in iso:
    f = max(0.0, min(value, cap - total))
    forces.append(f)
    total += f
levers = [r["lever"] for r in rows]
m_rd = sum(h * f for h, f in zip(levers, forces)) / 1e3
z = sum(k * h * h for k, h in zip(ks, levers)) / sum(k * h for k, h in zip(ks, levers))
k_eq = sum(k * h for k, h in zip(ks, levers)) / z
k1 = 0.38 * col["avc"] / (col["beta"] * z)
k2 = 0.7 * b_eff_c * col["twc"] / col["dc"]
s_ini = E * z * z / (1 / k1 + 1 / k2 + 1 / k_eq) / 1e3
rigid, pinned = 8 * E * IB / LB / 1e3, 0.5 * E * IB / LB / 1e3
cls = "pinned" if s_ini <= pinned else "rigid" if s_ini >= rigid else "semiRigid"
mu = 1.0 if M_ED <= 2 / 3 * m_rd else (1.5 * M_ED / m_rd) ** 2.7
print(json.dumps({"col_ft_rd": col_ft, "col_mode": col_mode, "ft_wc": ft_wc, "fc_wc": fc_wc, "v_wp": v_wp, "f_beam": f_beam, "comp_cap": cap, "comp_demand": M_ED * 1e3 / z, "comp_util": M_ED * 1e3 / z / cap, "ft_wb_example": 210.0 * 7.1 * 355.0 / GM0, "iso": iso, "plate_modes": plate_modes, "forces": forces, "m_j_rd_nm": m_rd, "z_mm": z, "s_ini_nm_per_rad": s_ini, "rigid_nm_per_rad": rigid, "pinned_nm_per_rad": pinned, "stiffness": cls, "mu": mu, "phi_ed_rad": M_ED * mu / s_ini}))
