/** ✏️ Traces the outline of the volume solids selected by id substrings in a view and prints it as a simplified polygon in the local coordinates of a drawn shape (translation fitted from the shape's world ring and its raw bounding box).
 *  bun animals-appliances-components-b-trace.ts <asset> <scale> <view> <shapeId> <solidSubstr,solidSubstr|bone:id> [tolCells=0.7]
 *  Batch: shapeId `@` and the selector a path of a JSON file [{id, sel}]: prints one JSON object id to {pts, info} or {err}. */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, measureView, project, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, shapeArg, selArg, tolArg] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const all = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const mm = (list: readonly any[]) => { const xs = list.flat().map((p: any) => p.x), ys = list.flat().map((p: any) => p.y); return [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)]; };
const CLOSE = Number(process.env.CLOSE ?? 2);
let gridCache: { cell: number; box: { minX: number; minY: number; maxX: number; maxY: number } } | undefined;
function gridOf() {
  if (gridCache === undefined) { const m = measureView(asset, state, all, view as never, scale, undefined, feature, Infinity, false); gridCache = { cell: m.cell, box: m.box }; }
  return gridCache;
}
function traceOne(shapeId: string, sel: string): { pts: number[][]; info: string } {
  const clip = sel.startsWith("&");
  const keys = (clip ? sel.slice(1) : sel).split(",");
  const chosen = all.filter((s) => keys.some((k) => (k.startsWith("bone:") ? s.bone === k.slice(5) : s.id.includes(k))));
  if (chosen.length === 0) throw new Error("no solids");
  const ref = referenceOf(view as never, chosen, undefined, undefined);
  const [x0, x1, y0, y1] = mm(ref.full.flat());
  if (!Number.isFinite(x0)) throw new Error("empty projection");
  const grid = process.env.GRID === "1" ? gridOf() : undefined;
  const cell = grid?.cell ?? feature / Number(process.env.FINE ?? 6);
  const mask = new Mask(grid?.box ?? { minX: x0! - cell * 3, minY: y0! - cell * 3, maxX: x1! + cell * 3, maxY: y1! + cell * 3 }, cell);
  for (const g of ref.full) mask.fill(g, true);
  mask.seal();
  const { width: w, height: h, box } = mask;
  const close = (src: Uint8Array, r: number) => {
    const dil = new Uint8Array(src.length), out = new Uint8Array(src.length);
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { let v = 0; for (let b = -r; b <= r && !v; b++) for (let a = -r; a <= r; a++) { const ii = i + a, jj = j + b; if (ii >= 0 && jj >= 0 && ii < w && jj < h && src[jj * w + ii]) { v = 1; break; } } dil[j * w + i] = v; }
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { let v = 1; for (let b = -r; b <= r && v; b++) for (let a = -r; a <= r; a++) { const ii = i + a, jj = j + b; if (ii < 0 || jj < 0 || ii >= w || jj >= h || !dil[jj * w + ii]) { v = 0; break; } } out[j * w + i] = v; }
    return out;
  };
  let base = CLOSE > 0 ? close(mask.cells, CLOSE) : mask.cells;
  if (clip) {
    const pm = new Mask(box, cell);
    const ev = (state.shapes as any)[view].find((s: any) => s.shape.id === shapeId);
    pm.fill(ev.rings as never);
    base = base.map((v, k) => (v && pm.cells[k] ? 1 : 0)) as unknown as Uint8Array;
  }
  const cells = base;
  const on = (i: number, j: number) => i >= 0 && j >= 0 && i < w && j < h && cells[j * w + i] !== 0;
  const edges = new Map<string, [number, number][]>();
  const add = (a: [number, number], b: [number, number]) => { const k = a.join(","); const l = edges.get(k); if (l) l.push(b); else edges.set(k, [b]); };
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (!on(i, j)) continue;
    if (!on(i, j - 1)) add([i, j], [i + 1, j]);
    if (!on(i + 1, j)) add([i + 1, j], [i + 1, j + 1]);
    if (!on(i, j + 1)) add([i + 1, j + 1], [i, j + 1]);
    if (!on(i - 1, j)) add([i, j + 1], [i, j]);
  }
  const loops: [number, number][][] = [];
  while (edges.size > 0) {
    const [k0] = edges.entries().next().value as [string, [number, number][]];
    const start = k0.split(",").map(Number) as [number, number];
    const loop: [number, number][] = [start];
    let cur = start;
    for (;;) {
      const key = cur.join(",");
      const l = edges.get(key);
      if (!l) break;
      const nxt = l.pop()!;
      if (l.length === 0) edges.delete(key);
      if (nxt[0] === start[0] && nxt[1] === start[1]) break;
      loop.push(nxt);
      cur = nxt;
    }
    loops.push(loop);
  }
  const area = (l: [number, number][]) => { let a = 0; for (let i = 0; i < l.length; i++) { const p = l[i]!, q = l[(i + 1) % l.length]!; a += p[0] * q[1] - q[0] * p[1]; } return a / 2; };
  loops.sort((a, b) => Math.abs(area(b)) - Math.abs(area(a)));
  const tol = Number(tolArg ?? process.env.TOL ?? 1.2) * cell;
  const dp = (pts: { x: number; y: number }[]): { x: number; y: number }[] => {
    if (pts.length < 3) return pts;
    const a = pts[0]!, b = pts[pts.length - 1]!;
    let best = -1, bi = 0;
    const dx = b.x - a.x, dy = b.y - a.y, L = Math.hypot(dx, dy);
    for (let i = 1; i < pts.length - 1; i++) {
      const p = pts[i]!;
      const d = L === 0 ? Math.hypot(p.x - a.x, p.y - a.y) : Math.abs(dy * (p.x - a.x) - dx * (p.y - a.y)) / L;
      if (d > best) { best = d; bi = i; }
    }
    if (best <= tol) return [a, b];
    return [...dp(pts.slice(0, bi + 1)).slice(0, -1), ...dp(pts.slice(bi))];
  };
  const simplify = (l: [number, number][]) => {
    const o = l.map(([i, j]) => ({ x: box.minX + i * cell, y: box.minY + j * cell }));
    let far = 0;
    for (let i = 1; i < o.length; i++) if (Math.hypot(o[i]!.x - o[0]!.x, o[i]!.y - o[0]!.y) > Math.hypot(o[far]!.x - o[0]!.x, o[far]!.y - o[0]!.y)) far = i;
    const h1 = dp(o.slice(0, far + 1)), h2 = dp([...o.slice(far), o[0]!]);
    return [...h1.slice(0, -1), ...h2.slice(0, -1)];
  };
  const sign = Math.sign(area(loops[0]!));
  const parts = loops.filter((l) => Math.sign(area(l)) === sign && Math.abs(area(l)) >= 0.02 * Math.abs(area(loops[0]!)) && l.length >= 4).map(simplify);
  let outer = parts[0]!;
  for (const q of parts.slice(1)) {
    let bi = 0, bj = 0, bd = Infinity;
    for (let i = 0; i < outer.length; i++) for (let j = 0; j < q.length; j++) { const d = Math.hypot(outer[i]!.x - q[j]!.x, outer[i]!.y - q[j]!.y); if (d < bd) { bd = d; bi = i; bj = j; } }
    outer = [...outer.slice(0, bi + 1), ...q.slice(bj), ...q.slice(0, bj + 1), ...outer.slice(bi)];
  }
  return { pts: outer.map((p) => [p.x, p.y]), info: `solids ${chosen.length} points ${outer.length}` };
}
type Aff = { A: number[]; tx: number; ty: number };
function fit(local: number[][], world: { x: number; y: number }[]): Aff & { resid: number } {
  const n = local.length;
  const mx = local.reduce((s, p) => s + p[0]!, 0) / n, my = local.reduce((s, p) => s + p[1]!, 0) / n;
  const qx = world.reduce((s, p) => s + p.x, 0) / n, qy = world.reduce((s, p) => s + p.y, 0) / n;
  let sxx = 0, sxy = 0, syy = 0, ax = 0, ay = 0, bx = 0, by = 0;
  for (let k = 0; k < n; k++) {
    const dx = local[k]![0]! - mx, dy = local[k]![1]! - my, ex = world[k]!.x - qx, ey = world[k]!.y - qy;
    sxx += dx * dx; sxy += dx * dy; syy += dy * dy; ax += dx * ex; ay += dy * ex; bx += dx * ey; by += dy * ey;
  }
  const det = sxx * syy - sxy * sxy;
  const m00 = (ax * syy - ay * sxy) / det, m01 = (ay * sxx - ax * sxy) / det, m10 = (bx * syy - by * sxy) / det, m11 = (by * sxx - bx * sxy) / det;
  const tx = qx - (m00 * mx + m01 * my), ty = qy - (m10 * mx + m11 * my);
  let resid = 0;
  for (let k = 0; k < n; k++) resid = Math.max(resid, Math.hypot(m00 * local[k]![0]! + m01 * local[k]![1]! + tx - world[k]!.x, m10 * local[k]![0]! + m11 * local[k]![1]! + ty - world[k]!.y));
  return { A: [m00, m01, m10, m11], tx, ty, resid };
}
function mapAll(targets: Record<string, number[][]>): Record<string, { pts: number[][]; info: string }> {
  const geometry = (asset.representations as any)[String(scale)].geometry[view] as any[];
  const origin = (bone: string) => { const o = state.bones[bone].frame.o; return project(view as never, o.x ?? o[0], o.y ?? o[1], o.z ?? o[2]); };
  const local: Record<string, number[][]> = {};
  const flags: Record<string, boolean | undefined> = {};
  for (const [id, world] of Object.entries(targets)) {
    const raw = geometry.find((s) => s.id === id);
    flags[id] = raw.followTip;
    const o = origin(raw.bone);
    local[id] = world.map((p) => [p[0]! - o.x, p[1]! - o.y]);
  }
  const probe = (pts: Record<string, number[][]>) => {
    const test = geometry.map((s) => (s.id in pts ? { id: s.id, bone: s.bone, role: "body", filled: true, kind: "poly", points: pts[s.id], closed: true, ...(flags[s.id] === undefined ? {} : { followTip: flags[s.id] }) } : s));
    const asset2 = { ...asset, representations: { ...asset.representations, [String(scale)]: { ...(asset.representations as any)[String(scale)], geometry: { ...(asset.representations as any)[String(scale)].geometry, [view]: test } } } } as Asset;
    const st = evaluate(asset2, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
    const out: Record<string, { x: number; y: number }[]> = {};
    for (const id of Object.keys(pts)) out[id] = st.shapes[view].find((s: any) => s.shape.id === id).rings[0];
    return out;
  };
  const infos: Record<string, string> = {};
  for (let it = 0; it < 4; it++) {
    const got = probe(local);
    for (const id of Object.keys(local)) {
      const g = got[id]!;
      const n = local[id]!.length;
      if (g.length < n) { infos[id] = "ring shorter"; continue; }
      const f = fit(local[id]!, g.slice(0, n));
      const det = f.A[0]! * f.A[3]! - f.A[1]! * f.A[2]!;
      const world = targets[id]!;
      local[id] = world.map((p) => { const X = p[0]! - f.tx, Y = p[1]! - f.ty; return [(f.A[3]! * X - f.A[1]! * Y) / det, (-f.A[2]! * X + f.A[0]! * Y) / det]; });
      infos[id] = `affine ${f.A.map((v) => +v.toFixed(3))} resid ${f.resid.toFixed(4)}`;
    }
  }
  const final = probe(local);
  const res: Record<string, { pts: number[][]; info: string }> = {};
  for (const id of Object.keys(local)) {
    let err = 0;
    const g = final[id]!, w = targets[id]!;
    for (let k = 0; k < w.length; k++) err = Math.max(err, Math.hypot(g[k]!.x - w[k]![0]!, g[k]!.y - w[k]![1]!));
    res[id] = { pts: local[id]!.map((p) => [+p[0]!.toFixed(5), +p[1]!.toFixed(5)]), info: `[DEBUG] ${infos[id] ?? ""} maxErr ${err.toFixed(4)}` };
  }
  return res;
}
const jobs = shapeArg === "@" ? (JSON.parse(await Bun.file(selArg!).text()) as { id: string; sel: string }[]) : [{ id: shapeArg!, sel: selArg! }];
const worlds: Record<string, number[][]> = {};
const out: Record<string, unknown> = {};
for (const j of jobs) {
  try { const r = traceOne(j.id, j.sel); worlds[j.id] = r.pts; out[j.id] = { info: r.info }; } catch (e) { out[j.id] = { err: String(e).slice(0, 80) }; }
}
const mapped = Object.keys(worlds).length > 0 ? mapAll(worlds) : {};
for (const [id, r] of Object.entries(mapped)) out[id] = { pts: r.pts, info: r.info + " " + (out[id] as any).info };
if (shapeArg === "@") console.log("@@" + JSON.stringify(out));
else { const r = out[shapeArg!] as any; console.log(r.info ?? r.err); console.log(JSON.stringify(r.pts ?? [])); }
