/** 🔬️ Bakes the outline of volume solids into 2D polygons: `bun silhouette-bake.ts <slug> <view> <groups.json> <out.json>`.
 * groups: [{ id, bone, solids: [solidId...], tolerance? }]. Elevation outlines are in the bone-local (u, v) plane (shape coordinates of the elevation), front and plan outlines in rest-world paper coordinates. */
import { readFileSync, writeFileSync } from "node:fs";
import polygonClipping from "polygon-clipping";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { pieces } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { QUALITY_FOR, catalogEntries, frames, loadAsset, projectWorld, simplifyRing, toWorld, type View } from "./volume-lib.ts";
const [slug, view, groupsFile, out] = process.argv.slice(2);
const asset = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
const rest = frames(asset);
const d = asset.dimensions;
const quality = QUALITY_FOR(Math.max(d.widthM, d.heightM, d.depthM));
const groups = JSON.parse(readFileSync(groupsFile!, "utf8")) as { id: string; bone: string; solids: string[]; tolerance?: number; split?: boolean }[];
const hull = (pts: [number, number][]): [number, number][] => {
  const p = [...new Map(pts.map((q) => [`${q[0]},${q[1]}`, q])).values()].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o: [number, number], a: [number, number], b: [number, number]) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lo: [number, number][] = [];
  for (const q of p) { while (lo.length >= 2 && cross(lo[lo.length - 2]!, lo[lo.length - 1]!, q) <= 0) lo.pop(); lo.push(q); }
  const up: [number, number][] = [];
  for (const q of [...p].reverse()) { while (up.length >= 2 && cross(up[up.length - 2]!, up[up.length - 1]!, q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
};
const result: Record<string, number[][][]> = {};
for (const g of groups) {
  const tris: [number, number][][] = [];
  const splitRings: number[][][] = [];
  for (const sid of g.solids) {
    const solid = asset.volume.solids.find((s: any) => s.id === sid);
    const geo = tessellateSolid({ ...solid, node: solid.bone } as never, quality);
    const pt = (i: number): [number, number] => {
      const p: [number, number, number] = [geo.positions[i * 3]!, geo.positions[i * 3 + 1]!, geo.positions[i * 3 + 2]!];
      if (view === "elevation") return [p[0], p[1]];
      const w = toWorld(rest[solid.bone]!, p);
      const q = projectWorld(view as View, w);
      return [q.x, q.y];
    };
    if (g.split) {
      for (const piece of pieces({ id: sid, positions: Float64Array.from(geo.positions), indices: Uint32Array.from(geo.indices) })) {
        const q = new Set<number>(piece.indices);
        splitRings.push(simplifyRing(hull([...q].map((i) => pt(i))).map(([x, y]) => ({ x, y })), g.tolerance ?? 0.004).map((p) => [Number(p.x.toFixed(4)), Number(p.y.toFixed(4))]));
      }
      continue;
    }
    for (let t = 0; t < geo.indices.length; t += 3) tris.push([pt(geo.indices[t]!), pt(geo.indices[t + 1]!), pt(geo.indices[t + 2]!)]);
  }
  if (g.split) {
    result[g.id] = splitRings;
    continue;
  }
  let multi: polygonClipping.MultiPolygon = [];
  for (const unit of [1e-4, 5e-4, 2e-3, 5e-3]) {
    const snap = (p: [number, number]): [number, number] => [Math.round(p[0] / unit) * unit, Math.round(p[1] / unit) * unit];
    const polys = tris.map((t) => t.map(snap)).filter((t) => Math.abs((t[1]![0] - t[0]![0]) * (t[2]![1] - t[0]![1]) - (t[2]![0] - t[0]![0]) * (t[1]![1] - t[0]![1])) > unit * unit).map((t) => [[...t, t[0]!]] as polygonClipping.Polygon);
    try {
      let parts = polys;
      while (parts.length > 1) {
        const next: polygonClipping.MultiPolygon[] = [];
        for (let i = 0; i < parts.length; i += 16) next.push(polygonClipping.union(parts[i]!, ...parts.slice(i + 1, i + 16)));
        parts = next as never;
      }
      multi = (parts[0] ?? []) as polygonClipping.MultiPolygon;
      break;
    } catch {
      continue;
    }
  }
  result[g.id] = multi.map((poly) => simplifyRing(poly[0]!.slice(0, -1).map(([x, y]) => ({ x, y })), g.tolerance ?? 0.004).map((p) => [Number(p.x.toFixed(4)), Number(p.y.toFixed(4))]));
}
writeFileSync(out!, JSON.stringify(result));
console.log(Object.entries(result).map(([k, v]) => `${k}: ${v.length} rings, ${v.reduce((s, r) => s + r.length, 0)} points`).join("\n"));
