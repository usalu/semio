"""🧭 Independent EN 1992-1-1 member families beyond flexure and shear: torsion with the thin-walled section (6.3.1, 6.3.2: T_Rd,max, links and longitudinal steel of Eq. 6.28, combined torsion and shear Eq. 6.29) and the three-node strut-and-tie model of a deep beam (6.5.2 to 6.5.4, 5.3.1(3): tie, strut, support node CCT and load node CCC). Written from the clauses and the strut-and-tie geometry in `📐️specs/en1992-target.md` (ST-01..ST-04, T-01..T-04); the truss is solved by the method of joints, the second method (`--self-check`) is a least-squares equilibrium solve with numpy and the thin-walled geometry of an inward mitre buffer with shapely. Units inside: N, mm, MPa."""

import json
import math
from pathlib import Path

import numpy as np
from shapely.geometry import box

DEEP_BEAM_RATIO = 3.0
_REGISTRY = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
_VERIFIED = {row["id"] for row in json.load(open(_REGISTRY, encoding="utf8"))["entries"] if row["status"] == "verified"}


def basis(row_id, utilization, cited):
    """🚧 The row as the kernel reports it: a pass that cites a registry row which is not double-keyed is `unverifiedBasis`, a failure stays a failure, a row that cites nothing is a plain pass."""
    unverified = utilization <= 1.0 + 1e-9 and any(i not in _VERIFIED for i in cited)
    return (row_id, utilization, {"status": "unverifiedBasis"}) if unverified else (row_id, utilization)

T_MIN_WALL = lambda cover, link, phi_l: 2.0 * (cover + link + phi_l / 2.0)


def torsion_wall(b, h, cover, link, phi_l):
    """🌀 t_ef = max(A/u, 2 (c + phi_link + phi_l/2)) (6.3.2(1): edge to centre of the bar, the nominal cover reaches the link), A_k and u_k of the mid-line of the wall of a rectangle (mm)."""
    t = max(b * h / (2.0 * (b + h)), T_MIN_WALL(cover, link, phi_l))
    t = min(t, 0.999 * min(b, h) / 2.0)
    return t, (b - t) * (h - t), 2.0 * ((b - t) + (h - t))


def strut_nu(p, fck):
    """🧱 ν of 6.2.3(103): 0.6 (1 - fck / 250), DE 0.75 min(1.1 - fck / 500, 1)."""
    return p["nu_de"] * min(1.1 - fck / 500.0, 1.0) if p["nu_de"] else 0.6 * (1.0 - fck / 250.0)


def torsion_rows(member, p, fck, fyk, bw, d, a_tension, a_compression, uls_t, uls_v, uls_m, m_rd, cot, stirrups):
    """🌀 `torsion`, `torsion-links`, `torsion-longitudinal`, `torsion-longitudinal-compression` and `interaction` rows of a member with a torsion moment."""
    mid = member["id"]
    b, h, cover = member["width"] * 1e3, member["height"] * 1e3, member["cover"] * 1e3
    phi_l = max((layer["diameter"] * 1e3 for layer in member["longitudinal"]), default=0.0)
    link = stirrups["diameter"] * 1e3 if stirrups else 0.0
    t, a_k, u_k = torsion_wall(b, h, cover, link, phi_l)
    fcd, fyd, nu = p["alpha_cc"] * fck / p["gamma_c"], fyk / p["gamma_s"], strut_nu(p, fck)
    t_ed = abs(uls_t) * 1e3
    t_rd = lambda c: 2.0 * nu * fcd * a_k * t * c / (1.0 + c * c)
    cited = ("ndp.en1992.t-04.1",)
    rows = [basis(f"en1992.6.3.torsion.{mid}", t_ed / max(t_rd(1.0), 1.0), cited)]
    if stirrups and stirrups["spacing"] > 0 and stirrups["diameter"] > 0:
        z = 0.9 * d
        leg = math.pi * (stirrups["diameter"] * 500.0) ** 2 / (stirrups["spacing"] * 1e3)
        shear_share = abs(uls_v) / (z * fyd * cot) / max(stirrups["legs"], 1)
        provided = max(leg - shear_share, 1e-9)
        asw_s = t_ed / (2.0 * a_k * fyd * cot)
        asl = t_ed * u_k * cot / (2.0 * a_k * fyd)
        rows += [
            basis(f"en1992.6.3.torsion-links.{mid}", asw_s / provided, cited),
            basis(f"en1992.6.3.torsion-longitudinal.{mid}", (a_tension * abs(uls_m) / max(m_rd, 1.0) + 0.5 * asl) / max(a_tension, 1e-3), cited),
            basis(f"en1992.6.3.torsion-longitudinal-compression.{mid}", 0.5 * asl / max(a_compression, 1e-3), cited),
        ]
    z = 0.9 * d
    v_rd_max = bw * z * nu * fcd / (cot + 1.0 / cot)
    rows.append(basis(f"en1992.6.3.interaction.{mid}", t_ed / t_rd(cot) + abs(uls_v) / v_rd_max, cited))
    return rows


def truss(span, height, effective_depth, load):
    """🕸️ The three-node truss of a deep beam (mm, N): supports at 0.15 h from the ends, tie at the effective depth (0.1 h above the bottom without one), top node at mid-span 0.1 h below the top. Returns the tie force (tension positive), the strut force (compression positive) and the strut angle."""
    z_tie = height / 2.0 - effective_depth if 0.0 < effective_depth < height else -height / 2.0 + 0.1 * height
    x_a, z_c = 0.15 * height, height / 2.0 - 0.1 * height
    theta = math.atan2(z_c - z_tie, span / 2.0 - x_a)
    strut = load / (2.0 * math.sin(theta))
    return strut * math.cos(theta), strut, theta


def strut_and_tie_rows(member, p, fck, fyk, uls_v, a_tension):
    """🕸️ `tie`, `strut`, `node-cct` and `node-ccc` rows of a beam with span at most 3 h; strut and nodes need the bearing plate width a (not applicable without)."""
    mid = member["id"]
    span, height = member["span"] * 1e3, member["height"] * 1e3
    load = 2.0 * abs(uls_v)
    if member["kind"] != "Beam" or span <= 0 or height <= 0 or span / height > DEEP_BEAM_RATIO or load <= 0:
        return []
    tie, strut, theta = truss(span, height, member["effectiveDepth"] * 1e3, load)
    if tie <= 0:
        return []
    rows = [{"id": f"en1992.6.5.tie.{mid}", "utilization": tie / (fyk / p["gamma_s"]) / max(a_tension, 1e-9)}]
    plate = member["bearingWidth"] * 1e3
    names = ("strut", "node-cct", "node-ccc")
    if plate <= 0:
        return rows + [{"id": f"en1992.6.5.{n}.{mid}", "utilization": 0.0, "status": "notApplicable"} for n in names]
    fcd, nu_prime, b = p["alpha_cc"] * fck / p["gamma_c"], 1.0 - fck / 250.0, member["width"] * 1e3
    k1, k2, strut_factor = p["node_k1"], p["node_k2"], p["strut_cracked"]
    width = plate * math.sin(theta)
    persistent = ("ndp.en1992.m-01.alpha-cc-persistent", "ndp.en1992.m-01.gamma-c-persistent")
    nodes = ("ndp.en1992.node-k1", "ndp.en1992.node-k2") + persistent
    graded = [
        basis(f"en1992.6.5.strut.{mid}", strut / (b * width) / (strut_factor * nu_prime * fcd), ("ndp.en1992.strut-cracked",) + persistent),
        basis(f"en1992.6.5.node-cct.{mid}", load / 2.0 / (b * plate) / (k2 * nu_prime * fcd), nodes),
        basis(f"en1992.6.5.node-ccc.{mid}", load / (b * plate) / (k1 * nu_prime * fcd), nodes),
    ]
    return rows + [{"id": g[0], "utilization": g[1], **(g[2] if len(g) > 2 else {})} for g in graded]


def self_check():
    """🔬 The truss against a least-squares equilibrium solve with numpy and the thin-walled geometry against the shapely inward mitre buffer."""
    span, height, depth, load = 2400.0, 1200.0, 1100.0, 1.0e6
    tie, strut, theta = truss(span, height, depth, load)
    z_tie, x_a, z_c = height / 2.0 - depth, 0.15 * height, height / 2.0 - 0.1 * height
    nodes = np.array([(x_a, z_tie), (span - x_a, z_tie), (span / 2.0, z_c)])
    members = [(0, 1), (0, 2), (1, 2)]
    a = np.zeros((6, 6))
    for k, (i, j) in enumerate(members):
        e = (nodes[j] - nodes[i]) / np.linalg.norm(nodes[j] - nodes[i])
        a[2 * i : 2 * i + 2, k] += e
        a[2 * j : 2 * j + 2, k] -= e
    a[0, 3], a[1, 4], a[3, 5] = 1.0, 1.0, 1.0
    solved = np.linalg.lstsq(a, np.array([0, 0, 0, 0, 0, -load], dtype=float), rcond=None)[0]
    t, a_k, u_k = torsion_wall(400.0, 800.0, 35.0, 8.0, 25.0)
    mitre = box(0, 0, 400.0, 800.0).buffer(-t / 2.0, join_style=2)
    return {"tie": tie, "tieLeastSquares": float(abs(solved[0])), "strut": strut, "strutLeastSquares": float(abs(solved[1])), "a_k": a_k, "a_kShapely": mitre.area, "u_k": u_k, "u_kShapely": mitre.length}
