/** 🧰️ Shared pieces of the silhouette repair of the plants of SV-2: the catalog file and the repair ops of a plant, the drawing and the volume of one view as the guard compares them, the outline of a raster and the frame a drawn shape is written in.
 * @see ./retrace.ts
 * @see ./axes.ts */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, bodyOutline, drawnFor, project, referenceOf, strokeQuad, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, readOps, type Op } from "./ops.ts";
import { fileOf } from "./measure-lib.ts";
import { patched } from "./spans.ts";

export type P = { x: number; y: number };
export type Box = { minX: number; minY: number; maxX: number; maxY: number };
export const HERE = import.meta.dir;
export const opsFile = (slug: string): string => `${HERE}/repair/${slug}.json`;
export { featureOf, fileOf };

/** 📥️ A plant as its catalog file has it, with the repair ops of the plant applied (they are idempotent). */
export async function load(slug: string, withOps = true): Promise<{ text: string; doc: any; asset: any; ops: Op[] }> {
  const text = await Bun.file(fileOf(slug)).text();
  const doc = JSON.parse(text);
  const ops = await readOps(opsFile(slug));
  if (withOps) applyOps(doc.asset, ops);
  return { text, doc, asset: doc.asset, ops };
}

/** 📤️ Writes a plant back: a file that is what `JSON.stringify` writes is written that way, any other file keeps every byte that did not change (`spans.ts`). The file is read again right before it is written and must still be the text the plant was loaded from. */
export async function save(slug: string, text: string, doc: any): Promise<void> {
  if ((await Bun.file(fileOf(slug)).text()) !== text) throw new Error(`${slug}: the catalog file changed since it was read`);
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const plain = JSON.stringify(JSON.parse(text), null, indent) + "\n" === text;
  const next = plain ? JSON.stringify(doc, null, indent) + "\n" : patched(text, doc);
  if (next !== text) await Bun.write(fileOf(slug), next);
}

export const stateOf = (asset: any, scale: number): any => evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
export const bodyOf = (asset: any, state: any, scale: number): any[] => worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true) as any[];

export type Entry = { shape: any; rings: P[][]; closed: boolean; fills: boolean; quads: P[][]; box: Box };
export type Tri = { solid: any; ax: number; ay: number; bx: number; by: number; cx: number; cy: number };

const boxOfRings = (rings: readonly (readonly P[])[]): Box => {
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const r of rings) for (const p of r) { if (p.x < b.minX) b.minX = p.x; if (p.x > b.maxX) b.maxX = p.x; if (p.y < b.minY) b.minY = p.y; if (p.y > b.maxY) b.maxY = p.y; }
  return b;
};
export const grown = (b: Box, d: number): Box => ({ minX: b.minX - d, minY: b.minY - d, maxX: b.maxX + d, maxY: b.maxY + d });
export const touches = (a: Box, b: Box): boolean => a.maxX >= b.minX && a.minX <= b.maxX && a.maxY >= b.minY && a.minY <= b.maxY;

/** 🎞️ One view of a plant at one scale as the guard reads it: the drawn shapes that take part (which of them fill, the quads of the strokes), the projected reference of the volume and the frame of both. */
export function viewOf(asset: any, scale: number, view: string): { state: any; solids: any[]; entries: Entry[]; groups: { g: P[][]; box: Box }[]; frame: Box; feature: number; cell: number; snap: (b: Box) => Box } {
  const state = stateOf(asset, scale), solids = bodyOf(asset, state, scale), feature = featureOf(scale);
  const rep = asset.representations[String(scale)];
  if (rep.sections?.[view] !== undefined) throw new Error(`${asset.id} 1:${scale} ${view} is a section`);
  const pen = (s: any): number => ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
  const entries: Entry[] = (state.shapes[view] as any[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0).map((s) => {
    const fills = s.closed && (s.shape.filled !== false || bodyOutline(view as never, solids as never, s.rings));
    const quads: P[][] = [];
    if (!fills) for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) quads.push(strokeQuad(r[i], r[i + 1], Math.max(pen(s), 1e-4)) as P[]); }
    return { shape: s.shape, rings: s.rings, closed: s.closed, fills, quads, box: boxOfRings(fills ? s.rings : quads) };
  });
  const groups = (referenceOf(view as never, solids as never, undefined, drawnFor(asset, state, view as never, scale)).cut as P[][][]).map((g) => ({ g, box: boxOfRings(g) }));
  const frame = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const b of [...entries.map((e) => boxOfRings(e.rings)), ...groups.map((g) => g.box)]) { frame.minX = Math.min(frame.minX, b.minX); frame.minY = Math.min(frame.minY, b.minY); frame.maxX = Math.max(frame.maxX, b.maxX); frame.maxY = Math.max(frame.maxY, b.maxY); }
  const long = Math.max(frame.maxX - frame.minX, frame.maxY - frame.minY);
  const cell = Math.max(5e-5, Math.min(feature / 4, long / 256));
  const origin = { x: frame.minX - 2 * cell, y: frame.minY - 2 * cell };
  const snap = (b: Box): Box => ({ minX: origin.x + Math.floor((b.minX - origin.x) / cell) * cell, minY: origin.y + Math.floor((b.minY - origin.y) / cell) * cell, maxX: origin.x + Math.ceil((b.maxX - origin.x) / cell) * cell, maxY: origin.y + Math.ceil((b.maxY - origin.y) / cell) * cell });
  return { state, solids, entries, groups, frame, feature, cell, snap };
}

/** 🧊️ The volume of a view on a raster of a window, sealed like the guard seals it. */
export function volumeMask(groups: readonly { g: P[][]; box: Box }[], box: Box, cell: number): Mask {
  const m = new Mask(box, cell);
  for (const { g, box: b } of groups) if (touches(b, box)) m.fill(g, true);
  m.seal();
  return m;
}

/** ✏️ The drawing of a view on a raster of a window: the filling shapes and the strokes at their pen. */
export function drawnMask(entries: readonly Entry[], box: Box, cell: number, skip?: (e: Entry) => boolean): Mask {
  const m = new Mask(box, cell);
  for (const e of entries) if (touches(e.box, box) && !(skip?.(e))) { if (e.fills) m.fill(e.rings); else for (const q of e.quads) m.fill([q]); }
  return m;
}

/** 🔺️ The projected triangles of the body solids of a view (a slender solid included at its true section). */
export function trianglesOf(solids: readonly any[], view: string, keep: (solid: any) => boolean = () => true): Tri[] {
  const out: Tri[] = [];
  for (const solid of solids) if (keep(solid)) {
    const p = solid.positions, q: P[] = [];
    for (let i = 0; i < p.length; i += 3) q.push(project(view as never, p[i], p[i + 1], p[i + 2]));
    for (let t = 0; t < solid.indices.length; t += 3) { const [a, b, c] = [q[solid.indices[t]]!, q[solid.indices[t + 1]]!, q[solid.indices[t + 2]]!]; out.push({ solid, ax: a.x, ay: a.y, bx: b.x, by: b.y, cx: c.x, cy: c.y }); }
  }
  return out;
}
export const inTriangle = (t: Tri, x: number, y: number): boolean => {
  const d1 = (x - t.bx) * (t.ay - t.by) - (t.ax - t.bx) * (y - t.by), d2 = (x - t.cx) * (t.by - t.cy) - (t.bx - t.cx) * (y - t.cy), d3 = (x - t.ax) * (t.cy - t.ay) - (t.cx - t.ax) * (y - t.ay);
  return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
};

/** 🧵️ The boundary loops of a raster as rings of cell corners in metres (outer rings and holes; a diagonal touch stays two rings). */
export function trace(m: Mask): P[][] {
  const { width: w, height: h, cells, box, cell } = m;
  const stride = w + 1;
  const from: number[] = [], to: number[] = [];
  const out = new Map<number, number[]>();
  const add = (a: number, b: number): void => { const k = from.length; from.push(a); to.push(b); const l = out.get(a); if (l === undefined) out.set(a, [k]); else l.push(k); };
  const on = (i: number, j: number): boolean => i >= 0 && j >= 0 && i < w && j < h && cells[j * w + i] === 1;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (!cells[j * w + i]) continue;
    if (!on(i, j - 1)) add(j * stride + i, j * stride + i + 1);
    if (!on(i + 1, j)) add(j * stride + i + 1, (j + 1) * stride + i + 1);
    if (!on(i, j + 1)) add((j + 1) * stride + i + 1, (j + 1) * stride + i);
    if (!on(i - 1, j)) add((j + 1) * stride + i, j * stride + i);
  }
  const used = new Uint8Array(from.length);
  const loops: P[][] = [];
  for (let s = 0; s < from.length; s++) {
    if (used[s]) continue;
    const ring: P[] = [];
    let e = s;
    for (;;) {
      used[e] = 1;
      ring.push({ x: box.minX + (from[e]! % stride) * cell, y: box.minY + Math.floor(from[e]! / stride) * cell });
      const next = out.get(to[e]!)!.filter((k) => !used[k] || k === s);
      if (next.length === 0) break;
      let pick = next[0]!;
      if (next.length > 1) {
        const [dx, dy] = [(to[e]! % stride) - (from[e]! % stride), Math.floor(to[e]! / stride) - Math.floor(from[e]! / stride)];
        pick = next.find((k) => { const [ex, ey] = [(to[k]! % stride) - (from[k]! % stride), Math.floor(to[k]! / stride) - Math.floor(from[k]! / stride)]; return dx * ey - dy * ex > 0; }) ?? next[0]!;
      }
      e = pick;
      if (e === s) break;
    }
    loops.push(ring);
  }
  return loops;
}

export const areaOf = (r: readonly P[]): number => { let s = 0; for (let i = 0; i < r.length; i++) { const a = r[i]!, b = r[(i + 1) % r.length]!; s += a.x * b.y - b.x * a.y; } return s / 2; };

/** 🪒️ A closed ring with the vertices dropped that lie within `eps` of the chord of their neighbours (Douglas-Peucker from the two vertices farthest apart). */
export function simplify(points: readonly P[], eps: number): P[] {
  const n = points.length;
  if (n < 5) return points.slice();
  let far = 0, best = -1;
  for (let i = 1; i < n; i++) { const d = (points[i]!.x - points[0]!.x) ** 2 + (points[i]!.y - points[0]!.y) ** 2; if (d > best) { best = d; far = i; } }
  const keep = new Uint8Array(n);
  keep[0] = 1; keep[far] = 1;
  const stack: [number, number][] = [[0, far], [far, n]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    const p = points[a]!, q = points[b % n]!;
    const dx = q.x - p.x, dy = q.y - p.y, len = Math.hypot(dx, dy);
    let m = -1, md = eps;
    for (let i = a + 1; i < b; i++) { const r = points[i]!; const d = len === 0 ? Math.hypot(r.x - p.x, r.y - p.y) : Math.abs(dx * (r.y - p.y) - dy * (r.x - p.x)) / len; if (d > md) { md = d; m = i; } }
    if (m >= 0) { keep[m] = 1; stack.push([a, m], [m, b]); }
  }
  return points.filter((_, i) => keep[i]);
}

/** 🪞️ The affine frame a flat drawn shape is written in: the place in the view of the local points (0, 0), (1, 0) and (0, 1) of a shape with the bone and the placement of `shape`, undefined where the placement is not affine (a shape with a `solid`). */
export function frameOf(asset: any, scale: number, view: string, shape: any): { linear: (d: P) => P; toView: (p: P) => P; toLocal: (p: P) => P } | undefined {
  if (shape.solid !== undefined) return undefined;
  const { d: _d, points: _p, closed: _c, x: _x, y: _y, w: _w, h: _h, r: _r, cx: _cx, cy: _cy, rx: _rx, ry: _ry, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, rotationDeg: _rot, ...rest } = shape;
  const probe = { ...rest, kind: "poly", closed: true, filled: true, role: "body", points: [[0, 0], [1, 0], [1, 1], [0, 1]] };
  delete (probe as any).hatch;
  const sc = String(scale), rep = asset.representations[sc];
  const lone = { ...asset, representations: { ...asset.representations, [sc]: { ...rep, geometry: { ...rep.geometry, [view]: [probe] } } } };
  const hit = (stateOf(lone, scale).shapes[view] as any[]).find((s) => s.shape.id === shape.id);
  if (hit === undefined || hit.rings.length !== 1 || hit.rings[0].length !== 4) return undefined;
  const [o, a, c, b] = hit.rings[0] as P[];
  const [ux, uy, vx, vy] = [a.x - o.x, a.y - o.y, b.x - o.x, b.y - o.y];
  if (Math.hypot(o.x + ux + vx - c.x, o.y + uy + vy - c.y) > 1e-6) return undefined;
  const det = ux * vy - uy * vx;
  if (Math.abs(det) < 1e-9) return undefined;
  return { linear: (d) => ({ x: (d.x * vy - d.y * vx) / det, y: (ux * d.y - uy * d.x) / det }), toView: (p) => ({ x: o.x + p.x * ux + p.y * vx, y: o.y + p.x * uy + p.y * vy }), toLocal: (p) => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (dx * vy - dy * vx) / det, y: (ux * dy - uy * dx) / det }; } };
}

const GEOMETRY = ["d", "points", "closed", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "rotationDeg"];
const num = (v: number): string => { const s = v.toFixed(4).replace(/\.?0+$/u, ""); return s === "-0" || s === "" ? "0" : s; };

/** ✒️ A drawn shape with new rings (local metres): a path of closed rings, or of open lines with `open`; every other property stays in its place. */
export function withRings(shape: any, rings: readonly (readonly P[])[], open = false): any {
  const out: any = {};
  let placed = false;
  const d = rings.map((r) => `M ${r.map((p) => `${num(p.x)} ${num(p.y)}`).join(" L ")}${open ? "" : " Z"}`).join(" ");
  for (const [k, v] of Object.entries(shape)) {
    if (k === "kind") { out.kind = "path"; continue; }
    if (GEOMETRY.includes(k)) { if (!placed) { out.d = d; placed = true; } continue; }
    out[k] = v;
  }
  if (!placed) out.d = d;
  if (out.kind === undefined) out.kind = "path";
  return out;
}

/** 📌️ A drawn shape rewritten so that it evaluates to the given rings of the view (metres of the view): the local points start from the affine frame of the shape and, in the front and the plan (whose vertices `evaluate` carries one by one with the depth of the elevation of their bone), are corrected until every evaluated vertex lies on its target. Undefined for a shape that is not flat. */
export function placed(asset: any, scale: number, view: string, source: any, rings: readonly (readonly P[])[], open = false): { shape: any; residual: number } | undefined {
  const frame = frameOf(asset, scale, view, source);
  if (frame === undefined) return undefined;
  const local = rings.map((r) => r.map(frame.toLocal));
  let residual = 0;
  const sc = String(scale), rep = asset.representations[sc];
  for (let pass = 0; pass < 6; pass++) {
    const shape = withRings(source, local, open);
    const lone = { ...asset, representations: { ...asset.representations, [sc]: { ...rep, geometry: { ...rep.geometry, [view]: [shape] } } } };
    const hit = (stateOf(lone, scale).shapes[view] as any[]).find((s) => s.shape.id === source.id);
    if (hit === undefined || hit.rings.length !== rings.length || hit.rings.some((r: P[], k: number) => r.length !== rings[k]!.length)) return pass === 0 && view === "elevation" ? { shape, residual: 0 } : undefined;
    residual = 0;
    for (let k = 0; k < rings.length; k++) for (let i = 0; i < rings[k]!.length; i++) {
      const d = { x: rings[k]![i]!.x - hit.rings[k][i].x, y: rings[k]![i]!.y - hit.rings[k][i].y };
      residual = Math.max(residual, Math.hypot(d.x, d.y));
      const c = frame.linear(d);
      local[k]![i] = { x: local[k]![i]!.x + c.x, y: local[k]![i]!.y + c.y };
    }
    if (residual < 1.2e-4) return { shape, residual };
  }
  return { shape: withRings(source, local, open), residual };
}
