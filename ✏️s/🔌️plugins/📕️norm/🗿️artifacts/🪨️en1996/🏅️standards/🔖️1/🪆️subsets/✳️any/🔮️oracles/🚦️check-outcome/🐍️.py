"""🚦️ Independent EN 1996 document oracle for the check-outcome case: a committed snapshot (JSON on stdin or a path) to the utilisation of the wall checks (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the wall section modulus against sectionproperties, the wind moment against PyNite, the confined bearing factor against scipy). It wraps the compression, mid-height, slenderness and simplified-method rows of `../⚖️compliance/🐍️.py` (family ids `en1996.<family>.<wall>`, the worst row over the load cases as the shared step layer reads it) and adds the fire thickness, chase, movement joint, opening, lintel, bar layout and reinforced-masonry families of the detailing checks and the floor diaphragm classification (PyNite shear-beam analogy), and per load case the declared eccentricity 6.1.2.2, in-plane shear/sliding 6.2 under wind and earth pressure, lateral flexure 6.3 and the concentrated load 6.1.3 with its area rule, written from EN 1996-1-1 6.1.3, 6.2, 6.3 and EN 1990 6.10 with the base module's keyed tables. SI throughout."""

import importlib.util
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
BASE_PATH = HERE.parent / "⚖️compliance" / "🐍️.py"
K_E = {"en": 1000.0, "de": 950.0}
RIGID_RATIO = 1.10
JOINT_SPACING_M = {"clay": 12.0, "calcium_silicate": 8.0, "aerated": 6.0, "concrete": 6.0}
MATERIALS = {"clay": "clay", "calciumsilicate": "calcium_silicate", "aerated": "aerated", "concrete": "concrete"}


def base():
    spec = importlib.util.spec_from_file_location("en1996_compliance", BASE_PATH)
    module = importlib.util.module_from_spec(spec)
    sys.modules["en1996_compliance"] = module
    spec.loader.exec_module(module)
    return module


def effects(m, factors, wall, lc, exceptional):
    """⚖️ EN 1990 6.10 vertical stations and the lateral action of the three combinations (max N with imposed or snow leading, min N with wind and earth pressure leading)."""
    t, h, length = wall["thicknessM"], wall["heightM"], wall["lengthM"]
    open_volume = sum(o["widthM"] * o["heightM"] * t for o in wall["openings"])
    g_self = wall["densityKgM3"] * 9.81 * max(t * length * h - open_volume, 0.0)
    g = max(lc["gKSlabN"], 0.0) + sum(max(c["forceN"], 0.0) for c in lc["concentrated"])
    area = lc["tributaryAreaM2"]
    q_imp, q_snow = max(lc["qKImposedPa"] * area, 0.0), max(lc["qKSnowPa"] * area, 0.0)
    w_k = max(lc["qPWindPa"] * abs(lc["cPe"]), 0.0)
    v_k = 0.5 * w_k * h * length + max(lc["hKEarthN"], 0.0)
    yg, yq = factors.gamma_g_sup(), factors.gamma_q()
    psi0, psi1, psi2 = factors.psi(str(lc["imposedCategory"]).upper())
    psi_snow, psi_wind = factors.psi("snow")[0], factors.psi("wind")[0]
    sit = str(lc["designSituation"]).lower()
    lead = psi1 if sit == "accidental" else psi2 if sit == "seismic" else psi1 if exceptional else 1.0
    rows = []
    stations = lambda q: (yg * g + q, yg * (g + 0.5 * g_self) + q, yg * (g + g_self) + q)
    rows.append((stations(yq * (lead * q_imp + psi_snow * q_snow)), yq * v_k, yq * w_k))
    if q_snow > 0:
        rows.append((stations(yq * (lead * q_snow + psi0 * q_imp)), yq * psi_wind * v_k, yq * psi_wind * w_k))
    rows.append(((g, g + 0.5 * g_self, g + g_self), yq * v_k, yq * w_k))
    return rows, g_self, yq


def wall_rows(m, doc, wall, annex, cls):
    rows, wid = [], wall["id"]
    t, h, length = wall["thicknessM"], wall["heightM"], wall["lengthM"]
    material = {"clay": "clay", "calciumsilicate": "calcium_silicate", "aerated": "aerated", "concrete": "concrete"}[m.norm(wall["unitMaterial"])]
    group = int(str(wall["unitGroup"])[-1])
    mortar = {"generalpurpose": "gp", "thinlayer": "tl", "lightweight": "lw"}[m.norm(wall["mortarType"])]
    fm = (wall["mortarStrengthPa"] or 0) / 1e6
    fb_n = wall["fBPa"] / 1e6 * m.delta(wall["unitHeightM"], min(wall["unitWidthM"], wall["unitLengthM"]))
    fk = m.f_k_mpa(annex, material, group, mortar, fb_n, fm) * 1e6
    l_ef = max(length - sum(o["widthM"] for o in wall["openings"]), 1e-3)
    area = l_ef * t
    z = l_ef * t * t / 6
    for lc in wall["loadCases"]:
        sit = m.norm(lc["designSituation"])
        exceptional = sit in ("accidental", "seismic") or m.norm(doc.get("designSituation")) == "accidental"
        reduced = (sit if sit != "persistent" else m.norm(doc.get("designSituation"))) in ("transient", "accidental", "seismic")
        gm = m.gamma_m(annex, cls, reduced)
        fd = fk / gm
        rows.append((f"en1996.6.1.2.eccentricity.{wid}.{lc['id']}", max(abs(wall["eccentricityBottomM"]), abs(wall["eccentricityTopM"])) / max(0.2 * t, 1e-9)))
        factors = m.FACTORS.ActionFactors(annex, reliability_class=int(str(doc["reliabilityClass"])[-1]), situation="accidental" if exceptional else "persistent")
        combos, g_self, yq = effects(m, factors, wall, lc, exceptional)
        n_t, n_m, n_b = max(combos, key=lambda c: c[0][2])[0]
        shear = max(combos, key=lambda c: c[1] / max(c[0][1], 1.0))
        (_, n_mid, _), v_ed, _ = shear
        v_ed += abs(wall.get("inPlaneShearN") or 0.0)
        e = (v_ed * h + abs(wall.get("inPlaneMomentNm") or 0.0)) / max(n_mid, 1.0)
        l_c = min(max(3 * (l_ef / 2 - e), 1e-3), l_ef)
        sigma = n_mid / max(l_c * t, 1e-9)
        fvk = min(m.fvk0_mpa(annex, material, mortar, fm) * 1e6 + 0.4 * max(sigma, 0), m.fvlt_mpa(annex, group, fb_n, max(sigma, 0) / 1e6) * 1e6)
        v_rd = min(fvk / gm * l_c * t, max(wall["mu"] * n_mid / gm, 1e-9))
        rows.append((f"en1996.6.2.shear.{wid}.{lc['id']}", v_ed / max(v_rd, 1e-9)))
        w_ed = max(c[2] for c in combos)
        m_ed = w_ed * h * h / 8 * l_ef + yq * max(lc["hKEarthN"], 0.0) * h * 2 / (9 * math.sqrt(3))
        if m_ed > 0:
            n_min = min(c[0][1] for c in combos)
            sigma_d = min(max(n_min / max(area, 1e-9), 0.0), 0.2 * fd)
            fxd1 = m.fxk_mpa(1, fm, material) * 1e6 / gm
            m_reinf = wall["asVerticalM2"] * wall["fYdPa"] * 0.9 * t if wall["asVerticalM2"] > 0 and wall["fYdPa"] > 0 else 0.0
            rows.append((f"en1996.6.3.flexure.{wid}.{lc['id']}", m_ed / max((fxd1 + sigma_d) * z + m_reinf, 1e-9)))
        for c in lc["concentrated"]:
            ab = c["bearingAreaM2"] if c["bearingAreaM2"] > 0 else c["bearingLengthM"] * t
            a_ef = min(c["bearingLengthM"] + h / math.sqrt(3), l_ef) * t
            beta = m.beta_c(max(c["distanceA1M"], 0.0), h, ab, a_ef)
            rows.append((f"en1996.6.1.3.concentrated.{wid}.{lc['id']}.{c['id']}", factors.gamma_g_sup() * c["forceN"] / max(beta * fd * ab, 1e-9)))
            rows.append((f"en1996.6.1.3.area.{wid}.{lc['id']}.{c['id']}", ab / max(0.45 * a_ef, 1e-9)))
    return rows


def uncovered(x0, x1, start, end):
    """🪟️ Length of the opening [x0, x1] outside the lintel clear span [start, end]."""
    return max(min(start, x1) - x0, 0.0) + max(x1 - max(end, x0), 0.0)


def detail_rows(m, doc, annex):
    """🪚️ Fire thickness (EN 1996-1-2 tabulated, registry rows unverified), chases and movement joints (8.6, EN 1996-2 2.3), openings and their lintels (5.4, 5.5), reinforced masonry ratios (6.6) and bar layout (8.2), each from the wall rows of the snapshot."""
    rows, walls = [], {w["id"]: w for w in doc["walls"]}
    for wall in doc["walls"]:
        wid, t, h, length = wall["id"], wall["thicknessM"], wall["heightM"], wall["lengthM"]
        material = MATERIALS[m.norm(wall["unitMaterial"])]
        if wall["fireReiMin"] > 0:
            rows.append((f"en1996.1-2.fire.{wid}", m.fire_mm(wall["fireReiMin"], material) / t))
        for o in wall["openings"]:
            rows.append((f"en1996.5.4.opening.{wid}.{o['id']}", (o["sillHeightM"] + o["heightM"]) / h))
        members = [("openings", o["id"], o["positionM"], o["positionM"], o["widthM"]) for o in wall["openings"]]
        members += [("piers", p["id"], p["positionM"], p["positionM"], p["widthM"]) for p in wall["piers"]]
        members += [("lintels", x["id"], x["positionM"], x["positionM"] - x["bearingLengthM"], x["spanM"] + 2 * x["bearingLengthM"]) for x in wall["lintels"]]
        members += [("confined", c["id"], c["positionM"], c["positionM"] - 0.5 * c["widthM"], c["widthM"]) for c in wall["confined"]]
        for coll, mid, _, start, width in members:
            rows.append((f"en1996.5.4.extent.{wid}.{coll}.{mid}", 1 - start / length if start < 0 else (start + width) / length))
        for o in wall["openings"]:
            if not wall["lintels"]:
                continue
            x0, x1 = o["positionM"], o["positionM"] + o["widthM"]
            best = min(wall["lintels"], key=lambda x: (uncovered(x0, x1, x["positionM"], x["positionM"] + x["spanM"]), -x["spanM"]))
            gap = uncovered(x0, x1, best["positionM"], best["positionM"] + best["spanM"])
            rows.append((f"en1996.5.5.lintel-cover.{wid}.{o['id']}", o["widthM"] / max(best["spanM"], 1e-9) if gap <= 1e-9 else 1 + gap / max(o["widthM"], 1e-9)))
        for bar in wall["bars"]:
            rows.append((f"en1996.8.2.bar-layout.{wid}.{bar['id']}", max(bar["diameterM"] / max(bar["spacingM"], 1e-9), bar["lengthM"] / max(max(length, h), 1e-9))))
        if wall["reinforced"] and wall["fYdPa"] > 0:
            l_ef = max(length - sum(o["widthM"] for o in wall["openings"]), 1e-3)
            area, have = l_ef * t, max(wall["asVerticalM2"], 0.0) + max(wall["asHorizontalM2"], 0.0)
            rows.append((f"en1996.6.6.reinforced.{wid}", max(0.0005 * area / max(have, 1e-12), have / max(0.04 * area, 1e-12))))
        stops = sorted(min(max(j["positionM"], 0.0), length) for j in doc.get("movementJoints", []) if j["wallId"] == wid)
        if stops:
            panels = [b - a for a, b in zip([0.0] + stops, stops + [length])]
            rows.append((f"en1996.8.5.joint-spacing.{wid}", max(panels) / JOINT_SPACING_M[material]))
    def extent(chase):
        x0, y0 = chase["xM"], chase["yM"]
        if m.norm(chase["kind"]) == "horizontal":
            return x0, x0 + chase["lengthM"], y0, y0 + chase["widthM"]
        return x0, x0 + chase["widthM"], y0, y0 + chase["lengthM"]

    chases = [c for c in doc.get("chases", []) if c["wallId"] in walls]
    for chase in chases:
        wall = walls[chase["wallId"]]
        kind = m.norm(chase["kind"])
        x0, x1, y0, y1 = extent(chase)
        limits = m.chase_limits(kind, wall["thicknessM"], chase["lengthM"], y1, wall["heightM"])
        rows.append((f"en1996.8.6.chase-depth.{chase['id']}", chase["depthM"] / max(limits[0] if limits else 0.0, 1e-9)))
        if limits and limits[1] is not None:
            rows.append((f"en1996.8.6.chase-width.{chase['id']}", chase["widthM"] / max(limits[1], 1e-9)))
        rows.append((f"en1996.8.6.chase-extent.{chase['id']}", max(x1 / max(wall["lengthM"], 1e-9), y1 / max(wall["heightM"], 1e-9), 2.0 if x0 < 0 or y0 < 0 else 0.0)))
        near = []
        for other in chases:
            if other is chase or other["wallId"] != chase["wallId"] or (m.norm(other["kind"]) == "horizontal") != (kind == "horizontal"):
                continue
            o0, o1, _, _ = extent(other)
            need = m.chase_gap(kind, m.norm(other["kind"]), max(chase["widthM"], other["widthM"]), max(chase["lengthM"], other["lengthM"]))
            near.append(need / max(max(o0 - x1, x0 - o1), 1e-9))
        for o in wall["openings"]:
            need = m.chase_opening_gap(kind, chase["widthM"])
            near.append(need / max(max(o["positionM"] - x1, x0 - (o["positionM"] + o["widthM"])), 1e-9))
        if near:
            rows.append((f"en1996.8.6.chase-distance.{chase['id']}", max(near)))
    for wid, wall in walls.items():
        widths = sum(c["widthM"] for c in chases if c["wallId"] == wid and m.norm(c["kind"]) != "horizontal")
        if any(c["wallId"] == wid and m.norm(c["kind"]) != "horizontal" for c in chases):
            rows.append((f"en1996.8.6.chase-cumulative.{wid}", widths / max(m.T["chase"]["widthRatio"] * wall["lengthM"], 1e-9)))
    for joint in doc.get("movementJoints", []):
        wall = walls.get(joint["wallId"])
        if wall is not None and (joint["positionM"] <= 0 or joint["positionM"] >= wall["lengthM"]):
            rows.append((f"en1996.8.5.joint-position.{joint['id']}", max(abs(joint["positionM"]), wall["lengthM"] - joint["positionM"], 1e-9) / wall["lengthM"]))
    return rows


def shear_beam(span, rigidity, springs, force=1.0, sub=20):
    """🥞️ PyNite: the diaphragm as a shear beam is the axial bar of the same rigidity under the same load, so the uniformly loaded bar on spring supports at the wall lines gives its in-plane displacements; returns the largest nodal displacement."""
    from Pynite import FEModel3D

    xs = sorted({span[0], span[1], *[s for s, _ in springs]})
    points = [a + (b - a) * i / sub for a, b in zip(xs, xs[1:]) for i in range(sub)] + [xs[-1]]
    model = FEModel3D()
    model.add_material("M", 1.0, 0.4, 0.3, 0.0)
    model.add_section("S", rigidity, 1.0, 1.0, 1.0)
    for i, x in enumerate(points):
        model.add_node(f"N{i}", x, 0, 0)
        model.def_support(f"N{i}", False, True, True, True, True, True)
    q = force / (span[1] - span[0])
    for i in range(len(points) - 1):
        model.add_member(f"B{i}", f"N{i}", f"N{i + 1}", "M", "S")
        model.add_member_dist_load(f"B{i}", "Fx", q, q)
    for s, k in springs:
        model.def_support_spring(f"N{min(range(len(points)), key=lambda i: abs(points[i] - s))}", "DX", k)
    model.analyze()
    return max(model.nodes[f"N{i}"].DX["Combo 1"] for i in range(len(points)))


def diaphragm_rows(m, doc, annex):
    """🥞️ Rigid or flexible floor diaphragm (EN 1998-1 4.3.1(4)) per plan direction: the displacement ratio of the PyNite shear-beam analogy against 1.10. The shear distribution that follows the classification is the bracing case M-21."""
    g_t = min((w["diaphragmStiffnessNPerM"] for w in doc["walls"] if w["diaphragmStiffnessNPerM"] > 0), default=None)
    if g_t is None:
        return []
    ends = {}
    for w in doc["walls"]:
        a = math.radians(w["planAngleDeg"])
        ends[w["id"]] = ((w["planXM"], w["planYM"]), (w["planXM"] + w["lengthM"] * math.cos(a), w["planYM"] + w["lengthM"] * math.sin(a)))
    xs = [p[0] for pair in ends.values() for p in pair]
    ys = [p[1] for pair in ends.values() for p in pair]
    rows = []
    for axis in ("x", "y"):
        bracing = [w for w in doc["walls"] if m.norm(w["wallType"]) != "nonloadbearing" and (("x" if abs(ends[w["id"]][1][0] - ends[w["id"]][0][0]) >= abs(ends[w["id"]][1][1] - ends[w["id"]][0][1]) else "y") == axis)]
        if len(bracing) < 2:
            continue
        span, along = ((min(ys), max(ys)), (min(xs), max(xs))) if axis == "x" else ((min(xs), max(xs)), (min(ys), max(ys)))
        depth = max(along[1] - along[0], 1e-3)
        springs = []
        for w in bracing:
            material = MATERIALS[m.norm(w["unitMaterial"])]
            group = int(str(w["unitGroup"])[-1])
            mortar = {"generalpurpose": "gp", "thinlayer": "tl", "lightweight": "lw"}[m.norm(w["mortarType"])]
            fb_n = w["fBPa"] / 1e6 * m.delta(w["unitHeightM"], min(w["unitWidthM"], w["unitLengthM"]))
            e = K_E[annex] * m.f_k_mpa(annex, material, group, mortar, fb_n, (w["mortarStrengthPa"] or 0) / 1e6) * 1e6
            l, t, h = max(w["lengthM"], 1e-3), max(w["thicknessM"], 1e-3), max(w["heightM"], 1e-3)
            a, b = ends[w["id"]]
            springs.append((0.5 * (a[1] + b[1]) if axis == "x" else 0.5 * (a[0] + b[0]), 1 / (h ** 3 / (e * t * l ** 3) + 1.2 * h / (0.4 * e * t * l))))
        total_k = sum(k for _, k in springs)
        ratio = 1.0 if span[1] - span[0] < 1e-6 else shear_beam(span, g_t * depth, springs) * total_k
        rows.append((f"en1996.9.3.diaphragm.{axis}", ratio / RIGID_RATIO))
    return rows


def evaluate(doc):
    m = base()
    annex = m.norm(doc.get("annex", "de"))
    cls = int(str(doc.get("masonryClass", "class1"))[-1])
    families = {"compression": "en1996.6.1.2.compression.", "mid": "en1996.6.1.2.compression.mid.", "slenderness": "en1996.5.5.1.slenderness.", "simplified": "en1996.3.simplified."}
    rows = []
    for wall_id, values in m.snapshot(doc).items():
        for key, prefix in families.items():
            if key in values:
                rows.append({"id": prefix + wall_id, "utilization": values[key]})
    for wall in doc["walls"]:
        rows += [{"id": k, "utilization": v} for k, v in wall_rows(m, doc, wall, annex, cls)]
    rows += [{"id": k, "utilization": v} for k, v in detail_rows(m, doc, annex) + diaphragm_rows(m, doc, annex)]
    return rows


def self_check():
    """🧪 Z of the net wall section against sectionproperties, the wind midspan moment against PyNite, the bearing factor bound against scipy.optimize."""
    from Pynite import FEModel3D
    from scipy.optimize import minimize_scalar
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import rectangular_section

    t, length, h, w = 0.365, 4.2, 2.75, 1.2e3
    geometry = rectangular_section(d=t, b=length)
    geometry.create_mesh(mesh_sizes=[0.01])
    section = Section(geometry)
    section.calculate_geometric_properties()
    z_ref = section.get_z()[0]
    model = FEModel3D()
    model.add_material("W", 1e9, 4e8, 0.2, 1800)
    for name, x in (("A", 0.0), ("B", h)):
        model.add_node(name, x, 0, 0)
    model.add_section("Sec", 0.01, 1e-4, 1e-4, 1e-6)
    model.add_member("M", "A", "B", "W", "Sec")
    model.def_support("A", True, True, True, True, False, False)
    model.def_support("B", False, True, True, False, False, False)
    model.add_member_dist_load("M", "Fy", -w * length, -w * length)
    model.analyze()
    m_ref = max(abs(model.members["M"].max_moment("Mz")), abs(model.members["M"].min_moment("Mz")))
    beta_max = -minimize_scalar(lambda a: -min((1 + 0.3 * a / h) * (1.5 - 1.1 * 0.1), 1.25 + a / (2 * h), 1.5), bounds=(0, 3 * h), method="bounded").fun
    results = {"Z_ratio": (length * t * t / 6) / z_ref, "wind_moment_ratio": (w * length * h * h / 8) / m_ref, "beta_cap": beta_max}
    assert abs(results["Z_ratio"] - 1) < 1e-9 and abs(results["wind_moment_ratio"] - 1) < 1e-3 and abs(beta_max - 1.5) < 1e-6, results
    return results


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        print(json.dumps(self_check()))
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        text = open(paths[0], encoding="utf8").read() if paths else sys.stdin.read()
        print(json.dumps({"checks": evaluate(json.loads(text))}, ensure_ascii=False))
