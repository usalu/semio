"""🧩️ Independent oracle of the joint plate and foundation block worked examples of the EN 1993 showcase: closed-form bolt group, block tearing, directional weld, grout and concrete bearing and anchors (never reusing the Rust expressions), with the bolt shear and bearing resistance of Table 3.4 cross-checked against the third-party `blue-prints` (EN 1993-1-8:2005 Table 3.4). Run with `python -I 🐍️.py`; it prints the JSON that the Rust test `joint_plates_and_foundation_blocks_match_the_python_oracle` pins."""
import json
import math

import numpy as np

from blueprints.codes.eurocode.en_1993_1_8_2005.chapter_3_connections_made_with_bolts_rivets_or_pins.table_3_1 import FastenerClass
from blueprints.codes.eurocode.en_1993_1_8_2005.chapter_3_connections_made_with_bolts_rivets_or_pins.table_3_4 import ShearPlane, Table3Dot4BearingResistance, Table3Dot4ShearResistanceBolt

GM0, GM2, GC = 1.0, 1.25, 1.5
FUB, AS20 = 800.0, 245.0


def group(points, fx, fy, mz):
    ip = sum(x * x + y * y for x, y in points)
    k = mz / ip if ip else 0.0
    n = len(points)
    return max(math.hypot(fx / n - k * y, fy / n + k * x) for x, y in points)


def rows(n1, n2, p1, p2):
    return [((c - (n2 - 1) / 2) * p2, (r - (n1 - 1) / 2) * p1) for r in range(n1) for c in range(n2)]


def fin_plate():
    V, z, t, fu, fy, d, d0, e1, e2, p1 = 30e3, 60.0, 10.0, 470.0, 355.0, 20.0, 22.0, 40.0, 40.0, 70.0
    f_ed = group(rows(3, 1, p1, 0.0), 0.0, V, V * z)
    fv = Table3Dot4ShearResistanceBolt(f_ub=FUB, a=AS20, bolt_class=FastenerClass.CLASS_8_8, shear_plane=ShearPlane.THREADED, gamma_m2=GM2)
    alpha_end = min(e1 / (3 * d0), FUB / fu, 1.0)
    alpha_inner = min(p1 / (3 * d0) - 0.25, FUB / fu, 1.0)
    alpha_b = min(alpha_end, alpha_inner)
    k1 = min(2.8 * e2 / d0 - 1.7, 2.5)
    fb = Table3Dot4BearingResistance(k_1=k1, alpha_b=alpha_b, f_u=fu, d=d, t=t, gamma_m2=GM2)
    fv_rd, fb_rd = float(fv), float(fb)
    assert abs(fv_rd - 0.6 * FUB * AS20 / GM2) < 1e-6 and abs(fb_rd - k1 * alpha_b * fu * d * t / GM2) < 1e-6
    lv = e1 + 2 * p1 - 2.5 * d0
    a_nt, a_nv = (e2 - d0 / 2) * t, lv * t
    block = 0.5 * fu * a_nt / GM2 + fy * a_nv / (math.sqrt(3) * GM0)
    a, L = 5.0, 240.0
    sigma = V * z / (a * L * L / 3) / math.sqrt(2)
    tau = V / (2 * a * L)
    equiv = math.sqrt(sigma**2 + 3 * (sigma**2 + tau**2))
    weld = max(equiv / (470.0 / (0.9 * GM2)), sigma / (0.9 * 470.0 / GM2))
    return {"fin_f_ed": f_ed, "fin_fv_rd": fv_rd, "fin_fb_rd": fb_rd, "fin_bolt_util": f_ed / min(fv_rd, fb_rd), "fin_block_rd": block, "fin_block_util": V / block, "fin_weld_util": weld}


def flange_cover():
    M, h, tf, t = 80e3, 0.240, 0.017, 14.0
    F = M / (h - tf)
    pts = rows(3, 2, 70.0, 100.0)
    f_ed = group(pts, 0.0, F, 0.0)
    fu, d, d0, e1, e2, p1, p2 = 470.0, 20.0, 22.0, 50.0, 50.0, 70.0, 100.0
    alpha = min(min(e1 / (3 * d0), FUB / fu, 1.0), min(p1 / (3 * d0) - 0.25, FUB / fu, 1.0))
    k1 = min(min(2.8 * e2 / d0 - 1.7, 1.4 * p2 / d0 - 1.7), 2.5)
    fb = k1 * alpha * fu * d * t / GM2
    fv = 0.6 * FUB * AS20 / GM2
    net = 0.9 * (200.0 - 2 * d0) * t * fu / GM2
    gross = 200.0 * t * 355.0 / GM0
    return {"flange_force": F, "flange_f_ed": f_ed, "flange_bolt_util": f_ed / min(fv, fb), "flange_net_util": F / min(net, gross)}


def tstub(tp, fyp, ft):
    m, e, as_, n, lb = 50.0, 50.0, 353.0, 2, 200.0
    l1 = l2 = min(4 * m + 1.25 * e, 2 * math.pi * m)
    mp1 = mp2 = 0.25 * l1 * tp**2 * fyp / GM0
    sum_ft = n * ft
    n_p = min(e, 1.25 * m)
    prying = lb <= 8.8 * m**3 * as_ * n / (l1 * tp**3)
    f1, f2 = (4 * mp1 / m, (2 * mp2 + n_p * sum_ft) / (m + n_p)) if prying else (2 * mp1 / m, math.inf)
    ft_rd = min(f1, f2, sum_ft)
    return ft_rd, prying, 1 if ft_rd == f1 else 2 if ft_rd == f2 else 3


def required_c(tfc, bfc, bp, area):
    lo, hi = 0.0, 1e4
    for _ in range(200):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if (tfc + 2 * mid) * min(bfc + 2 * mid, bp) < area else (lo, mid)
    return hi if (tfc * min(bfc, bp)) < area else 0.0


def strip(lp, bp, tp, fyp, tfc, bfc, a, w, h, fcd):
    c = tp
    for _ in range(8):
        b1, l1 = min(tfc + 2 * c, lp), min(bfc + 2 * c, bp)
        b2, d2 = min(a, b1 + h, 3 * b1), min(w, l1 + h, 3 * l1)
        f_rdu = min(b1 * l1 * fcd * math.sqrt(b2 * d2 / (b1 * l1)), 3 * fcd * b1 * l1)
        fjd = 2 / 3 * f_rdu / (b1 * l1)
        c = tp * math.sqrt(fyp / (3 * fjd * GM0))
    return fjd, b1, l1, c


def wide_plate():
    fcd, C = 30.0 / GC, 350e3
    fjd, b1, l1, c = strip(350.0, 450.0, 30.0, 355.0, 17.0, 240.0, 900.0, 900.0, 800.0, fcd)
    c_req = required_c(17.0, 240.0, 450.0, C / fjd)
    return {"wide_fjd": fjd, "wide_b_eff": b1, "wide_l_eff": l1, "wide_c": c, "wide_fc_rd": fjd * b1 * l1, "wide_base_util": C / (fjd * b1 * l1), "wide_cbend_util": c_req / c}


def all_compression():
    N, M, zl, zr = -300e3, 20e3, 0.15, 0.15
    c_left, c_right = np.linalg.solve([[1.0, 1.0], [-zl, zr]], [-N, M])
    fcd = 30.0 / GC
    fjd, b1, l1, c = strip(350.0, 300.0, 30.0, 355.0, 17.0, 240.0, 900.0, 900.0, 800.0, fcd)
    return {"allc_c_left": c_left, "allc_c_right": c_right, "allc_base_util": max(c_left, c_right) / (fjd * b1 * l1)}


def foundation():
    N, M, zl, zr, V = -300e3, 60e3, 0.15, 0.15, 30e3
    T = (M + N * zr) / (zl + zr)
    C = T - N
    lp, bp, tp, fyp, tfc, bfc = 350.0, 300.0, 30.0, 355.0, 17.0, 240.0
    a, w, h, fck = 900.0, 900.0, 800.0, 30.0
    fcd = fck / GC
    fjd, b1, l1, c = strip(lp, bp, tp, fyp, tfc, bfc, a, w, h, fcd)
    fc = fjd * b1 * l1
    ft = 0.9 * FUB * 353.0 / GM2
    alpha = 0.44 - 0.0003 * 640.0
    fv = 2 * alpha * FUB * 353.0 / GM2
    c_final = tp * math.sqrt(fyp / (3 * fjd * GM0))
    c_req = required_c(tfc, bfc, bp, C / fjd)
    b_req = tfc + 2 * c_req
    t_req = tp * c_req / c_final
    cbend = t_req / tp if b_req <= lp else b_req / lp
    ft_rd, prying, mode = tstub(tp, fyp, ft)
    ft_thin, prying_thin, mode_thin = tstub(12.0, fyp, ft)
    ft_side = min(ft_rd, 2 * ft)
    hi, lo = min(ft_side, N + fc), max(-fc, N - ft_side)
    m_rd = (N * zr - lo * (zl + zr)) * 1e3
    cone_k1, cone_fck, hef, c1 = 8.9, 30.0, 400.0, 200.0
    ccr = 1.5 * hef
    n0 = cone_k1 * math.sqrt(cone_fck) * hef**1.5
    n_rk_c = n0 * (min(c1, ccr) + ccr) / (2 * ccr) * min(0.7 + 0.3 * min(c1, ccr) / ccr, 1.0) * min(0.5 + hef / 200, 1.0)
    cone_rd = n_rk_c / (GC * 1.0)
    extra = {"base_c_strip": c_final, "cbend_util": cbend, "tbend_ft_rd": ft_rd, "tbend_prying": prying, "tbend_mode": mode, "thin_ft_rd": ft_thin, "thin_prying": prying_thin, "thin_mode": mode_thin, "thin_util": T / ft_thin, "tbend_util": T / ft_rd, "mjrd_m_rd": m_rd / 1e3, "mjrd_util": M / (m_rd / 1e3), "cone_n_rk_c": n_rk_c, "cone_util": T / 2 / cone_rd}
    return {**extra, "anchor_t": T, "base_c": C, "base_fjd": fjd, "base_b_eff": b1, "base_l_eff": l1, "base_fc_rd": fc, "base_util": C / fc, "anchor_t_util": T / 2 / ft, "anchor_v_util": V / fv, "grout_util": max(30.0 / (0.2 * 300.0), 0.2 * 30.0 / 40.0)}


print(json.dumps({**fin_plate(), **flange_cover(), **foundation(), **wide_plate(), **all_compression()}, indent=1))
