"""🧪️ Independent oracle of the volume measurements: shapely and trimesh recompute, from the very triangles the kernel tessellated (written by `volumes-oracle.ts`), the silhouette IoU and Hausdorff distance of every view, the outer surface area of every thermal region, the union volume and the mesh volumes of every list.

Usage (through the pinned, hash-locked uv environment, never directly): `bun volumes-oracle.ts <category>/<slug> [--scale N]` writes `🗑️generated/volumes-tools/oracle/<category>/<slug>.s<N>.in.json`, runs this script on it and prints the comparison with the owned and product measurements.
Input: `{ "views": { view: { "axes": [[axis, sign], [axis, sign]], "drawn": [group...] } }, "body": [mesh], "thermal": [{ "mesh", "region", "rank" }], "structure": [mesh], "cell": metres }` with mesh = `{ "id", "positions", "indices" }` in world metres and group = a list of rings of `[x, y]` that fill by the even-odd rule.
Output (stdout): `{ "views": { view: { "iou", "hausdorffM", "drawnArea", "volumeArea" } }, "regions": { region: { "outerAreaM2" } }, "unionVolumeM3", "meshVolumeM3": { list: v }, "bbox": [x, y, z] }`.
"""
import json
import sys

import numpy as np
import shapely
import trimesh
from shapely.geometry import Polygon
from shapely.ops import unary_union


def mesh_of(entry):
    return trimesh.Trimesh(vertices=np.array(entry["positions"], dtype=float).reshape(-1, 3), faces=np.array(entry["indices"], dtype=np.int64).reshape(-1, 3), process=False)


def group_polygon(group):
    """Non-zero winding area of a group of rings: a ring of the orientation of the first adds, one of the opposite orientation cuts (nested holes)."""
    result = None
    first = None
    for ring in group:
        if len(ring) < 3:
            continue
        p = Polygon(ring)
        sign = 1 if p.exterior.is_ccw else -1
        p = p.buffer(0)
        if first is None:
            first = sign
            result = p
        elif sign == first:
            result = result.union(p)
        else:
            result = result.difference(p)
    return result


def silhouette(meshes, axes):
    (a, sa), (b, sb) = axes
    polygons = []
    for m in meshes:
        tri = m.triangles[:, :, [a, b]] * np.array([sa, sb])
        for t in tri:
            p = Polygon(t)
            if p.area > 1e-14:
                polygons.append(p.buffer(0))
    return unary_union(polygons) if polygons else Polygon()


def union_volume(meshes, cell_min):
    lo = np.min([m.bounds[0] for m in meshes], axis=0)
    hi = np.max([m.bounds[1] for m in meshes], axis=0)
    cell = max(float(np.prod(hi - lo) / 1.5e6) ** (1 / 3), cell_min)
    axes = [np.arange(lo[k] + cell / 2, hi[k], cell) for k in range(3)]
    grid = np.stack(np.meshgrid(*axes, indexing="ij"), axis=-1).reshape(-1, 3)
    inside = np.zeros(len(grid), dtype=bool)
    for m in meshes:
        near = np.all((grid >= m.bounds[0]) & (grid <= m.bounds[1]), axis=1)
        if near.any():
            inside[near] |= m.contains(grid[near])
    return float(inside.sum() * cell**3), cell


def p95_directed(a, b, step):
    """95th percentile of the distance of the densified boundary of `a` to the region `b`."""
    pts = []
    for geom in getattr(a.boundary, "geoms", [a.boundary]):
        pts.extend(shapely.segmentize(geom, step).coords)
    if not pts:
        return 0.0
    return float(np.percentile(shapely.distance(shapely.points(np.array(pts)), b), 95))


def directed(a, b, step):
    """Directed Hausdorff distance of regions: the largest distance from any point of `a` to the region `b` (0 inside). Candidates are the densified boundary of `a` and a grid of points of `a - b` (the farthest point can lie in the interior, e.g. a solid disc against a drawn ring). GEOS `hausdorff_distance` measures to the nearest vertex only, so it is not used."""
    diff = a.difference(b)
    if diff.is_empty:
        return 0.0
    pts = []
    for geom in getattr(diff.boundary, "geoms", [diff.boundary]):
        pts.extend(shapely.segmentize(geom, step).coords)
    x0, y0, x1, y1 = diff.bounds
    gx, gy = np.meshgrid(np.arange(x0, x1 + step, step), np.arange(y0, y1 + step, step))
    gx, gy = gx.ravel(), gy.ravel()
    keep = shapely.contains_xy(diff, gx, gy)
    pts.extend(zip(gx[keep], gy[keep]))
    if not pts:
        return 0.0
    return float(shapely.distance(shapely.points(np.array(pts)), b).max())


def outer_area(meshes, eps=0.0):
    """Area of the triangles whose centroid, moved `eps` metres along the outward normal, lies in none of the other meshes (a face buried against a neighbour counts as inner even when both touch exactly)."""
    total = 0.0
    for k, m in enumerate(meshes):
        centroids = m.triangles_center + eps * m.face_normals
        covered = np.zeros(len(centroids), dtype=bool)
        for j, other in enumerate(meshes):
            if j == k:
                continue
            near = np.all((centroids >= other.bounds[0]) & (centroids <= other.bounds[1]), axis=1)
            if near.any():
                covered[near] |= other.contains(centroids[near])
        total += float(m.area_faces[~covered].sum())
    return total


def outer_area_vs(items, others, eps, samples=40000):
    """Buried-only outer area as the check defines it (C1a): surface samples of each item, moved `eps` along the normal, count unless they lie in a solid of `others` with another base."""
    total_area = sum(m.area for m, _ in items)
    total = 0.0
    for m, base in items:
        n = int(max(200, samples * m.area / max(total_area, 1e-12)))
        pts, face = trimesh.sample.sample_surface(m, n, seed=7)
        pts = pts + eps * m.face_normals[face]
        covered = np.zeros(len(pts), dtype=bool)
        for other, obase in others:
            if obase == base:
                continue
            near = np.all((pts >= other.bounds[0]) & (pts <= other.bounds[1]), axis=1) & ~covered
            if near.any():
                covered[near] |= other.contains(pts[near])
        total += m.area * float((~covered).mean())
    return total


def outer_area_sampled(meshes, eps, samples=60000):
    """Area-weighted surface samples of every mesh (fixed seed), moved `eps` metres along the face normal; the outer area is the area of each mesh times the share of its samples that lie in none of the other meshes. Resolves partly buried triangles, which a centroid test cannot."""
    total_area = sum(m.area for m in meshes)
    total = 0.0
    for k, m in enumerate(meshes):
        n = int(max(200, samples * m.area / max(total_area, 1e-12)))
        pts, face = trimesh.sample.sample_surface(m, n, seed=7)
        pts = pts + eps * m.face_normals[face]
        covered = np.zeros(len(pts), dtype=bool)
        for j, other in enumerate(meshes):
            if j == k:
                continue
            near = np.all((pts >= other.bounds[0]) & (pts <= other.bounds[1]), axis=1) & ~covered
            if near.any():
                covered[near] |= other.contains(pts[near])
        total += m.area * float((~covered).mean())
    return total


def main():
    data = json.load(open(sys.argv[1], encoding="utf8"))
    body = [mesh_of(e) for e in data["body"]]
    out = {"views": {}, "regions": {}, "meshVolumeM3": {}}
    if body:
        allv = np.concatenate([m.vertices for m in body])
        out["bbox"] = (allv.max(axis=0) - allv.min(axis=0)).tolist()
        for view, spec in data["views"].items():
            drawn = unary_union([g for g in (group_polygon(x) for x in spec["drawn"]) if g is not None and not g.is_empty])
            vol = silhouette(body, spec["axes"])
            inter = drawn.intersection(vol).area
            union = drawn.union(vol).area
            out["views"][view] = {
                "iou": inter / union if union > 0 else 1.0,
                "hausdorffM": max(directed(drawn, vol, spec["step"]), directed(vol, drawn, spec["step"])) if not drawn.is_empty and not vol.is_empty else float("inf"),
                "p95M": max(p95_directed(drawn, vol, spec["step"]), p95_directed(vol, drawn, spec["step"])) if not drawn.is_empty and not vol.is_empty else float("inf"),
                "drawnArea": float(drawn.area),
                "volumeArea": float(vol.area),
            }
        out["unionVolumeM3"], out["unionCellM"] = union_volume(body, data["cell"])
    out["meshVolumeM3"]["body"] = float(sum(m.volume for m in body))
    others = [(mesh_of(e["mesh"]), e["base"]) for e in data["allSolids"]]
    by_region = {}
    for entry in data["thermal"]:
        by_region.setdefault(entry["region"], []).append(entry)
    for region, items in by_region.items():
        top = max(e["rank"] for e in items)
        top_items = [mesh_of(e["mesh"]) for e in items if e["rank"] == top]
        out["regions"][region] = {"outerAreaM2": outer_area(top_items), "outerAreaOffsetM2": outer_area(top_items, data["eps"]), "outerAreaSampledM2": outer_area_sampled(top_items, data["eps"]), "outerAreaVsM2": outer_area_vs([(mesh_of(e["mesh"]), e["base"]) for e in items if e["rank"] == top], others, data["eps"]), "plainAreaM2": float(sum(m.area for m in top_items))}
    out["meshVolumeM3"]["thermal"] = float(sum(mesh_of(e["mesh"]).volume for e in data["thermal"]))
    out["meshVolumeM3"]["structure"] = float(sum(mesh_of(e).volume for e in data["structure"]))
    json.dump(out, sys.stdout)


main()
