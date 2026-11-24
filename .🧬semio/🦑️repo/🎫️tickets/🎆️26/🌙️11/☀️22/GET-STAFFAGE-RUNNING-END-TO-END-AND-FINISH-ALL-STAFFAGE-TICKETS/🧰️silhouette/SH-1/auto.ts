/** 🤖️ Greedy silhouette repair of one person and scale (after `humansb-D-auto.ts` of the 2026-11-17 guard ticket): the worst mismatch cluster of the guard is repaired by replacing the nearest drawn shape by the traced silhouette of the volume solids of its bone (drawn-only), or by adding that trace (volume-only); a change that worsens the view is reverted, and changes that make `validateAsset` report an issue are taken back. Writes the accumulated ops (all scales of the start file plus this scale) for `apply.ts`.
 *  bun auto.ts <slug|file.json> <scale> <ops-out.json> [maxIter] [startOps.json]   (the start ops may already be in the file: they are idempotent; MAXSEC time budget, LIMIT a tighter largest distance in metres than two smallest features) */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { restFrames } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { worldFrameOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
type P = { x: number; y: number };
const [name, sc, outFile, maxArg, startFile] = process.argv.slice(2) as string[];
const scale = Number(sc), feature = featureOf(scale), limit = Number(process.env.LIMIT ?? 2 * feature), maxIter = Number(maxArg ?? 40);
const FILE = name!.endsWith(".json") ? name! : `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${name}/🔣️.json`;
const { asset } = (await Bun.file(FILE).json()) as { asset: Asset };
const ops: any[] = [];
const geo = (a: any, view: string): any[] => a.representations[sc].geometry[view];
const applyOp = (a: any, o: any): void => {
  if (!String(o.op).startsWith("shape") || o.op === "shape-remove") { try { applyOther(a, o); } catch (e) { if (o.op !== "shape-remove") throw e; } return; }
  const l = a.representations[o.scale].geometry[o.view] as any[];
  if (o.op === "shape-add") l.splice(l.findIndex((x: any) => x.id === o.after) + 1, 0, o.shape);
  else if (o.op === "shape") l[l.findIndex((x: any) => x.id === o.id)] = o.shape;
  else if (o.op === "shape-set") Object.assign(l.find((x: any) => x.id === o.id), o.set);
};
const { applyOther, applyOps, writeOps } = await import("./ops.ts");
const untouched: any = structuredClone(asset);
if (startFile && startFile !== "-" && await Bun.file(startFile).exists()) { const start = JSON.parse(await Bun.file(startFile).text()); applyOps(asset, start); ops.push(...start); }
const pristine: any = process.env.WHOLE === undefined ? structuredClone(asset) : untouched;
const started: any = structuredClone(asset), startOps = ops.slice();
const keyOf = (i: any): string => JSON.stringify([i.code, i.path, i.data?.bone, [...(i.data?.shapes ?? [])].sort(), i.data?.bands]);
/** The print suitability of the working scale under the default profile as issues, so that a repair never leaves a drawing the check CLI refuses. */
const printIssues = (a: any): any[] => { try { return measureSuitability(a, a.representations[sc], scale, DEFAULT_PRINT_PROFILE).violations.map((v: any) => ({ code: `suitability-${v.check}`, path: `representations.${sc}`, data: { shapes: [...v.shapes] }, message: { en: v.shapes.map((x: string) => `"${x}"`).join(" ") } })); } catch (e) { return [{ code: "suitability-error", path: `representations.${sc}`, message: { en: String(e).slice(0, 200) } }]; } };
const allIssues = (): any[] => { const a: any = structuredClone(asset); return [...(validateAsset(a as never) as any[]), ...printIssues(a)]; };
/** The file of the session start with every start op that is not a drawn shape (declared dimensions, volume solids): what it reports is inherited, only what the drawn repair adds is taken back. */
const baseline = (): any => { const a: any = structuredClone(untouched); applyOps(a, ops.filter((o) => !String(o.op).startsWith("shape"))); return a; };
const baseKeys = new Set<string>((process.env.WHOLE === undefined ? allIssues() : [...(validateAsset(baseline() as never) as any[]), ...printIssues(baseline())]).map(keyOf));
/** The issues `validateAsset` reports now that it did not report before this run (an issue the run inherits is none of its own). */
const issuesOf = (): any[] => allIssues().filter((i) => !baseKeys.has(keyOf(i)) && !(/^representations\.(\d+)/u.exec(String(i.path ?? ""))?.[1] !== undefined && /^representations\.(\d+)/u.exec(String(i.path ?? ""))![1] !== sc));
const baseIssues = 0;
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
const traceLoops = (view: string, solidIds: Set<string>, state: any, tolerance = Math.max(0.0007, feature / Number(process.env.RDPDIV ?? 6)), cellM = 0.0012 * Math.max(1, scale / 100), erode?: P, inset = INFL): P[][] => {
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
  if (erode !== undefined && Math.hypot(erode.x, erode.y) > cellM / 2) {
    const n = Math.ceil(Math.hypot(erode.x, erode.y) / cellM), raw = g.slice();
    for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (raw[j * W + i]) for (let k = -n; k <= n; k++) { const ii = Math.round(i + (k / n) * (erode.x / cellM)), jj = Math.round(j + (k / n) * (erode.y / cellM)); if (ii < 0 || jj < 0 || ii >= W || jj >= H || !raw[jj * W + ii]) { g[j * W + i] = 0; break; } }
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
    if (Math.abs(area) / 2 > (area > 0 ? 25 : 60) * cellM * cellM) out.push(offsetRing(rdpRing(ring, tolerance), area > 0 ? inset : -inset));
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
  const own = Object.fromEntries(Object.entries(keep).filter(([k]) => ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"].includes(k)));
  return { kind: "poly", points: local.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true, id, bone, ...own };
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
    b = process.env.CLIMB === undefined ? null : (boneParent.get(b) ?? null);
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
type Res = ReturnType<typeof measure>["res"];
const metricOf = (rs: Res, v: string): { max: number; sum: number; iou: number } => { const r = rs.find((x) => x.view === v)!; const pen = r.iou < 0.925 ? 1 + (0.925 - r.iou) * 20 : 0; return { max: Math.max(r.h / limit, pen), sum: r.clusters.reduce((t, c) => t + Math.max(0, c.d / limit - 1), 0) + pen, iou: r.iou }; };
/** A change is kept when the worst distance of its view improves, or when the worst stays and the sum of the excesses of all mismatch clusters shrinks or the areas agree better. */
const improves = (after: Res, before: Res, v: string): boolean => { const [a, b] = [metricOf(after, v), metricOf(before, v)]; return a.max < b.max - 0.02 || (a.max <= b.max + 0.005 && (a.sum < b.sum * 0.95 - 1e-9 || a.iou > b.iou + 0.0005)); };
const T0 = performance.now();
const MAXSEC = Number(process.env.MAXSEC ?? 300);
const timeLeft = (): boolean => (performance.now() - T0) / 1000 < MAXSEC;
const bannedIn: Record<string, Set<string>> = { elevation: new Set(), front: new Set(), plan: new Set() };
{
  const probe: any = structuredClone(asset);
  const boneIds = new Set<string>((asset as any).volumes[sc].body.map((x: any) => x.bone));
  for (const view of VIEWPOINTS) for (const bone of boneIds) {
    const tmpl = tmplOf(view, bone);
    if (tmpl === undefined) continue;
    geo(probe, view).push(mkShape(tmpl, `probe-${view}-${bone}`, bone, [{ x: 0, y: -0.04 }, { x: 0.08, y: -0.04 }, { x: 0.08, y: 0.04 }, { x: 0, y: 0.04 }]));
  }
  for (const i of validateAsset(probe as never) as any[]) {
    const names = [...(i.data?.shapes ?? []), ...[...String(i.message?.en ?? "").matchAll(/"(probe-[^"]+)"/g)].map((m) => m[1]!)] as string[];
    for (const n of names) { const m = /^probe-(elevation|front|plan)-(.+)$/.exec(n); if (m) bannedIn[m[1]!]!.add(m[2]!); }
  }
}
const hull2 = (pts: P[]): P[] => {
  const p = pts.slice().sort((a, b) => a.x - b.x || a.y - b.y);
  if (p.length < 3) return p;
  const lower: P[] = [], upper: P[] = [];
  for (const q of p) { while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop(); lower.push(q); }
  for (let i = p.length - 1; i >= 0; i--) { const q = p[i]!; while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop(); upper.push(q); }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
};
const areaOf = (r: P[]): number => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!, q = r[(i + 1) % r.length]!; a += p.x * q.y - q.x * p.y; } return Math.abs(a) / 2; };
const REST = restFrames((asset as any).rig) as Record<string, { frame: any }>;
const regionOfSolid = new Map<string, string>(((asset as any).volumes[sc].thermal as any[]).filter((t) => t.of !== undefined && t.region !== undefined).map((t) => [t.of, t.region]));
/** Draws a bone as its volume models it: every rigid body solid of the bone becomes a drawn prism (the convex outline of the solid across the axis of its tightest prism, swept over the extent of the solid along that axis), placed by the 3D frame of the bone like the solid itself; the filled shapes the bone had in the view give way to them. Returns the ops, or undefined when the bone has nothing to draw this way. */
const prismBone = (view: string, bone: string, state: any, solids: readonly any[]): any[] | undefined => {
  const list = geo(asset, view);
  const defs = new Map<string, any>(((asset as any).volumes[sc].body as any[]).map((x) => [x.id, x]));
  const mine = solids.filter((w) => w.bone === bone && w.slender !== true && defs.get(w.id)?.weights === undefined && defs.get(w.id)?.texture !== true);
  if (mine.length === 0 || solids.some((w) => w.bone === bone && defs.get(w.id)?.weights !== undefined)) return undefined;
  const f = state.bones[bone].frame as { o: any; u: any; v: any; w: any };
  const base = worldFrameOf(REST[bone]!.frame).o[2] as number;
  const old = (state.shapes[view] as any[]).filter((e) => e.shape.bone === bone && e.closed && e.shape.filled !== false && e.shape.role !== "ground").map((e) => e.shape.id as string);
  const oldShapes = list.filter((x: any) => old.includes(x.id));
  const template = (material: string | undefined): any => oldShapes.find((x: any) => x.material === material) ?? oldShapes[0] ?? tmplOf(view, bone);
  if (template(undefined) === undefined) return undefined;
  const depth = (w: any): number => { let m = 0; const n = w.positions.length / 3; for (let i = 0; i < w.positions.length; i += 3) m += view === "elevation" ? -w.positions[i + 2]! : view === "front" ? w.positions[i]! : w.positions[i + 1]!; return m / n; };
  const made: any[] = [];
  const taken = new Set<string>(list.filter((x: any) => !old.includes(x.id)).map((x: any) => x.id));
  const zs = oldShapes.map((x: any) => x.z ?? 0), z0 = zs.length > 0 ? Math.min(...zs) : (template(undefined).z ?? 0), z1 = zs.length > 0 ? Math.max(...zs) : z0;
  const ordered = mine.slice().sort((a, b) => depth(a) - depth(b));
  ordered.forEach((w, index) => {
    const local: number[][] = [];
    for (let i = 0; i < w.positions.length; i += 3) { const d = [w.positions[i]! - f.o.x, w.positions[i + 1]! - f.o.y, w.positions[i + 2]! - f.o.z]; local.push([d[0]! * f.u.x + d[1]! * f.u.y + d[2]! * f.u.z, d[0]! * f.v.x + d[1]! * f.v.y + d[2]! * f.v.z, d[0]! * f.w.x + d[1]! * f.w.y + d[2]! * f.w.z]); }
    let best: { sweep: string; ring: P[]; lo: number; hi: number; volume: number } | undefined;
    for (const [sweep, k, i, j] of [["w", 2, 0, 1], ["u", 0, 1, 2], ["v", 1, 0, 2]] as const) {
      const ring = hull2(local.map((q) => ({ x: q[i]!, y: q[j]! })));
      const lo = Math.min(...local.map((q) => q[k]!)), hi = Math.max(...local.map((q) => q[k]!));
      const volume = areaOf(ring) * (hi - lo);
      if (ring.length >= 3 && hi - lo > 1e-4 && (best === undefined || volume < best.volume)) best = { sweep, ring, lo, hi, volume };
    }
    if (best === undefined) return;
    const def = defs.get(w.id)!;
    const tmpl = template(def.material);
    let id = w.id as string;
    while (taken.has(id)) id = `${id}-p`;
    taken.add(id);
    const ring = rdpRing(best.ring, Math.max(0.0004, feature / 25));
    const shape = mkShape(tmpl, id, bone, ring.length >= 3 ? ring : best.ring);
    shape.points = (ring.length >= 3 ? ring : best.ring).map((p) => [+p.x.toFixed(5), +p.y.toFixed(5)]);
    shape.filled = true;
    shape.tone = def.tone ?? tmpl.tone;
    if (def.material !== undefined) shape.material = def.material;
    const region = regionOfSolid.get(w.id);
    if (region !== undefined) { delete shape.regions; shape.region = region; }
    shape.z = +(z0 + ((z1 - z0) * index) / Math.max(1, ordered.length - 1)).toFixed(4);
    shape.solid = { kind: "extruded", sweep: best.sweep, z0: +(best.lo + (best.sweep === "w" ? base : 0)).toFixed(5), z1: +(best.hi + (best.sweep === "w" ? base : 0)).toFixed(5) };
    made.push(shape);
  });
  if (made.length === 0) return undefined;
  const lost = new Set<string>(oldShapes.flatMap((x: any) => [...(x.region === undefined ? [] : [x.region]), ...(x.regions ?? [])]));
  for (const m of made) for (const r of [...(m.region === undefined ? [] : [m.region]), ...(m.regions ?? [])]) lost.delete(r);
  if (lost.size > 0) { const first = made[0]; const all = new Set<string>([...(first.region === undefined ? [] : [first.region]), ...(first.regions ?? []), ...lost]); delete first.region; first.regions = [...all]; }
  const first = list.findIndex((x: any) => old.includes(x.id));
  const anchor = first > 0 ? list[first - 1].id : first === 0 ? undefined : list[list.length - 1]?.id;
  const out: any[] = old.map((id) => ({ op: "shape-remove", scale: sc, view, id, group: `prism|${view}|${bone}` }));
  let after = anchor ?? list.find((x: any) => !old.includes(x.id))?.id;
  for (const shape of made) { out.push({ op: "shape-add", scale: sc, view, after, shape, group: `prism|${view}|${bone}` }); after = shape.id; }
  return out;
};
/** Redraws one drawn shape from the body solid of the same id on the same bone: the candidates are the body of revolution about the bone axis whose profile is the radial envelope of the solid along the axis (a round limb) and the tightest prism of the solid (a flat or boxy part), a shape that is already revolved stays revolved; the better one for the view is kept and the shape keeps its id, order, tone, material and regions. */
const refitFrom = (view: string, id: string, state: any, solids: readonly any[]): any[] | undefined => {
  const shape = geo(asset, view).find((x: any) => x.id === id);
  const def = ((asset as any).volumes[sc].body as any[]).find((x) => x.id === id);
  if (shape === undefined || def === undefined || def.bone !== shape.bone || def.weights !== undefined || def.slender === true || def.texture === true || shape.kind === "member") return undefined;
  const w = solids.find((x) => x.id === id);
  if (w === undefined) return undefined;
  const f = state.bones[shape.bone].frame as { o: any; u: any; v: any; w: any };
  const local: number[][] = [];
  for (let i = 0; i < w.positions.length; i += 3) { const d = [w.positions[i]! - f.o.x, w.positions[i + 1]! - f.o.y, w.positions[i + 2]! - f.o.z]; local.push([d[0]! * f.u.x + d[1]! * f.u.y + d[2]! * f.u.z, d[0]! * f.v.x + d[1]! * f.v.y + d[2]! * f.v.z, d[0]! * f.w.x + d[1]! * f.w.y + d[2]! * f.w.z]); }
  const keep = (points: P[], solid: any): any => { const out = mkShape(shape, id, shape.bone, points); out.points = points.map((p) => [+p.x.toFixed(5), +p.y.toFixed(5)]); out.solid = solid; return out; };
  const so = shape.solid;
  const out: any[] = [];
  {
    const lo = Math.min(...local.map((q) => q[0]!)), hi = Math.max(...local.map((q) => q[0]!));
    const n = 16, step = (hi - lo) / n;
    if (step > 1e-5) {
      const radii = Array.from({ length: n + 1 }, (_, i) => { const a = lo + i * step; let r = 0; for (const q of local) if (Math.abs(q[0]! - a) <= step * 0.75) r = Math.max(r, Math.hypot(q[1]!, q[2]!)); return r; });
      const upper = radii.map((r, i) => ({ x: lo + i * step, y: Math.max(r, 1e-4) }));
      out.push(keep([...upper, ...upper.slice().reverse().map((p) => ({ x: p.x, y: -p.y }))], { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: 0, axisZ: 0, mirror: true }));
    }
  }
  if (so?.kind === "revolved") return out;
  const base = worldFrameOf(REST[shape.bone]!.frame).o[2] as number;
  let best: { sweep: string; ring: P[]; lo: number; hi: number; volume: number } | undefined;
  for (const [sweep, k, i, j] of [["w", 2, 0, 1], ["u", 0, 1, 2], ["v", 1, 0, 2]] as const) {
    const ring = hull2(local.map((q) => ({ x: q[i]!, y: q[j]! })));
    const lo = Math.min(...local.map((q) => q[k]!)), hi = Math.max(...local.map((q) => q[k]!));
    const volume = areaOf(ring) * (hi - lo);
    if (ring.length >= 3 && hi - lo > 1e-4 && (best === undefined || volume < best.volume)) best = { sweep, ring, lo, hi, volume };
  }
  if (best === undefined) return out;
  const ring = rdpRing(best.ring, Math.max(0.0004, feature / 25));
  out.push(keep(ring.length >= 3 ? ring : best.ring, { kind: "extruded", sweep: best.sweep, z0: +(best.lo + (best.sweep === "w" ? base : 0)).toFixed(5), z1: +(best.hi + (best.sweep === "w" ? base : 0)).toFixed(5) }));
  return out;
};
const refitDone = new Set<string>();
const tryRefit = (view: string, id: string, why: string, cur: ReturnType<typeof measure>): boolean => {
  if (refitDone.has(`${view}|${id}`)) return false;
  refitDone.add(`${view}|${id}`);
  const candidates = refitFrom(view, id, cur.state, cur.solids);
  if (candidates === undefined || candidates.length === 0) return false;
  const l = geo(asset, view), i = l.findIndex((x: any) => x.id === id), was = l[i];
  let next: any, after: ReturnType<typeof measure>["res"] | undefined;
  for (const candidate of candidates) {
    l[i] = candidate;
    let got: ReturnType<typeof measure>["res"] | undefined;
    try { got = measure(view).res; } catch { got = undefined; }
    if (got !== undefined && improves(got, after ?? cur.res, view)) { next = candidate; after = got; }
  }
  if (next === undefined || after === undefined) { l[i] = was; log(`refit of ${view} ${id} (${why}) revert`); return false; }
  l[i] = next;
  ops.push({ op: "shape", scale: sc, view, id, shape: structuredClone(next) });
  log(`refit ${view} ${id} from its solid as ${next.solid.kind}${next.solid.sweep ? " " + next.solid.sweep : ""} (${why}): ${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)}, iou ${metricOf(cur.res, view).iou.toFixed(4)} -> ${metricOf(after, view).iou.toFixed(4)}`);
  return true;
};
const prismDone = new Set<string>();
const tryPrism = (view: string, bone: string, why: string, cur: ReturnType<typeof measure>): boolean => {
  if (prismDone.has(`${view}|${bone}`)) return false;
  prismDone.add(`${view}|${bone}`);
  const made = prismBone(view, bone, cur.state, cur.solids);
  if (made === undefined) { log(`no prisms for ${view} ${bone}`); return false; }
  const snapshot = structuredClone(geo(asset, view)), kept = ops.length;
  try { for (const o of made) { applyOther(asset, o); ops.push(o); } } catch (e) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`prisms of ${view} ${bone} do not apply: ${String(e).slice(0, 120)}`); return false; }
  let after: ReturnType<typeof measure>["res"];
  try { after = measure(view).res; } catch (e) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`prisms of ${view} ${bone} cannot be evaluated: ${String(e).slice(0, 120)}`); return false; }
  const score1 = (rs: ReturnType<typeof measure>["res"]): number => { const r = rs.find((x) => x.view === view)!; return Math.max(r.h / limit, r.iou < 0.925 ? 1 + (0.925 - r.iou) * 20 : 0); };
  if (!improves(after, cur.res, view)) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`prisms of ${view} ${bone} (${why}) revert (${score1(cur.res).toFixed(3)} -> ${score1(after).toFixed(3)})`); return false; }
  log(`prisms ${view} ${bone}: ${made.filter((o) => o.op === "shape-add").length} solids drawn, ${made.filter((o) => o.op === "shape-remove").length} shapes replaced (${why}): ${score1(cur.res).toFixed(3)} -> ${score1(after).toFixed(3)}`);
  return true;
};
/** Draws a bone as the outline its volume has in the view: the body solids of the bone (or the named ones), grouped by material and tone, are traced in the default pose and each trace becomes one drawn plate, a prism whose profile is the trace and whose sweep is the bone axis that points most towards the viewer over the extent of the solids along it (shortened about its middle where the sweep would smear the outline by more than one and a half features or 12 mm; the trace is eroded by that smear first, so the prism projects to the outline again, and set in by 1 mm so that hairline slits between parts stay open as in the body), placed by the 3D frame of the bone; the filled shapes the bone had in the view give way to them unless `keep`. A bone larger than 0.15 m that stands more than 20° askew to the view gets no plate (OBLIQUE=1 allows it): the outline traced in the default pose would be stretched in every pose that turns the bone upright, and the three views would no longer be one object. Returns the ops, or undefined when nothing can be traced. */
const plateBone = (view: string, bone: string, state: any, solids: readonly any[], only?: ReadonlySet<string>, keep = false, name?: string): any[] | undefined => {
  const list = geo(asset, view);
  const defs = new Map<string, any>(((asset as any).volumes[sc].body as any[]).map((x) => [x.id, x]));
  const mine = solids.filter((w) => w.bone === bone && w.slender !== true && defs.get(w.id)?.weights === undefined && defs.get(w.id)?.texture !== true && (only === undefined || only.has(w.id)));
  if (mine.length === 0) return undefined;
  const f = state.bones[bone].frame as { o: any; u: any; v: any; w: any };
  const o2 = project(view as any, f.o.x, f.o.y, f.o.z);
  const axis2 = (a: any): P => { const q = project(view as any, f.o.x + a.x, f.o.y + a.y, f.o.z + a.z); return { x: q.x - o2.x, y: q.y - o2.y }; };
  const proj = { u: axis2(f.u), v: axis2(f.v), w: axis2(f.w) } as Record<string, P>;
  const sweep = (["w", "u", "v"] as const).slice().sort((a, b) => Math.hypot(proj[a]!.x, proj[a]!.y) - Math.hypot(proj[b]!.x, proj[b]!.y))[0]!;
  const [k1, k2] = sweep === "w" ? ["u", "v"] : sweep === "u" ? ["v", "w"] : ["u", "w"];
  const [e1, e2, es] = [proj[k1!]!, proj[k2!]!, proj[sweep]!];
  const det = e1.x * e2.y - e1.y * e2.x;
  if (Math.abs(det) < 0.2) return undefined;
  if (process.env.OBLIQUE === undefined && Math.hypot(es.x, es.y) > 0.35) {
    let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
    for (const w of mine) for (let i = 0; i < w.positions.length; i += 3) { const q = project(view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); x0 = Math.min(x0, q.x); x1 = Math.max(x1, q.x); y0 = Math.min(y0, q.y); y1 = Math.max(y1, q.y); }
    if (Math.max(x1 - x0, y1 - y0) > 0.15) { log(`no plate for ${view} ${bone}: the bone stands askew to the view (${(Math.asin(Math.min(1, Math.hypot(es.x, es.y))) * 180 / Math.PI).toFixed(0)}°) and its outline would not hold in other poses`); return undefined; }
  }
  const base = sweep === "w" ? (worldFrameOf(REST[bone]!.frame).o[2] as number) : 0;
  const old = keep ? [] : (state.shapes[view] as any[]).filter((e) => e.shape.bone === bone && e.closed && e.shape.filled !== false && e.shape.role !== "ground").map((e) => e.shape.id as string);
  const oldShapes = list.filter((x: any) => old.includes(x.id));
  const anyOf = list.filter((x: any) => x.bone === bone && x.role === "body" && x.filled !== false && x.kind !== "member");
  const template = (material: string | undefined): any => oldShapes.find((x: any) => x.material === material) ?? anyOf.find((x: any) => x.material === material) ?? oldShapes[0] ?? anyOf[0] ?? tmplOf(view, bone);
  if (template(undefined) === undefined) return undefined;
  const depth = (w: any): number => { let m = 0; const n = w.positions.length / 3; for (let i = 0; i < w.positions.length; i += 3) m += view === "elevation" ? -w.positions[i + 2]! : view === "front" ? w.positions[i]! : w.positions[i + 1]!; return m / n; };
  const groups = new Map<string, any[]>();
  for (const w of mine) { const d = defs.get(w.id)!; const key = `${d.material ?? ""}|${d.tone ?? ""}`; groups.set(key, [...(groups.get(key) ?? []), w]); }
  const ordered = [...groups.values()].sort((a, b) => Math.min(...a.map(depth)) - Math.min(...b.map(depth)));
  const zs = oldShapes.map((x: any) => x.z ?? 0), z0 = zs.length > 0 ? Math.min(...zs) : (template(undefined).z ?? 0), z1 = zs.length > 0 ? Math.max(...zs) : z0;
  const taken = new Set<string>(list.filter((x: any) => !old.includes(x.id)).map((x: any) => x.id));
  const made: any[] = [];
  ordered.forEach((group, index) => {
    let lo = Infinity, hi = -Infinity;
    for (const w of group) for (let i = 0; i < w.positions.length; i += 3) { const d = [w.positions[i]! - f.o.x, w.positions[i + 1]! - f.o.y, w.positions[i + 2]! - f.o.z]; const c = d[0]! * f[sweep].x + d[1]! * f[sweep].y + d[2]! * f[sweep].z; lo = Math.min(lo, c); hi = Math.max(hi, c); }
    const smear = Math.hypot(es.x, es.y), cap = Number(process.env.SMEAR ?? Math.min(1.5 * feature, 0.012)), mid = (lo + hi) / 2;
    const half = Math.max(5e-4, smear * (hi - lo) > cap ? cap / smear / 2 : (hi - lo) / 2);
    const loops = traceLoops(view, new Set(group.map((w) => w.id)), state, Number(process.env.PLATE_TOL ?? Math.min(0.0012, Math.max(0.0003, feature / 25))), Math.min(0.0025, Math.max(0.0004, feature / 12)), { x: half * es.x, y: half * es.y }, process.env.INFL === undefined ? -Number(process.env.INSET ?? 0.001) : INFL);
    if (loops.length === 0) return;
    const local = loops.map((loop) => loop.map((t) => { const dx = t.x - o2.x - mid * es.x, dy = t.y - o2.y - mid * es.y; return { x: (e2.y * dx - e2.x * dy) / det, y: (-e1.y * dx + e1.x * dy) / det }; }));
    const def = defs.get(group[0].id)!;
    const tmpl = template(def.material);
    let id = name !== undefined && ordered.length === 1 ? name : group.length === 1 ? (group[0].id as string) : `${bone}-${def.material ?? "body"}`;
    while (taken.has(id)) id = `${id}-p`;
    taken.add(id);
    const shape = mkShapeMulti(tmpl, id, bone, local);
    shape.filled = true;
    shape.tone = def.tone ?? tmpl.tone;
    if (def.material !== undefined) shape.material = def.material;
    const regions = [...new Set(group.map((w) => regionOfSolid.get(w.id)).filter((r) => r !== undefined))];
    if (regions.length === 1) { delete shape.regions; shape.region = regions[0]; } else if (regions.length > 1) { delete shape.region; shape.regions = regions; }
    shape.z = +(z0 + ((z1 - z0) * index) / Math.max(1, ordered.length - 1)).toFixed(4);
    shape.solid = { kind: "extruded", sweep, z0: +(mid - half + base).toFixed(5), z1: +(mid + half + base).toFixed(5) };
    made.push(shape);
  });
  if (made.length === 0) return undefined;
  if (!keep) {
    const lost = new Set<string>(oldShapes.flatMap((x: any) => [...(x.region === undefined ? [] : [x.region]), ...(x.regions ?? [])]));
    for (const m of made) for (const r of [...(m.region === undefined ? [] : [m.region]), ...(m.regions ?? [])]) lost.delete(r);
    if (lost.size > 0) { const first = made[0]; const all = new Set<string>([...(first.region === undefined ? [] : [first.region]), ...(first.regions ?? []), ...lost]); delete first.region; first.regions = [...all]; }
  }
  const first = list.findIndex((x: any) => old.includes(x.id));
  const anchor = first > 0 ? list[first - 1].id : first === 0 ? undefined : (anyOf[anyOf.length - 1]?.id ?? list[list.length - 1]?.id);
  const out: any[] = old.map((id) => ({ op: "shape-remove", scale: sc, view, id, group: `plate|${view}|${bone}` }));
  let after = anchor ?? list.find((x: any) => !old.includes(x.id))?.id;
  for (const shape of made) { out.push({ op: "shape-add", scale: sc, view, after, shape, group: `plate|${view}|${bone}` }); after = shape.id; }
  return out;
};
const plateDone = new Set<string>();
const tryPlate = (view: string, bone: string, why: string, cur: ReturnType<typeof measure>): boolean => {
  if (plateDone.has(`${view}|${bone}`)) return false;
  plateDone.add(`${view}|${bone}`);
  const made = plateBone(view, bone, cur.state, cur.solids);
  if (made === undefined) { log(`no plate for ${view} ${bone}`); return false; }
  const snapshot = structuredClone(geo(asset, view)), kept = ops.length;
  let after: ReturnType<typeof measure>["res"];
  try { for (const o of made) { applyOther(asset, o); ops.push(o); } after = measure(view).res; } catch (e) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plate of ${view} ${bone} does not apply: ${String(e).slice(0, 120)}`); return false; }
  if (!improves(after, cur.res, view)) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plate of ${view} ${bone} (${why}) revert (${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)})`); return false; }
  log(`plate ${view} ${bone}: ${made.filter((o) => o.op === "shape-add").length} outlines, ${made.filter((o) => o.op === "shape-remove").length} shapes replaced (${why}): ${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)}, iou ${metricOf(cur.res, view).iou.toFixed(4)} -> ${metricOf(after, view).iou.toFixed(4)}`);
  return true;
};
const treeDone = new Set<string>();
const kidsOf = (b: string): string[] => [b, ...(asset as any).rig.bones.filter((x: any) => x.parent === b).flatMap((x: any) => kidsOf(x.id))];
/** Draws a bone and every bone below it as plates in one step (a hand with its fingers, a foot with its toes): kept when the view improves. */
const tryPlateTree = (view: string, root: string, why: string, cur: ReturnType<typeof measure>): boolean => {
  if (treeDone.has(`${view}|${root}`)) return false;
  treeDone.add(`${view}|${root}`);
  const bones = kidsOf(root);
  if (bones.length < 2) return false;
  const snapshot = structuredClone(geo(asset, view)), kept = ops.length;
  let n = 0, after: ReturnType<typeof measure>["res"];
  try {
    for (const bone of bones) { const now = measure(view); const made = plateBone(view, bone, now.state, now.solids); if (made === undefined) continue; for (const o of made) { applyOther(asset, o); ops.push(o); } n++; }
    after = measure(view).res;
  } catch (e) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plates below ${view} ${root} do not apply: ${String(e).slice(0, 120)}`); return false; }
  if (n === 0 || !improves(after, cur.res, view)) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plates below ${view} ${root} (${why}) revert (${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)})`); return false; }
  for (const bone of bones) plateDone.add(`${view}|${bone}`);
  log(`plates below ${view} ${root}: ${n} bones drawn (${why}): ${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)}, iou ${metricOf(cur.res, view).iou.toFixed(4)} -> ${metricOf(after, view).iou.toFixed(4)}`);
  return true;
};
const pairDone = new Set<string>();
const mirrorOf = (b: string): string | undefined => { const m = /^(.*)-(l|r)((?:-.*)?)$/u.exec(b); if (m === null) return undefined; const other = `${m[1]}-${m[2] === "l" ? "r" : "l"}${m[3]}`; return boneParent.has(other) ? other : undefined; };
/** Draws a left bone and its right twin (each with the bones below it when `deep`) as plates in one step: two limbs that cover each other in a view cannot improve it one at a time. */
const tryPlatePair = (view: string, bone: string, deep: boolean, why: string, cur: ReturnType<typeof measure>): boolean => {
  const twin = mirrorOf(bone);
  const key = `${view}|${[bone, twin].sort().join("+")}|${deep}`;
  if (twin === undefined || pairDone.has(key)) return false;
  pairDone.add(key);
  const bones = deep ? [...kidsOf(bone), ...kidsOf(twin)] : [bone, twin];
  if (bones.length > 48) return false;
  const snapshot = structuredClone(geo(asset, view)), kept = ops.length;
  let n = 0, after: ReturnType<typeof measure>["res"];
  try {
    for (const b of bones) { const now = measure(view); const made = plateBone(view, b, now.state, now.solids); if (made === undefined) continue; for (const o of made) { applyOther(asset, o); ops.push(o); } n++; }
    after = measure(view).res;
  } catch (e) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plates of the pair ${view} ${bone} do not apply: ${String(e).slice(0, 120)}`); return false; }
  if (n === 0 || !improves(after, cur.res, view)) { (asset as any).representations[sc].geometry[view] = snapshot; ops.length = kept; log(`plates of the pair ${view} ${bone}${deep ? " and below" : ""} (${why}) revert (${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)})`); return false; }
  for (const b of bones) plateDone.add(`${view}|${b}`);
  log(`plates of the pair ${view} ${bone}${deep ? " and below" : ""}: ${n} bones drawn (${why}): ${metricOf(cur.res, view).max.toFixed(3)} -> ${metricOf(after, view).max.toFixed(3)}, iou ${metricOf(cur.res, view).iou.toFixed(4)} -> ${metricOf(after, view).iou.toFixed(4)}`);
  return true;
};
const slenderBones = new Set<string>((asset as any).volumes[sc].body.filter((s: any) => s.slender === true).map((s: any) => s.bone));
const done = new Set<string>();
const log = (s: string) => console.error(`[DEBUG] ${s}`);
const pass = (): void => {
for (let iter = 0; iter < maxIter; iter++) {
  const { state, solids, res } = measure();
  const summary = res.map((r) => `${r.view[0]}:${r.iou.toFixed(3)}/${(r.h * 1000).toFixed(0)}mm`).join(" ");
  if (iter === 0) log(`flat shapes are not placed on: ${VIEWPOINTS.map((v) => `${v}: ${[...bannedIn[v]!].join(" ")}`).join("; ")}`);
  if (!timeLeft()) { log("time budget used"); break; }
  log(`iter ${iter} 1:${sc} ${summary} ops=${ops.length}`);
  let all = res.flatMap((r) => r.clusters).sort((a, b) => b.d - a.d);
  if (all.length === 0) all = res.flatMap((r) => r.area).sort((a, b) => b.d - a.d);
  if (all.length === 0) break;
  const bonesSolids = bonesOf(state);
  let acted = false;
  for (const c of all) {
    const near = (state.shapes[c.view] as any[]).map((s) => { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(c, ring[i], ring[(i + 1) % ring.length])); return { s, m }; }).filter((x) => x.m < Math.max(0.012, feature)).sort((a, b) => a.m - b.m || (b.s.shape.z ?? 0) - (a.s.shape.z ?? 0));
    {
      const drawnBones = near.map((x) => x.s.shape.bone as string).filter((b) => bannedIn[c.view]!.has(b));
      const solidBones = solids.filter((w) => w.bone !== undefined && bannedIn[c.view]!.has(w.bone)).map((w) => { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(c.view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [P, P, P]; m = Math.min(m, distanceToTriangle(c, t)); } return { bone: w.bone as string, m }; }).filter((x) => x.m < Math.max(0.012, feature)).sort((a, b) => a.m - b.m).map((x) => x.bone);
      const inRing = (ring: P[]): boolean => { let k = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > c.y) !== (ring[j]!.y > c.y) && c.x < ((ring[j]!.x - ring[i]!.x) * (c.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) k = !k; return k; };
      const wide = (state.shapes[c.view] as any[]).map((s) => { let m = Infinity; for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) m = Math.min(m, dseg(c, ring[i], ring[(i + 1) % ring.length])); return { s, m }; }).filter((x) => x.m < Math.max(0.03, 2 * feature) || (x.s.closed && x.s.rings.some(inRing))).sort((a, b) => a.m - b.m);
      const again = wide.map((x) => x.s.shape).find((sh) => (bannedIn[c.view]!.has(sh.bone) || sh.solid !== undefined) && !refitDone.has(`${c.view}|${sh.id}`) && ((asset as any).volumes[sc].body as any[]).some((x) => x.id === sh.id && x.bone === sh.bone));
      if (again !== undefined) { tryRefit(c.view, again.id, `${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}`, { state, solids, res } as never); acted = true; break; }
      const target = [...new Set([...solidBones, ...drawnBones])].find((b) => !prismDone.has(`${c.view}|${b}`));
      const anyBones = process.env.ANYBONE === undefined ? [] : [...solids.filter((w) => w.bone !== undefined && w.slender !== true).map((w) => { let m = Infinity; for (let k = 0; k < w.indices.length; k += 3) { const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(c.view as any, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [P, P, P]; m = Math.min(m, distanceToTriangle(c, t)); } return { bone: w.bone as string, m }; }).filter((x) => x.m < Math.max(0.012, feature)).sort((a, b) => a.m - b.m).map((x) => x.bone), ...near.filter((x) => x.s.closed && x.s.shape.filled !== false && x.s.shape.role !== "ground").map((x) => x.s.shape.bone as string)];
      const plateTarget = [...new Set([...solidBones, ...drawnBones, ...anyBones])].find((b) => !plateDone.has(`${c.view}|${b}`));
      if (plateTarget !== undefined) {
        const why = `${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}`;
        if (!tryPlate(c.view, plateTarget, why, { state, solids, res } as never)) {
          const up = (b: string): string[] => { const out: string[] = []; let x: string | null | undefined = b; while (x) { out.push(x); x = boneParent.get(x); } return out; };
          const drawnNear = new Set<string>(wide.map((x) => x.s.shape.bone as string));
          const root = up(plateTarget).find((b) => drawnNear.has(b) && kidsOf(b).length > 1 && kidsOf(b).length <= 24 && !treeDone.has(`${c.view}|${b}`));
          if (!(root !== undefined && tryPlateTree(c.view, root, why, { state, solids, res } as never)) && !tryPlatePair(c.view, plateTarget, false, why, { state, solids, res } as never) && root !== undefined) tryPlatePair(c.view, root, true, why, { state, solids, res } as never);
        }
        acted = true; break;
      }
      if (target !== undefined) { if (tryPrism(c.view, target, `${c.side} ${c.d.toFixed(3)} at ${c.x.toFixed(3)},${c.y.toFixed(3)}`, { state, solids, res } as never)) { acted = true; break; } acted = true; break; }
    }
    let act: (() => void) | null = null;
    let actBone: string | undefined;
    if (c.side === "drawn-only") {
      const cand = near.find((x) => x.s.closed && x.s.shape.filled !== false && x.s.shape.role !== "ground" && bonesSolids.has(x.s.shape.bone) && !slenderBones.has(x.s.shape.bone) && !done.has(`${c.view}|r|${x.s.shape.id}`));
      if (!cand) continue;
      const id = cand.s.shape.id, bone = cand.s.shape.bone;
      actBone = bone;
      const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
      done.add(`${c.view}|r|${id}`);
      if (tr && bannedIn[c.view]!.has(tr.bone)) { continue; }
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
      actBone = bone;
      const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
      done.add(`${c.view}|r|${id}`);
      if (tr && bannedIn[c.view]!.has(tr.bone)) { continue; }
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
        actBone = bone;
        if (bannedIn[c.view]!.has(bone)) continue;
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
        actBone = bone;
        done.add(`${c.view}|a|${bone}`);
        const tr = tryTrace(c.view, bone, bonesSolids.get(bone)!, state);
        if (tr && bannedIn[c.view]!.has(tr.bone)) { continue; }
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
    const afterRes = measure(c.view).res;
    const after = vScore(afterRes, c.view);
    if (!improves(afterRes, res, c.view)) { (asset as any).representations[sc].geometry[c.view] = snapshot; ops.length = opsLen; log(`revert (${before.toFixed(3)} -> ${after.toFixed(3)})`);  if (actBone !== undefined && !prismDone.has(`${c.view}|${actBone}`)) tryPrism(c.view, actBone, `after the trace ${c.side} ${c.d.toFixed(3)}`, { state, solids, res } as never); }
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
      if (sh === undefined || sh.kind === "member" || ops.some((o) => o.op === "shape-add" && o.scale === sc && o.view === c.view && o.shape?.id === cand.id && /^(prism|plate)\|/u.test(String(o.group ?? "")))) continue;
      const saved = structuredClone(sh);
      const trial = (what: string, change: () => boolean): void => {
        Object.keys(sh).forEach((k) => delete sh[k]); Object.assign(sh, structuredClone(saved));
        if (!change()) return;
        let s2 = Infinity;
        try { s2 = viewScore(measure(c.view).res, c.view); } catch { s2 = Infinity; }
        if (s2 < (best?.s ?? before - 0.03)) best = { id: cand.id, shape: structuredClone(sh), s: s2, what };
      };
      for (const amount of [...new Set([c.d, c.d / 2, Math.min(c.d, 2 * feature), feature])]) {
        for (const k of SIZE) if (typeof saved[k] === "number") trial(`${k} ${(sign * amount * 1000).toFixed(1)}mm`, () => { const v = saved[k] + sign * amount * (k === "w" || k === "h" ? 2 : 1); if (v <= 1e-4) return false; sh[k] = +v.toFixed(5); return true; });
        for (const k of POSITION) if (typeof saved[k] === "number") for (const dir of [1, -1]) trial(`${k} ${(dir * amount * 1000).toFixed(1)}mm`, () => { sh[k] = +(saved[k] + dir * amount).toFixed(5); return true; });
        if (saved.solid?.kind === "revolved") for (const k of ["axisZ", "axisX"]) for (const dir of [1, -1]) trial(`revolved ${k} ${(dir * amount * 1000).toFixed(1)}mm`, () => { sh.solid = { ...saved.solid, [k]: +((saved.solid[k] ?? 0) + dir * amount).toFixed(5) }; return true; });
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
/** Takes out again every traced shape this run or an earlier one added that the guard does not need: without it the view is no worse. */
const prune = (): void => {
  for (const o of ops.filter((x) => x.op === "shape-add" && x.scale === sc && !/^(prism|plate)\|/u.test(String(x.group ?? ""))).slice()) {
    if (!timeLeft()) break;
    const l = geo(asset, o.view), i = l.findIndex((x: any) => x.id === o.shape.id);
    if (i < 0) continue;
    const before = measure(o.view).res, was = l[i];
    l.splice(i, 1);
    const after = measure(o.view).res;
    const [a, b] = [metricOf(after, o.view), metricOf(before, o.view)];
    if (a.max <= b.max + 0.005 && a.sum <= b.sum + 1e-9 && a.iou >= b.iou - 0.0003) { for (let k = ops.length - 1; k >= 0; k--) if (ops[k].scale === sc && ops[k].view === o.view && (ops[k].id ?? ops[k].shape?.id) === o.shape.id) ops.splice(k, 1); log(`pruned ${o.view} ${o.shape.id}: not needed`); }
    else l.splice(i, 0, was);
  }
};
if (process.env.PRUNE !== "0") prune();
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
        const sc2 = Math.max(metricOf(measure(worst.view).res, worst.view).max, ...cur.res.filter((r) => r.view !== worst.view).map((r) => metricOf(cur.res, r.view).max));
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
/** Takes one shape of a view back to what the file had before the repair; a shape that came with a group of ops (the plates or prisms of a bone) takes the whole group back, so that what the group removed returns with it. */
const revertShape = (view: string, id: string, seen = new Set<string>()): void => {
  if (seen.has(id)) return;
  seen.add(id);
  const groups = new Set<string>(ops.filter((o) => o.scale === sc && o.view === view && (o.id ?? o.shape?.id) === id && o.group !== undefined).map((o) => o.group as string));
  const others = ops.filter((o) => o.scale === sc && o.view === view && groups.has(o.group)).map((o) => (o.id ?? o.shape?.id) as string);
  const l = geo(asset, view), p = geo(pristine, view) as any[];
  const i = l.findIndex((x: any) => x.id === id), was = p.find((x: any) => x.id === id);
  if (was !== undefined && i >= 0) l[i] = structuredClone(was); else if (i >= 0) l.splice(i, 1); else if (was !== undefined) { const before = p.slice(0, p.indexOf(was)).reverse().find((x: any) => l.some((y: any) => y.id === x.id)); l.splice(before === undefined ? 0 : l.findIndex((y: any) => y.id === before.id) + 1, 0, structuredClone(was)); }
  for (let k = ops.length - 1; k >= 0; k--) if (ops[k].scale === sc && ops[k].view === view && (ops[k].id ?? ops[k].shape?.id) === id) ops.splice(k, 1);
  for (const other of others) revertShape(view, other, seen);
};
/** Named steps by hand before the greedy pass (HAND, one step per line, fields split by `|`): `plate|view|bone[/*]|[solid,solid]|[id]` draws a bone (and with `/*` every bone below it) as the traced outline of its solids on a plate (`plate+` keeps what is drawn), `prism|view|bone` draws a bone solid by solid, `tree|view|bone` does so for the bone and every bone below it, `prism1|view|bone|solid,solid` adds the prisms of single solids beside what is drawn, `refit|view|shape|0 or 1` redraws a shape from its solid (revolved or prism), `trace|view|bone|solid,solid|id` adds the traced outline of solids on a bone, `remove|view|shape`, `set|view|shape|json` merges values into a shape, `put|view|after|json` adds a shape. A step is kept as given; the validator and the final measure judge it. */
for (const step of (process.env.HAND ?? "").split("\n").map((x) => x.trim()).filter(Boolean)) {
  const [kind, view, target, extra, extra2] = step.split("|") as [string, string, string, string | undefined, string | undefined];
  const cur = measure(view);
  const before = metricOf(cur.res, view);
  const list = geo(asset, view);
  if (kind === "plate" || kind === "plate+") {
    const kids = (b: string): string[] => [b, ...(asset as any).rig.bones.filter((x: any) => x.parent === b).flatMap((x: any) => kids(x.id))];
    let n = 0;
    for (const bone of target.endsWith("/*") ? kids(target.slice(0, -2)) : [target]) { const now = measure(view); const made = plateBone(view, bone, now.state, now.solids, extra ? new Set(extra.split(",")) : undefined, kind === "plate+", extra2); if (made === undefined) continue; for (const o of made) { applyOther(asset, o); ops.push(o); } n++; }
    if (n === 0) { log(`hand ${step}: nothing to draw`); continue; }
  } else if (kind === "tree") {
    const kids = (b: string): string[] => [b, ...(asset as any).rig.bones.filter((x: any) => x.parent === b).flatMap((x: any) => kids(x.id))];
    let n = 0;
    for (const bone of kids(target)) { const now = measure(view); const made = prismBone(view, bone, now.state, now.solids); if (made === undefined) continue; for (const o of made) { applyOther(asset, o); ops.push(o); } n++; }
    if (n === 0) { log(`hand ${step}: nothing to draw`); continue; }
  } else if (kind === "prism") { const made = prismBone(view, target, cur.state, cur.solids); if (made === undefined) { log(`hand ${step}: nothing to draw`); continue; } for (const o of made) { applyOther(asset, o); ops.push(o); } }
  else if (kind === "prism1") {
    const wanted = new Set((extra ?? "").split(","));
    const made = (prismBone(view, target, cur.state, cur.solids) ?? []).filter((o) => o.op === "shape-add" && wanted.has(String(o.shape.id).replace(/(-p)+$/u, "")));
    if (made.length === 0) { log(`hand ${step}: nothing to draw`); continue; }
    let after = extra2 ?? list.filter((x: any) => x.bone === target).pop()?.id ?? list[list.length - 1].id;
    for (const o of made) { let id = String(o.shape.id).replace(/(-p)+$/u, ""); while (list.some((x: any) => x.id === id)) id = `${id}-p`; const shape = { ...o.shape, id }; delete shape.regions; if (shape.region === undefined) { const r = list.find((x: any) => x.bone === target && x.region !== undefined)?.region; if (r !== undefined) shape.region = r; } const op = { op: "shape-add", scale: sc, view, after, shape }; applyOther(asset, op); ops.push(op); after = id; }
  } else if (kind === "refit") { const c = refitFrom(view, target, cur.state, cur.solids)?.[Number(extra ?? 0)]; if (c === undefined) { log(`hand ${step}: no candidate`); continue; } list[list.findIndex((x: any) => x.id === target)] = c; ops.push({ op: "shape", scale: sc, view, id: target, shape: structuredClone(c) }); }
  else if (kind === "trace") {
    const loops = traceLoops(view, new Set((extra ?? "").split(",")), cur.state);
    const local = loops.map((l, k) => placeLocal(view, target, `h-${k}`, l));
    if (local.length === 0 || local.some((l) => l === null)) { log(`hand ${step}: cannot trace`); continue; }
    const tmpl = list.find((x: any) => x.bone === target && x.role === "body" && x.filled !== false && x.kind !== "member") ?? tmplOf(view, target);
    const id = extra2 ?? `${target}-sil`;
    const shape = mkShapeMulti(tmpl, id, target, local as P[][]);
    const op = list.some((x: any) => x.id === id) ? { op: "shape", scale: sc, view, id, shape } : { op: "shape-add", scale: sc, view, after: tmpl.id, shape };
    applyOther(asset, op); ops.push(op);
  } else if (kind === "remove") { const op = { op: "shape-remove", scale: sc, view, id: target }; applyOther(asset, op); ops.push(op); }
  else if (kind === "set") { const op = { op: "shape-set", scale: sc, view, id: target, set: JSON.parse(extra!) }; applyOther(asset, op); ops.push(op); }
  else if (kind === "put") { const op = { op: "shape-add", scale: sc, view, after: target, shape: JSON.parse(extra!) }; applyOther(asset, op); ops.push(op); }
  else throw new Error(`unknown hand step ${step}`);
  const after = metricOf(measure(view).res, view);
  log(`hand ${step.slice(0, 120)}: ${view} ${(before.max * limit * 1000).toFixed(1)} -> ${(after.max * limit * 1000).toFixed(1)} mm (or IoU penalty), iou ${before.iou.toFixed(4)} -> ${after.iou.toFixed(4)}`);
}
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
  if (issues.length > baseIssues) { for (const v of VIEWPOINTS) (asset as any).representations[sc].geometry[v] = structuredClone(geo(started, v)); ops.length = 0; ops.push(...startOps); issues = issuesOf(); log(`taken back every change of this run at 1:${sc} (what earlier runs repaired stays): ${issues.length} issues left`); break; }
  if (issues.length > baseIssues) { log(`validateAsset still reports ${issues.length} issues: ${issues.slice(0, 5).map((i) => `${i.code} ${i.path ?? ""} ${i.message?.en ?? ""}`.slice(0, 300)).join(" || ")}`); break; }
}
const fin = measure();
log(`issues ${issuesOf().length} new (inherited ${baseKeys.size})`);
log("final " + fin.res.map((r) => `${r.view[0]}:${r.iou.toFixed(3)}/${(r.h * 1000).toFixed(1)}mm(${r.clusters.length},${r.area.length})`).join(" "));
await writeOps(outFile!, ops);
