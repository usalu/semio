/** 🖊️ Round 3 child-on-kick-scooter: the orthographic outline of groups of body solids as the starting point of the redrawn 2D shapes (ruling E1).
 *
 * Usage: `bun r3-child-on-kick-scooter-project2d.ts <candidate asset json> <jobs json> <out json>`
 * A job is `{ key, scale, view, solids, bone?, pose?, clip? }`: `elevation` gives the outline in the (u, v) frame of `bone`, `lateral` in its (u, w) frame, both with the span of the swept axis (world z at rest for `elevation` with `sweep`, v for `lateral`; over the first and the last third of the bone, so a taper can be stated) written; swept outlines are traced on a raster turned by 0.2731 rad so that no two vertices share a coordinate to `<out json>.spans.json`; `front` (lateral, height) and `plan` (ahead, lateral) give it in rest-pose world coordinates, or in the world of `pose` when that is set. The union of the projected triangles is rasterised (cell = scale / 100 mm), its largest outer contour traced and simplified (Douglas-Peucker, 0.35 of a cell-scaled tolerance) to an economical closed polygon.
 * @see ./round3-brief.md */
import { readFileSync, writeFileSync } from "node:fs";

import { restFrames, solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { localGeometry, qualityOfAsset } from "./volumes-lib.ts";

type Job = { key: string; scale: number; view: "elevation" | "front" | "plan" | "lateral"; solids: string[]; bone?: string; pose?: string; sweep?: boolean; clip?: { axis: 0 | 1; min: number; max: number }; tol?: number };
const [docPath, jobsPath, outPath] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(docPath!, "utf8")).asset;
const jobs: Job[] = JSON.parse(readFileSync(jobsPath!, "utf8"));
const rest = restFrames(asset.rig);
const q = qualityOfAsset(asset);
const posed = new Map<string, ReturnType<typeof solveChain>>();
const frameOf = (pose: string | undefined) => {
  if (pose === undefined) return rest;
  let f = posed.get(pose);
  if (f === undefined) {
    const p = asset.poses.find((x: { id: string }) => x.id === pose);
    f = solveChain(asset.rig, p.bones, { x: p.root?.x ?? 0, y: p.root?.y ?? 0, angleDeg: p.root?.angleDeg ?? 0 });
    posed.set(pose, f);
  }
  return f;
};

function simplify(pts: [number, number][], tol: number): [number, number][] {
  const n = pts.length;
  if (n < 5) return pts;
  let far = 0;
  let best = -1;
  for (let i = 1; i < n; i++) {
    const d = Math.hypot(pts[i]![0] - pts[0]![0], pts[i]![1] - pts[0]![1]);
    if (d > best) [best, far] = [d, i];
  }
  const dp = (a: number, b: number, keep: boolean[]): void => {
    let worst = 0;
    let at = -1;
    const [ax, ay] = pts[a % n]!;
    const [bx, by] = pts[b % n]!;
    const len = Math.hypot(bx - ax, by - ay) || 1e-12;
    for (let i = a + 1; i < b; i++) {
      const [px, py] = pts[i % n]!;
      const d = Math.abs((bx - ax) * (ay - py) - (ax - px) * (by - ay)) / len;
      if (d > worst) [worst, at] = [d, i];
    }
    if (worst > tol && at >= 0) {
      keep[at % n] = true;
      dp(a, at, keep);
      dp(at, b, keep);
    }
  };
  const keep = new Array<boolean>(n).fill(false);
  keep[0] = keep[far] = true;
  dp(0, far, keep);
  dp(far, n, keep);
  return pts.filter((_, i) => keep[i]);
}

const out: Record<string, [number, number][]> = {};
const spans: Record<string, number[]> = {};
const TURN = 0.2731;
const turn = ([x, y]: [number, number], sign = 1): [number, number] => [x * Math.cos(TURN) - sign * y * Math.sin(TURN), sign * x * Math.sin(TURN) + y * Math.cos(TURN)];
for (const job of jobs) {
  const body = asset.volumes[String(job.scale)].body as { id: string; bone: string }[];
  const tris: number[] = [];
  const frames = frameOf(job.pose);
  for (const id of job.solids) {
    const solid = body.find((s) => s.id === id);
    if (solid === undefined) throw new Error(`${job.key}: no solid ${id} at 1:${job.scale}`);
    const g = localGeometry(solid as never, q);
    const f = frames[solid.bone]!.frame;
    const pt = (i: number): [number, number] => {
      const [a, b, c] = [g.positions[3 * i]!, g.positions[3 * i + 1]!, g.positions[3 * i + 2]!];
      if (job.bone !== undefined && job.pose === undefined) {
        const w = [0, 1, 2].map((k) => f.o[k]! + a * f.u[k]! + b * f.v[k]! + c * f.w[k]!);
        const t = frames[job.bone]!.frame;
        const d = [w[0]! - t.o[0]!, w[1]! - t.o[1]!, w[2]! - t.o[2]!];
        const along = (axis: readonly number[]): number => d[0]! * axis[0]! + d[1]! * axis[1]! + d[2]! * axis[2]!;
        const swept = job.view === "lateral" ? along(t.v) : along(t.w) + t.o[2]!;
        const u = along(t.u);
        const length = asset.rig.bones.find((x: { id: string }) => x.id === job.bone).lengthM as number;
        const span = spans[job.key] ?? (spans[job.key] = [Infinity, -Infinity, Infinity, -Infinity]);
        if (u <= 0.35 * length) [span[0], span[1]] = [Math.min(span[0]!, swept), Math.max(span[1]!, swept)];
        if (u >= 0.65 * length) [span[2], span[3]] = [Math.min(span[2]!, swept), Math.max(span[3]!, swept)];
        if (job.view === "lateral") return turn([u, along(t.w)]);
        if (job.sweep === true) return turn([u, along(t.v)]);
      }
      if (job.view === "elevation" && job.pose === undefined) {
        if (job.bone !== undefined && job.bone !== solid.bone) {
          const w = [0, 1, 2].map((k) => f.o[k]! + a * f.u[k]! + b * f.v[k]! + c * f.w[k]!);
          const t = frames[job.bone]!.frame;
          const d = [w[0]! - t.o[0]!, w[1]! - t.o[1]!, w[2]! - t.o[2]!];
          return [d[0]! * t.u[0]! + d[1]! * t.u[1]! + d[2]! * t.u[2]!, d[0]! * t.v[0]! + d[1]! * t.v[1]! + d[2]! * t.v[2]!];
        }
        return [a, b];
      }
      const w = [0, 1, 2].map((k) => f.o[k]! + a * f.u[k]! + b * f.v[k]! + c * f.w[k]!);
      return job.view === "front" ? [w[2]!, w[1]!] : job.view === "plan" ? [w[0]!, w[2]!] : [w[0]!, w[1]!];
    };
    for (let i = 0; i < g.indices.length; i += 3) for (let k = 0; k < 3; k++) tris.push(...pt(g.indices[i + k]!));
  }
  const cell = job.scale / 100000;
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (let i = 0; i < tris.length; i += 2) {
    x0 = Math.min(x0, tris[i]!);
    x1 = Math.max(x1, tris[i]!);
    y0 = Math.min(y0, tris[i + 1]!);
    y1 = Math.max(y1, tris[i + 1]!);
  }
  x0 -= 2 * cell;
  y0 -= 2 * cell;
  const W = Math.ceil((x1 - x0) / cell) + 4;
  const H = Math.ceil((y1 - y0) / cell) + 4;
  const mask = new Uint8Array(W * H);
  for (let i = 0; i < tris.length; i += 6) {
    const [ax, ay, bx, by, cx, cy] = [0, 1, 2, 3, 4, 5].map((k) => (tris[i + k]! - (k % 2 === 0 ? x0 : y0)) / cell) as [number, number, number, number, number, number];
    const det = (bx - ax) * (cy - ay) - (cx - ax) * (by - ay);
    if (Math.abs(det) < 1e-12) continue;
    const [lx, hx, ly, hy] = [Math.max(0, Math.floor(Math.min(ax, bx, cx))), Math.min(W - 1, Math.ceil(Math.max(ax, bx, cx))), Math.max(0, Math.floor(Math.min(ay, by, cy))), Math.min(H - 1, Math.ceil(Math.max(ay, by, cy)))];
    for (let y = ly; y <= hy; y++)
      for (let x = lx; x <= hx; x++) {
        const [px, py] = [x + 0.5, y + 0.5];
        const u = ((bx - ax) * (py - ay) - (px - ax) * (by - ay)) / det;
        const v = ((px - ax) * (cy - ay) - (cx - ax) * (py - ay)) / det;
        if (u >= -1e-9 && v >= -1e-9 && u + v <= 1 + 1e-9) mask[y * W + x] = 1;
      }
  }
  if (job.clip !== undefined)
    for (let y = 0; y < H; y++)
      for (let x = 0; x < W; x++) {
        const c = job.clip.axis === 0 ? x0 + (x + 0.5) * cell : y0 + (y + 0.5) * cell;
        if (c < job.clip.min || c > job.clip.max) mask[y * W + x] = 0;
      }
  const at = (x: number, y: number): number => (x < 0 || y < 0 || x >= W || y >= H ? 0 : mask[y * W + x]!);
  const V = W + 1;
  const next = new Map<number, number[]>();
  const edge = (ax: number, ay: number, bx: number, by: number): void => {
    const k = ay * V + ax;
    const l = next.get(k);
    if (l === undefined) next.set(k, [by * V + bx]);
    else l.push(by * V + bx);
  };
  for (let y = 0; y < H; y++)
    for (let x = 0; x < W; x++) {
      if (at(x, y) === 0) continue;
      if (at(x, y - 1) === 0) edge(x, y, x + 1, y);
      if (at(x + 1, y) === 0) edge(x + 1, y, x + 1, y + 1);
      if (at(x, y + 1) === 0) edge(x + 1, y + 1, x, y + 1);
      if (at(x - 1, y) === 0) edge(x, y + 1, x, y);
    }
  let bestLoop: [number, number][] = [];
  let bestArea = 0;
  const areas: number[] = [];
  for (const start of [...next.keys()]) {
    while ((next.get(start)?.length ?? 0) > 0) {
      const loop: [number, number][] = [];
      let cur = start;
      do {
        loop.push([cur % V, Math.floor(cur / V)]);
        const l = next.get(cur)!;
        cur = l.pop()!;
      } while (cur !== start && (next.get(cur)?.length ?? 0) > 0);
      let area = 0;
      for (let i = 0; i < loop.length; i++) {
        const [ax, ay] = loop[i]!;
        const [bx, by] = loop[(i + 1) % loop.length]!;
        area += ax * by - bx * ay;
      }
      if (area > 0) areas.push(area);
      if (area > bestArea) [bestArea, bestLoop] = [area, loop];
    }
  }
  const lost = areas.reduce((sum, a) => sum + a, 0) - bestArea;
  if (lost > 0.03 * bestArea) process.stdout.write(`SPLIT ${job.key} solids ${job.solids.join(",")}: ${areas.length} outlines, ${((100 * lost) / (lost + bestArea)).toFixed(0)} % of the area is outside the largest one\n`);
  const tol = (job.tol ?? 1.6) * cell;
  const world = bestLoop.map(([cx, cy]): [number, number] => [x0 + cx * cell, y0 + cy * cell]);
  const swept = job.bone !== undefined && job.pose === undefined && (job.view === "lateral" || job.sweep === true);
  out[job.key] = simplify(world, tol).map((p) => (swept ? turn(p, -1) : p)).map(([a, b]): [number, number] => [Math.round(a * 1e4) / 1e4, Math.round(b * 1e4) / 1e4]);
  if (out[job.key]!.length < 3) throw new Error(`${job.key}: empty outline`);
}
writeFileSync(outPath!, JSON.stringify(out));
writeFileSync(`${outPath!}.spans.json`, JSON.stringify(spans));
process.stdout.write(`${Object.keys(out).length} outlines, ${Object.values(out).reduce((n, r) => n + r.length, 0)} points\n`);
