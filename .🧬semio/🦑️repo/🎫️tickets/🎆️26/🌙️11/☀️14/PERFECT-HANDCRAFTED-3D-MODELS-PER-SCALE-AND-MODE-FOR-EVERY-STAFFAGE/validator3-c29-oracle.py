"""Oracle of the C29 outer area: trimesh surface samples of the union of the own solids of a region.
Input: the JSON of validator3-c29-dump.ts. Per region: faces within 45 degrees of a section axis are dropped; every other face is sampled (area-weighted, 4000 points per solid
plus the face centres) at points moved eps out along the outward normal; a sample inside another solid of the region (trimesh contains) is internal.
Output: {asset: {region: {oracle, measured, ratio}}}."""
import json
import sys

import numpy as np
import trimesh

COS45 = np.cos(np.pi / 4)


def solid(entry):
    m = trimesh.Trimesh(np.array(entry["positions"]).reshape(-1, 3), np.array(entry["indices"]).reshape(-1, 3), process=False)
    m.merge_vertices(merge_tex=False, merge_norm=False, digits_vertex=6)
    return m


def outward(m):
    if m.is_watertight:
        m = m.copy()
        trimesh.repair.fix_normals(m)
        if m.volume < 0:
            m.invert()
    return m


def region_area(solids, eps, samples=6000, per=None):
    meshes = [outward(solid(s)) for s in solids]
    total = 0.0
    rng = np.random.default_rng(11)
    for k, (m, s) in enumerate(zip(meshes, solids)):
        if len(m.faces) == 0:
            continue
        keep = np.ones(len(m.faces), dtype=bool)
        for axis in s["sections"]:
            keep &= ~(m.face_normals @ np.array(axis) >= COS45)
        if not keep.any():
            continue
        n = min(samples, max(200, int(m.area_faces[keep].sum() / 1e-5)))
        probs = m.area_faces * keep
        probs = probs / probs.sum()
        faces = rng.choice(len(m.faces), size=n, p=probs)
        r = rng.random((n, 2))
        flip = r.sum(axis=1) > 1
        r[flip] = 1 - r[flip]
        tri = m.triangles[faces]
        points = tri[:, 0] + r[:, :1] * (tri[:, 1] - tri[:, 0]) + r[:, 1:] * (tri[:, 2] - tri[:, 0])
        probes = points + m.face_normals[faces] * eps
        buried = np.zeros(n, dtype=bool)
        for j, other in enumerate(meshes):
            if j == k or not other.is_watertight:
                continue
            near = np.all((probes >= other.bounds[0]) & (probes <= other.bounds[1]), axis=1)
            if near.any():
                buried[near] |= other.contains(probes[near])
        part = float(m.area_faces[keep].sum() * (1 - buried.mean()))
        if per is not None:
            per.append({"id": s["id"], "oracle": part, "module": s.get("moduleArea"), "faces": len(m.faces), "watertight": bool(m.is_watertight)})
        total += part
    return total


def main():
    data = json.load(open(sys.argv[1]))
    eps = float(sys.argv[3]) if len(sys.argv) > 3 else 0.00015
    out = {}
    for slug, regions in data.items():
        out[slug] = {}
        for rid, r in regions.items():
            if r["measured"] is None or r["exchange"]:
                continue
            if r["hidden"]:
                o = float(sum(solid(s).area for s in r["solids"]))
            else:
                per = []
                o = region_area(r["solids"], eps, per=per)
            out[slug][rid] = {"per": per if not r["hidden"] else [], "oracle": o, "measured": r["measured"], "ratio": (r["measured"] / o) if o > 0 else None, "areaM2": r["areaM2"], "solids": len(r["solids"])}
    json.dump(out, open(sys.argv[2], "w"), indent=1)


main()
