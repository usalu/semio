/** 🧷 Op for `humans-ops.py` that adds a drawn polygon equal to the hull of volume solids in one view: the points are mapped into the coordinates of the chosen bone by a probe polygon (affine fit of the exact placement of the evaluator).
 *  bun humansb-D-addhull.ts <asset> <scale> <view> <bone> <solidIds,..> <newId> <afterId> <templateId> [tolM] [hullMode: convex|bands] > op.json */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, sc, view, bone, ids, newId, afterId, tmplId, tolArg] = process.argv.slice(2) as [string, string, Viewpoint, string, string, string, string, string, string | undefined];
const scale = Number(sc), tol = Number(tolArg ?? 0.002);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const geo = (a: any): any[] => a.representations[sc].geometry[view];
if (process.env.PATCH) for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) { const l = (asset as any).representations[o.scale].geometry[o.view]; if (o.op === "shape-add") l.splice(l.findIndex((x: any) => x.id === o.after) + 1, 0, o.shape); else if (o.op === "shape") l[l.findIndex((x: any) => x.id === o.id)] = o.shape; else if (o.op === "shape-set") Object.assign(l.find((x: any) => x.id === o.id), o.set); }
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const forward = (points: { x: number; y: number }[]): { x: number; y: number }[] => {
  const probeAsset: any = structuredClone(asset);
  geo(probeAsset).push({ kind: "poly", points: points.map((p) => [p.x, p.y]), closed: true, id: newId, bone, role: "body", z: 1, filled: true, tone: 0.5 });
  const ps: any = evaluate(probeAsset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
  return ps.shapes[view].find((s: any) => s.shape.id === newId && s.shape.z === 1 && s.shape.tone === 0.5).rings[0].slice(0, points.length);
};
const [o, a, b] = forward([{ x: 0, y: 0 }, { x: 0.05, y: 0 }, { x: 0, y: 0.05 }]);
const m = { a: (a.x - o.x) / 0.05, b: (a.y - o.y) / 0.05, c: (b.x - o.x) / 0.05, d: (b.y - o.y) / 0.05 };
const det = m.a * m.d - m.b * m.c;
const inv = (p: { x: number; y: number }) => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (m.d * dx - m.c * dy) / det, y: (-m.b * dx + m.a * dy) / det }; };
const solve = (targets: { x: number; y: number }[]) => {
  let local = targets.map(inv);
  for (let it = 0; it < 6; it++) {
    const got = forward(local);
    const err = targets.map((t, i) => ({ x: t.x - got[i]!.x, y: t.y - got[i]!.y }));
    const worst = Math.max(...err.map((e) => Math.hypot(e.x, e.y)));
    if (process.env.DEBUGSOLVE) console.error("[DEBUG] solve", it, worst);
    if (worst < 2e-4) break;
    if (it === 5 && worst > 0.003) { console.error(`[DEBUG] skip ${view} ${newId}: placement not invertible (${worst.toFixed(4)})`); console.log('[]'); process.exit(0); }
    local = local.map((l, i) => { const d = inv({ x: o.x + err[i]!.x, y: o.y + err[i]!.y }); return { x: l.x + d.x, y: l.y + d.y }; });
  }
  return local;
};
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
const want = new Set(ids.split(","));
const pts: { x: number; y: number }[] = [];
const tris: [any, any, any][] = [];
for (const w of worldSolids(asset, state.bones as never, scale)) if (want.has(w.id)) {
  for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
  for (let k = 0; k < w.indices.length; k += 3) tris.push([0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [any, any, any]);
}
pts.sort((p, q) => p.x - q.x || p.y - q.y);
const cross = (p: any, q: any, r: any) => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const half = (list: any[]) => { const h: any[] = []; for (const p of list) { while (h.length >= 2 && cross(h[h.length - 2], h[h.length - 1], p) <= 0) h.pop(); h.push(p); } h.pop(); return h; };
let hull = [...half(pts), ...half([...pts].reverse())];
const dist = (p: any, q: any, r: any) => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: any[], t: number): any[] => { if (l.length < 3) return l; let mi = 0, md = 0; for (let i = 1; i < l.length - 1; i++) { const d = dist(l[i], l[0], l[l.length - 1]); if (d > md) { md = d; mi = i; } } return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0], l[l.length - 1]]; };
const h2 = [...hull, hull[0]!];
const half1 = Math.floor(h2.length / 2);
hull = [...rdp(h2.slice(0, half1 + 1), tol).slice(0, -1), ...rdp(h2.slice(half1), tol).slice(0, -1)];
const cellM = Number(process.env.CELL ?? 0.0015);
const loopsOf = (): { x: number; y: number }[][] => {
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
  const x0 = Math.min(...xs) - 2 * cellM, y0 = Math.min(...ys) - 2 * cellM;
  const W = Math.ceil((Math.max(...xs) - x0) / cellM) + 3, H = Math.ceil((Math.max(...ys) - y0) / cellM) + 3;
  const g = new Uint8Array(W * H);
  for (const [a, b, c] of tris) {
    const i0 = Math.max(0, Math.floor((Math.min(a.x, b.x, c.x) - x0) / cellM)), i1 = Math.min(W - 1, Math.ceil((Math.max(a.x, b.x, c.x) - x0) / cellM));
    const j0 = Math.max(0, Math.floor((Math.min(a.y, b.y, c.y) - y0) / cellM)), j1 = Math.min(H - 1, Math.ceil((Math.max(a.y, b.y, c.y) - y0) / cellM));
    const den = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
      const px = x0 + (i + 0.5) * cellM, py = y0 + (j + 0.5) * cellM;
      if (Math.abs(den) < 1e-14) continue;
      const l1 = ((b.y - c.y) * (px - c.x) + (c.x - b.x) * (py - c.y)) / den, l2 = ((c.y - a.y) * (px - c.x) + (a.x - c.x) * (py - c.y)) / den, l3 = 1 - l1 - l2;
      const e = 0;
      if (l1 >= -e && l2 >= -e && l3 >= -e) g[j * W + i] = 1;
    }
  }
  const at = (i: number, j: number) => (i < 0 || j < 0 || i >= W || j >= H ? 0 : g[j * W + i]!);
  const next = new Map<string, string[]>();
  const add = (a: string, b: string) => { const l = next.get(a); if (l) l.push(b); else next.set(a, [b]); };
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (at(i, j)) {
    if (!at(i, j - 1)) add(`${i},${j}`, `${i + 1},${j}`);
    if (!at(i + 1, j)) add(`${i + 1},${j}`, `${i + 1},${j + 1}`);
    if (!at(i, j + 1)) add(`${i + 1},${j + 1}`, `${i},${j + 1}`);
    if (!at(i - 1, j)) add(`${i},${j + 1}`, `${i},${j}`);
  }
  const out: { x: number; y: number }[][] = [];
  while (next.size > 0) {
    const [start] = next.keys();
    let cur = start!; const loop: string[] = [];
    for (;;) { loop.push(cur); const l = next.get(cur)!; const n = l.pop()!; if (l.length === 0) next.delete(cur); cur = n; if (cur === start) break; if (!next.has(cur)) break; }
    const ring = loop.map((s) => { const [i, j] = s.split(",").map(Number); return { x: x0 + i! * cellM, y: y0 + j! * cellM }; });
    let area = 0; for (let i = 0; i < ring.length; i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; area += p.x * q.y - q.x * p.y; }
    if (area > 0 && area / 2 > 25 * cellM * cellM) out.push(ring);
  }
  return out;
};
const rdpRing = (r: any[], t: number): any[] => { let a = 0, b = 0; for (let i = 0; i < r.length; i++) { if (r[i].x < r[a].x) a = i; if (r[i].x > r[b].x) b = i; } if (a === b) return r; const rot = [...r.slice(a), ...r.slice(0, a)]; const k = rot.indexOf(r[b]); const s1 = rot.slice(0, k + 1), s2 = [...rot.slice(k), rot[0]]; return [...rdp(s1, t).slice(0, -1), ...rdp(s2, t).slice(0, -1)]; };
const tmpl: any = geo(asset as any).find((s: any) => s.id === tmplId);
const { kind: _k, points: _p, d: _d, id: _i, closed: _c, solid: _s, x: _x, y: _y, w: _w, h: _h, r: _r, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, cx: _cx, cy: _cy, rx: _rx, ry: _ry, ...keep } = tmpl;
const mk = (ptsIn: { x: number; y: number }[], id: string) => { const sh: any = { kind: "poly", points: solve(ptsIn).map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true, id, ...keep }; delete sh.followTip; return sh; };
const shape = mk(hull, newId);
if (process.env.TRACE !== undefined) {
  const infl = Number(process.env.INFL ?? 0);
  const offset = (r: { x: number; y: number }[]) => infl === 0 ? r : r.map((p, i) => { const a = r[(i + r.length - 1) % r.length]!, b = r[(i + 1) % r.length]!; const n1 = { x: p.y - a.y, y: a.x - p.x }, n2 = { x: b.y - p.y, y: p.x - b.x }; const l1 = Math.hypot(n1.x, n1.y) || 1, l2 = Math.hypot(n2.x, n2.y) || 1; const nx = n1.x / l1 + n2.x / l2, ny = n1.y / l1 + n2.y / l2; const nl = Math.hypot(nx, ny) || 1; return { x: p.x + (nx / nl) * infl, y: p.y + (ny / nl) * infl }; });
  const loops = loopsOf().map((l) => offset(rdpRing(l, tol))).sort((a, b) => b.length - a.length);
  const ops: any[] = [];
  loops.forEach((l, k) => { const sh = mk(l, k === 0 ? newId : `${newId}-${k + 1}`); ops.push(k === 0 && afterId.startsWith("=") ? { op: "shape", scale: sc, view, id: afterId.slice(1), shape: sh } : { op: "shape-add", scale: sc, view, after: k === 0 ? afterId.replace("=", "") : `${newId}${k === 1 ? "" : `-${k}`}`, shape: sh }); });
  console.log(JSON.stringify(ops, null, 1));
  process.exit(0);
}
console.log(JSON.stringify([afterId.startsWith("=") ? { op: "shape", scale: sc, view, id: afterId.slice(1), shape } : { op: "shape-add", scale: sc, view, after: afterId, shape }], null, 1));
