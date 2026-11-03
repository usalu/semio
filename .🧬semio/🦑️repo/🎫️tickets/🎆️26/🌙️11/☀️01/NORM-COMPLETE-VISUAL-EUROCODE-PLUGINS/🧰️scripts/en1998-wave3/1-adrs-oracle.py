"""Independent numpy/scipy check of the N2 method in ADRS format (EN 1998-1 Annex B) for a two-storey shear building: eigenmode (numpy.linalg.eigh), Gamma and m*, elastic-plastic pushover, T*, q_u, d_et*, d_t*, and the identity of the radial period line through the yield point. The printed literals are the ones of `🎬️scene/🌋️capacity/🧪️tests/🦀️.rs` and `📐️formulas/🧪️tests`."""
import numpy as np
from scipy.linalg import eigh

m = np.array([100e3, 100e3]); k = np.array([2.0e7, 1.0e7]); res = np.array([1.2e6, 0.4e6])
K = np.array([[k[0] + k[1], -k[1]], [-k[1], k[1]]]); M = np.diag(m)
w2, vec = eigh(K, M); phi = vec[:, 0] / vec[-1, 0]
gamma = (m @ phi) / (m @ phi**2); m_star = m @ phi
s = np.array([m[0] * phi[0] + m[1] * phi[1], m[1] * phi[1]])
lam = min(res / s); d_roof = float(np.sum(lam * s / k)); v_y = lam * s[0]
d_y, f_y = d_roof / gamma, v_y / gamma
t_star = 2 * np.pi * np.sqrt(m_star * d_y / f_y)
a_g, S, tb, tc, td = 0.25 * 9.81, 1.2, 0.15, 0.5, 2.0

def se(t, eta=1.0):
    if t <= tb: return a_g * S * (1 + t / tb * (2.5 * eta - 1))
    if t <= tc: return a_g * S * 2.5 * eta
    if t <= td: return a_g * S * 2.5 * eta * tc / t
    return a_g * S * 2.5 * eta * tc * td / t**2

sa_e = se(t_star); q_u = sa_e * m_star / f_y
d_et = sa_e * (t_star / (2 * np.pi))**2
d_t = d_et if (t_star >= tc or q_u <= 1) else d_et / q_u * (1 + (q_u - 1) * tc / t_star)
sa_y = f_y / m_star
radial = (2 * np.pi / t_star)**2
assert np.isclose(sa_y, radial * d_y, rtol=1e-12)
assert np.isclose(sa_e, radial * d_et, rtol=1e-12)
print(dict(phi0=phi[0], gamma=gamma, m_star=m_star, d_y_roof=d_roof, v_y=v_y, d_y_star=d_y, f_y_star=f_y, t_star=t_star, sa_e=sa_e, q_u=q_u, d_et=d_et, d_t=d_t, sa_y=sa_y))
