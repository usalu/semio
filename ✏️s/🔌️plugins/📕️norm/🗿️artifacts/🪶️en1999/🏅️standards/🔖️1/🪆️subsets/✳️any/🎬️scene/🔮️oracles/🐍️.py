"""🔮️ Independent oracle of the EN 1999 scene: closed-form section and HAZ areas, the EN 1999-1-1 6.3.1.2 reduction factors of classes A and B, the LTB classes, the ρ_c plate reduction, the HAZ width rule,
the EN 1999-1-3 S–N law, the EN 1999-1-2 gas and lumped-capacity aluminium temperature, the EN 1999-1-5 shell reduction and the M–N polygon (shapely when installed, shoelace otherwise).
the bolt group in plan, the fillet weld resistance, the cold-formed sheet reduction and utilisation (sectionproperties for the strip modulus when installed) and the torsional-flexural critical load (numpy generalised eigenproblem, bisection fallback; sectionproperties for the section constants when installed), the elastic force distribution of an eccentric bolt group (numpy stiffness method) and the axial shell buckling pattern (scipy minimisation of the axisymmetric critical load).
the torsional and welded-member buckling curves (the HAZ factor κ, the torsional load N_cr,T(L) from the validated section constants) and the bars of the outcome diagrams.
Reads {"scene", "diagrams", "sections", "fatigue", "fires", "shells", "connections", "sheets", "tf", "shellModes", "buckling"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

try:
    from shapely.geometry import Point, Polygon
except ImportError:
    Polygon = None

BUCKLING = {"chi-A": (0.20, 0.10), "chi-B": (0.32, 0.0), "chi-lt-12": (0.10, 0.6), "chi-lt-34": (0.20, 0.4)}
RHO = {
    "A": {(False, False): (32.0, 220.0, 22.0), (False, True): (29.0, 198.0, 18.0), (True, False): (10.0, 24.0, 6.0), (True, True): (9.0, 20.0, 5.0)},
    "B": {(False, False): (29.0, 198.0, 18.0), (False, True): (25.0, 150.0, 15.0), (True, False): (9.0, 20.0, 5.0), (True, True): (8.0, 16.0, 4.0)},
}
CLASS_LIMITS = {"A": {(False, False): (11, 16, 22), (False, True): (9, 13, 18), (True, False): (3, 4.5, 6), (True, True): (2.5, 4, 5)}, "B": {(False, False): (13, 16.5, 18), (False, True): (10, 13.5, 15), (True, False): (3.5, 4.5, 5), (True, True): (3, 3.5, 4)}}
N_C, N_D, N_L = 2.0e6, 5.0e6, 1.0e8


def chi(lam, alpha, lam0):
    if lam <= lam0:
        return 1.0
    phi = 0.5 * (1.0 + alpha * (lam - lam0) + lam * lam)
    return min(1.0, 1.0 / (phi + math.sqrt(max(phi * phi - lam * lam, 0.0))))


def i_area(h, b, tw, tf):
    return 2.0 * b * tf + (h - 2.0 * tf) * tw


def i_polygon(h, b, tw, tf):
    w, d = b / 2.0, h / 2.0
    return [(-w, -d), (w, -d), (w, -d + tf), (tw / 2, -d + tf), (tw / 2, d - tf), (w, d - tf), (w, d), (-w, d), (-w, d - tf), (-tw / 2, d - tf), (-tw / 2, -d + tf), (-w, -d + tf)]


def shoelace(points):
    n = len(points)
    return abs(sum(points[i][0] * points[(i + 1) % n][1] - points[(i + 1) % n][0] * points[i][1] for i in range(n))) / 2.0


def area(points):
    return Polygon(points).area if Polygon else shoelace(points)


def params(section_data):
    return {p["name"]: p["value"] for p in section_data["params"]}


def strength(dc, m1, m2, n):
    if n <= N_D:
        return dc * (N_C / n) ** (1.0 / m1)
    d = dc * (N_C / N_D) ** (1.0 / m1)
    if n >= N_L:
        return d * (N_D / N_L) ** (1.0 / m2)
    return d * (N_D / n) ** (1.0 / m2)


def haz_width_mm(t_mm):
    return 20.0 if t_mm <= 6.0 else 30.0 if t_mm <= 12.0 else 35.0 if t_mm <= 25.0 else 40.0


def gas(t_s):
    return 20.0 + 345.0 * math.log10(8.0 * t_s / 60.0 + 1.0)


def aluminium(theta0, am_v, duration):
    theta, t = max(theta0, 20.0), 0.0
    while t < duration - 1e-9:
        step = min(5.0, duration - t)
        t += step
        g = gas(t)
        h = 25.0 * (g - theta) + 0.3 * 5.67e-8 * ((g + 273.0) ** 4 - (theta + 273.0) ** 4)
        theta += am_v / ((0.41 * theta + 903.0) * 2700.0) * h * step
    return theta


def section_factor(s):
    if s["kind"] in ("tube", "chs"):
        d = max(s["d"], s["b"])
        t = max(s["tw"], s["tf"])
        return math.pi * d / (math.pi / 4.0 * (d * d - (d - 2 * t) ** 2))
    return (2 * s["h"] + 4 * s["b"] - 2 * s["tw"]) / i_area(s["h"], s["b"], s["tw"], s["tf"])


def alpha_x(r, t, q=25.0):
    dw = math.sqrt(r / t) * t / q
    return 0.62 / (1.0 + 1.91 * (dw / t) ** 1.44)


def shell_chi(lam, alpha, beta=0.6, eta=1.0, lam0=0.2):
    if lam <= lam0:
        return 1.0
    lam_p = math.sqrt(alpha / (1.0 - beta))
    return alpha / (lam * lam) if lam >= lam_p else 1.0 - beta * ((lam - lam0) / (lam_p - lam0)) ** eta


FILLER_FW = {"4043": 190.0e6, "alsi5": 190.0e6, "5356": 240.0e6, "almg5": 240.0e6, "5183": 270.0e6}
TF_PARTICIPATION = 0.05
MESH_DIVISOR = 16.0
SECTION_TOLERANCE = 2e-2
NT_TOLERANCE = 2e-2


def hole_diameter(d):
    return d + (0.002 if d <= 0.024 else 0.003)


def rho_c(beta_eps, class_a, welded):
    c1, c2, b3 = RHO["A" if class_a else "B"][(False, welded)]
    return 1.0 if beta_eps <= b3 else max(0.0, min(1.0, c1 / beta_eps - c2 / beta_eps ** 2))


def strip_modulus(width, thickness):
    """Elastic section modulus of the b x t strip about its strong axis: sectionproperties when installed, b t^2 / 6 otherwise."""
    try:
        from sectionproperties.analysis import Section
        from sectionproperties.pre.library import rectangular_section

        geometry = rectangular_section(d=thickness, b=width)
        geometry.create_mesh(mesh_sizes=[(width * thickness) / 4.0])
        section = Section(geometry)
        section.calculate_geometric_properties()
        return section.get_z()[0], True
    except Exception:
        return width * thickness ** 2 / 6.0, False


def elastic_group(rows, per_row, p1, p2, ecc):
    """Unit shear along x at the lever arm ecc over an equal-stiffness grid on a rigid plate by the stiffness method (numpy least squares of the 3x3 plate equilibrium): the (fx, fy) of every fastener, rows outermost."""
    import numpy

    pts = [((i - (rows - 1) / 2.0) * p1, (j - (per_row - 1) / 2.0) * p2) for i in range(rows) for j in range(per_row)]
    k = numpy.zeros((3, 3))
    for x, y in pts:
        t = numpy.array([[1.0, 0.0, -y], [0.0, 1.0, x]])
        k += t.T @ t
    u = numpy.linalg.lstsq(k, numpy.array([1.0, 0.0, -ecc]), rcond=None)[0]
    return pts, [(float(u[0] - u[2] * y), float(u[1] + u[2] * x)) for x, y in pts]


def ic_group(rows, per_row, p1, p2, ecc, mu, exponent):
    """Unit shear along x at the lever arm ecc by the instantaneous-centre method (curve R/R_ult = (1 - exp(-mu r/r_max))^k) with scipy: the plate equilibrium in the centre (x, y) and the scale is solved without assuming the symmetry axis; the (fx, fy) of every fastener, rows outermost."""
    from scipy.optimize import least_squares

    pts = [((i - (rows - 1) / 2.0) * p1, (j - (per_row - 1) / 2.0) * p2) for i in range(rows) for j in range(per_row)]
    if len(pts) < 2 or ecc == 0.0:
        return elastic_group(rows, per_row, p1, p2, ecc)

    def units(xc, yc):
        radii = [math.hypot(x - xc, y - yc) for x, y in pts]
        r_max = max(radii)
        out = []
        for (x, y), r in zip(pts, radii):
            w = (1.0 - math.exp(-mu * r / r_max)) ** exponent if r > 1e-15 else 0.0
            out.append((0.0, 0.0) if r <= 1e-15 else (-(y - yc) / r * w, (x - xc) / r * w))
        return out

    def residual(v):
        xc, yc, lam = v
        f = units(xc, yc)
        return [lam * sum(a for a, _ in f) - 1.0, lam * sum(b for _, b in f), lam * sum(x * b - y * a for (x, y), (a, b) in zip(pts, f)) + ecc]

    reach = max(math.hypot(x, y) for x, y in pts)
    for scale in (0.05, 0.3, 1.0, 3.0, 10.0, 30.0):
        sol = least_squares(residual, [0.0, -math.copysign(scale * reach, ecc), -1.0 / len(pts)], xtol=1e-14, ftol=1e-14, gtol=1e-14)
        if sol.cost < 1e-18:
            return pts, [(sol.x[2] * a, sol.x[2] * b) for a, b in units(sol.x[0], sol.x[1])]
    return elastic_group(rows, per_row, p1, p2, ecc)


def tf_matrices(system):
    nu, nv, nt, i0, y0, z0 = (system[k] for k in ("nu", "nv", "nt", "i0Sq", "y0", "z0"))
    return [[nu, 0.0, 0.0], [0.0, nv, 0.0], [0.0, 0.0, i0 * nt]], [[1.0, 0.0, z0], [0.0, 1.0, -y0], [z0, -y0, i0]]


def tf_lowest(system):
    """Lowest critical load of K x = N G x whose mode twists the section (the generalised eigenproblem, not the cubic)."""
    stiffness, geometric = tf_matrices(system)
    i0 = system["i0Sq"]
    import numpy

    values, vectors = numpy.linalg.eig(numpy.linalg.solve(numpy.array(geometric), numpy.array(stiffness)))
    for k in numpy.argsort(values.real):
        x = vectors[:, k].real
        weight = math.sqrt(i0) * abs(x[2])
        if weight > TF_PARTICIPATION * math.sqrt(x[0] ** 2 + x[1] ** 2 + weight ** 2):
            return float(values[k].real)
    return None


def tf_section_constants(section, member_length):
    """A, I_y, I_z, I_t, I_w and the shear centre offset of an extruded I or channel from sectionproperties, or None when it is not installed."""
    try:
        from sectionproperties.analysis import Section
        from sectionproperties.pre.library import channel_section, i_section

        h, b, tf_, tw = section["h"], section["b"], section["tf"], section["tw"]
        geometry = i_section(d=h, b=b, t_f=tf_, t_w=tw, r=0.0, n_r=1) if section["kind"].lower().replace("-", "").replace("_", "") == "extrudedi" else channel_section(d=h, b=b, t_f=tf_, t_w=tw, r=0.0, n_r=1)
        geometry.create_mesh(mesh_sizes=[(tf_ * tw) / MESH_DIVISOR])
        analysis = Section(geometry)
        analysis.calculate_geometric_properties()
        analysis.calculate_warping_properties()
        (cx, cy), (sx, sy) = analysis.get_c(), analysis.get_sc()
        ixx, iyy, _ = analysis.get_ic()
        return {"a": analysis.get_area(), "ixx": ixx, "iyy": iyy, "j": analysis.get_j(), "iw": analysis.get_gamma(), "offsets": sorted([abs(sx - cx), abs(sy - cy)])}
    except Exception:
        return None


def inside(ring, point):
    if Polygon:
        return Polygon(ring).buffer(1e-9).contains(Point(point))
    x, y = point
    hit = False
    for (x1, y1), (x2, y2) in zip(ring, ring[1:] + ring[:1]):
        if (y1 > y) != (y2 > y) and x < (x2 - x1) * (y - y1) / (y2 - y1 + 1e-300) + x1:
            hit = not hit
    return hit


def kappa(k0, lam):
    return k0 + (1.0 - k0) * min(max(lam - 0.2, 0.0), 1.0)


def lam_torsion(item, length):
    n_cr = (27000.0e6 * item["it"] + math.pi ** 2 * 70000.0e6 * item["iw"] / (length * length)) / item["i0Sq"]
    return math.sqrt(item["aLoc"] * item["fo"] / n_cr)


def torsion_resistance(item, length):
    lam = lam_torsion(item, length)
    factor = kappa(item["k0"], lam) * chi(lam, item["alphaT"], item["lam0T"])
    return lam, factor, factor * item["aLoc"] * item["fo"] / item["g1"]


def main():
    data = json.load(sys.stdin)
    buckling = {item["id"]: item for item in data.get("buckling", [])}
    failures = []
    checks = [0]

    def check(ok, message):
        checks[0] += 1
        if not ok:
            failures.append(message)

    sections = {s["id"]: s for s in data["sections"]}
    bounds = data["scene"]["bounds"]
    max_b = max(s["b"] for s in sections.values()) / 2.0 + 1e-3
    max_h = max(s["h"] for s in sections.values()) / 2.0 + 1e-3
    for entity in data["scene"]["entities"]:
        geometry = entity["geometry"]
        ident = entity["entity"]["id"]
        if geometry["kind"] != "member":
            continue
        shape = geometry["data"]["section"]
        if shape["kind"] == "parametric" and shape["data"]["profile"] == "iShape":
            p = params(shape["data"])
            closed = i_area(p["h"], p["b"], p["tw"], p["tf"])
            check(abs(closed - area(i_polygon(p["h"], p["b"], p["tw"], p["tf"]))) < 1e-12, f"{ident}: closed form vs polygon")
            check(any(abs(s["h"] - p["h"]) < 1e-12 and abs(s["b"] - p["b"]) < 1e-12 and abs(i_area(s["h"], s["b"], s["tw"], s["tf"]) - closed) < 1e-12 for s in sections.values()), f"{ident}: outline matches a document section")
        if shape["kind"] == "polygon":
            poly = shape["data"]["polygon"]
            outer = [(q["x"], q["y"]) for q in poly["outer"]["points"]]
            holes = [[(q["x"], q["y"]) for q in h["points"]] for h in poly["holes"]]
            net = shoelace(outer) - sum(shoelace(h) for h in holes)
            check(net > 0.0, f"{ident}: positive net area")
            if ":haz-" in ident:
                xs, ys = [q[0] for q in outer], [q[1] for q in outer]
                check(len(outer) == 4 and abs(net - (max(xs) - min(xs)) * (max(ys) - min(ys))) < 1e-12, f"{ident}: HAZ band is a rectangle")
                check(max(abs(v) for v in xs) <= max_b + 1e-9 and max(abs(v) for v in ys) <= max_h + 1e-9, f"{ident}: HAZ band inside the section")
        for point in geometry["data"]["axis"]["data"]["points"]:
            check(all(bounds["min"][k] - 1e-9 <= point[k] <= bounds["max"][k] + 1e-9 for k in "xyz"), f"{ident}: axis outside bounds")

    for diagram in data["diagrams"]:
        kind = diagram["id"].split(":")[0]
        series = [s["data"] for s in diagram["series"]]
        rows = {len(c["values"]) for c in diagram["tableTwin"]["columns"]}
        check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
        if kind in ("buckling", "ltb"):
            for s in series:
                if s["id"] in BUCKLING:
                    alpha, lam0 = BUCKLING[s["id"]]
                    for q in s["points"]:
                        check(abs(q["y"] - chi(q["x"], alpha, lam0)) < 1e-9, f"{diagram['id']}: {s['id']} at {q['x']}")
            for marker in diagram["markers"]:
                x, y = marker["at"]["x"], marker["at"]["y"]
                curves = [BUCKLING[s["id"]] for s in series if s["id"] in BUCKLING]
                check(any(abs(chi(x, a, l) - y) < 1e-6 for a, l in curves), f"{diagram['id']}: marker {marker['id']} not on a curve")
        if kind == "torsional":
            item = buckling[diagram["id"].split(":")[1]]
            lam, factor, _ = torsion_resistance(item, item["lcr"])
            for s in series:
                if s["id"] == "chi-t":
                    for q in s["points"]:
                        check(abs(q["y"] - chi(q["x"], item["alphaT"], item["lam0T"])) < 1e-9, f"{diagram['id']}: chi-t at {q['x']}")
                if s["id"] == "kappa-chi-t":
                    for q in s["points"]:
                        check(abs(q["y"] - kappa(item["k0"], q["x"]) * chi(q["x"], item["alphaT"], item["lam0T"])) < 1e-9, f"{diagram['id']}: kappa-chi-t at {q['x']}")
            marker = diagram["markers"][0]
            check(abs(marker["at"]["x"] - min(lam, 3.0)) < 1e-6 and abs(marker["at"]["y"] - factor) < 1e-6, f"{diagram['id']}: design point ({marker['at']['x']}, {marker['at']['y']}) vs ({lam}, {factor})")
        if kind == "torsional-length":
            item = buckling[diagram["id"].split(":")[1]]
            lengths, resistance = [c["values"] for c in diagram["tableTwin"]["columns"]]
            for length, kn in zip(lengths, resistance):
                check(abs(kn - torsion_resistance(item, length)[2] / 1.0e3) < 1e-6 * max(1.0, kn), f"{diagram['id']}: N_b,T,Rd at L={length}")
            marker = diagram["markers"][0]
            check(abs(marker["at"]["y"] - torsion_resistance(item, item["lcr"])[2] / 1.0e3) < 1e-6 * max(1.0, marker["at"]["y"]), f"{diagram['id']}: marker resistance")
            check(marker["binding"]["path"].endswith(".bucklingLengthT") and abs(marker["binding"]["map"]["k"] - 1.0) < 1e-12, f"{diagram['id']}: marker drags the torsional buckling length")
        if kind == "haz-buckling":
            item = buckling[diagram["id"].split(":")[1]]
            for s in series:
                for q in s["points"]:
                    expected = {"chi": chi(q["x"], item["alpha"], item["lam0"]), "kappa": kappa(item["k0"], q["x"]), "kappa-chi": kappa(item["k0"], q["x"]) * chi(q["x"], item["alpha"], item["lam0"])}[s["id"]]
                    check(abs(q["y"] - expected) < 1e-9, f"{diagram['id']}: {s['id']} at {q['x']}")
        if kind == "check-outcomes":
            utilisation = diagram["tableTwin"]["columns"][1]["values"]
            bars = [s for s in series if s["id"].startswith("outcome-")]
            check(len(bars) == len(utilisation) == len(diagram["markers"]), f"{diagram['id']}: one bar and one marker per check")
            for bar, u, marker in zip(bars, utilisation, diagram["markers"]):
                top = max(q["y"] for q in bar["ring"])
                check(abs(top - max(u, 0.0)) < 1e-12 and abs(marker["at"]["y"] - max(u, 0.0)) < 1e-12, f"{diagram['id']}: bar {bar['id']} height")
            check(any(s["id"] == "limit" and abs(s["value"] - 1.0) < 1e-12 for s in series), f"{diagram['id']}: limit line at 1")
        if kind == "mn":
            ring = [(q["x"], q["y"]) for q in next(s for s in series if "ring" in s)["ring"]]
            check(len(ring) >= 3, f"{diagram['id']}: ring too small")
            check(abs(area(ring) - shoelace(ring)) < 1e-6 * max(1.0, shoelace(ring)), f"{diagram['id']}: polygon area")
            for marker in diagram["markers"]:
                point = (marker["at"]["x"], marker["at"]["y"])
                check(marker.get("utilisation") is None or marker["utilisation"] > 1.0 or inside(ring, point), f"{diagram['id']}: compliant demand outside the polygon")
        if kind == "sn":
            detail = next(f for f in data["fatigue"] if f["id"] == diagram["id"].split(":")[1])
            for s in series:
                if s["id"] == "wohler" or s["id"].startswith("category-"):
                    dc = detail["dc"] if s["id"] == "wohler" else float(s["id"].split("-")[1])
                    for q in s["points"]:
                        check(abs(q["y"] - strength(dc, detail["m1"], detail["m2"], q["x"])) < 1e-6 * q["y"], f"{diagram['id']}: {s['id']} at N={q['x']:.3g}")
            limits = {s["id"]: s["value"] for s in series if "value" in s}
            knee = detail["dc"] * (N_C / N_D) ** (1.0 / detail["m1"])
            check(abs(limits["delta-d"] - knee) < 1e-9, f"{diagram['id']}: fatigue limit")
            check(abs(limits["delta-l"] - knee * (N_D / N_L) ** (1.0 / detail["m2"])) < 1e-9, f"{diagram['id']}: cut-off limit")
        if kind == "hazwidth":
            for q in next(s for s in series if s["id"] == "b-haz")["points"]:
                check(abs(q["y"] - haz_width_mm(q["x"])) < 1e-9, f"{diagram['id']}: b_haz at {q['x']} mm")
        if kind == "haz":
            for s in series:
                check(all(0.0 < q["y"] <= 1.0 for q in s["points"]), f"{diagram['id']}: {s['id']} outside (0, 1]")
        if kind == "class":
            columns = {c["header"]["en"]: c["values"] for c in diagram["tableTwin"]["columns"]}
            for beta, l1, l2, l3, klass, rho in zip(columns["Slenderness β/ε"], columns["Limit class 1"], columns["Limit class 2"], columns["Limit class 3"], columns["Class"], columns["Reduction ρ_c"]):
                expected = 1 if beta <= l1 else 2 if beta <= l2 else 3 if beta <= l3 else 4
                check(int(klass) == expected, f"{diagram['id']}: class of beta/eps {beta}")
                check((rho == 1.0) == (klass < 4) and 0.0 <= rho <= 1.0, f"{diagram['id']}: rho_c {rho} of class {klass}")
        if kind == "effective":
            for s in series:
                outstand = "outstand" in s["id"]
                welded = "welded" in s["id"]
                matched = False
                for cls in ("A", "B"):
                    c1, c2, b3 = RHO[cls][(outstand, welded)]
                    if all(abs(q["y"] - (1.0 if q["x"] <= b3 else max(0.0, min(1.0, c1 / q["x"] - c2 / (q["x"] ** 2))))) < 1e-9 for q in s["points"]):
                        matched = True
                check(matched, f"{diagram['id']}: {s['id']} matches no table")
        if kind == "fire":
            fire = next(f for f in data["fires"] if f["id"] == diagram["id"].split(":")[1])
            am_v = section_factor(sections[fire["section"]])
            for q in next(s for s in series if s["id"] == "gas")["points"]:
                check(abs(q["y"] - gas(q["x"] * 60.0)) < 1e-9, f"{diagram['id']}: gas at {q['x']} min")
            for q in next(s for s in series if s["id"] == "aluminium")["points"]:
                check(abs(q["y"] - aluminium(fire["thetaA"], am_v, q["x"] * 60.0)) < 1e-6, f"{diagram['id']}: aluminium at {q['x']} min")
        if kind == "fire-k":
            ks = [q["y"] for q in next(s for s in series if s["id"] == "k-theta")["points"]]
            check(abs(ks[0] - 1.0) < 1e-12 and all(a >= b - 1e-12 for a, b in zip(ks, ks[1:])), f"{diagram['id']}: k_theta monotone from 1")
        if kind == "shell":
            shell = next(s for s in data["shells"] if s["id"] == diagram["id"].split(":")[1])
            alpha = alpha_x(shell["r"], shell["t"])
            for q in next(s for s in series if s["id"] == "chi-x")["points"]:
                check(abs(q["y"] - shell_chi(q["x"], alpha)) < 1e-9, f"{diagram['id']}: chi_x at {q['x']}")

        if kind == "bolt-group":
            conn = next(c for c in data["connections"] if c["id"] == diagram["id"].split(":")[1])
            d0 = hole_diameter(conn["d"])
            length = 2.0 * conn["e"] + (conn["rows"] - 1) * conn["p1"]
            width = 2.0 * conn["e"] + (conn["perRow"] - 1) * conn["p2"]
            columns = diagram["tableTwin"]["columns"]
            xs, ys = columns[0]["values"], columns[1]["values"]
            check(len(xs) == conn["rows"] * conn["perRow"], f"{diagram['id']}: one bolt per grid place")
            expected = [(conn["e"] + i * conn["p1"], conn["e"] + j * conn["p2"]) for i in range(conn["rows"]) for j in range(conn["perRow"])]
            check(all(abs(x - ex) < 1e-12 and abs(y - ey) < 1e-12 for x, y, (ex, ey) in zip(xs, ys, expected)), f"{diagram['id']}: bolt grid")
            plate = [(0.0, 0.0), (length, 0.0), (length, width), (0.0, width)]
            check(abs(area(plate) - length * width) < 1e-12, f"{diagram['id']}: plate area")
            for s in series:
                if s["id"].startswith("hole-"):
                    ring = [(q["x"], q["y"]) for q in s["points"]]
                    cx, cy = sum(q[0] for q in ring[:-1]) / (len(ring) - 1), sum(q[1] for q in ring[:-1]) / (len(ring) - 1)
                    check(all(abs(math.hypot(q[0] - cx, q[1] - cy) - d0 / 2.0) < 1e-12 for q in ring), f"{diagram['id']}: {s['id']} radius d0/2")
                    if conn["clear"]:
                        check(all(-1e-12 <= q[0] <= length + 1e-12 and -1e-12 <= q[1] <= width + 1e-12 for q in ring), f"{diagram['id']}: {s['id']} inside the plate")
            pts, unit = (ic_group if conn["method"] == "instantaneousCentre" else elastic_group)(conn["rows"], conn["perRow"], conn["p1"], conn["p2"], conn["ecc"], *((conn["mu"], conn["exponent"]) if conn["method"] == "instantaneousCentre" else ()))
            resultants = [math.hypot(fx, fy) for fx, fy in unit]
            peak = max(resultants)
            kn = columns[2]["values"]
            check(all(abs(v / max(kn) - r / peak) < 1e-9 for v, r in zip(kn, resultants)), f"{diagram['id']}: force distribution vs the stiffness method")
            check(abs(sum(fx for fx, _ in unit) - 1.0) < 1e-9 and abs(sum(fy for _, fy in unit)) < 1e-9, f"{diagram['id']}: force balance")
            check(abs(sum(x * fy - y * fx for (x, y), (fx, fy) in zip(pts, unit)) + conn["ecc"]) < 1e-9, f"{diagram['id']}: moment balance")
            arrow = 0.35 * max(conn["p1"], conn["p2"])
            for s in series:
                if s["id"].startswith("force-"):
                    row, line = (int(v) for v in s["id"].split("-")[1:])
                    fx, fy = unit[row * conn["perRow"] + line]
                    base, tip = s["points"][0], s["points"][1]
                    check(abs(tip["x"] - base["x"] - arrow * fx / peak) < 1e-9 and abs(tip["y"] - base["y"] - arrow * fy / peak) < 1e-9, f"{diagram['id']}: {s['id']} arrow vector")
            for marker in diagram["markers"]:
                binding = marker.get("binding")
                if binding and binding["path"].endswith("bolts.pitch"):
                    check(abs(binding["map"]["base"] + binding["map"]["k"] * marker["at"]["x"] - conn["p1"]) < 1e-12, f"{diagram['id']}: pitch binding")
                if binding and binding["path"].endswith("bolts.edgeDistance"):
                    check(abs(binding["map"]["base"] + binding["map"]["k"] * marker["at"]["x"] - conn["e"]) < 1e-12, f"{diagram['id']}: edge binding")
        if kind == "weld-resistance":
            conn = next(c for c in data["connections"] if c["id"] == diagram["id"].split(":")[1])
            f_w = FILLER_FW.get(conn["filler"].lower(), min(conn["fu"], 190.0e6))
            metal = next(s for s in series if s["id"] == "weld-metal")["points"]
            haz = next(s for s in series if s["id"] == "haz-boundary")["points"]
            for q in metal:
                check(abs(q["y"] * 1e3 - f_w * q["x"] / (conn["betaW"] * conn["gammaMw"])) < 1e-6 * q["y"] * 1e3, f"{diagram['id']}: weld metal at a={q['x']}")
            for q in haz:
                if q["x"] / 0.7 <= conn["plateT"]:
                    check(abs(q["y"] * 1e3 - conn["rhoHaz"] * conn["fu"] * (q["x"] / 0.7) / (math.sqrt(3.0) * conn["gammaM2"])) < 1e-6 * q["y"] * 1e3, f"{diagram['id']}: HAZ boundary at a={q['x']}")
            marker = diagram["markers"][0]
            f_w_at = FILLER_FW.get(conn["filler"].lower(), min(conn["fuAt"], 190.0e6))
            metal_at = f_w_at * conn["throat"] / (conn["betaW"] * conn["gammaMw"])
            haz_at = conn["rhoAt"] * conn["fuAt"] * (conn["throat"] / 0.7) / (math.sqrt(3.0) * conn["gammaM2"])
            check(abs(marker["at"]["y"] * 1e3 - min(metal_at, haz_at)) < 1e-6 * marker["at"]["y"] * 1e3, f"{diagram['id']}: design resistance is the smaller of metal and HAZ")
        if kind == "sheet-local":
            sheet = next(s for s in data["sheets"] if s["id"] == diagram["id"].split(":")[1])
            for q in next(s for s in series if s["id"] == "rho-c")["points"]:
                check(abs(q["y"] - rho_c(q["x"], sheet["classA"], sheet["welded"])) < 1e-9, f"{diagram['id']}: rho_c at {q['x']}")
            marker = diagram["markers"][0]
            beta_eps = sheet["width"] / sheet["thickness"] / sheet["eps"]
            check(abs(marker["at"]["x"] - min(beta_eps, 60.0)) < 1e-9 and abs(marker["at"]["y"] - rho_c(beta_eps, sheet["classA"], sheet["welded"])) < 1e-9, f"{diagram['id']}: marker on the curve")
            binding = marker["binding"]
            check(abs(binding["map"]["k"] * marker["at"]["x"] - sheet["width"]) < 1e-9 * sheet["width"] or beta_eps > 60.0, f"{diagram['id']}: width binding")
        if kind == "sheet-resistance":
            sheet = next(s for s in data["sheets"] if s["id"] == diagram["id"].split(":")[1])
            ts = diagram["tableTwin"]["columns"][0]["values"]
            curves = {c["header"]["en"]: c["values"] for c in diagram["tableTwin"]["columns"][1:]}
            for index, (t, band) in enumerate(zip(ts, sheet["samples"])):
                rho_haz = band["rhoHaz"] if sheet["welded"] else 1.0
                rho = rho_c(sheet["width"] / t / band["eps"], band["classA"], sheet["welded"])
                n_rd = rho * sheet["width"] * t * rho_haz * band["fo"] / sheet["gammaM1"]
                m_rd = rho_haz * (sheet["width"] * t * t / 6.0) * band["fo"] / sheet["gammaM1"]
                v_rd = sheet["width"] * t * band["fo"] / (math.sqrt(3.0) * sheet["gammaM1"])
                un, um, uv = sheet["nEd"] / n_rd, sheet["mEd"] / m_rd, sheet["vEd"] / v_rd
                for name, value in (("Axial N_Ed/N_Rd", un), ("Bending M_Ed/M_Rd", um), ("Shear V_Ed/V_Rd", uv), ("Combined N + M", un + um)):
                    check(abs(curves[name][index] - min(value, 3.0)) < 1e-9 * max(1.0, value), f"{diagram['id']}: {name} at t={t}")
            for t in (ts[1], ts[len(ts) // 2], ts[-1]):
                modulus, third_party = strip_modulus(sheet["width"], t)
                check(abs(modulus - sheet["width"] * t * t / 6.0) < 1e-6 * modulus, f"{diagram['id']}: strip modulus at t={t} ({'sectionproperties' if third_party else 'closed form'})")

    for item in data.get("tf", []):
        lowest = tf_lowest(item)
        check(lowest is not None and abs(lowest - item["factor"]) < 1e-7 * lowest, f"{item['member']}: torsional-flexural load {item['factor']} vs eigenproblem {lowest}")
        stiffness, geometric = tf_matrices(item)
        n = item["factor"]
        residual = [sum((stiffness[r][c] - n * geometric[r][c]) * item["mode"][c] for c in range(3)) for r in range(3)]
        check(max(abs(v) for v in residual) < 1e-7 * max(item["nu"], item["nv"], item["i0Sq"] * item["nt"]), f"{item['member']}: mode residual {residual}")
        if abs(item["y0"]) + abs(item["z0"]) > 1e-3:
            check(item["factor"] < item["nt"], f"{item['member']}: coupling through the shear centre offset lowers the twisting load below the torsional one")
        data = item["sectionData"]
        constants = tf_section_constants(data, item["length"]) if data["kind"].lower().replace("-", "").replace("_", "") in ("extrudedi", "channel") else None
        if constants:
            pi_l = math.pi / item["length"]
            e = 70000.0e6
            mine = item["constants"]
            size = max(data["h"], data["b"])
            lo, hi = sorted([mine["iy"], mine["iz"]])
            check(abs(mine["a"] - constants["a"]) < 1e-9 * constants["a"] and abs(lo - min(constants["ixx"], constants["iyy"])) < 1e-6 * lo and abs(hi - max(constants["ixx"], constants["iyy"])) < 1e-6 * hi, f"{item['member']}: A, I_y, I_z vs sectionproperties")
            check(abs(mine["it"] - constants["j"]) < SECTION_TOLERANCE * constants["j"], f"{item['member']}: St Venant constant I_t {mine['it']} vs sectionproperties {constants['j']}")
            check(abs(mine["iw"] - constants["iw"]) < SECTION_TOLERANCE * constants["iw"], f"{item['member']}: warping constant I_w {mine['iw']} vs sectionproperties {constants['iw']}")
            check(abs(sorted([abs(item["y0"]), abs(item["z0"])])[0] - constants["offsets"][0]) < 2e-3 * size and abs(sorted([abs(item["y0"]), abs(item["z0"])])[1] - constants["offsets"][1]) < 2e-3 * size, f"{item['member']}: shear centre offset vs sectionproperties")
            check(abs(sorted([item["nu"], item["nv"]])[0] - pi_l ** 2 * e * min(constants["ixx"], constants["iyy"])) < 1e-5 * item["nv"] and abs(sorted([item["nu"], item["nv"]])[1] - pi_l ** 2 * e * max(constants["ixx"], constants["iyy"])) < 1e-5 * item["nu"], f"{item['member']}: bending Euler loads vs sectionproperties")
            i0 = (constants["ixx"] + constants["iyy"]) / constants["a"] + sum(v * v for v in constants["offsets"])
            check(abs(i0 - item["i0Sq"]) < SECTION_TOLERANCE * i0, f"{item['member']}: polar radius of gyration vs sectionproperties")
            nt = (27000.0e6 * constants["j"] + pi_l ** 2 * e * constants["iw"]) / i0
            check(abs(nt - item["nt"]) < NT_TOLERANCE * nt, f"{item['member']}: torsional load {item['nt']} vs sectionproperties {nt}")

    for mode in data.get("shellModes", []):
        r, t = mode["r"], mode["t"]
        e, g = 70000.0e6, 27000.0e6
        nu = e / (2.0 * g) - 1.0
        flexural_rigidity = e * t ** 3 / (12.0 * (1.0 - nu ** 2))
        k0 = 1.0 / math.sqrt(r * t)
        from scipy.optimize import minimize_scalar

        best = minimize_scalar(lambda u: flexural_rigidity * (u * k0) ** 2 + e * t / (r * r * (u * k0) ** 2), bounds=(0.1, 10.0), method="bounded", options={"xatol": 1e-13})
        half = math.pi / (best.x * k0)
        check(abs(half - mode["halfWave"]) < 1e-6 * half, f"{mode['id']}: axial half-wave {mode['halfWave']} vs the minimised N(k) {half}")
        check(abs(best.fun / t - 0.605 * e * t / r) < 2e-3 * best.fun / t, f"{mode['id']}: classical stress {best.fun / t} vs 0.605 E t/r")
        w0, w1 = mode["window"]
        check(abs(half - 1.728 * math.sqrt(r * t)) < 1e-3 * half, f"{mode['id']}: 1.728 sqrt(r t)")
        for x, c, dx, dy, dz in mode["samples"]:
            w = math.sin(mode["waves"] * math.pi * (x - w0) / (w1 - w0)) if w0 - 1e-12 <= x <= w1 + 1e-12 else 0.0
            a = 2.0 * math.pi * c / 48.0
            check(abs(dx - w * math.cos(a)) < 1e-9 and abs(dy - w * math.sin(a)) < 1e-9 and dz == 0.0, f"{mode['id']}: displacement at x={x}, column {c}")

    print(json.dumps({"ok": not failures, "checks": checks[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
