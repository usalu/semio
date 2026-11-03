"""🧭 Independent EN 1994-1-1 check families of a composite beam and slab that had no Python twin: section class (5.5, EN 1993-1-1 Table 5.2), stud spacing (6.6.5.5) and layout (6.6.1.1), longitudinal shear of the slab (6.6.6, EN 1992-1-1 6.2.4), bending-shear interaction (6.2.2.4, reduced web strength (1 - rho) f_yd integrated over strips, not the closed web term), SLS stress limitation (7.2.2, staged elastic stresses) and composite slab punching (9.7.6, EN 1992-1-1 6.2.2). Every function takes plain numbers or the strip arrays of the caller's section. SI throughout."""

import math

import numpy as np

STUD_MIN_FACTOR, STUD_MAX_FACTOR, STUD_MAX_ABS = 5.0, 6.0, 0.8
REBAR_FSK, PUNCHING_FOOTPRINT = 500e6, 0.2


def root_radius(steel, area_catalogue, r_given):
    """🔘 The root radius: the given leaf, else implied by the catalogue area (the fillets add (4 - pi) r^2 to the three plates), at most half the outstand and half the web."""
    if r_given > 0:
        return r_given
    hw = max(steel["heightM"] - 2 * steel["tfM"], 0.0)
    fillets = area_catalogue - 2 * steel["widthM"] * steel["tfM"] - hw * steel["twM"]
    limit = max(min(0.5 * (steel["widthM"] - steel["twM"]), 0.5 * hw), 0.0)
    return 0.0 if fillets <= 0 else min(math.sqrt(fillets / (4 - math.pi)), limit)


def section_class(steel, fy):
    """🏷️ Class of the flange outstand (9, 10, 14 eps) and the web in bending (72, 83, 124 eps), c = (b - t_w) / 2 - r and c_w = h - 2 t_f - 2 r."""
    eps = math.sqrt(235e6 / fy)
    r = root_radius(steel, steel["aM2"], steel.get("rM", 0.0))
    ratios = (max((steel["widthM"] - steel["twM"]) / 2 - r, 1e-6) / steel["tfM"], max(steel["heightM"] - 2 * steel["tfM"] - 2 * r, 1e-6) / steel["twM"])
    limits = ((9, 10, 14), (72, 83, 124))
    return max(next((i + 1 for i, lim in enumerate(lims) if ratio <= lim * eps), 4) for ratio, lims in zip(ratios, limits))


STATION_TOLERANCE, MID_TOLERANCE, EDGE_TOLERANCE = 1e-3, 1e-3, 1e-6


def beam_studs(doc, beam):
    """🔩 The rows of the persisted `studs[]` collection that belong to `beam`, ascending in x."""
    return sorted((s for s in doc.get("studs", []) if s["beamId"] == beam["id"]), key=lambda s: (s["xM"], s["id"]))


def station_groups(rows):
    """🔩 Studs within 1 mm along the beam axis form one station (the studs of one rib, n_r)."""
    groups, first = [], None
    for row in rows:
        if first is None or row["xM"] - first > STATION_TOLERANCE:
            groups.append([])
            first = row["xM"]
        groups[-1].append(row)
    return groups


def design_origin(beam):
    """📍 Start of the designed span in the beam frame: after the neighbouring span for an interior span and a cantilever (the neighbouring span is the span itself when unset)."""
    adjacent = beam.get("adjacentSpanM") or 0.0
    return (adjacent if adjacent > 1e-9 else beam["spanM"]) if beam["support"] in ("interiorSpan", "cantilever") else 0.0


def in_span(beam, row):
    lo = design_origin(beam)
    return lo - EDGE_TOLERANCE <= row["xM"] <= lo + beam["spanM"] + EDGE_TOLERANCE


def left_weight(beam, row):
    """⚖️ 1 for a stud of the left half of the span, 0 for the right half, 1/2 for a stud on the middle (it belongs to both halves with half its resistance)."""
    d = row["xM"] - (design_origin(beam) + beam["spanM"] / 2)
    return 0.5 if abs(d) <= MID_TOLERANCE else (1.0 if d < 0 else 0.0)


def half_resistances(beam, rows, resistance):
    """🔩 (left, right) sums of w * P_Rd over the studs inside the span; `resistance(row)` is the design resistance of one stud."""
    inside = [r for r in rows if in_span(beam, r)]
    left = sum(left_weight(beam, r) * resistance(r) for r in inside)
    return left, sum((1.0 - left_weight(beam, r)) * resistance(r) for r in inside)


def stud_spacing(beam, hc, rows):
    """📏 The worst gap utilisation max(s / s_max, 5 d / s) over the gaps between neighbouring stud stations inside the span (d = the larger diameter of the two sides, s_max = min(6 h_c, 800 mm)); None below two stations."""
    groups = station_groups([r for r in rows if in_span(beam, r)])
    s_max = min(STUD_MAX_FACTOR * hc, STUD_MAX_ABS)
    worst = [max((nxt[0]["xM"] - prev[-1]["xM"]) / s_max, STUD_MIN_FACTOR * max(prev[-1]["diameterM"], nxt[0]["diameterM"]) / max(nxt[0]["xM"] - prev[-1]["xM"], 1e-9)) for prev, nxt in zip(groups, groups[1:])]
    return max(worst) if worst else None


def stud_layout(beam, rows):
    """🧱 (all studs, studs inside the span): the layout holds while no stud lies outside the span."""
    return len(rows), sum(1 for r in rows if in_span(beam, r))


def per_stud_utilisation(beam, rows, resistance, flow, samples=20000):
    """⚖️ The largest P_Ed / P_Rd of the studs for the uniform shear flow `flow` [N/m] on each half span, by sampling: every point of a half belongs to the nearest stud station of that half, the station shares the force of its points between its studs in proportion to w * P_Rd. None without studs inside the span."""
    inside = [r for r in rows if in_span(beam, r)]
    if not inside:
        return None
    lo, hi = design_origin(beam), design_origin(beam) + beam["spanM"]
    mid = (lo + hi) / 2
    force = {r["id"]: 0.0 for r in inside}
    for left, (a, b) in ((True, (lo, mid)), (False, (mid, hi))):
        weight = lambda r: left_weight(beam, r) if left else 1.0 - left_weight(beam, r)
        stations = [g for g in station_groups([r for r in inside if weight(r) > 0])]
        if not stations:
            continue
        xs = np.array([g[0]["xM"] for g in stations])
        points = a + (np.arange(samples) + 0.5) * (b - a) / samples
        owner = np.abs(points[:, None] - xs[None, :]).argmin(axis=1)
        for k, group in enumerate(stations):
            share = flow * float((owner == k).sum()) * (b - a) / samples
            total = sum(weight(r) * resistance(r) for r in group)
            for r in group:
                force[r["id"]] += share * weight(r) * resistance(r) / total if total > 1e-9 else 0.0
    return max(force[r["id"]] / resistance(r) for r in inside)


def slab_longitudinal_shear(n_req, equivalent_span, hf, fck, gamma_c, gamma_s, a_sf_provided):
    """🪢 EN 1992-1-1 6.2.4 for the two shear planes at 2 h_f: v_Ed = (N_c,req / (L_e / 2)) / (2 h_f), v_Rd,max = nu f_cd sin(theta) cos(theta), A_sf f_yd / s_f >= v_Ed h_f cot(theta); theta = 45 degrees. Utilisation = the larger of the strut and the reinforcement ratio."""
    flow = n_req / max(equivalent_span / 2, 1e-9)
    v_ed = flow / (2 * hf)
    nu = 0.6 * (1 - fck / 1e6 / 250)
    v_rd = nu * fck / gamma_c * math.sin(math.pi / 4) * math.cos(math.pi / 4)
    a_req = v_ed * hf / (REBAR_FSK / gamma_s)
    return max(v_ed / max(v_rd, 1.0), a_req / max(a_sf_provided, 1e-12))


def rho(v_ed, v_pl):
    """🔻 EN 1994-1-1 6.2.2.4: rho = (2 V_Ed / V_pl,Rd - 1)^2 above V_Ed = V_pl,Rd / 2, else 0; capped at 1 (the web carries nothing; V_Ed > V_pl,Rd fails the shear row)."""
    return min((2 * v_ed / v_pl - 1) ** 2, 1.0) if v_ed > 0.5 * v_pl else 0.0


def _axis_moment(steel, strength, target, nc=0.0, x=0.0, rebar=0.0, y_rebar=0.0, sagging=False):
    """⚖️ Plastic moment about the slab top of the strips with the given strengths: the axis where the strip force above minus below equals the target (concrete block N_c for sagging, bar force for hogging)."""
    depth, area = steel.depth, steel.area_strip * strength
    lo, hi = steel.ys, steel.ys + steel.h
    for _ in range(100):
        y = 0.5 * (lo + hi)
        net = np.sum(area[depth > y]) - np.sum(area[depth <= y])
        lo, hi = (y, hi) if net > target else (lo, y)
    y = 0.5 * (lo + hi)
    below, above = depth > y, depth <= y
    if sagging:
        return np.sum(area[below] * depth[below]) - np.sum(area[above] * depth[above]) - nc * x / 2
    return np.sum(area[below] * depth[below]) - np.sum(area[above] * depth[above]) - rebar * y_rebar


def web_strength(steel, fyd, reduction):
    """🕸️ Strip strengths with (1 - rho) f_yd in the web strips between the flanges."""
    web = (steel.depth > steel.ys + steel.tf) & (steel.depth < steel.ys + steel.h - steel.tf)
    return np.where(web, fyd * (1 - reduction), fyd)


def reduced_sagging(steel, fyd, beff, hc, fcc, red):
    """📐 Full-connection sagging moment with the reduced web strength: the concrete force is min(A_a f_yd, f_cc b_eff h_c) of the unreduced section, capped by the reduced steel force."""
    strength = web_strength(steel, fyd, red)
    nc = min(steel.area * fyd, fcc * beff * hc, float(np.sum(steel.area_strip * strength)))
    x = nc / (fcc * beff)
    return _axis_moment(steel, strength, nc, nc=nc, x=x, sagging=True)


def reduced_hogging(steel, fyd, as_total, fsd, y_rebar, red):
    """📐 Hogging moment with the reduced web strength and the bar force min(A_s f_sd, A_a f_yd) in tension at y_rebar."""
    strength = web_strength(steel, fyd, red)
    fs = min(as_total * fsd, float(np.sum(steel.area_strip * strength)))
    return _axis_moment(steel, strength, fs, rebar=fs, y_rebar=y_rebar)


def sagging_interaction(steel, fyd, beff, hc, fcc, m_ed, v_ed, v_pl, samples=200):
    """🔀 The worst M_Ed(x) / M_V,Rd(x) along a simply supported span under a uniform load, M(x) = 4 M s (1 - s) and V(x) = V |1 - 2 s| at s = k / samples."""
    worst = 0.0
    for k in range(samples + 1):
        s = k / samples
        m, v = 4 * m_ed * s * (1 - s), v_ed * abs(1 - 2 * s)
        worst = max(worst, m / reduced_sagging(steel, fyd, beff, hc, fcc, rho(v, v_pl)))
    return worst


def stress_limit(steel, ys, beff, hc, n0, nl, m_a, m_g, m_q, fy, fck, transformed):
    """🧵 EN 1994-1-1 7.2.2: staged elastic stresses, steel alone for M_a, long-term section for M_G, short-term for M_Q; utilisation max(sigma_a / f_y, sigma_c / (0.6 f_ck)), compression positive."""
    half = steel.h / 2
    top0, bot0 = m_a * half / steel.i_y, -m_a * half / steel.i_y
    y_l, i_l = transformed(steel, ys, beff, hc, nl)
    y_0, i_0 = transformed(steel, ys, beff, hc, n0)
    at = lambda m, y_na, inertia, y: m * (y_na - y) / inertia
    top = top0 + at(m_g, y_l, i_l, ys) + at(m_q, y_0, i_0, ys)
    bottom = bot0 + at(m_g, y_l, i_l, ys + steel.h) + at(m_q, y_0, i_0, ys + steel.h)
    sigma_c = max(at(m_g, y_l, i_l, 0.0) / nl + at(m_q, y_0, i_0, 0.0) / n0, 0.0)
    return max(max(abs(top), abs(bottom)) / fy, sigma_c / (0.6 * fck))


def punching(slab, f_ed, gamma_c, d_p, a_p, b_o, hc):
    """🔨 EN 1994-1-1 9.7.6 with EN 1992-1-1 6.2.2: V_p,Rd = C_crit h_c v_Rd,c, C_crit the perimeter at 2 d_p around the 200 mm square footprint (4 a + 2 pi 2 d_p), v_Rd,c = max((0.18 / gamma_c) k (100 rho f_ck)^(1/3), 0.035 k^(3/2) sqrt(f_ck)), k = min(2, 1 + sqrt(200 / d_p[mm])), rho = min(A_p / (b_o d_p), 0.02)."""
    fck = slab["fCkPa"] / 1e6
    k = min(1 + math.sqrt(200 / (d_p * 1e3)), 2.0)
    rho_l = min(a_p / (b_o * d_p), 0.02)
    v_c = max(0.18 / gamma_c * k * (100 * rho_l * fck) ** (1 / 3), 0.035 * k ** 1.5 * math.sqrt(fck))
    c_crit = 4 * PUNCHING_FOOTPRINT + 2 * math.pi * 2 * d_p
    return f_ed / (c_crit * hc * v_c * 1e6)


def stud_self_check():
    """🧪 The per-stud twin against closed forms: ten equally spaced studs carry flow * L / 10 each, a stud on the middle belongs to both halves with half its resistance, the spacing and layout rows follow the stations (numpy is the third-party sampler)."""
    beam = {"id": "b", "support": "simplySupported", "spanM": 8.0}
    rows = [{"id": f"s{k}", "beamId": "b", "xM": (k + 0.5) * 0.8, "diameterM": 0.019, "heightM": 0.095, "fUPa": 450e6} for k in range(10)]
    unit = lambda r: 1.0
    u = per_stud_utilisation(beam, rows, unit, 1000.0)
    assert abs(u - 1000.0 * 0.8) < 1e-6 * 800, u
    middle = [dict(rows[0], xM=2.0), dict(rows[1], xM=4.0), dict(rows[2], xM=6.0)]
    assert half_resistances(beam, middle, unit) == (1.5, 1.5)
    assert abs(stud_spacing(beam, 0.085, rows) - max(0.8 / 0.51, 0.095 / 0.8)) < 1e-9
    assert stud_layout(beam, rows + [dict(rows[0], id="out", xM=9.0)]) == (11, 10)
    assert station_groups([dict(r, xM=0.5) for r in rows[:2]] + rows[2:3]) and [len(g) for g in station_groups(sorted([dict(rows[0], xM=0.5), dict(rows[1], xM=0.5004), dict(rows[2], xM=1.0)], key=lambda r: r["xM"]))] == [2, 1]
    return {"uniformForceN": u, "middleHalves": 1.5}


def self_check(steel_class, transformed):
    """🧪 Third-party check with sectionproperties: the transformed composite section (area, centroid and second moment of the steel I plus a concrete block of modulus 1 / n) against `transformed`, and the plastic moment of the steel I with a reduced web strength (1 - rho) f_y (composite materials, N = 0) against the strip integration."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre import Material
    from sectionproperties.pre.library import rectangular_section

    h, b, tf, tw, ys, beff, hc, n, red, fy = 0.3, 0.3, 0.019, 0.011, 0.14, 2.5, 0.085, 15.0, 0.4, 355e6
    steel = steel_class({"heightM": h, "widthM": b, "tfM": tf, "twM": tw, "aM2": 2 * b * tf + (h - 2 * tf) * tw, "iYM4": 0.0}, ys)
    mm = lambda x: x * 1e3
    flange_mat = Material("flange", 1.0, 0.3, fy / 1e6, 1.0, "grey")
    web_mat = Material("web", 1.0, 0.3, fy / 1e6 * (1 - red), 1.0, "red")
    concrete = Material("concrete", 1.0 / n, 0.2, 1.0, 1.0, "blue")
    flange = lambda y: rectangular_section(d=mm(tf), b=mm(b), material=flange_mat).shift_section(-mm(b) / 2, y)
    web = rectangular_section(d=mm(h - 2 * tf), b=mm(tw), material=web_mat).shift_section(-mm(tw) / 2, mm(tf))
    section_geometry = flange(0.0) + web + flange(mm(h - tf))
    section_geometry.create_mesh(mesh_sizes=[4.0])
    section = Section(section_geometry)
    section.calculate_geometric_properties()
    section.calculate_plastic_properties()
    theirs = section.get_mp()[0] * 1e-3
    ours = reduced_hogging(steel, fy, 0.0, 0.0, 0.0, red)
    ours_about_axis = ours - 0.0
    assert abs(abs(ours_about_axis) - theirs) <= 2e-3 * theirs, (ours_about_axis, theirs)
    slab = rectangular_section(d=mm(hc), b=mm(beff), material=concrete).shift_section(-mm(beff) / 2, -mm(hc))
    steel_geo = (flange(0.0) + web + flange(mm(h - tf))).shift_section(0.0, 0.0)
    composite = slab + steel_geo
    composite.create_mesh(mesh_sizes=[20.0])
    section = Section(composite)
    section.calculate_geometric_properties()
    y_na_theirs = section.get_c()[1] * 1e-3
    inertia_theirs = section.get_eic()[0] * 1e-12
    y_ours, i_ours = transformed(steel_class({"heightM": h, "widthM": b, "tfM": tf, "twM": tw, "aM2": 2 * b * tf + (h - 2 * tf) * tw, "iYM4": (b * h ** 3 - (b - tw) * (h - 2 * tf) ** 3) / 12}, hc), hc, beff, hc, n)
    assert abs(i_ours - inertia_theirs) <= 5e-3 * inertia_theirs, (i_ours, inertia_theirs)
    return {"reducedWebMpl": theirs, "reducedWebStrip": abs(ours_about_axis), "transformedI": inertia_theirs, "transformedIOurs": i_ours}
