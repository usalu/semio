/** 🧲 Drags the boundary vertices of drawn polygons (world position, solved through the exact carry of the evaluator) onto the boundary of the volume of one view, splitting edges where the drawn boundary leaves the volume, and writes the result as ops for humans-try.ts / humans-ops.py.
 *  bun humansb-B-snap.ts <asset> <scale> <view> <out.json> [--in patch.json] [--only id,id] [--skip id,id] [--tol mm] [--reach mm] [--rounds n] [--cover] */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const argv = process.argv.slice(2);
const [name, scaleArg, view, out] = argv as [string, string, "elevation" | "front" | "plan", string];
const opt = (k: string): string | undefined => (argv.indexOf(k) >= 0 ? argv[argv.indexOf(k) + 1] : undefined);
const scale = Number(scaleArg);
const feature = featureOf(scale as never);
const tol = Number(opt("--tol") ?? 4) / 1000;
const reach = Number(opt("--reach") ?? 70) / 1000;
const rounds = Number(opt("--rounds") ?? 4);
const cover = argv.includes("--cover");
const only = opt("--only")?.split(",");
const skip = new Set(opt("--skip")?.split(",") ?? []);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const base = ((await readStreamedAsset(entry)) as { asset: Asset }).asset;
const geometry = (a: Asset): any[] => (a as any).representations[String(scale)].geometry[view];
const pre = opt("--in");
if (pre) {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(readFileSync(pre, "utf8"))) {
    if (String(o.scale) !== String(scale) || o.view !== view || !o.op.startsWith("shape")) continue;
    const list = geometry(base);
    if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
    else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o.shape;
  }
}
const original = new Map<string, any>(geometry(base).map((s) => [s.id, structuredClone(s)]));
const current = geometry(base).map((s) => structuredClone(s));
const build = (list: any[]): Asset => {
  const a = structuredClone(base) as any;
  a.representations[String(scale)].geometry[view] = structuredClone(list);
  return a as Asset;
};
const evalAsset = (a: Asset) => evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never) as any;
const ringsOf = (state: any): Map<string, { x: number; y: number }[]> => new Map(state.shapes[view].map((s: any) => [s.shape.id, s.rings[0]]));
const movable = (s: any): boolean => s.kind === "poly" && s.solid === undefined && s.closed !== false && s.filled !== false && !skip.has(s.id) && (only === undefined || only.includes(s.id));

type Pt = { x: number; y: number };
const dist2 = (a: Pt, b: Pt) => (a.x - b.x) ** 2 + (a.y - b.y) ** 2;
const nearestOnSeg = (p: Pt, a: Pt, b: Pt): { t: number; q: Pt } => {
  const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy;
  const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l));
  return { t, q: { x: a.x + t * dx, y: a.y + t * dy } };
};

const changed = new Set<string>();
for (let round = 0; round < rounds; round++) {
  const asset = build(current);
  const state = evalAsset(asset);
  const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: any) => w.texture !== true);
  const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature, Infinity, false);
  const { cell, box } = m;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.round((4 * feature / cell) ** 2);
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const toVol = distanceField(bv, w, h);
  const toDrawn = distanceField(bd, w, h);
  const cellOf = (p: Pt) => ({ i: Math.floor((p.x - box.minX) / cell), j: Math.floor((p.y - box.minY) / cell) });
  const inside = (i: number, j: number) => i >= 0 && j >= 0 && i < w && j < h;
  const dv = (p: Pt) => { const { i, j } = cellOf(p); return inside(i, j) ? Math.sqrt(toVol[j * w + i]!) * cell : Infinity; };
  const dd = (p: Pt) => { const { i, j } = cellOf(p); return inside(i, j) ? Math.sqrt(toDrawn[j * w + i]!) * cell : Infinity; };
  const nearestVol = (p: Pt, r: number): Pt | undefined => {
    const { i, j } = cellOf(p);
    const n = Math.ceil(r / cell);
    let best: Pt | undefined, bd2 = r * r;
    for (let dj = -n; dj <= n; dj++) for (let di = -n; di <= n; di++) {
      const ii = i + di, jj = j + dj;
      if (!inside(ii, jj) || !bv[jj * w + ii]) continue;
      const q = { x: box.minX + (ii + 0.5) * cell, y: box.minY + (jj + 0.5) * cell };
      const d2 = dist2(p, q);
      if (d2 < bd2) { bd2 = d2; best = q; }
    }
    return best;
  };
  const rings = ringsOf(state);
  const targets = current.filter(movable).filter((s) => rings.has(s.id) && rings.get(s.id)!.length === s.points.length);
  const probe = (dx: number, dy: number) => {
    const list = current.map((s) => structuredClone(s));
    for (const s of list) if (targets.some((t) => t.id === s.id)) s.points = s.points.map((p: number[]) => [p[0]! + dx, p[1]! + dy]);
    return ringsOf(evalAsset(build(list)));
  };
  const eps = 1e-3;
  const rx = probe(eps, 0), ry = probe(0, eps);
  const jac = (id: string, k: number) => {
    const a = rings.get(id)![k]!, bx = rx.get(id)![k]!, by = ry.get(id)![k]!;
    return { a: (bx.x - a.x) / eps, c: (bx.y - a.y) / eps, b: (by.x - a.x) / eps, d: (by.y - a.y) / eps };
  };
  const solve = (id: string, k: number, dwx: number, dwy: number): [number, number] | undefined => {
    const { a, b, c, d } = jac(id, k);
    const det = a * d - b * c;
    if (Math.abs(det) < 0.15) return undefined;
    return [(d * dwx - b * dwy) / det, (-c * dwx + a * dwy) / det];
  };
  const onDrawn = (p: Pt) => dd(p) <= 1.6 * cell;
  let moves = 0, splits = 0;
  const next: any[] = [];
  const wanted = new Map<string, Map<number, Pt>>();
  const inserts = new Map<string, { after: number; t: number; target: Pt }[]>();
  for (const s of targets) {
    const ring = rings.get(s.id)!;
    for (let k = 0; k < ring.length; k++) {
      const p = ring[k]!;
      if (opt("--debug") === s.id) { const j = jac(s.id, k); console.log(`[DEBUG] ${s.id}[${k}] (${p.x.toFixed(3)}, ${p.y.toFixed(3)}) onDrawn ${onDrawn(p)} dv ${dv(p).toFixed(3)} det ${(j.a * j.d - j.b * j.c).toFixed(2)}`); }
      if (!onDrawn(p)) continue;
      const q = dv(p) > tol ? nearestVol(p, reach) : undefined;
      if (q === undefined) continue;
      if (!wanted.has(s.id)) wanted.set(s.id, new Map());
      wanted.get(s.id)!.set(k, q);
    }
    for (let k = 0; k < ring.length; k++) {
      const a = ring[k]!, b = ring[(k + 1) % ring.length]!;
      const len = Math.sqrt(dist2(a, b));
      const steps = Math.max(1, Math.floor(len / (2 * cell)));
      let worst = 0, wt = 0;
      for (let u = 1; u < steps; u++) {
        const t = u / steps;
        const p = { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t };
        if (!onDrawn(p)) continue;
        const d = dv(p);
        if (d > worst) { worst = d; wt = t; }
      }
      if (worst > 1.5 * tol) {
        const t = wt, p = { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t };
        const q = nearestVol(p, reach);
        if (q !== undefined) {
          if (!inserts.has(s.id)) inserts.set(s.id, []);
          inserts.get(s.id)!.push({ after: k, t, target: q });
        }
      }
    }
  }
  if (cover) {
    const list: { p: Pt; d: number }[] = [];
    const step = Math.max(1, Math.round(0.02 / cell));
    for (let j = 0; j < h; j += step) for (let i = 0; i < w; i += step) if (bv[j * w + i]) {
      const d = Math.sqrt(toDrawn[j * w + i]!) * cell;
      if (d > 1.5 * tol && d < reach) list.push({ p: { x: box.minX + (i + 0.5) * cell, y: box.minY + (j + 0.5) * cell }, d });
    }
    list.sort((a, b) => b.d - a.d);
    const taken: Pt[] = [];
    for (const c of list) {
      if (taken.some((t) => dist2(t, c.p) < (0.025) ** 2)) continue;
      let best: { id: string; k: number; t: number; d2: number } | undefined;
      for (const s of targets) {
        const ring = rings.get(s.id)!;
        for (let k = 0; k < ring.length; k++) {
          const a = ring[k]!, b = ring[(k + 1) % ring.length]!;
          const { t, q } = nearestOnSeg(c.p, a, b);
          if (!onDrawn(q)) continue;
          const d2 = dist2(q, c.p);
          if (best === undefined || d2 < best.d2) best = { id: s.id, k, t, d2 };
        }
      }
      if (best === undefined || best.d2 > reach * reach) continue;
      taken.push(c.p);
      if (!inserts.has(best.id)) inserts.set(best.id, []);
      inserts.get(best.id)!.push({ after: best.k, t: best.t, target: c.p });
    }
  }
  for (const s of current) {
    if (!targets.some((t) => t.id === s.id)) { next.push(s); continue; }
    const ring = rings.get(s.id)!;
    const pts: number[][] = [];
    const add = inserts.get(s.id) ?? [];
    for (let k = 0; k < s.points.length; k++) {
      let cur = s.points[k]!;
      const target = wanted.get(s.id)?.get(k);
      if (target !== undefined) {
        const d = solve(s.id, k, target.x - ring[k]!.x, target.y - ring[k]!.y);
        if (d !== undefined) { cur = [cur[0]! + d[0], cur[1]! + d[1]]; moves++; changed.add(s.id); }
      }
      pts.push(cur);
      const here = add.filter((x) => x.after === k).sort((a, b) => a.t - b.t);
      for (const x of here) {
        const nb = s.points[(k + 1) % s.points.length]!;
        const base0 = s.points[k]!;
        const rest = [base0[0]! + (nb[0]! - base0[0]!) * x.t, base0[1]! + (nb[1]! - base0[1]!) * x.t];
        const j = jac(s.id, k);
        const at = { x: ring[k]!.x + (ring[(k + 1) % ring.length]!.x - ring[k]!.x) * x.t, y: ring[k]!.y + (ring[(k + 1) % ring.length]!.y - ring[k]!.y) * x.t };
        const dwx = x.target.x - at.x, dwy = x.target.y - at.y;
        const det = j.a * j.d - j.b * j.c;
        if (Math.abs(det) < 0.15) continue;
        pts.push([rest[0]! + (j.d * dwx - j.b * dwy) / det, rest[1]! + (-j.c * dwx + j.a * dwy) / det]);
        splits++; changed.add(s.id);
      }
    }
    next.push({ ...s, points: pts });
  }
  current.splice(0, current.length, ...next);
  const after = measureView(build(current), evalAsset(build(current)), solids, view as never, scale as never, undefined, feature, Infinity, false);
  console.log(`[DEBUG] round ${round + 1} moves ${moves} splits ${splits} start iou ${m.iou.toFixed(3)} h ${(m.hausdorffM * 1000).toFixed(1)}mm after iou ${after.iou.toFixed(3)} h ${(after.hausdorffM * 1000).toFixed(1)}mm`);
  if (moves + splits === 0) break;
}
const round5 = (v: number) => Math.round(v * 1e5) / 1e5;
const ops: any[] = [];
for (const s of current) if (changed.has(s.id)) ops.push({ op: "shape", scale: String(scale), view, id: s.id, shape: { ...s, points: s.points.map((p: number[]) => [round5(p[0]!), round5(p[1]!)]) } });
const preOps = pre ? JSON.parse(readFileSync(pre, "utf8")) : [];
writeFileSync(out, JSON.stringify([...preOps, ...ops], null, 1));
console.log(`[DEBUG] wrote ${ops.length} shape ops`);
void original;
