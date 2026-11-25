/** 🌬️ What the depth plates of a plant show in its wind poses, measured and drawn: `bun wind.ts <id> [--poses a,b,c] [--scales 50,100] [--sheets <folder>] [--size 800]` (run from the folder of the 3D ticket, whose `r3-2d.ts` and `sharp` it uses). Read-only.
 *
 * Measured per authored scale, pose and view (front, plan), in millimetres on paper at that scale: `fringe` = the area the plates (`mvc-*`) draw beyond the union of every other filled shape (the visible part of the plates) and its share of the drawn area; `pieces` = connected parts of it larger than 0.02 mm², `slivers` = those whose mean width (2 × area / perimeter) is below 0.3 mm; `cut` = the longest straight run (turns below 4 degrees) of the drawn outline that lies on the edge of a plate, beside it the longest straight run of the outline without plates in the same pose; `notches` = re-entrant corners sharper than 60 degrees of the outline with plates, at arms of 0.15 mm or more, that the outline without plates does not have.
 * With `--sheets` the poses are drawn as rows (side, front, plan; light line view of the product) into `<folder>/wind.<id>.s<scale>.png`.
 * @see ./carry.ts — the writer of the plates */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const clipping: any = (await import(Bun.resolveSync("polygon-clipping", `${join(S, "🧪️tests", "🧊️catalog-silhouette", "🔍️oracles")}/`))).default;
type P = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const id = args[0]!.split("/").at(-1)!.replace(/\.json$/u, "");
const asset = readDoc(args[0]!).doc.asset;
const poses = flag("poses")?.split(",") ?? asset.poses.map((pose: any) => pose.id);
const scales = flag("scales")?.split(",") ?? Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
const folder = flag("sheets");
const size = Number(flag("size") ?? 800);

const shoelace = (ring: readonly P[]): number => ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0) / 2;
const open = (ring: P[]): P[] => (ring.length > 1 && ring[0]![0] === ring.at(-1)![0] && ring[0]![1] === ring.at(-1)![1] ? ring.slice(0, -1) : ring);
const area = (multi: P[][][]): number => multi.reduce((sum, polygon) => sum + polygon.reduce((s, ring, i) => s + (i === 0 ? 1 : -1) * Math.abs(shoelace(ring)), 0), 0);
const around = (ring: readonly P[]): number => ring.reduce((sum, p, i) => sum + Math.hypot(ring[(i + 1) % ring.length]![0] - p[0], ring[(i + 1) % ring.length]![1] - p[1]), 0);
const snap = (multi: P[][][], step: number): P[][][] => multi.map((polygon) => polygon.map((ring) => ring.map((p) => [Math.round(p[0] / step) * step, Math.round(p[1] / step) * step] as P)));
const clip = (name: "union" | "difference", multis: P[][][][]): P[][][] => {
  for (const step of [1e-4, 7e-4, 1.1e-3, 3.1e-3, 7.3e-3, 1.37e-2, 2.3e-2]) {
    try { return clipping[name](...multis.map((multi) => snap(multi, step))); } catch {}
  }
  throw new Error(`no ${name} of ${multis.length} polygons`);
};
const union = (list: P[][][][]): P[][][] => {
  let level = list.filter((multi) => multi.length > 0);
  while (level.length > 1) level = Array.from({ length: Math.ceil(level.length / 8) }, (_, i) => clip("union", level.slice(i * 8, i * 8 + 8)));
  return level[0] ?? [];
};
const simplify = (ring: P[], eps: number): P[] => {
  const out: P[] = [];
  for (const p of open(ring)) if (out.length === 0 || Math.hypot(p[0] - out.at(-1)![0], p[1] - out.at(-1)![1]) > eps) out.push(p);
  return out;
};
const runs = (multi: P[][][], on: (p: P) => boolean): number => {
  let best = 0;
  for (const ring of multi.flat()) {
    const pts = simplify(ring, 0.01);
    if (pts.length < 3) continue;
    let length = 0, heading: number | undefined;
    for (let i = 0; i < pts.length * 2; i++) {
      const [a, b] = [pts[i % pts.length]!, pts[(i + 1) % pts.length]!];
      const angle = Math.atan2(b[1] - a[1], b[0] - a[0]);
      const step = Math.hypot(b[0] - a[0], b[1] - a[1]);
      if (!on([(a[0] + b[0]) / 2, (a[1] + b[1]) / 2])) { length = 0; heading = undefined; continue; }
      const turn = heading === undefined ? 0 : Math.abs(Math.atan2(Math.sin(angle - heading), Math.cos(angle - heading)));
      length = heading !== undefined && turn < 0.07 ? length + step : step;
      if (length === step) heading = angle;
      best = Math.max(best, Math.min(length, around(pts)));
    }
  }
  return best;
};
const distance = (p: P, rings: P[][]): number => {
  let best = Infinity;
  for (const ring of rings) for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    const d: P = [b[0] - a[0], b[1] - a[1]];
    const t = Math.max(0, Math.min(1, ((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / (d[0] * d[0] + d[1] * d[1] || 1)));
    best = Math.min(best, Math.hypot(a[0] + t * d[0] - p[0], a[1] + t * d[1] - p[1]));
  }
  return best;
};
const corners = (multi: P[][][]): P[] => multi.flat().flatMap((ring, r) => {
  const pts = simplify(ring, 0.02);
  if (pts.length < 4) return [];
  return pts.filter((p, i) => {
    const [a, b] = [pts[(i + pts.length - 1) % pts.length]!, pts[(i + 1) % pts.length]!];
    const [u, v]: [P, P] = [[p[0] - a[0], p[1] - a[1]], [b[0] - p[0], b[1] - p[1]]];
    if (Math.hypot(...u) < 0.15 || Math.hypot(...v) < 0.15) return false;
    const turn = Math.atan2(u[0] * v[1] - u[1] * v[0], u[0] * v[0] + u[1] * v[1]) * Math.sign(shoelace(pts));
    return r >= 0 && turn < -Math.PI / 3;
  });
});

for (const key of scales) {
  const k = 1000 / Number(key);
  for (const pose of poses) {
    const state = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(key)));
    for (const view of ["front", "plan"]) {
      const multi = (shape: any): P[][][] => shape.rings.filter((ring: any[]) => ring.length > 2).map((ring: any[]) => [ring.map((p) => [Math.round(p.x * k * 1e4) / 1e4, Math.round(p.y * k * 1e4) / 1e4] as P)]);
      const drawn = state.shapes[view].filter((shape: any) => shape.shape.role !== "ground" && shape.shape.filled === true);
      const plates = drawn.filter((shape: any) => shape.shape.id.startsWith("mvc-"));
      if (plates.length === 0) continue;
      const base = union(drawn.filter((shape: any) => !shape.shape.id.startsWith("mvc-")).map(multi));
      const added = union(plates.map(multi));
      const fringe: P[][][] = clip("difference", [added, base]);
      const whole = union([base, added]);
      const parts = fringe.filter((polygon) => area([polygon]) > 0.02);
      const slivers = parts.filter((polygon) => (2 * area([polygon])) / polygon.reduce((sum, ring) => sum + around(ring), 0) < 0.3);
      const rings = base.flat().map(open);
      const baseCorners = corners(base);
      const fresh = corners(whole).filter((p) => !baseCorners.some((q) => Math.hypot(p[0] - q[0], p[1] - q[1]) < 0.05));
      const widest = parts.map((polygon) => Math.max(...open(polygon[0]!).map((p) => distance(p, rings)))).reduce((m, x) => Math.max(m, x), 0);
      console.log(`${id} 1:${key} ${pose} ${view}: plates ${plates.length} fringe ${area(fringe).toFixed(2)} mm² of ${area(whole).toFixed(1)} (${((area(fringe) / area(whole)) * 100).toFixed(1)} %), farthest ${widest.toFixed(2)} mm, pieces ${parts.length}, slivers ${slivers.length}, cut ${runs(whole, (p) => distance(p, rings) > 0.02).toFixed(2)} mm (without plates ${runs(base, () => true).toFixed(2)}), notches ${fresh.length}`);
    }
  }
  if (folder === undefined) continue;
  mkdirSync(folder, { recursive: true });
  const sharp: any = (await import(Bun.resolveSync("sharp", `${process.cwd()}/`))).default;
  const { drawingSvg } = await import(`${process.cwd()}/r3-2d.ts`);
  const tiles: any[] = [];
  for (const [r, pose] of poses.entries()) for (const [c, viewpoint] of ["elevation", "front", "plan"].entries()) {
    const svg: string = drawingSvg(asset, Number(key), { kind: "pose", pose }, "light-line", viewpoint);
    tiles.push({ input: await sharp(Buffer.from(svg), { density: Math.max(72, Math.min(1200, (size * 2 * 72) / Math.max(1, (await sharp(Buffer.from(svg)).metadata()).width ?? 1))), limitInputPixels: false }).resize(size - 10, size - 34, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer(), left: c * size + 5, top: r * size + 29 });
    tiles.push({ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${size}" height="26"><rect width="${size}" height="26" fill="#eeeeea"/><text x="8" y="19" font-family="Helvetica" font-size="17">${id} 1:${key} ${pose} ${viewpoint}</text></svg>`), left: c * size, top: r * size });
  }
  const out = join(folder, `wind.${id}.s${key}.png`);
  await sharp({ create: { width: size * 3, height: size * poses.length, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out);
  console.log(out);
}
