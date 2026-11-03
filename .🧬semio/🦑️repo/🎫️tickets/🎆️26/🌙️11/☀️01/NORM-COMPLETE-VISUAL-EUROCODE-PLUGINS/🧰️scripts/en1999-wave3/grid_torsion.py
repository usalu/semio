"""Prototype of the EN 1999 free-outline section solver (Prandtl torsion constant and warping constant on a Q4 raster); mirror of the Rust crate semio-s-artifact-norm-en1999-section-solver."""
import math
import numpy as np


def shoelace(ring):
    n = len(ring)
    return sum(ring[i][0] * ring[(i + 1) % n][1] - ring[(i + 1) % n][0] * ring[i][1] for i in range(n)) / 2.0


def inside(ring, px, py):
    """even-odd test of many points"""
    ins = np.zeros(px.shape, dtype=bool)
    n = len(ring)
    for i in range(n):
        ax, ay = ring[i]
        bx, by = ring[(i + 1) % n]
        if ay == by:
            continue
        cond = (ay > py) != (by > py)
        xin = (bx - ax) * (py - ay) / (by - ay) + ax
        ins ^= cond & (px < xin)
    return ins


def solve(outer, holes, cells_min=20000, cells_max=120000, per_wall=5.0):
    rings = [outer] + list(holes)
    xs = [p[0] for p in outer]; ys = [p[1] for p in outer]
    x0, y0, w, hh = min(xs), min(ys), max(xs) - min(xs), max(ys) - min(ys)
    area = abs(shoelace(outer)) - sum(abs(shoelace(r)) for r in holes)
    per = sum(sum(math.dist(r[i], r[(i + 1) % len(r)]) for i in range(len(r))) for r in rings)
    t_est = 2.0 * area / per
    n_wall = max(1, int(per_wall))
    h = t_est / n_wall
    while w * hh / (h * h) > cells_max:
        n_wall -= 1
        h = t_est / max(n_wall, 1) if n_wall >= 1 else math.sqrt(w * hh / cells_max)
        if n_wall < 1:
            break
    while w * hh / (h * h) < cells_min:
        h = h / 2.0
    nx, ny = int(math.ceil(w / h - 1e-9)), int(math.ceil(hh / h - 1e-9))
    cx = x0 + (np.arange(nx) + 0.5) * h
    cy = y0 + (np.arange(ny) + 0.5) * h
    X, Y = np.meshgrid(cx, cy, indexing="ij")
    inc = inside(outer, X, Y)
    for r in holes:
        inc &= ~inside(r, X, Y)
    # padded excluded labelling
    pad = np.ones((nx + 2, ny + 2), dtype=bool)
    pad[1:-1, 1:-1] = ~inc
    lab = np.zeros(pad.shape, dtype=int)
    k = 0
    for i0 in range(pad.shape[0]):
        for j0 in range(pad.shape[1]):
            if pad[i0, j0] and lab[i0, j0] == 0:
                stack = [(i0, j0)]
                lab[i0, j0] = k + 1
                while stack:
                    i, j = stack.pop()
                    for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        a, b = i + di, j + dj
                        if 0 <= a < pad.shape[0] and 0 <= b < pad.shape[1] and pad[a, b] and lab[a, b] == 0:
                            lab[a, b] = k + 1
                            stack.append((a, b))
                k += 1
    # component of the exterior = label of pad[0,0]
    ext = lab[0, 0]
    comp_ids = sorted(set(lab[pad].tolist()) - {ext})
    comp_index = {c: i + 1 for i, c in enumerate(comp_ids)}  # 1..m
    lab_cell = lab[1:-1, 1:-1]
    # node arrays (nx+1, ny+1); cell (i,j) has nodes (i,j),(i+1,j),(i+1,j+1),(i,j+1)
    nn = (nx + 1, ny + 1)
    n_inc = np.zeros(nn, dtype=int)
    for di, dj in ((0, 0), (1, 0), (1, 1), (0, 1)):
        n_inc[di:di + nx, dj:dj + ny] += inc
    # node boundary class: component touching (cell-adjacent excluded cells), padded labels
    node_comp = np.full(nn, -1, dtype=int)  # -1 none, 0 free interior(only if no excluded neighbour)
    touch = np.zeros(nn, dtype=int)  # number of excluded neighbours
    comp_of = np.zeros(nn, dtype=int)
    for di, dj in ((0, 0), (1, 0), (1, 1), (0, 1)):
        # cell (i,j) -> node (i+di, j+dj); padded cell index (i+1, j+1)
        sub_lab = lab[1:-1, 1:-1]
        ex = ~inc
        l = np.where(ex, np.vectorize(lambda c: comp_index.get(c, 0))(sub_lab), 0)
        view = comp_of[di:di + nx, dj:dj + ny]
        np.maximum(view, l, out=view)  # hole component index (if any); exterior -> 0 and ext flagged separately
        t = touch[di:di + nx, dj:dj + ny]
        t += ex
    ext_touch = np.zeros(nn, dtype=bool)
    for di, dj in ((0, 0), (1, 0), (1, 1), (0, 1)):
        e = (lab_cell == ext) & ~inc
        ext_touch[di:di + nx, dj:dj + ny] |= e
    # also nodes on the grid border adjacent to the exterior beyond the grid
    ext_touch[0, :] |= n_inc[0, :] > 0; ext_touch[-1, :] |= n_inc[-1, :] > 0
    ext_touch[:, 0] |= n_inc[:, 0] > 0; ext_touch[:, -1] |= n_inc[:, -1] > 0
    active = n_inc > 0
    boundary = active & (touch > 0) | (active & ext_touch)
    cidx = np.where(ext_touch, 0, comp_of)  # exterior wins -> 0
    m = len(comp_ids)
    # unknown numbering
    free = active & ~boundary
    fid = -np.ones(nn, dtype=int)
    fid[free] = np.arange(free.sum())
    nfree = int(free.sum())
    ndof = nfree + m
    hole_area = np.zeros(m + 1)
    for c in comp_ids:
        hole_area[comp_index[c]] = float((lab_cell == c).sum()) * h * h
    Ke = np.array([[4, -1, -2, -1], [-1, 4, -1, -2], [-2, -1, 4, -1], [-1, -2, -1, 4]], dtype=float) / 6.0
    offs = ((0, 0), (1, 0), (1, 1), (0, 1))

    def prolong(x):
        v = np.zeros(nn)
        v[free] = x[:nfree]
        cvals = np.concatenate([[0.0], x[nfree:]])
        bmask = boundary & ~ext_touch
        v[bmask] = cvals[cidx[bmask]]
        return v

    def apply_K(v):
        r = np.zeros(nn)
        loc = [v[di:di + nx, dj:dj + ny] for di, dj in offs]
        for a, (di, dj) in enumerate(offs):
            acc = sum(Ke[a, b] * loc[b] for b in range(4)) * inc
            r[di:di + nx, dj:dj + ny] += acc
        return r

    def restrict(r):
        out = np.zeros(ndof)
        out[:nfree] = r[free]
        bmask = boundary & ~ext_touch
        for node_c in range(1, m + 1):
            out[nfree + node_c - 1] = r[bmask & (cidx == node_c)].sum()
        return out

    fnode = np.zeros(nn)
    for di, dj in offs:
        fnode[di:di + nx, dj:dj + ny] += inc * (h * h / 2.0)
    b = restrict(fnode)
    for c in range(1, m + 1):
        b[nfree + c - 1] += 2.0 * hole_area[c]
    A = lambda x: restrict(apply_K(prolong(x)))
    # jacobi preconditioner
    dn = np.zeros(nn)
    for di, dj in offs:
        dn[di:di + nx, dj:dj + ny] += inc * (4.0 / 6.0)
    dg = np.ones(ndof)
    dg[:nfree] = dn[free]
    bmask = boundary & ~ext_touch
    for c in range(1, m + 1):
        dg[nfree + c - 1] = dn[bmask & (cidx == c)].sum()
    x = pcg(A, b, dg)
    jtors = float(x @ b)
    # warping on the centroid frame
    cellA = h * h
    ncell = inc.sum()
    cyc = (X[inc].sum()) / ncell
    czc = (Y[inc].sum()) / ncell
    # cell second moments exact
    def cell_moments():
        Xc, Yc = X[inc] - cyc, Y[inc] - czc
        a = cellA
        iy = (a * (Yc ** 2 + h * h / 12.0)).sum()  # ∫ z² dA
        iz = (a * (Xc ** 2 + h * h / 12.0)).sum()
        iyz = (a * Xc * Yc).sum()
        return a * ncell, iy, iz, iyz
    A_d, Iy, Iz, Iyz = cell_moments()
    g = np.array([0.5 - 0.5 / math.sqrt(3), 0.5 + 0.5 / math.sqrt(3)])
    def shape(xi, eta):
        N = np.array([(1 - xi) * (1 - eta), xi * (1 - eta), xi * eta, (1 - xi) * eta])
        dNxi = np.array([-(1 - eta), (1 - eta), eta, -eta])
        dNeta = np.array([-(1 - xi), -xi, xi, (1 - xi)])
        return N, dNxi, dNeta
    fw = np.zeros(nn)
    gp = [(a, b_) for a in g for b_ in g]
    Xc0, Yc0 = np.meshgrid(x0 + np.arange(nx) * h - cyc, y0 + np.arange(ny) * h - czc, indexing="ij")
    for xi, eta in gp:
        N, dNxi, dNeta = shape(xi, eta)
        yy = Xc0 + h * xi
        zz = Yc0 + h * eta
        for a, (di, dj) in enumerate(offs):
            # f_i = ∫ (z dN/dy − y dN/dz) dA, dN/dy = dNxi/h, dN/dz = dNeta/h, dA = h² /4 per gauss weight
            val = (zz * dNxi[a] / h - yy * dNeta[a] / h) * (h * h / 4.0)
            fw[di:di + nx, dj:dj + ny] += val * inc
    act = active
    aid = -np.ones(nn, dtype=int)
    aid[act] = np.arange(act.sum())
    na = int(act.sum())
    def Aw(xv):
        v = np.zeros(nn)
        v[act] = xv
        return apply_K(v)[act]
    dw = np.zeros(nn)
    for di, dj in offs:
        dw[di:di + nx, dj:dj + ny] += inc * (4.0 / 6.0)
    psi_v = pcg(Aw, fw[act], dw[act], tol=1e-11)
    psi = np.zeros(nn)
    psi[act] = psi_v
    Q = Ipsi = Iy_psi = Iz_psi = 0.0
    for xi, eta in gp:
        N, _, _ = shape(xi, eta)
        pv = sum(N[a] * psi[di:di + nx, dj:dj + ny] for a, (di, dj) in enumerate(offs))
        yy = Xc0 + h * xi
        zz = Yc0 + h * eta
        wgt = (h * h / 4.0) * inc
        Q += (pv * wgt).sum()
        Ipsi += (pv * pv * wgt).sum()
        Iy_psi += (zz * pv * wgt).sum()   # ∫ z ψ
        Iz_psi += (yy * pv * wgt).sum()   # ∫ y ψ
    # notation: Iy=∫z², Iz=∫y², Iyz=∫yz; I_yψ := ∫ z ψ ; I_zψ := ∫ y ψ  (see solver doc)
    D = Iy * Iz - Iyz ** 2
    Izpsi, Iypsi = Iz_psi, Iy_psi
    # ω = ψ + z_s y − y_s z with ∫ωy = ∫ωz = 0 (y horizontal)
    # ∫ωy: Izψ_ + z_s Iz − y_s Iyz = 0 ; ∫ωz: Iyψ_ + z_s Iyz − y_s Iy = 0  (ψ-moments with y and z respectively)
    # solve for (y_s, z_s)
    M = np.array([[-Iyz, Iz], [-Iy, Iyz]])
    rhs = np.array([-Izpsi, -Iypsi])
    y_s, z_s = np.linalg.solve(M, rhs)
    Iw = Ipsi - Q * Q / A_d + y_s * y_s * Iy + z_s * z_s * Iz - 2 * y_s * z_s * Iyz - 2 * y_s * Iypsi + 2 * z_s * Izpsi
    return {"A": A_d, "Iy": Iy, "Iz": Iz, "Iyz": Iyz, "It": jtors, "Iw": Iw, "y_s": y_s, "z_s": z_s, "h": h, "cells": int(ncell)}


def pcg(A, b, dg, tol=1e-10, maxit=20000):
    x = np.zeros_like(b)
    r = b - A(x)
    z = r / dg
    p = z.copy()
    rz = r @ z
    nb = math.sqrt(b @ b) or 1.0
    for it in range(maxit):
        Ap = A(p)
        alpha = rz / (p @ Ap)
        x += alpha * p
        r -= alpha * Ap
        if math.sqrt(r @ r) <= tol * nb:
            break
        z = r / dg
        rz2 = r @ z
        p = z + (rz2 / rz) * p
        rz = rz2
    return x
