"""🧰 PW-15 shared assembly of a grass or perennial asset: imports the round-3 miscanthus kit and model from their place (vectors, rig frames, lofts, plant parts, emitter strips) and adds what every PW-15 species needs around them: the planar rig from a shoot table, poses, clips, thermal model, structure list, symbols, representation frames and the manifest row.

Body and drawing are twins (`twin`, `drawings`): every part of a plant becomes one closed triangle mesh (`polyhedron`) through its stated sections, and its three drawn shapes are the outlines of that very mesh projected into the side elevation, the front and the plan (union of the projected triangles, shapely). No part is `slender` and no stroke stands for a body, so the catalog-silhouette guard compares the same polygons on both sides. The generators therefore run through `pw15-build.sh` in the `geometry` oracle environment of the staffage tests (shapely).

See: ../../📋️brief.md
See: ../../📝️PW-15.md
See: ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-vegetation-miscanthus-grass.md
"""
import importlib.util
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parents[1]
REPO = HERE.parents[8]
R3 = REPO / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
STAFFAGE = REPO / "🎓️teaching/🏛️architecture/🧍️staffage"
CATALOG = STAFFAGE / "🗂️catalog/🌳️vegetation"
OUT = TICKET / "🗑️generated/PW-15"
ROWS = TICKET / "📥️manifest-rows"
SCALES = (50, 100, 200, 500)
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


K = load(R3 / "r3-miscanthus-grass-kit.py", "r3_miscanthus_grass_kit")
M = load(R3 / "r3-miscanthus-grass-model.py", "r3_miscanthus_grass_model")


def t(en, de):
    return {"en": en, "de": de}


def r4(x):
    return round(x, 4)


class Plant(M.Plant):
    """🌱 The round-3 plant of parts with three additions: blades without the 1:22 width cap (broad reed and sunflower leaves), flat sheets (laminae, ray florets) with an optional roll of the lamina about its midrib, and the profile of the leaf passed per call."""

    def leaf(self, part_id, bone, start, d0, d1, length, power, width, thickness, profile, n, region, look, sheet=False, reach=False, floor=0.02, margin=None, roll=0.0):
        points, length = self.path(start, d0, d1, length, power, n, reach, width / 2 if margin is None else margin, floor)
        fallback = K.direction(90, math.degrees(math.atan2(d1[2], d1[0])) + 90)
        ts = K.tangents(points)
        rx = [max(width / 2 * M.profile(profile, i / (n - 1)), 2.5e-4) for i in range(n)]
        widths = [K.width_axis(q, fallback) for q in ts]
        if roll:
            c, r = math.cos(math.radians(roll)), math.sin(math.radians(roll))
            widths = [K.unit(K.add(K.mul(w, c), K.mul(K.cross(q, w), r))) for w, q in zip(widths, ts)]
        part = self.part(id=part_id, bone=bone, region=region, look=look, points=points, widths=widths, rx=rx, ry=[thickness / 2] * n, slender=not sheet, width=width, fallback=fallback)
        if sheet:
            part["sheet"] = True
        return part


def arc_length(points):
    return sum(K.norm(K.sub(b, a)) for a, b in zip(points, points[1:]))


def part_area(part):
    """📐 The outer area of a loft part: the mean ring perimeter times the path length (Ramanujan perimeter of the elliptic sections)."""
    if "capsule" in part:
        a, b, r = part["capsule"]
        return 2 * math.pi * r * K.norm(K.sub(b, a)) + 4 * math.pi * r * r
    if "dome" in part:
        rows = part["dome"]
        return sum(math.pi * (ra + rb) * math.hypot(ub - ua, rb - ra) for (ua, ra), (ub, rb) in zip(rows, rows[1:])) + math.pi * rows[0][1] ** 2
    total = 0.0
    for i in range(len(part["points"]) - 1):
        seg = K.norm(K.sub(part["points"][i + 1], part["points"][i]))
        a = (part["rx"][i] + part["rx"][i + 1]) / 2
        b = (part["ry"][i] + part["ry"][i + 1]) / 2
        h = ((a - b) / (a + b)) ** 2
        total += math.pi * (a + b) * (1 + 3 * h / (10 + math.sqrt(4 - 3 * h))) * seg
    return total


SIDES = {"sheet": {50: 4, 100: 4, 200: 4, 500: 4}, "round": {50: 8, 100: 8, 200: 8, 500: 8}, "mass": {50: 16, 100: 12, 200: 12, 500: 8}, "dome": {50: 32, 100: 24, 200: 20, 500: 16}}
PEN_FINE = 0.2


def kind_of(part):
    """🏷️ The mesh class of a part: a flat `sheet` (leaf blade, ray floret), a `round` stem, a `dome` of revolution or a `mass`."""
    if "dome" in part:
        return "dome"
    if "capsule" in part:
        return "round"
    if max(part["ry"]) <= 0.2 * max(part["rx"]):
        return "sheet"
    return "round" if part.get("slender") or max(part["rx"]) <= 0.02 else "mass"


def rings_of(part, scale):
    """⭕ The section rings of a part in world space: `sides` points per station on the superellipse of the station (a flat sheet is a rhombus of its width and thickness), the section normal along the path tangent; a capsule gets a shoulder ring at each end.

    A round stem is an octagonal prism set so that the side elevation looks at a flat of it (the first corner 22.5 degrees off the view) and 3 % narrower across than along that corner: its principal extents are then its two corner diameters, never less than the width any view draws, so `volumes-feature` reads the stem as thick as its drawing shows it whatever its lean."""
    sides = part.get("sides", {}).get(scale) if isinstance(part.get("sides"), dict) else part.get("sides")
    sides = sides or SIDES[kind_of(part)][scale]
    if "capsule" in part:
        a, b, r = part["capsule"]
        d = K.unit(K.sub(b, a))
        c = K.cross(d, (0.0, 0.0, 1.0))
        x = K.unit(c) if K.norm(c) > 0.2 else (1.0, 0.0, 0.0)
        stations = [(K.sub(a, K.mul(d, 0.75 * r)), d, x, 0.66 * r, 0.66 * r), (a, d, x, r, r), (b, d, x, r, r), (K.add(b, K.mul(d, 0.75 * r)), d, x, 0.66 * r, 0.66 * r)]
        exponent = 2.0
    else:
        ts = K.tangents(part["points"])
        stations = [(p, q, x, max(a, 2e-4), max(b, 2e-4)) for p, q, x, a, b in zip(part["points"], ts, part["widths"], part["rx"], part["ry"])]
        exponent = part.get("exponent", 2.0)
    e = 2.0 / exponent
    rings = []
    before = None
    stem = kind_of(part) == "round"
    for p, q, x, a, b in stations:
        if stem:
            flat = K.cross((0.0, 0.0, 1.0), q)
            flat = K.unit(flat) if K.norm(flat) > 0.2 else K.unit(K.cross((1.0, 0.0, 0.0), q))
            x = K.add(K.mul(flat, math.cos(math.pi / 8)), K.mul(K.cross(q, flat), math.sin(math.pi / 8)))
            b = 0.97 * a
        x = K.unit(K.sub(x, K.mul(q, K.dot(x, q))))
        if before is not None and K.dot(x, before) < 0:
            x = K.mul(x, -1)
        before = x
        y = K.cross(q, x)
        ring = []
        for k in range(sides):
            angle = 2 * math.pi * k / sides
            c, s = math.cos(angle), math.sin(angle)
            c, s = (0.0 if abs(c) < 1e-12 else c), (0.0 if abs(s) < 1e-12 else s)
            ring.append(K.add(p, K.add(K.mul(x, a * math.copysign(abs(c) ** e, c)), K.mul(y, b * math.copysign(abs(s) ** e, s)))))
        rings.append(ring)
    return rings


def world_of(rig, bone, q):
    u, v, w = rig.axes(bone)
    o = rig.origin(bone)
    return (o[0] + q[0] * u[0] + q[1] * v[0] + q[2] * w[0], o[1] + q[0] * u[1] + q[1] * v[1] + q[2] * w[1], o[2] + q[0] * u[2] + q[1] * v[2] + q[2] * w[2])


def signed_volume(vertices, triangles):
    return sum(K.dot(vertices[a], K.cross(vertices[b], vertices[c])) for a, b, c in triangles) / 6


def principal_extents(vertices):
    """📦 The three extents of a vertex cloud along its principal axes, ascending (the measure `volumes-feature` reads from a polyhedron)."""
    n = len(vertices)
    mean = [sum(q[i] for q in vertices) / n for i in range(3)]
    cov = [[sum((q[i] - mean[i]) * (q[j] - mean[j]) for q in vertices) for j in range(3)] for i in range(3)]
    axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    for _ in range(60):
        changed = False
        for a, b in ((0, 1), (0, 2), (1, 2)):
            off = cov[a][b]
            if abs(off) <= 1e-14 * (cov[0][0] + cov[1][1] + cov[2][2]):
                continue
            changed = True
            delta = (cov[b][b] - cov[a][a]) / (2 * off)
            tangent = (-1 if delta < 0 else 1) / (abs(delta) + math.hypot(1, delta))
            cosine = 1 / math.hypot(1, tangent)
            sine = tangent * cosine
            cov[a][a] -= tangent * off
            cov[b][b] += tangent * off
            cov[a][b] = cov[b][a] = 0.0
            k = 3 - a - b
            p, q = cov[k][a], cov[k][b]
            cov[a][k] = cov[k][a] = cosine * p - sine * q
            cov[b][k] = cov[k][b] = sine * p + cosine * q
            for i in range(3):
                p, q = axes[a][i], axes[b][i]
                axes[a][i], axes[b][i] = cosine * p - sine * q, sine * p + cosine * q
        if not changed:
            break
    return sorted(max(K.dot(q, ax) for q in vertices) - min(K.dot(q, ax) for q in vertices) for ax in axes)


def twin(rig, part, scale):
    """👯 A part as its body solid and the source of its drawings at once: a closed, outward triangle mesh through the rings of the part (`polyhedron`, vertices bone-local and rounded to 0.01 mm) and the same rounded vertices carried back to world space, so that the three views are projections of exactly the mesh the kernel reads. The two faces of a flat sheet are split along mirrored diagonals, so a twisted blade never cuts itself."""
    rings = rings_of(part, scale)
    m, n = len(rings[0]), len(rings)
    local = [tuple(rig.point(part["bone"], q)) for ring in rings for q in ring]
    assert len(set(local)) == len(local), f"{part['id']}: vertices coincide after rounding"
    triangles = []
    for i in range(n - 1):
        for k in range(m):
            a, b, c, d = i * m + k, i * m + (k + 1) % m, (i + 1) * m + (k + 1) % m, (i + 1) * m + k
            triangles += [(a, b, d), (b, c, d)] if m == 4 and k % 2 else [(a, b, c), (a, c, d)]
    end = (n - 1) * m
    for k in range(1, m - 1):
        triangles += [(0, k + 1, k), (end, end + k, end + k + 1)]
    if signed_volume(local, triangles) < 0:
        triangles = [(a, c, b) for a, b, c in triangles]
    assert signed_volume(local, triangles) > 0, f"{part['id']}: no volume"
    for a, b, c in triangles:
        assert K.norm(K.cross(K.sub(local[b], local[a]), K.sub(local[c], local[a]))) > 1e-12, f"{part['id']}: degenerate triangle"
    solid = {"id": part["id"], "kind": "polyhedron", "bone": part["bone"], **part["look"], "vertices": [list(q) for q in local], "triangles": [list(q) for q in triangles]}
    if kind_of(part) == "sheet":
        thickness, middle, _ = principal_extents(local)
        if middle >= 21 * thickness:
            solid["sheet"] = True
    if part.get("envelope"):
        solid["envelope"] = True
    return {"solid": solid, "local": local, "world": [world_of(rig, part["bone"], q) for q in local], "triangles": triangles}


def twins(rig, plant):
    if not hasattr(plant, "twins"):
        plant.twins = [twin(rig, part, plant.scale) for part in plant.parts]
    return plant.twins


def extent(plant):
    """📦 The extent of the body of a plant: of the vertices of its meshes."""
    pts = [q for tw in twins(plant.rig, plant) for q in tw["world"]]
    return {"x-": min(q[0] for q in pts), "x+": max(q[0] for q in pts), "y-": min(q[1] for q in pts), "y+": max(q[1] for q in pts), "z-": min(q[2] for q in pts), "z+": max(q[2] for q in pts)}


def simple_polygons(geometry):
    """🧩 The simple polygons (no holes) of a union: specks and pinholes of the arithmetic are dropped, a real hole (a ribbon that loops in the view) is opened by cutting the polygon through it."""
    from shapely.geometry import LineString, Polygon
    from shapely.ops import split
    out = []
    for polygon in getattr(geometry, "geoms", [geometry]):
        if polygon.geom_type != "Polygon" or polygon.area < 1e-9:
            continue
        holes = [ring for ring in polygon.interiors if Polygon(ring).area >= 1e-8]
        if not holes:
            out.append(Polygon(polygon.exterior))
            continue
        x = Polygon(holes[0]).representative_point().x
        y0, y1 = polygon.bounds[1] - 1, polygon.bounds[3] + 1
        out += simple_polygons(split(Polygon(polygon.exterior, holes), LineString([(x, y0), (x, y1)])))
    return out


def outlines(points, triangles):
    """🖼️ The outline of a projected mesh: the union of its projected triangles as simple polygons, coordinates to 0.1 micrometres."""
    from shapely import union_all
    from shapely.geometry import Polygon
    faces = []
    for a, b, c in triangles:
        pa, pb, pc = points[a], points[b], points[c]
        if abs((pb[0] - pa[0]) * (pc[1] - pa[1]) - (pb[1] - pa[1]) * (pc[0] - pa[0])) > 1e-13:
            faces.append(Polygon([pa, pb, pc]))
    if not faces:
        return []
    out = []
    for polygon in simple_polygons(union_all(faces)):
        ring = []
        for x, y in list(polygon.exterior.coords)[:-1]:
            q = [round(x, 7), round(y, 7)]
            if not ring or ring[-1] != q:
                ring.append(q)
        if len(ring) > 1 and ring[0] == ring[-1]:
            ring.pop()
        if len(ring) >= 3:
            out.append(ring)
    return out


def drawings(rig, plant, scale, rename, pens):
    """✏️ The three views of a scale: every part is one filled outline, the exact projection of its own mesh (side elevation in the frame of its bone, front and plan in rest-pose model metres), ordered by depth. A leaf seen edge-on is the thin outline of its thickness; its pen carries it on paper. No stroke stands for a body, so the drawing and the volume are the same polygons."""
    depth = {"elevation": lambda p: -p[2], "front": lambda p: p[0], "plan": lambda p: p[1]}
    flatten = {"front": lambda p: (p[2], p[1]), "plan": lambda p: (p[0], p[2])}
    out = {}
    for view in M.VIEWS:
        shapes = []
        for part, tw in zip(plant.parts, twins(rig, plant)):
            look = part["look"]
            role = look["role"] if scale <= 200 else "body"
            points = [(q[0], q[1]) for q in tw["local"]] if view == "elevation" else [flatten[view](q) for q in tw["world"]]
            key = sum(depth[view](p) for p in tw["world"]) / len(tw["world"])
            fine = kind_of(part) in ("sheet", "round")
            for index, ring in enumerate(outlines(points, tw["triangles"])):
                shape = {"id": part["id"] if index == 0 else f"{part['id']}-p{index + 1}", "bone": part["bone"], "role": role, "z": 0, "tone": look["tone"], "region": part["region"], "material": rename.get(look["material"], look["material"]), "filled": True}
                if fine and pens[role] != PEN_FINE:
                    shape["penMm"] = PEN_FINE
                shape.update({"kind": "poly", "points": ring, "closed": True})
                shapes.append((key, shape))
        shapes.sort(key=lambda s: s[0])
        for rank, (_, shape) in enumerate(shapes):
            shape["z"] = round(2 + rank * 0.01, 2)
        out[view] = [shape for _, shape in shapes]
    return out


def structure(rig, plant, scale, spec):
    """🏗️ The load path of one scale: the root mat fixed in the ground, a rod of the declared section along every bone path of `plant.stems` (slender unless it is as thick as the smallest feature of the scale), joints at 1:50 and 1:100, the support disc and the lumped masses (crown and heads or panicles)."""
    look = {"material": spec["stem_material"], "role": "body", "tone": 0.5}
    out = []
    mat = spec["mat"][scale]
    out.append({**K.loft(rig, "member-root-plate", "root-plate", [(0.0, u, 0.0) for u, _ in mat], [(1.0, 0.0, 0.0)] * len(mat), [r for _, r in mat], [r for _, r in mat], look), "member": "root-plate", "part": "member"})
    take = {50: 1, 100: 2, 200: 2, 500: 3}[scale]
    for bone, path in plant.stems.items():
        r = spec["member_radius"](bone, scale)
        points = path[::take] if (len(path) - 1) % take == 0 else path[::take] + [path[-1]]
        if scale == 500 or len(points) < 2:
            points = [path[0], path[len(path) // 2], path[-1]] if spec.get("member_points", {}).get(scale, 3) == 3 else [path[0], path[-1]]
        member = K.rod(rig, f"member-{bone}", bone, points, [r] * len(points), look)
        member.update({"slender": True, "member": bone, "part": "member"})
        if r >= 0.5 * 0.00015 * scale:
            del member["slender"]
        out.append(member)
    if scale <= 100:
        for bone in plant.stems:
            j = spec["joint_radius"][scale]
            j = j(bone) if callable(j) else j
            out.append({"id": f"joint-{bone}", "kind": "ellipsoid", "bone": bone, "material": spec["stem_material"], "role": "detail", "tone": 0.35, "radii": [j, j, j], "member": bone, "part": "joint"})
    thick = {50: 0.02, 100: 0.02, 200: 0.03, 500: 0.075}[scale]
    sr = spec["support_radius"]
    plate = K.loft(rig, "support-base-support", "root-plate", [(0.0, 0.0, 0.0), (0.0, thick + 0.001, 0.0)], [(1.0, 0.0, 0.0)] * 2, [sr, sr], [sr, sr], {"material": spec["mat_material"], "role": "detail", "tone": 0.25})
    out.append({**plate, "member": "root-plate", "part": "support", "anchor": "base-support"})
    m = spec["crown_mass_radii"][scale]
    out.append({"id": "mass-root-plate", "kind": "ellipsoid", "bone": "root-plate", "material": spec["mat_material"], "role": "detail", "tone": 0.4, "radii": m, "transform": {"translation": [spec["crown_length"] / 2, 0, 0]}, "member": "root-plate", "part": "mass"})
    for bone, (share, radius) in spec["masses"](scale).items():
        out.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": spec["top_material"], "role": "detail", "tone": 0.4, "radii": [radius] * 3, "transform": {"translation": rig.point(bone, K.along(plant.stems[bone], share))}, "member": bone, "part": "mass"})
    return out


def rig_bones(spec):
    """🦴 The planar rig: `root-plate` along the vertical, each shoot base on the crown at its world place, every distal bone at the tip of its parent with the lateral gain of the parent as `z`."""
    bones = [{"id": "root-plate", "parent": None, "title": spec["root_title"], "x": 0, "y": 0, "angleDeg": 90, "lengthM": spec["crown_length"], "joint": {"type": "fixed"}, "mass": mass(spec["root_mass"], spec["crown_length"] / 2, "uniform", spec), "section": spec["root_section"]}]
    for row in spec["bones"]:
        bone = {"id": row["id"], "parent": row["parent"], "title": row["title"]}
        if row["parent"] == "root-plate":
            x, y, z = row["base"]
            bone.update({"x": y, "y": r4(-x), "z": z})
        else:
            parent = next(b for b in spec["bones"] if b["id"] == row["parent"])
            bone.update({"x": parent["length"], "y": 0, "z": spec["lateral"][row["parent"]]})
        bone.update({"angleDeg": row["angle"], "lengthM": row["length"], "joint": {"type": "revolute", "minDeg": -row["limit"], "maxDeg": row["limit"]}, "mass": mass(row["kg"], row["length"] * row.get("com", 0.5), row.get("distribution", "uniform"), spec)})
        if "section" in row:
            bone["section"] = row["section"]
        bones.append(bone)
    return bones


def mass(kg, com, distribution, spec):
    return {"kg": kg, "comX": r4(com), "comY": 0, "distribution": distribution, "source": "authored-estimate", "reference": spec["mass_reference"]}


def circle_section(d, material, en, de):
    """⭕ A solid circular section of diameter d (area, second moment, extreme fibre)."""
    return {"areaM2": round(math.pi * d * d / 4, 7), "secondMomentM4": float(f"{math.pi * d ** 4 / 64:.4g}"), "extremeFibreM": r4(d / 2), "material": material, "title": t(en, de)}


def wind_force(area, cd, v):
    """💨 Drag 0.5 rho cd A v^2 with rho 1.25 kg/m3 (EN 1991-1-4)."""
    return 0.5 * 1.25 * cd * area * v * v


def pose(pid, title, description, bones, thermal, loads, key=False):
    out = {"id": pid, "title": title, "description": description}
    if key:
        out["key"] = True
    out.update({"bones": {b: {"angleDeg": round(a, 2)} for b, a in bones.items() if abs(a) > 1e-9}, "ground": "authored", "contacts": [], "thermal": thermal, "loads": loads})
    return out


def sway_clip(spec, windy_bones, amplitude, phase_of):
    """🌬️ The looping wind sway: eight keys over one period about the fresh-breeze lean, each bone swinging by its amplitude with a phase that travels through the stand."""
    period = spec["sway_period"]
    keys = []
    for k in range(9):
        tk = period * k / 8
        bones = {b: {"angleDeg": round(a + amplitude[b] * math.sin(2 * math.pi * k / 8 + phase_of[b]), 2)} for b, a in windy_bones.items()}
        keys.append({"t": round(tk, 4), "pose": "windy", "ease": "smooth", "bones": bones})
    return {"id": "wind-sway", "title": t("Wind sway", "Windschwingung"), "description": spec["sway_text"], "durationS": period, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "sway", "fromS": 0, "toS": period, "title": t("Sway cycle", "Schwingungszyklus")}], "events": [{"t": 0, "kind": "motion", "title": t("Cycle start at mean lean", "Zyklusbeginn in mittlerer Neigung")}], "tracks": [{"target": f"emitter:{spec['emitter']}", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": round(period / 2, 4), "v": 0.9}, {"t": period, "v": 1}]}]}


def gust_clip(spec, storm_bones, wind_loads):
    """🌪️ One gust of 8 s: the wind-load tracks rise to 2.75 times the fresh-breeze load (peak pressure with Iv 0.25) and decay; the bones lean up to 70 % of the gale lean."""
    shape = [(0, 0.05), (1.2, 0.3), (2.5, 1.0), (3.6, 0.65), (5, 0.2), (8, 0.05)]
    keys = [{"t": tk, "pose": "windy", "ease": "smooth", "bones": {b: {"angleDeg": round(0.7 * a * s, 2)} for b, a in storm_bones.items()}} for tk, s in shape]
    load_shape = [(0, 0.05), (1.2, 0.45), (2.5, 2.75), (3.6, 1.4), (5, 0.5), (8, 0.05)]
    tracks = [{"target": f"load:{lid}", "ease": "smooth", "keys": [{"t": tk, "v": v} for tk, v in load_shape]} for lid in wind_loads]
    return {"id": "gust", "title": t("Gust", "Windböe"), "description": spec["gust_text"], "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "rise", "fromS": 0, "toS": 2.5, "title": t("Gust front", "Böenfront")}, {"id": "peak", "fromS": 2.5, "toS": 3.6, "title": t("Peak", "Spitze")}, {"id": "decay", "fromS": 3.6, "toS": 8, "title": t("Decay and recovery", "Abklingen und Erholung")}], "events": [{"t": 2.5, "kind": "load-on", "title": t("Peak gust load", "Böenspitze")}, {"t": 5, "kind": "load-off", "title": t("Load below the mean again", "Last wieder unter dem Mittelwert")}], "tracks": tracks}


def diurnal_clip(spec, sunlit, extra_keys=None, extra_tracks=()):
    """🌞 A day of 24 h compressed to 24 s: the transpiration strength and the sunlit fraction of every region follow the sun."""
    sun = [(0, 0), (6, 0), (8, 0.43), (10, 0.79), (12, 1.0), (14, 0.79), (16, 0.36), (18, 0), (24, 0)]
    tracks = [{"target": f"emitter:{spec['emitter']}", "ease": "smooth", "keys": [{"t": a, "v": b} for a, b in [(0, 0.03), (5, 0.05), (7, 0.3), (9, 0.7), (12, 1), (15, 0.9), (18, 0.35), (20, 0.08), (24, 0.03)]]}]
    for region, peak in sunlit.items():
        tracks.append({"target": f"region:{region}:sunlitFraction", "ease": "smooth", "keys": [{"t": a, "v": round(peak * s, 3)} for a, s in sun]})
    keys = extra_keys or [{"t": 0, "pose": "calm", "ease": "smooth"}, {"t": 12, "pose": "calm", "ease": "smooth"}, {"t": 24, "pose": "calm", "ease": "smooth"}]
    return {"id": "diurnal-transpiration", "title": t("Day of transpiration", "Transpirationstag"), "description": spec["diurnal_text"], "durationS": 24, "fps": 10, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [], "phases": [{"id": "night", "fromS": 0, "toS": 6, "title": t("Night", "Nacht")}, {"id": "morning", "fromS": 6, "toS": 12, "title": t("Morning to noon", "Morgen bis Mittag")}, {"id": "afternoon", "fromS": 12, "toS": 18, "title": t("Afternoon", "Nachmittag")}, {"id": "evening", "fromS": 18, "toS": 24, "title": t("Evening and night", "Abend und Nacht")}], "events": [{"t": 6, "kind": "thermal", "title": t("Sunrise", "Sonnenaufgang")}, {"t": 12, "kind": "thermal", "title": t("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration")}, {"t": 18, "kind": "thermal", "title": t("Sunset", "Sonnenuntergang")}], "tracks": tracks + list(extra_tracks)}


def lobed(cx, cy, rx, ry, n, amp, phase, count=240, floor=None):
    pts = []
    for i in range(count):
        a = 2 * math.pi * i / count
        r = 1 - amp + amp * abs(math.sin(n * a / 2 + phase)) ** 0.7
        x, y = cx + rx * r * math.cos(a), cy + ry * r * math.sin(a)
        if floor is not None:
            y = max(y, floor)
        pts.append([r4(x), r4(y)])
    return pts


def symbol(spec, extent):
    """🔣 The true-extent plan sign in all three views: a lobed outline of the measured 1:50 body extent and the stem sections at their real places and thickness."""
    ex0, ex1, ey1, ez0, ez1 = extent["x-"], extent["x+"], extent["y+"], extent["z-"], extent["z+"]
    cx, cz = (ex0 + ex1) / 2, (ez0 + ez1) / 2
    n, amp = spec["symbol_lobes"]
    def shape(sid, role, z, tone, points):
        return {"id": sid, "bone": "root-plate", "role": role, "z": z, "filled": False, "tone": tone, "kind": "poly", "points": points, "closed": True}
    el = [shape("crown", "body", 2, 0.5, lobed(cx, 0, (ex1 - ex0) / 2, ey1, n, amp, 0.4, floor=0.0))]
    fr = [shape("crown", "body", 2, 0.5, lobed(cz, 0, (ez1 - ez0) / 2, ey1, n + 2, amp, 1.3, floor=0.0))]
    pl = [shape("crown", "body", 2, 0.5, lobed(cx, cz, (ex1 - ex0) / 2, (ez1 - ez0) / 2, n + 1, amp, 2.1))]
    for i, (x, z, r) in enumerate(spec["symbol_stems"]):
        pl.append(shape(f"stem-{i + 1}", "detail", 3, 0.4, [[r4(x + r * math.cos(2 * math.pi * k / 24)), r4(z + r * math.sin(2 * math.pi * k / 24))] for k in range(24)]))
    w, d, h = ex1 - ex0, ez1 - ez0, ey1
    return {"id": f"{spec['id']}-symbol", "title": t("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), "description": t("A conventional plan sign of the plant for site and landscape plans: outline of the true extent and the stem sections. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der Pflanze für Lage- und Freiraumpläne: Umriss der wahren Ausdehnung und die Stängelquerschnitte. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl}, "dimensionalNote": t(f"A conventional simplification of the plant drawn at its true extent in model metres in all three views ({fmt(w)} m wide, {fmt(d)} m deep and {fmt(h)} m high): the outline is the extent of the leaves and flowers, the stems are drawn at their real position and thickness. It is a symbol, never used silently for a pictorial scale.", f"Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten ({fmt(w, 'de')} m breit, {fmt(d, 'de')} m tief und {fmt(h, 'de')} m hoch): Der Umriss ist die Ausdehnung von Blättern und Blüten, die Stängel sind in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.")}


def fmt(x, language="en"):
    s = f"{x:.2f}".rstrip("0").rstrip(".")
    return s.replace(".", ",") if language == "de" else s


PENS = {50: {"body": 0.25, "detail": 0.2, "texture": 0.13, "glazing": 0.1, "ground": 0.35}, 100: {"body": 0.2, "detail": 0.2, "texture": 0.1, "glazing": None, "ground": 0.3}, 200: {"body": 0.2, "detail": 0.2, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}


def build_asset(spec, builders):
    """📦 Builds the whole asset document of one species: rig, the plant of every scale (its solids, emitter strips, structure and three drawings), anchors on the 1:50 strips, symbols, poses, clips, thermal and texts; returns the document and the per-scale report."""
    bones = rig_bones(spec)
    rig = K.Rig(bones)
    M.TARGET = spec["target"]
    M.P = spec["plant_module"]
    rename = spec["rename"]
    volumes, representations, anchors_at, report = {}, {}, {}, {}
    for scale in SCALES:
        plant = builders[scale](rig)
        plant.extent = extent(plant)
        body = [tw["solid"] for tw in twins(rig, plant)]
        thermal = [{"id": f"th-{part['id']}", "of": part["id"], "region": part["region"]} for part in plant.parts]
        for index, part, row in sorted(plant.strips, key=lambda s: s[0]):
            strip, mid = M.strip_of(rig, plant, index, part, row, scale)
            strip["material"] = spec["leaf_material"]
            strip["region"] = spec["leaf_region"]
            strip["tags"] = ["emitter", spec["emitter"]]
            thermal.append(strip)
            if scale == 50:
                anchors_at[index] = (part["bone"], mid)
        for s in body:
            if s.get("material") in rename:
                s["material"] = rename[s["material"]]
        struct = structure(rig, plant, scale, spec)
        volumes[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": struct, "decisions": spec["volume_decisions"][scale]}
        lo, hi = spec.get("ranges", RANGES)[scale]
        representations[str(scale)] = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": drawings(rig, plant, scale, rename, PENS[scale]), "decisions": spec["representation_decisions"][scale]}
        areas = {}
        for part in plant.parts:
            areas[part["region"]] = areas.get(part["region"], 0.0) + part_area(part)
        one_sided = sum(part_area(p) / 2 for p in plant.parts if p["region"] == spec["leaf_region"])
        report[scale] = {"body": len(body), "triangles": sum(len(s["triangles"]) for s in body), "sheets": sum(1 for s in body if s.get("sheet")), "thermal": len(thermal), "structure": len(struct), "extent": {k: round(v, 4) for k, v in plant.extent.items()}, "areas": {k: round(v, 3) for k, v in areas.items()}, "one_sided_leaf_m2": round(one_sided, 3)}
        if scale == 50:
            symbol_extent = plant.extent
    anchors = [{"id": "base-support", "bone": "root-plate", "x": 0, "y": 0, "kind": "support", "title": spec["support_title"], "support": "fixed", "normalDeg": 90, "footprintLengthM": spec["footprint"], "footprintWidthM": spec["footprint"], "frictionCoefficient": 0.6}]
    for index in sorted(anchors_at):
        bone, mid = anchors_at[index]
        q = rig.point(bone, mid)
        anchors.append({"id": f"tp-{spec['leaf_region']}-{index}", "bone": bone, "x": r4(q[0]), "y": r4(q[1]), "kind": "emitter", "title": t(f"{spec['emitter_title']['en']}, anchor {index}", f"{spec['emitter_title']['de']}, Anker {index}"), "z": r4(q[2])})
    asset = {
        "id": spec["id"], "category": "vegetation", "rig": {"kind": "botanical", "root": "root-plate", "bones": bones}, "anchors": anchors,
        "title": spec["title"], "summary": spec["summary"], "tags": spec["tags"], "provenance": spec["provenance"], "dimensions": spec["dimensions"], "materials": spec["materials"],
        "poses": spec["poses"], "clips": spec["clips"], "thermal": spec["thermal"](report[50], [a["id"] for a in anchors[1:]]), "structure": spec["structure"],
        "volumes": volumes, "presentation": {"defaultPose": "calm", "defaultClip": "wind-sway", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"], "completion": "authored",
        "representations": representations, "symbols": [symbol(spec, symbol_extent)],
    }
    total = spec["root_mass"] + sum(row["kg"] for row in spec["bones"])
    assert abs(total - spec["dimensions"]["massKg"]) < 1e-6, f"bone masses {total} != massKg {spec['dimensions']['massKg']}"
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, report


def manifest_row(document, distinction):
    a = document["asset"]
    return {"id": a["id"], "category": a["category"], "title": a["title"], "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": 0, "poses": [p["id"] for p in a["poses"] if not p.get("key")], "clips": [c["id"] for c in a["clips"]], "representations": {k: v["status"] for k, v in a["representations"].items()}, "symbols": [s["id"] for s in a["symbols"]], "analytics": ["thermal", "transport", "structural"], "exports": a["exports"], "distinction": distinction, "completion": a["completion"]}


def write(document, report, distinction):
    """💾 Writes the asset document and its manifest row; with `PW15_CANDIDATE=1` in the environment the document goes to `🗑️generated/PW-15/cand/<id>/` and the catalog and the row stay untouched. Prints the per-scale counts and the body box against `dimensions` (limit: one smallest feature of the scale)."""
    import os
    a = document["asset"]
    OUT.mkdir(parents=True, exist_ok=True)
    candidate = os.environ.get("PW15_CANDIDATE") == "1"
    folder = (OUT / "cand" if candidate else CATALOG) / a["id"]
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "🔣️.json").write_text(json.dumps(document, ensure_ascii=False, separators=(",", ":")) + "\n", encoding="utf-8")
    if not candidate:
        ROWS.mkdir(parents=True, exist_ok=True)
        (ROWS / f"{a['id']}.json").write_text(json.dumps(manifest_row(document, distinction), ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (OUT / f"{a['id']}-report.json").write_text(json.dumps(report, indent=1) + "\n", encoding="utf-8")
    d = a["dimensions"]
    previous = None
    for scale, r in report.items():
        e = r["extent"]
        box = {"widthM": e["x+"] - e["x-"], "heightM": e["y+"] - e["y-"], "depthM": e["z+"] - e["z-"]}
        off = {k: round((v - d[k]) * 1000, 1) for k, v in box.items()}
        bad = [k for k, v in off.items() if abs(v) > 0.15 * scale]
        grows = previous is not None and (r["body"] > previous["body"] or r["triangles"] > previous["triangles"] or (r["body"] == previous["body"] and r["triangles"] == previous["triangles"]))
        print(f"[DEBUG] {a['id']} 1:{scale} body {r['body']} triangles {r['triangles']} sheets {r['sheets']} thermal {r['thermal']} structure {r['structure']} box off mm {off}{' BOX ✗ ' + str(bad) if bad else ''}{' NOT SIMPLER ✗' if grows else ''} areas {r['areas']} leaf one-sided {r['one_sided_leaf_m2']}")
        previous = r
    print(f"[DEBUG] {a['id']} written to {folder} ({'candidate' if candidate else 'catalog and row'})")
