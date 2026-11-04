"""🗂️ EN 1997 catalogue oracle: re-evaluates every catalogue entry with the independent reference library `📚️geo.py`.

`evaluate(entry, annex, num, text)` returns {outcome key: value}, where a limit outcome yields its utilisation (demand/resistance) and an information outcome its value.
"""
from __future__ import annotations

import json
import math
import pathlib
import statistics

PI = math.pi


def t90_factor():
    """⏳️ Time factor of the Terzaghi theory at U = 90 %: root of U(T) = 1 - sum 2/M^2 exp(-M^2 T), M = (2m+1) pi / 2, by bisection (theory, not a registry parameter)."""
    def degree(tv):
        return 1.0 - sum(2.0 / m ** 2 * math.exp(-m ** 2 * tv) for m in (math.pi * (2 * k + 1) / 2 for k in range(400)))
    lo, hi = 0.0, 20.0
    for _ in range(200):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if degree(mid) < 0.9 else (lo, mid)
    return (lo + hi) / 2

NDP = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in pathlib.Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())


def ndp_number(row_id, annex):
    """📖️ Number of a shared registry row for the annex (`recommended` for EN, `naDe` for DE), read from the registry JSON like the Rust subject does."""
    row = next(e for e in json.loads(NDP.read_text(encoding="utf-8"))["entries"] if e["id"] == row_id)
    return row["naDe"] if annex.lower() == "de" else row["recommended"]


def load_geo():
    import importlib.util
    import pathlib

    path = pathlib.Path(__file__).with_name("📚️geo.py")
    spec = importlib.util.spec_from_file_location("en1997_geo", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = load_geo()
R = math.radians
tand = G.tand


class Inputs:
    def __init__(self, annex, num, text):
        self.annex, self.num, self.text = annex, num, text
        self.de = annex.lower() == "de"

    def get(self, key, default=None):
        if key in self.num:
            return self.num[key]
        if default is None:
            raise KeyError(key)
        return default

    def has(self, key):
        return key in self.num

    def series(self, prefix):
        out, i = [], 1
        while f"{prefix}{i}" in self.num:
            out.append(self.num[f"{prefix}{i}"])
            i += 1
        return out

    def factors(self):
        return G.factors(self.annex, self.text.get("approach", "da2"), self.text.get("situation", "bsP"))

    def upl(self):
        return G.equ_upl_hyd(self.annex, self.text.get("situation", "bsP"))

    def gamma_w(self):
        return self.num.get("gamma_w", 10000.0 if self.de else 9810.0)


def pf_01(c):
    f, u = c.factors(), c.upl()
    return {**f, **{k: u[k] for k in ("equ_stb", "equ_dst", "equ_q", "upl_stb", "upl_dst", "upl_q", "hyd_h", "hyd_stb")}}


def rw_01(c):
    gamma, h, phi = c.get("gamma"), c.get("h"), c.get("phi_deg")
    delta, alpha, beta, q = c.get("delta_deg", 0.0), c.get("alpha_deg", 0.0), c.get("beta_deg", 0.0), c.get("q", 0.0)
    f = c.factors()
    phi = math.degrees(math.atan(math.tan(R(phi)) / f["gamma_phi"]))
    ka = G.ka_coulomb(phi, delta, alpha, beta)
    ea = 0.5 * gamma * h * h * ka + ka * q * h
    return dict(ka=ka, e_top=ka * q, e_bot=ka * (gamma * h + q), e_a=ea, e_ah=ea * math.cos(R(alpha + delta)), e_av=ea * math.sin(R(alpha + delta)))


def rw_02(c):
    gamma, h, phi = c.get("gamma"), c.get("h"), c.get("phi_deg")
    f = c.factors()
    phi = math.degrees(math.atan(math.tan(R(phi)) / f["gamma_phi"]))
    ka = G.ka_rankine(phi, c.get("beta_deg", 0.0))
    coh = c.get("c", 0.0) / f["gamma_c"]
    z = c.get("z", h)
    z0 = 2 * coh / (gamma * math.sqrt(ka)) if coh > 0 else 0.0
    thrust = 0.5 * (ka * gamma * h - 2 * coh * math.sqrt(ka)) * (h - z0) if h > z0 else 0.0
    return dict(ka=ka, z0=z0, e_z=max(ka * gamma * z - 2 * coh * math.sqrt(ka), 0.0), e_a=thrust)


def rw_03(c):
    gamma, t, phi, delta = c.get("gamma"), c.get("t"), c.get("phi_deg"), c.get("delta_deg", 0.0)
    f = c.factors()
    phi = math.degrees(math.atan(math.tan(R(phi)) / f["gamma_phi"]))
    kp = G.kp_log_spiral(phi, delta)
    ep = 0.5 * gamma * t * t * kp + 2 * c.get("c", 0.0) / f["gamma_c"] * t * math.sqrt(kp) + c.get("q", 0.0) * kp * t
    return dict(kp_rankine=G.kp_rankine(phi), kp_coulomb=G.kp_coulomb(phi, abs(delta)), kp=kp, e_p_k=ep, e_p_d=min(max(c.get("mobilisation", 1.0), 0.0), 1.0) * ep / c.factors()["gamma_r_e"])


def rw_04(c):
    phi, gamma, h = c.get("phi_deg"), c.get("gamma"), c.get("h")
    k = G.k0(phi, c.get("ocr", 1.0), c.get("beta_deg", 0.0))
    q = c.get("q", 0.0)
    return dict(k0=k, e_bot=k * (gamma * h + q), e_0=0.5 * k * gamma * h * h + k * q * h)


def simpson(f, a, b, n=2000):
    h = (b - a) / n
    s = f(a) + f(b)
    for i in range(1, n):
        s += (4 if i % 2 else 2) * f(a + i * h)
    return s * h / 3


def rw_05(c):
    h, ka, gamma = c.get("h"), c.get("ka"), c.get("gamma")
    gamma_sat = c.get("gamma_sat", gamma + 2000.0)
    z_w, q, gw = c.get("z_w", h), c.get("q", 0.0), c.gamma_w()
    zw = min(z_w, h)

    def sigma_eff(z):
        return gamma * z if z <= zw else gamma * zw + (gamma_sat - gw) * (z - zw)

    def earth(z):
        return ka * (sigma_eff(z) + q)

    def water(z):
        return gw * max(z - zw, 0.0)

    f_earth = simpson(earth, 0, zw, 400) + (simpson(earth, zw, h, 400) if h > zw else 0.0)
    f_water = simpson(water, zw, h, 400) if h > zw else 0.0
    out = dict(e_earth_base=earth(h), u_base=water(h), f_earth=f_earth, f_water=f_water)
    if all(c.has(k) for k in ("strip_b", "strip_a", "strip_z", "strip_q")):
        b, a, z, qs = c.get("strip_b"), c.get("strip_a"), c.get("strip_z"), c.get("strip_q")
        t1, t2 = math.atan((a + b) / z), math.atan(a / z)
        beta = t1 - t2
        out["strip_dsigma_h"] = 2 * qs / PI * (beta - math.sin(beta) * math.cos(beta + 2 * t2))
    return out


def rw_06(c):
    b, w, xw = c.get("b"), c.get("w"), c.get("x_w")
    eg, ag, eq, aq = c.get("eah_g"), c.get("arm_g"), c.get("eah_q", 0.0), c.get("arm_q", 1.0)
    ev, xv = c.get("eav", 0.0), c.get("x_v", b)
    f, u = c.factors(), c.upl()
    hd = f["gamma_g"] * eg + f["gamma_q"] * eq
    rd = (w + ev) * math.tan(R(c.get("phi_base_deg"))) / f["gamma_phi"] / f["gamma_r_h"]
    mdst = u["equ_dst"] * eg * ag + u["equ_q"] * eq * aq
    mstb = u["equ_stb"] * (w * xw + ev * xv)
    vk = w + ev
    ep = b / 2 - (w * xw + ev * xv - eg * ag) / vk
    et = b / 2 - (w * xw + ev * xv - eg * ag - eq * aq) / vk
    mean = vk / b
    smax, smin = (mean * (1 + 6 * abs(et) / b), mean * (1 - 6 * abs(et) / b)) if abs(et) <= b / 6 else (2 * vk / (3 * (b / 2 - abs(et))), 0.0)
    return dict(sliding=hd / rd, equ=mdst / mstb, e_perm=abs(ep) / (b / 6), e_total=abs(et) / (b / 3), sigma_max=smax, sigma_min=smin)


def rw_08(c):
    gamma, h, phi = c.get("gamma"), c.get("h"), c.get("phi_deg")
    d, psi, beta, kh, kv = c.get("delta_deg", 0.0), c.get("alpha_deg", 0.0), c.get("beta_deg", 0.0), c.get("kh"), c.get("kv", 0.0)
    ka = G.ka_coulomb(phi, d, psi, beta)
    up, down = G.k_ae(phi, d, psi, beta, kh, kv), G.k_ae(phi, d, psi, beta, kh, -kv)
    e_up, e_down = 0.5 * gamma * (1 - kv) * h * h * up, 0.5 * gamma * (1 + kv) * h * h * down
    kae, eae = (up, e_up) if e_up >= e_down else (down, e_down)
    return dict(kae=kae, ka=ka, e_ae=eae, delta_e=eae - 0.5 * gamma * h * h * ka)


def rw_09(c):
    k, sv, s, fs = c.get("k"), c.get("sigma_v"), c.get("s_v"), c.get("f_star")
    t = k * sv * s
    return dict(t_max=t, l_e=t / (2 * c.get("b", 1.0) * fs * sv))


def rw_10(c):
    q, b, z, ka = c.get("q_total"), c.get("b"), c.get("z", 0.0), c.get("ka")
    return dict(q_z=q / (b + 2 * z), e_h=ka * q / (b + 2 * z))


def _bearing_inputs(c, drained):
    f = c.factors()
    b, l, d, v = c.get("b"), c.get("l"), c.get("d", 0.0), c.get("v_d")
    phi_k = c.get("phi_k_deg", 0.0)
    phi = math.degrees(math.atan(math.tan(R(phi_k)) / f["gamma_phi"])) if drained else 0.0
    g2 = c.get("gamma2", c.get("gamma1", 0.0))
    return f, dict(b=b, l=l, d=d, alpha=c.get("alpha_deg", 0.0), v=v, hb=c.get("hb_d", 0.0), hl=c.get("hl_d", 0.0), eb=c.get("mb_d", 0.0) / v, el=c.get("ml_d", 0.0) / v,
                   phi=phi, c=c.get("c_k", 0.0) / f["gamma_c"], g_below=c.get("gamma1", g2), q_over=g2 * d)


def _bearing_out(res, f, v, b, eb, l=1.0, el=0.0):
    out = {k: res[k] for k in ("nq", "nc", "ng", "sq", "sc", "sg", "iq", "ic", "ig", "bq", "b_eff", "l_eff", "r_over_a", "r_k")}
    out["bearing"] = v / (res["r_k"] / f["gamma_r_v"])
    out["eccentricity_b"] = abs(eb) / (b / 3)
    out["eccentricity_l"] = abs(el) / (l / 3)
    out["eccentricity_ellipse"] = ((eb / b) ** 2 + (el / l) ** 2) / (1.0 / 9.0)
    return out


def geo_f01(c):
    f, i = _bearing_inputs(c, True)
    res = G.bearing_drained(i["b"], i["l"], i["d"], i["alpha"], i["v"], i["hb"], i["hl"], i["eb"], i["el"], i["phi"], i["c"], i["g_below"], i["q_over"], c.get("include_depth", 0.0) > 0.5)
    return _bearing_out(res, f, i["v"], i["b"], i["eb"], i["l"], i["el"])


def geo_f02(c):
    f, i = _bearing_inputs(c, False)
    res = G.bearing_undrained(i["b"], i["l"], i["d"], i["alpha"], i["v"], i["hb"], i["hl"], i["eb"], i["el"], c.get("cu_k") / f["gamma_cu"], i["q_over"])
    return _bearing_out(res, f, i["v"], i["b"], i["eb"], i["l"], i["el"])


def geo_f03(c):
    f, i = _bearing_inputs(c, True)
    res = G.bearing_din4017(i["b"], i["l"], i["v"], i["hb"], i["hl"], i["eb"], i["el"], i["phi"], i["c"], i["g_below"], i["q_over"])
    return _bearing_out(res, f, i["v"], i["b"], i["eb"], i["l"], i["el"])


def geo_f04(c):
    f = c.factors()
    hd = f["gamma_g"] * c.get("h_g") + f["gamma_q"] * c.get("h_q", 0.0)
    rt = c.get("a_eff") * c.get("cu_k") / f["gamma_cu"] if c.has("cu_k") else c.get("v_k") * math.tan(R(c.get("delta_deg"))) / f["gamma_phi"]
    rd = rt / f["gamma_r_h"] + c.get("e_p_k", 0.0) * c.get("passive_share", 0.5) / f["gamma_r_e"]
    return dict(sliding=hd / rd, r_t_k=rt, h_d=hd)


def equ_01(c):
    u = c.upl()
    return dict(equ=(u["equ_dst"] * c.get("m_dst_g") + u["equ_q"] * c.get("m_dst_q", 0.0)) / (u["equ_stb"] * c.get("m_stb_g") + c.get("m_tension", 0.0)))


def upl_01(c):
    u = c.upl()
    water = c.get("head") * c.gamma_w() * c.get("area", 1.0) if c.has("head") else 0.0
    vd = u["upl_dst"] * (c.get("g_dst", 0.0) + water) + u["upl_q"] * c.get("q_dst", 0.0)
    gs, rd = u["upl_stb"] * c.get("g_stb"), c.get("r_d", 0.0)
    return dict(uplift=vd / (gs + rd), ballast=max((vd - rd) / u["upl_stb"] - c.get("g_stb"), 0.0))


def hyd_01(c):
    u, gw = c.upl(), c.gamma_w()
    i = c.get("delta_h") / c.get("l_flow")
    gp = c.get("gamma_prime")
    out = dict(heave=u["hyd_h"] * i * gw / (u["hyd_stb"] * gp), i=i, i_crit=gp / gw, safety=gp / gw / i)
    if c.has("d15_f") and c.has("d85_b") and c.has("d15_b"):
        out["filter_stability"] = c.get("d15_f") / (4 * c.get("d85_b"))
        out["filter_permeability"] = 4 * c.get("d15_b") / c.get("d15_f")
    return out


def _layer_sum(c):
    return c.series("h"), c.series("e")


def sls_01(c):
    s0, shape = c.get("sigma0"), c.text.get("shape", "rect")
    hs, es = _layer_sum(c)
    z, total, out = c.get("z_start", 0.0), 0.0, {}
    for i, (h, e) in enumerate(zip(hs, es), start=1):
        zm = z + h / 2
        if shape == "circle":
            sz = G.sigma_z_circle(s0, c.get("a"), zm)
        elif shape == "strip":
            sz = G.sigma_z_strip_centre(s0, c.get("b"), zm)
        elif shape == "two_to_one":
            sz = s0 * c.get("b") * c.get("l") / ((c.get("b") + zm) * (c.get("l") + zm))
        else:
            sz = G.sigma_z_rect_centre(s0, c.get("b"), c.get("l"), zm)
        out[f"sigma_z{i}"] = sz
        total += sz * h / e
        z += h
    total *= c.get("rigid_factor", 1.0)
    out["s"] = total
    if c.has("s_lim"):
        out["serviceability"] = total / c.get("s_lim")
    return out


def sls_01b(c):
    p, e, nu = c.get("p"), c.get("e_m"), c.get("nu", 0.3)
    shape = c.text.get("shape", "circle_flexible")
    if shape == "circle_rigid":
        s = PI * p * c.get("a") * (1 - nu * nu) / (2 * e)
    elif shape == "factor":
        s = p * c.get("b") * c.get("f") / e
    else:
        s = 2 * p * c.get("a") * (1 - nu * nu) / e
    return dict(s=s)


def tv_of_u(u):
    return PI / 4 * u * u if u < 0.6 else -0.9332 * math.log10(1 - u) - 0.0851


def sls_02(c):
    h, cc, e0, s0, ds = c.get("h"), c.get("cc"), c.get("e0"), c.get("sigma0"), c.get("dsigma")
    cs, sp = c.get("cs", 0.1 * cc), c.get("sigma_p", s0)
    k = h / (1 + e0)
    if s0 + ds <= sp:
        s = cs * k * math.log10((s0 + ds) / s0)
    elif sp > s0:
        s = cs * k * math.log10(sp / s0) + cc * k * math.log10((s0 + ds) / sp)
    else:
        s = cc * k * math.log10((s0 + ds) / s0)
    out = dict(s_c=s)
    if c.has("cv") and c.has("h_dr"):
        out["t90"] = t90_factor() * c.get("h_dr") ** 2 / c.get("cv")
        if c.has("t"):
            tv = c.get("cv") * c.get("t") / c.get("h_dr") ** 2
            lo, hi = 0.0, 0.999999
            for _ in range(100):
                mid = (lo + hi) / 2
                lo, hi = (mid, hi) if tv_of_u(mid) < tv else (lo, mid)
            out["u_avg"], out["s_t"] = lo, s * lo
    return out


def sls_03(c):
    return dict(s_sec=c.get("c_alpha") * c.get("h") / (1 + c.get("e0")) * math.log10(c.get("t2") / c.get("t1")))


def sls_04(c):
    tilt = abs(c.get("s_a") - c.get("s_b")) / c.get("l")
    return dict(tilt=tilt * c.get("limit_den", 500.0), tan_omega=tilt)


XI_STATIC_EN = {1: (1.40, 1.40), 2: (1.30, 1.20), 3: (1.20, 1.05), 4: (1.10, 1.00)}
XI_STATIC_DE = {1: (1.35, 1.35), 2: (1.25, 1.15), 3: (1.15, 1.05), 4: (1.05, 1.00)}
XI_GROUND = [(1, 1.40, 1.40), (2, 1.35, 1.27), (3, 1.33, 1.23), (4, 1.31, 1.20), (5, 1.29, 1.15), (7, 1.27, 1.12), (10, 1.25, 1.08)]
XI_DYNAMIC = [(2, 1.60, 1.50), (5, 1.50, 1.35), (10, 1.45, 1.30), (15, 1.42, 1.25), (20, 1.40, 1.25)]


def lerp_table(table, n):
    row = [r for r in table if r[0] <= n + 1e-12]
    row = row[-1] if row else table[0]
    return row[1], row[2]


def xi_static(n, de, stiff):
    table = XI_STATIC_DE if de else XI_STATIC_EN
    x1, x2 = table.get(min(max(n, 1), 5), (1.0, 1.0)) if n < 5 else (1.0, 1.0)
    return (max(x1 / 1.1, 1.0), max(x2 / 1.1, 1.0)) if stiff else (x1, x2)


def characteristic(values, xa, xb):
    mean = sum(values) / len(values)
    return mean, min(values), min(mean / xa, min(values) / xb)


def pile_01(c):
    r = c.series("rm")
    x1, x2 = xi_static(len(r), c.de, c.get("stiff", 0.0) > 0.5)
    mean, mn, rk = characteristic(r, x1, x2)
    f = c.factors()
    rd = rk / f["gamma_t"]
    out = dict(mean=mean, min=mn, xi1=x1, xi2=x2, r_c_k=rk, r_c_d=rd)
    if c.has("f_cd"):
        out["compression"] = c.get("f_cd") / rd
    return out


def pile_02(c):
    rb, rs = c.series("rb"), c.series("rs")
    x3, x4 = (1.0, 1.0) if c.get("empirical", 0.0) > 0.5 else lerp_table(XI_GROUND, len(rb))
    f = c.factors()
    rbk, rsk = characteristic(rb, x3, x4)[2], characteristic(rs, x3, x4)[2]
    rd = rbk / f["gamma_b"] + rsk / f["gamma_s"]
    out = dict(xi3=x3, xi4=x4, r_b_k=rbk, r_s_k=rsk, r_c_d=rd)
    if c.has("f_cd"):
        out["compression"] = c.get("f_cd") / rd
    return out


def pile_03(c):
    f = c.factors()
    if c.has("rt_k"):
        rk = c.get("rt_k")
    else:
        r = c.series("rt")
        rk = characteristic(r, *xi_static(len(r), c.de, False))[2]
    rd = rk / f["gamma_st"]
    return dict(tension=c.get("f_td") / (rd + c.get("g_pile", 0.0)), r_t_d=rd)


def pile_04(c):
    d, h, e = c.get("d"), c.get("h_d"), c.get("e", 0.0)
    gr = c.factors()["gamma_r_e"]
    out = {}
    if c.has("k_s") and c.has("ei"):
        lam = (c.get("k_s") * d / (4 * c.get("ei"))) ** 0.25
        y0 = 2 * h * lam / (c.get("k_s") * d)
        out.update(y0=y0, m_max=0.322 * h / lam, **{"lambda": lam})
        if c.has("y_lim"):
            out["deflection"] = y0 / c.get("y_lim")
    if c.has("gamma") and c.has("l") and c.has("phi_deg"):
        kp = math.tan(R(45 + c.get("phi_deg") / 2)) ** 2
        hu = 0.5 * c.get("gamma") * d * c.get("l") ** 3 * kp / (e + c.get("l"))
        out.update(transverse=h / (hu / gr), h_u=hu)
    if c.has("row"):
        out["p_multiplier"] = 0.8 if c.get("row") <= 1 else 0.4 if c.get("row") <= 2 else 0.3
    return out


def pile_05(c):
    d, beta = c.get("d"), c.get("beta", 0.25)
    neg = sum(PI * d * beta * s * h for h, s in zip(c.series("h"), c.series("sv")))
    f = c.factors()
    fd = f["gamma_g"] * neg
    out = dict(f_neg_k=neg, f_neg_d=fd)
    if c.has("f_cd") and c.has("r_c_d"):
        out["downdrag"] = (c.get("f_cd") + fd) / c.get("r_c_d")
    return out


def pile_06(c):
    m, n, s, d = int(c.get("m")), int(c.get("n")), c.get("s"), c.get("d")
    theta = math.degrees(math.atan(d / s))
    eta = 1 - theta * ((m - 1) * n + (n - 1) * m) / (90 * m * n)
    sx, sy = c.get("sx", s), c.get("sy", s)
    coords = [((i - (n - 1) / 2) * sx, (j - (m - 1) / 2) * sy) for i in range(n) for j in range(m)]
    v, mx, my = c.get("v", 0.0), c.get("mx", 0.0), c.get("my", 0.0)
    sxx, syy = sum(x * x for x, _ in coords), sum(y * y for _, y in coords)
    loads = [v / len(coords) + (mx * y / syy if syy else 0.0) + (my * x / sxx if sxx else 0.0) for x, y in coords]
    out = dict(eta=eta, n_max=max(loads), n_min=min(loads))
    if all(c.has(k) for k in ("block_b", "block_l", "cu_h_sum", "cu_b")):
        rd = (2 * (c.get("block_b") + c.get("block_l")) * c.get("cu_h_sum") + c.get("block_b") * c.get("block_l") * c.get("nc", 9.0) * c.get("cu_b")) / c.factors()["gamma_b"]
        out["r_block_d"] = rd
        if v > 0:
            out["block"] = v / rd
    if c.has("r_c_d"):
        out["max_pile"] = max(loads) / c.get("r_c_d")
    return out


def pile_07(c):
    ncr = PI ** 2 * c.get("ei") / c.get("l_eff") ** 2
    return dict(buckling=c.get("n_ed") / ncr, n_cr=ncr)


def pile_08(c):
    r = c.series("rm")
    x5, x6 = lerp_table(XI_DYNAMIC, len(r))
    rk = characteristic(r, x5, x6)[2]
    rd = rk / c.factors()["gamma_t"]
    out = dict(xi5=x5, xi6=x6, r_c_k=rk, r_c_d=rd)
    if all(c.has(k) for k in ("eta_h", "w", "h_drop", "s_set", "s_e")):
        out["r_driving"] = c.get("eta_h") * c.get("w") * c.get("h_drop") / (c.get("s_set") + 0.5 * c.get("s_e"))
    if c.has("f_cd"):
        out["compression"] = c.get("f_cd") / rd
    return out


def pile_09(c):
    n, rs, rb, d, ea, l = c.get("n"), c.get("rs_max"), c.get("rb_max"), c.get("d"), c.get("ea"), c.get("l")
    ss, sb = c.get("s_ref_s", 0.01 * d), c.get("s_ref_b", 0.10 * d)
    # piecewise-linear mobilisation solved exactly, breakpoints sorted
    knots = sorted({0.0, ss, sb})
    def total(s):
        return rs * min(s / ss, 1.0) + rb * min(s / sb, 1.0)
    s_mob = knots[-1]
    for a, b in zip(knots, knots[1:]):
        if total(a) <= n <= total(b):
            s_mob = a + (n - total(a)) / (total(b) - total(a)) * (b - a)
            break
    rs_mob = rs * min(s_mob / ss, 1.0)
    s_el = (n - 0.5 * rs_mob) * l / ea
    out = dict(s_mobilised=s_mob, s_elastic=s_el, s_total=s_mob + s_el)
    if c.has("s_lim"):
        out["serviceability"] = (s_mob + s_el) / c.get("s_lim")
    return out


def _wall_coefficients(c):
    phi, da, dp = c.get("phi_deg"), c.get("delta_a_deg", 0.0), c.get("delta_p_deg", 0.0)
    return G.ka_coulomb(phi, da) * math.cos(R(da)), G.kp_log_spiral(phi, dp) * math.cos(R(dp))


def rw_07(c):
    h, gamma, q = c.get("h"), c.get("gamma"), c.get("q", 0.0)
    f = c.factors()
    kah, kph = _wall_coefficients(c)
    gg, gre = f["gamma_g"], f["gamma_r_e"]
    cant = c.text.get("kind", "free_earth") == "cantilever"

    def act_moment(point, depth):
        return gg * kah * (gamma * (depth ** 3 / 3 - point * depth ** 2 / 2) + q * (depth ** 2 / 2 - point * depth))

    def pas_moment(point, t):
        return kph * gamma / gre * (t ** 3 / 3 + (h - point) * t ** 2 / 2)

    if cant:
        def g(t):
            depth = h + t
            return -(act_moment(depth, depth) - pas_moment(depth, t))
        a = None
    else:
        a = c.get("prop_depth")

        def g(t):
            return act_moment(a, h + t) - pas_moment(a, t)

    lo, hi = 0.01, 4 * h + 5
    for _ in range(100):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if g(mid) > 0 else (lo, mid)
    t0 = (lo + hi) / 2
    td = 1.2 * t0 if cant else t0
    depth = h + td
    act = gg * kah * (gamma * depth ** 2 / 2 + q * depth)
    pas = kph * gamma * td ** 2 / (2 * gre)
    prop = 0.0 if cant else act - pas

    def shear(z):
        v = gg * kah * (gamma * z * z / 2 + q * z)
        if z > h:
            v -= kph * gamma * (z - h) ** 2 / (2 * gre)
        if a is not None and z >= a:
            v -= prop
        return v

    mmax, m, steps = 0.0, 0.0, 40000
    dz = depth / steps
    for i in range(steps):
        m += shear((i + 0.5) * dz) * dz
        mmax = max(mmax, abs(m))
    out = dict(m_max_d=mmax, t0=t0, t_design=td)
    if not cant:
        out["prop_force"] = prop
    if c.has("cu_k") and c.has("nc"):
        out["basal_heave"] = (f["gamma_g"] * gamma * h + f["gamma_q"] * q) / (c.get("cu_k") / f["gamma_cu"] * c.get("nc"))
    return out


def anc_01(c):
    r = c.series("ra")
    xi = c.get("xi_a", 1.10)
    mean, mn, rk = characteristic(r, xi, xi)
    rd = rk / c.factors()["gamma_a"]
    out = dict(mean=mean, min=mn, r_a_k=rk, r_a_d=rd)
    if c.has("a_d"):
        out["pullout"] = c.get("a_d") / rd
    return out


def anc_02(c):
    rk = PI * c.get("d") * c.get("l_f") * c.get("tau_f")
    rd = rk / c.factors()["gamma_a"]
    out = dict(r_a_k=rk, r_a_d=rd)
    if c.has("a_d"):
        out["pullout"] = c.get("a_d") / rd
    return out


def anc_03(c):
    eh, theta = c.get("e_h_d"), c.get("theta_deg")
    ad = eh / math.cos(R(theta))
    ak = c.get("a_k", ad)
    out = dict(a_d=ad, proof=1.25 * ak, lock_off=0.9 * ak)
    if all(c.has(k) for k in ("t_w", "z_m", "x_m")):
        tw, zm, xm = c.get("t_w"), c.get("z_m"), c.get("x_m")
        gamma, phi, e1, e2 = c.get("gamma"), R(c.get("phi_deg")), c.get("e1", 0.0), c.get("e2")
        weight = gamma * xm * (tw + zm) / 2 + c.get("q", 0.0) * xm
        eps = math.atan2(zm - tw, xm)
        # slip direction away from the wall and the face normal into the wedge
        dx, dy = math.cos(eps), -math.sin(eps)
        nx, ny = math.sin(eps), math.cos(eps)
        qx, qy = math.cos(phi) * nx + math.sin(phi) * dx, math.cos(phi) * ny + math.sin(phi) * dy
        th = R(theta)
        # unknowns (A, Q):  -A cos th + Q qx = e2 - e1 ;  A sin th + Q qy = weight
        det = (-math.cos(th)) * qy - qx * math.sin(th)
        a_max = ((e2 - e1) * qy - qx * weight) / det
        out["a_kranz"] = a_max
        out["kranz"] = ak * c.get("eta_kranz", 1.5) / a_max
    return out


def dew_radius(c, s, k, big_h):
    if c.has("r_influence"):
        return c.get("r_influence")
    return 575 * s * math.sqrt(big_h * k) if c.text.get("radius", "sichardt") == "kusakin" else 3000 * s * math.sqrt(k)


def dew_01(c):
    k, bh, h, r = c.get("k"), c.get("h_aquifer"), c.get("h_well"), c.get("r_well")
    s = bh - h
    rad = dew_radius(c, s, k, bh)
    if c.text.get("aquifer", "unconfined") == "confined":
        q = 2 * PI * k * c.get("thickness") * s / math.log(rad / r)
    else:
        q = PI * k * (bh * bh - h * h) / math.log(rad / r)
    out = dict(radius=rad, q=q, q_m3h=q * 3600)
    if c.has("q_well"):
        out["wells"] = math.ceil(q / c.get("q_well"))
    return out


def dew_02(c):
    k, bh, h, area = c.get("k"), c.get("h_aquifer"), c.get("h_well"), c.get("area")
    rad = dew_radius(c, bh - h, k, bh)
    req = math.sqrt(area / PI)
    out = dict(r_eq=req, q=PI * k * (bh * bh - h * h) / math.log(rad / req))
    xs, ys, qs = c.series("x"), c.series("y"), c.series("qw")
    if xs and len(xs) == len(ys) == len(qs):
        px, py, d = c.get("px"), c.get("py"), c.get("thickness")
        out["drawdown"] = sum(qj / (2 * PI * k * d) * math.log(rad / max(math.hypot(x - px, y - py), c.get("r_well", 0.1))) for x, y, qj in zip(xs, ys, qs))
    return out


def dew_03(c):
    ds = c.gamma_w() * c.get("delta_h")
    return dict(d_sigma=ds, s=ds * c.get("h") / c.get("e_oed"))


def _slope_f(c):
    return G.factors(c.annex, c.text.get("approach", "geo3"), c.text.get("situation", "bsP"))


def _slices(c, n=400):
    h, beta = c.get("h"), c.get("beta_deg")
    xc, yc, r = c.get("xc"), c.get("yc"), c.get("r")
    gamma, gsat = c.get("gamma"), c.get("gamma_sat", c.get("gamma"))
    gwl, ru, q = c.get("gwl_level", -1000.0), c.get("ru", 0.0), c.get("q", 0.0) * _slope_f(c)["gamma_q"]
    run = h / math.tan(R(beta))

    def ground(x):
        return h if x <= 0 else 0.0 if x >= run else h - x * math.tan(R(beta))

    def base(x):
        u = r * r - (x - xc) ** 2
        return None if u < 0 else yc - math.sqrt(u)

    xs = [xc - r + 2 * r * i / 20000 for i in range(20001)]
    inside = [x for x in xs if base(x) is not None and base(x) < ground(x)]
    xa, xb = inside[0], inside[-1]
    dx = (xb - xa) / n
    out = []
    for i in range(n):
        x = xa + (i + 0.5) * dx
        yb, yg = base(x), ground(x)
        ht = max(yg - yb, 0.0)
        wet = min(max(min(yg, gwl) - yb, 0.0), ht)
        w = gamma * (ht - wet) * dx + gsat * wet * dx + q * dx
        u = ru * gamma * ht * dx if ru > 0 else 9810.0 * wet * dx
        out.append(dict(b=dx, w=w, alpha=math.asin((xc - x) / r), u=u, x=x, yb=yb, yg=yg))
    return out, run


def _drive_resist(c):
    f = _slope_f(c)
    return math.tan(R(c.get("phi_k_deg", 0.0))) / f["gamma_phi"], c.get("c_k", 0.0) / f["gamma_c"]


def slope_01(c):
    if c.has("arc_deg") and c.has("w") and c.has("x_w"):
        f = _slope_f(c)
        mr = c.get("cu_k") * c.get("r") * R(c.get("arc_deg")) * c.get("r")
        return dict(undrained_circle=c.get("w") * c.get("x_w") / (mr / f["gamma_cu"]), m_r_k=mr)
    t, cd = _drive_resist(c)
    sl, _ = _slices(c)
    drive = sum(s["w"] * math.sin(s["alpha"]) for s in sl)
    fs = 1.5
    for _ in range(500):
        res = sum((cd * s["b"] + max(s["w"] - s["u"], 0.0) * t) / (math.cos(s["alpha"]) * (1 + math.tan(s["alpha"]) * t / fs)) for s in sl)
        new = res / drive
        if abs(new - fs) < 1e-12:
            break
        fs = new
    return dict(bishop=1 / fs, f=fs)


def slope_01b(c):
    t, cd = _drive_resist(c)
    sl, _ = _slices(c)
    drive = sum(s["w"] * math.sin(s["alpha"]) for s in sl)
    res = sum(cd * s["b"] / math.cos(s["alpha"]) + max(s["w"] * math.cos(s["alpha"]) - s["u"] / math.cos(s["alpha"]), 0.0) * t for s in sl)
    return dict(fellenius=drive / res, f=res / drive)


def slope_02(c):
    t, cd = _drive_resist(c)
    sl, _ = _slices(c)
    drive = sum(s["w"] * math.tan(s["alpha"]) for s in sl)
    chord = sl[-1]["x"] - sl[0]["x"] + sl[0]["b"]
    depth = max(s["yg"] - s["yb"] for s in sl)
    b1 = 0.69 if (cd > 0 and t > 0) else 0.31 if t > 0 else 0.5
    ratio = depth / chord
    f0 = 1 + b1 * (ratio - 1.4 * ratio * ratio)
    fs = 1.5
    for _ in range(500):
        res = sum((cd * s["b"] + max(s["w"] - s["u"], 0.0) * t) / (math.cos(s["alpha"]) ** 2 * (1 + math.tan(s["alpha"]) * t / fs)) for s in sl)
        new = f0 * res / drive
        if abs(new - fs) < 1e-12:
            break
        fs = new
    return dict(janbu=1 / fs, f=fs)


def slope_03(c):
    f = _slope_f(c)
    beta = R(c.get("beta_deg"))
    tk = math.tan(R(c.get("phi_k_deg")))
    td = tk / f["gamma_phi"]
    g, gs, gw = c.get("gamma", 0.0), c.get("gamma_sat", c.get("gamma", 0.0)), c.gamma_w()
    if c.get("seepage", 0.0) > 0.5:
        red = (gs - gw) / gs
        return dict(infinite_slope=math.tan(beta) / (red * td), f_k=red * tk / math.tan(beta))
    if c.has("c_k") and c.has("z"):
        coh = c.get("c_k") / g / c.get("z")
        fk = c.get("c_k") / (g * c.get("z") * math.sin(beta) * math.cos(beta)) + tk / math.tan(beta)
        fd = c.get("c_k") / f["gamma_c"] / (g * c.get("z") * math.sin(beta) * math.cos(beta)) + td / math.tan(beta)
        return dict(infinite_slope=1 / fd, f_k=fk)
    return dict(infinite_slope=math.tan(beta) / td, f_k=tk / math.tan(beta))


def slope_04(c):
    h, beta, gamma = c.get("h"), R(c.get("beta_deg")), c.get("gamma")
    t, cd = _drive_resist(c)
    best = (1e18, 0.0)
    for i in range(1, 20000):
        th = beta * i / 20000
        w = 0.5 * gamma * h * h * (1 / math.tan(th) - 1 / math.tan(beta))
        fs = (cd * h / math.sin(th) + w * math.cos(th) * t) / (w * math.sin(th))
        best = min(best, (fs, th))
    return dict(wedge=1 / best[0], f=best[0], theta_deg=math.degrees(best[1]))


def g_01(c):
    hs, gs, sats = c.series("h"), c.series("gamma"), c.series("gamma_sat")
    z, zw, gw = c.get("z"), c.get("z_w", 1e18), c.gamma_w()
    sigma, top, tphi, thick = 0.0, 0.0, 0.0, 0.0
    for i, (h, g) in enumerate(zip(hs, gs)):
        gsat = sats[i] if i < len(sats) else g
        bot = min(top + h, z)
        if bot > top:
            dry = max(min(zw, bot) - top, 0.0)
            sigma += g * dry + gsat * (bot - top - dry)
            if c.has(f"phi{i + 1}"):
                tphi += math.tan(R(c.get(f"phi{i + 1}"))) * (bot - top)
                thick += bot - top
        top += h
    u = gw * max(z - zw, 0.0)
    out = dict(sigma_v=sigma, u=u, sigma_v_eff=sigma - u)
    if thick > 0:
        out["phi_equivalent_deg"] = math.degrees(math.atan(tphi / thick))
    return out


def g_02(c):
    x = c.series("x")
    n = len(x)
    mean = statistics.fmean(x)
    s = statistics.stdev(x) if n > 1 else 0.0
    vk = c.get("v_known") if c.has("v_known") else None
    v = vk if vk is not None else s / mean
    t = 1.645 if vk is not None else G.t95(n - 1)
    kn = t * math.sqrt(1 + 1 / n) if c.text.get("mode", "mean") == "local" else t * math.sqrt(1 / n)
    out = dict(mean=mean, s=s, v=v, kn=kn, xk=max(mean * (1 - kn * v), 0.0))
    if n < 3 or v > 0.3:
        out["warning"] = 1.0
    return out


def gi_01(c):
    if c.has("d_b"):
        depth, lo, hi = max(5.0, 3 * c.get("d_b")), 0.0, 0.0
    else:
        depth = max(6.0, 2 * c.get("b"))
        row = "gc3" if c.get("gk", 2.0) >= 3 else "gc12"
        lo, hi = (ndp_number(f"ndp.en1997.2.4.2.spacing.{row}.{side}", c.annex) for side in ("min", "max"))
    out = dict(z_a=depth, spacing_min=lo, spacing_max=hi)
    if c.has("z_have"):
        out["coverage"] = depth / c.get("z_have")
    return out


def gi_02(c):
    qc, fs, sv, svp = c.get("qc"), c.get("fs"), c.get("sigma_v0"), c.get("sigma_v0_eff")
    qt = qc + c.get("u2", 0.0) * (1 - c.get("a", 0.8))
    qn, fn = (qt - sv) / svp, fs / (qt - sv) * 100
    return dict(qt=qt, rf=fs / qc * 100, ic=math.hypot(3.47 - math.log10(qn), math.log10(fn) + 1.22))


def gi_03(c):
    nkt = c.get("nkt", ndp_number("ndp.en1997.2.cpt-nkt", c.annex))
    return dict(cu=(c.get("qt") - c.get("sigma_v0")) / nkt, nkt=nkt)


def gi_04(c):
    return dict(phi_deg=17.6 + 11 * math.log10(c.get("qc") / 1e5 / math.sqrt(c.get("sigma_v0_eff") / 1e5)))


def gi_05(c):
    return dict(e_oed=c.get("alpha_e", 4.0) * c.get("qc"))


def gi_06(c):
    n60 = c.get("n") * c.get("er", 60.0) / 60 * c.get("cb", 1.0) * c.get("cs", 1.0) * c.get("cr", 1.0)
    return dict(n60=n60, n1_60=n60 * min(math.sqrt(1e5 / c.get("sigma_v0_eff")), 1.7))


def gi_07(c):
    return dict(phi_deg=math.sqrt(20 * c.get("n1_60")) + 20)


def gi_08(c):
    return dict(dr=math.sqrt(c.get("n1_60") / 60))


def gi_09(c):
    pts = [(2, 27.5), (5, 30.0), (10, 32.5), (20, 35.0), (30, 37.5)]
    n = c.get("n10")
    if n <= pts[0][0]:
        return dict(phi_deg=pts[0][1])
    if n >= pts[-1][0]:
        return dict(phi_deg=pts[-1][1])
    for (a, pa), (b, pb) in zip(pts, pts[1:]):
        if n <= b:
            return dict(phi_deg=pa + (n - a) / (b - a) * (pb - pa))


def gi_10(c):
    e, s = c.series("e"), c.series("sigma")
    cc = (e[-2] - e[-1]) / math.log10(s[-1] / s[-2])
    cs = (e[0] - e[1]) / math.log10(s[1] / s[0])
    out = dict(cc=cc, cs=cs)
    if c.has("h_dr") and c.has("t90"):
        out["cv"] = t90_factor() * c.get("h_dr") ** 2 / c.get("t90")
    return out


def gi_11(c):
    p, q = c.series("p"), c.series("q")
    fit = statistics.linear_regression(p, q)
    phi = math.asin(fit.slope)
    return dict(phi_deg=math.degrees(phi), c=fit.intercept / math.cos(phi))


def gi_12(c):
    fit = statistics.linear_regression(c.series("sigma"), c.series("tau"))
    return dict(phi_deg=math.degrees(math.atan(fit.slope)), c=fit.intercept)


def gi_14(c):
    return dict(q_b_k=c.get("k_p", 1.0) * c.get("p_le"), e_oed=c.get("e_m") / c.get("alpha_m", 0.5))


def gi_15(c):
    d10, d30, d60 = c.get("d10"), c.get("d30"), c.get("d60")
    return dict(cu=d60 / d10, cc=d30 * d30 / (d10 * d60), k=0.0116 * d10 * d10)


def gi_16(c):
    return dict(k=c.get("q") * math.log(c.get("r2") / c.get("r1")) / (PI * (c.get("h2") ** 2 - c.get("h1") ** 2)))


def _line_fit(x, y):
    """📐️ Slope, intercept and their standard errors; scipy.stats.linregress when installed, closed form otherwise."""
    try:
        from scipy import stats  # third-party reference when installed

        r = stats.linregress(x, y)
        return float(r.slope), float(r.intercept), float(r.stderr), float(r.intercept_stderr)
    except Exception:
        n, mx = len(x), statistics.fmean(x)
        fit = statistics.linear_regression(x, y)
        sxx = sum((v - mx) ** 2 for v in x)
        s_e = math.sqrt(sum((v - fit.intercept - fit.slope * u) ** 2 for u, v in zip(x, y)) / (n - 2))
        return fit.slope, fit.intercept, s_e / math.sqrt(sxx), s_e * math.sqrt(1 / n + mx * mx / sxx)


def gi_17(c):
    triaxial = c.text.get("kind", "shear") == "triaxial"
    x, y = (c.series("p"), c.series("q")) if triaxial else (c.series("sigma"), c.series("tau"))
    b, a, se_b, se_a = _line_fit(x, y)
    t = G.t95(len(x) - 2)
    b_k, a_k = b - t * se_b, a - t * se_a
    phi, phi_k = (math.asin(b), math.asin(b_k)) if triaxial else (math.atan(b), math.atan(b_k))
    scale, scale_k = (1 / math.cos(phi), 1 / math.cos(phi_k)) if triaxial else (1.0, 1.0)
    out = dict(phi_mean_deg=math.degrees(phi), c_mean=a * scale, t=t, se_slope=se_b, se_intercept=se_a, phi_k_deg=math.degrees(phi_k), c_k=max(a_k * scale_k, 0.0))
    if c.has("phi_assumed_deg"):
        out["phi_assumed"] = math.tan(R(c.get("phi_assumed_deg"))) / math.tan(phi_k)
    if c.has("c_assumed"):
        cap, dem = max(a_k * scale_k, 0.0), c.get("c_assumed")
        out["c_assumed"] = min(dem / cap, 1.0e6) if cap > 2.3e-16 else (0.0 if dem <= 2.3e-16 else 1.0e6)
    return out


def pile_10(c):
    ts, tq, d = c.series("ts"), c.series("tq"), c.get("d")
    s_g = c.get("s_crit_factor", 0.1) * d
    try:
        import numpy as np  # third-party reference when installed

        r_m = float(np.interp(s_g, ts, tq)) if s_g <= ts[-1] else max(tq)
    except ImportError:
        r_m = max(tq)
        for (s0, s1), (q0, q1) in zip(zip(ts, ts[1:]), zip(tq, tq[1:])):
            if s0 <= s_g <= s1:
                r_m = q0 + (q1 - q0) * (s_g - s0) / (s1 - s0)
                break
    allr = [r_m] + c.series("rm")
    x1, x2 = xi_static(len(allr), c.de, c.get("stiff", 0.0) > 0.5)
    mean, mn, rk = characteristic(allr, x1, x2)
    rd = rk / c.factors()["gamma_t"]
    out = dict(criterion_reached=s_g / ts[-1], s_g=s_g, r_c_m=r_m, mean=mean, min=mn, xi1=x1, xi2=x2, r_c_k=rk, r_c_d=rd, xi_basis_unverified=1.0 if c.de else 0.0)
    if c.has("f_cd"):
        out["compression"] = c.get("f_cd") / rd
    return out


def pile_11(c):
    d = c.get("d")
    perimeter, area = math.pi * d, math.pi * d * d / 4
    num, out = dict(c.num), {}
    j = 1
    while c.series(f"p{j}_h"):
        h, qs = c.series(f"p{j}_h"), c.series(f"p{j}_qs")
        rs = sum(perimeter * hh * q for hh, q in zip(h, qs))
        rb = c.get(f"p{j}_qb") * area
        num[f"rb{j}"], num[f"rs{j}"] = rb, rs
        out[f"r_b_cal{j}"], out[f"r_s_cal{j}"] = rb, rs
        j += 1
    out.update(pile_02(Inputs(c.annex, num, c.text)))
    return out


def hyd_02(c):
    dh, gp, gw, u = c.get("delta_h"), c.get("gamma_prime"), c.gamma_w(), c.upl()
    lv, lh = c.series("lv"), c.series("lh")
    path = sum(lv) + sum(lh)
    if c.has("i_exit"):
        i = c.get("i_exit")
    elif c.has("d_embed"):
        i = dh / (math.pi * math.hypot(0.0, c.get("d_embed")))
    else:
        i = dh / path
    l_lane = sum(lv) + sum(lh) / 3
    out = dict(piping=u["hyd_h"] * i * gw / (u["hyd_stb"] * gp), i_exit=i, i_crit=gp / gw, safety=gp / gw / i)
    if path > 0:
        out.update(l_bligh=path, l_lane=l_lane, c_bligh_provided=path / dh, c_lane_provided=l_lane / dh)
        if c.has("c_lane"):
            out["lane"] = c.get("c_lane") / (l_lane / dh)
        if c.has("c_bligh"):
            out["bligh"] = c.get("c_bligh") / (path / dh)
    if c.has("k"):
        out["v_exit"] = c.get("k") * i
    return out


def slope_05(c):
    t, cd = _drive_resist(c)
    sl, _ = _slices(c)
    xc, yc, r, width = c.get("xc"), c.get("yc"), c.get("r"), c.get("width")
    drive = sum(s["w"] * math.sin(s["alpha"]) for s in sl)
    f2 = slope_01(c)["f"]
    arm, area = 0.0, 0.0
    for s in sl:
        ht = max(s["yg"] - s["yb"], 0.0)
        arm += s["b"] * ht * math.hypot(s["x"] - xc, s["yb"] + 0.5 * ht - yc)
        area += s["b"] * ht
    gain = 2 * cd * arm / (r * drive * width)
    return dict(bishop_3d=1 / (f2 + gain), f_2d=f2, f_3d=f2 + gain, end_gain=gain, volume=area * width)


def _flow_net(dh, d, t, per_wall=12, cap=160):
    """🌊️ Head field of a sheet pile in a layer: the same cell-centred finite-volume stencil as the Rust kernel, solved with scipy.sparse.linalg.spsolve instead of relaxation."""
    import numpy as np
    import scipy.sparse as sp
    import scipy.sparse.linalg as sla

    m = min(max(int(per_wall), 4), 40)
    h = d / m
    nz = min(max(round(t / h), m + 1), cap)
    half = min(max(round(2 * max(t, d) / h) + m, m + 2), cap)
    nx, centre = 2 * half, half
    rows, cols, vals = [], [], []
    rhs = np.zeros(nx * nz)
    for j in range(nz):
        for i in range(nx):
            r, diag = j * nx + i, 0.0
            for i2, j2, blocked in ((i - 1, j, i == centre and j < m), (i + 1, j, i + 1 == centre and j < m), (i, j - 1, False), (i, j + 1, False)):
                if j2 < 0:
                    diag += 2.0
                    rhs[r] += 2.0 * (dh if i < centre else 0.0)
                elif 0 <= i2 < nx and j2 < nz and not blocked:
                    rows.append(r), cols.append(j2 * nx + i2), vals.append(-1.0)
                    diag += 1.0
            rows.append(r), cols.append(r), vals.append(diag)
    head = sla.spsolve(sp.csr_matrix((vals, (rows, cols)), shape=(nx * nz, nx * nz)), rhs).reshape(nz, nx)
    return head, h, centre, m


def _max_gradient(head, h, centre, m, dh):
    """🧭️ Largest |grad h| of the head field with numpy: central differences that never cross the wall, one-sided at the edges, the surface row against the boundary head."""
    import numpy as np

    nz, nx = head.shape
    vertical = np.empty_like(head)
    vertical[0] = (head[0] - np.where(np.arange(nx) < centre, dh, 0.0)) / (0.5 * h)
    vertical[1:-1] = (head[2:] - head[:-2]) / (2 * h)
    vertical[-1] = (head[-1] - head[-2]) / h
    horizontal = np.zeros_like(head)
    for j in range(nz):
        for i in range(nx):
            left = i > 0 and not (i == centre and j < m)
            right = i + 1 < nx and not (i + 1 == centre and j < m)
            if left and right:
                horizontal[j, i] = (head[j, i + 1] - head[j, i - 1]) / (2 * h)
            elif left:
                horizontal[j, i] = (head[j, i] - head[j, i - 1]) / h
            elif right:
                horizontal[j, i] = (head[j, i + 1] - head[j, i]) / h
    return float(np.hypot(vertical, horizontal).max())


def hyd_03(c):
    dh, d, t, gp = c.get("delta_h"), c.get("d_embed"), c.get("t_layer"), c.get("gamma_prime")
    head, h, centre, m = _flow_net(dh, d, t, round(c.get("cells", 12.0)))
    u, gw, k = c.upl(), c.gamma_w(), c.get("k", 1.0)
    i_exit = 2 * head[0, centre] / h
    width = max(m // 2, 1)
    hm = sum((head[m - 1, i] + head[m, i]) / 2 for i in range(centre, centre + width)) / width
    out = dict(
        heave_prism=u["hyd_h"] * gw * hm * d / 2 / (u["hyd_stb"] * gp * d * d / 2),
        piping=u["hyd_h"] * i_exit * gw / (u["hyd_stb"] * gp),
        i_exit=i_exit, i_crit=gp / gw, safety=gp / gw / i_exit, i_max=_max_gradient(head, h, centre, m, dh), h_prism=hm, h_prism_ratio=hm / dh,
        internal_erosion=_max_gradient(head, h, centre, m, dh) * gw / gp, d_required=u["hyd_h"] * gw * hm / (u["hyd_stb"] * gp),
        q=2 * k * float(head[0, centre:].sum()), cells=float(head.size),
    )
    if c.has("k"):
        out["v_exit"] = k * i_exit
    return out


def rw_11(c):
    h, tb, b, a, ts, phi = c.get("h"), c.get("t_base"), c.get("b"), c.get("a_toe", 0.0), c.get("t_stem"), c.get("phi_deg")
    gc, gamma, delta, q, dt = c.get("gamma_c", 25000.0), c.get("gamma"), c.get("delta_deg", 2.0 / 3.0 * phi), c.get("q", 0.0), c.get("d_toe", 0.0)
    heel = b - a - ts
    height = h + tb
    parts = [(ts * h * gc, a + ts / 2), (b * tb * gc, b / 2), (heel * h * gamma, b - heel / 2), (a * dt * gamma, a / 2)]
    w = sum(f for f, _ in parts)
    xw = sum(f * x for f, x in parts) / w
    ka = G.ka_coulomb(phi, delta)
    eg, eq = 0.5 * ka * gamma * height ** 2, ka * q * height
    num = dict(b=b, w=w, x_w=xw, eah_g=eg * math.cos(R(delta)), arm_g=height / 3, eah_q=eq * math.cos(R(delta)), arm_q=height / 2, eav=(eg + eq) * math.sin(R(delta)), x_v=b, phi_base_deg=c.get("phi_base_deg", phi))
    out = rw_06(Inputs(c.annex, num, {"approach": c.text.get("approach", "da2"), "situation": c.text.get("situation", "bsP")}))
    if c.has("sigma_rd"):
        out["bearing"] = out["sigma_max"] / c.get("sigma_rd")
    out.update(heel=heel, w=w, x_w=xw, ka=ka, e_ah=num["eah_g"], e_av=num["eav"])
    return out


def rw_12(c):
    """🧗️ Embedded wall in soil and water with scipy.integrate.quad for the load integrals and scipy.optimize.brentq for the roots."""
    import warnings

    warnings.simplefilter("ignore")
    from scipy.integrate import quad
    from scipy.optimize import brentq

    h, gamma, phi = c.get("h"), c.get("gamma"), c.get("phi_deg")
    gw, f = c.gamma_w(), c.factors()
    gb = c.get("gamma_sat", gamma) - gw
    phid = math.degrees(math.atan(math.tan(R(phi)) / f["gamma_phi"]))
    da, dp = c.get("delta_a_deg", 0.0), c.get("delta_p_deg", 0.0)
    kah, kph = G.ka_coulomb(phid, da) * math.cos(R(da)), G.kp_log_spiral(phid, dp) * math.cos(R(dp))
    zw = c.get("z_w", math.inf)
    zwf = c.get("z_wf", zw)
    q, kind = c.get("q", 0.0), c.text.get("kind", "free_earth")
    dry = max(zwf - h, 0.0)

    def sv_b(z):
        return gamma * z if z <= zw else gamma * zw + gb * (z - zw)

    def sv_f(z):
        zf = z - h
        return gamma * zf if zf <= dry else gamma * dry + gb * (zf - dry)

    def water(z):
        return gw * (max(z - zw, 0.0) - max(z - zwf, 0.0))

    def net(z):
        act = f["gamma_g"] * (kah * sv_b(z) + water(z)) + f["gamma_q"] * kah * q
        pas = 0.0 if z <= h else kph * sv_f(z) / f["gamma_r_e"]
        return act - pas

    kinks = [x for x in (h, zw, zwf) if math.isfinite(x)]

    def integral(fn, a, b):
        return quad(fn, a, b, points=[x for x in kinks if a < x < b] or None, limit=400, epsabs=1e-9, epsrel=1e-12)[0]

    def peak(depth, support):
        def shear(z):
            return integral(net, 0.0, z) - (support[1] if support and z >= support[0] else 0.0)

        def moment(z):
            return integral(lambda s: (z - s) * net(s), 0.0, z) - (support[1] * (z - support[0]) if support and z > support[0] else 0.0)

        grid = [depth * i / 400 for i in range(1, 401)]
        best = 0.0
        values = [shear(z) for z in grid]
        for z0, z1, v0, v1 in zip(grid, grid[1:], values, values[1:]):
            if v0 * v1 < 0:
                best = max(best, abs(moment(brentq(shear, z0, z1, xtol=1e-12))))
        return max(best, max(abs(moment(z)) for z in grid[::20]))

    lo, hi = 0.01, 4 * h + 5
    blum = 1.2
    out = {}
    if kind == "cantilever":
        t0 = brentq(lambda t: -integral(lambda z: net(z) * (z - (h + t)), 0.0, h + t), lo, hi, xtol=1e-12)
        out.update(m_max_d=peak(h + t0, None), t0=t0)
        required = blum * t0
    elif kind == "fixed_earth":
        a = c.get("prop_depth")
        zs = [h + 1e-6 + (hi - lo) * i / 4000 for i in range(4001)]
        z1 = next(z for z, zn in zip(zs, zs[1:]) if net(zn) <= 0 < net(z))
        zy = brentq(net, z1, z1 + (hi - lo) / 4000, xtol=1e-12)
        reaction = integral(lambda z: net(z) * (z - a), 0.0, zy) / (zy - a)
        anchor = integral(net, 0.0, zy) - reaction
        x = brentq(lambda x: reaction * x + integral(lambda z: net(z) * (x - (z - zy)), zy, zy + x), 1e-3, hi, xtol=1e-12)
        out.update(m_max_d=peak(zy, (a, anchor)), prop_force=anchor, c_force=reaction, y=zy - h, x=x)
        required = (zy - h) + blum * x
    else:
        a = c.get("prop_depth")
        t0 = brentq(lambda t: integral(lambda z: net(z) * (z - a), 0.0, h + t), lo, hi, xtol=1e-12)
        anchor = integral(net, 0.0, h + t0)
        out.update(m_max_d=peak(h + t0, (a, anchor)), t0=t0, prop_force=anchor)
        required = t0
    if c.has("t_provided"):
        out["embedment"] = required / c.get("t_provided")
    out.update(t_required=required, u_net=water(h + required), delta_h=max(zwf - zw, 0.0) if math.isfinite(zw) and math.isfinite(zwf) else 0.0)
    return out


def emb_01(c):
    from scipy.integrate import quad

    h, crest, slope, gamma = c.get("h"), c.get("b_crest"), c.get("n_slope"), c.get("gamma")
    cu, t, e_oed, q = c.get("cu_k"), c.get("t_soft"), c.get("e_oed"), c.get("q", 0.0)
    f = c.factors()
    fill = gamma * h
    run, base, b = slope * h, crest + 2 * slope * h, crest / 2
    cu_d = cu / f["gamma_cu"]
    phid = math.atan(math.tan(R(c.get("phi_deg", 30.0))) / f["gamma_phi"])
    ka = math.tan(math.pi / 4 - phid / 2) ** 2

    def sigma(z):
        def line(x):
            ax = abs(x)
            p = 1.0 if ax <= b else max(1.0 - (ax - b) / run, 0.0)
            return p * 2 * z ** 3 / (math.pi * (x * x + z * z) ** 2)

        return quad(line, -(b + run), b + run, points=[-b, 0.0, b], limit=400, epsabs=1e-13, epsrel=1e-12)[0]

    n = 20
    s = sum((fill + q) * sigma((i + 0.5) * t / n) * (t / n) / e_oed for i in range(n))
    out = dict(
        bearing=(f["gamma_g"] * fill + f["gamma_q"] * q) / ((math.pi + 2) * cu_d / f["gamma_r_v"]),
        spreading=(f["gamma_g"] * 0.5 * ka * gamma * h * h + f["gamma_q"] * ka * q * h) / (cu_d * base / f["gamma_r_h"]),
        s_centre=s, q_fill=fill, b_base=base, sigma_z_mid=(fill + q) * sigma(t / 2), ka=ka,
    )
    if c.has("s_lim"):
        out["settlement"] = s / c.get("s_lim")
    return out


def _consolidation_pde(tv, n=200, steps=240):
    """⏳️ Average degree of consolidation at time factor `tv` from a finite-volume Crank-Nicolson solution of du/dT = d2u/dz2 (drained at z = 0, sealed at z = 1, u = 1 initially, Rannacher start): independent of the Terzaghi series of the Rust kernel."""
    h, dt = 1.0 / n, tv / steps
    u = [1.0] * n

    def apply(v):
        out = [(v[i - 1] - 2 * v[i] + v[i + 1]) / (h * h) for i in range(1, n - 1)]
        return [(v[1] - 3 * v[0]) / (h * h)] + out + [(v[-2] - v[-1]) / (h * h)]

    def solve(rhs, theta, step):
        k = theta * step / (h * h)
        lower, diag, upper = [-k] * n, [1 + 2 * k] * n, [-k] * n
        diag[0], diag[-1] = 1 + 3 * k, 1 + k
        cp, dp = [0.0] * n, [0.0] * n
        cp[0], dp[0] = upper[0] / diag[0], rhs[0] / diag[0]
        for i in range(1, n):
            den = diag[i] - lower[i] * cp[i - 1]
            cp[i] = upper[i] / den
            dp[i] = (rhs[i] - lower[i] * dp[i - 1]) / den
        x = [0.0] * n
        x[-1] = dp[-1]
        for i in range(n - 2, -1, -1):
            x[i] = dp[i] - cp[i] * x[i + 1]
        return x

    for j in range(steps):
        if j < 4:
            for _ in range(2):
                u = solve(u, 1.0, dt / 2)
        else:
            lu = apply(u)
            u = solve([a + 0.5 * dt * b for a, b in zip(u, lu)], 0.5, dt)
    return 1.0 - h * sum(u)


def _tv_of_u_pde(degree):
    lo, hi = 1e-4, 4.0
    for _ in range(36):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if _consolidation_pde(mid, 100, 120) < degree else (lo, mid)
    return (lo + hi) / 2


def sls_05(c):
    s_final, h_dr = c.get("s_final"), c.get("h_dr")
    cv = c.get("cv") if c.has("cv") else _tv_of_u_pde(0.9) * c.get("h_dr_lab") ** 2 / c.get("t90_lab")
    out = dict(cv=cv, t50=_tv_of_u_pde(0.5) * h_dr ** 2 / cv, t90=_tv_of_u_pde(0.9) * h_dr ** 2 / cv)
    if c.has("t"):
        u = _consolidation_pde(cv * c.get("t") / h_dr ** 2)
        out.update(u_avg=u, s_t=s_final * u, s_remaining=s_final * (1 - u))
        if c.has("s_res_lim"):
            out["residual"] = s_final * (1 - u) / c.get("s_res_lim")
    return out


def pile_12(c):
    m, n, s, d, l, v = (c.get(k) for k in ("m", "n", "s", "d", "l", "v"))
    grow = 2 * c.get("spread") * c.get("raft_depth_factor") * l
    b, ln = (n - 1) * c.get("sx", s) + d + grow, (m - 1) * c.get("sy", s) + d + grow
    p = v / (b * ln)
    z, total = 0.0, 0.0
    for h, e in zip(c.series("h"), c.series("e")):
        total += G.sigma_z_rect_centre(p, b, ln, z + h / 2) * h / e
        z += h
    out = dict(b_raft=b, l_raft=ln, p_raft=p, s_group=total)
    if c.has("s_lim"):
        out["serviceability"] = total / c.get("s_lim")
    return out


def _spencer_terms(sl, t, cd, f, lam):
    """⛰️ Shear capacity and total base normal force of every slice at factor `f` and interslice inclination `lam = tan(theta)`."""
    terms = []
    for s in sl:
        ca, sa = math.cos(s["alpha"]), math.sin(s["alpha"])
        n_eff = max((s["w"] - s["u"] / ca * (ca + lam * sa) - cd * s["b"] / ca * (sa - lam * ca) / f) / (ca + lam * sa + t / f * (sa - lam * ca)), 0.0)
        terms.append((cd * s["b"] / ca + n_eff * t, n_eff + s["u"] / ca, ca, sa))
    return terms


def slope_06(c):
    t, cd = _drive_resist(c)
    sl, _ = _slices(c)
    drive = sum(s["w"] * math.sin(s["alpha"]) for s in sl)
    f_bishop = slope_01(c)["f"]

    def residual(x):
        f, lam = x
        terms = _spencer_terms(sl, t, cd, f, lam)
        return (f * drive - sum(sf for sf, _, _, _ in terms), f * sum(n * sa for _, n, _, sa in terms) - sum(sf * ca for sf, _, ca, _ in terms))

    x = [f_bishop, 0.0]
    try:
        from scipy.optimize import fsolve

        x = list(fsolve(residual, x, xtol=1e-12))
    except ImportError:
        for _ in range(60):
            r = residual(x)
            jac = []
            for k in range(2):
                y = list(x)
                y[k] += 1e-7
                jac.append([(a - b) / 1e-7 for a, b in zip(residual(y), r)])
            det = jac[0][0] * jac[1][1] - jac[1][0] * jac[0][1]
            step = ((jac[1][1] * r[0] - jac[1][0] * r[1]) / det, (-jac[0][1] * r[0] + jac[0][0] * r[1]) / det)
            x = [x[0] - step[0], x[1] - step[1]]
            if max(abs(v) for v in step) < 1e-12:
                break
    return dict(spencer=1 / x[0], f=x[0], theta_deg=math.degrees(math.atan(x[1])), f_bishop=f_bishop)


REGISTRY = {
    "PF-01": pf_01, "RW-01": rw_01, "RW-02": rw_02, "RW-03": rw_03, "RW-04": rw_04, "RW-05": rw_05, "RW-06": rw_06, "RW-07": rw_07,
    "RW-08": rw_08, "RW-09": rw_09, "RW-10": rw_10, "GEO-F01": geo_f01, "GEO-F02": geo_f02, "GEO-F03": geo_f03, "GEO-F04": geo_f04,
    "EQU-01": equ_01, "UPL-01": upl_01, "HYD-01": hyd_01, "SLS-01": sls_01, "SLS-01B": sls_01b, "SLS-02": sls_02, "SLS-03": sls_03,
    "SLS-04": sls_04, "PILE-01": pile_01, "PILE-02": pile_02, "PILE-03": pile_03, "PILE-04": pile_04, "PILE-05": pile_05, "PILE-06": pile_06,
    "PILE-07": pile_07, "PILE-08": pile_08, "PILE-09": pile_09, "ANC-01": anc_01, "ANC-02": anc_02, "ANC-03": anc_03, "DEW-01": dew_01,
    "DEW-02": dew_02, "DEW-03": dew_03, "SLOPE-01": slope_01, "SLOPE-01B": slope_01b, "SLOPE-02": slope_02, "SLOPE-03": slope_03,
    "SLOPE-04": slope_04, "G-01": g_01, "G-02": g_02, "GI-01": gi_01, "GI-02": gi_02, "GI-03": gi_03, "GI-04": gi_04, "GI-05": gi_05,
    "GI-06": gi_06, "GI-07": gi_07, "GI-08": gi_08, "GI-09": gi_09, "GI-10": gi_10, "GI-11": gi_11, "GI-12": gi_12, "GI-13": g_02,
    "GI-14": gi_14, "GI-15": gi_15, "GI-16": gi_16,
    "GI-17": gi_17, "PILE-10": pile_10, "PILE-11": pile_11, "HYD-02": hyd_02, "SLOPE-05": slope_05,
    "HYD-03": hyd_03, "SLS-05": sls_05, "PILE-12": pile_12, "SLOPE-06": slope_06, "RW-11": rw_11, "RW-12": rw_12, "EMB-01": emb_01,
}

ANC_04 = None


def anc_04(c):
    ad, pt = c.get("a_d"), c.get("p_t_k")
    out = dict(tendon=ad / (pt / c.get("gamma_s", 1.15)))
    if c.has("l_free"):
        lmin = c.get("l_min", 3.0 if c.text.get("tendon", "strand") == "bar" else 4.5)
        out["free_length"] = lmin / c.get("l_free")
    return out


REGISTRY["ANC-04"] = anc_04


def evaluate(entry, annex, num, text):
    return REGISTRY[entry](Inputs(annex, num, text))
