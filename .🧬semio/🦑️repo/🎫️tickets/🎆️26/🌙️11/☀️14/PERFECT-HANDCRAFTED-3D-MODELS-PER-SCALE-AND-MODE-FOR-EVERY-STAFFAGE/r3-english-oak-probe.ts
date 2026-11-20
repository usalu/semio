/** 🔬️ Round 3 English oak: per body solid, the front-view (z, y) and elevation (x, y) bounding box of the tessellated solid in the default pose next to the box of the 2D shape of its bone. Usage: bun r3-english-oak-probe.ts <scale> [front|elevation] */
import { entryOf, poseState, scaleMeshes } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 100);
const view = (process.argv[3] ?? "front") as "front" | "elevation";
const { asset } = entryOf("vegetation/english-oak");
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const st = poseState(asset, scale);
const box = (pts: Iterable<[number, number]>): string => {
  let [a, b, c, d] = [1e9, -1e9, 1e9, -1e9];
  for (const [x, y] of pts) [a, b, c, d] = [Math.min(a, x), Math.max(b, x), Math.min(c, y), Math.max(d, y)];
  return `${a.toFixed(3)}..${b.toFixed(3)} | ${c.toFixed(3)}..${d.toFixed(3)}`;
};
sm.body.forEach((mesh, i) => {
  const s = sm.solids.body[i]!;
  if (s.kind === "ellipsoid") return;
  const pts: [number, number][] = [];
  for (let k = 0; k < mesh.world.length; k += 3) pts.push([view === "front" ? mesh.world[k + 2]! : mesh.world[k]!, mesh.world[k + 1]!]);
  console.log("3D", s.id.padEnd(14), s.bone.padEnd(12), box(pts));
});
for (const e of st[view]) if (e.shape.role === "body" && e.shape.filled && !String(e.shape.bone).startsWith("fol-")) console.log("2D", String(e.shape.id).padEnd(14), String(e.shape.bone).padEnd(12), box((function* () { for (const r of e.rings) for (const p of r) yield [p.x, p.y] as [number, number]; })()));
const rings = st[view].filter((e: any) => e.shape.role === "body" && e.shape.filled && e.closed).flatMap((e: any) => e.rings.map((r: any[]) => r.map((p) => [p.x, p.y] as [number, number])));
const inside = (x: number, y: number): boolean => rings.some((r: [number, number][]) => {
  let c = false;
  for (let i = 0, j = r.length - 1; i < r.length; j = i++) if (r[i]![1] > y !== r[j]![1] > y && x < ((r[j]![0] - r[i]![0]) * (y - r[i]![1])) / (r[j]![1] - r[i]![1]) + r[i]![0]) c = !c;
  return c;
});
const dist = (x: number, y: number): number => {
  let best = 1e9;
  for (const r of rings) for (let i = 0; i < r.length; i++) {
    const [a, b] = [r[i]!, r[(i + 1) % r.length]!];
    const [dx, dy] = [b[0] - a[0], b[1] - a[1]];
    const t = Math.max(0, Math.min(1, ((x - a[0]) * dx + (y - a[1]) * dy) / (dx * dx + dy * dy || 1)));
    best = Math.min(best, Math.hypot(x - a[0] - dx * t, y - a[1] - dy * t));
  }
  return best;
};
sm.body.forEach((mesh, i) => {
  const s = sm.solids.body[i]!;
  let [worst, at] = [0, [0, 0]];
  for (let k = 0; k < mesh.world.length; k += 3) {
    const [x, y] = [view === "front" ? mesh.world[k + 2]! : mesh.world[k]!, mesh.world[k + 1]!];
    if (inside(x, y)) continue;
    const d = dist(x, y);
    if (d > worst) [worst, at] = [d, [x, y]];
  }
  if (worst > 0.02) console.log("OUT", s.id.padEnd(16), worst.toFixed(3), "at", at.map((v) => v.toFixed(2)).join(","));
});
