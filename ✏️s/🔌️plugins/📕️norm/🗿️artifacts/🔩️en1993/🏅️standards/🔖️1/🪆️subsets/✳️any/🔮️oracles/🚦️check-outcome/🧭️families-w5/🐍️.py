"""🧭️ Independent EN 1993 families of the fifth oracle wave, written from the clause formulas and the printed tables, not from the Rust: the component method of end plate joints (EN 1993-1-8 6.2.4 T-stub modes 1 to 3 with and without prying, 6.2.6.1 column web panel in shear, 6.2.6.2 column web in compression, 6.2.6.3 column web in tension, 6.2.6.7 beam compression, 6.2.6.8 beam web in tension, 6.2.7.2 plastic row distribution with group rows, 6.3.1 initial stiffness S_j,ini with the Table 6.11 coefficients, 5.2.2.5 classification) and the second-order frame analysis of EN 1993-1-1 5.2.2 with the imperfections of 5.3.2 (P-delta and P-small-delta by stability-function-free geometric stiffness iteration on a refined mesh, cross-check by `scipy.linalg` eigenvalue amplification). SI in the interface, mm / N / MPa inside the joint functions. Every function returns rows `(id, utilization, [registry ids])`."""

import math

import numpy as np

E_MPA = 210000.0
BOLT_FUB = {"4.6": 400.0, "4.8": 400.0, "5.6": 500.0, "5.8": 500.0, "6.8": 600.0, "8.8": 800.0, "10.9": 1000.0}


def t_stub(l1, l2, tf, fy, m, n, sum_ft, a_s, n_b, lb, gm0):
    """🔩 EN 1993-1-8 Table 6.2 (method 2: F_T,1 = 4 M_pl,1 / m, without the washer term of method 1): modes 1, 2, 3 evaluated separately; prying forces exist when L_b <= L_b* = 8.8 m^3 A_s n_b / (sum l_eff,1 t_f^3), otherwise modes 1-2 and 3 only (F = 2 M_pl,1 / m). Returns (F_T,Rd, mode)."""
    mp1, mp2 = 0.25 * l1 * tf * tf * fy / gm0, 0.25 * l2 * tf * tf * fy / gm0
    lb_star = 8.8 * m**3 * a_s * n_b / (l1 * tf**3)
    if lb <= lb_star:
        modes = [4 * mp1 / m, (2 * mp2 + n * sum_ft) / (m + n), sum_ft]
    else:
        modes = [2 * mp1 / m, math.inf, sum_ft]
    best = min(modes)
    return best, modes.index(best) + 1


def omega_table_6_3(beta, b_eff, twc, avc):
    """🔩 Table 6.3: omega = 1 up to beta = 0.5, omega_1 at beta = 1, omega_2 at beta = 2, linear in between."""
    x = (b_eff * twc / avc) ** 2
    w1, w2 = 1 / math.sqrt(1 + 1.3 * x), 1 / math.sqrt(1 + 5.2 * x)
    if beta <= 0.5:
        return 1.0
    if beta < 1.0:
        return w1 + 2 * (1 - beta) * (1 - w1)
    return w1 + (min(beta, 2.0) - 1) * (w2 - w1)


def end_plate_lengths(kind, r, w, bp):
    """🔩 Table 6.6 effective lengths (cp, nc) of the end plate bolt-row outside the tension flange (extended) and of rows below it, individually."""
    if kind == "Extended":
        cp = min(2 * math.pi * r["mx"], math.pi * r["mx"] + w, math.pi * r["mx"] + 2 * r["e"])
        nc = min(4 * r["mx"] + 1.25 * r["ex"], r["e"] + 2 * r["mx"] + 0.625 * r["ex"], 0.5 * bp, 0.5 * w + 2 * r["mx"] + 0.625 * r["ex"])
        return cp, nc
    return 2 * math.pi * r["m"], r["alpha"] * r["m"]


def end_group_column(m, e, p):
    """🔩 Table 6.4 end row of a group, column flange: cp = pi m + p, nc = 2 m + 0.625 e + 0.5 p."""
    return math.pi * m + p, 2 * m + 0.625 * e + 0.5 * p


def adjacent_group(m, e, p, alpha):
    """🔩 Tables 6.5 and 6.6 row adjacent to a stiffener (or first row below the tension flange) as part of a group: cp = pi m + p, nc = 0.5 p + alpha m - (2 m + 0.625 e)."""
    return math.pi * m + p, 0.5 * p + alpha * m - (2 * m + 0.625 * e)


def group_lengths(rows, first, last, m_c, e_c):
    """🔩 Group effective lengths (column flange cp, nc, end plate cp, nc) of the consecutive rows first..last: the end rows with the pitch to their neighbour (Table 6.4 end row, Table 6.5 and 6.6 row adjacent to a stiffener or the first row below the tension flange), every inner row with p the mean of the two adjacent pitches (cp = 2p, nc = p), summed over the rows."""
    col_cp = col_nc = pl_cp = pl_nc = 0.0
    for i in range(first, last + 1):
        r = rows[i]
        if i in (first, last):
            p = rows[min(i + 1, last)]["pitch"] if i == first else r["pitch"]
            c_cp, c_nc = adjacent_group(m_c, e_c, p, r["col_alpha"]) if r["col_alpha"] > 0 else end_group_column(m_c, e_c, p)
            if i == first and r["kind"] == "FirstBelow":
                q_cp, q_nc = adjacent_group(r["m"], r["e"], p, r["alpha"])
            else:
                q_cp, q_nc = end_group_column(r["mx"] if r["kind"] == "Extended" else r["m"], r["e"], p)
        else:
            p = 0.5 * (r["pitch"] + rows[i + 1]["pitch"])
            c_cp, c_nc = q_cp, q_nc = 2 * p, p
        col_cp += c_cp
        col_nc += c_nc
        pl_cp += q_cp
        pl_nc += q_nc
    return col_cp, col_nc, pl_cp, pl_nc


def read_rows(doc, j, params):
    """🔩 The bolt rows of an end plate joint in mm: the explicit `jointRows` of the joint sorted from the tension flange down, else the rows of its bolt grid (first row extended, others below, all with the pitch)."""
    explicit = sorted((r for r in doc.get("jointRows", []) if r["jointId"] == j["id"] and r["bolts"] > 0 and r["lever"] > 0), key=lambda r: -r["lever"])
    gm2 = params["gm2"]

    def bolt(cls, a_s_mm2):
        return 0.9 * BOLT_FUB[cls] * a_s_mm2 / gm2

    if explicit:
        rows = [dict(kind=r["rowType"], lever=r["lever"] * 1e3, m=r["boltToWeb"] * 1e3, e=r["edgeDistance"] * 1e3, mx=r["extensionToFlange"] * 1e3, ex=r["extensionEdge"] * 1e3, alpha=r["alphaFactor"], pitch=r["pitchToPrevious"] * 1e3, n=r["bolts"], a_s=r["boltStressArea"] * 1e6, lb=r["gripLength"] * 1e3, ft=bolt(r["boltClass"], r["boltStressArea"] * 1e6), group=r["groupWithPrevious"] == 1, col_alpha=0.0, id=r["id"]) for r in explicit]
        return rows, True
    a_s = j["boltStressArea"] * 1e6
    rows = [dict(kind="Extended" if i == 0 else "FirstBelow", lever=j["compressionLever"] * 1e3 - i * j["pitch"] * 1e3, m=j["boltToWeb"] * 1e3, e=j["edgeDistance"] * 1e3, mx=j["extensionToFlange"] * 1e3, ex=j["extensionEdge"] * 1e3, alpha=j["alphaFactor"], pitch=j["pitch"] * 1e3, n=j["boltsPerRow"], a_s=a_s, lb=j["boltGripLength"] * 1e3, ft=bolt(j["boltClass"], a_s), group=False, col_alpha=0.0, id=None) for i in range(max(j["boltRows"], 1))]
    return rows, False


def component_method(j, rows, tp, params):
    """🔩 EN 1993-1-8 6.2.6 and 6.2.7: the isolated and the plastically distributed row resistances, the compression caps, M_j,Rd, the equivalent lever arm z and S_j,ini of an end plate joint with end plate thickness tp [mm]. Returns a dict."""
    gm0, gm1 = params["gm0"], params["gm1"]
    tfc, twc, dc, avc, rc = j["columnFlangeThickness"] * 1e3, j["columnWebThickness"] * 1e3, j["columnWebDepth"] * 1e3, j["columnWebShearArea"] * 1e6, j["columnRootRadius"] * 1e3
    fy, w, e_c, sigma = j["columnFy"] / 1e6, j["gauge"] * 1e3, j["edgeDistance"] * 1e3, j["columnWebStress"] / 1e6
    beta = max(j["transformationParameter"], 1e-6)
    tfb, ab = j["beamFlangeThickness"] * 1e3, j["weldThroat"] * 1e3
    bp, fyp = j["plateWidth"] * 1e3, j["plateFy"] / 1e6
    twb, fy_wb = j["beamWebThickness"] * 1e3, j["beamWebFy"] / 1e6
    m_c = (w - twc) / 2 - 0.8 * rc
    cp_c, nc_c = 2 * math.pi * m_c, 4 * m_c + 1.25 * e_c
    n_c = min(e_c, 1.25 * m_c)

    def web_beam(b):
        return b * twb * fy_wb / gm0 if twb > 0 and fy_wb > 0 else math.inf

    iso, parts, ks, levers = [], [], [], []
    for r in rows:
        sum_ft = r["n"] * r["ft"]
        f_col, mode_col = t_stub(min(cp_c, nc_c), nc_c, tfc, fy, m_c, n_c, sum_ft, r["a_s"], r["n"], r["lb"], gm0)
        b_t = min(cp_c, nc_c) if mode_col == 1 else nc_c
        f_web = omega_table_6_3(beta, b_t, twc, avc) * b_t * twc * fy / gm0
        cp, nc = end_plate_lengths(r["kind"], r, w, bp)
        m_use = r["mx"] if r["kind"] == "Extended" else r["m"]
        n_pl = min(r["e"], r["ex"]) if r["ex"] > 0 else r["e"]
        n_pl = min(n_pl, 1.25 * m_use)
        f_pl, mode_pl = t_stub(min(cp, nc), nc, tp, fyp, m_use, n_pl, sum_ft, r["a_s"], r["n"], r["lb"], gm0)
        f_wb = web_beam(min(cp, nc) if mode_pl == 1 else nc)
        iso.append(min(f_col, f_pl, f_web, f_wb))
        parts.append((f_col, f_pl, f_web, f_wb, mode_col, mode_pl))
        k3 = 0.7 * b_t * twc / dc
        k4 = 0.9 * min(cp_c, nc_c) * tfc**3 / m_c**3
        k5 = 0.9 * min(cp, nc) * tp**3 / m_use**3
        k10 = 1.6 * r["a_s"] * r["n"] / r["lb"]
        ks.append(1 / (1 / k3 + 1 / k4 + 1 / k5 + 1 / k10))
        levers.append(r["lever"])

    def group_capacity(first, last):
        col_cp, col_nc, pl_cp, pl_nc = group_lengths(rows, first, last, m_c, e_c)
        n_b = sum(rows[i]["n"] for i in range(first, last + 1))
        sum_ft = sum(rows[i]["n"] * rows[i]["ft"] for i in range(first, last + 1))
        a_s, lb = rows[first]["a_s"], rows[first]["lb"]
        f_col, mode_col = t_stub(min(col_cp, col_nc), col_nc, tfc, fy, m_c, n_c, sum_ft, a_s, n_b, lb, gm0)
        m_use = rows[first]["mx"] if rows[first]["kind"] == "Extended" else rows[first]["m"]
        n_pl = min(min(rows[first]["e"], rows[first]["ex"]) if rows[first]["ex"] > 0 else rows[first]["e"], 1.25 * m_use)
        f_pl, mode_pl = t_stub(min(pl_cp, pl_nc), pl_nc, tp, fyp, m_use, n_pl, sum_ft, a_s, n_b, lb, gm0)
        b_t = min(col_cp, col_nc) if mode_col == 1 else col_nc
        b_p = min(pl_cp, pl_nc) if mode_pl == 1 else pl_nc
        return min(f_col, f_pl, omega_table_6_3(beta, b_t, twc, avc) * b_t * twc * fy / gm0, web_beam(b_p))

    b_c = tfb + 2 * math.sqrt(2) * ab + 5 * (tfc + rc)
    omega_c = omega_table_6_3(beta, b_c, twc, avc)
    kwc = 1.0 if sigma <= 0.7 * fy else 1.7 - sigma / fy
    lam_p = 0.932 * math.sqrt(b_c * dc * fy / (E_MPA * twc**2))
    rho = 1.0 if lam_p <= 0.72 else (lam_p - 0.2) / lam_p**2
    fc_wc = min(omega_c * kwc * rho * b_c * twc * fy / gm1, omega_c * kwc * b_c * twc * fy / gm0)
    v_wp = 0.9 * fy * avc / (math.sqrt(3) * gm0) / beta
    f_beam = j["beamMomentResistance"] * 1e3 / (j["compressionLever"] * 1e3)
    cap = min(fc_wc, v_wp, f_beam)
    forces = []
    for x, value in enumerate(iso):
        f = min(value, cap - sum(forces))
        for r in range(x):
            if all(rows[i]["group"] for i in range(r + 1, x + 1)):
                f = min(f, group_capacity(r, x) - sum(forces[r:x]))
        forces.append(max(f, 0.0))
    m_rd = sum(h * f for h, f in zip(levers, forces))
    num, den = sum(k * h * h for k, h in zip(ks, levers)), sum(k * h for k, h in zip(ks, levers))
    z = num / den
    k_eq = den / z
    k1 = 0.38 * avc / (beta * z)
    k2 = 0.7 * b_c * twc / dc
    s_ini = E_MPA * z * z / (1 / k1 + 1 / k2 + 1 / k_eq)
    return dict(iso=iso, parts=parts, forces=forces, cap=cap, fc_wc=fc_wc, v_wp=v_wp, f_beam=f_beam, m_rd=m_rd, z=z, s_ini=s_ini, levers=levers)


def stiffness_class_ratio(j, s_ini):
    """🔩 EN 1993-1-8 5.2.2.5: utilisation of the assumed stiffness class (rigid: k_b E I_b / L_b over S_j,ini; pinned: S_j,ini over 0.5 E I_b / L_b; semi-rigid: the larger distance to either limit)."""
    ib, lb = j["beamInertia"] * 1e12, j["beamSpan"] * 1e3
    upper = (8.0 if j["bracedFrame"] == 1 else 25.0) * E_MPA * ib / lb
    lower = 0.5 * E_MPA * ib / lb
    cls = j["assumedClass"]
    if cls == "Rigid":
        return upper / s_ini
    if cls == "Pinned":
        return s_ini / lower
    return max(lower / s_ini, s_ini / upper)


def joint_component_rows(doc, params):
    """🔩 Rows `en1993.1-8.6.2.row.<row>`, `en1993.1-8.6.2.comp.<joint>`, `en1993.1-8.6.3.mj.<joint>`, `en1993.1-8.5.2.sj.<joint>` of every end plate joint with end plate, column flange and compression lever."""
    out = []
    gammas = ["ndp.en1993.gamma-m0", "ndp.en1993.gamma-m1", "ndp.en1993.gamma-m2"]
    for j in doc["joints"]:
        if not (j["kind"] == "EndPlate" and j["plateThickness"] > 0 and j["columnFlangeThickness"] > 0 and j["compressionLever"] > 0):
            continue
        rows, explicit = read_rows(doc, j, params)
        res = component_method(j, rows, j["plateThickness"] * 1e3, params)
        m_ed = j["momentDemand"]
        if explicit:
            h2 = sum(h * h for h in res["levers"])
            for r, iso in zip(rows, res["iso"]):
                out.append((f"en1993.1-8.6.2.row.{r['id']}", m_ed * 1e3 * r["lever"] / h2 / iso, gammas + ["ndp.en1993.v-420.1", "ndp.en1993.v-420.2"]))
            if m_ed != 0:
                out.append((f"en1993.1-8.6.2.comp.{j['id']}", abs(m_ed) * 1e3 / res["z"] / res["cap"], gammas[:2]))
        out.append((f"en1993.1-8.6.3.mj.{j['id']}", m_ed / (res["m_rd"] / 1e3), gammas + ["ndp.en1993.v-420.1"]))
        if j["assumedClass"] != "Unset" and j["beamInertia"] > 0 and j["beamSpan"] > 0:
            out.append((f"en1993.1-8.5.2.sj.{j['id']}", stiffness_class_ratio(j, res["s_ini"]), ["ndp.en1993.v-431.2"]))
    return out


def self_check_joint():
    """🔬 The T-stub of the spec example E10 (two M20 8.8 bolts, tp = 15 mm S355, m = n = 40 mm, l_eff = 210 mm) by hand: F_T,1 = 419.3, F_T,2 = 245.96, F_T,3 = 282.24 kN, mode 2."""
    ft, mode = t_stub(210.0, 210.0, 15.0, 355.0, 40.0, 40.0, 2 * 141120.0, 245.0, 2, 60.0, 1.0)
    assert mode == 2 and abs(ft - 245956.0) < 200.0, (ft, mode)
    return {"tstub_e10": ft, "mode": mode}


SEGMENTS = 8
SUPPORT = {"Fixed": (True, True, True), "Pinned": (True, True, False), "Free": (False, False, False), "Roller": (False, True, False)}
BOW_ELASTIC = {"a0": 350.0, "a": 300.0, "b": 250.0, "c": 200.0, "d": 150.0}


def local_z(c, s):
    """🧭 The in-plane direction of the local z axis of a bar with direction cosines (c, s) in the kernel convention ey = Z x ex (global Y for vertical bars), ez = ex x ey: (-s, c) for bars running in +x, (s, -c) for bars running in -x, (-sign(s), 0) for vertical bars."""
    if abs(s) > 0.999999:
        return (-math.copysign(1.0, s), 0.0)
    sign = 1.0 if c > 0 else -1.0
    return (-s * sign, c * sign)


class Frame:
    """🕸 Planar frame of the snapshot meshed at `SEGMENTS` Euler-Bernoulli elements per bar (cubic Hermite stiffness, consistent geometric stiffness of the element axial force); pinned bar ends keep their own rotation; line and nodal loads in the global x-z plane."""

    def __init__(self, doc):
        self.doc = doc
        nodes = {n["id"]: n for n in doc["frameNodes"]}
        self.index = {n["id"]: i for i, n in enumerate(doc["frameNodes"])}
        self.count = 3 * len(self.index)
        self.elements = []
        self.bars = {}
        for bar in doc["frameBars"]:
            member = next(m for m in doc["members"] if m["id"] == bar["memberId"])
            section = next(s for s in doc["sections"] if s["id"] == member["sectionId"])
            material = next(m for m in doc["materials"] if m["id"] == member["materialId"])
            a, b = nodes[bar["startNodeId"]], nodes[bar["endNodeId"]]
            length = math.hypot(b["x"] - a["x"], b["z"] - a["z"])
            c, s = (b["x"] - a["x"]) / length, (b["z"] - a["z"]) / length
            chain = [[3 * self.index[a["id"]] + k for k in range(3)]]
            for _ in range(1, SEGMENTS):
                chain.append([self.count, self.count + 1, self.count + 2])
                self.count += 3
            chain.append([3 * self.index[b["id"]] + k for k in range(3)])
            for end, slot in ((bar["releaseStart"], 0), (bar["releaseEnd"], -1)):
                if end == "Pinned":
                    chain[slot] = [chain[slot][0], chain[slot][1], self.count]
                    self.count += 1
            first = len(self.elements)
            for d0, d1 in zip(chain[:-1], chain[1:]):
                self.elements.append(dict(dofs=d0 + d1, l=length / SEGMENTS, c=c, s=s, ea=material["eModulus"] * section["area"], ei=material["eModulus"] * section["iy"], bar=bar["id"]))
            self.bars[bar["id"]] = (first, len(self.elements), length, c, s)
        self.held = {3 * self.index[n["id"]] + k for n in doc["frameNodes"] for k, flag in enumerate(SUPPORT[n["support"]]) if flag}
        self.free = [d for d in range(self.count) if d not in self.held]

    @staticmethod
    def rotation(c, s):
        r = np.zeros((6, 6))
        for o in (0, 3):
            r[o:o + 3, o:o + 3] = [[c, s, 0], [-s, c, 0], [0, 0, 1]]
        return r

    @staticmethod
    def stiffness(l, ea, ei):
        k = np.zeros((6, 6))
        k[np.ix_([0, 3], [0, 3])] = ea / l * np.array([[1, -1], [-1, 1]])
        k[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = ei / l**3 * np.array([[12, 6 * l, -12, 6 * l], [6 * l, 4 * l * l, -6 * l, 2 * l * l], [-12, -6 * l, 12, -6 * l], [6 * l, 2 * l * l, -6 * l, 4 * l * l]])
        return k

    @staticmethod
    def geometric(l, n):
        g = np.zeros((6, 6))
        g[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = n / (30 * l) * np.array([[36, 3 * l, -36, 3 * l], [3 * l, 4 * l * l, -3 * l, -l * l], [-36, -3 * l, 36, -3 * l], [3 * l, -l * l, -3 * l, 4 * l * l]])
        return g

    def solve(self, nodal, line=None, iterate=True):
        """🔁 Second-order (or first-order) displacement vector under the nodal forces `{node: (fx, fz)}` and the transverse bar loads `line = {bar: q}` along the kernel local z; returns (displacements, per-element local end forces (tension positive axial), converged, iterations)."""
        f = np.zeros(self.count)
        for node, (fx, fz) in nodal.items():
            f[3 * self.index[node]] += fx
            f[3 * self.index[node] + 1] += fz
        equivalent = {}
        for bar, q in (line or {}).items():
            first, last, length, c, s = self.bars[bar]
            ez = local_z(c, s)
            w = ez[0] * (-s) + ez[1] * c
            for i in range(first, last):
                l = self.elements[i]["l"]
                local = np.array([0.0, w * q * l / 2, w * q * l * l / 12, 0.0, w * q * l / 2, -w * q * l * l / 12])
                equivalent[i] = local
                f[self.elements[i]["dofs"]] += self.rotation(self.elements[i]["c"], self.elements[i]["s"]).T @ local
        k_mat = np.zeros((self.count, self.count))
        rotations = [self.rotation(el["c"], el["s"]) for el in self.elements]
        locals_k = [self.stiffness(el["l"], el["ea"], el["ei"]) for el in self.elements]
        for el, r, k in zip(self.elements, rotations, locals_k):
            k_mat[np.ix_(el["dofs"], el["dofs"])] += r.T @ k @ r
        free = self.free
        u = np.zeros(self.count)
        u[free] = np.linalg.solve(k_mat[np.ix_(free, free)], f[free])
        n = self.axial(u, rotations)
        converged, iterations = True, 0
        if iterate:
            converged = False
            for iterations in range(1, 101):
                t_mat = k_mat.copy()
                for el, r, forces in zip(self.elements, rotations, n):
                    t_mat[np.ix_(el["dofs"], el["dofs"])] += r.T @ self.geometric(el["l"], forces) @ r
                if np.linalg.eigvalsh(t_mat[np.ix_(free, free)])[0] <= 0:
                    raise ValueError("the load reaches the elastic critical load")
                u = np.zeros(self.count)
                u[free] = np.linalg.solve(t_mat[np.ix_(free, free)], f[free])
                new = self.axial(u, rotations)
                change = max(abs(a - b) for a, b in zip(new, n))
                n = new
                if change <= 1e-9 * max(1.0, max(abs(v) for v in n)):
                    converged = True
                    break
        ends = []
        for i, (el, r, k) in enumerate(zip(self.elements, rotations, locals_k)):
            local = r @ u[el["dofs"]]
            force = k @ local - equivalent.get(i, np.zeros(6))
            ends.append(force)
        return u, ends, converged, iterations

    def axial(self, u, rotations):
        return [el["ea"] / el["l"] * ((r @ u[el["dofs"]])[3] - (r @ u[el["dofs"]])[0]) for el, r in zip(self.elements, rotations)]

    def effects(self, ends):
        """📊 Per bar the signed axial force of the largest magnitude (tension positive, mean of the element end forces) and the largest end moment magnitude."""
        out = {}
        for bar, (first, last, *_) in self.bars.items():
            n = max((0.5 * (-ends[i][0] + ends[i][3]) for i in range(first, last)), key=abs)
            m = max(max(abs(ends[i][2]), abs(ends[i][5])) for i in range(first, last))
            out[bar] = (n, m)
        return out


def sway_forces(doc, frame, nodal, first_n, phi0):
    """📐 EN 1993-1-1 5.3.2(3) to (7): phi = phi0 alpha_h alpha_m (alpha_h = 2/sqrt(h) within 2/3..1, alpha_m = sqrt(0.5 (1 + 1/m)), m the columns of a storey whose compression is at least half the storey mean, the smallest m over the storeys), the equivalent horizontal force phi V at every node that is not a fixed or pinned support, dropped where the storey below carries H >= 0.15 V (5.3.2(4)). Returns (phi, {node: force}, m, exempt storeys)."""
    nodes = doc["frameNodes"]
    z_lo, z_hi = min(n["z"] for n in nodes), max(n["z"] for n in nodes)
    height = z_hi - z_lo
    level = {n["id"]: n["z"] for n in nodes}
    compressed = {}
    for bar in doc["frameBars"]:
        a, b = bar["startNodeId"], bar["endNodeId"]
        dx, dz = abs(next(n["x"] for n in nodes if n["id"] == b) - next(n["x"] for n in nodes if n["id"] == a)), abs(level[b] - level[a])
        if dz >= dx and -first_n[bar["id"]] > 0:
            compressed.setdefault(round(min(level[a], level[b]), 6), []).append(-first_n[bar["id"]])
    m = min((sum(1 for v in forces if v >= 0.5 * sum(forces) / len(forces)) for forces in compressed.values()), default=0)
    if m == 0 or height <= 0:
        return 0.0, {}, m, 0
    phi = phi0 * min(max(2 / math.sqrt(height), 2 / 3), 1.0) * math.sqrt(0.5 * (1 + 1 / m))
    levels = sorted({round(n["z"], 6) for n in nodes})
    exempt = {}
    for lv in levels[1:]:
        above = [n for n in nodes if n["z"] >= lv - 1e-6]
        h = sum(nodal.get(n["id"], (0.0, 0.0))[0] for n in above)
        v = -sum(nodal.get(n["id"], (0.0, 0.0))[1] for n in above)
        exempt[lv] = v > 0 and abs(h) >= 0.15 * v
    forces = {}
    for n in nodes:
        if n["support"] in ("Fixed", "Pinned"):
            continue
        below = levels.index(round(n["z"], 6)) - 1
        if below >= 0 and exempt[levels[below + 1]]:
            continue
        force = phi * -nodal.get(n["id"], (0.0, 0.0))[1]
        if force > 0:
            forces[n["id"]] = force
    return phi, forces, m, sum(exempt.values())


def bow_loads(doc, M, first_n):
    """📐 EN 1993-1-1 5.3.2(6) and (11): a bow e0 = L / (350, 300, 250, 200, 150 for curve a0 to d) as the equivalent load q = 8 N e0 / L^2 on every compressed bar with at least one moment-resisting end whose slenderness exceeds 0.5 sqrt(A f_y / N_Ed)."""
    out = {}
    for bar in doc["frameBars"]:
        if bar["releaseStart"] == "Pinned" and bar["releaseEnd"] == "Pinned":
            continue
        member = next(m for m in doc["members"] if m["id"] == bar["memberId"])
        section = next(s for s in doc["sections"] if s["id"] == member["sectionId"])
        material = next(m for m in doc["materials"] if m["id"] == member["materialId"])
        n_ed = -first_n[bar["id"]]
        nodes = {n["id"]: n for n in doc["frameNodes"]}
        a, b = nodes[bar["startNodeId"]], nodes[bar["endNodeId"]]
        length = math.hypot(b["x"] - a["x"], b["z"] - a["z"])
        l_cr = member["bucklingLengthY"] if member["bucklingLengthY"] > 0 else length
        lam = l_cr / math.sqrt(section["iy"] / section["area"]) / (math.pi * math.sqrt(material["eModulus"] / material["fy"]))
        if n_ed > 0 and length > 0 and lam > 0.5 * math.sqrt(section["area"] * material["fy"] / n_ed):
            curve = M.buckling_curve(section, True, material["fy"])
            out[bar["id"]] = 8 * n_ed * (length / BOW_ELASTIC[curve]) / length**2
    return out


def variant_peak(frame, effects):
    return sum(m for _, m in effects.values())


def unfavourable(frame, nodal, sway, bows, with_bow):
    """🔁 The four sign combinations of the sway forces and the bow loads (EN 1993-1-1 5.3.2(10)); the one with the largest sum of the bar peak moments is the unfavourable one (a tie keeps the first). Returns (ends, effects, converged)."""
    best = None
    for s in ([1.0, -1.0] if sway else [1.0]):
        for q in ([1.0, -1.0] if with_bow and bows else [1.0]):
            loads = {k: (v[0], v[1]) for k, v in nodal.items()}
            for node, force in sway.items():
                x, z = loads.get(node, (0.0, 0.0))
                loads[node] = (x + s * force, z)
            u, ends, converged, _ = frame.solve(loads, {b: q * v for b, v in bows.items()} if with_bow else None)
            effects = frame.effects(ends)
            if best is None or variant_peak(frame, effects) > variant_peak(frame, best[1]):
                best = (ends, effects, converged)
    return best


def section_data(doc, bar, M, params):
    member = next(m for m in doc["members"] if m["id"] == bar["memberId"])
    section = next(s for s in doc["sections"] if s["id"] == member["sectionId"])
    material = next(m for m in doc["materials"] if m["id"] == member["materialId"])
    return member, section, material, M.section_class(section, material["fy"])


def second_order_rows(doc, M, FAM, params):
    """🕸 EN 1993-1-1 5.2.2(3)c rows `en1993.1-1.5.2.second-order.<bar>` (class 1 to 3 cross-section resistance 6.2.9 under the second-order forces with sway and bow imperfections) and 5.2.2(3)b rows `en1993.1-1.5.2.member.<bar>` (6.3.3, Annex B, with the sway imperfection only), emitted when alpha_cr < 10; node loads only (no `frameActions`), no semi-rigid releases."""
    if not doc.get("frameBars") or doc.get("frameActions") or any(b["releaseStart"] == "SemiRigid" or b["releaseEnd"] == "SemiRigid" for b in doc["frameBars"]):
        return []
    if FAM.frame_governing(doc) >= 10.0:
        return []
    registry = FAM.registry()
    phi0 = registry["ndp.en1993.1-1.phi0"]["recommended"]
    frame = Frame(doc)
    nodal = {n["id"]: (n["loadX"], n["loadZ"]) for n in doc["frameNodes"] if n["loadX"] or n["loadZ"]}
    try:
        _, base_ends, _, _ = frame.solve(nodal, None, iterate=False)
        first_n = {bar: n for bar, (n, _) in frame.effects(base_ends).items()}
        phi, sway, m_cols, _ = sway_forces(doc, frame, nodal, first_n, phi0)
        bows = bow_loads(doc, M, first_n)
        _, full, _ = unfavourable(frame, nodal, sway, bows, True)
        _, sway_only, _ = unfavourable(frame, nodal, sway, bows, False) if bows else (None, full, True)
    except ValueError:
        return []
    out = []
    cites = ["ndp.en1993.gamma-m0", "ndp.en1993.1-1.phi0"]
    for bar in doc["frameBars"]:
        member, section, material, cls = section_data(doc, bar, M, params)
        fy = material["fy"]
        if cls == 4 or section["kind"] != "RolledI":
            continue
        n, m = full[bar["id"]]
        w_y = section["wPlY"] if cls <= 2 else section["wElY"]
        w_z = section["wPlZ"] if cls <= 2 else section["wElZ"]
        eta = M.mn_eta(-n, m, 0.0, section["area"] * fy / params["gm0"], w_y * fy / params["gm0"], w_z * fy / params["gm0"], section["area"], section["b"], section["tf"], cls, section["kind"])
        out.append((f"en1993.1-1.5.2.second-order.{bar['id']}", eta, cites))
        n, m = sway_only[bar["id"]]
        row = member_interaction(member, section, material, cls, -n, m, M, params)
        if row is not None:
            out.append((f"en1993.1-1.5.2.member.{bar['id']}", row, ["ndp.en1993.gamma-m1", "ndp.en1993.1-1.phi0"]))
    return out


def member_interaction(member, section, material, cls, n_ed, m_ed, M, params):
    """🔗 EN 1993-1-1 6.3.3 (Annex B, equations 6.61 and 6.62) of a compressed beam-column under the second-order forces of the frame, with the buckling lengths of the member; the larger of the two equations. None for members that are not beam-columns or columns, tension or hollow sections."""
    kind = member["memberType"].lower().replace("_", "").replace("-", "")
    if kind not in ("beamcolumn", "column") or n_ed <= 0:
        return None
    fy, e = material["fy"], material["eModulus"]
    lam1 = math.pi * math.sqrt(e / fy)
    length = member["length"]
    l_y = member["bucklingLengthY"] if member["bucklingLengthY"] > 0 else length
    l_z = member["bucklingLengthZ"] if member["bucklingLengthZ"] > 0 else length
    lam_y = l_y / math.sqrt(section["iy"] / section["area"]) / lam1
    lam_z = l_z / math.sqrt(section["iz"] / section["area"]) / lam1
    chi_y = M.chi(lam_y, M.buckling_curve(section, True, fy))
    chi_z = M.chi(lam_z, M.buckling_curve(section, False, fy))
    w_y = section["wPlY"] if cls <= 2 else section["wElY"]
    w_z = section["wPlZ"] if cls <= 2 else section["wElZ"]
    torsion = kind != "column"
    m_cr = M.critical_moment(section, material, max(member["ltbLength"], member["ltbRestraintSpacing"]), member)
    lam_lt = 10.0 if m_cr <= 0 else math.sqrt(w_y * fy / m_cr)
    curve_lt = ("c" if section["h"] / section["b"] <= 2 else "d") if "weld" in section["kind"].lower() else ("a" if section["h"] / section["b"] <= 2 else "b")
    chi_lt = M.chi(lam_lt, curve_lt) if torsion else 1.0
    cm = M.cm_factor(member)
    return M.interaction_annex_b(n_ed, m_ed, 0.0, section["area"] * fy, w_y * fy, w_z * fy, chi_y, chi_z, chi_lt, lam_y, lam_z, cm, cm, cm, cls, torsion, params["gm1"])


def crane_rows(doc, params, registry):
    """🚜 EN 1993-6 5.7 and 9.4: the web under the wheel load (effective loaded length l_eff = 3.25 (I_rf / t_w)^(1/3) with I_rf = I_r (fixed rail) + b_eff t_f^3 / 12, sigma_oz = F_z / (l_eff t_w), tau = 0.2 sigma_oz + T t_w / I_T with T = F_z e, equivalent stress sqrt(sx^2 + sz^2 - sx sz + 3 tau^2) / (f_y / gamma_M0)) and the fatigue of the runway (lambda = (nu k_Q)^(1/m), gamma_Ff lambda phi_fat Delta sigma gamma_Mf / Delta sigma_C), `en1993.6.5.7.vm.<crane>` and `en1993.6.9.fat.<crane>`."""
    out = []
    for c in doc.get("craneRunways", []):
        if not (c["maxWheelLoad"] > 0 and c["webThickness"] > 0):
            continue
        tw, fy = c["webThickness"] * 1e3, c["fy"] / 1e6
        fz = c["loadFactor"] * c["phi"] * c["maxWheelLoad"]
        i_rf = (c["railInertia"] * 1e12 if c["railFixed"] == 1 else 0.0) + c["flangeEffectiveWidth"] * 1e3 * (c["flangeThickness"] * 1e3) ** 3 / 12.0
        l_eff = 3.25 * (i_rf / tw) ** (1.0 / 3.0)
        s_oz = fz / (l_eff * tw)
        tau = 0.2 * s_oz + fz * c["wheelEccentricity"] * 1e3 * tw / (c["torsionConstant"] * 1e12)
        s_x = c["longitudinalStress"] / 1e6
        out.append((f"en1993.6.5.7.vm.{c['id']}", math.sqrt(s_x**2 + s_oz**2 - s_x * s_oz + 3 * tau**2) / (fy / params["gm0"]), ["ndp.en1993.v-890.1", "ndp.en1993.gamma-m0"]))
        if c["craneNu"] > 0 and c["stressRange"] > 0 and c["detailCategoryStress"] > 0:
            lam = (c["craneNu"] * c["craneKq"]) ** (1.0 / c["damageExponent"])
            gm_f = registry["ndp.en1993.gamma-mf"]["recommended"]
            out.append((f"en1993.6.9.fat.{c['id']}", 1.0 * lam * c["fatiguePhi"] * c["stressRange"] / 1e6 * gm_f / (c["detailCategoryStress"] / 1e6), ["ndp.en1993.v-890.2", "ndp.en1993.gamma-mf"]))
    return out


def finish(rows, FAM):
    """⚖ Rows to oracle dicts: fail above 1, `unverifiedBasis` when a cited registry row is unverified, else pass."""
    entries = FAM.registry()
    out = []
    for row_id, u, cites in rows:
        unverified = any(entries[name]["status"] != "verified" for name in cites)
        out.append({"id": row_id, "utilization": u, "status": "fail" if u > 1.0 + 1e-9 else "unverifiedBasis" if unverified else "pass"})
    return out


def evaluate(doc, M, FAM, params):
    rows = joint_component_rows(doc, params)
    rows += second_order_rows(doc, M, FAM, params)
    rows += crane_rows(doc, params, FAM.registry())
    return finish(rows, FAM)


def self_check_second_order():
    """🔬 Cantilever column with tip load H and axial load P: base moment M = H tan(kL)/k (k = sqrt(P/EI)) of the exact second-order solution against the eight-element solver, and the same column in PyNite (`analyze_PDelta`, eight member segments)."""
    length, ei, ea, p, h = 4.0, 2.1e11 * 1.126e-4, 2.1e11 * 0.0106, 3.0e5, 2.0e4
    doc = dict(frameNodes=[dict(id="a", x=0.0, z=0.0, support="Fixed", loadX=0.0, loadZ=0.0), dict(id="b", x=0.0, z=length, support="Free", loadX=h, loadZ=-p)], frameBars=[dict(id="c", memberId="m", startNodeId="a", endNodeId="b", releaseStart="Rigid", releaseEnd="Rigid")], members=[dict(id="m", sectionId="s", materialId="e")], sections=[dict(id="s", area=0.0106, iy=1.126e-4)], materials=[dict(id="e", eModulus=2.1e11)])
    frame = Frame(doc)
    _, ends, converged, _ = frame.solve({"a": (0.0, 0.0), "b": (h, -p)})
    ours = frame.effects(ends)["c"][1]
    k = math.sqrt(p / ei)
    exact = h * math.tan(k * length) / k
    assert converged and abs(ours - exact) <= 2e-3 * exact, (ours, exact)
    from Pynite import FEModel3D

    model = FEModel3D()
    model.add_material("S", 2.1e11, 8.1e10, 0.3, 7850)
    model.add_section("Sec", 0.0106, 1.126e-4 * 4, 1.126e-4, 1e-6)
    steps = 8
    for i in range(steps + 1):
        model.add_node(f"n{i}", 0, length * i / steps, 0)
        if i:
            model.add_member(f"m{i}", f"n{i - 1}", f"n{i}", "S", "Sec")
    model.def_support("n0", True, True, True, True, True, True)
    model.add_node_load(f"n{steps}", "FX", h, "Case 1")
    model.add_node_load(f"n{steps}", "FY", -p, "Case 1")
    model.add_load_combo("Combo 1", {"Case 1": 1.0})
    model.analyze_PDelta(check_stability=False)
    theirs = abs(model.members["m1"].moment("Mz", 0, "Combo 1"))
    assert abs(theirs - exact) <= 1.5e-2 * exact, (theirs, exact)
    return {"ours": ours, "exact": exact, "pynite": theirs}


def self_check():
    return {"joint": self_check_joint(), "secondOrder": self_check_second_order()}
