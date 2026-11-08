"""🪜️ Independent oracle of the EN 1993-1-1 6.4 built-up member check (chi by the closed Phi form, Table 6.9 limits). Run with `python3 -I`; prints one JSON object for the showcase row (battened, 2 planes, HEA 200 chords) and the third-party cross-check of the clause quantities: the chord section of `sectionproperties`, the shear stiffness S_v of the battened ladder (V-132, check en1993.1-1.6.4.chord) and of the laced systems 1 and 2 of Fig. 6.9 (V-131, check en1993.1-1.6.4.chord) from `anastruct` frame and truss models by the difference of the tip deflection of N + 1 and N panels, the closely spaced limits of Table 6.9 (V-133, check en1993.1-1.6.4.4.spacing) from the radius of gyration of an `angle_section`, and the general conditions of V-130 (check en1993.1-1.6.4.chord) as the buckling load N_cr,tot = 1 / (1 / N_cr + 1 / S_v) of a shear flexible column by `scipy.linalg.eigh`."""
import json
import math

import numpy as np
from anastruct import SystemElements
from scipy import linalg
from sectionproperties.analysis import Section
from sectionproperties.pre.library import angle_section, i_section

SPEC_ROWS = {
    "V-130": ("en1993.1-1.6.4.chord", "scipy.linalg.eigh shear flexible column against 1 / N_cr + 1 / S_v"),
    "V-131": ("en1993.1-1.6.4.chord", "anastruct truss, Fig. 6.9 system 1 and 2"),
    "V-132": ("en1993.1-1.6.4.chord", "anastruct frame ladder"),
    "V-133": ("en1993.1-1.6.4.4.spacing", "sectionproperties angle_section i_min"),
}


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


def ladder_tip(panels, a, h0, e, i_ch, i_b):
    """🪜️ Tip deflection of a Vierendeel ladder (axially rigid chords) fixed at the left end under the shear 1 kN at the right end."""
    s = SystemElements()
    ea = 1.0e12
    for k in range(panels):
        for y in (0.0, h0):
            s.add_element([[k * a, y], [(k + 1) * a, y]], EA=ea, EI=e * i_ch)
    for k in range(panels + 1):
        s.add_element([[k * a, 0.0], [k * a, h0]], EA=ea, EI=e * i_b)
    for y in (0.0, h0):
        s.add_support_fixed(s.find_node_id([0.0, y]))
        s.point_load(s.find_node_id([panels * a, y]), Fy=-500.0)
    s.solve()
    return abs(s.get_node_displacements(s.find_node_id([panels * a, 0.0]))["uy"])


def lacing_tip(panels, a, h0, e, a_d, system):
    """🪜️ Tip deflection of a pinned lacing truss (axially rigid chords and posts) fixed by two hinges at the left end under 1 kN; system 1: diagonals alternating over the panel length a (Fig. 6.9 a), system 2: one diagonal and one post per panel."""
    s = SystemElements()
    link = lambda p, q, k: s.add_truss_element([p, q], EA=k)
    rigid = 1.0e12
    if system == 2:
        for k in range(panels):
            link([k * a, 0.0], [(k + 1) * a, 0.0], rigid)
            link([k * a, h0], [(k + 1) * a, h0], rigid)
            link([k * a, h0], [(k + 1) * a, 0.0], e * a_d)
        for k in range(panels + 1):
            link([k * a, 0.0], [k * a, h0], rigid)
        hinges = [[0.0, 0.0], [0.0, h0]]
    else:
        for k in range(panels):
            x = k * a
            link([x, 0.0], [x + a, 0.0], rigid)
            link([x + a / 2, h0], [x + 3 * a / 2, h0], rigid)
            link([x, 0.0], [x + a / 2, h0], e * a_d)
            link([x + a / 2, h0], [x + a, 0.0], e * a_d)
        hinges = [[0.0, 0.0], [a / 2, h0]]
    for point in hinges:
        s.add_support_hinged(s.find_node_id(point))
    end = s.find_node_id([panels * a, 0.0])
    s.point_load(end, Fy=-1000.0)
    s.solve()
    return abs(s.get_node_displacements(end)["uy"])


def shear_column_load(length, e, i_eff, sv, n=48):
    """🪜️ Smallest buckling load of a pinned column with the shear stiffness S_v (Timoshenko kinematics w and theta, one point shear integration) by `scipy.linalg.eigh`."""
    h = length / n
    size = n + 1
    k = np.zeros((2 * size, 2 * size))
    g = np.zeros_like(k)
    for j in range(n):
        dofs = [j, j + 1, size + j, size + j + 1]
        b = np.array([-1.0 / h, 1.0 / h, -0.5, -0.5])
        k[np.ix_(dofs, dofs)] += sv * h * np.outer(b, b)
        k[np.ix_(dofs[2:], dofs[2:])] += e * i_eff / h * np.array([[1.0, -1.0], [-1.0, 1.0]])
        g[np.ix_(dofs[:2], dofs[:2])] += np.array([[1.0, -1.0], [-1.0, 1.0]]) / h
    free = [i for i in range(2 * size) if i not in (0, n)]
    mu = linalg.eigh(g[np.ix_(free, free)], k[np.ix_(free, free)], eigvals_only=True)
    return float(1.0 / mu.max())


def polygon_moments(points):
    """🪜️ Area and centroidal second moments (I_x, I_y, I_xy) of a simple polygon by the shoelace sums."""
    x, y = np.array([p[0] for p in points]), np.array([p[1] for p in points])
    x1, y1 = np.roll(x, -1), np.roll(y, -1)
    cross = x * y1 - x1 * y
    area = cross.sum() / 2
    cx, cy = ((x + x1) * cross).sum() / (6 * area), ((y + y1) * cross).sum() / (6 * area)
    ixx = ((y * y + y * y1 + y1 * y1) * cross).sum() / 12 - area * cy * cy
    iyy = ((x * x + x * x1 + x1 * x1) * cross).sum() / 12 - area * cx * cx
    ixy = ((x * y1 + 2 * x * y + 2 * x1 * y1 + x1 * y) * cross).sum() / 24 - area * cx * cy
    return area, ixx, iyy, ixy


def cross_check():
    """🧪 Third-party cross-check of the clause quantities of 6.4 (see the module docstring)."""
    e, h0, a, a_ch, i_ch, i_b = 210000.0, 300.0, 800.0, 5380.0, 1.336e7, 5.0e6
    geometry = i_section(d=190.0, b=200.0, t_f=10.0, t_w=6.5, r=18.0, n_r=8)
    geometry.create_mesh(mesh_sizes=[20.0])
    chord = Section(geometry)
    chord.calculate_geometric_properties()
    assert abs(chord.get_area() - a_ch) <= 0.015 * a_ch and abs(chord.get_ic()[1] - i_ch) <= 0.015 * i_ch, (chord.get_area(), chord.get_ic())
    metre = lambda mm: mm / 1e3
    tip = lambda n: ladder_tip(n, metre(a), metre(h0), e * 1e6, i_ch * 1e-12, i_b * 1e-12)
    sv_fe = 1000.0 * metre(a) / (tip(7) - tip(6))
    sv_closed = 24 * e * i_ch / (a * a * (1 + 2 * i_ch / i_b * h0 / a))
    assert abs(sv_fe - sv_closed) <= 0.04 * sv_closed, (sv_fe, sv_closed)
    out = {"chord": {"area": chord.get_area(), "iz": chord.get_ic()[1]}, "battenedSv": {"anastruct": sv_fe, "closed": sv_closed}}
    a_d = 6.0e-4
    for system in (1, 2):
        run = metre(a) / (2 if system == 1 else 1)
        d = math.hypot(run, metre(h0))
        tips = [lacing_tip(n, metre(a), metre(h0), 2.1e11, a_d, system) for n in (7, 8)]
        sv_fe = 1000.0 * metre(a) / (tips[1] - tips[0])
        sv_closed = 2.1e11 * a_d * metre(a) * metre(h0) ** 2 / ((2 if system == 1 else 1) * d**3)
        assert abs(sv_fe - sv_closed) <= 0.04 * sv_closed, (system, sv_fe, sv_closed)
        out[f"lacedSystem{system}Sv"] = {"anastruct": sv_fe, "closed": sv_closed}
    length = 4.0
    i_eff = 0.5 * h0 * h0 * a_ch + 2 * i_ch
    n_cr = math.pi**2 * e * 1e6 * i_eff * 1e-12 / length**2
    sv = 24 * e * i_ch / (a * a * (1 + 2 * i_ch / i_b * h0 / a))
    total = shear_column_load(length, e * 1e6, i_eff * 1e-12, sv)
    assert abs(total - 1.0 / (1.0 / n_cr + 1.0 / sv)) <= 0.01 * total, (total, n_cr, sv)
    out["generalConditions"] = {"scipy": total, "closed": 1.0 / (1.0 / n_cr + 1.0 / sv)}
    t = 8.0
    points = [(0, 0), (80, 0), (80, t), (t, t), (t, 80), (0, 80)]
    angle_geometry = angle_section(d=80.0, b=80.0, t=t, r_r=0.0, r_t=0.0, n_r=1)
    angle_geometry.create_mesh(mesh_sizes=[2.0])
    angle = Section(angle_geometry)
    angle.calculate_geometric_properties()
    i_min = math.sqrt(min(angle.get_ip()) / angle.get_area())
    area, ixx, iyy, ixy = polygon_moments(points)
    closed = math.sqrt(((ixx + iyy) / 2 - math.hypot((ixx - iyy) / 2, ixy)) / area)
    assert abs(i_min - closed) <= 0.01 * closed, (i_min, closed)
    out["closelySpaced"] = {"iMin": i_min, "closed": closed, "angleLimit": 15 * i_min, "otherLimit": 70 * i_min}
    return out


if __name__ == "__main__":
    print(json.dumps({"showcase": battened(4000.0, 300.0, 800.0, 5380.0, 1.336e7, 5.0e6, 210000.0, 355.0, 1.1, 2.0, 400000.0, 5.0e6), "closely_limit_m": {"angles": 15 * 0.04, "other": 70 * 0.04}, "crossCheck": cross_check()}))
