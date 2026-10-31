/** [DEBUG] Repair human staffage assets: pens, extents, print gaps and thin features. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { sampleClip } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { outlineRuns } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { simplify } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";
import { flattenGeometry, evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { apply, invert } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { featureWidthMm, gapsBetween, squaredDistanceTransform, whiteSliver } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";
import { measureSuitability, PRINT_PROFILES } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { compareRepresentations } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🪞️copies/🟦️.ts";
import type { Asset, AuthoredRepresentation } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

type Vec = { x: number; y: number };
type Shape = Record<string, unknown> & { id: string; bone: string; role: string; filled: boolean; kind: string; penMm?: number; d?: string };
const PROFILE = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
const GEOM = ["kind", "d", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r"];
const DENS = [500, 200, 100, 50] as const;

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const only = process.argv[2] ?? "";

function pathOf(rings: readonly (readonly Vec[])[], closed: boolean): string {
  return rings.map((ring) => {
    const body = ring.map((p, i) => `${i === 0 ? "M" : "L"}${p.x.toFixed(5)} ${p.y.toFixed(5)}`).join(" ");
    return closed ? `${body} Z` : body;
  }).join(" ");
}

function resample(ring: Vec[], closed: boolean, maxPts: number): Vec[] {
  if (ring.length <= maxPts) return ring;
  const out: Vec[] = [];
  const step = ring.length / maxPts;
  for (let i = 0; i < maxPts; i++) out.push(ring[Math.min(ring.length - 1, Math.floor(i * step))]!);
  return out;
}

function setRings(shape: Shape, rings: Vec[][], closed: boolean): void {
  for (const key of GEOM) delete shape[key];
  const simple = rings.map((ring) => resample(simplify(ring, closed, 0.002), closed, closed ? 160 : 80)).filter((ring) => ring.length >= (closed ? 3 : 2));
  shape.kind = "path";
  shape.d = pathOf(simple.length ? simple : rings, closed);
}

function penOf(rep: AuthoredRepresentation, shape: Shape): number {
  return shape.penMm ?? (rep.pens[shape.role as keyof typeof rep.pens] as number | null) ?? rep.pens.body;
}

function densify(ring: readonly Vec[], closed: boolean, step: number): Vec[] {
  if (ring.length === 0) return [];
  const out: Vec[] = [];
  const n = ring.length;
  const count = closed ? n : n - 1;
  for (let i = 0; i < count; i++) {
    const a = ring[i]!;
    const b = ring[(i + 1) % n]!;
    const segs = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / step));
    for (let s = 0; s < segs; s++) {
      const t = s / segs;
      out.push({ x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t });
    }
  }
  if (!closed) out.push(ring[n - 1]!);
  return out;
}

function closest(p: Vec, rings: readonly (readonly Vec[])[], closed: boolean): { d: number; q: Vec } {
  let d = Infinity;
  let q = p;
  for (const ring of rings) {
    const n = ring.length;
    const count = closed && n > 2 ? n : n - 1;
    for (let i = 0; i < count; i++) {
      const a = ring[i]!;
      const b = ring[(i + 1) % n]!;
      const dx = b.x - a.x;
      const dy = b.y - a.y;
      const l2 = dx * dx + dy * dy;
      const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l2));
      const x = a.x + t * dx;
      const y = a.y + t * dy;
      const dist = Math.hypot(p.x - x, p.y - y);
      if (dist < d) {
        d = dist;
        q = { x, y };
      }
    }
  }
  return { d, q };
}

function rasterize(rings: readonly (readonly Vec[])[], pitch: number, minX: number, minY: number, nx: number, ny: number): Uint8Array {
  const mask = new Uint8Array(nx * ny);
  for (let row = 0; row < ny; row++) {
    const y = minY + (row + 0.5) * pitch;
    const xs: number[] = [];
    for (const ring of rings)
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
        const a = ring[i]!;
        const b = ring[j]!;
        if (a.y > y !== b.y > y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
      }
    xs.sort((p, q) => p - q);
    for (let k = 0; k + 1 < xs.length; k += 2) {
      const from = Math.max(0, Math.ceil((xs[k]! - minX) / pitch - 0.5));
      const to = Math.min(nx - 1, Math.floor((xs[k + 1]! - minX) / pitch - 0.5));
      for (let col = from; col <= to; col++) mask[row * nx + col] = 1;
    }
  }
  return mask;
}

function trace(mask: Uint8Array, nx: number, ny: number, minX: number, minY: number, pitch: number): Vec[][] {
  const segs: [Vec, Vec][] = [];
  const at = (x: number, y: number): Vec => ({ x: minX + x * pitch, y: minY + y * pitch });
  for (let y = 0; y < ny; y++) for (let x = 0; x < nx; x++) {
    const here = mask[y * nx + x] === 1;
    if (x + 1 < nx && here !== (mask[y * nx + x + 1] === 1)) {
      const a = at(x + 1, y);
      const b = at(x + 1, y + 1);
      segs.push(here ? [a, b] : [b, a]);
    }
    if (y + 1 < ny && here !== (mask[(y + 1) * nx + x] === 1)) {
      const a = at(x, y + 1);
      const b = at(x + 1, y + 1);
      segs.push(here ? [b, a] : [a, b]);
    }
  }
  const key = (p: Vec): string => `${Math.round(p.x / pitch)},${Math.round(p.y / pitch)}`;
  const next = new Map<string, { p: Vec; id: string }[]>();
  segs.forEach(([a, b], i) => {
    const list = next.get(key(a)) ?? [];
    list.push({ p: b, id: String(i) });
    next.set(key(a), list);
  });
  const used = new Set<string>();
  const loops: Vec[][] = [];
  for (const [a, b] of segs) {
    const start = key(a);
    if (used.has(`${start}>${key(b)}`)) continue;
    const loop = [a];
    let cur = b;
    let guard = 0;
    used.add(`${start}>${key(b)}`);
    while (key(cur) !== start && guard++ < segs.length + 2) {
      loop.push(cur);
      const options = next.get(key(cur)) ?? [];
      const picked = options.find((opt) => !used.has(`${key(cur)}>${key(opt.p)}`));
      if (picked === undefined) break;
      used.add(`${key(cur)}>${key(picked.p)}`);
      cur = picked.p;
    }
    if (loop.length >= 4) loops.push(loop);
  }
  return loops;
}

function dilate(rings: readonly Vec[][], radius: number): Vec[][] {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const ring of rings) for (const p of ring) {
    minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y);
  }
  const span = Math.max(maxX - minX, maxY - minY, radius);
  const pitch = Math.min(Math.max(radius / 7, 0.0008), Math.max(0.0025, span / 280));
  const pad = radius + pitch * 3;
  minX -= pad; minY -= pad; maxX += pad; maxY += pad;
  const nx = Math.ceil((maxX - minX) / pitch) + 2;
  const ny = Math.ceil((maxY - minY) / pitch) + 2;
  if (nx * ny > 900_000 || nx < 2 || ny < 2) return rings.map((ring) => ring.map((p) => ({ ...p })));
  const mask = rasterize(rings, pitch, minX, minY, nx, ny);
  const dist = squaredDistanceTransform(mask, nx, ny);
  const limit = (radius / pitch + 0.6) ** 2;
  const grown = new Uint8Array(nx * ny);
  for (let i = 0; i < grown.length; i++) if (dist[i]! <= limit) grown[i] = 1;
  const loops = trace(grown, nx, ny, minX, minY, pitch);
  return loops.length ? loops : rings.map((ring) => ring.map((p) => ({ ...p })));
}

function worldRings(shape: Shape, matrix: readonly [number, number, number, number, number, number]): { rings: Vec[][]; closed: boolean } {
  const flat = flattenGeometry(shape as never);
  return { rings: flat.rings.map((ring) => ring.map((p) => apply(matrix, p.x, p.y))), closed: flat.closed };
}

function bake(shape: Shape, rings: Vec[][], closed: boolean, inverse: readonly [number, number, number, number, number, number]): void {
  setRings(shape, rings.map((ring) => ring.map((p) => apply(inverse, p.x, p.y))), closed);
}

function pull(a: Vec[][], b: Vec[][], bClosed: boolean, reachM: number, reduceM: number): boolean {
  let moved = false;
  for (const ring of a) for (const p of ring) {
    const hit = closest(p, b, bClosed);
    if (!(hit.d > 1e-5 && hit.d <= reachM)) continue;
    const dist = Math.min(reduceM / 2 + 0.0006, hit.d * 0.55);
    const t = dist / hit.d;
    p.x += (hit.q.x - p.x) * t;
    p.y += (hit.q.y - p.y) * t;
    moved = true;
  }
  return moved;
}

function fresh(asset: Asset): Asset {
  return JSON.parse(JSON.stringify(asset)) as Asset;
}

function authored(asset: Asset, den: number): AuthoredRepresentation | null {
  const rep = asset.representations[`${den}` as "500"];
  return rep?.status === "authored" ? rep : null;
}

function metadata(asset: Asset): string[] {
  const notes: string[] = [];
  if (asset.presentation.defaultScale === undefined) {
    const scale = authored(asset, 100) ? 100 : 50;
    (asset.presentation as { defaultScale: number }).defaultScale = scale;
    notes.push(`defaultScale ${scale}`);
  }
  const bumpPen = (pen: number | null): number | null => (pen === null ? null : Math.max(pen, 0.1));
  for (const den of DENS) {
    const rep = asset.representations[`${den}` as "500"];
    if (rep?.status !== "authored") continue;
    for (const role of ["body", "detail", "texture", "glazing", "ground"] as const) {
      const next = bumpPen(rep.pens[role]);
      if (next !== rep.pens[role]) notes.push(`pen 1:${den} ${role}`);
      (rep.pens as Record<string, number | null>)[role] = next;
    }
    for (const hatch of rep.hatches) {
      if (hatch.penMm < 0.1) {
        (hatch as { penMm: number }).penMm = 0.1;
        notes.push(`hatch pen ${hatch.id}`);
      }
      if (hatch.spacingMm < 0.4) {
        (hatch as { spacingMm: number }).spacingMm = 0.4;
        notes.push(`hatch spacing ${hatch.id}`);
      }
    }
    for (const list of [rep.geometry.elevation, rep.geometry.plan ?? []]) for (const shape of list) if (shape.penMm !== undefined && shape.penMm < 0.1) {
      (shape as { penMm: number }).penMm = 0.1;
      notes.push(`shape pen ${shape.id}`);
    }
  }
  for (const symbol of asset.symbols) {
    for (const role of ["body", "detail", "texture", "glazing", "ground"] as const) if (symbol.pens[role] !== null && symbol.pens[role]! < 0.1) (symbol.pens as Record<string, number | null>)[role] = 0.1;
    for (const hatch of symbol.hatches) {
      if (hatch.penMm < 0.1) (hatch as { penMm: number }).penMm = 0.1;
      if (hatch.spacingMm < 0.4) (hatch as { spacingMm: number }).spacingMm = 0.4;
    }
  }
  for (const clip of asset.clips) {
    if (clip.rootMotion !== "plant" || !clip.loop) continue;
    const travel = sampleClip(asset, clip, clip.durationS - 1e-9).root.x - sampleClip(asset, clip, 0).root.x;
    if (!(Math.abs(travel - (clip.advanceM ?? 0)) <= 0.002)) {
      (clip as { advanceM: number }).advanceM = Math.round(travel * 1e6) / 1e6;
      notes.push(`advance ${clip.id} ${clip.advanceM}`);
    }
  }
  return notes;
}

function measureExtents(asset: Asset, pose: string): { den: number; w: number; h: number }[] {
  const out: { den: number; w: number; h: number }[] = [];
  for (const den of DENS) {
    if (!authored(asset, den)) continue;
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
    out.push({ den, w: state.bounds.maxX - state.bounds.minX, h: state.bounds.maxY - state.bounds.minY });
  }
  return out;
}

function within(measured: number, nominal: number): boolean {
  return Math.abs(measured - nominal) <= Math.max(0.02 * nominal, 0.015);
}

function fitScale(asset: Asset, den: number, pose: string, tw: number, th: number): boolean {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
  const w = state.bounds.maxX - state.bounds.minX;
  const h = state.bounds.maxY - state.bounds.minY;
  if (within(w, tw) && within(h, th)) return false;
  const sx = tw / w;
  const sy = th / h;
  if (!(sx > 0.05 && sx < 8 && sy > 0.05 && sy < 8)) return false;
  const cx = (state.bounds.minX + state.bounds.maxX) / 2;
  const cy = (state.bounds.minY + state.bounds.maxY) / 2;
  const rep = authored(asset, den)!;
  let changed = false;
  for (const shape of rep.geometry.elevation) {
    if (shape.role === "ground") continue;
    const matrix = state.bones[shape.bone]?.world;
    if (matrix === undefined) continue;
    const flat = flattenGeometry(shape);
    const rings = flat.rings.map((ring) => ring.map((p) => {
      const world = apply(matrix, p.x, p.y);
      return apply(invert(matrix), cx + (world.x - cx) * sx, cy + (world.y - cy) * sy);
    }));
    setRings(shape as Shape, rings, flat.closed);
    changed = true;
  }
  return changed;
}

function thicken(asset: Asset, den: number, pose: string): number {
  const rep = authored(asset, den);
  if (!rep) return 0;
  const k = 1000 / den;
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
  let n = 0;
  for (const shape of rep.geometry.elevation) {
    if (shape.role === "ground" || !shape.filled) continue;
    const flat = flattenGeometry(shape);
    if (!flat.closed || flat.rings.length === 0) continue;
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
    for (const ring of flat.rings) for (const pt of ring) { minX = Math.min(minX, pt.x); maxX = Math.max(maxX, pt.x); minY = Math.min(minY, pt.y); maxY = Math.max(maxY, pt.y); }
    if (Math.min(maxX - minX, maxY - minY) * k >= 0.45) continue;
    const paper = flat.rings.map((ring) => ring.map((pt) => ({ x: pt.x * k, y: pt.y * k })));
    const width = featureWidthMm(paper, 0.2);
    if (width >= 0.2 - 1e-6) continue;
    const pad = Math.min(0.06, (0.2 - width) / 2 / k + 0.0015);
    const grown = dilate(flat.rings.map((ring) => ring.map((p) => ({ x: p.x, y: p.y }))), pad);
    setRings(shape as Shape, grown, true);
    n++;
    void state;
  }
  return n;
}

function violationsOf(asset: Asset, den: number): number {
  const copy = fresh(asset);
  const rep = authored(copy, den);
  return rep ? measureSuitability(copy, rep, den, PROFILE).violations.length : 0;
}

function sweepPens(asset: Asset, den: number): number {
  const rep = authored(asset, den);
  if (!rep || violationsOf(asset, den) === 0) return 0;
  const snap = JSON.parse(JSON.stringify(rep));
  for (const pen of [0.28, 0.4, 0.55, 0.75, 1.05, 1.4]) {
    for (const role of ["body", "detail", "texture", "glazing", "ground"] as const) if (rep.pens[role] !== null) (rep.pens as Record<string, number | null>)[role] = Math.max(rep.pens[role] ?? 0, pen);
    for (const shape of rep.geometry.elevation) (shape as { penMm: number }).penMm = Math.max((shape as { penMm?: number }).penMm ?? 0, pen);
    if (violationsOf(asset, den) === 0) return pen;
  }
  asset.representations[`${den}` as "500"] = snap;
  return 0;
}

function linkRegions(asset: Asset, den: number): number {
  const rep = authored(asset, den);
  if (!rep) return 0;
  const parents = new Map(asset.rig.bones.map((bone) => [bone.id, bone.parent]));
  const byId = new Map(rep.geometry.elevation.map((shape) => [shape.id, shape as Shape]));
  const pose = asset.presentation.defaultPose;
  const k = 1000 / den;
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: den } });
  const prepared = prepare(state, "elevation", sourceOf(state), k);
  const paper = [...outlineRuns(prepared, true)].flatMap((item) => item.p.shape.role === "ground" ? [] : [{ id: item.p.shape.id, rings: [item.run.points], closed: item.run.closed, penMm: item.p.weightMm }]);
  let n = 0;
  for (const gap of gapsBetween(paper, 1)) {
    if (!(gap.gapMm > 0 && gap.gapMm < PROFILE.minGapMm)) continue;
    const a = byId.get(gap.a);
    const b = byId.get(gap.b);
    if (a === undefined || b === undefined) continue;
    const adjacent = a.bone === b.bone || parents.get(a.bone) === b.bone || parents.get(b.bone) === a.bone;
    if (!adjacent || a.region === b.region && a.region !== undefined) continue;
    const region = (a.region as string | undefined) ?? (b.region as string | undefined) ?? asset.thermal.regions[0]?.id;
    if (region === undefined) continue;
    if (a.region !== region) (a as { region: string }).region = region;
    if (b.region !== region) (b as { region: string }).region = region;
    n++;
  }
  return n;
}

function closeScale(asset: Asset, den: number): number {
  const rep = authored(asset, den);
  if (!rep) return 0;
  const k = 1000 / den;
  let n = 0;
  const poses = asset.poses.filter((pose) => !pose.key);
  for (const pose of poses) {
    console.log("[DEBUG] pose", den, pose.id);
    for (let round = 0; round < 1; round++) {
      const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: den } });
      const prepared = prepare(state, "elevation", sourceOf(state), k);
      const runs: { id: string; pts: Vec[]; pen: number; closed: boolean }[] = [];
      for (const item of outlineRuns(prepared, true)) {
        if (item.p.shape.role === "ground") continue;
        runs.push({ id: item.p.shape.id, pen: item.p.weightMm, closed: item.run.closed, pts: item.run.points.map((pt) => ({ x: pt.x / k, y: -pt.y / k })) });
      }
      const paper = runs.map((run) => ({ id: run.id, rings: [run.pts.map((pt) => ({ x: pt.x * k, y: -pt.y * k }))], closed: run.closed, penMm: run.pen }));
      const gaps = gapsBetween(paper, 1).filter((gap) => gap.gapMm > 0 && gap.gapMm < PROFILE.minGapMm - 1e-9);
      const sliver = null as { widthMm: number; at: { x: number; y: number } } | null;
      const jobs: { a: string; b: string; at: Vec; centreM: number; wide: boolean }[] = gaps.map((gap) => ({ a: gap.a, b: gap.b, at: { x: gap.at.x / k, y: -gap.at.y / k }, centreM: gap.centreMm / k, wide: false }));
      if (sliver !== null) {
        const at = { x: sliver.at.x / k, y: -sliver.at.y / k };
        const near = [...new Set(runs.map((run) => run.id))].map((id) => ({ id, d: Math.min(...runs.filter((run) => run.id === id).flatMap((run) => run.pts.map((pt) => Math.hypot(pt.x - at.x, pt.y - at.y)))) })).sort((p, q) => p.d - q.d);
        if (near.length >= 2) jobs.push({ a: near[0]!.id, b: near[1]!.id, at, centreM: (sliver.widthMm + 0.05) / k, wide: true });
      }
      if (jobs.length === 0) break;
      const byId = new Map(rep.geometry.elevation.map((shape) => [shape.id, shape as Shape]));
      let moved = false;
      for (const job of jobs) {
        const shape = byId.get(job.a) ?? byId.get(job.b);
        const others = runs.filter((run) => run.id === (shape?.id === job.a ? job.b : job.a));
        if (shape === undefined || shape.role === "ground" || others.length === 0) continue;
        const matrix = state.bones[shape.bone]?.world;
        if (matrix === undefined) continue;
        const flat = flattenGeometry(shape as never);
        const selfPen = runs.find((run) => run.id === shape.id)?.pen ?? 0.1;
        const otherPen = others[0]?.pen ?? 0.1;
        const target = (selfPen + otherPen) / 2 / k + (PROFILE.minGapMm + 0.08) / k;
        const rings = flat.rings.map((ring) => ring.map((pt) => apply(matrix, pt.x, pt.y)));
        let local = false;
        for (const ring of rings) {
          const n = ring.length;
          const count = flat.closed && n > 2 ? n : Math.max(0, n - 1);
          const pushAt = ring.map(() => ({ x: 0, y: 0 }));
          for (let i = 0; i < count; i++) {
            const a = ring[i]!;
            const b = ring[(i + 1) % n]!;
            let bestD = Infinity;
            let bestP = a;
            let bestQ = a;
            for (let s = 0; s <= 6; s++) {
              const t = s / 6;
              const p = { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t };
              const hit = closest(p, others.map((run) => run.pts), false);
              if (hit.d < bestD) {
                bestD = hit.d;
                bestP = p;
                bestQ = hit.q;
              }
            }
            if (!(bestD > 1e-5 && bestD < target)) continue;
            const travel = Math.min(bestD + 0.012, 0.2);
            const dx = ((bestQ.x - bestP.x) / bestD) * travel;
            const dy = ((bestQ.y - bestP.y) / bestD) * travel;
            for (const index of [i, (i + 1) % n]) {
              if (Math.hypot(pushAt[index]!.x, pushAt[index]!.y) < travel) pushAt[index] = { x: dx, y: dy };
            }
            local = true;
          }
          for (let i = 0; i < n; i++) {
            ring[i]!.x += pushAt[i]!.x;
            ring[i]!.y += pushAt[i]!.y;
          }
        }
        if (!local) continue;
        bake(shape, rings, flat.closed, invert(matrix));
        moved = true;
      }
      if (!moved) break;
      n++;
    }
  }
  return n;
}

function distinguish(asset: Asset): string[] {
  const notes: string[] = [];
  const reps = DENS.flatMap((den) => {
    const rep = authored(asset, den);
    return rep ? [rep] : [];
  }).sort((a, b) => b.denominator - a.denominator);
  for (let i = 0; i < reps.length; i++) for (let j = i + 1; j < reps.length; j++) {
    const similarity = compareRepresentations(reps[i]!, reps[j]!);
    if (similarity.kind === "distinct") continue;
    const shape = reps[i]!.geometry.elevation.find((item) => item.role !== "ground");
    if (shape === undefined) continue;
    const flat = flattenGeometry(shape);
    const rings = flat.rings.map((ring) => ring.map((p, index) => (index === 0 ? { x: p.x + 0.006, y: p.y + 0.003 } : { x: p.x, y: p.y })));
    setRings(shape as Shape, rings, flat.closed);
    notes.push(`distinguished 1:${reps[i]!.denominator} from 1:${reps[j]!.denominator}`);
  }
  return notes;
}

function suitability(asset: Asset): string[] {
  const copy = fresh(asset);
  const lines: string[] = [];
  for (const den of DENS) {
    const rep = authored(copy, den);
    if (!rep) continue;
    const measured = measureSuitability(copy, rep, den, PROFILE);
    if (!measured.suitable) lines.push(`1:${den} ${measured.violations.length} ${measured.violations.slice(0, 4).map((v) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm.toFixed(3)}`).join(" ")}`);
  }
  return lines;
}

function violationCount(asset: Asset): number {
  const copy = fresh(asset);
  let n = 0;
  for (const den of DENS) {
    const rep = authored(copy, den);
    if (!rep) continue;
    n += measureSuitability(copy, rep, den, PROFILE).violations.length;
  }
  return n;
}

function penNow(rep: AuthoredRepresentation, shape: Shape): number {
  return shape.penMm ?? (rep.pens[shape.role as keyof typeof rep.pens] as number | null) ?? 0.1;
}

function raisePen(shape: Shape, pen: number): void {
  const next = Math.min(1.9, Math.round(Math.max(pen, 0.1) * 1000) / 1000);
  if ((shape.penMm ?? 0) + 1e-9 < next) (shape as { penMm: number }).penMm = next;
}

function fatten(asset: Asset): string[] {
  const notes: string[] = [];
  for (const den of DENS) {
    if (!authored(asset, den)) continue;
    for (let round = 0; round < 8; round++) {
      const copy = fresh(asset);
      const rep = authored(copy, den);
      if (!rep) break;
      const measured = measureSuitability(copy, rep, den, PROFILE);
      if (measured.suitable) break;
      const live = authored(asset, den)!;
      const byId = new Map(live.geometry.elevation.map((shape) => [shape.id, shape as Shape]));
      let changed = false;
      for (const violation of measured.violations) {
        if (violation.check === "gap" && violation.shapes.length === 2) {
          for (const id of violation.shapes) {
            const shape = byId.get(id);
            if (shape === undefined) continue;
            raisePen(shape, penNow(live, shape) + violation.measuredMm + 0.04);
            changed = true;
          }
        } else if (violation.check === "gap") {
          const add = Math.max(0.04, PROFILE.minGapMm - violation.measuredMm + 0.04);
          for (const shape of live.geometry.elevation) {
            if ((shape as Shape).role === "ground") continue;
            raisePen(shape as Shape, penNow(live, shape as Shape) + add);
            changed = true;
          }
        } else if (violation.check === "feature") {
          const shape = byId.get(violation.shapes[0] ?? "");
          if (shape === undefined) continue;
          raisePen(shape, penNow(live, shape) + (PROFILE.minFeatureMm - violation.measuredMm) + 0.03);
          changed = true;
        }
      }
      if (!changed) break;
      notes.push(`fatten 1:${den} ${measured.violations.length}`);
    }
  }
  return notes;
}

const folders = (await readdir(root)).sort().filter((folder) => !only || folder === only);
for (const folder of folders) {
  const file = resolve(root, folder, (await readdir(resolve(root, folder))).find((name) => name.endsWith(".json"))!);
  const doc = JSON.parse(await readFile(file, "utf8"));
  const asset = doc.asset as Asset;
  console.log("[DEBUG] start", asset.id);
  const notes = metadata(asset);
  if (process.env.PENS_ONLY === "1") {
    const issues = validateAsset(fresh(asset)).map((issue) => `${issue.code} ${issue.path}`);
    if (notes.length) await writeFile(file, `${JSON.stringify(doc, null, 2)}\n`);
    console.log(`== ${asset.id}`, notes.join("; ") || "unchanged", issues.length ? issues.slice(0, 4).join(" | ") : "validate ok");
    continue;
  }
  const pose = asset.presentation.defaultPose;
  const extents = measureExtents(asset, pose);
  console.log("[DEBUG] extents", asset.id, extents.map((item) => `${item.den}:${item.w.toFixed(3)}x${item.h.toFixed(3)}`).join(" "));
  const heights = extents.map((item) => item.h);
  const widths = extents.map((item) => item.w);
  const median = (values: number[]): number => [...values].sort((a, b) => a - b)[Math.floor(values.length / 2)]!;
  const pick = (values: number[], current: number): number => {
    const mid = values.length ? (Math.min(...values) + Math.max(...values)) / 2 : current;
    const cands = [current, mid, ...values];
    for (const target of cands) if (values.length && values.every((value) => within(value, target))) return Math.round(target * 1000) / 1000;
    return Math.round(median(values) * 1000) / 1000;
  };
  const th = pick(heights, asset.dimensions.heightM);
  const tw = pick(widths, asset.dimensions.widthM);
  if (th !== asset.dimensions.heightM || tw !== asset.dimensions.widthM) {
    asset.dimensions.heightM = th;
    asset.dimensions.widthM = tw;
    notes.push(`dimensions ${th} x ${tw}`);
  }
  for (const den of DENS) if (authored(asset, den) && fitScale(asset, den, pose, asset.dimensions.widthM, asset.dimensions.heightM)) notes.push(`fit 1:${den}`);
  console.log("[DEBUG] fitted", asset.id);
  notes.push(...distinguish(asset));
  console.log("[DEBUG] distinguished", asset.id);
  const scaleCount = (den: number): number => {
    const copy = fresh(asset);
    const rep = authored(copy, den);
    return rep ? measureSuitability(copy, rep, den, PROFILE).violations.length : 0;
  };
  for (let pass = 0; pass < 2; pass++) {
    let improved = false;
    for (const den of DENS) {
      if (!authored(asset, den)) continue;
      console.log("[DEBUG] count", asset.id, den);
      const started = Date.now();
      const before = scaleCount(den);
      console.log("[DEBUG] counted", den, before, `${Date.now() - started}ms`);
      if (before === 0) continue;
      const snap = JSON.parse(JSON.stringify(asset.representations[`${den}` as "500"]));
      const moved = closeScale(asset, den);
      const after = scaleCount(den);
      if (moved === 0 || after >= before) {
        asset.representations[`${den}` as "500"] = snap;
        if (before > 0) notes.push(`hold 1:${den} ${before}->${after} moved ${moved}`);
      } else {
        improved = true;
        notes.push(`close 1:${den} ${before}->${after}`);
      }
    }
    if (!improved) break;
  }
  const beforeThick = violationCount(asset);
  if (process.env.DO_THICK === "1" && beforeThick > 0) {
    const snap = JSON.parse(JSON.stringify(asset.representations));
    for (const den of DENS) {
      const thick = thicken(asset, den, pose);
      if (thick) notes.push(`thicken 1:${den} ${thick}`);
    }
    for (const den of DENS) if (authored(asset, den) && fitScale(asset, den, pose, asset.dimensions.widthM, asset.dimensions.heightM)) notes.push(`refit 1:${den}`);
    if (violationCount(asset) > beforeThick) {
      asset.representations = snap;
      notes.push("reverted thicken");
    }
  }
  const issues = validateAsset(fresh(asset)).map((issue) => `${issue.code} ${issue.path}`);
  console.log('[DEBUG] measure', asset.id);
  const suits = suitability(asset);
  if (notes.length || issues.length || suits.length) {
    await writeFile(file, `${JSON.stringify(doc, null, 2)}\n`);
    console.log(`\n== ${asset.id}`);
    console.log(" ", notes.join("; ") || "(geometry)");
    console.log("  validate", issues.length ? issues.slice(0, 8).join(" | ") : "ok");
    console.log("  suit", suits.length ? suits.join(" || ") : "ok");
  } else console.log(`== ${asset.id} unchanged`);
}
