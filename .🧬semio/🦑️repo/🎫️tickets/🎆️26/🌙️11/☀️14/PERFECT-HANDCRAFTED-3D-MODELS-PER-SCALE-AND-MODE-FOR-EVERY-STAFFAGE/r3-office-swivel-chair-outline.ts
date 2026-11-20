/** ✏️ Round 3, furniture/office-swivel-chair: the true orthographic outline of groups of body solids of the candidate asset, as the starting point of the redrawn 2D shapes (ruling E1), and a plain picture of the evaluated 2D drawing of any pose for judging it.
 *
 * `bun r3-office-swivel-chair-outline.ts outline <asset.json> <requests.json> <out.json>`: every request `{ key, scale, view, solids, tol }` gives the outer ring of the union of the projected solids in paper metres (elevation x,y; front z,y; plan x,z), simplified to `tol`, with the world box of the group.
 * `bun r3-office-swivel-chair-outline.ts view <asset.json> --scale N [--pose ID] [--px 1000]`: elevation, front and plan as drawn (fills by tone in paint order, outlines by role).
 * @see ./round3-brief.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, meshesAt, type Json } from "./volumes-lib.ts";
import { Canvas, type RGB } from "./volumes-raster.ts";

type P = [number, number];

function simplify(ring: P[], tol: number): P[] {
  const dp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
    let worst = 0, at = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const d = Math.abs((b[0] - a[0]) * (a[1] - pts[i]![1]) - (a[0] - pts[i]![0]) * (b[1] - a[1])) / len;
      if (d > worst) [worst, at] = [d, i];
    }
    return worst <= tol ? [a, b] : [...dp(pts.slice(0, at + 1)).slice(0, -1), ...dp(pts.slice(at))];
  };
  let far = 0;
  for (let i = 1; i < ring.length; i++) if (Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]) > Math.hypot(ring[far]![0] - ring[0]![0], ring[far]![1] - ring[0]![1])) far = i;
  const first = dp(ring.slice(0, far + 1));
  const second = dp([...ring.slice(far), ring[0]!]);
  return [...first.slice(0, -1), ...second.slice(0, -1)];
}

function outline(points: Float64Array, indices: Uint32Array[], worlds: Float64Array[], view: string, tol: number): { ring: P[]; rings: P[][]; box: number[] } {
  const proj = (w: Float64Array, i: number): P => (view === "elevation" ? [w[i]!, w[i + 1]!] : view === "front" ? [w[i + 2]!, w[i + 1]!] : [w[i]!, w[i + 2]!]);
  const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  let [minX, minY, maxX, maxY] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const w of worlds)
    for (let i = 0; i < w.length; i += 3) {
      for (let k = 0; k < 3; k++) {
        box[k] = Math.min(box[k]!, w[i + k]!);
        box[k + 3] = Math.max(box[k + 3]!, w[i + k]!);
      }
      const p = proj(w, i);
      minX = Math.min(minX, p[0]);
      maxX = Math.max(maxX, p[0]);
      minY = Math.min(minY, p[1]);
      maxY = Math.max(maxY, p[1]);
    }
  const cell = Math.max(0.00004, Math.max(maxX - minX, maxY - minY) / 3200);
  const W = Math.ceil((maxX - minX) / cell) + 4, H = Math.ceil((maxY - minY) / cell) + 4;
  const ox = minX - 2 * cell, oy = minY - 2 * cell;
  const grid = new Uint8Array(W * H);
  worlds.forEach((w, m) => {
    const idx = indices[m]!;
    for (let t = 0; t < idx.length; t += 3) {
      const [a, b, c] = [proj(w, idx[t]! * 3), proj(w, idx[t + 1]! * 3), proj(w, idx[t + 2]! * 3)];
      const j0 = Math.max(0, Math.ceil((Math.min(a[1], b[1], c[1]) - oy) / cell - 0.5)), j1 = Math.min(H - 1, Math.floor((Math.max(a[1], b[1], c[1]) - oy) / cell - 0.5));
      for (let j = j0; j <= j1; j++) {
        const y = oy + (j + 0.5) * cell;
        let lo = Infinity, hi = -Infinity;
        for (const [p, q] of [[a, b], [b, c], [c, a]] as [P, P][])
          if (p[1] <= y !== q[1] <= y) {
            const x = p[0] + ((y - p[1]) / (q[1] - p[1])) * (q[0] - p[0]);
            lo = Math.min(lo, x);
            hi = Math.max(hi, x);
          }
        if (lo > hi) continue;
        const i0 = Math.max(0, Math.ceil((lo - ox) / cell - 0.5)), i1 = Math.min(W - 1, Math.floor((hi - ox) / cell - 0.5));
        for (let i = i0; i <= i1; i++) grid[j * W + i] = 1;
      }
    }
  });
  const raw = grid.slice();
  for (let j = 1; j < H - 1; j++) for (let i = 1; i < W - 1; i++) if (raw[j * W + i] && !(raw[j * W + i - 1] && raw[j * W + i + 1] && raw[(j - 1) * W + i] && raw[(j + 1) * W + i])) grid[j * W + i] = 0;
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= W || j >= H ? 0 : grid[j * W + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (W + 1) + i;
  const edge = (i0: number, j0: number, i1: number, j1: number): void => {
    const k = key(i0, j0);
    (next.get(k) ?? next.set(k, []).get(k)!).push(key(i1, j1));
  };
  for (let j = 0; j < H; j++)
    for (let i = 0; i < W; i++) {
      if (!at(i, j)) continue;
      if (!at(i, j - 1)) edge(i, j, i + 1, j);
      if (!at(i + 1, j)) edge(i + 1, j, i + 1, j + 1);
      if (!at(i, j + 1)) edge(i + 1, j + 1, i, j + 1);
      if (!at(i - 1, j)) edge(i, j + 1, i, j);
    }
  const loops: { ring: P[]; area: number }[] = [];
  const used = new Set<string>();
  for (const [start, outs] of next)
    for (const first of outs) {
      if (used.has(`${start}>${first}`)) continue;
      const loop: number[] = [start];
      let [prev, cur] = [start, first];
      used.add(`${start}>${first}`);
      for (let guard = 0; cur !== start && guard < 4e6; guard++) {
        loop.push(cur);
        const options = (next.get(cur) ?? []).filter((n) => !used.has(`${cur}>${n}`));
        if (options.length === 0) break;
        const dir = (a: number, b: number): P => [(b % (W + 1)) - (a % (W + 1)), Math.floor(b / (W + 1)) - Math.floor(a / (W + 1))];
        const d = dir(prev, cur);
        const pickNext = options.length === 1 ? options[0]! : options.sort((m, n) => { const [dm, dn] = [dir(cur, m), dir(cur, n)]; return (d[0] * dn[1] - d[1] * dn[0]) - (d[0] * dm[1] - d[1] * dm[0]); })[0]!;
        used.add(`${cur}>${pickNext}`);
        [prev, cur] = [cur, pickNext];
      }
      const ring: P[] = loop.map((k) => [ox + (k % (W + 1)) * cell, oy + Math.floor(k / (W + 1)) * cell]);
      let area = 0;
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) area += ring[j]![0] * ring[i]![1] - ring[i]![0] * ring[j]![1];
      loops.push({ ring, area: Math.abs(area) / 2 });
    }
  loops.sort((m, n) => n.area - m.area);
  const fine = Math.min(tol, 0.06 * Math.min(maxX - minX, maxY - minY));
  const round = (ring: P[]): P[] => simplify(ring, Math.max(fine, 1.2 * cell)).map((p) => [Math.round(p[0] * 1e5) / 1e5, Math.round(p[1] * 1e5) / 1e5]);
  const kept = loops.filter((l) => l.area >= Math.max(0.03 * loops[0]!.area, (4 * tol) ** 2)).map((l) => round(l.ring)).filter((r) => r.length >= 3);
  void points;
  return { ring: kept[0] ?? [], rings: kept, box: box.map((v) => Math.round(v * 1e5) / 1e5) };
}

const [mode, file, ...rest] = process.argv.slice(2);
const asset: Json = JSON.parse(readFileSync(file!, "utf8")).asset;
if (mode === "outline") {
  const requests: Json[] = JSON.parse(readFileSync(rest[0]!, "utf8"));
  const out: Record<string, unknown> = {};
  const cache = new Map<string, Json>();
  for (const scale of [50, 100, 200, 500]) {
    const body: Json[] = asset.volumes[String(scale)].body;
    const meshes = meshesAt(asset, scale, body);
    for (const r of requests.filter((q) => q.scale === scale)) {
      const picked = meshes.filter((m) => r.solids.includes(m.id));
      if (picked.length === 0) continue;
      const k = `${scale}|${r.view}|${r.tol}|${[...r.solids].sort().join(",")}`;
      if (!cache.has(k)) cache.set(k, outline(new Float64Array(0), picked.map((m) => m.indices), picked.map((m) => m.world), r.view, r.tol));
      out[r.key] = cache.get(k);
    }
  }
  mkdirSync(dirname(rest[1]!), { recursive: true });
  writeFileSync(rest[1]!, JSON.stringify(out));
  console.log(`${Object.keys(out).length} outlines of ${requests.length} requests`);
} else {
  const flag = (n: string, d: string): string => (rest.indexOf(n) < 0 ? d : rest[rest.indexOf(n) + 1]!);
  const scale = Number(flag("--scale", "50"));
  const pose = flag("--pose", asset.presentation.defaultPose);
  const state = evaluate(asset, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: scale } } as never) as Json;
  const H = Number(flag("--px", "1000"));
  const k = H / 1.25;
  const W = [Math.round(0.95 * k), Math.round(0.95 * k), Math.round(0.95 * k)];
  const canvas = new Canvas(W[0]! + W[1]! + W[2]!, H, [250, 250, 246]);
  const origin: Record<string, P> = { elevation: [0.5 * k, H - 0.12 * k], front: [W[0]! + 0.475 * k, H - 0.12 * k], plan: [W[0]! + W[1]! + 0.5 * k, H * 0.5] };
  const pens = asset.representations[String(scale)].pens;
  for (const view of ["elevation", "front", "plan"]) {
    const o = origin[view]!;
    const shapes = [...(state.shapes[view] as Json[])].map((s, i) => ({ s, i })).sort((a, b) => a.s.shape.z - b.s.shape.z || a.i - b.i).map((e) => e.s);
    for (const s of shapes) {
      const tone = Math.round(255 * s.shape.tone);
      const rings = s.rings as { x: number; y: number }[][];
      if (s.closed && s.shape.filled) {
        const edges: number[][] = [];
        for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) edges.push([o[0] + r[j]!.x * k, o[1] - r[j]!.y * k, o[0] + r[i]!.x * k, o[1] - r[i]!.y * k]);
        const ys = edges.flatMap((e) => [e[1]!, e[3]!]);
        for (let y = Math.max(0, Math.floor(Math.min(...ys))); y <= Math.min(H - 1, Math.ceil(Math.max(...ys))); y++) {
          const xs = edges.filter((e) => e[1]! <= y + 0.5 !== e[3]! <= y + 0.5).map((e) => e[0]! + ((y + 0.5 - e[1]!) / (e[3]! - e[1]!)) * (e[2]! - e[0]!)).sort((a, b) => a - b);
          for (let n = 0; n + 1 < xs.length; n += 2) for (let x = Math.round(xs[n]!); x < Math.round(xs[n + 1]!); x++) canvas.set(x, y, [tone, tone, tone]);
        }
      }
      const penMm = s.shape.penMm ?? pens[s.shape.role] ?? 0.13;
      const width = Math.max(1, Math.round(((penMm * scale) / 1000) * k * 0.35));
      const ink: RGB = s.shape.role === "body" ? [0, 0, 0] : s.shape.role === "detail" ? [50, 50, 50] : [120, 120, 120];
      for (const r of rings)
        for (let i = s.closed ? 0 : 1, j = s.closed ? r.length - 1 : 0; i < r.length; j = i++)
          for (let dx = 0; dx < width; dx++) for (let dy = 0; dy < width; dy++) canvas.line(o[0] + r[j]!.x * k + dx, o[1] - r[j]!.y * k + dy, o[0] + r[i]!.x * k + dx, o[1] - r[i]!.y * k + dy, ink);
    }
    canvas.text(o[0] - 0.42 * k, 8, `${view.toUpperCase()} 1:${scale} ${pose}  ${(state.shapes[view] as Json[]).length} SHAPES`, [20, 20, 20], 2);
  }
  const out = join(GENERATED, "r3-office-swivel-chair", `view2d.s${scale}.${pose}.png`);
  mkdirSync(dirname(out), { recursive: true });
  writeFileSync(out, canvas.png());
  console.log(out);
}
