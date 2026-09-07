"""🔬️ W7 kernel reference values.

Computes every number hard-coded in the `⚙️engine` `#[cfg(test)]` benchmark assertions, either
in closed form or with an independent third party (scikit-fem 12.0.2 / scipy / numpy).

Run: `uv run --no-sync python <this file> [section ...]`
Sections: modal buckling cook plate hex beams macneal (default: all).
"""

from __future__ import annotations

import sys

import numpy as np
import scipy.linalg as sla


def banner(title: str) -> None:
    print("\n" + "#" * 78)
    print("## " + title)
    print("#" * 78)


# ---------------------------------------------------------------- 2D frame kit
def frame_k_local(ea_over_l, ei, length):
    l, l2, l3 = length, length**2, length**3
    return np.array(
        [
            [ea_over_l, 0, 0, -ea_over_l, 0, 0],
            [0, 12 * ei / l3, 6 * ei / l2, 0, -12 * ei / l3, 6 * ei / l2],
            [0, 6 * ei / l2, 4 * ei / l, 0, -6 * ei / l2, 2 * ei / l],
            [-ea_over_l, 0, 0, ea_over_l, 0, 0],
            [0, -12 * ei / l3, -6 * ei / l2, 0, 12 * ei / l3, -6 * ei / l2],
            [0, 6 * ei / l2, 2 * ei / l, 0, -6 * ei / l2, 4 * ei / l],
        ]
    )


def frame_m_local(rho_a, length):
    l, l2 = length, length**2
    return (
        rho_a
        * l
        / 420.0
        * np.array(
            [
                [140, 0, 0, 70, 0, 0],
                [0, 156, 22 * l, 0, 54, -13 * l],
                [0, 22 * l, 4 * l2, 0, 13 * l, -3 * l2],
                [70, 0, 0, 140, 0, 0],
                [0, 54, 13 * l, 0, 156, -22 * l],
                [0, -13 * l, -3 * l2, 0, -22 * l, 4 * l2],
            ]
        )
    )


def frame_kg_local(axial_n, length):
    l, l2 = length, length**2
    kg = np.zeros((6, 6))
    block = np.array(
        [
            [6 / 5, l / 10, -6 / 5, l / 10],
            [l / 10, 2 * l2 / 15, -l / 10, -l2 / 30],
            [-6 / 5, -l / 10, 6 / 5, -l / 10],
            [l / 10, -l2 / 30, -l / 10, 2 * l2 / 15],
        ]
    )
    idx = [1, 2, 4, 5]
    kg[np.ix_(idx, idx)] = (axial_n / l) * block
    return kg


def frame_transform(c, s):
    t = np.zeros((6, 6))
    for o in (0, 3):
        t[o + 0, o + 0], t[o + 0, o + 1] = c, s
        t[o + 1, o + 0], t[o + 1, o + 1] = -s, c
        t[o + 2, o + 2] = 1.0
    return t


def frame_udl_local(length, wx, wy):
    l, l2 = length, length**2
    return np.array(
        [wx * l / 2, wy * l / 2, wy * l2 / 12, wx * l / 2, wy * l / 2, -wy * l2 / 12]
    )


class Frame2d:
    """🏗️ Minimal independent 2D-frame direct-stiffness solver (numpy/scipy only)."""

    def __init__(self, coords):
        self.coords = np.asarray(coords, float)
        self.n = len(coords)
        self.members = []
        self.k = np.zeros((3 * self.n, 3 * self.n))
        self.f = np.zeros(3 * self.n)

    def dofs(self, a, b):
        return [3 * a, 3 * a + 1, 3 * a + 2, 3 * b, 3 * b + 1, 3 * b + 2]

    def geometry(self, a, b):
        d = self.coords[b] - self.coords[a]
        length = float(np.hypot(d[0], d[1]))
        return length, d[0] / length, d[1] / length

    def add_member(self, a, b, e, area, inertia):
        length, c, s = self.geometry(a, b)
        t = frame_transform(c, s)
        kg = t.T @ frame_k_local(e * area / length, e * inertia, length) @ t
        self.k[np.ix_(self.dofs(a, b), self.dofs(a, b))] += kg
        self.members.append((a, b, e, area, inertia))
        return len(self.members) - 1

    def add_udl(self, member, wx, wy):
        a, b, *_ = self.members[member]
        length, c, s = self.geometry(a, b)
        wxl, wyl = wx * c + wy * s, -wx * s + wy * c
        t = frame_transform(c, s)
        self.f[self.dofs(a, b)] += t.T @ frame_udl_local(length, wxl, wyl)

    def add_load(self, node, dof, value):
        self.f[3 * node + dof] += value

    def solve(self, fixed):
        free = [i for i in range(3 * self.n) if i not in set(fixed)]
        u = np.zeros(3 * self.n)
        u[free] = np.linalg.solve(self.k[np.ix_(free, free)], self.f[free])
        reactions = self.k @ u - self.f
        return u, reactions


def cantilever_beam_system(n_elements, length, e, inertia, area, density):
    dl = length / n_elements
    ndof = 3 * (n_elements + 1)
    k = np.zeros((ndof, ndof))
    m = np.zeros((ndof, ndof))
    kl = frame_k_local(e * area / dl, e * inertia, dl)
    ml = frame_m_local(density * area, dl)
    for i in range(n_elements):
        idx = [3 * i, 3 * i + 1, 3 * i + 2, 3 * i + 3, 3 * i + 4, 3 * i + 5]
        k[np.ix_(idx, idx)] += kl
        m[np.ix_(idx, idx)] += ml
    return k, m


# --------------------------------------------------------------------- 1 modal
def section_modal():
    banner("1 · modal cantilever (BeamEb2, consistent mass)")
    e, iy, area, density, length = 200e9, 1e-5, 0.01, 7850.0, 3.0
    beta_l = np.array([1.875104068711961, 4.694091132974175, 7.854757438237613])
    analytic = beta_l**2 / (2 * np.pi * length**2) * np.sqrt(e * iy / (density * area))
    print("beta_n*L roots of cos(x)cosh(x)+1=0 :", beta_l)
    print("analytic f_n [Hz]                  :", analytic)

    for n in (9, 12, 13):
        k, m = cantilever_beam_system(n, length, e, iy, area, density)
        free = list(range(3, 3 * (n + 1)))
        vals = sla.eigh(k[np.ix_(free, free)], m[np.ix_(free, free)], eigvals_only=True)
        freqs = np.sqrt(np.abs(vals)) / (2 * np.pi)
        bending = freqs[:3]
        err = np.abs(bending - analytic) / analytic
        print(
            f"n={n:2d} elements ({len(free)} free dofs) fe={np.array2string(bending, precision=6)}"
            f" rel-err={np.array2string(err * 100, precision=4)} %"
        )


# ------------------------------------------------------------------ 2 buckling
def section_buckling():
    banner("2 · Euler column buckling (BeamEb2 + geometric stiffness)")
    e, iy, area, density, length = 200e9, 8e-6, 0.005, 7850.0, 3.0
    n = 7
    dl = length / n
    ndof = 3 * (n + 1)
    exact_fixed_pinned = 4.493409457909064  # tan(x)=x root -> P = x^2 EI/L^2
    cases = {
        "pinned-pinned (K=1.0)": (
            [0, 1, 3 * n + 1],
            1.0,
        ),
        "fixed-fixed (K=0.5)": (
            [0, 1, 2, 3 * n + 1, 3 * n + 2],
            0.5,
        ),
        "fixed-pinned (K=0.7)": (
            [0, 1, 2, 3 * n + 1],
            0.7,
        ),
        "fixed-free (K=2.0)": (
            [0, 1, 2],
            2.0,
        ),
    }
    for name, (fixed, kfac) in cases.items():
        k = np.zeros((ndof, ndof))
        kl = frame_k_local(e * area / dl, e * iy, dl)
        for i in range(n):
            idx = list(range(3 * i, 3 * i + 6))
            k[np.ix_(idx, idx)] += kl
        f = np.zeros(ndof)
        f[3 * n] = -1.0
        free = [i for i in range(ndof) if i not in set(fixed)]
        u = np.zeros(ndof)
        u[free] = np.linalg.solve(k[np.ix_(free, free)], f[free])
        kg = np.zeros((ndof, ndof))
        for i in range(n):
            idx = list(range(3 * i, 3 * i + 6))
            f_end = kl @ u[idx]
            axial = -f_end[0]
            kg[np.ix_(idx, idx)] += frame_kg_local(axial, dl)
        shifted = np.linalg.solve(k[np.ix_(free, free)], -kg[np.ix_(free, free)])
        mu = sla.eigvals(shifted)
        mu = mu[np.abs(mu.imag) < 1e-9 * np.abs(mu).max()].real
        lam = 1.0 / mu[mu > 1e-12 * np.abs(mu).max()]
        fe = float(np.min(lam[lam > 0]))
        euler = np.pi**2 * e * iy / (kfac * length) ** 2
        line = (
            f"{name:<24} fe_Pcr={fe:14.6f} N   pi^2EI/(KL)^2={euler:14.6f} N"
            f"   rel={100 * abs(fe - euler) / euler:7.4f} %"
        )
        if "0.7" in name:
            exact = exact_fixed_pinned**2 * e * iy / length**2
            line += f"   exact(4.4934^2 EI/L^2)={exact:.6f} rel={100 * abs(fe - exact) / exact:.4f} %"
        print(line)


# ---------------------------------------------------- plane-stress numpy kernel
def d_plane_stress(e, nu):
    return e / (1 - nu**2) * np.array([[1, nu, 0], [nu, 1, 0], [0, 0, (1 - nu) / 2]])


def shape_quad4(xi, eta):
    n = 0.25 * np.array(
        [(1 - xi) * (1 - eta), (1 + xi) * (1 - eta), (1 + xi) * (1 + eta), (1 - xi) * (1 + eta)]
    )
    dn = 0.25 * np.array(
        [
            [-(1 - eta), -(1 - xi)],
            [(1 - eta), -(1 + xi)],
            [(1 + eta), (1 + xi)],
            [-(1 + eta), (1 - xi)],
        ]
    )
    return n, dn


def shape_quad8(xi, eta):
    n = np.array(
        [
            -0.25 * (1 - xi) * (1 - eta) * (1 + xi + eta),
            0.25 * (1 + xi) * (1 - eta) * (xi - eta - 1),
            0.25 * (1 + xi) * (1 + eta) * (xi + eta - 1),
            0.25 * (1 - xi) * (1 + eta) * (eta - xi - 1),
            0.5 * (1 - xi**2) * (1 - eta),
            0.5 * (1 + xi) * (1 - eta**2),
            0.5 * (1 - xi**2) * (1 + eta),
            0.5 * (1 - xi) * (1 - eta**2),
        ]
    )
    dn = np.array(
        [
            [0.25 * (1 - eta) * (2 * xi + eta), 0.25 * (1 - xi) * (xi + 2 * eta)],
            [0.25 * (1 - eta) * (2 * xi - eta), -0.25 * (1 + xi) * (xi - 2 * eta)],
            [0.25 * (1 + eta) * (2 * xi + eta), 0.25 * (1 + xi) * (xi + 2 * eta)],
            [0.25 * (1 + eta) * (2 * xi - eta), 0.25 * (1 - xi) * (2 * eta - xi)],
            [-xi * (1 - eta), -0.5 * (1 - xi**2)],
            [0.5 * (1 - eta**2), -eta * (1 + xi)],
            [-xi * (1 + eta), 0.5 * (1 - xi**2)],
            [-0.5 * (1 - eta**2), -eta * (1 - xi)],
        ]
    )
    return n, dn


def gauss_1d(order):
    if order == 2:
        g = 1 / np.sqrt(3)
        return [(-g, 1.0), (g, 1.0)]
    g = np.sqrt(3 / 5)
    return [(-g, 5 / 9), (0.0, 8 / 9), (g, 5 / 9)]


def quad_stiffness(coords, e, nu, thickness, shape, order):
    ndof = 2 * len(coords)
    ke = np.zeros((ndof, ndof))
    d = d_plane_stress(e, nu)
    rule = gauss_1d(order)
    for xi, wx in rule:
        for eta, wy in rule:
            _, dn = shape(xi, eta)
            jac = dn.T @ coords
            det = np.linalg.det(jac)
            dn_xy = np.linalg.solve(jac, dn.T).T
            b = np.zeros((3, ndof))
            for i, g in enumerate(dn_xy):
                b[0, 2 * i], b[1, 2 * i + 1] = g[0], g[1]
                b[2, 2 * i], b[2, 2 * i + 1] = g[1], g[0]
            ke += b.T @ d @ b * det * thickness * wx * wy
    return ke


def assemble_plane(points, cells, e, nu, thickness, shape, order):
    ndof = 2 * len(points)
    k = np.zeros((ndof, ndof))
    for cell in cells:
        coords = points[cell]
        ke = quad_stiffness(coords, e, nu, thickness, shape, order)
        idx = np.array([[2 * c, 2 * c + 1] for c in cell]).ravel()
        k[np.ix_(idx, idx)] += ke
    return k


def solve_constrained(k, f, fixed):
    free = np.setdiff1d(np.arange(len(f)), np.asarray(sorted(fixed), int))
    u = np.zeros(len(f))
    u[free] = np.linalg.solve(k[np.ix_(free, free)], f[free])
    return u


# ------------------------------------------------------------- 3 Cook membrane
def cook_mesh(n, quadratic=False):
    p00, p10, p11, p01 = (0.0, 0.0), (48.0, 44.0), (48.0, 60.0), (0.0, 44.0)

    def blend(r, s):
        x = (1 - r) * (1 - s) * p00[0] + r * (1 - s) * p10[0] + r * s * p11[0] + (1 - r) * s * p01[0]
        y = (1 - r) * (1 - s) * p00[1] + r * (1 - s) * p10[1] + r * s * p11[1] + (1 - r) * s * p01[1]
        return x, y

    steps = 2 * n if quadratic else n
    index = {}
    points = []
    for i in range(steps + 1):
        for j in range(steps + 1):
            if quadratic and i % 2 == 1 and j % 2 == 1:
                continue
            index[(i, j)] = len(points)
            points.append(blend(i / steps, j / steps))
    points = np.array(points)
    cells = []
    if quadratic:
        for i in range(n):
            for j in range(n):
                a, b = 2 * i, 2 * j
                cells.append(
                    [
                        index[(a, b)],
                        index[(a + 2, b)],
                        index[(a + 2, b + 2)],
                        index[(a, b + 2)],
                        index[(a + 1, b)],
                        index[(a + 2, b + 1)],
                        index[(a + 1, b + 2)],
                        index[(a, b + 1)],
                    ]
                )
    else:
        for i in range(n):
            for j in range(n):
                cells.append(
                    [index[(i, j)], index[(i + 1, j)], index[(i + 1, j + 1)], index[(i, j + 1)]]
                )
    return points, np.array(cells), index, steps


def cook_reference(n, quadratic):
    points, cells, index, steps = cook_mesh(n, quadratic)
    shape, order = (shape_quad8, 3) if quadratic else (shape_quad4, 2)
    k = assemble_plane(points, cells, 1.0, 1.0 / 3.0, 1.0, shape, order)
    fixed = [d for j in range(steps + 1) if (0, j) in index for d in (2 * index[(0, j)], 2 * index[(0, j)] + 1)]
    f = np.zeros(2 * len(points))
    h = 16.0 / n
    traction = 1.0 / 16.0
    for j in range(n):
        if quadratic:
            ends = [index[(steps, 2 * j)], index[(steps, 2 * j + 2)]]
            mid = index[(steps, 2 * j + 1)]
            for node in ends:
                f[2 * node + 1] += traction * h / 6.0
            f[2 * mid + 1] += traction * h * 2.0 / 3.0
        else:
            for node in (index[(steps, j)], index[(steps, j + 1)]):
                f[2 * node + 1] += traction * h / 2.0
    u = solve_constrained(k, f, fixed)
    tip = index[(steps, steps // 2)]
    return points, cells, u[2 * tip + 1], float(f.sum())


def cook_skfem(n, quadratic):
    from skfem import Basis, ElementQuad1, ElementQuadS2, ElementVector, MeshQuad
    from skfem.models.elasticity import linear_elasticity

    points, cells, index, steps = cook_mesh(n, quadratic)
    corner_ids = sorted({c for cell in cells for c in cell[:4]})
    remap = {old: new for new, old in enumerate(corner_ids)}
    mesh = MeshQuad(points[corner_ids].T, np.array([[remap[c] for c in cell[:4]] for cell in cells]).T)
    element = ElementVector(ElementQuadS2() if quadratic else ElementQuad1())
    basis = Basis(mesh, element, intorder=4 if quadratic else 2)
    lam = 1.0 * (1.0 / 3.0) / (1.0 - (1.0 / 3.0) ** 2)
    mu = 1.0 / (2.0 * (1.0 + 1.0 / 3.0))
    k = linear_elasticity(lam, mu).assemble(basis)

    at_location = {}
    for dof in range(basis.N):
        key = (round(float(basis.doflocs[0, dof]), 9), round(float(basis.doflocs[1, dof]), 9))
        at_location.setdefault(key, []).append(dof)

    def dofs_at(point):
        return sorted(at_location[(round(float(point[0]), 9), round(float(point[1]), 9))])

    fixed = []
    for j in range(steps + 1):
        if (0, j) in index:
            fixed.extend(dofs_at(points[index[(0, j)]]))
    f = np.zeros(k.shape[0])
    h = 16.0 / n
    traction = 1.0 / 16.0
    if quadratic:
        for j in range(n):
            for p in (points[index[(steps, 2 * j)]], points[index[(steps, 2 * j + 2)]]):
                f[dofs_at(p)[1]] += traction * h / 6.0
            f[dofs_at(points[index[(steps, 2 * j + 1)]])[1]] += traction * h * 2.0 / 3.0
    else:
        for j in range(n):
            for p in (points[index[(steps, j)]], points[index[(steps, j + 1)]]):
                f[dofs_at(p)[1]] += traction * h / 2.0
    free = np.setdiff1d(np.arange(k.shape[0]), np.array(sorted(set(fixed)), int))
    kd = k.toarray()
    u = np.zeros(k.shape[0])
    u[free] = np.linalg.solve(kd[np.ix_(free, free)], f[free])
    return u[dofs_at(points[index[(steps, steps // 2)]])[1]]


def section_cook():
    banner("3 · Cook's membrane (E=1, nu=1/3, t=1, total shear F=1, tip point (48,52))")
    print("published converged reference tip deflection: 23.96")
    for n in (2, 4, 8, 16, 32):
        _, _, tip, total = cook_reference(n, False)
        sk = cook_skfem(n, False)
        print(
            f"Quad4 {n:2d}x{n:<2d} numpy={tip:.12f}  skfem={sk:.12f}  diff={abs(tip - sk):.3e}"
            f"  load_sum={total:.6f}  ratio_to_23.96={tip / 23.96:.4f}"
        )
    for n in (2, 4, 8):
        _, _, tip, total = cook_reference(n, True)
        sk = cook_skfem(n, True)
        print(
            f"Quad8 {n:2d}x{n:<2d} numpy={tip:.12f}  skfem={sk:.12f}  diff={abs(tip - sk):.3e}"
            f"  load_sum={total:.6f}  ratio_to_23.96={tip / 23.96:.4f}"
        )


# ------------------------------------------------------------------- 4 plate
def navier_ss_plate(a, b, q, d, terms=201):
    total = 0.0
    for m in range(1, terms, 2):
        for n in range(1, terms, 2):
            total += (
                np.sin(m * np.pi / 2)
                * np.sin(n * np.pi / 2)
                / (m * n * ((m / a) ** 2 + (n / b) ** 2) ** 2)
            )
    return 16 * q / (np.pi**6 * d) * total


def dkt_edge(pi, pj):
    x, y = pi[0] - pj[0], pi[1] - pj[1]
    l2 = x * x + y * y
    return (
        -x / l2,
        0.75 * x * y / l2,
        (0.25 * x * x - 0.5 * y * y) / l2,
        -y / l2,
        (0.25 * y * y - 0.5 * x * x) / l2,
    )


def tri6_grads(coords, xi, eta):
    """📐️ Physical gradients of the six quadratic area-coordinate shape functions."""
    l1, l2, l3 = 1 - xi - eta, xi, eta
    dparam = np.array(
        [
            [-(4 * l1 - 1), -(4 * l1 - 1)],
            [4 * l2 - 1, 0.0],
            [0.0, 4 * l3 - 1],
            [4 * l3, 4 * l2],
            [-4 * l3, 4 * (l1 - l3)],
            [4 * (l1 - l2), -4 * l2],
        ]
    )
    corners = np.array([[-1.0, -1.0], [1.0, 0.0], [0.0, 1.0]])
    jac = corners.T @ np.asarray(coords)
    return np.linalg.solve(jac, dparam.T).T, float(np.linalg.det(jac))


def dkt_b_matrix(coords, xi, eta):
    """🧮️ Batoz-Bathe-Ho DKT curvature matrix, dof order [w1,bx1,by1,w2,bx2,by2,w3,bx3,by3]."""
    a4, b4, c4, d4, e4 = dkt_edge(coords[1], coords[2])
    a5, b5, c5, d5, e5 = dkt_edge(coords[2], coords[0])
    a6, b6, c6, d6, e6 = dkt_edge(coords[0], coords[1])
    grads, det = tri6_grads(coords, xi, eta)
    g1, g2, g3, g4, g5, g6 = grads

    def hx(d1, d2, d3, d4v, d5v, d6v):
        return np.array(
            [
                1.5 * (a6 * d6v - a5 * d5v),
                b5 * d5v + b6 * d6v,
                d1 - c5 * d5v - c6 * d6v,
                1.5 * (a4 * d4v - a6 * d6v),
                b6 * d6v + b4 * d4v,
                d2 - c6 * d6v - c4 * d4v,
                1.5 * (a5 * d5v - a4 * d4v),
                b4 * d4v + b5 * d5v,
                d3 - c4 * d4v - c5 * d5v,
            ]
        )

    def hy(d1, d2, d3, d4v, d5v, d6v):
        return np.array(
            [
                1.5 * (d6 * d6v - d5 * d5v),
                -d1 + e5 * d5v + e6 * d6v,
                -b5 * d5v - b6 * d6v,
                1.5 * (d4 * d4v - d6 * d6v),
                -d2 + e4 * d4v + e6 * d6v,
                -b4 * d4v - b6 * d6v,
                1.5 * (d5 * d5v - d4 * d4v),
                -d3 + e4 * d4v + e5 * d5v,
                -b4 * d4v - b5 * d5v,
            ]
        )

    dhx_dx = hx(g1[0], g2[0], g3[0], g4[0], g5[0], g6[0])
    dhx_dy = hx(g1[1], g2[1], g3[1], g4[1], g5[1], g6[1])
    dhy_dx = hy(g1[0], g2[0], g3[0], g4[0], g5[0], g6[0])
    dhy_dy = hy(g1[1], g2[1], g3[1], g4[1], g5[1], g6[1])
    return np.vstack([dhx_dx, dhy_dy, dhx_dy + dhy_dx]), det


def dkt_stiffness(coords, e, nu, thickness):
    factor = e * thickness**3 / (12 * (1 - nu**2))
    d = factor * np.array([[1, nu, 0], [nu, 1, 0], [0, 0, (1 - nu) / 2]])
    ke = np.zeros((9, 9))
    for xi, eta in ((1 / 6, 1 / 6), (2 / 3, 1 / 6), (1 / 6, 2 / 3)):
        b, det = dkt_b_matrix(coords, xi, eta)
        ke += b.T @ d @ b * (det / 6.0)
    return ke


def dkt_ss_plate(n, a, q, e, nu, t):
    dx = a / n
    index = {}
    points = []
    for i in range(n + 1):
        for j in range(n + 1):
            index[(i, j)] = len(points)
            points.append([i * dx, j * dx])
    points = np.array(points)
    cells = []
    for i in range(n):
        for j in range(n):
            cells.append([index[(i, j)], index[(i + 1, j)], index[(i + 1, j + 1)]])
            cells.append([index[(i, j)], index[(i + 1, j + 1)], index[(i, j + 1)]])
    ndof = 3 * len(points)
    k = np.zeros((ndof, ndof))
    f = np.zeros(ndof)
    for cell in cells:
        ke = dkt_stiffness(points[cell], e, nu, t)
        idx = np.array([[3 * c, 3 * c + 1, 3 * c + 2] for c in cell]).ravel()
        k[np.ix_(idx, idx)] += ke
        share = q * (0.5 * dx * dx) / 3.0
        for c in cell:
            f[3 * c] -= share
    fixed = [3 * index[(i, j)] for i in range(n + 1) for j in range(n + 1) if i in (0, n) or j in (0, n)]
    u = solve_constrained(k, f, fixed)
    return -u[3 * index[(n // 2, n // 2)]], ndof


def section_plate():
    banner("4 · simply supported square plate under UDL")
    e, nu, t, a, q = 2e11, 0.3, 0.01, 2.0, 1000.0
    d = e * t**3 / (12 * (1 - nu**2))
    w_navier = navier_ss_plate(a, a, q, d)
    print(f"D = {d:.6f}")
    print(f"Navier series w_max            = {w_navier:.9e} m")
    print(f"alpha = w_max D /(q a^4)       = {w_navier * d / (q * a**4):.8f}  (tabulated 0.00406)")
    print(f"closed form 0.00406 q a^4 / D  = {0.00406 * q * a**4 / d:.9e} m")

    reference = 0.00406 * q * a**4 / d
    for n in (2, 4, 6, 8, 10, 12):
        w_fe, ndof = dkt_ss_plate(n, a, q, e, nu, t)
        print(
            f"DKT {n:2d}x{n:<2d} ({2 * n * n:3d} triangles, {ndof:4d} dofs) w_centre={w_fe:.12e}"
            f"  rel-to-0.00406={100 * abs(w_fe - reference) / reference:6.3f} %"
            f"  rel-to-Navier={100 * abs(w_fe - w_navier) / w_navier:6.3f} %"
        )

    try:
        from skfem import Basis, BilinearForm, ElementTriMorley, LinearForm, MeshTri, asm, condense, solve
        from skfem.helpers import ddot, dd
    except Exception as exc:  # pragma: no cover
        print("skfem Morley cross-check unavailable:", exc)
        return

    @BilinearForm
    def bending(u, v, w):
        ddu, ddv = dd(u), dd(v)
        trace_u = ddu[0, 0] + ddu[1, 1]
        trace_v = ddv[0, 0] + ddv[1, 1]
        return d * ((1 - nu) * ddot(ddu, ddv) + nu * trace_u * trace_v)

    @LinearForm
    def load(v, w):
        return q * v

    for refine in (3, 4, 5, 6):
        mesh = MeshTri().refined(refine).scaled([a, a])
        basis = Basis(mesh, ElementTriMorley())
        k = asm(bending, basis)
        f = asm(load, basis)
        boundary_vertices = mesh.boundary_nodes()
        u = solve(*condense(k, f, D=boundary_vertices))
        centre = np.argmin(np.linalg.norm(basis.doflocs.T - np.array([a / 2, a / 2]), axis=1))
        w_fe = u[centre]
        print(
            f"Morley refine={refine} ({basis.N:5d} dofs) w_centre={w_fe:.9e}"
            f"  rel-to-Navier={100 * abs(w_fe - w_navier) / w_navier:6.3f} %"
        )


# --------------------------------------------------------------------- 5 hex8
HEX_CORNERS = np.array(
    [
        [-1, -1, -1],
        [1, -1, -1],
        [1, 1, -1],
        [-1, 1, -1],
        [-1, -1, 1],
        [1, -1, 1],
        [1, 1, 1],
        [-1, 1, 1],
    ],
    float,
)


def d_solid(e, nu):
    factor = e / ((1 + nu) * (1 - 2 * nu))
    d = np.zeros((6, 6))
    for i in range(3):
        for j in range(3):
            d[i, j] = factor * (nu if i != j else 1 - nu)
    for i in range(3, 6):
        d[i, i] = factor * (1 - 2 * nu) / 2
    return d


def hex8_stiffness(coords, e, nu):
    ke = np.zeros((24, 24))
    d = d_solid(e, nu)
    g = 1 / np.sqrt(3)
    for xi in (-g, g):
        for eta in (-g, g):
            for zeta in (-g, g):
                dn = np.column_stack(
                    [
                        0.125 * HEX_CORNERS[:, 0] * (1 + eta * HEX_CORNERS[:, 1]) * (1 + zeta * HEX_CORNERS[:, 2]),
                        0.125 * HEX_CORNERS[:, 1] * (1 + xi * HEX_CORNERS[:, 0]) * (1 + zeta * HEX_CORNERS[:, 2]),
                        0.125 * HEX_CORNERS[:, 2] * (1 + xi * HEX_CORNERS[:, 0]) * (1 + eta * HEX_CORNERS[:, 1]),
                    ]
                )
                jac = dn.T @ coords
                det = np.linalg.det(jac)
                grads = np.linalg.solve(jac, dn.T).T
                b = np.zeros((6, 24))
                for i, gr in enumerate(grads):
                    b[0, 3 * i] = gr[0]
                    b[1, 3 * i + 1] = gr[1]
                    b[2, 3 * i + 2] = gr[2]
                    b[3, 3 * i], b[3, 3 * i + 1] = gr[1], gr[0]
                    b[4, 3 * i + 1], b[4, 3 * i + 2] = gr[2], gr[1]
                    b[5, 3 * i], b[5, 3 * i + 2] = gr[2], gr[0]
                ke += b.T @ d @ b * det
    return ke


def hex_cantilever(nx, ny, nz, length, width, height, e, nu, p_total):
    xs = np.linspace(0, length, nx + 1)
    ys = np.linspace(0, width, ny + 1)
    zs = np.linspace(0, height, nz + 1)
    index = {}
    points = []
    for i, x in enumerate(xs):
        for j, y in enumerate(ys):
            for kk, z in enumerate(zs):
                index[(i, j, kk)] = len(points)
                points.append([x, y, z])
    points = np.array(points)
    cells = []
    for i in range(nx):
        for j in range(ny):
            for kk in range(nz):
                cells.append(
                    [
                        index[(i, j, kk)],
                        index[(i + 1, j, kk)],
                        index[(i + 1, j + 1, kk)],
                        index[(i, j + 1, kk)],
                        index[(i, j, kk + 1)],
                        index[(i + 1, j, kk + 1)],
                        index[(i + 1, j + 1, kk + 1)],
                        index[(i, j + 1, kk + 1)],
                    ]
                )
    ndof = 3 * len(points)
    k = np.zeros((ndof, ndof))
    for cell in cells:
        ke = hex8_stiffness(points[cell], e, nu)
        idx = np.array([[3 * c, 3 * c + 1, 3 * c + 2] for c in cell]).ravel()
        k[np.ix_(idx, idx)] += ke
    fixed = []
    for j in range(ny + 1):
        for kk in range(nz + 1):
            node = index[(0, j, kk)]
            fixed.extend([3 * node, 3 * node + 1, 3 * node + 2])
    tip_nodes = [index[(nx, j, kk)] for j in range(ny + 1) for kk in range(nz + 1)]
    f = np.zeros(ndof)
    for node in tip_nodes:
        f[3 * node + 2] -= p_total / len(tip_nodes)
    u = solve_constrained(k, f, fixed)
    tip = np.mean([u[3 * node + 2] for node in tip_nodes])
    return points, cells, tip, len(points)


def section_hex():
    banner("5 · Hex8 meshed cantilever vs Euler-Bernoulli + shear correction")
    e, nu = 200e9, 0.3
    width, height, length, p = 1.0, 2.0, 4.0, 1e4
    g = e / (2 * (1 + nu))
    inertia = width * height**3 / 12
    kappa = 5.0 / 6.0
    d_bend = p * length**3 / (3 * e * inertia)
    d_shear = p * length / (kappa * g * width * height)
    print(f"geometry L={length} b={width} h={height} E={e} nu={nu} P={p}")
    print(f"PL^3/3EI          = {d_bend:.9e}")
    print(f"PL/(kappa G A)    = {d_shear:.9e}  ({100 * d_shear / d_bend:.2f} % of bending)")
    print(f"total closed form = {d_bend + d_shear:.9e}")
    for nx, ny, nz in ((4, 1, 1), (8, 2, 2), (8, 2, 3), (12, 2, 5), (16, 2, 6)):
        _, _, tip, nodes = hex_cantilever(nx, ny, nz, length, width, height, e, nu, p)
        print(
            f"hex {nx:2d}x{ny}x{nz} ({nodes:4d} nodes, {3 * nodes:5d} dofs) tip_dz={tip:.12e}"
            f"  |tip|/closed={abs(tip) / (d_bend + d_shear):.5f}"
        )
    section_hex_skfem(e, nu, width, height, length, p, d_bend + d_shear)


def section_hex_skfem(e, nu, width, height, length, p, closed):
    try:
        from skfem import Basis, ElementHex1, ElementVector, MeshHex
        from skfem.models.elasticity import linear_elasticity, lame_parameters
    except Exception as exc:  # pragma: no cover
        print("skfem hex cross-check unavailable:", exc)
        return
    for nx, ny, nz in ((8, 2, 2), (12, 2, 5), (16, 2, 6)):
        xs = np.linspace(0, length, nx + 1)
        ys = np.linspace(0, width, ny + 1)
        zs = np.linspace(0, height, nz + 1)
        mesh = MeshHex.init_tensor(xs, ys, zs)
        basis = Basis(mesh, ElementVector(ElementHex1()), intorder=2)
        lam, mu = lame_parameters(e, nu)
        k = linear_elasticity(lam, mu).assemble(basis).toarray()
        verts = mesh.p.T
        nodal = basis.nodal_dofs
        left = np.where(np.isclose(verts[:, 0], 0.0))[0]
        right = np.where(np.isclose(verts[:, 0], length))[0]
        fixed = nodal[:, left].ravel().tolist()
        f = np.zeros(k.shape[0])
        for node in right:
            f[nodal[2, node]] -= p / len(right)
        free = np.setdiff1d(np.arange(k.shape[0]), np.array(sorted(set(fixed)), int))
        u = np.zeros(k.shape[0])
        u[free] = np.linalg.solve(k[np.ix_(free, free)], f[free])
        tip = float(np.mean([u[nodal[2, node]] for node in right]))
        print(
            f"skfem ElementHex1 {nx}x{ny}x{nz} tip_dz={tip:.12e}  |tip|/closed={abs(tip) / closed:.5f}"
        )


# -------------------------------------------------------------------- 6 beams
def section_beams():
    banner("6 · closed-form beam / frame checks")
    e, iy, area, length, w = 200e9, 1e-5, 0.01, 6.0, 2000.0
    print("--- simply supported beam under UDL (4 BeamEb2 elements) ---")
    frame = Frame2d([[i * length / 4, 0.0] for i in range(5)])
    for i in range(4):
        m = frame.add_member(i, i + 1, e, area, iy)
        frame.add_udl(m, 0.0, -w)
    u, r = frame.solve([0, 1, 13])
    mid = u[3 * 2 + 1]
    closed = -5 * w * length**4 / (384 * e * iy)
    print(f"midspan v = {mid:.15e}   5wL^4/384EI = {closed:.15e}  rel={abs(mid - closed) / abs(closed):.3e}")
    print(f"end rotations = {u[2]:.12e} / {u[14]:.12e}   wL^3/24EI = {w * length**3 / (24 * e * iy):.12e}")
    print(f"support reactions Ty = {r[1]:.9f} / {r[13]:.9f}   wL/2 = {w * length / 2:.9f}")

    print("--- propped cantilever under UDL (4 BeamEb2 elements) ---")
    frame = Frame2d([[i * length / 4, 0.0] for i in range(5)])
    for i in range(4):
        m = frame.add_member(i, i + 1, e, area, iy)
        frame.add_udl(m, 0.0, -w)
    u, r = frame.solve([0, 1, 2, 13])
    print(f"fixed-end reaction  Ty = {r[1]:.9f}   5wL/8 = {5 * w * length / 8:.9f}")
    print(f"fixed-end moment    Rz = {r[2]:.9f}   -wL^2/8 = {-w * length**2 / 8:.9f}")
    print(f"prop reaction       Ty = {r[13]:.9f}   3wL/8 = {3 * w * length / 8:.9f}")
    print(f"max sag at x=0.4375L, w=wL^4/185EI = {w * length**4 / (185 * e * iy):.9e}")

    print("--- 2D portal frame, lateral sway (stiffness method, scipy) ---")
    h, span = 4.0, 6.0
    ec, ac, ic = 210e9, 0.008, 8.0e-5
    eb, ab, ib = 210e9, 0.012, 2.0e-4
    hload = 15000.0
    frame = Frame2d([[0.0, 0.0], [0.0, h], [span, h], [span, 0.0]])
    frame.add_member(0, 1, ec, ac, ic)
    frame.add_member(1, 2, eb, ab, ib)
    frame.add_member(3, 2, ec, ac, ic)
    frame.add_load(1, 0, hload)
    u, r = frame.solve([0, 1, 2, 9, 10, 11])
    print(f"sway ux(node1) = {u[3]:.12e}   ux(node2) = {u[6]:.12e}")
    print(f"base reactions Fx = {r[0]:.9f} / {r[9]:.9f}   sum = {r[0] + r[9]:.9f}  (must equal -{hload})")
    print(f"base reactions Fy = {r[1]:.9f} / {r[10]:.9f}   sum = {r[1] + r[10]:.3e}")
    print(f"base moments   Mz = {r[2]:.9f} / {r[11]:.9f}")
    moment_about_origin = r[2] + r[11] + r[10] * span - hload * h
    print(f"global Mz equilibrium residual about (0,0) = {moment_about_origin:.6e}")
    print(f"rotations rz(node1) = {u[5]:.12e}   rz(node2) = {u[8]:.12e}")


# ----------------------------------------------------------------- 7 MacNeal
def macneal_mesh(kind, n=6, length=6.0, height=0.2):
    xs = np.linspace(0, length, n + 1)
    points = []
    index = {}
    for i, x in enumerate(xs):
        for j in range(2):
            if kind == "rectangular":
                px, py = x, j * height
            elif kind == "parallelogram":
                skew = height / np.tan(np.deg2rad(45.0))
                px, py = x + j * skew, j * height
            else:
                offset = (height / 2) * (1 if i % 2 == 0 else -1)
                px = x + (offset if j == 1 else -offset)
                px = x if i in (0, n) else px
                py = j * height
            index[(i, j)] = len(points)
            points.append([px, py])
    points = np.array(points)
    cells = np.array(
        [[index[(i, 0)], index[(i + 1, 0)], index[(i + 1, 1)], index[(i, 1)]] for i in range(n)]
    )
    return points, cells, index, n


def macneal_case(kind):
    e, nu, thickness, length, height = 1e7, 0.3, 0.1, 6.0, 0.2
    points, cells, index, n = macneal_mesh(kind, length=length, height=height)
    k = assemble_plane(points, cells, e, nu, thickness, shape_quad4, 2)
    fixed = [d for j in (0, 1) for d in (2 * index[(0, j)], 2 * index[(0, j)] + 1)]
    f = np.zeros(2 * len(points))
    for j in (0, 1):
        f[2 * index[(n, j)] + 1] += 0.5
    theory = 0.1081
    u = solve_constrained(k, f, fixed)
    tip = 0.5 * (u[2 * index[(n, 0)] + 1] + u[2 * index[(n, 1)] + 1])
    return tip, theory, points, cells, index, n


def macneal_skfem(kind):
    from skfem import Basis, ElementQuad1, ElementVector, MeshQuad
    from skfem.models.elasticity import linear_elasticity

    e, nu, thickness, length, height = 1e7, 0.3, 0.1, 6.0, 0.2
    points, cells, index, n = macneal_mesh(kind, length=length, height=height)
    mesh = MeshQuad(points.T, cells.T)
    basis = Basis(mesh, ElementVector(ElementQuad1()), intorder=2)
    lam = e * nu / (1 - nu**2)
    mu = e / (2 * (1 + nu))
    k = (linear_elasticity(lam, mu).assemble(basis) * thickness).toarray()
    verts = mesh.p.T
    lookup = {(round(x, 9), round(y, 9)): i for i, (x, y) in enumerate(verts)}

    def vid(node):
        return lookup[(round(points[node][0], 9), round(points[node][1], 9))]

    nodal = basis.nodal_dofs
    fixed = [int(nodal[c, vid(index[(0, j)])]) for j in (0, 1) for c in (0, 1)]
    f = np.zeros(k.shape[0])
    for j in (0, 1):
        f[int(nodal[1, vid(index[(n, j)])])] += 0.5
    free = np.setdiff1d(np.arange(k.shape[0]), np.array(sorted(set(fixed)), int))
    u = np.zeros(k.shape[0])
    u[free] = np.linalg.solve(k[np.ix_(free, free)], f[free])
    return 0.5 * sum(u[int(nodal[1, vid(index[(n, j)])])] for j in (0, 1))


def section_macneal():
    banner("7 · MacNeal-Harder straight cantilever, Quad4 distortion sensitivity")
    print("E=1e7 nu=0.3 t=0.1 L=6 h=0.2, 6 elements, unit tip shear; beam theory tip v = 0.1081")
    print("(normalized values for a PLAIN fully-integrated bilinear quad; MacNeal-Harder's own 0.904/")
    print(" 0.071/0.080 table row is for QUAD4 WITH incompatible modes, which this kernel does not have)")
    for kind in ("rectangular", "parallelogram", "trapezoidal"):
        tip, theory, *_ = macneal_case(kind)
        try:
            sk = macneal_skfem(kind)
        except Exception as exc:  # pragma: no cover
            sk = float("nan")
            print("  skfem:", exc)
        print(
            f"{kind:<15} numpy tip={tip:.12f}  skfem tip={sk:.12f}  diff={abs(tip - sk):.3e}"
            f"  normalized={tip / theory:.5f}"
        )


SECTIONS = {
    "modal": section_modal,
    "buckling": section_buckling,
    "cook": section_cook,
    "plate": section_plate,
    "hex": section_hex,
    "beams": section_beams,
    "macneal": section_macneal,
}


if __name__ == "__main__":
    np.set_printoptions(precision=9, suppress=False)
    wanted = sys.argv[1:] or list(SECTIONS)
    for name in wanted:
        SECTIONS[name]()
