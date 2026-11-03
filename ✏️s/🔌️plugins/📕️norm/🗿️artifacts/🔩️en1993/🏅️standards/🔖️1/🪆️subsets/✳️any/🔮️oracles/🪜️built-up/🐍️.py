"""🪜️ Independent stdlib oracle of the EN 1993-1-1 6.4 built-up member check (chi by the closed Phi form, Table 6.9 limits). Run with `python3 -I`; prints one JSON object for the showcase row (battened, 2 planes, HEA 200 chords)."""
import json
import math


def battened(length, h0, a, a_ch, i_ch, i_b, e, fy, gm1, n, n_ed, m_first):
    i1 = 0.5 * h0 * h0 * a_ch + 2 * i_ch
    lam = length / math.sqrt(i1 / (2 * a_ch))
    mu = 0.0 if lam >= 150 else (2 - lam / 75 if lam > 75 else 1.0)
    i_eff = 0.5 * h0 * h0 * a_ch + 2 * mu * i_ch
    n_cr = math.pi**2 * e * i_eff / length**2
    sv = min(24 * e * i_ch / (a * a * (1 + 2 * i_ch / (n * i_b) * h0 / a)), 2 * math.pi**2 * e * i_ch / a**2)
    m_ed = (n_ed * length / 500 + m_first) / (1 - n_ed / n_cr - n_ed / sv)
    n_ch = 0.5 * n_ed + m_ed * h0 * a_ch / (2 * i_eff)
    lc = (a / math.sqrt(i_ch / a_ch)) / (93.9 * math.sqrt(235 / fy))
    phi = 0.5 * (1 + 0.34 * (lc - 0.2) + lc * lc)
    chi = 1 / (phi + math.sqrt(phi * phi - lc * lc))
    n_b = chi * a_ch * fy / gm1
    return {"lambda": lam, "mu": mu, "i_eff": i_eff, "n_cr": n_cr, "sv": sv, "m_ed_nmm": m_ed, "n_ch_ed": n_ch, "lambda_chord": lc, "chi": chi, "n_b_rd": n_b, "utilisation": n_ch / n_b}


print(json.dumps({"showcase": battened(4000.0, 300.0, 800.0, 5380.0, 1.336e7, 5.0e6, 210000.0, 355.0, 1.1, 2.0, 400000.0, 5.0e6), "closely_limit_m": {"angles": 15 * 0.04, "other": 70 * 0.04}}))
