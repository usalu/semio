/** 🤖️ Greedy silhouette repair of one person and scale (after `humansb-D-auto.ts` of the 2026-11-17 guard ticket): the worst mismatch cluster of the guard is repaired by replacing the nearest drawn shape by the traced silhouette of the volume solids of its bone (drawn-only), or by adding that trace (volume-only); a change that worsens the view is reverted, and changes that make `validateAsset` report an issue are taken back. Writes the accumulated ops (all scales of the start file plus this scale) for `apply.ts`.
 *  bun auto.ts <slug|file.json> <scale> <ops-out.json> [maxIter] [startOps.json] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
type P = { x: number; y: number };
const [name, sc, outFile, maxArg, startFile] = process.argv.slice(2) as string[];
const scale = Number(sc), feature = featureOf(scale), limit = 2 * feature, maxIter = Number(maxArg ?? 40);
const FILE = name!.endsWith(".json") ? name! : `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${name}/🔣️.json`;
const { asset } = (await Bun.file(FILE).json()) as { asset: Asset };
const ops: any[] = [];
const geo = (a: any, view: string): any[] => a.representations[sc].geometry[view];
const applyOp = (a: any, o: any): void => {
  if (!String(o.op).startsWith("shape")) { applyOther(a, o); return; }
  const l = a.representations[o.scale].geometry[o.view] as any[];
  if (o.op === "shape-add") l.splice(l.findIndex((x: any) => x.id === o.after) + 1, 0, o.shape);
  else if (o.op === "shape") l[l.findIndex((x: any) => x.id === o.id)] = o.shape;
  else if (o.op === "shape-set") Object.assign(l.find((x: any) => x.id === o.id), o.set);
};
const { applyOther } = await import("./ops.ts");
if (startFile && startFile !== "-" && await Bun.file(startFile).exists()) for (const o of JSON.parse(await Bun.file(startFile).text())) { applyOp(asset, o); ops.push(o); }
const pristine: any = structuredClone(asset);
const issuesOf = (): any[] => validateAsset(structuredClone(asset) as never) as any[];
const baseIssues = issuesOf().length;
var IOU_TARGET = 0.935;
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = asset.presentation.defaultPose;
const boneParent = new Map<string, string | null>((asset as any).rig.bones.map((b: any) => [b.id, b.parent]));
const tmplOf = (view: string, bone: string): any => {
  const l = geo(asset, view).filter((s: any) => s.role === "body" && s.filled !== false && s.kind !== "member");
  return l.find((s: any) => s.bone === bone && s.id === bone) ?? l.filter((s: any) => s.bone === bone).sort((a: any, b: any) => (a.z ?? 0) - (b.z ?? 0))[0] ?? l.sort((a: any, b: any) => (a.z ?? 0) - (b.z ?? 0))[0];
};
const cross = (p: P, q: P, r: P) => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const dist = (p: P, q: P, r: P) => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: P[], t: number): P[] => { if (l.length < 3) return l; let mi = 0, md = 0; for (let i = 1; i < l.length - 1; i++) { const d = dist(l[i]!, l[0]!, l[l.length - 1]!); if (d > md) { md = d; mi = i; } } return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0]!, l[l.length - 1]!]; };
const rdpRing = (r: P[], t: number): P[] => { let a = 0, b = 0; for (let i = 0; i < r.length; i++) { if (r[i]!.x < r[a]!.x) a = i; if (r[i]!.x > r[b]!.x) b = i; } if (a === b) return r; const rot = [...r.slice(a), ...r.slice(0, a)]; const k = rot.indexOf(r[b]!); return [...rdp(rot.slice(0, k + 1), t).slice(0, -1), ...rdp([...rot.slice(k), rot[0]!], t).slice(0, -1)]; };
const INFL = Number(process.env.INFL ?? 0) * feature;
const offsetRing = (r: P[], d: number): P[] => d === 0 ? r : r.map((p, i) => { const a = r[(i + r.length - 1) % r.length]!, b = r[(i + 1) % r.length]!; const n1 = { x: p.y - a.y, y: a.x - p.x }, n2 = { x: b.y - p.y, y: p.x - b.x }; const l1 = Math.hypot(n1.x, n1.y) || 1, l2 = Math.hypot(n2.x, n2.y) || 1; const nx = n1.x / l1 + n2.x / l2, ny = n1.y / l1 + n2.y / l2; const nl = Math.hypot(nx, ny) || 1; return { x: p.x + (nx / nl) * d, y: p.y + (ny / nl) * d }; });
const cellM = 0.0012 * Math.max(1, scale / 100);
const traceLoops = (view: string, solidIds: Set<string>, state: any): P[][] => {
  const tris: [P, P, P][] = [], pts: P[] = [];
  for (const w of worldSolids(asset, state.bones as never, scale)) if (solidIds.has(w.id) && w.slender !== true) {
    for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!));
    for (let k = 0; k < w.indices.length; k += 3) tris.push([0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [P, P, P]);
  }
  if (pts.length === 0) return [];
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
  const x0 = Math.min(...xs) - 2 * cellM, y0 = Math.min(...ys) - 2 * cellM;
  const W = Math.ceil((Math.max(...xs) - x0) / cellM) + 3, H = Math.ceil((Math.max(...ys) - y0) / cellM) + 3;
  const g = new Uint8Array(W * H);
  for (const [a, b, c] of tris) {
    const i0 = Math.max(0, Math.floor((Math.min(a.x, b.x, c.x) - x0) / cellM)), i1 = Math.min(W - 1, Math.ceil((Math.max(a.x, b.x, c.x) - x0) / cellM));
    const j0 = Math.max(0, Math.floor((Math.min(a.y, b.y, c.y) - y0) / cellM)), j1 = Math.min(H - 1, Math.ceil((Math.max(a.y, b.y, c.y) - y0) / cellM));
    const den = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    if (Math.abs(den) < 1e-14) continue;
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
      const px = x0 + (i + 0.5) * cellM, py = y0 + (j + 0.5) * cellM;
      const l1 = ((b.y - c.y) * (px - c.x) + (c.x - b.x) * (py - c.y)) / den, l2 = ((c.y - a.y) * (px - c.x) + (a.x - c.x) * (py - c.y)) / den;
      if (l1 >= 0 && l2 >= 0 && 1 - l1 - l2 >= 0) g[j * W + i] = 1;
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
  const out: P[][] = [];
  while (next.size > 0) {
    const [start] = next.keys();
    let cur = start!; const loop: string[] = [];
    for (;;) { loop.push(cur); const l = next.get(cur)!; const n = l.pop()!; if (l.length === 0) next.delete(cur); cur = n; if (cur === start) break; if (!next.has(cur)) break; }
    const ring = loop.map((s) => { const [i, j] = s.split(",").map(Number); return { x: x0 + i! * cellM, y: y0 + j! * cellM }; });
    let area = 0; for (let i = 0; i < ring.length; i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; area += p.x * q.y - q.x * p.y; }
    if (Math.abs(area) / 2 > (area > 0 ? 25 : 60) * cellM * cellM) out.push(offsetRing(rdpRing(ring, Math.max(0.0007, feature / Number(process.env.RDPDIV ?? 6))), area > 0 ? INFL : -INFL));
  }
  return out.sort((a, b) => b.length - a.length);
};
const placeLocal = (view: string, bone: string, id: string, targets: P[]): P[] | null => {
  const forward = (points: P[]): P[] => {
    const probe: any = structuredClone(asset);
    geo(probe, view).push({ kind: "poly", points: points.map((p) => [p.x, p.y]), closed: true, id, bone, role: "body", z: 1, filled: true, tone: 0.5 });
    const ps: any = evaluate(probe, { kind: "pose", pose }, sel);
    return ps.shapes[view].find((s: any) => s.shape.id === id && s.shape.tone === 0.5 && s.shape.z === 1).rings[0].slice(0, points.length);
  };
  const [o, a, b] = forward([{ x: 0, y: 0 }, { x: 0.05, y: 0 }, { x: 0, y: 0.05 }]);
  const m = { a: (a!.x - o!.x) / 0.05, b: (a!.y - o!.y) / 0.05, c: (b!.x - o!.x) / 0.05, d: (b!.y - o!.y) / 0.05 };
  const det = m.a * m.d - m.b * m.c;
  if (Math.abs(det) < 0.15) return null;
  const inv = (p: P) => { const dx = p.x - o!.x, dy = p.y - o!.y; return { x: (m.d * dx - m.c * dy) / det, y: (-m.b * dx + m.a * dy) / det }; };
  let local = targets.map(inv);
  for (let it = 0; it < 6; it++) {
    const got = forward(local);
    const err = targets.map((t, i) => ({ x: t.x - got[i]!.x, y: t.y - got[i]!.y }));
    const worst = Math.max(...err.map((e) => Math.hypot(e.x, e.y)));
    if (worst < Math.max(2e-4, feature / 30)) return local;
    if (it === 5) return null;
    local = local.map((l, i) => { const d = inv({ x: o!.x + err[i]!.x, y: o!.y + err[i]!.y }); return { x: l.x + d.x, y: l.y + d.y }; });
  }
  return local;
};
const mkShapeMulti = (tmpl: any, id: string, bone: string, locals: P[][]): any => {
  if (locals.length === 1) return mkShape(tmpl, id, bone, locals[0]!);
  const base = mkShape(tmpl, id, bone, locals[0]!);
  const { points: _pts, closed: _cl, kind: _k, ...rest } = base;
  return { kind: "path", d: locals.map((l) => "M" + l.map((p) => `${p.x.toFixed(5)} ${p.y.toFixed(5)}`).join(" L") + " Z").join(" "), ...rest };
};
const mkShape = (tmpl: any, id: string, bone: string, local: P[]): any => {
  const { kind: _k, points: _p, d: _d, id: _i, closed: _c, solid: _s, x: _x, y: _y, w: _w, h: _h, r: _r, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, cx: _cx, cy: _cy, rx: _rx, ry: _ry, rotationDeg: _rd, followTip: _f, bone: _b, ...keep } = tmpl;
  return { kind: "poly", points: local.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true, id, bone, ...keep };
};
const bonesOf = (state: any): Map<string, string[]> => { const m = new Map<string, string[]>(); for (const s of (asset as any).volumes[sc].body) if (s.texture !== true && s.slender !== true) { const l = m.get(s.bone) ?? []; l.push(s.id); m.set(s.bone, l); } return m; };
const tryTrace = (view: string, bone: string, solidIds: string[], state: any): { loops: P[][]; bone: string; local: P[][] } | null => {
  const loops = traceLoops(view, new Set(solidIds), state);
  if (loops.length === 0) return null;
  let b: string | null = bone;
  while (b) {
    const local: P[][] = [];
    let ok = true;
    for (const [k, loop] of loops.entries()) { const l = placeLocal(view, b, `t-${k}`, loop); if (l === null) { ok = false; break; } local.push(l); }
    if (ok) return { loops, bone: b, local };
    b = boneParent.get(b) ?? null;
  }
  return null;
};
const dseg = (p: P, a: P, b: P) => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((p.x - a.x) * dx + (p.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy); };
type Cluster = { view: string; side: string; x: number; y: number; d: number };
const measure = (only?: string) => {
  const state: any = evaluate(asset, { kind: "pose", pose }, sel);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const res: { view: string; iou: number; h: number; clusters: Cluster[]; area: Cluster[] }[] = [];
  for (const view of VIEWPOINTS) {
    if (only !== undefined && only !== view) continue;
    const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
    const cell = m.cell, box = m.box;
    const drawn = new Mask(box, cell);
    for (const g of m.filled) drawn.fill(g);
    for (const g of m.outlines) drawn.fill(g);
    const vol = new Mask(box, cell);
    for (const g of m.reference) vol.fill(g, true);
    if (m.section) for (const g of m.visible) vol.fill(g, true);
    vol.seal();
    const [w, h] = [vol.width, vol.height];
    const holes = Math.round((4 * feature / cell) ** 2);
    const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
    const toVol = distanceField(bv, w, h), toDrawn = distanceField(bd, w, h);
    const clusters: Cluster[] = [];
    for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
      const list: { i: number; j: number; d: number }[] = [];
      for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
      list.sort((a, b) => b.d - a.d);
      const seen = new Set<number>();
      for (const c of list) { const key = Math.round(c.i / (0.08 / cell)) * 100000 + Math.round(c.j / (0.08 / cell)); if (seen.has(key)) continue; seen.add(key); clusters.push({ view, side, x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell, d: c.d }); if (clusters.length > 60) break; }
    }
    const area: Cluster[] = [];
    if (m.iou < IOU_TARGET) {
      const bin = Math.max(3, Math.round(0.05 / cell)), bw = Math.ceil(w / bin), bh = Math.ceil(h / bin);
      const dOnly = new Float64Array(bw * bh), vOnly = new Float64Array(bw * bh);
      for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) { const k = (j / bin | 0) * bw + (i / bin | 0); const a = drawn.cells[j * w + i], b = vol.cells[j * w + i]; if (a && !b) dOnly[k]!++; else if (b && !a) vOnly[k]!++; }
      for (let k = 0; k < bw * bh; k++) { const by = (k / bw) | 0, bx = k % bw; const side = dOnly[k]! >= vOnly[k]! ? "drawn-only" : "volume-only"; const n = Math.max(dOnly[k]!, vOnly[k]!); if (n > 0.15 * bin * bin) area.push({ view, side, x: box.minX + (bx + 0.5) * bin * cell, y: box.minY + (by + 0.5) * bin * cell, d: n * cell * cell }); }
      area.sort((a, b) => b.d - a.d);
    }
    res.push({ view, iou: m.iou, h: m.hausdorffM, clusters: clusters.sort((a, b) => b.d - a.d), area: area.slice(0, 40) });
  }
  return { state, solids, res };
};
const T0 = performance.now();
const MAXSEC = Number(process.env.MAXSEC ?? 300);
const timeLeft = (): boolean => (performance.now() - T0) / 1000 < MAXSEC;
const banned = new Set<string>();
{
  const probe: any = structuredClone(asset);
  const boneIds = new Set<string>((asset as any).volumes[sc].body.map((x: any) => x.bone));
  for (const view of VIEWPOINTS) for (const bone of boneIds) {
    const tmpl = tmplOf(view, bone);
    if (tmpl === undefined) continue;
    geo(probe, view).push(mkShape(tmpl, `probe-${bone}`, bone, [{ x: 0, y: -0.05 }, { x: 0.12, y: -0.05 }, { x: 0.12, y: 0.05 }, { x: 0, y: 0.05 }]));
  }
  for (const i of validateAsset(probe as never) as any[]) {
    if (i.code === "lateral-ambiguous" && i.data?.bone !== undefined && (i.data.shapes as string[]).some((x) => x.startsWith("probe-"))) banned.add(i.data.bone);
    if (i.code === "elevation-lateral") { const m = /Shape "probe-([^"]+)"/.exec(i.message.en); if (m) banned.add(m[1]!); }
  }
}
const slenderBones = new Set<string>((asset as any).volumes[sc].body.filter((s: any) => s.slender === true).map((s: any) => s.bone));
const done = new Set<string>();
const log = (s: string) => console.error(`[DEBUG] ${s}`);
const pass = (): void => {
for (let iter = 0; iter < maxIter; iter++) {
  const { state, solids, res } = measure();
  const summary = res.map((r) => `${r.view[0]}:${r.iou.toFixed(3)}/${(r.h * 1000).toFixed(0)}mm`).join(" ");
  if (iter === 0) log(`flat shapes are not placed on: ${[...banned].join(" ")}`);
  if (!timeLeft()) { log("time budget used"); break; }
  log(`iter ${iter} 1:${sc} ${summary} ops=${ops.length}`);
  let all = res.flatMap((r) => r.clusters).sort((a, b) => b.d - a.d);
  if (all.length === 0) all = res.flatMap((r) => r.area).sort((a, b) => b.d - a.d);
  if (all.length === 0) break;
  const bonesSolids = bonesOf(state);
  let acted = false;
  for (const c of all) {
    const near = (state.shapes[c.view] as any[]).map((s) => { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(c, ring[i], ring[(i + 1) % ring.length])); return { s, m }; }).filter((x) => x.m < Math.max(0.012, feature)).sort((a, b) => a.m - b.m || (b.s.shape.z ?? 0) - (a.s.shape.z ?? 0));
    let act: (() => void) | null = null;
    if (c.side === "drawn-only") {
      const cand = near.find((x) => x.s.closed && x.s.shape.filled !== false && x.s.shape.role !== "ground" && bonesSolids.has(x.s.shape.bone) && !slenderBones.has(x.s.shape.bone) && !done.has(`${c.view}|r|${x.s.shape.id}`));
      if (!cand) continue;
      const id = cand.s.shape.id, bone = cand.s.shape.bone;
      const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
      done.add(`${c.view}|r|${id}`);
      if (tr && banned.has(tr.bone)) { continue; }
      if (!tr) { log(`cannot trace ${c.view} ${bone} for ${id}`); continue; }
      act = () => {
        const base = geo(asset, c.view).find((s: any) => s.id === id);
        const sh = mkShapeMulti(base, id, tr.bone, tr.local);
        const o = { op: "shape", scale: sc, view: c.view, id, shape: sh };
        applyOp(asset, o); ops.push(o);
        log(`replace ${c.view} ${id} (${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}) by trace of ${bone}`);
      };
    } else if (!done.has(`${c.view}|i|${Math.round(c.x / 0.03)}|${Math.round(c.y / 0.03)}`) && (state.shapes[c.view] as any[]).some((s) => s.closed && s.shape.filled !== false && s.shape.role !== "ground" && bonesSolids.has(s.shape.bone) && !slenderBones.has(s.shape.bone) && !done.has(`${c.view}|r|${s.shape.id}`) && s.rings.some((ring: P[]) => { let inside = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > c.y) !== (ring[j]!.y > c.y) && c.x < ((ring[j]!.x - ring[i]!.x) * (c.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) inside = !inside; return inside; }))) {
      done.add(`${c.view}|i|${Math.round(c.x / 0.03)}|${Math.round(c.y / 0.03)}`);
      const cover = (state.shapes[c.view] as any[]).filter((s) => s.closed && s.shape.filled !== false && s.shape.role !== "ground" && bonesSolids.has(s.shape.bone) && !slenderBones.has(s.shape.bone) && !done.has(`${c.view}|r|${s.shape.id}`) && s.rings.some((ring: P[]) => { let inside = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > c.y) !== (ring[j]!.y > c.y) && c.x < ((ring[j]!.x - ring[i]!.x) * (c.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) inside = !inside; return inside; }))[0]!;
      const id = cover.shape.id, bone = cover.shape.bone;
      const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
      done.add(`${c.view}|r|${id}`);
      if (tr && banned.has(tr.bone)) { continue; }
      if (!tr) { log(`cannot trace ${c.view} ${bone} for ${id}`); continue; }
      act = () => {
        const base = geo(asset, c.view).find((s: any) => s.id === id);
        const o = { op: "shape", scale: sc, view: c.view, id, shape: mkShapeMulti(base, id, tr.bone, tr.local) };
        applyOp(asset, o); ops.push(o);
        log(`replace covering ${c.view} ${id} (${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}) by trace of ${bone}`);
      };
    } else {
      const sol = solids.filter((w) => w.slender !== true).map((w) => { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(c.view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [P, P, P]; m = Math.min(m, distanceToTriangle(c, t)); } return { w, m }; }).filter((x) => x.m < 0.006 && x.w.bone !== undefined).sort((a, b) => a.m - b.m);
      const cand = sol.find((x) => !done.has(`${c.view}|a|${x.w.bone}`));
      if (!cand) {
        const key = `${c.view}|u|${Math.round(c.x / 0.04)}|${Math.round(c.y / 0.04)}`;
        const wide = solids.filter((w) => w.slender !== true).filter((w) => { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(c.view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [P, P, P]; m = Math.min(m, distanceToTriangle(c, t)); } return m < 0.04; });
        if (done.has(key) || wide.length === 0 || sol.length === 0) continue;
        done.add(key);
        const bone = sol[0]!.w.bone!;
        if (banned.has(bone)) continue;
        const loops = traceLoops(c.view, new Set(wide.map((w) => w.id)), state);
        const local = loops.map((l, k) => placeLocal(c.view, bone, `u-${k}`, l));
        if (local.some((l) => l === null) || local.length === 0) { log(`cannot union-trace ${c.view} near ${c.x.toFixed(2)},${c.y.toFixed(2)}`); continue; }
        act = () => {
          const tmpl = tmplOf(c.view, bone);
          const base = geo(asset, c.view).find((s: any) => s.bone === bone && s.role === "body") ?? tmpl;
          let after = base.id;
          const id = `${bone}-u${ops.length}`; const o = { op: "shape-add", scale: sc, view: c.view, after, shape: mkShapeMulti(tmpl, id, bone, local as P[][]) }; applyOp(asset, o); ops.push(o); after = id;
          log(`add union ${c.view} near ${c.x.toFixed(3)},${c.y.toFixed(3)} of ${wide.map((w) => w.id).join("+")}`);
        };
      } else {
        const bone = cand.w.bone!;
        done.add(`${c.view}|a|${bone}`);
        const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
        if (tr && banned.has(tr.bone)) { continue; }
      if (!tr) { log(`cannot trace ${c.view} ${bone}`); continue; }
        act = () => {
          const tmpl = tmplOf(c.view, bone) ?? tmplOf(c.view, tr.bone);
          const base = geo(asset, c.view).find((s: any) => s.bone === tr.bone && s.role === "body") ?? tmpl;
          let after = base.id;
          const id = `${bone}-sil`;
          const o = { op: "shape-add", scale: sc, view: c.view, after, shape: mkShapeMulti(tmpl, id, tr.bone, tr.local) };
          applyOp(asset, o); ops.push(o); after = id;
          log(`add ${c.view} ${bone}-sil (${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)})`);
        };
      }
    }
    const vScore = (rs: typeof res, v: string): number => { const r = rs.find((x) => x.view === v)!; return Math.max(r.h / limit, r.iou < 0.925 ? 1 + (0.925 - r.iou) * 20 : 0); };
    const before = vScore(res, c.view), snapshot = structuredClone(geo(asset, c.view)), opsLen = ops.length;
    act();
    const after = vScore(measure().res, c.view);
    if (after > before + 0.02) { (asset as any).representations[sc].geometry[c.view] = snapshot; ops.length = opsLen; log(`revert (${before.toFixed(3)} -> ${after.toFixed(3)})`); }
    acted = true;
    break;
  }
  if (!acted) { log("no more actionable clusters"); break; }
}

const POSITION = ["x", "y", "cx", "cy", "x0", "y0", "x1", "y1"], SIZE = ["w", "h", "r", "r0", "r1", "rx", "ry"];
const viewScore = (rs: ReturnType<typeof measure>["res"], v: string): number => { const r = rs.find((x) => x.view === v)!; return Math.max(r.h / limit, r.iou < 0.925 ? 1 + (0.925 - r.iou) * 20 : 0); };
/** Fits the numbers of the drawn shape next to a mismatch cluster (position, radii, width, offset of a polygon) by the distance of the cluster; keeps a change that improves its view and worsens no other. */
const paramFit = (): void => {
  const tried = new Set<string>();
  for (let round = 0; round < Number(process.env.PARAM ?? 60) && timeLeft(); round++) {
    const cur = measure();
    const c = cur.res.flatMap((r) => r.clusters).sort((a, b) => b.d - a.d).find((x) => !tried.has(`${x.view}|${x.side}|${Math.round(x.x / 0.02)}|${Math.round(x.y / 0.02)}`));
    if (c === undefined) break;
    tried.add(`${c.view}|${c.side}|${Math.round(c.x / 0.02)}|${Math.round(c.y / 0.02)}`);
    const state: any = cur.state;
    const near = (state.shapes[c.view] as any[]).map((s) => { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(c, ring[i], ring[(i + 1) % ring.length])); return { id: s.shape.id as string, m, role: s.shape.role }; }).filter((x) => x.m < Math.max(0.02, 1.5 * feature, c.side === "volume-only" ? c.d * 1.2 : 0) && x.role !== "ground").sort((a, b) => a.m - b.m).slice(0, 3);
    const before = viewScore(cur.res, c.view);
    const sign = c.side === "drawn-only" ? -1 : 1;
    let best: { id: string; shape: any; s: number; what: string } | null = null;
    for (const cand of near) {
      const sh = geo(asset, c.view).find((x: any) => x.id === cand.id);
      if (sh === undefined || sh.kind === "member") continue;
      const saved = structuredClone(sh);
      const trial = (what: string, change: () => boolean): void => {
        Object.keys(sh).forEach((k) => delete sh[k]); Object.assign(sh, structuredClone(saved));
        if (!change()) return;
        let s2 = Infinity;
        try { s2 = viewScore(measure(c.view).res, c.view); } catch { s2 = Infinity; }
        if (s2 < (best?.s ?? before - 0.03)) best = { id: cand.id, shape: structuredClone(sh), s: s2, what };
      };
      for (const amount of [c.d, c.d / 2]) {
        for (const k of SIZE) if (typeof saved[k] === "number") trial(`${k} ${(sign * amount * 1000).toFixed(1)}mm`, () => { const v = saved[k] + sign * amount * (k === "w" || k === "h" ? 2 : 1); if (v <= 1e-4) return false; sh[k] = +v.toFixed(5); return true; });
        for (const k of POSITION) if (typeof saved[k] === "number") for (const dir of [1, -1]) trial(`${k} ${(dir * amount * 1000).toFixed(1)}mm`, () => { sh[k] = +(saved[k] + dir * amount).toFixed(5); return true; });
        if (saved.kind === "poly" && Array.isArray(saved.points)) {
          const pts: P[] = saved.points.map(([x, y]: number[]) => ({ x: x!, y: y! }));
          let area = 0; for (let i = 0; i < pts.length; i++) { const p = pts[i]!, q = pts[(i + 1) % pts.length]!; area += p.x * q.y - q.x * p.y; }
          trial(`offset ${(sign * amount * 1000).toFixed(1)}mm`, () => { sh.points = offsetRing(pts, sign * amount * (area >= 0 ? 1 : -1)).map((p) => [+p.x.toFixed(5), +p.y.toFixed(5)]); return true; });
          for (const [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) trial(`shift ${(dx * amount * 1000).toFixed(1)},${(dy * amount * 1000).toFixed(1)}mm`, () => { sh.points = pts.map((p) => [+(p.x + dx * amount).toFixed(5), +(p.y + dy * amount).toFixed(5)]); return true; });
        }
      }
      Object.keys(sh).forEach((k) => delete sh[k]); Object.assign(sh, saved);
    }
    if (best === null) continue;
    const chosen: { id: string; shape: any; s: number; what: string } = best;
    const l = geo(asset, c.view), i = l.findIndex((x: any) => x.id === chosen.id), was = l[i];
    l[i] = chosen.shape;
    const after = measure().res;
    if (VIEWPOINTS.some((v) => viewScore(after, v) > Math.max(1, viewScore(cur.res, v)) + 0.02 && viewScore(after, v) > viewScore(cur.res, v) + 0.02)) { l[i] = was; log(`fit ${c.view} ${chosen.id} ${chosen.what} would worsen another view`); continue; }
    ops.push({ op: "shape", scale: sc, view: c.view, id: chosen.id, shape: structuredClone(chosen.shape) });
    tried.delete(`${c.view}|${c.side}|${Math.round(c.x / 0.02)}|${Math.round(c.y / 0.02)}`);
    log(`fit ${c.view} ${chosen.id} ${chosen.what} (${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}): ${before.toFixed(3)} -> ${chosen.s.toFixed(3)}`);
  }
};
if (process.env.PARAM !== "0") paramFit();
const score = (res: ReturnType<typeof measure>["res"]): number => Math.max(...res.map((r) => Math.max(r.h / limit, r.iou < 0.925 ? 1 + (0.925 - r.iou) * 20 : 0)));
if (process.env.POLISH !== "0") {
  for (let round = 0; round < Number(process.env.POLISH ?? 25) && timeLeft(); round++) {
    const cur = measure();
    const base = score(cur.res);
    if (base <= 0.96) break;
    const worst = cur.res.flatMap((r) => r.clusters).sort((a, b) => b.d - a.d)[0];
    if (!worst) break;
    const state: any = cur.state;
    const isTool = (sh: any): boolean => sh.kind === "poly" || (sh.kind === "path" && /^M[-\d. L]+Z( M[-\d. L]+Z)*$/.test(sh.d));
    const getLoops = (sh: any): number[][][] => sh.kind === "poly" ? [sh.points] : sh.d.split("Z").filter((x: string) => x.trim().length > 0).map((seg: string) => seg.trim().replace(/^M/, "").split("L").map((q: string) => q.trim().split(/\s+/).map(Number)));
    const setLoops = (sh: any, loops: P[][]): void => { if (sh.kind === "poly") sh.points = loops[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]); else sh.d = loops.map((l) => "M" + l.map((p) => `${p.x.toFixed(5)} ${p.y.toFixed(5)}`).join(" L") + " Z").join(" "); };
    const orient = (l: P[]): number => { let a = 0; for (let i = 0; i < l.length; i++) { const p = l[i]!, q = l[(i + 1) % l.length]!; a += p.x * q.y - q.x * p.y; } return a >= 0 ? 1 : -1; };
    const xform = (loops: number[][][], dx: number, dy: number, d: number): P[][] => loops.map((l) => { const pts = l.map(([x, y]) => ({ x: x! + dx, y: y! + dy })); return offsetRing(pts, d * orient(pts)); });
    const cands = (state.shapes[worst.view] as any[]).map((s) => { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(worst, ring[i], ring[(i + 1) % ring.length])); return { s, m }; }).filter((x) => x.m < Math.max(0.03, 2 * feature) && x.s.closed && isTool(geo(asset, worst.view).find((g: any) => g.id === x.s.shape.id) ?? {})).map((x) => x.s.shape.id as string);
    let best: { id: string; d: number; sc: number; dx?: number; dy?: number } | null = null;
    for (const id of [...new Set(cands)]) {
      const sh = geo(asset, worst.view).find((s: any) => s.id === id);
      if (!sh) continue;
      const orig = getLoops(sh);
      const tries: { mul: number; dx: number; dy: number }[] = [0.15, -0.15, 0.3, -0.3, 0.6, -0.6].map((mul) => ({ mul, dx: 0, dy: 0 }));
      for (const m of [0.1, 0.2, 0.3]) for (const [a, b] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) tries.push({ mul: 0, dx: a * m * feature, dy: b * m * feature });
      const saved = structuredClone(sh);
      for (const tr of tries) {
        setLoops(sh, xform(orig, tr.dx, tr.dy, tr.mul * feature));
        const sc2 = score(measure().res);
        if (best === null ? sc2 < base - 0.02 : sc2 < best.sc) best = { id, d: tr.mul * feature, sc: sc2, dx: tr.dx, dy: tr.dy };
      }
      Object.assign(sh, saved);
    }
    if (best === null) { log(`polish: no improving change for cluster ${worst.view} ${worst.x.toFixed(3)},${worst.y.toFixed(3)} d=${worst.d.toFixed(3)}`); break; }
    const sh = geo(asset, worst.view).find((s: any) => s.id === best!.id);
    setLoops(sh, xform(getLoops(sh), best.dx ?? 0, best.dy ?? 0, best.d));
    const o = { op: "shape", scale: sc, view: worst.view, id: best.id, shape: structuredClone(sh) };
    ops.push(o);
    log(`polish ${worst.view} ${best.id} by ${(best.d * 1000).toFixed(2)}mm shift ${((best.dx ?? 0) * 1000).toFixed(1)},${((best.dy ?? 0) * 1000).toFixed(1)}: score ${base.toFixed(3)} -> ${best.sc.toFixed(3)}`);
  }
}
};
const revertShape = (view: string, id: string): void => {
  const l = geo(asset, view), p = geo(pristine, view) as any[];
  const i = l.findIndex((x: any) => x.id === id), was = p.find((x: any) => x.id === id);
  if (was !== undefined && i >= 0) l[i] = structuredClone(was); else if (i >= 0) l.splice(i, 1);
  for (let k = ops.length - 1; k >= 0; k--) if (ops[k].scale === sc && ops[k].view === view && (ops[k].id ?? ops[k].shape?.id) === id) ops.splice(k, 1);
};
for (let round = 0; round < 4; round++) {
  pass();
  let issues = issuesOf();
  if (issues.length <= baseIssues) break;
  log(`validateAsset: ${issues.length} issues (base ${baseIssues}): ${[...new Set(issues.map((i) => i.code))].join(", ")}`);
  const mine = (): { view: string; id: string }[] => { const seen = new Set<string>(); const out: { view: string; id: string }[] = []; for (let k = ops.length - 1; k >= 0; k--) { const o = ops[k]; if (o.scale !== sc || !String(o.op).startsWith("shape")) continue; const id = o.id ?? o.shape?.id; if (!seen.has(`${o.view}|${id}`)) { seen.add(`${o.view}|${id}`); out.push({ view: o.view, id }); } } return out; };
  const revertMany = (pairs: { view: string; id: string }[], why: string): void => {
    const live = pairs.filter((m) => ops.some((o) => o.scale === sc && o.view === m.view && (o.id ?? o.shape?.id) === m.id));
    if (live.length === 0) return;
    const snapshot = Object.fromEntries(VIEWPOINTS.map((v) => [v, structuredClone(geo(asset, v))])), kept = ops.slice();
    for (const m of live) revertShape(m.view, m.id);
    const after = issuesOf();
    if (after.length < issues.length) { issues = after; for (const m of live) done.add(`${m.view}|r|${m.id}`); log(`taken back ${live.map((m) => `${m.view} ${m.id}`).join(", ")} (${why}): ${after.length} issues left`); }
    else { for (const v of VIEWPOINTS) (asset as any).representations[sc].geometry[v] = snapshot[v]; ops.length = 0; ops.push(...kept); }
  };
  for (const issue of issues.slice()) {
    if (issues.length <= baseIssues) break;
    const named = new Set<string>([...(issue.data?.shapes ?? []), ...[...String(issue.message?.en ?? "").matchAll(/"([^"]+)"/g)].map((m) => m[1]!)]);
    revertMany(mine().filter((m) => named.has(m.id)), issue.code);
  }
  for (const m of mine()) { if (issues.length <= baseIssues) break; revertMany([m], "single"); }
  if (issues.length > baseIssues) { const all = mine(); for (const m of all) revertShape(m.view, m.id); issues = issuesOf(); log(`taken back every change of 1:${sc}: ${issues.length} issues left`); break; }
  if (issues.length > baseIssues) { log(`validateAsset still reports ${issues.length} issues: ${issues.slice(0, 5).map((i) => `${i.code} ${i.path ?? ""} ${i.message?.en ?? ""}`.slice(0, 300)).join(" || ")}`); break; }
}
const fin = measure();
log(`issues ${issuesOf().length} (base ${baseIssues})`);
log("final " + fin.res.map((r) => `${r.view[0]}:${r.iou.toFixed(3)}/${(r.h * 1000).toFixed(1)}mm(${r.clusters.length},${r.area.length})`).join(" "));
await Bun.write(outFile, JSON.stringify(ops));
