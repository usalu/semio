/** 🩹️ Brings the drawing of a plant onto its volume where the guard measures a boundary distance above two smallest features: per scale and view, every cluster of offending boundary cells is repaired on the guard's own raster. Where the volume shows beyond the drawing, a filled patch traced from the volume is added on the bone of the solid it belongs to, in the look of the neighbouring shape (it keeps half a smallest feature away from every white gap narrower than a smallest feature, so no unprintable gap arises between two outlines); where the drawing shows beyond the volume, the shapes that cover the excess are cut back to the volume (polygon difference) and strokes are clipped. A patch that `validateAsset` then calls `lateral-ambiguous` on its bone (the bone turns in a pose) moves to the parent bone, up the chain until it is accepted. The ops accumulate in `ops/<id>.json` for `apply.ts`; a run on a repaired file finds nothing and changes nothing.
 *  bun fix.ts <id> [--join <scale>:<view>:<shape>:<shape>]… [--file <asset.json>] [--scales 50,100] [--views elevation,front] [--rounds 6] [--limit 2] [--slack 1] [--dry]   (--limit: the boundary distance in smallest features above which a place is repaired, two by default as the guard; a smaller value also raises the IoU of thin parts; --slack: cells kept below the limit; --close: patches may reach up to a narrow white gap (for a place where keeping off leaves the body uncovered; check the print suitability after it); --join: two patches whose outlines the print profile finds closer than its smallest gap become one patch, their union closed over the gap, before the views are repaired)
 * @see ./lib.ts — the raster and the offences
 * @see ./apply.ts — replays the ops on the catalog file */
import { parseArgs } from "node:util";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { boundsOf, bodyOutline, fillSmallHoles, project } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { Mask } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { VIEWS, clusters, edgeDistance, inside, offences, scene, tileMasks, tiles, type Offence, type P, type Scene } from "./lib.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf, opsFile } from "./plants.ts";
import { area, dilate, erode, label, outline, simplify } from "./trace.ts";

const clipping: any = await import(Bun.resolveSync("polygon-clipping", "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/"));
const pc: { xor: (...g: any[]) => any[]; difference: (...g: any[]) => any[] } = clipping.default ?? clipping;
const { values, positionals } = parseArgs({ allowPositionals: true, options: { file: { type: "string" }, scales: { type: "string" }, views: { type: "string" }, rounds: { type: "string" }, dry: { type: "boolean" }, slack: { type: "string" }, limit: { type: "string" }, join: { type: "string", multiple: true }, close: { type: "boolean" } } });
const id = positionals[0]!;
let asset = (await Bun.file(values.file ?? fileOf(id)).json()).asset;
const issueKey = (i: any): string => JSON.stringify([i.code, i.path, i.data?.bone, [...(i.data?.shapes ?? [])].sort()]);
const inherited = new Set<string>((validateAsset(structuredClone(asset) as never) as any[]).map(issueKey));
const start = await readOps(opsFile(id));
applyOps(asset, start);
let ops: Op[] = start.slice();
const BASE = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const r4 = (x: number): number => Number(x.toFixed(4));
let serial = 0;

/** 🧭️ Carries world points of a view back into the local coordinates of a bone by probing `evaluate` with a shape of the final id on that bone (the carry of a front or plan vertex depends on the elevation twin of that id): an affine start, then Newton steps per vertex with a Jacobian from two neighbours one millimetre away, keeping the best place of every vertex. So a shape lands where it was traced whatever the carry of the view does; a vertex that cannot be placed within a millimetre (it stands on a jump of the carry) comes back undefined and is left out of its ring. */
function toLocal(scale: number, view: string, bone: string, template: any, sid: string, targets: P[], solid?: unknown): ([number, number] | undefined)[] {
  const rep = asset.representations[String(scale)];
  const probe = (points: [number, number][]): P[] => {
    const shape = { ...Object.fromEntries(BASE.filter((k) => template[k] !== undefined).map((k) => [k, template[k]])), id: sid, bone, filled: true, kind: "poly", points, closed: true, ...(solid === undefined ? {} : { solid }) };
    const geometry = { elevation: view === "elevation" ? [shape] : rep.geometry.elevation, front: view === "front" ? [shape] : [], plan: view === "plan" ? [shape] : [] };
    const a = { ...asset, representations: { ...asset.representations, [String(scale)]: { ...rep, geometry } } };
    const state: any = evaluate(a as never, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
    const ring = (state.shapes[view] as any[]).find((s) => s.shape.id === sid).rings[0] as P[];
    if (ring.length !== points.length) throw new Error(`probe of ${points.length} points came back with ${ring.length}`);
    return ring;
  };
  const [o, ex, ey] = probe([[0, 0], [1, 0], [0, 1]]);
  const [a, b, c, d] = [ex!.x - o!.x, ey!.x - o!.x, ex!.y - o!.y, ey!.y - o!.y];
  const det = a * d - b * c;
  let local = targets.map((p) => [(d * (p.x - o!.x) - b * (p.y - o!.y)) / det, (-c * (p.x - o!.x) + a * (p.y - o!.y)) / det] as [number, number]);
  const best = local.map((q) => ({ q, e: Infinity }));
  const h = 0.001;
  for (let step = 0; step < 12; step++) {
    const got = probe([...local, ...local.map(([x, y]) => [x + h, y] as [number, number]), ...local.map(([x, y]) => [x, y + h] as [number, number])]);
    const n = local.length;
    let worst = 0;
    local = local.map((q, i) => {
      const [g, gx, gy] = [got[i]!, got[n + i]!, got[2 * n + i]!];
      const [rx, ry] = [targets[i]!.x - g.x, targets[i]!.y - g.y];
      const e = Math.hypot(rx, ry);
      if (e < best[i]!.e) best[i] = { q, e };
      worst = Math.max(worst, best[i]!.e);
      const [j11, j12, j21, j22] = [(gx.x - g.x) / h, (gy.x - g.x) / h, (gx.y - g.y) / h, (gy.y - g.y) / h];
      const jd = j11 * j22 - j12 * j21;
      const sane = Math.abs(jd) > 0.05 && Math.abs(jd) < 20;
      const [u, v] = sane ? [(j22 * rx - j12 * ry) / jd, (-j21 * rx + j11 * ry) / jd] : [(d * rx - b * ry) / det, (-c * rx + a * ry) / det];
      const damp = step < 6 ? 1 : 0.5;
      return [q[0] + damp * u, q[1] + damp * v] as [number, number];
    });
    if (worst < 2e-5) break;
    if (step === 11 && worst > 1e-3) console.log(`   ! ${sid}@${bone}: ${best.filter((x) => x.e > 0.001).length} of ${n} vertices stand on a jump of the carry (up to ${(worst * 1000).toFixed(1)} mm) and are left out`);
  }
  return best.map((x) => (x.e > 0.001 ? undefined : [r4(x.q[0]), r4(x.q[1])]));
}

/** 🧊️ The local profile points of a solid shape for world points of its outline, where the outline of the shape in this view is its profile: the affine map from the authored points (a poly, or a path of straight segments) to the evaluated ring is fitted and inverted (where the ring has another number of points, a rigid placement by the two bounding boxes that puts every ring point on the authored outline); undefined where the outline is not the profile point by point (residual above half a millimetre). */
function solidLocal(source: any, rings: readonly P[][], targets: P[]): [number, number][] | undefined {
  const authored: [number, number][] = source.kind === "poly" ? source.points : source.kind === "path" && !/[HVCQAhvcqamlz]/u.test(source.d.replace(/[MLZ]/gu, "")) ? (source.d.match(/-?\d+(\.\d+)?(e-?\d+)?/gu) ?? []).map(Number).reduce((acc: [number, number][], x: number, i: number, all: number[]) => (i % 2 === 1 ? [...acc, [all[i - 1]!, x] as [number, number]] : acc), []) : [];
  const got = rings.flat();
  if (authored.length < 3 || got.length < 3) return undefined;
  if (authored.length !== got.length) {
    const box = (l: readonly (readonly [number, number])[]): [number, number, number, number] => [Math.min(...l.map((q) => q[0])), Math.max(...l.map((q) => q[0])), Math.min(...l.map((q) => q[1])), Math.max(...l.map((q) => q[1]))];
    const world = got.map((q) => [q.x, q.y] as [number, number]);
    const [wb, ab] = [box(world), box(authored)];
    let best: { m: number[]; e: number } | undefined;
    for (const [a, b, c, d] of [[1, 0, 0, 1], [-1, 0, 0, 1], [1, 0, 0, -1], [-1, 0, 0, -1], [0, 1, 1, 0], [0, -1, 1, 0], [0, 1, -1, 0], [0, -1, -1, 0]] as const) {
      const turned = box(authored.map(([x, y]) => [a * x + b * y, c * x + d * y] as [number, number]));
      const [e, f] = [(wb[0] + wb[1]) / 2 - (turned[0] + turned[1]) / 2, (wb[2] + wb[3]) / 2 - (turned[2] + turned[3]) / 2];
      const ring = authored.map(([x, y]) => ({ x: a * x + b * y + e, y: c * x + d * y + f }));
      const err = Math.max(...got.map((q) => edgeDistance(q, [ring], true)), Math.abs(turned[1] - turned[0] - (wb[1] - wb[0])), Math.abs(turned[3] - turned[2] - (wb[3] - wb[2])));
      if (best === undefined || err < best.e) best = { m: [a, b, c, d, e, f], e: err };
    }
    void ab;
    if (best === undefined || !(best.e < 5e-4)) return undefined;
    const [a, b, c, d, e, f] = best.m as [number, number, number, number, number, number];
    const det = a * d - b * c;
    return targets.map((q) => [r4((d * (q.x - e) - b * (q.y - f)) / det), r4((-c * (q.x - e) + a * (q.y - f)) / det)]);
  }
  const solve = (pick: (q: P) => number): [number, number, number] => {
    let [sxx, sxy, sx, syy, sy, n, bx, by, b1] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    authored.forEach(([x, y], i) => { const t = pick(got[i]!); sxx += x * x; sxy += x * y; sx += x; syy += y * y; sy += y; n++; bx += x * t; by += y * t; b1 += t; });
    const det = sxx * (syy * n - sy * sy) - sxy * (sxy * n - sy * sx) + sx * (sxy * sy - syy * sx);
    return [(bx * (syy * n - sy * sy) - sxy * (by * n - sy * b1) + sx * (by * sy - syy * b1)) / det, (sxx * (by * n - b1 * sy) - bx * (sxy * n - sy * sx) + sx * (sxy * b1 - by * sx)) / det, (sxx * (syy * b1 - sy * by) - sxy * (sxy * b1 - by * sx) + bx * (sxy * sy - syy * sx)) / det];
  };
  const [a, b, e] = solve((q) => q.x), [c, d, f] = solve((q) => q.y);
  const residual = Math.max(...authored.map(([x, y], i) => Math.hypot(a * x + b * y + e - got[i]!.x, c * x + d * y + f - got[i]!.y)));
  const det = a * d - b * c;
  if (!(residual < 5e-4) || Math.abs(det) < 1e-6) return undefined;
  return targets.map((q) => [r4((d * (q.x - e) - b * (q.y - f)) / det), r4((-c * (q.x - e) + a * (q.y - f)) / det)]);
}

const pathOf = (rings: [number, number][][], closed = true): string => rings.map((r) => `M ${r.map(([x, y]) => `${x} ${y}`).join(" L ")}${closed ? " Z" : ""}`).join(" ");

/** 🧱️ Repairs the offences of one cluster inside a window of the guard's raster; returns the ops. */
function repair(sc: Scene, group: Offence[], touched: Map<string, any>): Op[] {
  const { cell, feature, frame, view, scale } = sc;
  const pad = 10 * feature;
  const gi = (x: number, lo: number): number => Math.floor((x - lo) / cell);
  const box = { minX: frame.minX + gi(Math.min(...group.map((o) => o.x)) - pad, frame.minX) * cell, minY: frame.minY + gi(Math.min(...group.map((o) => o.y)) - pad, frame.minY) * cell, maxX: frame.minX + (gi(Math.max(...group.map((o) => o.x)) + pad, frame.minX) + 1) * cell, maxY: frame.minY + (gi(Math.max(...group.map((o) => o.y)) + pad, frame.minY) + 1) * cell };
  const touches = (q: readonly [number, number, number, number], m = 0.02): boolean => q[1] + m >= box.minX && q[0] - m <= box.maxX && q[3] + m >= box.minY && q[2] - m <= box.maxY;
  const t = { core: box, box, reference: sc.reference.filter((g) => touches(boundsOf(g))), evaluated: sc.evaluated.filter((s: any) => touches(boundsOf(s.rings))) };
  const { drawn, vol, width, height } = tileMasks(sc, t);
  const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
  const [fd, fv] = [fillSmallHoles(drawn, width, height, holes), fillSmallHoles(vol, width, height, holes)];
  const vonly = fv.map((x, k) => (x && !fd[k] ? 1 : 0)), donly = fd.map((x, k) => (x && !fv[k] ? 1 : 0));
  const [lv, ld] = [label(vonly, width, height), label(donly, width, height)];
  const patch = new Set<number>(), trim = new Set<number>(), holes2 = new Set<number>();
  const lh = label(drawn.map((x) => (x ? 0 : 1)), width, height);
  const around = (k: number): number[] => [k - 1, k + 1, k - width, k + width, k - width - 1, k - width + 1, k + width - 1, k + width + 1].filter((n) => n >= 0 && n < fd.length);
  for (const o of group) {
    const [i, j] = [Math.floor((o.x - box.minX) / cell), Math.floor((o.y - box.minY) / cell)];
    if (i < 0 || j < 0 || i >= width || j >= height) continue;
    const k = j * width + i;
    if (lv.labels[k]) patch.add(lv.labels[k]!);
    else if (ld.labels[k]) trim.add(ld.labels[k]!);
    else {
      const before = patch.size + trim.size;
      for (const n of around(k)) { if (o.side === "drawn" && lv.labels[n]) patch.add(lv.labels[n]!); if (o.side === "volume" && ld.labels[n]) trim.add(ld.labels[n]!); }
      if (o.side === "drawn" && patch.size + trim.size === before) for (const n of around(k)) if (lh.labels[n]) holes2.add(lh.labels[n]!);
    }
  }
  if (process.env.SV1_DEBUG) console.log(`[DEBUG] group of ${group.length}: ${patch.size} patch components, ${trim.size} trim components, window ${width}x${height}, flags ${group.slice(0, 3).map((o) => `${o.side}/${o.inDrawn ? 1 : 0}${o.inVolume ? 1 : 0}`).join(" ")}`);
  const world = (ring: P[]): P[] => ring.map((p) => ({ x: box.minX + p.x * cell, y: box.minY + p.y * cell }));
  const centre = (k: number): P => ({ x: box.minX + ((k % width) + 0.5) * cell, y: box.minY + (Math.floor(k / width) + 0.5) * cell });
  const cellsOf = (labels: Int32Array, l: number): number[] => { const out: number[] = []; for (let k = 0; k < labels.length; k++) if (labels[k] === l) out.push(k); return out; };
  const sample = (cells: number[], n: number): number[] => (cells.length <= n ? cells : Array.from({ length: n }, (_, i) => cells[Math.floor((i * cells.length) / n)]!));
  const out: Op[] = [];
  const list = asset.representations[String(scale)].geometry[view] as any[];
  const reach = Math.ceil(feature / cell / 2) + 1;
  const inked = fd.map((x, k) => (x || patch.has(lv.labels[k]!) || holes2.has(lh.labels[k]!) ? 1 : 0));
  const thin = erode(dilate(inked, width, height, reach), width, height, reach).map((x, k) => (x && !inked[k] ? 1 : 0));
  const keepOff = values.close ? new Uint8Array(fd.length) : dilate(thin, width, height, reach);
  const pockets: number[][] = [...patch].map((l) => cellsOf(lv.labels, l));
  for (const l of holes2) {
    const cells = cellsOf(lh.labels, l);
    const edge = cells.some((k) => k % width === 0 || k % width === width - 1 || k < width || k >= fd.length - width);
    if (!edge && cells.filter((k) => fv[k]).length >= 0.9 * cells.length) pockets.push(cells.filter((k) => fv[k]));
  }
  for (const cells of pockets) {
    const own = new Uint8Array(fd.length);
    for (const k of cells) own[k] = 1;
    const grown = dilate(own, width, height, 2).map((x, k) => (x && fv[k] && !keepOff[k] ? 1 : 0));
    const rings = outline(grown, width, height).map((r) => simplify(r, 0.75)).filter((r) => Math.abs(area(r)) >= 1).map(world);
    if (rings.length === 0) continue;
    const probes = sample(cells, 40).map(centre);
    const votes = new Map<string, number>();
    for (const w of sc.solids as any[]) {
      const pts: P[] = [];
      for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
      const [x0, x1, y0, y1] = [Math.min(...pts.map((p) => p.x)), Math.max(...pts.map((p) => p.x)), Math.min(...pts.map((p) => p.y)), Math.max(...pts.map((p) => p.y))];
      let hits = 0;
      for (const p of probes) {
        if (p.x < x0 || p.x > x1 || p.y < y0 || p.y > y1) continue;
        for (let q = 0; q < w.indices.length; q += 3) if (inside(p, [[pts[w.indices[q]]!, pts[w.indices[q + 1]]!, pts[w.indices[q + 2]]!]])) { hits++; break; }
      }
      if (hits > 0) votes.set(w.bone, (votes.get(w.bone) ?? 0) + hits);
    }
    const mid = centre(cells[Math.floor(cells.length / 2)]!);
    const filledShapes = sc.evaluated.filter((s: any) => s.closed && s.shape.filled !== false && s.shape.role === "body" && s.shape.kind !== "member");
    const ranked = filledShapes.map((s: any) => ({ s, d: edgeDistance(mid, s.rings, true) })).sort((p, q) => p.d - q.d);
    const bone = [...votes].sort((p, q) => q[1] - p[1])[0]?.[0] ?? ranked[0]?.s.shape.bone;
    const template = (ranked.find((r) => r.s.shape.bone === bone && r.d < 20 * feature) ?? ranked[0])?.s.shape;
    if (template === undefined || bone === undefined) { console.log(`   no template for a patch at (${mid.x.toFixed(3)}, ${mid.y.toFixed(3)})`); continue; }
    let sid = "";
    do sid = `sv1-${scale}-${view[0]}-${++serial}`; while (list.some((x) => x.id === sid));
    const flat = toLocal(scale, view, bone, template, sid, rings.flat());
    let n = 0;
    const local = rings.map((r) => flat.slice(n, (n += r.length)).filter((q) => q !== undefined) as [number, number][]).filter((r) => r.length >= 3);
    if (local.length === 0) continue;
    const shape = { ...Object.fromEntries(BASE.filter((k) => template[k] !== undefined).map((k) => [k, template[k]])), id: sid, bone, filled: true, kind: "path", d: pathOf(local) };
    out.push({ op: "shape-add", scale: String(scale), view, after: template.id, shape });
    console.log(`   + ${sid}@${bone} like ${template.id}: ${cells.length} cells (${(cells.length * cell * cell * 1e4).toFixed(1)} cm2) at (${mid.x.toFixed(3)}, ${mid.y.toFixed(3)}), ${rings.length} rings, ${flat.length} points`);
  }
  for (const l of trim) {
    const cells = cellsOf(ld.labels, l);
    const own = new Uint8Array(fd.length);
    for (const k of cells) own[k] = 1;
    const grown = dilate(own, width, height, 1).map((x, k) => (x && !fv[k] ? 1 : 0));
    const cutRings = outline(grown, width, height).map((r) => simplify(r, 0.5)).map(world);
    const cut = pc.xor(...(cutRings.map((r) => [[r.map((p) => [p.x, p.y] as [number, number])]]) as [any, ...any[]]));
    const probes = sample(cells, 300).map(centre);
    const [x0, x1, y0, y1] = [Math.min(...probes.map((p) => p.x)), Math.max(...probes.map((p) => p.x)), Math.min(...probes.map((p) => p.y)), Math.max(...probes.map((p) => p.y))];
    const mid = probes[Math.floor(probes.length / 2)]!;
    for (const s of t.evaluated as any[]) {
      const q = boundsOf(s.rings);
      if (q[1] < x0 - 0.02 || q[0] > x1 + 0.02 || q[3] < y0 - 0.02 || q[2] > y1 + 0.02) continue;
      const fills = s.closed && (s.shape.filled !== false || bodyOutline(view as never, sc.solids as never, s.rings));
      const index = list.findIndex((x) => x.id === s.shape.id);
      if (index < 0) continue;
      if (fills) {
        if (!probes.some((p) => inside(p, s.rings))) continue;
        if (touched.get(s.shape.id) === null) continue;
        const whole = touched.get(s.shape.id) ?? pc.xor(...(s.rings.map((r: P[]) => [[r.map((p) => [p.x, p.y] as [number, number])]]) as [any, ...any[]]));
        const kept = pc.difference(whole, cut).filter((poly: any[]) => Math.abs(area(poly[0]!.map(([x, y]: number[]) => ({ x: x!, y: y! })))) > 4 * cell * cell);
        const rest = kept.map((poly: any[]) => poly.map((ring: any[]) => ring.slice(0, -1)));
        touched.set(s.shape.id, kept.length === 0 ? null : kept);
        if (rest.length === 0) { out.push({ op: "shape-remove", scale: String(scale), view, id: s.shape.id }); console.log(`   - ${s.shape.id}@${s.shape.bone} removed: it lies outside the volume at (${mid.x.toFixed(3)}, ${mid.y.toFixed(3)})`); continue; }
        const rings = rest.flat();
        const targets = rings.flat().map(([x, y]) => ({ x, y }));
        const profile = list[index].solid === undefined ? undefined : solidLocal(list[index], s.rings, targets);
        const keepSolid = list[index].solid !== undefined && profile !== undefined;
        if (list[index].solid !== undefined && profile === undefined) { console.log(`   ? ${s.shape.id}@${s.shape.bone}: a solid shape whose outline in the ${view} is not its profile; it is not cut at (${mid.x.toFixed(3)}, ${mid.y.toFixed(3)})`); touched.set(s.shape.id, null); continue; }
        const flat: ([number, number] | undefined)[] = profile ?? toLocal(scale, view, s.shape.bone, list[index], s.shape.id, targets);
        let n = 0;
        const local = rings.map((r) => flat.slice(n, (n += r.length)).filter((q) => q !== undefined) as [number, number][]).filter((r) => r.length >= 3);
        if (local.length === 0) continue;
        const shape = { ...Object.fromEntries(BASE.filter((k) => list[index][k] !== undefined).map((k) => [k, list[index][k]])), kind: "path", d: pathOf(local), ...(keepSolid ? { solid: list[index].solid } : {}) };
        out.push({ op: "shape", scale: String(scale), view, id: s.shape.id, shape });
        console.log(`   ~ ${s.shape.id}@${s.shape.bone} (${list[index].kind}${list[index].solid ? (keepSolid ? ", solid kept" : ", solid given up") : ""}) cut back by ${cells.length} cells (${(cells.length * cell * cell * 1e4).toFixed(1)} cm2) at (${mid.x.toFixed(3)}, ${mid.y.toFixed(3)}): ${rings.length} rings, ${flat.length} points`);
      } else {
        const lines: P[][] = s.rings.map((r: P[]) => (s.closed ? [...r, r[0]!] : r));
        const keep: P[][] = [];
        let [gone, total] = [0, 0];
        const bad = (p: P): boolean => { const [i, j] = [Math.floor((p.x - box.minX) / cell), Math.floor((p.y - box.minY) / cell)]; return i >= 0 && j >= 0 && i < width && j < height && own[j * width + i] === 1; };
        for (const line of lines) {
          let run: P[] = [];
          for (let i = 0; i + 1 < line.length; i++) {
            const [p, q2] = [line[i]!, line[i + 1]!];
            const steps = Math.max(1, Math.ceil(Math.hypot(q2.x - p.x, q2.y - p.y) / cell));
            for (let k = 0; k <= steps; k++) {
              const at = { x: p.x + ((q2.x - p.x) * k) / steps, y: p.y + ((q2.y - p.y) * k) / steps };
              total++;
              if (bad(at)) { gone++; if (run.length > 1) keep.push(run); run = []; }
              else if (k === 0 || k === steps || run.length === 0) run.push(at);
            }
          }
          if (run.length > 1) keep.push(run);
        }
        if (gone === 0 || touched.has(s.shape.id)) continue;
        touched.set(s.shape.id, null);
        if (keep.length === 0 || gone > 0.5 * total) { out.push({ op: "shape-remove", scale: String(scale), view, id: s.shape.id }); console.log(`   - stroke ${s.shape.id}@${s.shape.bone} removed: ${gone} of ${total} samples outside the volume`); continue; }
        const flat = toLocal(scale, view, s.shape.bone, list[index], s.shape.id, keep.flat());
        let n = 0;
        const local = keep.map((r) => flat.slice(n, (n += r.length)).filter((q) => q !== undefined) as [number, number][]).filter((r) => r.length >= 2);
        if (local.length === 0) continue;
        out.push({ op: "shape", scale: String(scale), view, id: s.shape.id, shape: { ...Object.fromEntries(BASE.filter((k) => list[index][k] !== undefined).map((k) => [k, list[index][k]])), kind: "path", d: pathOf(local, false) } });
        console.log(`   ~ stroke ${s.shape.id}@${s.shape.bone} clipped: ${gone} of ${total} samples outside the volume`);
      }
    }
  }
  return out;
}

for (const spec of values.join ?? []) {
  const [sd, view, ida, idb] = spec.split(":") as [string, string, string, string];
  const sc = scene(asset, Number(sd), view);
  const [ea, eb] = [ida, idb].map((x) => sc.evaluated.find((s: any) => s.shape.id === x));
  if (ea === undefined || eb === undefined) { console.log(`join ${spec}: a shape is missing (already joined)`); continue; }
  const reach = Math.ceil((0.5e-3 * sc.scale) / sc.cell);
  const all: P[] = [...ea.rings.flat(), ...eb.rings.flat()];
  const pad = (reach + 4) * sc.cell;
  const box = { minX: Math.min(...all.map((q) => q.x)) - pad, minY: Math.min(...all.map((q) => q.y)) - pad, maxX: Math.max(...all.map((q) => q.x)) + pad, maxY: Math.max(...all.map((q) => q.y)) + pad };
  const mask = new Mask(box, sc.cell);
  mask.fill(ea.rings);
  mask.fill(eb.rings);
  const closed = erode(dilate(mask.cells, mask.width, mask.height, reach), mask.width, mask.height, reach).map((x, k) => (x || mask.cells[k] ? 1 : 0));
  const rings = outline(closed, mask.width, mask.height).map((r) => simplify(r, 0.75)).filter((r) => Math.abs(area(r)) >= 1).map((r) => r.map((q) => ({ x: box.minX + q.x * sc.cell, y: box.minY + q.y * sc.cell })));
  const list = asset.representations[sd].geometry[view] as any[];
  const template = list.find((x) => x.id === ida);
  const flat = toLocal(sc.scale, view, template.bone, template, ida, rings.flat());
  let n = 0;
  const local = rings.map((r) => flat.slice(n, (n += r.length)).filter((q) => q !== undefined) as [number, number][]).filter((r) => r.length >= 3);
  const added: Op[] = [{ op: "shape", scale: sd, view, id: ida, shape: { ...Object.fromEntries(BASE.filter((k) => template[k] !== undefined).map((k) => [k, template[k]])), kind: "path", d: pathOf(local) } }, { op: "shape-remove", scale: sd, view, id: idb }];
  applyOps(asset, added);
  ops = mergeOps(ops, added);
  asset = structuredClone(asset);
  console.log(`join ${spec}: ${ida} now holds ${idb} (${rings.length} rings, ${flat.length} points)`);
}
const scales = (values.scales ?? "50,100,200,500").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored");
const rounds = Number(values.rounds ?? 6);
for (const scale of scales) for (const view of (values.views ?? VIEWS.join(",")).split(",")) {
  for (let round = 0; round <= rounds; round++) {
    const sc = scene(asset, scale, view);
    const limit = Number(values.limit ?? 2) * sc.feature - Number(values.slack ?? 1) * sc.cell;
    const all: Offence[] = [];
    let worst = 0;
    for (const t of tiles(sc)) {
      const { m, drawn, vol, width, height } = tileMasks(sc, t);
      worst = Math.max(worst, m.hausdorffM);
      if (m.hausdorffM > limit) all.push(...offences(sc, t, drawn, vol, width, height, limit));
    }
    const groups = clusters(all, 6 * sc.feature);
    console.log(`${id} 1:${scale} ${view} round ${round}: largest distance ${(worst * 1000).toFixed(1)} mm against ${(2 * sc.feature * 1000).toFixed(1)} mm, ${all.length} offending cells in ${groups.length} clusters`);
    if (groups.length === 0 || round === rounds) break;
    const touched = new Map<string, any>();
    const added: Op[] = [];
    for (const g of groups) added.push(...repair(sc, g, touched));
    if (added.length === 0) { console.log(`   nothing to repair automatically`); break; }
    applyOps(asset, added);
    ops = mergeOps(ops, added);
    asset = structuredClone(asset);
  }
}
const parentOf = new Map<string, string | null>((asset.rig.bones as any[]).map((bone) => [bone.id, bone.parent ?? null]));
const VIEW_OF: Record<string, string> = { e: "elevation", f: "front", p: "plan" };
for (let depth = 0; depth < 10; depth++) {
  const fresh = (validateAsset(structuredClone(asset) as never) as any[]).filter((i) => i.code === "lateral-ambiguous" && !inherited.has(issueKey(i)));
  const moves = fresh.flatMap((i) => (i.data?.shapes ?? []).filter((x: string) => x.startsWith("sv1-")).map((x: string) => ({ sid: x, bone: i.data.bone as string })));
  if (moves.length === 0) break;
  const added: Op[] = [];
  const scenes = new Map<string, Scene>();
  for (const { sid, bone } of moves) {
    const [, sd, letter] = sid.split("-") as [string, string, string];
    const view = VIEW_OF[letter!]!;
    const list = asset.representations[sd!].geometry[view] as any[];
    const shape = list.find((x) => x.id === sid);
    const parent = parentOf.get(shape?.bone ?? bone);
    if (shape === undefined || parent === null || parent === undefined) { console.log(`   ? ${sid}@${bone}: flat and ambiguous, and no parent bone to carry it`); continue; }
    const k = `${sd}:${view}`;
    if (!scenes.has(k)) scenes.set(k, scene(asset, Number(sd), view));
    const rings = scenes.get(k)!.evaluated.find((x: any) => x.shape.id === sid).rings as P[][];
    const flat = toLocal(Number(sd), view, parent, shape, sid, rings.flat());
    let n = 0;
    const local = rings.map((r) => flat.slice(n, (n += r.length)).filter((q) => q !== undefined) as [number, number][]).filter((r) => r.length >= 3);
    if (local.length === 0) continue;
    added.push({ op: "shape", scale: sd, view, id: sid, shape: { ...Object.fromEntries(BASE.filter((key) => shape[key] !== undefined).map((key) => [key, shape[key]])), bone: parent, kind: "path", d: pathOf(local) } });
    console.log(`   ^ ${sid}: from ${shape.bone} to its parent ${parent} (a flat shape on ${shape.bone} is ambiguous in a pose)`);
  }
  if (added.length === 0) break;
  applyOps(asset, added);
  ops = mergeOps(ops, added);
  asset = structuredClone(asset);
}
if (!values.dry) await writeOps(opsFile(id), ops);
console.log(`${id}: ${ops.length} ops${values.dry ? " (dry)" : ` in ${opsFile(id)}`}`);
