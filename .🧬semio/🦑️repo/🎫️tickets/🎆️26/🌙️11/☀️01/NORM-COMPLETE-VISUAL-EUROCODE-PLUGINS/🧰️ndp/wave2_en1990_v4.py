import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
N = "wave 2 v4, UNVERIFIED: value recalled from the standard, not checked against the printed edition"
P = "wave 2 v4, UNVERIFIED PLACEHOLDER: structure of the rule recalled, magnitude is a modelling choice"
rows = [
    ("a1.override.gamma.allowed", "A1.3", "project γ override allowed", 1.0, 0.0, "", f"1 = a project may replace a partial factor γ (with justification), 0 = only the National Annex sets it ({P})"),
    ("a1.override.psi.allowed", "A1.1", "project ψ override allowed", 1.0, 1.0, "", f"1 = a project may replace ψ0, ψ1, ψ2 (with justification) ({P})"),
    ("a1.override.xi.allowed", "A1.3", "project ξ override allowed", 1.0, 0.0, "", f"1 = a project may replace the reduction factor ξ ({P})"),
    ("a1.override.k_fi.allowed", "A1.3", "project K_FI override allowed", 0.0, 0.0, "", f"1 = a project may replace K_FI; 0 = K_FI follows the reliability class only ({P})"),
    ("e90-12.alpha_cr_first_order", "5.1", "α_cr first-order limit", 10.0, 10.0, "", f"First-order analysis admissible when α_cr is at least this value; owned by the material parts (EN 1993-1-1 5.2.1, elastic) ({N})"),
    ("a2.rail.f_range_min", "6.4.4 EN 1991-2", "span range min", 4.0, 4.0, "m", f"Fig. 6.10 span range lower end ({N})"),
    ("a2.rail.f_range_max", "6.4.4 EN 1991-2", "span range max", 100.0, 100.0, "m", f"Fig. 6.10 span range upper end ({N})"),
    ("a2.rail.f_lower_break", "6.4.4 EN 1991-2", "lower limit break span", 20.0, 20.0, "m", f"Fig. 6.10 lower limit changes formula at this span ({N})"),
    ("a2.rail.f_lower_short", "6.4.4 EN 1991-2", "n0 lower limit, short spans", 80.0, 80.0, "m/s", f"n0,lower = value / L for 4 m <= L <= 20 m ({N})"),
    ("a2.rail.f_lower_a", "6.4.4 EN 1991-2", "n0 lower limit coefficient", 23.58, 23.58, "", f"n0,lower = a * L^b for 20 m < L <= 100 m ({N})"),
    ("a2.rail.f_lower_b", "6.4.4 EN 1991-2", "n0 lower limit exponent", -0.592, -0.592, "", f"n0,lower = a * L^b for 20 m < L <= 100 m ({N})"),
    ("a2.rail.f_upper_a", "6.4.4 EN 1991-2", "n0 upper limit coefficient", 94.76, 94.76, "", f"n0,upper = a * L^b for 4 m <= L <= 100 m ({N})"),
    ("a2.rail.f_upper_b", "6.4.4 EN 1991-2", "n0 upper limit exponent", -0.748, -0.748, "", f"n0,upper = a * L^b for 4 m <= L <= 100 m ({N})"),
    ("a2.rail.damping_default", "6.4.6.3 EN 1991-2", "default damping ratio", 0.01, 0.01, "", f"Deck damping ratio used when none is declared ({P})"),
]
hslm = {1: (18, 18.0, 2.0, 170e3), 2: (17, 19.0, 3.5, 200e3), 3: (16, 20.0, 2.0, 180e3), 4: (15, 21.0, 3.0, 190e3), 5: (14, 22.0, 2.0, 170e3), 6: (13, 23.0, 2.0, 180e3), 7: (13, 24.0, 2.0, 190e3), 8: (12, 25.0, 2.5, 190e3), 9: (11, 26.0, 2.0, 210e3), 10: (11, 27.0, 2.0, 210e3)}
for k, (n, dc, da, p) in hslm.items():
    rows += [
        (f"a2.rail.hslm.a{k}.coaches", "Annex E EN 1991-2", f"HSLM-A{k} coaches", float(n), float(n), "", f"Number of coaches of the universal train ({N})"),
        (f"a2.rail.hslm.a{k}.coach_length", "Annex E EN 1991-2", f"HSLM-A{k} coach length D", dc, dc, "m", f"Distance between bogie centres of consecutive coaches ({N})"),
        (f"a2.rail.hslm.a{k}.axle_spacing", "Annex E EN 1991-2", f"HSLM-A{k} axle spacing d", da, da, "m", f"Axle spacing within a bogie ({N})"),
        (f"a2.rail.hslm.a{k}.axle_load", "Annex E EN 1991-2", f"HSLM-A{k} axle load P", p, p, "N", f"Point load per axle; the power cars of the real train are not modelled ({N})"),
    ]
for key, clause, symbol, en, de, unit, note in rows:
    put(d, "1990", key, clause, symbol, en, de, unit, note)
save(d)
print(len(rows))
