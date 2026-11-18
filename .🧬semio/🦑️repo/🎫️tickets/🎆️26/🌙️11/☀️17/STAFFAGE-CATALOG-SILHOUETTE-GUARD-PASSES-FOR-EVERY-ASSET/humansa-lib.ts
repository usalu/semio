/** 🧰 Shared helpers of the humans silhouette pass: clusters of mismatch, redrawing a shape from volume solids. */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, distanceToTriangle, drawnFor, fillSmallHoles, measureAsset, measureView, project, referenceOf, worldSolids, type Point, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

export { GATE, VIEWPOINTS, featureOf, measureAsset };
export type { Viewpoint };
export const loadAsset = async (name: string): Promise<Asset> => {
  const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
  return ((await readStreamedAsset(entry)) as { asset: Asset }).asset;
};
export const stateOf = (a: Asset, scale: number) => evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
export const shapesOf = (a: Asset, scale: number, view: Viewpoint): any[] => (a as any).representations[String(scale)].geometry[view];

export type Cluster = { side: "drawn-only" | "volume-only"; x: number; y: number; d: number };
export const clustersOf = (asset: Asset, scale: number, view: Viewpoint, max = 12): { iou: number; h: number; excess: number; clusters: Cluster[] } => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
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
  const toVol = distanceField(bv, w, h), toDrawn = distanceField(bd, w, h);
  const limit = 2 * feature;
  const out: Cluster[] = [];
  for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
    const list: { i: number; j: number; d: number }[] = [];
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
    list.sort((a, b) => b.d - a.d);
    const seen = new Set<number>();
    for (const c of list) {
      const key = Math.round(c.i / (0.08 / cell)) * 100000 + Math.round(c.j / (0.08 / cell));
      if (seen.has(key)) continue;
      seen.add(key);
      out.push({ side, x: +(box.minX + (c.i + 0.5) * cell).toFixed(4), y: +(box.minY + (c.j + 0.5) * cell).toFixed(4), d: +c.d.toFixed(4) });
    }
  }
  const excess = out.reduce((t, c) => t + (c.d - limit) / limit, 0);
  return { iou: m.iou, h: m.hausdorffM, excess, clusters: out.sort((a, b) => b.d - a.d).slice(0, max) };
};

const dseg = (P: Point, a: Point, b: Point): number => { const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy; let t = l === 0 ? 0 : ((P.x - a.x) * dx + (P.y - a.y) * dy) / l; t = Math.max(0, Math.min(1, t)); return Math.hypot(P.x - a.x - t * dx, P.y - a.y - t * dy); };
export const nearOf = (asset: Asset, scale: number, view: Viewpoint, P: Point, R: number): { shapes: { id: string; d: number }[]; solids: { id: string; d: number }[] } => {
  const state = stateOf(asset, scale);
  const shapes: { id: string; d: number }[] = [];
  for (const s of (state.shapes as any)[view]) {
    let d = Infinity;
    for (const ring of s.rings) for (let i = 0; i < ring.length - 1 + (s.closed ? 1 : 0); i++) d = Math.min(d, dseg(P, ring[i], ring[(i + 1) % ring.length]));
    if (d < R) shapes.push({ id: s.shape.id, d });
  }
  const solids: { id: string; d: number }[] = [];
  for (const w of worldSolids(asset, state.bones as never, scale).filter((x) => x.texture !== true)) {
    let d = Infinity;
    for (let k = 0; k < w.indices.length; k += 3) {
      const t = [0, 1, 2].map((j) => { const i = w.indices[k + j]! * 3; return project(view, w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!); }) as [Point, Point, Point];
      d = Math.min(d, distanceToTriangle(P, t));
    }
    if (d < R) solids.push({ id: w.id, d });
  }
  return { shapes, solids };
};

const simplify = (pts: Point[], tol: number): Point[] => {
  const keep = new Uint8Array(pts.length);
  const rec = (i: number, j: number): void => {
    let [best, at] = [0, -1];
    const [a, b] = [pts[i]!, pts[j]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const l = Math.hypot(dx, dy) || 1e-12;
    for (let k = i + 1; k < j; k++) {
      const d = Math.abs((pts[k]!.x - a.x) * dy - (pts[k]!.y - a.y) * dx) / l;
      if (d > best) [best, at] = [d, k];
    }
    if (best > tol) { keep[at] = 1; rec(i, at); rec(at, j); }
  };
  keep[0] = 1; keep[pts.length - 1] = 1;
  let far = 0;
  for (let k = 1; k < pts.length; k++) if (Math.hypot(pts[k]!.x - pts[0]!.x, pts[k]!.y - pts[0]!.y) > Math.hypot(pts[far]!.x - pts[0]!.x, pts[far]!.y - pts[0]!.y)) far = k;
  keep[far] = 1; rec(0, far); rec(far, pts.length - 1);
  return pts.filter((_, k) => keep[k]);
};
const loopsOf = (m: Mask): Point[][] => {
  const { width: w, height: h, cells, box, cell } = m;
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= w || j >= h ? 0 : cells[j * w + i]!);
  const next = new Map<string, string[]>();
  const add = (a: [number, number], b: [number, number]): void => { const k = a.join(","); (next.get(k) ?? next.set(k, []).get(k)!).push(b.join(",")); };
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (!at(i, j)) continue;
    if (!at(i, j - 1)) add([i, j], [i + 1, j]);
    if (!at(i + 1, j)) add([i + 1, j], [i + 1, j + 1]);
    if (!at(i, j + 1)) add([i + 1, j + 1], [i, j + 1]);
    if (!at(i - 1, j)) add([i, j + 1], [i, j]);
  }
  const loops: Point[][] = [];
  while (next.size > 0) {
    const start = next.keys().next().value as string;
    let cur = start;
    const pts: Point[] = [];
    for (;;) {
      const list = next.get(cur);
      if (list === undefined || list.length === 0) break;
      const nx = list.pop()!;
      if (list.length === 0) next.delete(cur);
      const [i, j] = cur.split(",").map(Number) as [number, number];
      pts.push({ x: box.minX + i * cell, y: box.minY + j * cell });
      cur = nx;
      if (cur === start) break;
    }
    if (pts.length > 2) loops.push(pts);
  }
  return loops;
};
const area = (p: Point[]): number => p.reduce((t, a, k) => { const b = p[(k + 1) % p.length]!; return t + (a.x * b.y - b.x * a.y) / 2; }, 0);
const hullOf = (pts: Point[]): Point[] => {
  const s = [...pts].sort((a, b) => a.x - b.x || a.y - b.y);
  const cross = (o: Point, a: Point, b: Point): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const half = (list: Point[]): Point[] => { const hh: Point[] = []; for (const p of list) { while (hh.length >= 2 && cross(hh[hh.length - 2]!, hh[hh.length - 1]!, p) <= 0) hh.pop(); hh.push(p); } hh.pop(); return hh; };
  return [...half(s), ...half([...s].reverse())];
};

export const plainOf = (asset: Asset, scale: number, view: Viewpoint, id: string): any | undefined => {
  const s = shapesOf(asset, scale, view).find((x) => x.id === id);
  return s === undefined || (s.solid !== undefined && view !== "elevation") || s.kind === "member" ? undefined : s;
};

const affineOf = (asset: Asset, scale: number, view: Viewpoint, original: any): { o: Point; ux: Point; uy: Point } => {
  if (original.kind === "poly" && Array.isArray(original.points) && original.points.length >= 3) {
    const ev = (stateOf(asset, scale).shapes as any)[view].find((q: any) => q.shape.id === original.id);
    const ring = ev?.rings?.[0] as Point[] | undefined;
    if (ring !== undefined && ring.length === original.points.length) {
      const P = original.points as number[][];
      const n = P.length;
      let [sx, sy, sxx, sxy, syy] = [0, 0, 0, 0, 0];
      for (const q of P) { sx += q[0]!; sy += q[1]!; sxx += q[0]! * q[0]!; sxy += q[0]! * q[1]!; syy += q[1]! * q[1]!; }
      const M = [[sxx, sxy, sx], [sxy, syy, sy], [sx, sy, n]];
      const solve = (rhs: number[]): number[] | undefined => {
        const A = M.map((row, i) => [...row, rhs[i]!]);
        for (let c = 0; c < 3; c++) {
          let piv = c;
          for (let r = c + 1; r < 3; r++) if (Math.abs(A[r]![c]!) > Math.abs(A[piv]![c]!)) piv = r;
          if (Math.abs(A[piv]![c]!) < 1e-12) return undefined;
          [A[c], A[piv]] = [A[piv]!, A[c]!];
          for (let r = 0; r < 3; r++) if (r !== c) { const f = A[r]![c]! / A[c]![c]!; for (let k = c; k < 4; k++) A[r]![k]! -= f * A[c]![k]!; }
        }
        return [A[0]![3]! / A[0]![0]!, A[1]![3]! / A[1]![1]!, A[2]![3]! / A[2]![2]!];
      };
      let [bx0, bx1, bx2, by0, by1, by2] = [0, 0, 0, 0, 0, 0];
      P.forEach((q, i) => { bx0 += q[0]! * ring[i]!.x; bx1 += q[1]! * ring[i]!.x; bx2 += ring[i]!.x; by0 += q[0]! * ring[i]!.y; by1 += q[1]! * ring[i]!.y; by2 += ring[i]!.y; });
      const X = solve([bx0, bx1, bx2]), Y = solve([by0, by1, by2]);
      if (X !== undefined && Y !== undefined) return { o: { x: X[2]!, y: Y[2]! }, ux: { x: X[0]!, y: Y[0]! }, uy: { x: X[1]!, y: Y[1]! } };
    }
  }
  const probe = structuredClone(asset) as any;
  probe.representations[String(scale)].geometry[view].push({ id: "zz-probe", kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true, filled: true, role: "body", z: 99, ...(original.bone === undefined ? {} : { bone: original.bone }) });
  const pr = (stateOf(probe, scale).shapes as any)[view].find((s: any) => s.shape.id === "zz-probe").rings[0] as Point[];
  const [o, ux, uy] = [pr[0]!, { x: pr[1]!.x - pr[0]!.x, y: pr[1]!.y - pr[0]!.y }, { x: pr[2]!.x - pr[0]!.x, y: pr[2]!.y - pr[0]!.y }];
  return { o, ux, uy };
};

const localizer = (asset: Asset, scale: number, view: Viewpoint, original: any): ((p: Point) => [number, number]) => {
  const { o, ux, uy } = affineOf(asset, scale, view, original);
  const det = ux.x * uy.y - ux.y * uy.x;
  return (p) => { const [dx, dy] = [p.x - o.x, p.y - o.y]; return [(dx * uy.y - dy * uy.x) / det, (-dx * ux.y + dy * ux.x) / det]; };
};

const centroidOf = (rings: Point[][]): Point => { const pts = rings[0]!; return { x: pts.reduce((t, p) => t + p.x, 0) / pts.length, y: pts.reduce((t, p) => t + p.y, 0) / pts.length }; };

const solidFrame = (asset: Asset, scale: number, view: Viewpoint, original: any): { local: (p: Point) => [number, number]; sweep: Point } | undefined => {
  const sd = original.solid;
  if (sd === undefined || sd.kind !== "extruded" || sd.sweep !== "w" || sd.zTip !== undefined) return undefined;
  const h = 0.01;
  const centre = (x: number, y: number, z0: number): Point => {
    const c = structuredClone(asset) as any;
    c.representations[String(scale)].geometry[view].push({ id: "zz-probe", kind: "poly", points: [[x, y], [x + h, y], [x, y + h]], closed: true, filled: true, role: "body", z: 99, bone: original.bone, solid: { kind: "extruded", sweep: "w", z0, z1: z0 + 1e-4 } });
    const ring = (stateOf(c, scale).shapes as any)[view].find((q: any) => q.shape.id === "zz-probe").rings[0] as Point[];
    return { x: ring.reduce((t, q) => t + q.x, 0) / ring.length, y: ring.reduce((t, q) => t + q.y, 0) / ring.length };
  };
  const [c0, cx, cy] = [centre(0, 0, sd.z0), centre(1, 0, sd.z0), centre(0, 1, sd.z0)];
  const moved = centre(0, 0, sd.z0 + 1);
  const k = sd.z1 - sd.z0;
  const [ux, uy] = [{ x: cx.x - c0.x, y: cx.y - c0.y }, { x: cy.x - c0.x, y: cy.y - c0.y }];
  const det = ux.x * uy.y - ux.y * uy.x;
  const t = h / 3;
  const local = (p: Point): [number, number] => { const [dx, dy] = [p.x - c0.x, p.y - c0.y]; return [(dx * uy.y - dy * uy.x) / det + t, (-dx * ux.y + dy * ux.x) / det + t]; };
  return { local, sweep: { x: (moved.x - c0.x) * k, y: (moved.y - c0.y) * k } };
};

export const strokePoints = (shape: any): [number, number][] | undefined => {
  if (shape.kind === "poly" && shape.closed === false) return shape.points as [number, number][];
  if (shape.kind === "path" && /^[ML0-9eE+.\- ]+$/.test(shape.d) && !shape.d.includes("Z")) {
    const t = shape.d.trim().split(/\s+/);
    const pts: [number, number][] = [];
    for (let i = 0; i < t.length; i++) if (t[i] === "M" || t[i] === "L") pts.push([Number(t[i + 1]), Number(t[i + 2])]);
    return pts.length >= 2 ? pts : undefined;
  }
  return undefined;
};

export const clipStroke = (asset: Asset, scale: number, view: Viewpoint, shapeId: string, erodeF = 0): any | undefined => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const original = plainOf(asset, scale, view, shapeId);
  if (original === undefined || original.filled !== false) return undefined;
  const pts = strokePoints(original);
  if (pts === undefined) return undefined;
  const { o, ux, uy } = affineOf(asset, scale, view, original);
  const fwd = (q: [number, number]): Point => ({ x: o.x + ux.x * q[0] + uy.x * q[1], y: o.y + ux.y * q[0] + uy.y * q[1] });
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const ref = referenceOf(view, solids, undefined, drawnFor(asset, state, view, scale)).cut;
  const cell = feature / 10;
  const world = pts.map(fwd);
  const b = { minX: Math.min(...world.map((p) => p.x)) - 0.05, maxX: Math.max(...world.map((p) => p.x)) + 0.05, minY: Math.min(...world.map((p) => p.y)) - 0.05, maxY: Math.max(...world.map((p) => p.y)) + 0.05 };
  const V = new Mask(b, cell);
  for (const g of ref) V.fill(g, true);
  V.seal();
  const rep = (asset.representations as any)[String(scale)];
  const pen = ((original.penMm ?? rep.pens[original.role] ?? 0) * scale) / 1000;
  const inside = (p: Point): boolean => {
    const k = Math.round((erodeF * feature + pen * 0.0) / cell);
    for (let dj = -k; dj <= k; dj++) for (let di = -k; di <= k; di++) if (di * di + dj * dj <= k * k && !V.has(p.x + di * cell, p.y + dj * cell)) return false;
    return V.has(p.x, p.y);
  };
  const samples: { q: [number, number]; ok: boolean; vertex: boolean }[] = [];
  for (let i = 0; i + 1 < pts.length; i++) {
    const len = Math.hypot(world[i + 1]!.x - world[i]!.x, world[i + 1]!.y - world[i]!.y);
    const n = Math.max(1, Math.ceil(len / cell));
    for (let k = 0; k < n; k++) { const t = k / n; const q: [number, number] = [pts[i]![0] + t * (pts[i + 1]![0] - pts[i]![0]), pts[i]![1] + t * (pts[i + 1]![1] - pts[i]![1])]; samples.push({ q, ok: inside(fwd(q)), vertex: k === 0 }); }
  }
  const last = pts[pts.length - 1]!;
  samples.push({ q: last, ok: inside(fwd(last)), vertex: true });
  let best: typeof samples = [];
  let run: typeof samples = [];
  for (const s of samples) { if (s.ok) run.push(s); else { if (run.length > best.length) best = run; run = []; } }
  if (run.length > best.length) best = run;
  if (best.length < 2 || best.length === samples.length) return undefined;
  const kept = best.filter((s, i) => s.vertex || i === 0 || i === best.length - 1).map((s) => [+s.q[0].toFixed(5), +s.q[1].toFixed(5)] as [number, number]);
  const shape: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(original)) {
    if (k === "points" || k === "d") continue;
    if (original.kind === "path" && k === "kind") { shape.kind = "path"; shape.d = `M ${kept.map((q) => q.join(" ")).join(" L ")}`; }
    else if (k === "closed") shape.closed = false;
    else shape[k] = v;
    if (original.kind === "poly" && k === "kind") shape.points = kept;
  }
  return shape;
};

const polyOf = (original: any, outline: Point[], local: (p: Point) => [number, number], feature: number, tolF: number, exact = false, fit?: (simple: Point[]) => [number, number][] | undefined): any => {
  const simple = exact ? outline : simplify([...outline, outline[0]!], feature * tolF).slice(0, -1);
  const fitted = fit === undefined ? undefined : fit(simple);
  const points = fitted !== undefined ? fitted.map((q) => q.map((v) => +v.toFixed(4))) : simple.map((p) => local(p).map((v) => +v.toFixed(4)));
  const shape: Record<string, unknown> = {};
  for (const [k, v] of Object.entries(original)) {
    if (k === "kind" && v === "path") { shape.kind = "path"; shape.d = `M ${points.map((q) => q.join(" ")).join(" L ")} Z`; }
    else if (k === "kind") { shape.kind = "poly"; shape.points = points; shape.closed = true; }
    else if (["d", "points", "closed", "followTip", "rx", "ry", "cx", "cy", "x", "y", "w", "h", "r", "x0", "y0", "x1", "y1", "r0", "r1", "rotationDeg"].includes(k)) continue;
    else shape[k] = v;
  }
  if (shape.filled === undefined) shape.filled = true;
  return shape;
};

const outerOf = (m: Mask): Point[] | undefined => {
  const loops = loopsOf(m).sort((p, q) => Math.abs(area(q)) - Math.abs(area(p)));
  if (loops[0] === undefined) return undefined;
  return area(loops[0]) < 0 ? [...loops[0]].reverse() : loops[0];
};

export const redrawShape = (asset: Asset, scale: number, view: Viewpoint, shapeId: string, solidIds: string[], mode = "contour", tolF = 0.12, keep?: (x: number, y: number) => boolean): any | undefined => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const original = plainOf(asset, scale, view, shapeId);
  if (original === undefined) return undefined;
  const frame = original.solid === undefined ? undefined : solidFrame(asset, scale, view, original);
  if (original.solid !== undefined && frame === undefined) return undefined;
  const local = frame === undefined ? localizer(asset, scale, view, original) : frame.local;
  const matches = (id: string): boolean => solidIds.some((q) => q === "*" || q === id || (q.endsWith("*") && id.startsWith(q.slice(0, -1))));
  const chosen = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true && matches(w.id));
  if (chosen.length === 0) return undefined;
  const ref = referenceOf(view, chosen, undefined, drawnFor(asset, state, view, scale)).cut;
  if (mode === "hull") return polyOf(original, hullOf(ref.flatMap((g) => g.flatMap((r) => [...r]))), local, feature, tolF, true);
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const g of ref) for (const r of g) for (const p of r) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  const cell = feature / 10;
  const m = new Mask({ minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell }, cell);
  for (const g of ref) m.fill(g, true);
  m.seal();
  if (frame !== undefined) {
    const src = Uint8Array.from(m.cells);
    const steps = Math.max(1, Math.ceil(Math.hypot(frame.sweep.x, frame.sweep.y) / cell));
    for (let j = 0; j < m.height; j++) for (let i = 0; i < m.width; i++) {
      let ok = src[j * m.width + i]!;
      for (let t = 1; ok && t <= steps; t++) { const [ii, jj] = [Math.round(i + (frame.sweep.x * t) / steps / cell), Math.round(j + (frame.sweep.y * t) / steps / cell)]; ok = ii >= 0 && jj >= 0 && ii < m.width && jj < m.height ? src[jj * m.width + ii]! : 0; }
      m.cells[j * m.width + i] = ok;
    }
  }
  if (keep !== undefined) for (let j = 0; j < m.height; j++) for (let i = 0; i < m.width; i++) if (!keep(box0(m, i), box1(m, j))) m.cells[j * m.width + i] = 0;
  const outline = outerOf(m);
  return outline === undefined ? undefined : polyOf(original, outline, local, feature, tolF);
};

const box0 = (m: Mask, i: number): number => m.box.minX + (i + 0.5) * m.cell;
const box1 = (m: Mask, j: number): number => m.box.minY + (j + 0.5) * m.cell;

const fitter = (asset: Asset, scale: number, view: Viewpoint, original: any, iters = 8): ((target: Point[]) => [number, number][] | undefined) => (target) => {
  const feature = featureOf(scale);
  const local = localizer(asset, scale, view, original);
  const { ux, uy } = affineOf(asset, scale, view, original);
  const det = ux.x * uy.y - ux.y * uy.x;
  const inv = (dx: number, dy: number): [number, number] => [(dx * uy.y - dy * uy.x) / det, (-dx * ux.y + dy * ux.x) / det];
  let pts: [number, number][] = target.map((p) => local(p));
  for (let it = 0; it < iters; it++) {
    const shape: any = { ...original, kind: "poly", points: pts.map((q) => [+q[0].toFixed(5), +q[1].toFixed(5)]), closed: true };
    delete shape.d; delete shape.solid;
    const probe = structuredClone(asset) as any;
    const list = probe.representations[String(scale)].geometry[view] as any[];
    list[list.findIndex((x) => x.id === original.id)] = shape;
    const ring = ((stateOf(probe, scale).shapes as any)[view].find((x: any) => x.shape.id === original.id).rings[0]) as Point[];
    if (ring.length !== pts.length) return undefined;
    let err = 0;
    pts = pts.map((q, i) => { const [dx, dy] = [target[i]!.x - ring[i]!.x, target[i]!.y - ring[i]!.y]; err = Math.max(err, Math.hypot(dx, dy)); const [a, b] = inv(dx, dy); return [q[0] + a, q[1] + b] as [number, number]; });
    if (err < feature / 20) break;
  }
  return pts;
};

export const patchShape = (asset: Asset, scale: number, view: Viewpoint, shapeId: string, c: Cluster, discR: number, marginF = 0.5, tolF = 0.12, side: "clip" | "grow" = c.side === "drawn-only" ? "clip" : "grow", iter = false): any | undefined => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const original = plainOf(asset, scale, view, shapeId);
  if (original === undefined || original.filled === false) return undefined;
  const ev = (state.shapes as any)[view].find((s: any) => s.shape.id === shapeId);
  if (ev === undefined || !ev.closed) return undefined;
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const ref = referenceOf(view, solids, undefined, drawnFor(asset, state, view, scale)).cut;
  const cell = feature / 10;
  const b = { minX: c.x - discR, minY: c.y - discR, maxX: c.x + discR, maxY: c.y + discR };
  for (const r of ev.rings) for (const p of r as Point[]) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  const box = { minX: b.minX - 3 * cell, minY: b.minY - 3 * cell, maxX: b.maxX + 3 * cell, maxY: b.maxY + 3 * cell };
  const A = new Mask(box, cell);
  A.fill(ev.rings);
  const V = new Mask(box, cell);
  for (const g of ref) V.fill(g, true);
  V.seal();
  const [w, h] = [A.width, A.height];
  const k = Math.max(1, Math.round((marginF * feature) / cell));
  const near = new Uint8Array(w * h);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (V.cells[j * w + i]) for (let dj = -k; dj <= k; dj++) for (let di = -k; di <= k; di++) { const [ii, jj] = [i + di, j + dj]; if (ii >= 0 && jj >= 0 && ii < w && jj < h && di * di + dj * dj <= k * k) near[jj * w + ii] = 1; }
  const out = new Mask(box, cell);
  out.cells.set(A.cells);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const [x, y] = [box.minX + (i + 0.5) * cell, box.minY + (j + 0.5) * cell];
    if (Math.hypot(x - c.x, y - c.y) > discR) continue;
    const k2 = j * w + i;
    if (side === "clip") { if (A.cells[k2] && !near[k2]) out.cells[k2] = 0; }
    else if (V.cells[k2]) out.cells[k2] = 1;
  }
  const outline = outerOf(out);
  return outline === undefined ? undefined : polyOf(original, outline, localizer(asset, scale, view, original), feature, tolF, false, iter ? fitter(asset, scale, view, original) : undefined);
};

export const applyShape = (asset: Asset, scale: number, view: Viewpoint, shape: any): void => {
  const list = shapesOf(asset, scale, view);
  list[list.findIndex((s) => s.id === shape.id)] = shape;
};

export const checkOf = (asset: Asset, scale: number): { fails: string[]; score: number } => {
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale as never, undefined, undefined, feature);
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const fails: string[] = [];
  let score = 0;
  m.bbox.forEach((b, k) => { if (Math.abs(b - dims[k]!) > feature) { fails.push(`bbox${k}(${b.toFixed(4)}vs${dims[k]})`); score += 1; } });
  for (const v of VIEWPOINTS) {
    const x = m.views[v];
    if (x.iou < GATE.iou) { fails.push(`i${v[0]}(${x.iou.toFixed(3)})`); score += 1; }
    if (x.hausdorffM > 2 * feature) { fails.push(`h${v[0]}(${x.hausdorffM.toFixed(4)})`); score += 1; }
  }
  return { fails, score };
};

export const patchOf = (asset: Asset, scale: number, view: Viewpoint, px: number, py: number, r: number, cols = 70): string[] => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const { cell, box } = m;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  if (process.env.FILLED) {
    const holes = Math.round((4 * feature / cell) ** 2);
    drawn.cells.set(fillSmallHoles(drawn.cells, drawn.width, drawn.height, holes));
    vol.cells.set(fillSmallHoles(vol.cells, vol.width, vol.height, holes));
  }
  const step = Math.max(1, Math.round((2 * r) / cell / cols));
  const rows: string[] = [];
  for (let y = py + r; y > py - r; y -= step * cell * 2) {
    let line = "";
    for (let x = px - r; x < px + r; x += step * cell) {
      const i = Math.floor((x - box.minX) / cell), j = Math.floor((y - box.minY) / cell);
      const [a, b] = [drawn.cells[j * drawn.width + i], vol.cells[j * drawn.width + i]];
      line += a && b ? "#" : a ? "D" : b ? "V" : ".";
    }
    rows.push(line);
  }
  rows.push(`grid x ${(px - r).toFixed(3)}..${(px + r).toFixed(3)} step ${(step * cell * 1000).toFixed(2)}mm per char horizontally, y ${(py + r).toFixed(3)}..${(py - r).toFixed(3)} step ${(step * cell * 2000).toFixed(2)}mm per row`);
  return rows;
};

export const areaClustersOf = (asset: Asset, scale: number, view: Viewpoint, max = 12): Cluster[] => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const { cell, box } = m;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const bin = Math.max(0.04, 3 * feature);
  const per = Math.max(1, Math.round(bin / cell));
  const bins = new Map<string, { d: number; v: number; sx: number; sy: number }>();
  for (let j = 0; j < vol.height; j++) for (let i = 0; i < vol.width; i++) {
    const [a, b] = [drawn.cells[j * vol.width + i]!, vol.cells[j * vol.width + i]!];
    if (a === b) continue;
    const key = `${Math.floor(i / per)},${Math.floor(j / per)}`;
    const e = bins.get(key) ?? { d: 0, v: 0, sx: 0, sy: 0 };
    if (a) e.d++; else e.v++;
    e.sx += box.minX + (i + 0.5) * cell; e.sy += box.minY + (j + 0.5) * cell;
    bins.set(key, e);
  }
  return [...bins.values()].map((e) => ({ e, n: e.d + e.v })).sort((p, q) => q.n - p.n).slice(0, max).map(({ e, n }) => ({ side: e.d >= e.v ? "drawn-only" as const : "volume-only" as const, x: +(e.sx / n).toFixed(4), y: +(e.sy / n).toFixed(4), d: +bin.toFixed(3) }));
};

export const maskPair = (asset: Asset, scale: number, view: Viewpoint): { drawn: Mask; vol: Mask; cell: number; box: { minX: number; minY: number; maxX: number; maxY: number }; feature: number } => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const { cell, box } = m;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  return { drawn, vol, cell, box, feature };
};

export const applyOps = (asset: Asset, file: string | undefined, scale?: number): void => {
  if (file === undefined || !require("node:fs").existsSync(file)) return;
  for (const o of JSON.parse(require("node:fs").readFileSync(file, "utf8"))) {
    if (scale !== undefined && o.scale !== String(scale)) continue;
    const list: any[] = o.op.startsWith("shape") ? (asset as any).representations[o.scale].geometry[o.view] : (asset as any).volumes[o.scale].body;
    const find = (id: string): number => list.findIndex((x) => x.id === id);
    if (o.op.endsWith("-add")) list.splice(find(o.after) + 1, 0, o.shape ?? o.solid);
    else if (o.op.endsWith("-set")) Object.assign(list[find(o.id)], o.set);
    else list[find(o.id)] = o.shape ?? o.solid;
  }
};

export const worldContour = (asset: Asset, scale: number, view: Viewpoint, solidIds: string[], tolF = 0.12): Point[] | undefined => {
  const feature = featureOf(scale);
  const state = stateOf(asset, scale);
  const matches = (id: string): boolean => solidIds.some((q) => q === "*" || q === id || (q.endsWith("*") && id.startsWith(q.slice(0, -1))));
  const chosen = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true && matches(w.id));
  if (chosen.length === 0) return undefined;
  const ref = referenceOf(view, chosen, undefined, drawnFor(asset, state, view, scale)).cut;
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const g of ref) for (const r of g) for (const p of r) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  const cell = feature / 10;
  const m = new Mask({ minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell }, cell);
  for (const g of ref) m.fill(g, true);
  m.seal();
  const outline = outerOf(m);
  return outline === undefined ? undefined : simplify([...outline, outline[0]!], feature * tolF).slice(0, -1);
};

export const redrawIter = (asset0: Asset, scale: number, view: Viewpoint, shapeId: string, solidIds: string[], iters = 8, tolF = 0.12, dropSolid = false): any | undefined => {
  const feature = featureOf(scale);
  let asset = asset0;
  const first = shapesOf(asset0, scale, view).find((q) => q.id === shapeId);
  if (first === undefined) return undefined;
  if (first.solid !== undefined) {
    if (!dropSolid) return undefined;
    asset = structuredClone(asset0) as Asset;
    const c = shapesOf(asset, scale, view).find((q) => q.id === shapeId)!;
    delete c.solid;
  }
  const original = plainOf(asset, scale, view, shapeId)!;
  const target = worldContour(asset, scale, view, solidIds, tolF);
  if (target === undefined) return undefined;
  const local = localizer(asset, scale, view, original);
  const { o, ux, uy } = affineOf(asset, scale, view, original);
  const det = ux.x * uy.y - ux.y * uy.x;
  const inv = (dx: number, dy: number): [number, number] => [(dx * uy.y - dy * uy.x) / det, (-dx * ux.y + dy * ux.x) / det];
  void o;
  let pts: [number, number][] = target.map((p) => local(p));
  let shape = polyOf(original, target, local, feature, 0, true);
  for (let it = 0; it < iters; it++) {
    shape = { ...shape, kind: "poly", points: pts.map((q) => [+q[0].toFixed(5), +q[1].toFixed(5)]), closed: true };
    delete shape.d;
    const probe = structuredClone(asset) as any;
    const list = probe.representations[String(scale)].geometry[view] as any[];
    list[list.findIndex((s) => s.id === shapeId)] = shape;
    const ev = (stateOf(probe, scale).shapes as any)[view].find((s: any) => s.shape.id === shapeId);
    const ring = ev.rings[0] as Point[];
    if (ring.length !== pts.length) { if (process.env.DBG) console.log("[DEBUG] ring", ring.length, "pts", pts.length); return undefined; }
    let err = 0;
    pts = pts.map((q, i) => { const [dx, dy] = [target[i]!.x - ring[i]!.x, target[i]!.y - ring[i]!.y]; err = Math.max(err, Math.hypot(dx, dy)); const [a, b] = inv(dx, dy); return [q[0] + a, q[1] + b] as [number, number]; });
    if (err < feature / 20) break;
  }
  return polyOf(original, target, () => [0, 0], feature, 0, true) && { ...shape, points: pts.map((q) => [+q[0].toFixed(5), +q[1].toFixed(5)]) };
};

export const addWorldShape = (asset: Asset, scale: number, view: Viewpoint, after: string, template: Record<string, unknown>, target: Point[]): any | undefined => {
  const list = shapesOf(asset, scale, view);
  const shape: any = { ...template, kind: "poly", points: target.map(() => [0, 0]), closed: true };
  delete shape.d; delete shape.solid;
  const probe = structuredClone(asset) as Asset;
  const pl = shapesOf(probe, scale, view);
  const at = pl.findIndex((s) => s.id === after);
  pl.splice(at + 1, 0, { ...shape, points: [[0, 0], [1, 0], [0, 1], ...target.slice(3).map(() => [0, 0])] });
  const fit = fitter(probe, scale, view, shape);
  void list;
  const pts = fit(target);
  if (pts === undefined) return undefined;
  return { ...shape, points: pts.map((q) => q.map((v) => +v.toFixed(4))) };
};

export const boneLocal = (asset: Asset, scale: number, view: Viewpoint, bone: string): ((p: Point) => [number, number]) => localizer(asset, scale, view, { id: "zz-bone", bone });
export { worldContour as contourOf };
