/** 🔷 Refits drawn shapes to the convex hull of the projected volume solids: bun humansb-A-hullfit.ts <asset> <scale> <view> <out.json> <shapeId:solid+solid[,shapeId:solid...]>   env PATCH=ops.json, EPS=0.1 (fraction of the feature for the line simplification), OFFSET=0 (metres the hull grows). Writes `shape` ops that turn the shape into a closed polygon (path stays a path) on the same bone. */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, out, spec] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string, string];
const scale = Number(scaleArg);
const feature = featureOf(scale);
const eps = Number(process.env.EPS ?? 0.1) * feature;
const grow = Number(process.env.OFFSET ?? 0);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const find = (list: any[], id: string) => list.find((x) => x.id === id);
if (process.env.PATCH) {
  for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
const geometry = (): any[] => (asset as any).representations[scaleArg].geometry[view];
const run = () => evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const COMMON = new Set(["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm", "tags", "dash", "texture"]);
const cross = (o: any, a: any, b: any) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
const hullOf = (pts: { x: number; y: number }[]) => {
  const p = [...pts].sort((a, b) => a.x - b.x || a.y - b.y);
  const half = (list: any[]) => { const h: any[] = []; for (const q of list) { while (h.length >= 2 && cross(h[h.length - 2], h[h.length - 1], q) <= 0) h.pop(); h.push(q); } h.pop(); return h; };
  return [...half(p), ...half([...p].reverse())];
};
const dp = (pts: { x: number; y: number }[], e: number): { x: number; y: number }[] => {
  if (pts.length < 3) return pts;
  let idx = 0, dmax = 0;
  const a = pts[0]!, b = pts[pts.length - 1]!;
  const l = Math.hypot(b.x - a.x, b.y - a.y);
  for (let i = 1; i < pts.length - 1; i++) { const d = l === 0 ? Math.hypot(pts[i]!.x - a.x, pts[i]!.y - a.y) : Math.abs(cross(a, b, pts[i]) / l); if (d > dmax) { dmax = d; idx = i; } }
  if (dmax <= e) return [a, b];
  return [...dp(pts.slice(0, idx + 1), e).slice(0, -1), ...dp(pts.slice(idx), e)];
};
const simplify = (ring: { x: number; y: number }[], e: number) => {
  let far = 0;
  for (let i = 1; i < ring.length; i++) if (Math.hypot(ring[i]!.x - ring[0]!.x, ring[i]!.y - ring[0]!.y) > Math.hypot(ring[far]!.x - ring[0]!.x, ring[far]!.y - ring[0]!.y)) far = i;
  const rot = [...ring.slice(far), ...ring.slice(0, far)];
  const half = rot.findIndex((_, i) => i > 0 && i >= 0 && false);
  void half;
  const mid = Math.floor(rot.length / 2);
  const a = dp(rot.slice(0, mid + 1), e), b = dp([...rot.slice(mid), rot[0]!], e);
  return [...a.slice(0, -1), ...b.slice(0, -1)];
};
const simplifyClosed = (ring: { x: number; y: number }[], e: number) => {
  let far = 0;
  for (let i = 1; i < ring.length; i++) if (Math.hypot(ring[i]!.x - ring[0]!.x, ring[i]!.y - ring[0]!.y) > Math.hypot(ring[far]!.x - ring[0]!.x, ring[far]!.y - ring[0]!.y)) far = i;
  const rot = [...ring.slice(far), ...ring.slice(0, far)];
  const a = dp(rot.slice(0, Math.floor(rot.length / 2) + 1), e), b = dp([...rot.slice(Math.floor(rot.length / 2)), rot[0]!], e);
  return [...a.slice(0, -1), ...b.slice(0, -1)];
};
const ops: any[] = [];
for (const item of spec.split(",")) {
  const [id, sol] = item.split(":") as [string, string];
  const shape = find(geometry(), id);
  const saved = JSON.parse(JSON.stringify(shape));
  const probe: any = { kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true };
  for (const k of Object.keys(saved)) if (COMMON.has(k) && k !== "texture") probe[k] = saved[k];
  const list = geometry();
  list[list.indexOf(shape)] = probe;
  const st = run();
  const ev = st.shapes[view].find((s: any) => s.shape.id === id);
  list[list.indexOf(probe)] = shape;
  if (ev === undefined || ev.rings[0].length < 3) { console.log(id, "probe failed"); continue; }
  const r = ev.rings[0] as { x: number; y: number }[];
  const o = r[0]!, ux = { x: r[1]!.x - o.x, y: r[1]!.y - o.y }, vy = { x: r[2]!.x - o.x, y: r[2]!.y - o.y };
  const det = ux.x * vy.y - ux.y * vy.x;
  const toLocal = (p: { x: number; y: number }) => { const X = p.x - o.x, Y = p.y - o.y; return [(X * vy.y - Y * vy.x) / det, (-X * ux.y + Y * ux.x) / det]; };
  const state = run();
  const wants = sol.split("+").map((x) => { const m = /^(.*?)([<>])(-?[\d.]+)$/.exec(x); return m === null ? { id: x, dir: 0, at: 0 } : { id: m[1]!, dir: m[2] === ">" ? 1 : -1, at: Number(m[3]) }; });
  const pts: { x: number; y: number }[] = [];
  for (const w of worldSolids(asset, state.bones as never, scale)) for (const want of wants) if (want.id === w.id) for (let i = 0; i < w.positions.length; i += 3) { const p = project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); if (want.dir === 0 || (p.x - want.at) * want.dir > 0) pts.push(p); }
  let hull = hullOf(pts);
  if (process.env.TRACE !== undefined) {
    const cell = feature / 4;
    const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
    const box = { minX: Math.min(...xs) - 2 * cell, minY: Math.min(...ys) - 2 * cell, maxX: Math.max(...xs) + 2 * cell, maxY: Math.max(...ys) + 2 * cell };
    const mask = new Mask(box, cell);
    for (const w of worldSolids(asset, state.bones as never, scale)) for (const want of wants) if (want.id === w.id) for (let k = 0; k < w.indices.length; k += 3) {
      const ring = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as never, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); });
      if (want.dir === 0 || ring.every((p) => (p.x - want.at) * want.dir > 0)) mask.fill([ring], true);
    }
    mask.seal();
    const W = mask.width, H = mask.height;
    const dil = Number(process.env.DILATE ?? 0);
    let c = mask.cells;
    for (let r = 0; r < dil; r++) {
      const n = new Uint8Array(c);
      for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (c[j * W + i]) for (let dj = -1; dj <= 1; dj++) for (let di = -1; di <= 1; di++) { const a = i + di, b = j + dj; if (a >= 0 && b >= 0 && a < W && b < H) n[b * W + a] = 1; }
      c = n;
    }
    const at = (i: number, j: number) => i >= 0 && j >= 0 && i < W && j < H && c[j * W + i] === 1;
    let start = -1;
    for (let k = 0; k < W * H && start < 0; k++) if (c[k]) start = k;
    const dirs = [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]];
    const si = start % W, sj = Math.floor(start / W);
    const trace: { x: number; y: number }[] = [];
    let ci = si, cj = sj, back = 4, guard = 0;
    do {
      trace.push({ x: box.minX + (ci + 0.5) * cell, y: box.minY + (cj + 0.5) * cell });
      let found = false;
      for (let t = 1; t <= 8; t++) {
        const d = (back + t) % 8;
        const ni = ci + dirs[d]![0]!, nj = cj + dirs[d]![1]!;
        if (at(ni, nj)) { back = (d + 4) % 8; ci = ni; cj = nj; found = true; break; }
      }
      if (!found) break;
    } while ((ci !== si || cj !== sj) && ++guard < 1_000_000);
    hull = trace;
  }
  if (grow !== 0) { const cx = hull.reduce((s, p) => s + p.x, 0) / hull.length, cy = hull.reduce((s, p) => s + p.y, 0) / hull.length; hull = hull.map((p) => { const d = Math.hypot(p.x - cx, p.y - cy); return { x: p.x + ((p.x - cx) / d) * grow, y: p.y + ((p.y - cy) / d) * grow }; }); }
  const simp = process.env.TRACE !== undefined ? simplifyClosed(hull, eps) : simplify(hull, eps);
  const local = simp.map(toLocal).map((p) => [+p[0]!.toFixed(4), +p[1]!.toFixed(4)]);
  const next: any = {};
  for (const k of Object.keys(saved)) if (COMMON.has(k)) next[k] = saved[k];
  if (saved.kind === "path") { next.kind = "path"; next.d = "M" + local.map((p) => `${p[0]!.toFixed(5)} ${p[1]!.toFixed(5)}`).join(" L") + " Z"; }
  else { next.kind = "poly"; next.points = local; next.closed = true; }
  const ordered: any = { kind: next.kind, ...(next.d !== undefined ? { d: next.d } : { points: next.points, closed: true }) };
  for (const k of Object.keys(saved)) if (COMMON.has(k)) ordered[k] = saved[k];
  ops.push({ op: "shape", scale: scaleArg, view, id, shape: ordered });
  console.log(id, "hull", hull.length, "points", local.length);
}
await Bun.write(out, JSON.stringify(ops));
