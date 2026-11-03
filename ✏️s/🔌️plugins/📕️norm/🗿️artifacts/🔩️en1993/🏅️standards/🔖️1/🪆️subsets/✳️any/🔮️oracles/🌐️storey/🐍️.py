"""🌐️ Independent oracle of the EN 1993-1-1 §5 storey checks: stdlib formulas for alpha_cr, phi and q_d, and a scipy generalised eigenproblem (Hermite beam-column FE, guided fixed-base column) as third-party validation of the lateral-stiffness model behind alpha_cr. Run with `python3 -I`; prints one JSON object."""
import json
import math


def alpha_cr(h_ed, v_ed, h, drift):
    return (h_ed / v_ed) * (h / drift)


def sway_phi(h, m):
    alpha_h = min(max(2.0 / math.sqrt(h), 2.0 / 3.0), 1.0)
    return 0.005 * alpha_h * math.sqrt(0.5 * (1.0 + 1.0 / m))


def bracing_qd(sum_n, m, length, delta_q):
    e0 = math.sqrt(0.5 * (1.0 + 1.0 / m)) * length / 500.0
    return sum_n * 8.0 * (e0 + delta_q) / length**2


def fe_critical_load(ei, h, n_el=16):
    import numpy as np
    from scipy.linalg import eigh

    le = h / n_el
    ke = ei / le**3 * np.array([[12, 6 * le, -12, 6 * le], [6 * le, 4 * le**2, -6 * le, 2 * le**2], [-12, -6 * le, 12, -6 * le], [6 * le, 2 * le**2, -6 * le, 4 * le**2]])
    kg = 1.0 / (30.0 * le) * np.array([[36, 3 * le, -36, 3 * le], [3 * le, 4 * le**2, -3 * le, -(le**2)], [-36, -3 * le, 36, -3 * le], [3 * le, -(le**2), -3 * le, 4 * le**2]])
    n = 2 * (n_el + 1)
    k = np.zeros((n, n))
    g = np.zeros((n, n))
    for e in range(n_el):
        s = slice(2 * e, 2 * e + 4)
        k[s, s] += ke
        g[s, s] += kg
    free = [i for i in range(n) if i not in (0, 1, n - 1)]
    values = eigh(k[np.ix_(free, free)], g[np.ix_(free, free)], eigvals_only=True)
    return float(values[0])


def main():
    ei, h, v, h_ed = 210.0e9 * 3.923e-5, 4.0, 4.0e5, 2.0e4
    stiffness = 2.0 * 12.0 * ei / h**3
    drift = h_ed / stiffness
    n_cr_fe = fe_critical_load(ei, h)
    out = {
        "alpha_showcase": alpha_cr(2.0e4, 4.0e5, 4.0, 0.002),
        "util_showcase": 3.0 / alpha_cr(2.0e4, 4.0e5, 4.0, 0.002),
        "phi_h4_m2": sway_phi(4.0, 2),
        "phi_h4_m2_force": sway_phi(4.0, 2) * 4.0e5,
        "alpha_soft": alpha_cr(2.0e4, 4.0e5, 4.0, 0.1),
        "qd_6m": bracing_qd(1.0e6, 2, 6.0, 0.02),
        "n_cr_fe_over_euler": n_cr_fe / (math.pi**2 * ei / h**2),
        "alpha_portal_first_order": alpha_cr(h_ed, v, h, drift),
        "alpha_portal_fe": 2.0 * n_cr_fe / v,
    }
    out["portal_first_order_over_fe"] = out["alpha_portal_first_order"] / out["alpha_portal_fe"]
    print(json.dumps(out))


main()
