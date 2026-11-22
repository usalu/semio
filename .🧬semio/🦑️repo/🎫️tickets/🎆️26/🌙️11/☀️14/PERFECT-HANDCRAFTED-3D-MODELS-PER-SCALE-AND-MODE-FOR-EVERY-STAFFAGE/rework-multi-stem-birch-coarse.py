"""🌿 Multi-stem birch, coarse scales by hand: the side-shoot sleeves that carry the hanging tresses at 1:500, the one loose side shoot at 1:100 and the thin branch tip at 1:500. Edits the volumes source in place; every step skips what is already done."""
import json
import math
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "vegetation", "multi-stem-birch.json")
SLEEVE_MIN_ACROSS = 0.1
NECK_INTO = 0.05
TIP_MIN_ACROSS = 0.056


def dist(a, b):
    return math.dist(a, b)


def centre(ring):
    return [sum(p[i] for p in ring) / len(ring) for i in range(3)]


def rounded(p):
    return [round(c, 8) for c in p]


def samples(solid):
    """🔬 Surface points of a polyhedron: its vertices and seven points per triangle."""
    v = solid["vertices"]
    out = [list(p) for p in v]
    for a, b, c in solid["triangles"]:
        for s, t in ((1 / 3, 1 / 3), (0.6, 0.2), (0.2, 0.6), (0.2, 0.2), (0.5, 0.5), (0, 0.5), (0.5, 0)):
            out.append([v[a][i] * (1 - s - t) + v[b][i] * s + v[c][i] * t for i in range(3)])
    return out


def nearest(points, cloud):
    best = (1e9, None, None)
    for p in points:
        for q in cloud:
            d = dist(p, q)
            if d < best[0]:
                best = (d, p, q)
    return best


def widen(ring, least):
    """🫧 A ring whose thinnest diameter is at least `least`, grown about its centre."""
    c = centre(ring)
    half = len(ring) // 2
    thin = min(dist(ring[i], ring[i + half]) for i in range(half))
    k = max(1.0, least / thin) if thin > 1e-6 else 1.0
    return [[c[i] + (p[i] - c[i]) * k for i in range(3)] for p in ring]


def coarse_sleeve(fine, like, new_id):
    """🍃 The side-shoot sleeve of 1:200 restated for 1:500: five sections instead of six, the three inner ones at least a feature and a third across."""
    v = fine["vertices"]
    rings = [v[i * 6:i * 6 + 6] for i in range(6)]
    mid = [[(rings[2][k][i] + rings[3][k][i]) / 2 for i in range(3)] for k in range(6)]
    five = [rings[0], widen(rings[1], SLEEVE_MIN_ACROSS), widen(mid, SLEEVE_MIN_ACROSS), widen(rings[4], SLEEVE_MIN_ACROSS), rings[5]]
    vertices = [rounded(p) for ring in five for p in ring] + [rounded(v[36]), rounded(v[37])]
    return {"id": new_id, "kind": "polyhedron", "bone": fine["bone"], "material": fine["material"], "role": fine["role"], "tone": fine["tone"], "vertices": vertices, "triangles": like["triangles"]}


def neck(solid, onto):
    """🪢 Carries the starting section of a spindle into the solid that bears it; returns the gap it found."""
    v = solid["vertices"]
    cap = len(v) - 2
    d, p, q = nearest(samples(solid), samples(onto))
    if d < 1e-4:
        return d
    shift = [(q[i] - p[i]) * (1 + NECK_INTO / d) for i in range(3)]
    for k in list(range(6)) + [cap]:
        v[k] = rounded([v[k][i] + shift[i] for i in range(3)])
    return d


def swell(solid, least):
    """🌱 Thickens a twig about its own centre line (bone-local x is its run) until its widest section is `least` across in its thinner direction."""
    v = solid["vertices"]
    x0, x1 = min(p[0] for p in v), max(p[0] for p in v)
    bins = 12
    cells = [[] for _ in range(bins)]
    for p in v:
        cells[min(bins - 1, int((p[0] - x0) / (x1 - x0) * bins))].append(p)
    cells = [c for c in cells if c]
    line = [centre(c) for c in cells]
    across = max(min(max(p[i] for p in c) - min(p[i] for p in c) for i in (1, 2)) for c in cells)
    factor = max(1.0, least / across)
    for k, p in enumerate(v):
        c = min(line, key=lambda q: abs(q[0] - p[0]))
        v[k] = rounded([p[0], c[1] + (p[1] - c[1]) * factor, c[2] + (p[2] - c[2]) * factor])
    return across, factor


def main():
    doc = json.load(open(SOURCE))
    vol = doc["volumes"]
    v500, v200, v100 = vol["500"], vol["200"], vol["100"]
    body = {s["id"]: s for s in v500["body"]}
    fine = {s["id"]: s for s in v200["body"]}
    like = body["leaf-l-b-v1-1"]
    shoots = sorted({m.group(1) for i in body for m in [re.match(r"(leaf-.+-v\d+)-\d+$", i)] if m})
    added = 0
    for shoot in shoots:
        if shoot in body:
            continue
        sleeve = coarse_sleeve(fine[shoot], like, shoot)
        main_sleeve = re.sub(r"-v\d+$", "-s", shoot)
        at = next(k for k, s in enumerate(v500["body"]) if s["id"] == main_sleeve) + 1
        v500["body"].insert(at, sleeve)
        at = next(k for k, s in enumerate(v500["thermal"]) if s.get("of") == main_sleeve) + 1
        v500["thermal"].insert(at, {"id": "th-" + shoot, "of": shoot, "region": "leaf-crown"})
        added += 1
    print("[DEBUG] 1:500 side-shoot sleeves added:", added, "of", len(shoots))
    wood = [s for s in v200["body"] if re.match(r"(branch-(l|c|r|tl|tc|tr)\d+|br-[lcr]-[ab])$", s["id"]) and s["id"] not in body]
    first_leaf = next(k for k, s in enumerate(v500["body"]) if s["id"].startswith("leaf-"))
    bark = {s.get("of"): s for s in v200["thermal"]}
    for n, s in enumerate(wood):
        v500["body"].insert(first_leaf + n, {**{k: s[k] for k in ("id", "kind", "bone", "material", "role", "tone")}, "slender": True, "vertices": s["vertices"], "triangles": s["triangles"]})
        if s["id"] in bark and not any(t.get("id") == bark[s["id"]]["id"] for t in v500["thermal"]):
            v500["thermal"].append(dict(bark[s["id"]]))
    print("[DEBUG] 1:500 branches at their true section (slender):", len(wood))
    print("[DEBUG] 1:500 tip-c-b across before, factor:", swell(body["tip-c-b"], TIP_MIN_ACROSS))
    b100 = {s["id"]: s for s in v100["body"]}
    print("[DEBUG] 1:100 leaf-l-b-v1 gap before:", round(neck(b100["leaf-l-b-v1"], b100["leaf-l-b-s"]), 4))
    json.dump(doc, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))


if __name__ == "__main__":
    main()
