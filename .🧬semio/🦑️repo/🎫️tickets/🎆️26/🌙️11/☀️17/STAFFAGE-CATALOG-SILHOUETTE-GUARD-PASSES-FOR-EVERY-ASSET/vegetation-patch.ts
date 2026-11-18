/** 🩹 Local patches for the remaining boundary clusters where the volume has material the drawing lacks: for every cluster of one view that is farther than the limit from the other boundary, the cells of the volume that the drawing does not cover within a window around the cluster become a small polygon on the bone of the volume solid there.
 *  bun vegetation-patch.ts <asset> <out.json> <scale>/<view> ... */
import { writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, boundsOf, distanceField, drawnFor, fillSmallHoles, measureView, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, out, ...views] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
type P = [number, number];
function trace(m: Mask): P[][] {
  const { width: w, height: h, cells } = m;
  const stride = w + 1;
  const from: number[] = [];
  const to: number[] = [];
  const outE = new Map<number, number[]>();
  const add = (a: number, b: number): void => { const k = from.length; from.push(a); to.push(b); const l = outE.get(a); if (l === undefined) outE.set(a, [k]); else l.push(k); };
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
      ring.push([from[e]! % stride, Math.floor(from[e]! / stride)]);
      const next = outE.get(to[e]!)!.filter((k) => !used[k] || k === s);
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

function rdp(points: P[], eps: number): P[] {
  const n = points.length;
  if (n < 5) return points;
  let far = 0;
  let best = -1;
  for (let i = 1; i < n; i++) { const d = (points[i]![0] - points[0]![0]) ** 2 + (points[i]![1] - points[0]![1]) ** 2; if (d > best) { best = d; far = i; } }
  const keep = new Uint8Array(n);
  keep[0] = 1;
  keep[far] = 1;
  const stack: [number, number][] = [[0, far], [far, n]];
  while (stack.length > 0) {
    const [a, b] = stack.pop()!;
    const [p, q] = [points[a]!, points[b % n]!];
    const [dx, dy] = [q[0] - p[0], q[1] - p[1]];
    const len = Math.hypot(dx, dy);
    let m = -1;
    let md = eps;
    for (let i = a + 1; i < b; i++) { const r = points[i]!; const d = len === 0 ? Math.hypot(r[0] - p[0], r[1] - p[1]) : Math.abs(dx * (r[1] - p[1]) - dy * (r[0] - p[0])) / len; if (d > md) { md = d; m = i; } }
    if (m >= 0) { keep[m] = 1; stack.push([a, m], [m, b]); }
  }
  return points.filter((_, i) => keep[i]);
}
const cuts: unknown[] = [];
const result: unknown[] = [];
for (const sv of views) {
  const [scaleArg, view] = sv.split("/") as [string, string];
  const scale = Number(scaleArg);
  const feature = featureOf(scale);
  const limit = 2 * feature;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
  const solids = worldSolids(asset, state.bones as never, scale).filter((s) => s.texture !== true);
  const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
  const drawn = new Mask(m.box, m.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(m.box, m.cell);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.round((4 * feature / m.cell) ** 2);
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const toDrawn = distanceField(bd, w, h);
  const spots: { i: number; j: number; d: number }[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (bv[j * w + i]) { const d = Math.sqrt(toDrawn[j * w + i]!) * m.cell; if (d > limit) spots.push({ i, j, d }); }
  spots.sort((a, b) => b.d - a.d);
  const chosen: { i: number; j: number; d: number }[] = [];
  for (const s of spots) if (chosen.every((c) => Math.hypot(c.i - s.i, c.j - s.j) * m.cell > 3 * Math.max(c.d, limit))) { chosen.push(s); if (chosen.length >= 8) break; }
  const drawnBones = drawnFor(asset, state, view as never, scale);
  void drawnBones;
  chosen.forEach((c, n) => {
    const [px, py] = [m.box.minX + (c.i + 0.5) * m.cell, m.box.minY + (c.j + 0.5) * m.cell];
    const R = Math.max(3 * c.d, 6 * feature);
    const [i0, i1, j0, j1] = [Math.max(0, Math.floor((px - R - m.box.minX) / m.cell)), Math.min(w - 1, Math.ceil((px + R - m.box.minX) / m.cell)), Math.max(0, Math.floor((py - R - m.box.minY) / m.cell)), Math.min(h - 1, Math.ceil((py + R - m.box.minY) / m.cell))];
    const [ww, hh] = [i1 - i0 + 1, j1 - j0 + 1];
    const sub = new Mask({ minX: m.box.minX + i0 * m.cell, minY: m.box.minY + j0 * m.cell, maxX: m.box.minX + (i1 + 1) * m.cell, maxY: m.box.minY + (j1 + 1) * m.cell }, m.cell);
    for (let j = 0; j < hh; j++) for (let i = 0; i < ww; i++) {
      const k = (j0 + j) * w + (i0 + i);
      if (Math.hypot(i0 + i - c.i, j0 + j - c.j) * m.cell <= R && vol.cells[k] && !drawn.cells[k]) sub.cells[j * sub.width + i] = 1;
    }
    let owner: { bone: string; role: string; tone: number; area: number } | undefined;
    for (const s of solids) {
      const g = referenceOf(view as never, [s], undefined, drawnFor(asset, state, view as never, scale)).cut;
      const b = g.reduce((a, q) => { const c2 = boundsOf(q); return [Math.min(a[0]!, c2[0]), Math.max(a[1]!, c2[1]), Math.min(a[2]!, c2[2]), Math.max(a[3]!, c2[3])]; }, [Infinity, -Infinity, Infinity, -Infinity]);
      if (px < b[0]! - m.cell * 2 || px > b[1]! + m.cell * 2 || py < b[2]! - m.cell * 2 || py > b[3]! + m.cell * 2) continue;
      const area = (b[1]! - b[0]!) * (b[3]! - b[2]!);
      if (owner === undefined || area < owner.area) {
        const body = ((asset.volumes as any)[String(scale)].body as any[]).find((q) => q.id === s.id);
        owner = { bone: s.bone!, role: body?.role === "detail" ? "detail" : "body", tone: body?.tone ?? 0.5, area };
      }
    }
    if (owner === undefined) return;
    const f = state.bones[owner.bone].world as readonly number[];
    const det = f[0]! * f[3]! - f[1]! * f[2]!;
    const back = ([x, y]: P): P => { const [dx, dy] = [x - f[4]!, y - f[5]!]; return [(f[3]! * dx - f[2]! * dy) / det, (-f[1]! * dx + f[0]! * dy) / det]; };
    const rings: string[] = [];
    const target: string[] = [];
    for (const loop of trace(sub)) {
      const world = loop.map(([i, j]) => [sub.box.minX + i * m.cell, sub.box.minY + j * m.cell] as P);
      let ar = 0;
      for (let q = 0; q < world.length; q++) { const [a, b] = [world[q]!, world[(q + 1) % world.length]!]; ar += a[0] * b[1] - b[0] * a[1]; }
      if (ar / 2 < (0.25 * feature) ** 2) continue;
      const local = view === "elevation" ? world.map(back) : world;
      rings.push(local.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
      target.push(world.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
    }
    if (rings.length > 0) result.push({ scale: String(scale), view, id: `${owner.bone}-patch-${view[0]}${n}`, bone: owner.bone, role: owner.role, tone: owner.tone, d: +c.d.toFixed(4), rings, target });
  });

  const toVol = distanceField(bv, w, h);
  const spots2: { i: number; j: number; d: number }[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (bd[j * w + i]) { const d = Math.sqrt(toVol[j * w + i]!) * m.cell; if (d > limit) spots2.push({ i, j, d }); }
  spots2.sort((a, b) => b.d - a.d);
  const chosen2: { i: number; j: number; d: number }[] = [];
  for (const s of spots2) if (chosen2.every((c) => Math.hypot(c.i - s.i, c.j - s.j) * m.cell > 3 * Math.max(c.d, limit))) { chosen2.push(s); if (chosen2.length >= 8) break; }
  const shapesNow = (state.shapes[view] as any[]).filter((q) => ["body", "detail", "glazing"].includes(q.shape.role) && q.rings.length > 0 && q.closed && q.shape.solid === undefined);
  const cutCells = new Uint8Array(w * h);
  for (const c of [...chosen2, ...chosen]) {
    const R = Math.max(3 * c.d, 6 * feature) / m.cell;
    for (let j = Math.max(0, Math.floor(c.j - R)); j <= Math.min(h - 1, Math.ceil(c.j + R)); j++) for (let i = Math.max(0, Math.floor(c.i - R)); i <= Math.min(w - 1, Math.ceil(c.i + R)); i++) if (Math.hypot(i - c.i, j - c.j) <= R && drawn.cells[j * w + i] && !vol.cells[j * w + i]) cutCells[j * w + i] = 1;
  }
  if (chosen2.length + chosen.length > 0) {
    for (const s of shapesNow) {
      const gb = s.rings.reduce((acc: number[], r: any[]) => r.reduce((a2: number[], q: any) => [Math.min(a2[0]!, q.x), Math.max(a2[1]!, q.x), Math.min(a2[2]!, q.y), Math.max(a2[3]!, q.y)], acc), [Infinity, -Infinity, Infinity, -Infinity]);
      const [i0, i1, j0, j1] = [Math.max(0, Math.floor((gb[0]! - m.box.minX) / m.cell) - 1), Math.min(w - 1, Math.ceil((gb[1]! - m.box.minX) / m.cell) + 1), Math.max(0, Math.floor((gb[2]! - m.box.minY) / m.cell) - 1), Math.min(h - 1, Math.ceil((gb[3]! - m.box.minY) / m.cell) + 1)];
      const sub = new Mask({ minX: m.box.minX + i0 * m.cell, minY: m.box.minY + j0 * m.cell, maxX: m.box.minX + (i1 + 1) * m.cell, maxY: m.box.minY + (j1 + 1) * m.cell }, m.cell);
      sub.fill(s.rings);
      let hit = 0;
      for (let j = 0; j < sub.height; j++) for (let i = 0; i < sub.width; i++) {
        const k = (j0 + j) * w + (i0 + i);
        if (sub.cells[j * sub.width + i] && cutCells[k]) { hit++; sub.cells[j * sub.width + i] = 0; }
      }
      if (hit === 0) continue;
      const f = state.bones[s.shape.bone].world as readonly number[];
      const det = f[0]! * f[3]! - f[1]! * f[2]!;
      const back = ([x, y]: P): P => { const [dx, dy] = [x - f[4]!, y - f[5]!]; return [(f[3]! * dx - f[2]! * dy) / det, (-f[1]! * dx + f[0]! * dy) / det]; };
      const rings: string[] = [];
      const target: string[] = [];
      for (const loop of trace(sub)) {
        const world = loop.map(([i, j]) => [sub.box.minX + i * m.cell, sub.box.minY + j * m.cell] as P);
        let ar = 0;
        for (let q = 0; q < world.length; q++) { const [a, b] = [world[q]!, world[(q + 1) % world.length]!]; ar += a[0] * b[1] - b[0] * a[1]; }
        ar /= 2;
        if (ar < 0 ? -ar < (2 * feature) ** 2 : ar < (0.25 * feature) ** 2) continue;
        const simple = rdp(world, 0.04 * feature);
        const local = view === "elevation" ? simple.map(back) : simple;
        rings.push(local.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
        target.push(simple.map(([x, y]) => `${+x.toFixed(4)} ${+y.toFixed(4)}`).join(";"));
      }
      cuts.push({ scale: String(scale), view, id: s.shape.id, bone: s.shape.bone, rings, target });
    }
  }
  console.log(`[DEBUG] ${name} 1:${scale} ${view}: ${spots2.length} drawn-only spots, ${chosen2.length} clusters, ${cuts.length} shapes to cut`);
  console.log(`[DEBUG] ${name} 1:${scale} ${view}: ${spots.length} spots, ${chosen.length} clusters`);
}
writeFileSync(out, JSON.stringify(result));
writeFileSync(out.replace('.json', '-cuts.json'), JSON.stringify(cuts));
