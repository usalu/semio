"""EN 1998 wave 3d: stand-alone numpy/scipy/groundhog worked examples of the new kernels (isolation, soil-structure springs, liquefaction hazard, pile in liquefied soil). Prints the values the Rust unit tests and the feature rows use.
Usage: /home/deck/semio/.venv/bin/python -I 1-hand-values.py"""
import json, math
import numpy as np
from scipy.optimize import brentq
from scipy.linalg import eigh_tridiagonal

G = 9.81
out = {}

def spectrum(a_gs_ag, s, tb, tc, td, t, eta):
    ags = a_gs_ag * s
    if t <= tb: return ags * (1 + t / tb * (2.5 * eta - 1))
    if t <= tc: return ags * 2.5 * eta
    if t <= td: return ags * 2.5 * eta * tc / t
    return ags * 2.5 * eta * tc * td / t ** 2

def isolation(q, kd, dy, mass, ag, s, tb, tc, td):
    k1 = kd + q / dy
    force = lambda d: k1 * d if d <= dy else q + kd * d
    def state(d):
        keff = force(d) / d
        t = 2 * math.pi * math.sqrt(mass / keff)
        ed = 4 * q * max(d - dy, 0.0)
        xi = ed / (2 * math.pi * keff * d * d)
        eta = max(math.sqrt(10 / (5 + 100 * xi)), 0.55)
        return keff, t, xi, eta
    def residual(d):
        keff, t, xi, eta = state(d)
        return force(d) - mass * spectrum(ag, s, tb, tc, td, t, eta)
    d = brentq(residual, 1e-6, 10.0, xtol=1e-14, rtol=1e-14)
    keff, t, xi, eta = state(d)
    se = spectrum(ag, s, tb, tc, td, t, eta)
    assert abs(d - se * (t / (2 * math.pi)) ** 2) < 1e-9
    return dict(d_cd=d, k_eff=keff, t_eff=t, xi_eff=xi, eta=eta, force=force(d), e_d=4 * q * max(d - dy, 0.0), k1=k1, s_e=se)

out["isolation"] = isolation(2.0e5, 6.0e6, 0.01, 3.0e5, 2.5, 1.2, 0.15, 0.5, 2.0)
out["isolation_stiff"] = isolation(1.0e5, 1.5e7, 0.005, 3.0e5, 2.5, 1.2, 0.15, 0.5, 2.0)

def gazetas(gamma, vs, ratio, nu, r):
    gmax = gamma / G * vs ** 2
    g = gmax * ratio ** 2
    return dict(g_max=gmax, g=g, kx=8 * g * r / (2 - nu), kz=4 * g * r / (1 - nu), krx=8 * g * r ** 3 / (3 * (1 - nu)), kt=16 * g * r ** 3 / 3)

def table41(x, table):
    xs = [0.10, 0.20, 0.30]
    return float(np.interp(x, xs, table))

ratio = table41(0.15, [0.9, 0.7, 0.6])
damp = table41(0.15, [0.03, 0.06, 0.10])
out["table41_015"] = dict(vs_ratio=ratio, damping=damp)
sp = gazetas(19000.0, 200.0, ratio, 0.35, 2.0)
out["gazetas"] = sp
ei, h = 3.0e9, 8.0
kp = 3 * ei / h ** 3
out["ssi"] = dict(k_pier=kp, lengthening=math.sqrt(1 + kp / sp["kx"] + kp * h * h / sp["krx"]))
# independent check of the lengthening: period of a cantilever with top mass on translational and rotational springs, from the 3 x 3 flexibility
m = 1.0
flex = h ** 3 / (3 * ei) + 1 / sp["kx"] + h * h / sp["krx"]
t_fixed = 2 * math.pi * math.sqrt(m * h ** 3 / (3 * ei))
t_ssi = 2 * math.pi * math.sqrt(m * flex)
out["ssi"]["lengthening_flex"] = t_ssi / t_fixed

def r_u(fs, alpha=0.7, b=0.34):
    if fs <= 1: return 1.0
    x = fs ** (-1 / (b * alpha))
    return 0.5 + math.asin(2 * x - 1) / math.pi
out["r_u"] = {str(fs): r_u(fs) for fs in (0.8, 1.0, 1.25, 1.5, 2.0, 3.0)}

# pile in liquefied soil: Euler load by finite differences of a pinned column of effective length L
def euler_fd(ei, length, n=400):
    hh = length / n
    main = np.full(n - 1, 2.0 / hh ** 2); off = np.full(n - 2, -1.0 / hh ** 2)
    return ei * eigh_tridiagonal(main, off, eigvals_only=True, select="i", select_range=(0, 0))[0]
d, e_p, kh = 0.8, 30e9, 25e6
ei_p = e_p * math.pi * d ** 4 / 64
beta = (kh * d / (4 * ei_p)) ** 0.25
l_u = 4.0
l_eff = 1.0 * (l_u + 1.0 / beta)
out["pile"] = dict(ei=ei_p, beta=beta, l_eff=l_eff, n_cr=math.pi ** 2 * ei_p / l_eff ** 2, n_cr_fd=euler_fd(ei_p, l_eff),
                   m_free=100e3 * l_eff, m_fixed=100e3 * l_eff / 2)

# third party: groundhog CSR / MSF / r_d at 6 m, a_max = 0.25 g, magnitude 6.5
try:
    from groundhog.soildynamics.liquefaction import cyclicstressratio_youd
    z, sv, svp = 6.0, 6.0 * 18.0, 6.0 * 18.0 - 9.81 * 4.0
    r = cyclicstressratio_youd(0.25 * G, sv, svp, z, 6.5)
    mine_rd = 1 - 0.00765 * z
    mine_csr = 0.65 * 0.25 * (sv / svp) * mine_rd
    mine_msf = 10 ** 2.24 / 6.5 ** 2.56
    out["groundhog_csr"] = dict(csr=r["CSR [-]"], msf=r["MSF [-]"], rd=r["rd [-]"], mine_csr=mine_csr, mine_msf=mine_msf, mine_rd=mine_rd)
    assert abs(r["CSR [-]"] - mine_csr) < 1e-12 and abs(r["MSF [-]"] - mine_msf) < 1e-12
except ImportError:
    out["groundhog_csr"] = "groundhog not installed"
print(json.dumps(out, indent=1))
