#!/usr/bin/env python3
"""🌍️ EN 1997 compliance oracle (stdlib only, run with `python3 -I`).

Modes
  --json        evaluates a snapshot read from stdin with the independent library `📚️geo.py` and prints the utilisation of every check it reproduces.
  --catalogue   evaluates the catalogue fixture read from stdin entry by entry with `📇️catalogue.py`.
  --reference   asserts the hand-check numbers of the specification (`📌️reference-values.py`) and prints them.
  (none)        prints the Annex D factors for phi = 30 degrees.

The slice search of the layered Bishop check mirrors the Rust circle grid (documented in the report); every other value has its own formulation.
"""
from __future__ import annotations

import importlib.util
import json
import math
import pathlib
import sys
from typing import Any


def _load(name: str, file: str):
    spec = importlib.util.spec_from_file_location(name, pathlib.Path(__file__).with_name(file))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


G = _load("en1997_geo", "📚️geo.py")
C = _load("en1997_catalogue", "📇️catalogue.py")
R = math.radians
PRESUMPTIVE_GK1_PA = {"gravel": 400000.0, "kies": 400000.0, "sand": 200000.0, "sandig": 200000.0, "silt": 150000.0, "schluff": 150000.0, "clay": 120000.0, "ton": 120000.0}  # unverified: no table in the specs


def effective_gamma_below(gamma, gamma_prime, embedment, gwl, width):
    zone = max(width, 0.1)
    if gwl >= embedment + zone:
        return gamma
    if gwl <= embedment:
        return max(gamma_prime, 1.0)
    wet = (embedment + zone - gwl) / zone
    return gamma * (1 - wet) + max(gamma_prime, 1.0) * wet


def is_cohesive(layer):
    t = layer.get("soilType", "").lower()
    return layer.get("cohesionUndrained", 0.0) > 1.0 or any(k in t for k in ("clay", "ton", "cohesive"))


def layer_at(layers, depth):
    for layer in layers:
        if layer["depthTop"] <= depth <= layer["depthBottom"]:
            return layer
    return layers[0] if layers else None


def phi_cpt(qc, sigma):
    return 17.6 + 11 * math.log10((qc / 1e5) / math.sqrt(sigma / 1e5))


def phi_spt(n, sigma):
    return math.sqrt(20 * n * min(math.sqrt(1e5 / sigma), 1.7)) + 20


def investigation_phi(layer, overburden):
    sigma = max(overburden, 50000.0)
    best = None
    if layer.get("cptQc", 0.0) > 0:
        best = phi_cpt(layer["cptQc"], sigma)
    if layer.get("sptN", 0.0) > 0:
        v = phi_spt(layer["sptN"], sigma)
        best = v if best is None or v < best - 1e-9 else best
    return best


def xi_ground(n):
    return C.lerp_table(C.XI_GROUND, max(n, 1))


def stress_sublayers(layers, width, length, embedment, q, gwl):
    """🏔️ Stress-strain settlement: Boussinesq under the centre, sublayers of at most 0.5 m, cut-off at sigma_z <= 0.2 sigma'_v0."""
    total = 0.0
    contrib = {}
    for layer in layers:
        top, bot = max(layer["depthTop"] - embedment, 0.0), max(layer["depthBottom"] - embedment, 0.0)
        if bot <= top + 1e-12:
            continue
        parts = max(math.ceil((bot - top) / 0.5), 1)
        dz = (bot - top) / parts
        e = max(layer["oedometricModulus"], 1.0)
        part_sum = 0.0
        for k in range(parts):
            zmid = top + (k + 0.5) * dz
            sz = G.sigma_z_rect_centre(q, width, max(length, width), max(zmid, 1e-3), 120)
            if sz <= 0.2 * effective_stress(layers, embedment + zmid, gwl):
                contrib[layer["id"]] = part_sum
                return total + part_sum, contrib
            part_sum += sz / e * dz
        contrib[layer["id"]] = part_sum
        total += part_sum
    return total, contrib


def effective_stress(layers, depth, gwl):
    sigma = 0.0
    for layer in layers:
        top, bot = min(layer["depthTop"], depth), min(layer["depthBottom"], depth)
        if bot <= top:
            continue
        dry = max(min(min(gwl, bot) - top, bot - top), 0.0)
        sigma += layer["gamma"] * dry + layer["gammaPrime"] * (bot - top - dry)
    return sigma


def centre_settlement_halfspace(p, b, l, nu, e, n=4000):
    """📉️ w = p(1-nu^2)/(pi E) * integral of the boundary distance R(theta) over the full angle (flexible rectangle, centre)."""
    total, h = 0.0, 2 * math.pi / n
    for i in range(n + 1):
        th = i * h
        c, s = abs(math.cos(th)), abs(math.sin(th))
        r = min((b / 2) / c if c > 1e-12 else 1e18, (l / 2) / s if s > 1e-12 else 1e18)
        total += (1 if i in (0, n) else 4 if i % 2 else 2) * r
    return p * (1 - nu * nu) / (math.pi * e) * total * h / 3


def elastic_equivalent(layers, b, embedment):
    """📐️ Equivalent half-space of the ground below a base over the depth 2B: (nu, E_m) with E_m = sum h / sum (h / E) and the thickness-weighted nu; None without ground in range."""
    limit = 2 * max(b, 0.1)
    sh = she = shn = 0.0
    for layer in layers:
        top = max(layer["depthTop"] - embedment, 0.0)
        if top >= limit:
            continue
        h = max(min(max(layer["depthBottom"] - embedment, top), limit) - top, 0.0)
        if h < 1e-9:
            continue
        e = max(layer["oedometricModulus"], 1.0)
        sh, she, shn = sh + h, she + h / e, shn + layer["poissonRatio"] * h
    if sh < 1e-9:
        return None
    return min(max(shn / sh, 0.0), 0.49), sh / she


def elastic_layered(layers, b, l, embedment, q):
    equivalent = elastic_equivalent(layers, b, embedment)
    if equivalent is None:
        return 0.0
    return centre_settlement_halfspace(q, max(b, 0.1), max(l, b), equivalent[0], equivalent[1])


def boussinesq_stress(q, b, l, px, py, z):
    """🏔️ Vertical stress at offset (px, py) from the centre of a rectangle and depth z: scipy.integrate.dblquad of the point-load kernel 3 P z^3 / (2 pi R^5) over the area (third party, independent of the corner solution)."""
    from scipy import integrate

    kernel = lambda y, x: 3.0 * q * z**3 / (2.0 * math.pi * ((x - px) ** 2 + (y - py) ** 2 + z * z) ** 2.5)
    return integrate.dblquad(kernel, -b / 2, b / 2, -l / 2, l / 2, epsabs=1e-3, epsrel=1e-9)[0]


def halfspace_displacement(q, b, l, px, py, nu, e):
    """📉️ Surface displacement at offset (px, py) outside a flexible rectangle: scipy.integrate.dblquad of the point-load kernel (1 - nu^2) P / (pi E r)."""
    from scipy import integrate

    kernel = lambda y, x: q * (1 - nu * nu) / (math.pi * e * math.hypot(x - px, y - py))
    return integrate.dblquad(kernel, -b / 2, b / 2, -l / 2, l / 2, epsabs=1e-12, epsrel=1e-9)[0]


def interaction_settlements(layers, footings, places, values):
    """🤝 Adds the influence of every neighbour to the isolated settlements: oedometric = Boussinesq stress over 10 sublayers per layer below the loaded base to twice its longer side, elastic = half-space displacement with the equivalent nu and E_m of the receiving footing; point in the loaded footing's frame (yaw about +Z)."""
    out = {"oed": list(values["oed"]), "el": list(values["el"])}
    for i, receiver in enumerate(footings):
        equivalent = elastic_equivalent(layers, receiver["width"], receiver["embedment"])
        for j, loaded in enumerate(footings):
            if i == j:
                continue
            at, frm = places[receiver["id"]], places[loaded["id"]]
            dx, dy = at["x"] - frm["x"], at["y"] - frm["y"]
            yaw = frm.get("yaw", 0.0)
            px, py = dx * math.cos(yaw) + dy * math.sin(yaw), -dx * math.sin(yaw) + dy * math.cos(yaw)
            b, l, d = loaded["width"], loaded["length"], loaded["embedment"]
            q = max([(lc.get("verticalPermanent", 0.0) + lc.get("verticalVariable", 0.0)) / max(b * l, 1e-9) for lc in loaded.get("loadCases", [])] + [0.0])
            limit = 2 * max(b, l)
            for layer in layers:
                top, bot = max(layer["depthTop"], d), min(layer["depthBottom"], d + limit)
                if bot <= top or layer["oedometricModulus"] <= 0.0:
                    continue
                h = (bot - top) / 10
                out["oed"][i] += sum(boussinesq_stress(q, b, l, px, py, max(top + (k + 0.5) * h - d, 1e-3)) * h / layer["oedometricModulus"] for k in range(10))
            if equivalent is not None:
                out["el"][i] += halfspace_displacement(q, b, l, px, py, equivalent[0], equivalent[1])
    return out


def pile_factors(pile_type, n, f, approach):
    x3, x4 = xi_ground(n)
    if G.approach_key(approach) != "da1c2":
        return f["gamma_b"], f["gamma_s"], x3, x4
    key = (pile_type or "").lower()
    if key in ("bored", "bohr", "bohrpfahl"):
        return 1.6, 1.3, x3, x4
    if key in ("cfa", "sob", "schnecke"):
        return 1.45, 1.3, x3, x4
    return 1.3, 1.3, x3, x4


def reached_depth(doc):
    """🔎️ Depth below the ground reached by the deepest investigation point; the declared depth only without a point."""
    return max([point["depth"] for point in doc.get("investigations", [])] or [doc.get("investigationDepth", 0.0)])


def investigation_extent_checks(doc, annex):
    """🔭️ GI-01 spacing and pile depth from the committed rows (registry spacing 20-40 m for GK1-2, 10-25 m for GK3; pile: length + max(5 m, 3 D_b))."""
    out = []
    points = doc.get("investigations", [])
    if len(points) > 1:
        row = "gc3" if doc.get("geotechnicalCategory", 2) >= 3 else "gc12"
        hi = C.ndp_number(f"ndp.en1997.2.4.2.spacing.{row}.max", annex)
        nearest = [min(math.hypot(a["positionX"] - b["positionX"], a["positionY"] - b["positionY"]) for b in points if b is not a) for a in points]
        out.append({"id": "en1997.gi-01.spacing", "utilization": max(nearest) / hi})
    for pile in doc.get("piles", []):
        out.append({"id": f"en1997.gi-01.pile.{pile['id']}", "utilization": minimum_util(reached_depth(doc), pile["length"] + max(5.0, 3 * pile["diameter"]))})
    return out


def minimum_util(computed, minimum):
    if minimum <= 0.0:
        return 0.0
    ratio = min(minimum / computed, 1.0e6) if computed > 0 else 1.0e6
    return ratio if computed >= minimum else max(ratio, 1.0)


def entity_checks(doc: dict[str, Any], annex: str, approach: str) -> list[dict[str, Any]]:
    """🌐️ The catalogue cases that the snapshot entities (anchors, embedded walls, pile groups, laboratory series, layer water) raise, evaluated by the independent catalogue implementation."""
    out: list[dict[str, Any]] = []
    text = {"approach": approach, "situation": doc.get("designSituation") or "persistent"}
    layers = {layer["id"]: layer for layer in doc.get("layers", [])}
    piles = {pile["id"]: pile for pile in doc.get("piles", [])}

    def util(demand, capacity):
        if capacity <= 1e-9:
            return 0.0 if demand <= 1e-9 else 1.0e6
        return demand / capacity

    for a in doc.get("anchors", []):
        num = {f"ra{i + 1}": t["pullOutResistance"] for i, t in enumerate(a["tests"])}
        num.update(xi_a=a["testFactor"], a_d=a["designForce"])
        out.append({"id": f"en1997.anc-01.pullout.{a['id']}", "utilization": C.evaluate("ANC-01", annex, num, text)["pullout"]})
        empirical = C.evaluate("ANC-02", annex, dict(d=a["fixedDiameter"], l_f=a["fixedLength"], tau_f=a["bondStress"], a_d=a["designForce"]), text)
        out.append({"id": f"en1997.anc-02.pullout.{a['id']}", "utilization": empirical["pullout"]})
        tendon = C.evaluate("ANC-04", annex, dict(a_d=a["designForce"], p_t_k=a["tendonYieldForce"], l_min=a["minFreeLength"], l_free=a["freeLength"]), {**text, "tendon": a["tendon"]})
        out.append({"id": f"en1997.anc-04.tendon.{a['id']}", "utilization": tendon["tendon"]})
        out.append({"id": f"en1997.anc-04.free_length.{a['id']}", "utilization": tendon["free_length"]})

    for w in doc.get("embeddedWalls", []):
        num = dict(h=w["retainedHeight"], gamma=w["backfillGamma"], phi_deg=w["phiDeg"], delta_a_deg=w["deltaActiveDeg"], delta_p_deg=w["deltaPassiveDeg"], q=w["surcharge"])
        kind = "cantilever" if w["kind"] == "cantilever" else "free_earth"
        if kind != "cantilever":
            num["prop_depth"] = w["propDepth"]
        if w["undrainedStrength"] > 0:
            num.update(cu_k=w["undrainedStrength"], nc=5.14)
        result = C.evaluate("RW-07", annex, num, {**text, "kind": kind})
        out.append({"id": f"en1997.rw-07.embedment.{w['id']}", "utilization": util(result["t_design"], w["embedment"])})
        head = w["retainedHeight"] - doc.get("groundwaterLevel", 0.0)
        soil = layer_at(doc.get("layers", []), w["retainedHeight"] + 0.5 * w["embedment"])
        gwl = doc.get("groundwaterLevel", 0.0)
        if w["embedment"] > 1e-6 and gwl < w["retainedHeight"] + w["embedment"] and soil is not None:
            gamma_w = 10000.0 if annex.lower() == "de" else 9810.0
            wet = C.evaluate("RW-12", annex, {**num, "gamma_sat": soil["gammaPrime"] + gamma_w, "z_w": max(gwl, 0.0), "z_wf": max(w["retainedHeight"], gwl), "t_provided": w["embedment"]}, {**text, "kind": kind})
            out.append({"id": f"en1997.rw-12.embedment.{w['id']}", "utilization": wet["embedment"]})
        if head > 1e-6 and w["embedment"] > 1e-6 and soil is not None:
            piping = C.evaluate("HYD-02", annex, dict(delta_h=head, gamma_prime=soil["gammaPrime"], d_embed=w["embedment"]), text)
            out.append({"id": f"en1997.hyd-02.piping.{w['id']}", "utilization": piping["piping"]})
            flow = C.evaluate("HYD-03", annex, dict(delta_h=head, gamma_prime=soil["gammaPrime"], d_embed=w["embedment"], t_layer=max(doc.get("investigationDepth", 0.0) - w["retainedHeight"], 2 * w["embedment"]), cells=8), text)
            out.append({"id": f"en1997.hyd-03.heave_prism.{w['id']}", "utilization": flow["heave_prism"]})
            out.append({"id": f"en1997.hyd-03.piping.{w['id']}", "utilization": flow["piping"]})

    for g in doc.get("pileGroups", []):
        pile = piles.get(g["pileId"])
        if pile is None:
            continue
        d, s_, m, n = pile["diameter"], g["spacing"], g["rows"], g["columns"]
        num = dict(m=m, n=n, s=s_, d=d, v=g["capVertical"], mx=g["capMomentX"], my=g["capMomentY"], r_c_d=g["singleResistance"],
                   block_b=(n - 1) * s_ + d, block_l=(m - 1) * s_ + d, cu_h_sum=g["shaftCohesionSum"], cu_b=g["baseCohesion"])
        result = C.evaluate("PILE-06", annex, num, text)
        out.append({"id": f"en1997.pile-06.max_pile.{g['id']}", "utilization": result["max_pile"]})
        if "block" in result:
            out.append({"id": f"en1997.pile-06.block.{g['id']}", "utilization": result["block"]})

    for t in doc.get("labTests", []):
        layer = layers.get(t["layerId"])
        if t["kind"] == "oedometer" or layer is None:
            continue
        pts = t["points"]
        if t["kind"] == "triaxial":
            num = {**{f"p{i + 1}": p["normalStress"] for i, p in enumerate(pts)}, **{f"q{i + 1}": p["shearStress"] for i, p in enumerate(pts)}}
            result, entry = C.evaluate("GI-11", annex, num, text), "gi-11"
        else:
            num = {**{f"sigma{i + 1}": p["normalStress"] for i, p in enumerate(pts)}, **{f"tau{i + 1}": p["shearStress"] for i, p in enumerate(pts)}}
            result, entry = C.evaluate("GI-12", annex, num, text), "gi-12"
        out.append({"id": f"en1997.{entry}.phi_layer.{t['id']}", "utilization": util(layer["phiPrimeDeg"], result["phi_deg"])})
        out.append({"id": f"en1997.{entry}.c_layer.{t['id']}", "utilization": util(layer["cohesionEffective"], max(result["c"], 0.0))})
        bound = C.evaluate("GI-17", annex, {**num, "phi_assumed_deg": layer["phiPrimeDeg"], "c_assumed": layer["cohesionEffective"]}, {**text, "kind": "triaxial" if t["kind"] == "triaxial" else "shear"})
        out.append({"id": f"en1997.gi-17.phi_assumed.{t['id']}", "utilization": bound["phi_assumed"]})
        out.append({"id": f"en1997.gi-17.c_assumed.{t['id']}", "utilization": bound["c_assumed"]})

    for w in doc.get("layerWaters", []):
        layer = layers.get(w["layerId"])
        if layer is None:
            continue
        result = C.evaluate("HYD-01", annex, dict(delta_h=w["headDifference"], l_flow=w["flowLength"], gamma_prime=layer["gammaPrime"]), text)
        out.append({"id": f"en1997.hyd-01.heave.{w['id']}", "utilization": result["heave"]})
    out.extend(lateral_and_seismic_checks(doc, annex, approach, text["situation"]))
    return out


LATERAL_LIMIT_M = 0.010
SEISMIC_GAMMA = 1.0


def lateral_head_deflection(ei, k_h, d, length, head_load):
    """🪵️ Head deflection of a finite free-tip pile on a Winkler foundation: `scipy.integrate.solve_bvp` of EI·y'''' + k_h·D·y = 0 with y'' = 0, y''' = H/EI at the head and a free tip."""
    import numpy as np
    from scipy.integrate import solve_bvp

    ks = k_h * d
    x = np.linspace(0.0, length, 400)
    solution = solve_bvp(lambda _, s: np.vstack([s[1], s[2], s[3], -ks / ei * s[0]]), lambda a, b: np.array([a[2], a[3] - head_load / ei, b[2], b[3]]), x, np.zeros((4, x.size)), tol=1e-12, max_nodes=200000)
    assert solution.status == 0, "solve_bvp did not converge"
    return float(solution.sol(0.0)[0])


def winkler_critical_load(ei, length, line_stiffness, nodes=800):
    """🪵️ Elastic critical load of a pinned beam on a Winkler foundation: smallest generalised eigenvalue of the finite-difference stiffness matrix (`scipy.linalg.eigh`)."""
    import numpy as np
    from scipy.linalg import eigh

    h, n = length / nodes, nodes - 1
    second = (np.diag(-2.0 * np.ones(n)) + np.diag(np.ones(n - 1), 1) + np.diag(np.ones(n - 1), -1)) / h**2
    geometric = (np.diag(2.0 * np.ones(n)) - np.diag(np.ones(n - 1), 1) - np.diag(np.ones(n - 1), -1)) / h
    return float(eigh(ei * (second @ second.T) * h + line_stiffness * h * np.eye(n), geometric, eigvals_only=True, subset_by_index=[0, 0])[0])


def seismic_wedge_thrust(gamma, h, phi_deg, delta_deg, kh, vertical):
    """🌋 Mononobe-Okabe thrust by maximising the sliding-wedge force polygon over the plane angle with `scipy.optimize.minimize_scalar` (vertical wall, horizontal backfill), independent of the closed form."""
    from scipy.optimize import minimize_scalar

    phi, delta = math.radians(phi_deg), math.radians(delta_deg)

    def thrust(rho):
        a, b = -math.sin(rho) + math.tan(phi) * math.cos(rho), math.cos(rho) + math.tan(phi) * math.sin(rho)
        return 0.5 * gamma * h * h / math.tan(rho) * (kh * b - vertical * a) / (math.cos(delta) * b - math.sin(delta) * a)

    best = minimize_scalar(lambda rho: -thrust(rho), bounds=(math.radians(1.0), math.radians(89.0)), method="bounded", options={"xatol": 1e-12})
    return -float(best.fun), math.degrees(best.x)


def seismic_wall_sliding(wall):
    """🌋 Base sliding of a wall under the pseudo-static thrust of the more onerous vertical sign (inertia of the wall k_h·W included), partial factor 1.0 (unverified); `None` where theta exceeds phi and no limit equilibrium exists."""
    phi = min(max(wall["backfillPhiDeg"], 0.0), 50.0)
    delta = min(max(wall["wallFrictionDeg"], 0.0), phi)
    kh, kv, h = wall["seismicKh"], wall.get("seismicKv", 0.0), wall["height"]
    weight = wall["verticalPermanent"] + wall["stemThickness"] * (h + wall["baseWidth"]) * max(wall["concreteGamma"], 1000.0)
    best = None
    for sign in (1.0, -1.0):
        vertical = 1.0 - sign * kv
        if math.radians(phi) - math.atan(kh / vertical) < 0.0:
            return None
        thrust, _ = seismic_wedge_thrust(wall["backfillGamma"], h, phi, delta, kh, vertical)
        if best is None or thrust > best[0]:
            best = (thrust, vertical)
    thrust, vertical = best
    demand = thrust * math.cos(math.radians(delta)) + kh * weight
    resistance = (weight * vertical + thrust * math.sin(math.radians(delta))) * math.tan(math.radians(phi))
    return demand / (resistance / SEISMIC_GAMMA)


def lateral_and_seismic_checks(doc, annex, approach, situation):
    """🌋 PILE-04 lateral deflection, PILE-07 buckling and the seismic wall sliding of the entities, from third-party numerics (scipy boundary value solver, scipy eigenvalue solver, scipy scalar optimiser)."""
    out = []
    for pile in doc.get("piles", []):
        ei, k_h = pile.get("bendingStiffnessEi", 0.0), pile.get("horizontalSubgradeModulus", 0.0)
        d, length, head = pile["diameter"], pile["length"], pile.get("lateralHeadLoad", 0.0)
        if ei > 0 and k_h > 0 and head > 0:
            out.append({"id": f"en1997.pile-04.deflection.{pile['id']}", "utilization": lateral_head_deflection(ei, k_h, d, length, head) / LATERAL_LIMIT_M})
        if ei > 0 and length > 0:
            f = G.factors(annex, approach, situation)
            n_ed = (f["gamma_g"] * pile.get("compressionPermanent", 0.0) + f["gamma_q"] * pile.get("compressionVariable", 0.0)) / max(pile.get("count", 1), 1)
            if n_ed > 0:
                out.append({"id": f"en1997.pile-07.buckling.{pile['id']}", "utilization": n_ed / winkler_critical_load(ei, length, k_h * d)})
    for wall in doc.get("retainingWalls", []):
        if wall.get("seismicKh", 0.0) > 0 and wall["height"] > 0:
            utilization = seismic_wall_sliding(wall)
            if utilization is not None:
                out.append({"id": f"en1998.5.7.3.seismic-sliding.{wall['id']}", "utilization": utilization})
    return out


def sls_pair_checks(doc, layers, gwl):
    """📏️ Angular distortion, relative rotation and tilt from the footing placements with the neighbours superposed (scipy.integrate.dblquad of the Boussinesq kernel); the settlement plane is fitted with numpy.linalg.lstsq (third party)."""
    import numpy as np

    places = {p["id"]: p for p in doc.get("placements", [])}
    footings = [f for f in doc.get("footings", []) if f["id"] in places and f["width"] > 1e-6 and f["length"] > 1e-6]
    if len(footings) < 2:
        return []
    values = {"oed": [], "el": []}
    for f in footings:
        b, l, d = f["width"], f["length"], f["embedment"]
        q = max([(lc.get("verticalPermanent", 0.0) + lc.get("verticalVariable", 0.0)) / max(b * l, 1e-9) for lc in f.get("loadCases", [])] + [0.0])
        values["oed"].append(stress_sublayers(layers, b, l, d, q, gwl)[0])
        values["el"].append(elastic_layered(layers, b, l, d, q))
    values = interaction_settlements(layers, footings, places, values)
    out = []
    beta_limit, tilt_limit = 0.002, 0.004
    xy = np.array([[places[f["id"]]["x"], places[f["id"]]["y"]] for f in footings])
    for method, s in values.items():
        s = np.array(s)
        plane = None
        if len(footings) >= 3:
            design = np.column_stack([np.ones(len(footings)), xy[:, 0], xy[:, 1]])
            if np.linalg.matrix_rank(design) == 3:
                plane = np.linalg.lstsq(design, s, rcond=None)[0]
        for i in range(len(footings)):
            for j in range(i + 1, len(footings)):
                d_xy = xy[j] - xy[i]
                length = float(np.hypot(*d_xy))
                if length < 1e-6:
                    continue
                delta = float(s[j] - s[i])
                ids = f"{footings[i]['id']}.{footings[j]['id']}"
                out.append({"id": f"en1997.6.6.angular.{method}.{ids}", "utilization": abs(delta) / length / beta_limit})
                if plane is not None:
                    out.append({"id": f"en1997.6.6.rotation.{method}.{ids}", "utilization": abs(delta / length - float(plane[1] * d_xy[0] + plane[2] * d_xy[1]) / length) / beta_limit})
        if plane is not None:
            tilt = float(math.hypot(plane[1], plane[2]))
        else:
            tilt = abs(float(s[1] - s[0])) / float(np.hypot(*(xy[1] - xy[0])))
        out.append({"id": f"en1997.6.6.tilt.{method}", "utilization": tilt / tilt_limit})
    return out


def evaluate(doc: dict[str, Any]) -> dict[str, Any]:
    checks = []
    approach, annex = doc.get("designApproach", "da2"), doc.get("annex", "de")
    layers, gwl, gk = doc.get("layers", []), doc.get("groundwaterLevel", 0.0), doc.get("geotechnicalCategory", 2)
    situations = [doc.get("designSituation") or "persistent"]
    for footing in doc.get("footings", []):
        for lc in footing.get("loadCases", []):
            s = lc.get("designSituation") or doc.get("designSituation") or "persistent"
            if s not in situations:
                situations.append(s)

    for layer in layers:
        overburden = layer["gamma"] * max((layer["depthTop"] + layer["depthBottom"]) * 0.5, 0.5)
        sigma = max(overburden, 50000.0)
        if layer.get("cptQc", 0.0) > 0:
            checks.append({"id": f"en1997.2.phi.cpt.{layer['id']}", "utilization": layer["phiPrimeDeg"] / max(phi_cpt(layer["cptQc"], sigma), 1e-6)})
        if layer.get("sptN", 0.0) > 0:
            checks.append({"id": f"en1997.2.phi.spt.{layer['id']}", "utilization": layer["phiPrimeDeg"] / max(phi_spt(layer["sptN"], sigma), 1e-6)})
        derived = investigation_phi(layer, overburden)
        if derived is not None:
            checks.append({"id": f"en1997.2.phi.derived.{layer['id']}", "utilization": layer["phiPrimeDeg"] / max(derived, 1e-6)})

    for footing in doc.get("footings", []):
        layer = layer_at(layers, footing["embedment"]) or dict(id="default", soilType="sand", gamma=18000.0, gammaPrime=10000.0, phiPrimeDeg=30.0, cohesionEffective=0.0, cohesionUndrained=0.0, oedometricModulus=30e6, poissonRatio=0.3, depthTop=0.0, depthBottom=10.0)
        d, b, l = footing["embedment"], footing["width"], footing["length"]
        derived = investigation_phi(layer, layer["gamma"] * max(d, 0.5))
        phi = min(layer["phiPrimeDeg"], derived) if derived is not None else layer["phiPrimeDeg"]
        g_below = effective_gamma_below(layer["gamma"], layer["gammaPrime"], d, gwl, b)
        for lc in footing.get("loadCases", []):
            sit = lc.get("designSituation") or doc.get("designSituation") or "persistent"
            f = G.factors(annex, approach, sit)
            act = lambda g, q: f["gamma_g"] * lc.get(g, 0.0) + f["gamma_q"] * lc.get(q, 0.0)
            vd, hd, md = act("verticalPermanent", "verticalVariable"), act("horizontalPermanent", "horizontalVariable"), act("momentPermanent", "momentVariable")
            eb = md / max(vd, 1.0)
            short = G.situation_key(sit) in ("T", "A")
            undrained = is_cohesive(layer) and short and layer.get("cohesionUndrained", 0.0) > 0
            area = b * l
            if gk == 1:
                q_all = PRESUMPTIVE_GK1_PA.get(str(layer.get("soilType", "")).lower(), 180000.0) * (1.0 + 0.1 * min(d, 3.0))
                r = q_all * area / f["gamma_r_v"]
            elif undrained:
                bb, ll = G.eff_dims(b, l, eb, 0.0)
                cud = layer["cohesionUndrained"] / f["gamma_cu"]
                r = G.bearing_undrained(b, l, d, 0.0, vd, hd, 0.0, eb, 0.0, cud, g_below / f["gamma_gamma"] * d)["r_k"] / f["gamma_r_v"] if hd <= bb * ll * cud else 0.0
            else:
                phid = math.degrees(math.atan(math.tan(R(phi)) / f["gamma_phi"]))
                try:
                    r = G.bearing_drained(b, l, d, footing.get("baseInclinationDeg", 0.0), vd, hd, 0.0, eb, 0.0, phid, layer.get("cohesionEffective", 0.0) / f["gamma_c"], g_below / f["gamma_gamma"], layer["gamma"] / f["gamma_gamma"] * d)["r_k"] / f["gamma_r_v"]
                except ValueError:
                    r = 0.0
            checks.append({"id": f"en1997.6.5.bearing.{footing['id']}.{lc['id']}", "utilization": vd / max(r, 1.0)})
            if undrained:
                rh = area * layer["cohesionUndrained"] / f["gamma_cu"] / f["gamma_r_h"]
            else:
                base = layer.get("cohesionEffective", 0.0) / f["gamma_c"] * area + lc.get("verticalPermanent", 0.0) * math.tan(R(phi)) / f["gamma_phi"]
                passive = 0.0
                if d > 0:
                    gp = effective_gamma_below(layer["gamma"], layer["gammaPrime"], 0.0, gwl, max(d, 0.1))
                    passive = 0.5 * 0.5 * gp * d * d * G.kp_rankine(phi) * l / f["gamma_r_e"]
                rh = base / f["gamma_r_h"] + passive
            checks.append({"id": f"en1997.6.5.3.sliding.{footing['id']}.{lc['id']}", "utilization": hd / max(rh, 1.0)})
            if layer.get("cohesionUndrained", 0.0) > 0 or is_cohesive(layer):
                cu = max(layer.get("cohesionUndrained", 0.0), 1.0)
                bb, ll = G.eff_dims(b, l, eb, 0.0)
                rvu = G.bearing_undrained(b, l, d, 0.0, vd, hd, 0.0, eb, 0.0, cu / f["gamma_cu"], g_below / f["gamma_gamma"] * d)["r_k"] / f["gamma_r_v"] if hd <= bb * ll * cu / f["gamma_cu"] else 0.0
                checks.append({"id": f"en1997.6.5.bearing.undrained.{footing['id']}.{lc['id']}", "utilization": vd / max(rvu, 1.0)})
                checks.append({"id": f"en1997.6.5.3.sliding.undrained.{footing['id']}.{lc['id']}", "utilization": hd / max(area * cu / f["gamma_cu"] / f["gamma_r_h"], 1.0)})
            vg, vq = lc.get("verticalPermanent", 0.0), lc.get("verticalVariable", 0.0)
            e_perm = lc.get("momentPermanent", 0.0) / max(vg, 1.0)
            e_tot = (lc.get("momentPermanent", 0.0) + lc.get("momentVariable", 0.0)) / max(vg + vq, 1.0)
            checks.append({"id": f"en1997.6.5.4.eccentricity.{footing['id']}.{lc['id']}", "utilization": max(e_perm / (b / 6), e_tot / (b / 3))})
        q_sls = max([(lc.get("verticalPermanent", 0.0) + lc.get("verticalVariable", 0.0)) / max(b * l, 1e-9) for lc in footing.get("loadCases", [])] + [0.0])
        s, _ = stress_sublayers(layers, b, l, d, q_sls, gwl)
        limit = footing.get("settlementLimit", 0.025)
        checks.append({"id": f"en1997.6.6.settlement.{footing['id']}", "utilization": s / limit})
        checks.append({"id": f"en1997.6.6.elastic.{footing['id']}", "utilization": elastic_layered(layers, b, l, d, q_sls) / limit})
        z_min = footing["embedment"] + max(6.0, 2 * max(b, l))
        checks.append({"id": f"en1997.2.4.2.investigation.{footing['id']}", "utilization": minimum_util(reached_depth(doc), z_min)})

    checks.extend(investigation_extent_checks(doc, annex))
    checks.extend(sls_pair_checks(doc, layers, gwl))

    for pile in doc.get("piles", []):
        profiles = pile.get("testProfiles") or []
        a_base = math.pi * (pile["diameter"] / 2) ** 2
        if profiles:
            shafts, bases = [t["shaftResistance"] for t in profiles], [t["baseResistance"] for t in profiles]
        else:
            shafts = [pile.get("alphaS", 0.7) * math.pi * pile["diameter"] * pile.get("unitShaftResistance", 0.0) * pile["length"]]
            bases = [pile.get("unitBaseResistance", 0.0) * a_base]
        n = len(shafts)
        pile_model = (pile.get("unitBaseResistance", 0.0) * a_base, pile.get("alphaS", 0.7) * math.pi * pile["diameter"] * pile.get("unitShaftResistance", 0.0) * pile["length"])
        best_c, best_t = None, None
        for sit in situations:
            f = G.factors(annex, approach, sit)
            gb, gs, x3, x4 = pile_factors(pile.get("pileType", "bored"), n, f, approach)
            m3, m4 = xi_ground(1)
            rbk = min(C.characteristic(bases, x3, x4)[2], pile_model[0] / max(m3, m4))
            rsk = min(C.characteristic(shafts, x3, x4)[2], pile_model[1] / max(m3, m4))
            rcd = max((rbk / gb + rsk / gs) * pile.get("count", 1), 1.0)
            ned = f["gamma_g"] * pile.get("compressionPermanent", 0.0) + f["gamma_q"] * pile.get("compressionVariable", 0.0)
            u = ned / rcd
            best_c = u if best_c is None or u > best_c else best_c
            ted = f["gamma_g"] * pile.get("tensionPermanent", 0.0) + f["gamma_q"] * pile.get("tensionVariable", 0.0)
            if ted > 1.0 or pile.get("tensionPermanent", 0.0) > 0 or pile.get("tensionVariable", 0.0) > 0:
                rtd = max(rsk / f["gamma_st"] * pile.get("count", 1), 1.0)
                best_t = ted / rtd if best_t is None or ted / rtd > best_t else best_t
        checks.append({"id": f"en1997.7.6.2.compression.{pile['id']}", "utilization": best_c})
        if best_t is not None:
            checks.append({"id": f"en1997.7.6.3.tension.{pile['id']}", "utilization": best_t})

    for wall in doc.get("retainingWalls", []):
        mode = wall.get("earthPressureMode", "active").lower().replace("-", "").replace("_", "")
        phi0 = min(max(wall["backfillPhiDeg"], 0.0), 50.0)
        gamma, h = wall["backfillGamma"], wall["height"]
        delta0 = min(max(wall["wallFrictionDeg"], 0.0), phi0)

        def earth(f):
            phi = math.degrees(math.atan(math.tan(R(phi0)) / f["gamma_phi"]))
            delta = min(delta0, phi)
            if mode in ("atrest", "k0", "ruhe"):
                k, dr = G.k0(phi, wall.get("ocr", 1.0)), 0.0
            elif mode in ("increasedactive", "increased"):
                k, dr = 0.5 * (G.ka_coulomb(phi, delta) + G.k0(phi, wall.get("ocr", 1.0))), R(delta)
            else:
                k, dr = G.ka_coulomb(phi, delta), R(delta)
            ea = 0.5 * k * gamma * h * h + k * wall["surcharge"] * h
            return k, dr, ea * math.cos(dr), ea * math.sin(dr), phi, delta

        gw, gc = 9810.0, max(wall["concreteGamma"], 1000.0)
        area_stem = wall["stemThickness"] * (wall["height"] + wall["baseWidth"])
        worst = (-1.0, situations[0])
        for sit in situations:
            f = G.factors(annex, approach, sit)
            _, _, eah_t, eav_t, phi_t, delta_t = earth(f)
            htry = f["gamma_g"] * (wall["horizontalPermanent"] + eah_t)
            vtry = wall["verticalPermanent"] + area_stem * gc + eav_t
            gp = max(gamma - gw, 1000.0) if gwl < wall["embedment"] else gamma
            epk = 0.5 * gp * wall["embedment"] ** 2 * G.kp_log_spiral(phi_t, delta_t) if wall["embedment"] > 0 else 0.0
            rs = vtry * math.tan(R(phi0)) / f["gamma_phi"] / f["gamma_r_h"] + 0.5 * epk / f["gamma_r_e"]
            u = htry / rs if rs > 1e-9 else 0.0
            if u > worst[0]:
                worst = (u, sit)
        f = G.factors(annex, approach, worst[1])
        k, dr, eah, eav, phi_d, delta_d = earth(f)
        movement = str(wall.get("wallMovement", "free")).lower().replace("-", "").replace("_", "").replace(" ", "")
        if mode in ("active", "atrest", "k0", "increasedactive", "increased", "ruhe") and movement in ("free", "rigid", "propped", ""):
            ka = G.ka_coulomb(phi_d, delta_d)
            k_required = ka if movement in ("free", "") else 0.5 * (ka + G.k0(phi_d, wall.get("ocr", 1.0)))
            checks.append({"id": f"en1997.9.earthPressure.{wall['id']}", "utilization": k_required / k})
        frac_sub_h = wall["height"] + wall["embedment"]
        gc_p = max(gc - gw, 1000.0)
        below = max(min(wall["embedment"] - gwl, frac_sub_h), 0.0)
        if gwl >= frac_sub_h:
            sw = area_stem * gc
        elif gwl <= 0.0:
            sw = area_stem * gc_p
        else:
            fs = min(max(below / max(frac_sub_h, 0.1), 0.0), 1.0)
            sw = area_stem * (gc * (1 - fs) + gc_p * fs)
        hd = f["gamma_g"] * (wall["horizontalPermanent"] + eah)
        vk = wall["verticalPermanent"] + sw + eav
        gp = max(gamma - gw, 1000.0) if gwl < wall["embedment"] else gamma
        epk = 0.5 * gp * wall["embedment"] ** 2 * G.kp_log_spiral(phi_d, delta_d) if wall["embedment"] > 0 else 0.0
        rslide = vk * math.tan(R(phi0)) / f["gamma_phi"] / f["gamma_r_h"] + 0.5 * epk / f["gamma_r_e"]
        checks.append({"id": f"en1997.9.sliding.{wall['id']}", "utilization": hd / rslide})
        equ = G.equ_upl_hyd(annex, worst[1])
        es, eq_ = 0.5 * k * gamma * h * h * math.cos(dr), k * wall["surcharge"] * h * math.cos(dr)
        mdst = equ["equ_dst"] * (es * h / 3 + wall["horizontalPermanent"] * h / 2) + equ["equ_q"] * eq_ * h / 2
        mstb = equ["equ_stb"] * ((wall["verticalPermanent"] + sw) * wall["baseWidth"] / 2 + eav * wall["baseWidth"])
        checks.append({"id": f"en1997.9.overturning.{wall['id']}", "utilization": minimum_util(mstb, mdst)})

    for slope in doc.get("slopes", []):
        if not any(l["id"] == slope.get("governingLayerId") for l in layers):
            continue
        geo3 = G.factors(annex, "geo3", "persistent")
        if slope.get("circleRadius", 0.0) > 1e-6:
            fos = bishop_fos_circle(layers, slope.get("height", 1.0), slope.get("angleDeg", 30.0), slope.get("length", 0.0), gwl, geo3["gamma_phi"], geo3["gamma_c"], slope.get("circleCentreX", 0.0), slope.get("circleCentreZ", 0.0), slope["circleRadius"])
        else:
            fos = bishop_fos_slices(layers, slope.get("height", 1.0), slope.get("angleDeg", 30.0), slope.get("length", 0.0), gwl, geo3["gamma_phi"], geo3["gamma_c"])
        if fos is None:
            continue
        checks.append({"id": f"en1997.11.bishop.{slope['id']}", "utilization": minimum_util(fos, 1.0)})

    for slope in doc.get("slopes", []):
        layer = next((l for l in layers if l["id"] == slope.get("governingLayerId")), None)
        if layer is None or slope.get("width", 0.0) <= 1e-6:
            continue
        geo3 = G.factors(annex, "geo3", "persistent")
        num = {"h": slope["height"], "beta_deg": slope["angleDeg"], "gamma": layer["gamma"], "gamma_sat": layer["gammaPrime"] + 9810.0, "gwl_level": slope["height"] - gwl, "phi_k_deg": layer["phiPrimeDeg"], "c_k": layer.get("cohesionEffective", 0.0), "width": slope["width"]}
        text = {"approach": "geo3", "situation": "persistent"}
        if slope.get("circleRadius", 0.0) > 1e-6:
            num.update(xc=slope["circleCentreX"], yc=slope["circleCentreZ"], r=slope["circleRadius"])
        else:
            run, best = slope["height"] / math.tan(R(slope["angleDeg"])), None
            for ix in range(7):
                for iy in range(6):
                    xc, yc = -0.5 * slope["height"] + (run + slope["height"]) * ix / 6.0, 0.6 * slope["height"] + 1.8 * slope["height"] * iy / 5.0
                    for k in (1.0, 1.08, 1.18, 1.3):
                        trial = dict(num, xc=xc, yc=yc, r=math.hypot(run - xc, yc) * k)
                        try:
                            f2 = C.slope_01(C.Inputs(annex, trial, text))["f"]
                        except (IndexError, ValueError, ZeroDivisionError):
                            continue
                        if f2 > 0 and (best is None or f2 < best[0]):
                            best = (f2, trial)
            if best is None:
                continue
            num = best[1]
        try:
            f3 = C.slope_05(C.Inputs(annex, num, text))["f_3d"]
        except (IndexError, ValueError, ZeroDivisionError):
            continue
        checks.append({"id": f"en1997.11.bishop3d.{slope['id']}", "utilization": minimum_util(f3, 1.0)})

    for case in doc.get("upliftCases", []):
        worst = (-1.0, situations[0])
        for sit in situations:
            u = G.equ_upl_hyd(annex, sit)
            stab = case["permanentStabilizing"] * u["upl_stb"]
            dest = u["upl_dst"] * case["permanentDestabilizing"] + u["upl_q"] * case["variableDestabilizing"]
            ratio = dest / stab if stab > 1e-9 else 0.0
            if ratio > worst[0]:
                worst = (ratio, sit)
        u = G.equ_upl_hyd(annex, worst[1])
        stab = case["permanentStabilizing"] * u["upl_stb"]
        dest = u["upl_dst"] * case["permanentDestabilizing"] + u["upl_q"] * case["variableDestabilizing"]
        checks.append({"id": f"en1997.10.upl.{case['id']}", "utilization": minimum_util(stab, dest)})
        if case["porePressure"] <= 1.0:
            continue
        pore = case["porePressure"]
        checks.append({"id": f"en1997.10.hyd.{case['id']}", "utilization": minimum_util(case["totalStress"] * u["hyd_stb"], pore * u["hyd_h"])})
    checks.extend(entity_checks(doc, annex, approach))
    return {"ok": True, "checks": checks}


def layer_props_at(layers, depth):
    for layer in layers:
        if layer["depthTop"] <= depth <= layer["depthBottom"]:
            return layer
    return layers[0] if layers else {"gamma": 18000.0, "gammaPrime": 10000.0, "phiPrimeDeg": 30.0, "cohesionEffective": 0.0}


def bishop_fos_slices(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c):
    """🌀️ Layered simplified Bishop over the circle grid xc in {0 … 1.05}·run, yc in {1 … 3.2}·H, lowest point 0 … 1·H below the toe (mirrors the Rust grid)."""
    beta = max(0.05, min(1.45, math.radians(angle_deg)))
    run = height / math.tan(beta)
    horiz = max(length, run)
    best = float("inf")
    for xcf in (0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 1.05):
        for ycf in (1.0, 1.4, 1.8, 2.4, 3.2):
            for df in (0.0, 0.15, 0.3, 0.6, 1.0):
                f = _bishop_one_circle(layers, height, beta, run, horiz, xcf * horiz, ycf * height, ycf * height + df * height, gwl, gamma_phi, gamma_c, 60)
                if f is not None and math.isfinite(f) and f > 0.0:
                    best = min(best, f)
    return None if best == float("inf") else best


def bishop_fos_circle(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c, centre_x, centre_z, radius):
    """🎯️ Simplified Bishop on ONE given circle (centre abscissa from the crest edge, centre elevation above the toe, radius), the slope override; None when the circle cuts no admissible sliding mass."""
    beta = max(0.05, min(1.45, math.radians(angle_deg)))
    run = height / math.tan(beta)
    return _bishop_one_circle(layers, height, beta, run, max(length, run), centre_x, centre_z, radius, gwl, gamma_phi, gamma_c, 60)


def _bishop_one_circle(layers, height, beta, run, margin, xc, yc, r, gwl, gamma_phi, gamma_c, n):
    def ground(x):
        return height if x <= 0 else 0.0 if x >= run else height - x * math.tan(beta)

    def base(x):
        u = r * r - (x - xc) ** 2
        return None if u < 0 else yc - math.sqrt(u)

    xs = [xc - r + 2 * r * i / 2000 for i in range(2001)]
    inside = [x for x in xs if base(x) is not None and base(x) < ground(x)]
    if not inside:
        return None
    xa, xb = inside[0], inside[-1]
    if xb - xa < 1e-6 or xa < -margin - 1e-9 or xb > run + margin + 1e-9:
        return None
    dx = (xb - xa) / n
    slices, drive = [], 0.0
    for i in range(n):
        x = xa + (i + 0.5) * dx
        yb, yg = base(x), ground(x)
        h = max(yg - yb, 0.0)
        if h < 1e-9:
            continue
        layer = layer_props_at(layers, max(height - 0.5 * (yg + yb), 0.0))
        y_w = yg - gwl
        wet = min(max(y_w - yb, 0.0), h)
        w = (layer["gamma"] * (h - wet) + (layer["gammaPrime"] + 9810.0) * wet) * dx
        u = 9810.0 * max(y_w - yb, 0.0) * dx
        al = math.asin((xc - x) / r)
        drive += w * math.sin(al)
        slices.append((w, al, math.tan(math.radians(layer["phiPrimeDeg"])) / gamma_phi, layer.get("cohesionEffective", 0.0) / gamma_c, dx, u))
    if drive <= 1e-9:
        return None
    f = 1.5
    for _ in range(200):
        resist = 0.0
        for w, al, t, c, b, u in slices:
            m = math.cos(al) + math.sin(al) * t / f
            if m <= 0.2:
                return None
            resist += (c * b + max(w - u, 0.0) * t) / m
        nf = resist / drive
        if not math.isfinite(nf) or nf <= 0:
            return None
        if abs(nf - f) < 1e-9:
            return nf
        f = nf
    return f


def self_check() -> None:
    assert abs(G.nq(30) - 18.401) < 0.05
    assert abs(G.nc(30) - 30.14) < 0.1
    assert abs(G.ng(30) - 20.093) < 0.15
    assert abs(G.ka_coulomb(30, 20) - 0.29731) < 1e-4


def catalogue(fixture: dict[str, Any]) -> dict[str, Any]:
    cases = []
    for row in fixture["cases"]:
        case = row["case"]
        cases.append({"entry": row["entry"], "values": C.evaluate(row["entry"], case["annex"], case.get("num", {}), case.get("text", {}))})
    return {"cases": cases}


def reference() -> dict[str, Any]:
    ka = G.ka_coulomb(30, 20)
    sq = 1 + math.sin(R(30))
    out = {
        "ka_30_20": ka,
        "ka_30_0": G.ka_coulomb(30, 0),
        "sc_square": (sq * G.nq(30) - 1) / (G.nq(30) - 1),
        "mu_ex97_01": 555.0 / ((18 * G.nq(30) + 0.5 * 18 * 2 * G.ng(30)) * 2 / 1.4),
        "rcd_ex97_12": min(1533.333 / 1.15, 1450 / 1.05) / 1.1,
        "kp_rankine_30": G.kp_rankine(30),
        "kp_curved_30_15": G.kp_log_spiral(30, 15),
    }
    assert abs(out["ka_30_20"] - 0.29731) < 1e-4 and abs(out["sc_square"] - 1.5287) < 1e-3
    assert abs(out["mu_ex97_01"] - 0.5607) < 1e-3 and abs(out["rcd_ex97_12"] - 1212.12) < 0.05
    assert 4.0 <= out["kp_curved_30_15"] <= 4.9
    return out


def main(argv: list[str]) -> int:
    self_check()
    if "--json" in argv:
        json.dump(evaluate(json.load(sys.stdin)), sys.stdout)
    elif "--catalogue" in argv:
        json.dump(catalogue(json.load(sys.stdin)), sys.stdout)
    elif "--reference" in argv:
        json.dump(reference(), sys.stdout)
    else:
        json.dump({"ok": True, "n_q_30": G.nq(30), "n_c_30": G.nc(30), "n_gamma_30": G.ng(30)}, sys.stdout)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
