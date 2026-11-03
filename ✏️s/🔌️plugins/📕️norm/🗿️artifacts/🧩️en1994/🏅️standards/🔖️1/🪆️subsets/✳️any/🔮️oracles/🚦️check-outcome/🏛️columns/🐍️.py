"""🏛 Independent EN 1994-1-1 6.7 composite column rows of a committed snapshot: the simplified method (N_pl,Rd, buckling resistance chi N_pl,Rd, second-order amplified M-N interaction) from the cross-section by horizontal strip integration of the plastic stress blocks (Annex C style polygon A, C, D, B of Fig. 6.19). The concrete-filled and the stored-area encased column use the equivalent square tube of the stored steel area; an encased or partially encased column with its section leaves set uses the I-section with its concrete. SI throughout. Unverified: the equivalent-tube idealisation and the bar level (50 mm from the edge) are catalogue assumptions, not clauses."""

import math

import numpy as np

E_A = E_S = 210e9
GAMMA = dict(c=1.5, s=1.15, m0=1.0)
STRIPS = 4000
EDGE = 0.05
ALPHA_CURVE = {"a": 0.21, "b": 0.34, "c": 0.49}


def ecm_pa(fck):
    return 22e9 * ((fck / 1e6 + 8.0) / 10.0) ** 0.3


def psi0(kind, category):
    if kind == "imposed":
        return {"C": 0.7, "D": 0.7, "E": 1.0, "H": 0.0}.get(category, 0.7)
    return {"snow": 0.5, "wind": 0.6, "construction": 1.0}.get(kind, 0.7)


def design_effects(col):
    """⚖️ EN 1990 6.10 axial force and moment of the composite-stage actions, each the largest over the leading variable action."""
    staged = [a for a in col["actions"] if a["stage"] in ("composite", "any")]
    base = [1.35 * sum(a[key] for a in staged if a["kind"] == "permanent") for key in ("nKN", "mKNm")]
    var = [a for a in staged if a["kind"] in ("imposed", "snow", "wind", "construction")]
    best = list(base)
    for lead in var:
        total = [base[k] + 1.5 * lead[key] for k, key in enumerate(("nKN", "mKNm"))]
        for other in var:
            if other is not lead:
                total = [total[k] + 1.5 * psi0(other["kind"], other["category"]) * other[key] for k, key in enumerate(("nKN", "mKNm"))]
        best = [max(best[k], total[k], key=abs) for k in range(2)]
    return abs(best[0]), abs(best[1]), 1.35 * abs(sum(a["nKN"] for a in staged if a["kind"] == "permanent"))


class Section:
    """🧱 A section as strips: per strip height, the steel width, concrete width and the two bar layers; resultants for a plastic neutral axis at height `xi` above the centroid (compression above)."""

    def __init__(self, ys, steel_w, conc_w, bar_area, bar_y, fyd, fcd, fsd):
        self.ys, self.steel_w, self.conc_w, self.bar_area, self.bar_y = ys, steel_w, conc_w, bar_area, bar_y
        self.dy = ys[1] - ys[0]
        self.fyd, self.fcd, self.fsd = fyd, fcd, fsd

    def resultant(self, xi):
        up = self.ys > xi
        steel = self.fyd * self.steel_w * self.dy
        conc = self.fcd * self.conc_w * self.dy
        f = np.where(up, steel + conc, -steel)
        n, m = float(np.sum(f)), float(np.sum(f * self.ys))
        for y in (self.bar_y, -self.bar_y):
            fb = self.bar_area / 2 * self.fsd * (1.0 if y > xi else -1.0)
            n, m = n + fb, m + fb * y
        return n, m

    def n_pl(self):
        return float(np.sum(self.fyd * self.steel_w * self.dy + self.fcd * self.conc_w * self.dy)) + self.bar_area * self.fsd

    def n_pm(self):
        return float(np.sum(self.fcd * self.conc_w * self.dy))

    def polygon(self):
        """📐 (N, M) of A, C, D, B."""
        top = float(self.ys[-1] + self.dy / 2)
        lo, hi = -top, top
        for _ in range(80):
            mid = 0.5 * (lo + hi)
            lo, hi = (mid, hi) if self.resultant(mid)[0] > 0 else (lo, mid)
        m_pl = self.resultant(0.5 * (lo + hi))[1]
        m_max = self.resultant(0.0)[1]
        npm = self.n_pm()
        return [(self.n_pl(), 0.0), (npm, m_pl), (npm / 2, m_max), (0.0, m_pl)]


def tube(col, fyd, fcd, fsd):
    """🧱 The equivalent square tube of the stored steel area with its concrete core and two bar layers."""
    b = col["outerSizeM"]
    t = min(max((b - math.sqrt(max(b * b - col["steelAM2"], 0.0))) / 2, 1e-4), b / 4)
    ys = (np.arange(STRIPS) + 0.5) * b / STRIPS - b / 2
    wall = np.abs(ys) >= b / 2 - t
    steel_w = np.where(wall, b, 2 * t)
    conc_w = np.where(wall, 0.0, b - 2 * t)
    return Section(ys, steel_w, conc_w, col["reinforcementAsM2"], max(b / 2 - EDGE, 0.0), fyd, fcd, fsd), t


def encased(col, fyd, fcd, fsd):
    """🧱 The I-section with its concrete from the section leaves (full encasement between flanges and around), or None when the leaves are not set."""
    h, w, tw, tf = col["sectionHeightM"], col["sectionWidthM"], col["sectionTwM"], col["sectionTfM"]
    if min(h, w, tw, tf) <= 1e-6 or col["kind"] not in ("encased", "partiallyEncased"):
        return None
    full = col["kind"] == "encased"
    bc = hc = col["outerSizeM"]
    ext = hc if full else h
    top = max(ext, h) / 2
    ys = (np.arange(STRIPS) + 0.5) * 2 * top / STRIPS - top
    flange = (np.abs(ys) >= h / 2 - tf) & (np.abs(ys) <= h / 2)
    web = np.abs(ys) < h / 2 - tf
    steel_w = np.where(flange, w, np.where(web, tw, 0.0))
    gross = np.where(np.abs(ys) <= ext / 2, bc if full else w, 0.0)
    conc_w = np.maximum(gross - steel_w, 0.0) if full else np.where(web, w - tw, 0.0)
    area_steel = 2 * w * tf + (h - 2 * tf) * tw
    gross_area = bc * hc - area_steel if full else (w - tw) * (h - 2 * tf)
    net = max(gross_area - col["reinforcementAsM2"], 0.0) / max(gross_area, 1e-12)
    cover = col["rebarCoverM"] if col["rebarCoverM"] > 0 else EDGE
    limit = ext / 2 if full else ext / 2 - tf
    return Section(ys, steel_w, conc_w * net, col["reinforcementAsM2"], min(max(ext / 2 - cover, 0.0), max(limit, 0.0)), fyd, fcd, fsd)


def mu_d(poly, n):
    """📐 mu_d(N) along A-C-D-B, the polygon normalised by M_pl,Rd."""
    m_pl = poly[3][1]
    pts = [(poly[0][0], 0.0), (poly[1][0], poly[1][1] / m_pl), (poly[2][0], poly[2][1] / m_pl), (poly[3][0], 1.0)]
    if n >= pts[0][0]:
        return 0.0
    for (n0, m0), (n1, m1) in zip(pts, pts[1:]):
        if n1 <= n <= n0:
            return m0 + (n - n0) / (n1 - n0) * (m1 - m0)
    return 1.0


EPS_C2, EPS_CU = 0.002, 0.0035


def strain_curve(sec, points=400):
    """🌡️ Independent strain-compatibility N-M curve of a strip section (EN 1992-1-1 3.1.7 parabola-rectangle concrete without tension, elastic perfectly plastic steel and bars, plane sections, compression positive): the strain plane runs from tension at the bottom edge through pivot B (eps_cu at the top) to pivot C (eps_c2 at 3/7 of the depth from the top)."""
    top, h = float(sec.ys[-1] + sec.dy / 2), float(sec.ys[-1] - sec.ys[0] + sec.dy)
    depth = top - sec.ys
    curve = []
    for bottom in np.concatenate([np.linspace(-0.02, 0.0, points // 2, endpoint=False), np.linspace(0.0, EPS_C2, points // 2 + 1)]):
        e_top = EPS_CU if bottom <= 0.0 else EPS_C2 + (EPS_C2 - bottom) * 0.75
        strain = lambda d: e_top + (bottom - e_top) * d / h
        e = strain(depth)
        ratio = np.clip(e / EPS_C2, 0.0, 1.0)
        concrete = sec.fcd * (1.0 - (1.0 - ratio) ** 2) * (e > 0)
        steel = np.clip(E_A * e, -sec.fyd, sec.fyd)
        f = concrete * sec.conc_w * sec.dy + steel * sec.steel_w * sec.dy
        n, m = float(np.sum(f)), float(np.sum(f * sec.ys))
        for y in (sec.bar_y, -sec.bar_y):
            fb = sec.bar_area / 2 * float(np.clip(E_S * strain(top - y), -sec.fsd, sec.fsd))
            n, m = n + fb, m + fb * y
        curve.append((n, m))
    return curve


def plastic_curve(sec, points=600):
    """🧱 The exact rigid-plastic N-M curve (plastic neutral axis swept from the top to the bottom edge, no polygon chords)."""
    top = float(sec.ys[-1] + sec.dy / 2)
    return [sec.resultant(xi) for xi in np.linspace(top, -top, points)]


def moment_at(curve, n):
    """📐 The largest |M| of the curve (mirrored about M = 0) at the axial force `n`, 0 outside the curve."""
    best = 0.0
    for (n0, m0), (n1, m1) in zip(curve, curve[1:]):
        if min(n0, n1) <= n <= max(n0, n1) and n0 != n1:
            best = max(best, abs(m0 + (n - n0) / (n1 - n0) * (m1 - m0)))
    return best


def strain_compatibility_bound(col):
    """🔒 (N_Ed, strain-compatible M, exact plastic-curve M, polygon M_Rd) at the first-order axial force of a column. The exact rigid-plastic curve bounds the strain-compatible resistance from above, and the polygon chords lie inside the plastic curve."""
    fy, fyk = col["steelFYPa"], col["reinforcementFYkPa"]
    filled = col["kind"] == "concreteFilled"
    fyd, fcd, fsd = fy / GAMMA["m0"], (1.0 if filled else 0.85) * col["concreteFCkPa"] / GAMMA["c"], fyk / GAMMA["s"]
    sec = tube(col, fyd, fcd, fsd)[0] if encased(col, fyd, fcd, fsd) is None else encased(col, fyd, fcd, fsd)
    n_ed = design_effects(col)[0]
    poly = sec.polygon()
    return n_ed, moment_at(strain_curve(sec), n_ed), moment_at(plastic_curve(sec), n_ed), mu_d(poly, n_ed) * poly[3][1]


def checks(doc, col):
    """🧩 The N_pl,Rd buckling and the M-N interaction utilisations of one composite column."""
    fy, fyk = col["steelFYPa"], col["reinforcementFYkPa"]
    filled = col["kind"] == "concreteFilled"
    factor = 1.0 if filled else 0.85
    fyd, fcd, fsd = fy / GAMMA["m0"], factor * col["concreteFCkPa"] / GAMMA["c"], fyk / GAMMA["s"]
    sec, t = tube(col, fyd, fcd, fsd)
    b = col["outerSizeM"]
    n_pl = col["steelAM2"] * fyd + col["concreteAM2"] * fcd + col["reinforcementAsM2"] * fsd
    n_pl_rk = col["steelAM2"] * fy + factor * col["concreteAM2"] * col["concreteFCkPa"] + col["reinforcementAsM2"] * fyk
    n_ed, m1, ng = design_effects(col)
    ecm = ecm_pa(col["concreteFCkPa"])
    i_s = col["reinforcementAsM2"] * max(b / 2 - EDGE, 0.0) ** 2
    i_c = (b - 2 * t) ** 4 / 12 if filled else max(b ** 4 / 12 - col["iM4"], 1e-9)
    length = max(col["lengthM"], 0.1)

    def ncr(modulus, ke, reduction=1.0):
        return math.pi ** 2 * reduction * (E_A * col["iM4"] + E_S * i_s + ke * modulus * i_c) / length ** 2

    lam0 = math.sqrt(n_pl_rk / max(ncr(ecm, 0.6), 1.0))
    ecm_eff = ecm / (1 + ng / max(n_ed, 1e-9) * 2.5) if lam0 > (0.8 if filled else 0.5) else ecm
    lam = math.sqrt(n_pl_rk / max(ncr(ecm_eff, 0.6), 1.0))
    alpha = ALPHA_CURVE.get(col["bucklingCurve"], 0.76)
    phi = 0.5 * (1 + alpha * (lam - 0.2) + lam * lam)
    chi = min(1.0 / (phi + math.sqrt(max(phi * phi - lam * lam, 0.0))), 1.0)
    out = [(f"en1994.6.7.3.npl.{col['id']}", n_ed / (chi * n_pl))]
    ncr_eff = ncr(ecm_eff, 0.5, 0.9)
    k = 1.0 if n_ed / ncr_eff <= 0.1 else max(1.0 / (1 - n_ed / ncr_eff), 1.0)
    true = encased(col, fyd, fcd, fsd)
    if true is not None:
        poly = true.polygon()
    else:
        poly = sec.polygon()
        scale = n_pl / poly[0][0]
        poly = [(n * scale, m) for n, m in poly]
    m_ed = k * m1
    m_rd = (0.9 if fy <= 355.1e6 else 0.8) * mu_d(poly, n_ed) * poly[3][1]
    util = n_ed / n_pl if m_ed < 1.0 else 1e6 if m_rd < 1.0 else m_ed / m_rd
    out.append((f"en1994.6.7.3.mn.{col['id']}", util))
    return out


def self_check():
    """🧪 The steel-only plastic moment of the strip tube against the plastic modulus of a sectionproperties square hollow section, and the filled tube against the closed form of a concrete core."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import rectangular_hollow_section

    col = {"outerSizeM": 0.3, "steelAM2": 4 * 0.01 * (0.3 - 0.01), "reinforcementAsM2": 0.0}
    sec, t = tube(col, 355e6, 0.0, 0.0)
    ours = sec.polygon()[3][1] / 355e6
    geometry = rectangular_hollow_section(d=300.0, b=300.0, t=t * 1e3, r_out=0.0, n_r=1)
    geometry.create_mesh(mesh_sizes=[20.0])
    section = Section(geometry)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    theirs = section.get_s()[0] * 1e-9
    assert abs(ours - theirs) <= 5e-3 * theirs, (ours, theirs)
    core = (0.3 - 2 * t) ** 2
    filled, _ = tube(col, 355e6, 20e6, 0.0)
    n_pm = filled.n_pm()
    assert abs(n_pm - core * 20e6) <= 1e-3 * n_pm, (n_pm, core * 20e6)
    curve, plastic = strain_curve(filled), plastic_curve(filled)
    for n in (0.5e6, 1.5e6, 3.0e6):
        assert moment_at(curve, n) <= 1.01 * moment_at(plastic, n), n
    return {"tubeWpl": ours, "sectionproperties": theirs}
