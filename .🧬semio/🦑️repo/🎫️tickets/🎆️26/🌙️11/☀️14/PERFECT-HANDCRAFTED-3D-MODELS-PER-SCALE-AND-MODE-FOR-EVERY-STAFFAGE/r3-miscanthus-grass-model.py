"""🌾 Round 3, vegetation/miscanthus-grass: converts the hand-stated plant (`r3-miscanthus-grass-plant.py`) into the root fixes (rig origins, anchors, texts), the volumes of the four scales and the 2D drawings of the three views (ruling E1); writes the candidate asset document and the volumes source.

Usage: python3 r3-miscanthus-grass-model.py
See: ./r3-vegetation-miscanthus-grass.md
"""
import copy
import importlib.util
import json
import math
from pathlib import Path

TICKET = Path(__file__).resolve().parent
CATALOG = TICKET.parents[6] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/miscanthus-grass/🔣️.json"
OUT = TICKET / "🗑️generated/r3-miscanthus-grass"
SOURCE = TICKET / "volumes/vegetation/miscanthus-grass.json"
TARGET = {"x": 1.07, "z": 0.9, "y": 1.75}
VIEWS = ["elevation", "front", "plan"]


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), TICKET / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load("r3-miscanthus-grass-kit")
P = load("r3-miscanthus-grass-plant")
TEXTS = json.loads((TICKET / "r3-miscanthus-grass-texts.json").read_text(encoding="utf-8"))

BLADE = {"material": "miscanthus-blade", "role": "body", "tone": 0.5}
STEM = {"material": "miscanthus-culm", "role": "body", "tone": 0.36}
LITTER = {"material": "miscanthus-culm-mat", "role": "body", "tone": 0.3}
PLUME = {"material": "miscanthus-plume", "role": "detail", "tone": 0.78}


def profile(table, t):
    for (t0, v0), (t1, v1) in zip(table, table[1:]):
        if t <= t1:
            return v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    return table[-1][1]


def section_table(table, n):
    out = []
    for i in range(n):
        t = i / (n - 1)
        for a, b in zip(table, table[1:]):
            if t <= b[0] + 1e-9:
                s = (t - a[0]) / (b[0] - a[0])
                out.append((a[1] + (b[1] - a[1]) * s, a[2] + (b[2] - a[2]) * s))
                break
    return out


def settle(path, diameter):
    """📏 A stem path the centre line of the validator reads as drawn: a path that bows less than six diameters off its chord is made straight (the validator draws such a part along its principal axis), a stronger bow stays."""
    a, b = path[0], path[-1]
    chord = K.unit(K.sub(b, a))
    bow = max(K.norm(K.sub(K.sub(q, a), K.mul(chord, K.dot(K.sub(q, a), chord)))) for q in path)
    BOWS.append(bow / diameter)
    if bow >= 6 * diameter:
        return path
    return [K.lerp(a, b, i / (len(path) - 1)) for i in range(len(path))]


BOWS = []


def azimuth_of(vector):
    return math.degrees(math.atan2(vector[2], vector[0]))


class Plant:
    """🌱 The parts of one scale: world paths with their sections, the bone that carries each and the thermal region."""

    def __init__(self, rig, scale, fit):
        self.rig, self.scale, self.fit, self.parts, self.stems, self.strips, self.count = rig, scale, fit, [], {}, [], 0

    def path(self, start, d0, d1, length, power, n, reach, margin, floor):
        """📏 An arching path kept inside the envelope of `dimensions` and above the ground: a `reach` path is grown first and ends on the envelope, every other path that would leave it is shortened to a staggered length inside it."""
        self.count += 1
        length *= 1.15 if reach else 1.0
        cut = False
        for _ in range(800):
            fine_points = K.arch(start, d0, d1, length, power, 25)
            if all(abs(q[0]) <= TARGET["x"] - margin and abs(q[2]) <= TARGET["z"] - margin and q[1] >= floor for q in fine_points):
                break
            length *= 0.995
            cut = True
        if cut and not reach:
            length *= 0.8 + 0.2 * ((self.count * 0.618) % 1)
        return K.arch(start, d0, d1, length, power, n), length

    def part(self, **fields):
        self.parts.append(fields)
        return fields

    def blade(self, part_id, bone, start, stem_tangent, azimuth, row, table, reach=None):
        _, _, divergence, end, length, width = row
        horizontal = K.direction(90, azimuth)
        d0 = K.unit(K.add(K.mul(stem_tangent, math.cos(math.radians(divergence))), K.mul(horizontal, math.sin(math.radians(divergence)))))
        n = table["blade_points"]
        points, length = self.path(start, d0, K.direction(end, azimuth), length, 1.5, n, reach, width / 2, 0.012)
        width = min(width, length / 22)
        fallback = K.direction(90, azimuth + 90)
        ts = K.tangents(points)
        rx = [width / 2 * profile(P.BLADE_PROFILE, i / (n - 1)) for i in range(n)]
        return self.part(id=part_id, bone=bone, region="blades", look=BLADE, points=points, widths=[K.width_axis(t, fallback) for t in ts], rx=rx, ry=[table["blade_half_thickness"]] * n, slender=True, width=width, fallback=fallback)

    def ribbon(self, part_id, bone, start, d0, azimuth, end, length, table, reach=None):
        half, thick = table["ribbon"]
        n = table["ribbon_points"]
        points, length = self.path(start, d0, K.direction(end, azimuth), length, 1.4, n, reach, 0.004, 0.02)
        fallback = K.direction(90, azimuth + 90)
        return self.part(id=part_id, bone=bone, region="blades", look=BLADE, points=points, widths=[K.width_axis(t, fallback) for t in K.tangents(points)], rx=[half * profile(P.BLADE_PROFILE, i / (n - 1)) for i in range(n)], ry=[thick] * n, slender=True, width=2 * half, fallback=fallback)

    def round(self, part_id, bone, points, radii, region, look, slender=True):
        widths = []
        for t in K.tangents(points):
            c = K.cross(t, (0.0, 0.0, 1.0))
            widths.append(K.unit(c) if K.norm(c) > 0.2 else (1.0, 0.0, 0.0))
        return self.part(id=part_id, bone=bone, region=region, look=look, points=points, widths=widths, rx=list(radii), ry=list(radii), slender=slender, width=2 * max(radii))

    def mass(self, part_id, bone, points, sections, azimuth, region, look, exponent=2.0, round_end=True):
        fallback = K.direction(90, azimuth + 90)
        ts = K.tangents(points)
        if sections[0][0] > 0.02:
            points = [K.sub(points[0], K.mul(ts[0], 0.8 * sections[0][1]))] + list(points)
            sections = [(0.35 * sections[0][0], 0.35 * sections[0][1])] + list(sections)
        if round_end and sections[-1][0] > 0.02:
            points = list(points) + [K.add(points[-1], K.mul(ts[-1], 0.8 * sections[-1][1]))]
            sections = list(sections) + [(0.35 * sections[-1][0], 0.35 * sections[-1][1])]
        return self.part(id=part_id, bone=bone, region=region, look=look, points=points, widths=[K.width_axis(t, fallback) for t in K.tangents(points)], rx=[s[0] for s in sections], ry=[s[1] for s in sections], slender=False, width=2 * max(s[0] for s in sections), exponent=exponent)

    def dome(self, part_id, table_rows, look):
        points = [(0.0, u, 0.0) for u, _ in table_rows]
        return self.part(id=part_id, bone="root-plate", region="culms", look=look, points=points, widths=[(1.0, 0.0, 0.0)] * len(points), rx=[r for _, r in table_rows], ry=[r for _, r in table_rows], slender=False, width=2 * max(r for _, r in table_rows), dome=table_rows)

    def bundle_stems(self, name, n, diameter):
        a, b = f"bl-{name}a", f"bl-{name}b"
        o, j = self.rig.origin(a), self.rig.origin(b)
        da, db = K.unit(K.sub(j, o)), self.axis(b)
        h = K.unit(K.add(da, db))
        la = K.norm(K.sub(j, o))
        path = K.bezier(o, K.sub(j, K.mul(h, 0.62 * la)), j, n)
        return o, j, h, db, settle(path, diameter) if diameter > 0 else path

    def axis(self, bone):
        """🧭 The true direction of an end bone: its sagittal direction with the lateral gain of `LATERAL` over its length."""
        tip = K.add(self.rig.tip(bone), (0.0, 0.0, P.LATERAL[bone]))
        return K.unit(K.sub(tip, self.rig.origin(bone)))

    def culm_paths(self, name, n):
        a, b, p = f"culm-{name}a", f"culm-{name}b", f"plume-{name}"
        o, j, e = self.rig.origin(a), self.rig.origin(b), self.rig.origin(p)
        pa = [K.lerp(o, j, i / (n - 1)) for i in range(n)]
        pb = [K.lerp(j, e, i / (n - 1)) for i in range(n)]
        return pa, pb


def leaves(plant, prefix, bone, path, azimuth, rows, table, reach=None):
    ts = K.tangents(path)
    for i, row in enumerate(rows):
        at = K.along(path, row[0])
        k = min(range(len(path)), key=lambda q: K.norm(K.sub(path[q], at)))
        plant.blade(f"{prefix}-{i + 1}", bone, at, ts[k], azimuth + row[1], row, table, reach if i == len(rows) - 1 else None)


def fine(rig, scale, table, fit):
    """🌾 1:50 and 1:100: every shoot with its own stem and leaves, every raceme of the panicles."""
    plant = Plant(rig, scale, fit)
    plant.dome("crown", table["mound"], LITTER)
    for i, (ring, azimuth, height, tilt, tilt_azimuth) in enumerate(table["stubble"]):
        centre = K.mul(K.direction(90, azimuth), ring)
        across = K.direction(90, azimuth + 90)
        for k, sign in enumerate((1, -1) if "stubble_pair" in table else (0,)):
            apart, taller, lean = table.get("stubble_pair", (0.0, 0.0, 0))
            foot = (centre[0] + sign * apart * across[0], 0.02, centre[2] + sign * apart * across[2])
            tip = K.add(foot, K.mul(K.direction(tilt + sign * lean, tilt_azimuth), height + sign * taller))
            plant.part(id=f"stub-{i + 1}" + ("" if sign == 0 else "ab"[k]), bone="root-plate", region="culms", look=LITTER, capsule=(foot, tip, table["stubble_radius"]), slender=False, width=2 * table["stubble_radius"])
    for i, row in enumerate(table["basal"]):
        azimuth, incline, end, length, width = row
        foot = K.mul(K.direction(90, azimuth), table["basal_ring"])
        plant.blade(f"basal-{i + 1}", "root-plate", (foot[0], 0.035, foot[2]), K.direction(incline, azimuth), azimuth, (0, 0, 0, end, length, width), table)
    n = 7 if scale == 50 else 5
    for name in P.BUNDLES:
        a, b = f"bl-{name}a", f"bl-{name}b"
        o, j, h, db, stem_a = plant.bundle_stems(name, n, 2 * table["stem_a_radius"][0])
        ra = table["stem_a_radius"]
        plant.round(f"stem-{name}a", a, stem_a, [ra[0] + (ra[1] - ra[0]) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.stems[a] = stem_a
        sb, rb0, rb1 = table["stem_b"]
        q = K.add(j, K.mul(db, sb))
        stem_b = settle(K.bezier(j, K.add(j, K.mul(h, 0.4 * sb)), q, 4), 2 * rb0)
        plant.round(f"stem-{name}b", b, stem_b, [rb0 + (rb1 - rb0) * i / 3 for i in range(4)], "culms", STEM)
        azimuth = azimuth_of(K.sub(j, o))
        leaves(plant, f"blade-{name}a", a, stem_a, azimuth, table["main_a"], table)
        leaves(plant, f"blade-{name}b", b, stem_b, azimuth, table["main_b"], table, "x")
        plant.stems[b] = stem_b + plant.parts[-1]["points"][1:]
        for s, (turn, lean, length) in enumerate(table["satellites"][name]):
            lean_azimuth = azimuth + turn
            away = K.direction(90, lean_azimuth)
            foot = (o[0] + 0.05 * away[0], o[1] - 0.01, o[2] + 0.05 * away[2])
            end = K.add(foot, K.mul(K.direction(lean, lean_azimuth), length))
            control = K.add(foot, K.mul(K.direction(lean * 0.45, lean_azimuth), 0.55 * length))
            path = settle(K.bezier(foot, control, end, 5), 2 * table["satellite_radius"][0])
            rs = table["satellite_radius"]
            plant.round(f"tiller-{name}-{s + 1}", a, path, [rs[0] + (rs[1] - rs[0]) * i / 4 for i in range(5)], "culms", STEM)
            side = "z"
            leaves(plant, f"blade-{name}s{s + 1}", a, path, lean_azimuth, table["satellite_leaves"], table, side)
    for name in P.CULMS:
        a, b, p = f"culm-{name}a", f"culm-{name}b", f"plume-{name}"
        path_a, path_b = plant.culm_paths(name, n)
        ra, rb = table["culm_a_radius"], table["culm_b_radius"]
        plant.round(f"culm-{name}a", a, path_a, [ra[0] + (ra[1] - ra[0]) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.round(f"culm-{name}b", b, path_b, [rb[0] + (rb[1] - rb[0]) * i / (n - 1) for i in range(n)], "culms", STEM)
        plant.stems[a], plant.stems[b] = path_a, path_b
        lean = azimuth_of(K.sub(path_b[-1], path_a[0]))
        azimuth = lean + table["culm_twist"][name]
        leaves(plant, f"blade-c{name}a", a, path_a, azimuth, table["culm_a_leaves"], table)
        leaves(plant, f"blade-c{name}b", b, path_b, azimuth, table["culm_b_leaves"], table)
        for c, (turn, tilt, length) in enumerate(table["companions"][name]):
            away = K.direction(90, lean + turn)
            foot = (path_a[0][0] + 0.045 * away[0], path_a[0][1] - 0.01, path_a[0][2] + 0.045 * away[2])
            path = settle(K.bezier(foot, K.add(foot, K.mul(K.direction(tilt * 0.4, lean + turn), 0.55 * length)), K.add(foot, K.mul(K.direction(tilt, lean + turn), length)), 5), 2 * table["satellite_radius"][0])
            rs = table["satellite_radius"]
            plant.round(f"tiller-c{name}-{c + 1}", a, path, [rs[0] + (rs[1] - rs[0]) * i / 4 for i in range(5)], "culms", STEM)
            leaves(plant, f"blade-c{name}t{c + 1}", a, path, lean + turn, table["satellite_leaves"], table)
        if "racemes" not in table:
            plume(plant, rig, name, table, fit)
            continue
        base, axis = rig.origin(p), plant.axis(p)
        length, r0, r1 = table["rachis"]
        rachis = [K.add(base, K.mul(axis, length * i / 3)) for i in range(4)]
        plant.round(f"rachis-{name}", p, rachis, [r0 + (r1 - r0) * i / 3 for i in range(4)], "plumes", PLUME)
        plant.stems[p] = rachis
        side = K.unit(K.cross(axis, (0.0, 0.0, 1.0)))
        top = name in ("2", "3")
        for i, (t, around, opening, end, raceme_length, radius) in enumerate(table["racemes"]):
            c, s = math.cos(math.radians(around)), math.sin(math.radians(around))
            radial = K.add(K.mul(side, c), K.mul((0.0, 0.0, 1.0), s))
            d0 = K.unit(K.add(K.mul(axis, math.cos(math.radians(opening))), K.mul(radial, math.sin(math.radians(opening)))))
            out = K.unit(K.add(K.mul((radial[0], 0.0, radial[2]), 1 - table["nod"]), K.mul(K.direction(90, lean), table["nod"])))
            d1 = K.unit(K.add(K.mul(out, math.sin(math.radians(end))), K.mul(K.UP, math.cos(math.radians(end)))))
            m = table["raceme_points"]
            points = K.arch(K.add(base, K.mul(axis, length * t)), d0, d1, raceme_length * (fit["y"] if top else 1.0), 1.7, m)
            plant.round(f"raceme-{name}-{i + 1}", p, points, [radius * (1 - 0.45 * q / (m - 1)) for q in range(m)], "plumes", PLUME)
    lead = {f"bl-{name}{seg}": f"blade-{name}{seg}-{len(table['main_' + seg])}" for name in P.BUNDLES for seg in "ab"}
    by_id = {part["id"]: part for part in plant.parts}
    for i, bone in enumerate([f"bl-{name}{seg}" for name in P.BUNDLES for seg in "ab"]):
        plant.strips.append((i + 1, by_id[lead[bone]], table["strip"]))
    return plant


def coarse200(rig, table, fit):
    """🌾 1:200: leaves merged into arching ribbons of 24 mm, shoots into sheaves, each panicle into one flat nodding plume."""
    plant = Plant(rig, 200, fit)
    plant.dome("crown", table["mound"], LITTER)
    for i, (azimuth, incline, end, length) in enumerate(table["basal"]):
        foot = K.mul(K.direction(90, azimuth), table["basal_ring"])
        plant.ribbon(f"ribbon-basal-{i + 1}", "root-plate", (foot[0], 0.04, foot[2]), K.direction(incline, azimuth), azimuth, end, length, table)
    for name in P.BUNDLES:
        a, b = f"bl-{name}a", f"bl-{name}b"
        o, j, h, db, stem_a = plant.bundle_stems(name, 5, 2 * table["stem_a_radius"])
        plant.round(f"sheaf-{name}", a, stem_a, [table["stem_a_radius"]] * 5, "culms", STEM)
        plant.stems[a] = stem_a
        azimuth = azimuth_of(K.sub(j, o))
        for i, (t, dphi, divergence, end, length) in enumerate(table["main_b"]):
            horizontal = K.direction(90, azimuth + dphi)
            d0 = K.unit(K.add(K.mul(h, math.cos(math.radians(divergence))), K.mul(horizontal, math.sin(math.radians(divergence)))))
            lead = i == len(table["main_b"]) - 1
            plant.ribbon(f"ribbon-{name}b-{i + 1}", b, j, K.unit(K.add(h, db)) if lead else d0, azimuth + dphi, end, length, table, "x" if lead else None)
        plant.stems[b] = plant.parts[-1]["points"]
        for i, (turn, lean, length) in enumerate(table["laterals"][name]):
            plant.ribbon(f"ribbon-{name}a-{i + 1}", a, (o[0], o[1], o[2]), K.direction(lean * 0.5, azimuth + turn), azimuth + turn, 135, length, table, "z")
    for name in P.CULMS:
        a, b = f"culm-{name}a", f"culm-{name}b"
        path_a, path_b = plant.culm_paths(name, 5)
        r0, r1 = table["culm_radius"]
        plant.round(f"culm-{name}a", a, path_a, [r0] * 5, "culms", STEM)
        plant.round(f"culm-{name}b", b, path_b, [r0 + (r1 - r0) * i / 4 for i in range(5)], "culms", STEM)
        plant.stems[a], plant.stems[b] = path_a, path_b
        lean = azimuth_of(K.sub(path_b[-1], path_a[0]))
        for i, (t, dphi, divergence, end, length) in enumerate(table["culm_leaves"]):
            azimuth = lean + table["culm_twist"][name] + dphi
            at = K.along(path_a, t)
            tangent = K.tangents(path_a)[2]
            d0 = K.unit(K.add(K.mul(tangent, math.cos(math.radians(divergence))), K.mul(K.direction(90, azimuth), math.sin(math.radians(divergence)))))
            plant.ribbon(f"ribbon-c{name}-{i + 1}", a, at, d0, azimuth, end, length, table)
        plume(plant, rig, name, table, fit)
    by_id = {part["id"]: part for part in plant.parts}
    for i, name in enumerate(P.BUNDLES):
        plant.strips.append((2 * i + 1, by_id[f"ribbon-{name}a-1"], table["strip"]))
        plant.strips.append((2 * i + 2, by_id[f"ribbon-{name}b-{len(table['main_b'])}"], table["strip"]))
    return plant


def plume(plant, rig, name, table, fit):
    p = f"plume-{name}"
    base, axis = rig.origin(p), plant.axis(p)
    lean = azimuth_of(K.sub(base, rig.origin(f"culm-{name}a")))
    rows = table["plume"]
    length = rows[-1][0] * (fit["y"] if name in ("2", "3") else 1.0)
    nod = K.unit(K.add(K.mul(axis, math.cos(math.radians(table["plume_nod"]))), K.mul(K.direction(90, lean), math.sin(math.radians(table["plume_nod"])))))
    points = K.arch(base, axis, nod, length, 1.3, 7)
    plant.stems[p] = [K.lerp(points[0], K.along(points, 0.5), i / 3) for i in range(4)]
    side = K.unit(K.cross(axis, (0.0, 0.0, 1.0)))
    for i, (opening, around, share) in enumerate(table.get("plume_fan", [(0, 0, 1.0)])):
        c, s = math.cos(math.radians(around)), math.sin(math.radians(around))
        radial = K.add(K.mul(side, c), K.mul((0.0, 0.0, 1.0), s))
        d0 = K.unit(K.add(K.mul(axis, math.cos(math.radians(opening))), K.mul(radial, math.sin(math.radians(opening)))))
        d1 = K.unit(K.add(nod, K.mul(radial, 1.6 * math.sin(math.radians(opening)))))
        path = points if i == 0 else K.arch(base, d0, d1, length * share, 1.3, 7)
        plant.mass(f"plume-{name}" if i == 0 else f"plume-{name}-{i + 1}", p, path, section_table([(r[0] / rows[-1][0], r[1], r[2]) for r in rows], 7), lean, "plumes", PLUME)


def coarse500(rig, table, fit):
    """🌾 1:500: the leaves of every shoot merged into pointed arching tongues that follow the bones, culms as lines with a flat panicle plume."""
    plant = Plant(rig, 500, fit)
    plant.dome("crown", table["mound"], LITTER)
    for name in P.BUNDLES:
        a, b = f"bl-{name}a", f"bl-{name}b"
        o, j, h, db, stem_a = plant.bundle_stems(name, 5, 0)
        base = azimuth_of(K.sub(j, o))
        plant.mass(f"lobe-{name}a", a, stem_a, section_table(table["lobe_a"], 5), base, "blades", BLADE, round_end=False)
        plant.stems[a] = stem_a
        end = min(170.0, math.degrees(math.acos(max(-1.0, min(1.0, db[1])))) + table["lobe_b_droop"])
        points, _ = plant.path(j, h, K.direction(end, azimuth_of(db)), K.norm(K.sub(K.add(rig.tip(b), (0.0, 0.0, P.LATERAL[b])), j)), 1.3, 7, True, 0.004, 0.02)
        plant.mass(f"lobe-{name}b", b, points, section_table(table["lobe_b"], 7), azimuth_of(db), "blades", BLADE)
        plant.stems[b] = points
        for i, (turn, lean, end_lateral, length) in enumerate(table["laterals"][name]):
            points, _ = plant.path((o[0], o[1], o[2]), K.direction(lean * 0.5, base + turn), K.direction(end_lateral, base + turn), length, 1.4, 7, True, 0.004, 0.02)
            plant.mass(f"lobe-{name}s{i + 1}", a, points, section_table(table["lateral_section"], 7), base + turn, "blades", BLADE)
    for name in P.CULMS:
        a, b = f"culm-{name}a", f"culm-{name}b"
        path_a, path_b = plant.culm_paths(name, 4)
        plant.round(f"culm-{name}a", a, path_a, [table["culm_radius"]] * 4, "culms", STEM)
        plant.round(f"culm-{name}b", b, path_b, [table["culm_radius"]] * 4, "culms", STEM)
        plant.stems[a], plant.stems[b] = path_a, path_b
        lean = azimuth_of(K.sub(path_b[-1], path_a[0]))
        for i, (turn, tilt, end, length) in enumerate(table["inner"][name]):
            points, _ = plant.path(path_a[0], K.direction(tilt * 0.4, lean + turn), K.direction(end, lean + turn), length, 1.8, 7, False, 0.004, 0.02)
            plant.mass(f"lobe-c{name}-{i + 1}", a, points, section_table(table["inner_section"], 7), lean + turn, "blades", BLADE)
        plume(plant, rig, name, table, fit)
    for part in plant.parts:
        if part["id"].startswith("lobe-"):
            part["envelope"] = True
    by_id = {part["id"]: part for part in plant.parts}
    for i, name in enumerate(P.BUNDLES):
        plant.strips.append((2 * i + 1, by_id[f"lobe-{name}s1"], table["strip"]))
        plant.strips.append((2 * i + 2, by_id[f"lobe-{name}b"], table["strip"]))
    return plant


def ring(part, i, count=20):
    p, x, t = part["points"][i], part["widths"][i], K.tangents(part["points"])[i]
    x = K.unit(K.sub(x, K.mul(t, K.dot(x, t))))
    y = K.cross(t, x)
    e = 2.0 / part.get("exponent", 2.0)
    out = []
    for k in range(count):
        a = 2 * math.pi * k / count
        c, s = math.cos(a), math.sin(a)
        out.append(K.add(p, K.add(K.mul(x, part["rx"][i] * math.copysign(abs(c) ** e, c)), K.mul(y, part["ry"][i] * math.copysign(abs(s) ** e, s)))))
    return out


def boundary(part):
    if "capsule" in part:
        a, b, r = part["capsule"]
        return [K.add(q, d) for q in (a, b) for d in ((r, 0, 0), (-r, 0, 0), (0, r, 0), (0, -r, 0), (0, 0, r), (0, 0, -r))]
    return [q for i in range(len(part["points"])) for q in ring(part, i)]


def extent(plant):
    pts = [q for part in plant.parts for q in boundary(part)]
    return {"x-": min(q[0] for q in pts), "x+": max(q[0] for q in pts), "y-": min(q[1] for q in pts), "y+": max(q[1] for q in pts), "z-": min(q[2] for q in pts), "z+": max(q[2] for q in pts)}


def build(rig, scale):
    """📏 Builds the plant of the scale and fits the lengths of the outermost leaves and of the two upright panicles to `dimensions`."""
    fit = {"y": 1.0, "sheaf": 1.0}
    maker = {50: lambda: fine(rig, 50, P.S50, fit), 100: lambda: fine(rig, 100, P.S100, fit), 200: lambda: coarse200(rig, P.S200, fit), 500: lambda: coarse500(rig, P.S500, fit)}[scale]
    for _ in range(8):
        plant = maker()
        e = extent(plant)
        fit["y"] *= 1 + (TARGET["y"] - e["y+"]) / 0.3
    plant = maker()
    plant.extent = extent(plant)
    return plant


def solid_of(rig, part):
    if "capsule" in part:
        a, b, r = part["capsule"]
        return {"id": part["id"], "kind": "capsule", "bone": part["bone"], **part["look"], "p0": rig.point(part["bone"], a), "p1": rig.point(part["bone"], b), "r0": r, "r1": r}
    solid = K.loft(rig, part["id"], part["bone"], part["points"], part["widths"], part["rx"], part["ry"], part["look"], part.get("exponent", 2.0))
    if part["slender"]:
        solid["slender"] = True
    if part.get("envelope"):
        solid["envelope"] = True
    return solid


def strip_of(rig, plant, index, part, row, scale):
    t0, t1, w = row
    n = len(part["points"])
    ts = K.tangents(part["points"])
    lengths = [0.0]
    for a, b in zip(part["points"], part["points"][1:]):
        lengths.append(lengths[-1] + K.norm(K.sub(b, a)))
    points, widths, rx = [], [], []
    m = 6
    for k in range(m):
        t = t0 + (t1 - t0) * k / (m - 1)
        target = t * lengths[-1]
        i = max(q for q in range(n - 1) if lengths[q] <= target + 1e-12)
        s = (target - lengths[i]) / (lengths[i + 1] - lengths[i])
        p = K.lerp(part["points"][i], part["points"][i + 1], s)
        tangent = K.unit(K.lerp(ts[i], ts[i + 1], s))
        x = K.unit(K.lerp(part["widths"][i], part["widths"][i + 1], s))
        x = K.unit(K.sub(x, K.mul(tangent, K.dot(x, tangent))))
        normal = K.cross(tangent, x)
        if normal[1] < 0:
            normal = K.mul(normal, -1)
        half = part["rx"][i] + (part["rx"][i + 1] - part["rx"][i]) * s
        thick = part["ry"][i] + (part["ry"][i + 1] - part["ry"][i]) * s
        points.append(K.add(p, K.mul(normal, thick + 0.0004)))
        widths.append(x)
        rx.append(half * w if scale <= 100 else w)
    solid = K.loft(rig, f"th-transpiration-blades-{index}", part["bone"], points, widths, rx, [0.0005] * m, {"material": "miscanthus-blade", "role": "detail", "tone": 0.6})
    solid.update({"slender": True, "region": "blades", "tags": ["emitter", "transpiration-blades"]})
    mid = K.add(K.lerp(points[2], points[3], 0.5), (0.0, 0.003, 0.0))
    return solid, mid


def structure_of(rig, plant, scale):
    """🏗️ The load path: the rhizome mat fixed in the ground, a rod of the declared section along every culm, the shoot axis and the midrib of the leading leaf along every blade bundle, the panicle axis, joints, the support and the masses."""
    look = {"material": "miscanthus-culm", "role": "body", "tone": 0.5}
    out = []
    mat = {50: [(0.002, 0.175), (0.03, 0.165), (0.048, 0.12), (0.06, 0.06)], 100: [(0.004, 0.175), (0.035, 0.155), (0.06, 0.06)], 200: [(0.004, 0.175), (0.06, 0.12)], 500: [(0.004, 0.175), (0.08, 0.15)]}[scale]
    crown = {"points": [(0.0, u, 0.0) for u, _ in mat], "widths": [(1.0, 0.0, 0.0)] * len(mat), "rx": [r for _, r in mat], "ry": [r for _, r in mat]}
    out.append({**K.loft(rig, "member-root-plate", "root-plate", crown["points"], crown["widths"], crown["rx"], crown["ry"], look), "member": "root-plate", "part": "member"})
    radius = {"culm": (0.006, 0.004), "bl": ((0.003, 0.0015) if scale <= 100 else (0.004, 0.003)), "plume": (0.002, 0.002)}
    take = {50: 1, 100: 2, 200: 2, 500: 3}[scale]
    for bone, path in plant.stems.items():
        kind = "culm" if bone.startswith("culm") else "plume" if bone.startswith("plume") else "bl"
        r = radius[kind][0 if bone.endswith("a") or kind == "plume" else 1]
        points = path[::take] if (len(path) - 1) % take == 0 else path[::take] + [path[-1]]
        if scale == 500:
            points = [path[0], path[len(path) // 2], path[-1]]
        radii = [r if kind != "bl" or scale > 50 else r * (1 - 0.3 * i / (len(points) - 1)) for i in range(len(points))]
        solid = K.rod(rig, f"member-{bone}", bone, points, radii, look)
        solid.update({"slender": True, "member": bone, "part": "member"})
        out.append(solid)
    if scale <= 100:
        r = (P.S50 if scale == 50 else P.S100)["joint_radius"]
        for bone in plant.stems:
            out.append({"id": f"joint-{bone}", "kind": "ellipsoid", "bone": bone, "material": "miscanthus-culm", "role": "detail", "tone": 0.35, "radii": [r, r, r], "member": bone, "part": "joint"})
    t = {50: 0.02, 100: 0.02, 200: 0.03, 500: 0.075}[scale]
    plate = K.loft(rig, "support-base-support", "root-plate", [(0.0, 0.0, 0.0), (0.0, t + 0.001, 0.0)], [(1.0, 0.0, 0.0)] * 2, [0.2, 0.2], [0.2, 0.2], {"material": "miscanthus-culm-mat", "role": "detail", "tone": 0.25})
    out.append({**plate, "member": "root-plate", "part": "support", "anchor": "base-support"})
    m = {50: [0.022, 0.06, 0.06], 100: [0.024, 0.055, 0.055], 200: [0.03, 0.05, 0.05], 500: [0.04, 0.045, 0.045]}[scale]
    out.append({"id": "mass-root-plate", "kind": "ellipsoid", "bone": "root-plate", "material": "miscanthus-culm-mat", "role": "detail", "tone": 0.4, "radii": m, "transform": {"translation": [P.CROWN_LENGTH / 2, 0, 0]}, "member": "root-plate", "part": "mass"})
    if scale <= 200:
        r = {50: 0.009, 100: 0.009, 200: 0.016}[scale]
        for name in P.CULMS:
            out.append({"id": f"mass-plume-{name}", "kind": "ellipsoid", "bone": f"plume-{name}", "material": "miscanthus-plume", "role": "detail", "tone": 0.4, "radii": [r, r, r], "transform": {"translation": rig.point(f"plume-{name}", K.along(plant.stems[f"plume-{name}"], 0.7))}, "member": f"plume-{name}", "part": "mass"})
    return out


def outline(part, view, rig):
    """✏️ The outline of a mass in the frame of its 2D shape: the two flanks of the projected loft, or the hull where the path is seen end-on."""
    bone = part["bone"]
    rings = [[K.view_xy(view, rig, bone, q) for q in ring(part, i, 24)] for i in range(len(part["points"]))]
    centres = [K.view_xy(view, rig, bone, p) for p in part["points"]]
    run = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in zip(centres, centres[1:]))
    if "dome" in part:
        rows = part["dome"]
        if view == "plan":
            r = max(q[1] for q in rows)
            return [[round(r * math.cos(2 * math.pi * k / 32), 4), round(r * math.sin(2 * math.pi * k / 32), 4)] for k in range(32)]
        flank = [(0.0, 0.0)] + [(r, u) for u, r in rows] + [(0.0, rows[-1][0])]
        world = [((x, y, 0.0) if view == "elevation" else (0.0, y, x)) for x, y in flank[1:]] + [((-x, y, 0.0) if view == "elevation" else (0.0, y, -x)) for x, y in flank[::-1][1:]]
        return [[round(q[0], 4), round(q[1], 4)] for q in (K.view_xy(view, rig, bone, w) for w in world)]
    if run < 1.2 * part["width"]:
        return [[round(x, 4), round(y, 4)] for x, y in K.hull([q for r in rings for q in r])]
    left, right = [], []
    for i, c in enumerate(centres):
        a, b = centres[max(i - 1, 0)], centres[min(i + 1, len(centres) - 1)]
        l = math.hypot(b[0] - a[0], b[1] - a[1]) or 1.0
        nx, ny = -(b[1] - a[1]) / l, (b[0] - a[0]) / l
        ds = [(q[0] - c[0]) * nx + (q[1] - c[1]) * ny for q in rings[i]]
        left.append((c[0] + nx * max(ds), c[1] + ny * max(ds)))
        right.append((c[0] + nx * min(ds), c[1] + ny * min(ds)))
    tx, ty = centres[-1][0] - centres[-2][0], centres[-1][1] - centres[-2][1]
    l = math.hypot(tx, ty)
    reach = max((q[0] - centres[-1][0]) * tx / l + (q[1] - centres[-1][1]) * ty / l for q in rings[-1])
    nose = (centres[-1][0] + tx / l * max(reach, 0.3 * part["rx"][-1]), centres[-1][1] + ty / l * max(reach, 0.3 * part["rx"][-1]))
    return [[round(x, 4), round(y, 4)] for x, y in left + [nose] + right[::-1]]


def drawings(rig, plant, scale):
    """✏️ The three views of the scale: a pen stroke of the true width along every slender part, a clean outline of every mass, ordered by depth."""
    feature = 0.00015 * scale
    depth = {"elevation": lambda p: -p[2], "front": lambda p: p[0], "plan": lambda p: p[1]}
    out = {}
    for view in VIEWS:
        shapes = []
        for part in plant.parts:
            bone, region, look = part["bone"], part["region"], part["look"]
            centre = [part["capsule"][0], part["capsule"][1]] if "capsule" in part else part["points"]
            key = sum(depth[view](p) for p in centre) / len(centre)
            base = {"id": part["id"], "bone": bone, "role": look["role"] if scale <= 200 else "body", "z": 0, "tone": look["tone"], "region": region, "material": look["material"]}
            if part["slender"] or "capsule" in part:
                pen = round(min(max(part["width"] * 1000 / scale, 0.1), 0.5), 2)
                points = K.simplify([K.view_xy(view, rig, bone, p) for p in centre], 0.0002)
                if math.hypot(points[-1][0] - points[0][0], points[-1][1] - points[0][1]) < feature / 2 and len(points) == 2:
                    points = [points[0], (points[0][0] + feature / 2, points[0][1])]
                shapes.append((key, {**base, "filled": False, "penMm": pen, "kind": "poly", "points": [[round(x, 4), round(y, 4)] for x, y in points], "closed": False}))
            else:
                shapes.append((key, {**base, "filled": True, "kind": "poly", "points": outline(part, view, rig), "closed": True}))
        shapes.sort(key=lambda s: s[0])
        for rank, (_, shape) in enumerate(shapes):
            shape["z"] = round(2 + rank * 0.01, 2)
        out[view] = [shape for _, shape in shapes]
    return out


def main():
    document = json.loads(CATALOG.read_text(encoding="utf-8"))
    asset = document["asset"]
    for bone in asset["rig"]["bones"]:
        if bone["id"] == "root-plate":
            bone["lengthM"] = P.CROWN_LENGTH
            bone["mass"]["comX"] = round(P.CROWN_LENGTH / 2, 4)
        if bone["parent"] in P.LATERAL:
            keys = list(bone)
            ordered = {k: bone[k] for k in keys[: keys.index("y") + 1]}
            ordered["z"] = P.LATERAL[bone["parent"]]
            ordered.update({k: bone[k] for k in keys[keys.index("y") + 1 :] if k != "z"})
            bone.clear()
            bone.update(ordered)
        if bone["id"] in P.SHOOT_BASE:
            x, y, z = P.SHOOT_BASE[bone["id"]]
            keys = list(bone)
            bone["x"], bone["y"], bone["z"] = y, -x, z
            ordered = {k: bone[k] for k in keys[: keys.index("y") + 1]}
            ordered["z"] = z
            ordered.update({k: bone[k] for k in keys[keys.index("y") + 1 :] if k != "z"})
            bone.clear()
            bone.update(ordered)
        if bone["id"] in P.CULM_LENGTH:
            bone["lengthM"] = P.CULM_LENGTH[bone["id"]]
            bone["mass"]["comX"] = round(P.CULM_LENGTH[bone["id"]] / 2, 4)
        if bone["parent"] in P.CULM_LENGTH:
            bone["x"] = P.CULM_LENGTH[bone["parent"]]
        if bone["id"].startswith("culm") and "section" in bone:
            d = "1.2 cm" if bone["id"].endswith("a") else "8 mm"
            bone["section"]["title"] = {"en": f"Solid section, d = {d}", "de": f"Vollquerschnitt, d = {d.replace('.', ',')}"}
    rig = K.Rig(asset["rig"]["bones"])
    volumes, anchors_at = {}, {}
    for scale in (50, 100, 200, 500):
        plant = build(rig, scale)
        body = [solid_of(rig, part) for part in plant.parts]
        thermal = [{"id": f"th-{part['id']}", "of": part["id"], "region": part["region"]} for part in plant.parts]
        for index, part, row in sorted(plant.strips, key=lambda s: s[0]):
            solid, mid = strip_of(rig, plant, index, part, row, scale)
            thermal.append(solid)
            if scale == 50:
                anchors_at[index] = (part["bone"], mid)
        if scale == 500:
            for name in P.BUNDLES:
                axis = K.rod(rig, f"th-shoot-{name}", f"bl-{name}a", plant.stems[f"bl-{name}a"], [P.S500["shoot_axis_radius"]] * 5, STEM)
                axis.update({"slender": True, "region": "culms"})
                thermal.append(axis)
        structure = structure_of(rig, plant, scale)
        volumes[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": TEXTS["volumes"][str(scale)]}
        representation = asset["representations"][str(scale)]
        representation["geometry"] = drawings(rig, plant, scale)
        representation["decisions"] = TEXTS["representations"][str(scale)]
        area = sum(2 * sum(part["rx"]) / len(part["rx"]) * sum(K.norm(K.sub(b, a)) for a, b in zip(part["points"], part["points"][1:])) for part in plant.parts if part["region"] == "blades" and part["slender"])
        print(f"1:{scale} stems made straight {sum(1 for b in BOWS if b < 6)} curved {sum(1 for b in BOWS if b >= 6)} nearest curved ratio {min([b for b in BOWS if b >= 6], default=0):.1f} largest straightened {max([b for b in BOWS if b < 6], default=0):.1f}")
        BOWS.clear()
        print(f"1:{scale} body {len(body)} thermal {len(thermal)} structure {len(structure)} extent " + " ".join(f"{k} {v:.3f}" for k, v in plant.extent.items()) + f" one-sided blade area {area:.2f} m2 fit " + " ".join(f"{k} {v:.2f}" for k, v in plant.fit.items()))
    for anchor in asset["anchors"]:
        if anchor["id"].startswith("tp-blades-"):
            bone, mid = anchors_at[int(anchor["id"].rsplit("-", 1)[1])]
            q = rig.point(bone, mid)
            keys = [k for k in anchor if k != "z"]
            anchor.update({"bone": bone, "x": round(q[0], 4), "y": round(q[1], 4)})
            ordered = {k: anchor[k] for k in keys}
            ordered["z"] = round(q[2], 4)
            anchor.clear()
            anchor.update(ordered)
        if anchor["id"] == "base-support":
            anchor["footprintLengthM"], anchor["footprintWidthM"] = 0.41, 0.41
    for region in asset["thermal"]["regions"]:
        if region["id"] == "culms":
            region["areaM2"] = P.CULMS_AREA_M2
    asset["summary"] = TEXTS["summary"]
    asset["provenance"]["notes"] = TEXTS["notes"]
    references = [r for r in asset["provenance"]["references"] if r["id"] not in {t["id"] for t in TEXTS["references"]}]
    asset["provenance"]["references"] = references + TEXTS["references"]
    asset["volumes"] = volumes
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "🔣️.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    SOURCE.write_text(json.dumps({"volumes": volumes}, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
