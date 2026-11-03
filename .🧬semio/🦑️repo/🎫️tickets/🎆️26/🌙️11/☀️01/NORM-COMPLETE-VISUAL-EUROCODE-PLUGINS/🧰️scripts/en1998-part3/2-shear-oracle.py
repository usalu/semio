"""Independent numpy check of the worked example of the cyclic shear capacity (EN 1998-3 A.3.3, spec AS-06) and of the strengths from tests (AS-03): the literals of `📐️formulas/🏚️assessment-state/🧪️tests/🦀️.rs` (python3 -I 2-shear-oracle.py). No third-party library implements EN 1998-3 (see `🔍️audits/oracle-library-coverage.md`), so numpy evaluates the closed form on vectors independently of the Rust kernel."""
import numpy as np

b, h, d, lv, n, gamma_el = 0.3, 0.5, 0.46, 1.75, 3.0e5, 1.0
cf, gamma_c, gamma_s = 1.2, 1.5, 1.15
fc, fyw = 30.0 / (cf * gamma_c), 500.0 / (cf * gamma_s)
ac, x, mu_pl = b * h, 0.1, 2.0
rho_tot = (0.0012 + 0.0008) / ac
terms = np.array([
    (h - x) / (2 * lv) * min(n, 0.55 * ac * fc * 1e6),
    0.16 * max(0.5, 100 * rho_tot) * (1 - 0.16 * min(5.0, lv / h)) * np.sqrt(fc) * ac * 1e6,
    (1.0e-4 / (b * 0.1)) * b * (2 * d - h) * fyw * 1e6,
])
degradation = 1 - 0.05 * min(5.0, mu_pl)
v_r = (terms[0] + degradation * terms[1:].sum()) / gamma_el
expected = np.array([34285.714285714, 57481.3592979, 152173.913043, 0.9, 222975.459392])
got = np.array([*terms, degradation, v_r])
assert np.allclose(got, expected, rtol=1e-6), (got, expected)
assert np.isclose(30.0 / cf, 25.0) and np.isclose(30.0 / (cf * gamma_c), 30.0 / 1.8)
print("cyclic shear worked example ok", got)
