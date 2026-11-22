/** 🧰️ Shared generator kit of worker HW-16 (weather and seasons humans): reading and writing catalog assets, pruning bones, uniform rescaling, rest frames and the projection of authored solids into per-scale, per-view outlines bound to their bone.
 *
 * The 2D outlines of a garment are the projection of the very solids authored for that scale (kernel tessellation, posed at rest, projected through the staffage cameras, united with polygon-clipping and generalised by Ramer-Douglas-Peucker at a paper tolerance), so drawing and volume are one body by construction.
 *
 * @see ../../📋️brief.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { contours } from "../../../../../../../../../node_modules/d3-contour/src/index.js";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { project, worldSolids, type Viewpoint } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

export const ROOT = resolve(import.meta.dir, "../../../../../../../../..");
export const STAFFAGE = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage");
export const HUMANS = join(STAFFAGE, "🗂️catalog/👤️humans");
export const TICKET = resolve(import.meta.dir, "../..");
export const GENERATED = join(TICKET, "🗑️generated/HW-16");
export const SCALES = ["50", "100", "200", "500"] as const;
export const VIEWS = ["elevation", "front", "plan"] as const;

export type Json = any;
export type P2 = [number, number];

/** 📥️ The asset document of a catalog human. */
export const readDoc = async (slug: string): Promise<Json> => Bun.file(join(HUMANS, slug, "🔣️.json")).json();

/** 📤️ Writes the asset document of a catalog human (two-space JSON like the catalog), or with `HW16_DRAFT=1` a draft to `🗑️generated/HW-16/<slug>.json`. */
export function writeDoc(slug: string, doc: Json): string {
  const file = process.env.HW16_DRAFT === "1" ? join(GENERATED, `${slug}.json`) : join(HUMANS, slug, "🔣️.json");
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify(doc, null, 2)}\n`);
  return file;
}

export const clone = <T>(x: T): T => structuredClone(x);
export const r5 = (v: number): number => Math.round(v * 1e5) / 1e5;
export const t = (en: string, de: string) => ({ en, de });

/** 🌳️ The bone ids of the subtrees rooted at `roots`. */
export function subtree(asset: Json, roots: readonly string[]): Set<string> {
  const out = new Set<string>(roots);
  let grew = true;
  while (grew) {
    grew = false;
    for (const b of asset.rig.bones) if (b.parent && out.has(b.parent) && !out.has(b.id)) (out.add(b.id), (grew = true));
  }
  return out;
}

/** ✂️ Removes bones with their subtrees and everything bound to them: shapes, solids (and thermal/structure references to them), anchors, pose and clip offsets, contacts, loads. */
export function dropBones(asset: Json, roots: readonly string[]): Set<string> {
  const gone = subtree(asset, roots);
  asset.rig.bones = asset.rig.bones.filter((b: Json) => !gone.has(b.id));
  const anchors = new Set(asset.anchors.filter((a: Json) => gone.has(a.bone)).map((a: Json) => a.id));
  asset.anchors = asset.anchors.filter((a: Json) => !gone.has(a.bone));
  for (const sc of Object.keys(asset.representations)) {
    const r = asset.representations[sc];
    if (r.status !== "authored") continue;
    for (const v of VIEWS) r.geometry[v] = r.geometry[v].filter((s: Json) => !gone.has(s.bone));
  }
  for (const sc of Object.keys(asset.volumes)) {
    const v = asset.volumes[sc];
    if (v.status !== "authored") continue;
    const removed = new Set(v.body.filter((s: Json) => gone.has(s.bone)).map((s: Json) => s.id));
    v.body = v.body.filter((s: Json) => !gone.has(s.bone));
    for (const list of ["thermal", "structure"]) v[list] = v[list].filter((s: Json) => !("of" in s ? removed.has(s.of) : gone.has(s.bone)) && !(s.members ?? []).some((m: string) => gone.has(m)) && !gone.has(s.member));
  }
  const strip = (bones: Json) => {
    for (const id of Object.keys(bones ?? {})) if (gone.has(id)) delete bones[id];
  };
  for (const p of asset.poses) {
    strip(p.bones);
    p.contacts = p.contacts.filter((c: string) => !anchors.has(c));
    p.loads = p.loads.filter((l: Json) => !anchors.has(l.anchor) && !gone.has(l.bone));
  }
  for (const c of asset.clips) {
    for (const k of c.keys) strip(k.bones);
    c.contacts = c.contacts.filter((s: Json) => !anchors.has(s.anchor));
    c.events = c.events.filter((e: Json) => !anchors.has(e.target) && !gone.has(e.target));
  }
  asset.structure.supports = asset.structure.supports.filter((s: string) => !anchors.has(s));
  return gone;
}

/** 🧹️ Removes shapes (every scale and view) and body solids (with their thermal/structure references) selected by a predicate. */
export function dropParts(asset: Json, shape: (s: Json, scale: string, view: string) => boolean, solid: (s: Json, scale: string) => boolean): void {
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    if (r?.status === "authored") for (const v of VIEWS) r.geometry[v] = r.geometry[v].filter((s: Json) => !shape(s, sc, v));
    const vol = asset.volumes[sc];
    if (vol?.status !== "authored") continue;
    const removed = new Set(vol.body.filter((s: Json) => solid(s, sc)).map((s: Json) => s.id));
    vol.body = vol.body.filter((s: Json) => !removed.has(s.id));
    vol.thermal = vol.thermal.filter((s: Json) => !("of" in s && removed.has(s.of)) && !(!("of" in s) && solid(s, sc)));
    vol.structure = vol.structure.filter((s: Json) => !("of" in s && removed.has(s.of)));
  }
}

const scalePts = (pts: number[][], s: number) => pts.map((p) => p.map((c) => c * s));

/** 📏️ Rescales every length of an asset uniformly by `s` (rig, anchors, drawings, volumes, root motion, loads' points, emitter openings are kept), masses by s³; thermal areas are rescaled by the caller. */
export function scaleUniform(asset: Json, s: number): void {
  for (const b of asset.rig.bones) {
    for (const k of ["x", "y", "z", "lengthM"]) if (typeof b[k] === "number") b[k] = r5(b[k] * s);
    if (b.mass) (b.mass.comX = r5(b.mass.comX * s)), (b.mass.comY = r5(b.mass.comY * s)), (b.mass.kg = Math.round(b.mass.kg * s ** 3 * 1e4) / 1e4);
    if (b.joint?.type === "prismatic") for (const k of ["minM", "maxM"]) if (typeof b.joint[k] === "number") b.joint[k] = r5(b.joint[k] * s);
  }
  for (const a of asset.anchors) for (const k of ["x", "y", "z", "footprintLengthM", "footprintWidthM"]) if (typeof a[k] === "number") a[k] = r5(a[k] * s);
  const shape = (sh: Json) => {
    if (sh.points) sh.points = scalePts(sh.points, s).map((p) => p.map(r5));
    for (const k of ["cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "x", "y", "w", "h"]) if (typeof sh[k] === "number") sh[k] = r5(sh[k] * s);
    if (sh.d) throw new Error(`path shape ${sh.id} cannot be rescaled`);
    if (sh.solid) for (const k of ["z0", "z1", "axisX", "axisZ"]) if (typeof sh.solid[k] === "number") sh.solid[k] = r5(sh.solid[k] * s);
    if (sh.solid?.zTip) sh.solid.zTip = sh.solid.zTip.map((v: number) => r5(v * s));
  };
  for (const sc of Object.keys(asset.representations)) {
    const r = asset.representations[sc];
    if (r.status !== "authored") continue;
    for (const v of VIEWS) r.geometry[v].forEach(shape);
    for (const sec of Object.values(r.sections ?? {}) as Json[]) sec.plane = r5(sec.plane * s);
  }
  const solid = (so: Json) => {
    for (const k of ["p0", "p1", "radii", "size", "origin", "translation"]) if (Array.isArray(so[k])) so[k] = so[k].map((v: number) => r5(v * s));
    for (const k of ["r0", "r1", "cornerRadius", "from", "to"]) if (typeof so[k] === "number") so[k] = r5(so[k] * s);
    if (so.profile) so.profile = so.profile.map((p: number[]) => p.map((v) => r5(v * s)));
    if (so.holes) so.holes = so.holes.map((h: number[][]) => h.map((p) => p.map((v) => r5(v * s))));
    if (so.vertices) so.vertices = so.vertices.map((p: number[]) => p.map((v) => r5(v * s)));
    if (so.sections) for (const sec of so.sections) (sec.at = sec.at.map((v: number) => r5(v * s))), (sec.rx = r5(sec.rx * s)), (sec.ry = r5(sec.ry * s));
    if (so.transform?.translation) so.transform.translation = so.transform.translation.map((v: number) => r5(v * s));
    if (so.repeat) for (const k of ["translation", "origin"]) if (so.repeat[k]) so.repeat[k] = so.repeat[k].map((v: number) => r5(v * s));
  };
  for (const sc of Object.keys(asset.volumes)) {
    const v = asset.volumes[sc];
    if (v.status !== "authored") continue;
    for (const list of ["body", "thermal", "structure"]) for (const so of v[list]) if (!("of" in so)) solid(so);
  }
  for (const p of asset.poses) {
    for (const o of Object.values(p.bones) as Json[]) for (const k of ["dx", "dy"]) if (typeof o[k] === "number") o[k] = r5(o[k] * s);
    if (p.root) for (const k of ["x", "y"]) if (typeof p.root[k] === "number") p.root[k] = r5(p.root[k] * s);
    for (const l of p.loads) for (const k of ["x", "y", "z"]) if (typeof l[k] === "number") l[k] = r5(l[k] * s);
  }
  for (const c of asset.clips) {
    if (typeof c.advanceM === "number") c.advanceM = r5(c.advanceM * s);
    for (const k of c.keys) for (const o of Object.values(k.bones ?? {}) as Json[]) for (const q of ["dx", "dy"]) if (typeof o[q] === "number") o[q] = r5(o[q] * s);
  }
  for (const m of asset.thermal.respiration?.motion ?? []) if (typeof m.dy === "number") m.dy = r5(m.dy * s);
  for (const r of asset.thermal.regions) if (typeof r.characteristicLengthM === "number") r.characteristicLengthM = r5(r.characteristicLengthM * s);
  for (const k of ["heightM", "widthM", "depthM"]) asset.dimensions[k] = Math.round(asset.dimensions[k] * s * 1000) / 1000;
}

/** 🦴️ The rest frames of the rig, plain vectors. */
export function rest(asset: Json): Record<string, { o: number[]; u: number[]; v: number[]; w: number[] }> {
  const f = restFrames(asset.rig);
  return Object.fromEntries(Object.entries(f).map(([id, s]: [string, Json]) => [id, { o: s.frame.o, u: s.frame.u, v: s.frame.v, w: s.frame.w }]));
}

/** 🌍️ The rest frames as the `{ frame }` records `worldSolids` poses with. */
export function restBones(asset: Json): Record<string, { frame: Json }> {
  const f = restFrames(asset.rig);
  const vec = (a: number[]) => ({ x: a[0]!, y: a[1]!, z: a[2]! });
  return Object.fromEntries(Object.entries(f).map(([id, s]: [string, Json]) => [id, { frame: { o: vec(s.frame.o), u: vec(s.frame.u), v: vec(s.frame.v), w: vec(s.frame.w) } }]));
}

/** 〰️ Ramer-Douglas-Peucker on a closed ring. */
export function simplify(ring: P2[], tol: number): P2[] {
  if (ring.length <= 4) return ring;
  const far = (a: P2, b: P2, p: P2) => {
    const [dx, dy] = [b[0] - a[0], b[1] - a[1]];
    const l = Math.hypot(dx, dy);
    return l < 1e-12 ? Math.hypot(p[0] - a[0], p[1] - a[1]) : Math.abs(dx * (a[1] - p[1]) - dy * (a[0] - p[0])) / l;
  };
  const rdp = (pts: P2[]): P2[] => {
    let [best, at] = [0, -1];
    for (let i = 1; i < pts.length - 1; i++) {
      const d = far(pts[0]!, pts[pts.length - 1]!, pts[i]!);
      if (d > best) (best = d), (at = i);
    }
    if (best <= tol || at < 0) return [pts[0]!, pts[pts.length - 1]!];
    return [...rdp(pts.slice(0, at + 1)).slice(0, -1), ...rdp(pts.slice(at))];
  };
  let i0 = 0;
  let dmax = 0;
  for (let i = 1; i < ring.length; i++) {
    const d = Math.hypot(ring[i]![0] - ring[0]![0], ring[i]![1] - ring[0]![1]);
    if (d > dmax) (dmax = d), (i0 = i);
  }
  const a = rdp(ring.slice(0, i0 + 1));
  const b = rdp([...ring.slice(i0), ring[0]!]);
  return [...a.slice(0, -1), ...b.slice(0, -1)];
}

/** 🖼️ The outline (outer rings, holes dropped) of the selected authored solids of one scale and list at rest, projected into a view, in paper coordinates of that view (world metres): elevation (x, y), front (z, y), plan (x, z). The projected triangles are rasterised at `cellM` and traced by marching squares (d3-contour), so the outline is the union to within half a cell. */
export function outlineOf(asset: Json, scale: string, ids: ReadonlySet<string>, view: Viewpoint, list: "body" | "thermal" | "structure" = "body", cellM = (0.02 * Number(scale)) / 1000, bones: Json = restBones(asset)): P2[][] {
  const solids = worldSolids(asset, bones as never, Number(scale), list).filter((w) => ids.has(w.id));
  const tris: P2[][] = [];
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const w of solids)
    for (let i = 0; i < w.indices.length; i += 3) {
      const q = [0, 1, 2].map((k) => {
        const j = w.indices[i + k]! * 3;
        const p = project(view, w.positions[j]!, w.positions[j + 1]!, w.positions[j + 2]!);
        return [p.x, p.y] as P2;
      });
      for (const [x, y] of q) (x0 = Math.min(x0, x)), (y0 = Math.min(y0, y)), (x1 = Math.max(x1, x)), (y1 = Math.max(y1, y));
      tris.push(q);
    }
  if (tris.length === 0) return [];
  const pad = 3 * cellM;
  (x0 -= pad), (y0 -= pad), (x1 += pad), (y1 += pad);
  const [W, H] = [Math.ceil((x1 - x0) / cellM), Math.ceil((y1 - y0) / cellM)];
  const mask = new Float64Array(W * H);
  for (const q of tris) {
    const ys = q.map((p) => p[1]);
    const jA = Math.max(0, Math.floor((Math.min(...ys) - y0) / cellM));
    const jB = Math.min(H - 1, Math.ceil((Math.max(...ys) - y0) / cellM));
    for (let j = jA; j <= jB; j++) {
      const yc = y0 + (j + 0.5) * cellM;
      let [lo, hi] = [Infinity, -Infinity];
      for (let e = 0; e < 3; e++) {
        const [a, b] = [q[e]!, q[(e + 1) % 3]!];
        if ((a[1] <= yc && b[1] > yc) || (b[1] <= yc && a[1] > yc)) {
          const x = a[0] + ((yc - a[1]) / (b[1] - a[1])) * (b[0] - a[0]);
          (lo = Math.min(lo, x)), (hi = Math.max(hi, x));
        }
      }
      if (lo > hi) continue;
      for (let i = Math.max(0, Math.round((lo - x0) / cellM - 0.5)); i <= Math.min(W - 1, Math.round((hi - x0) / cellM - 0.5)); i++) mask[j * W + i] = 1;
    }
  }
  const polys = contours().size([W, H]).thresholds([0.5])(mask as never)[0]!.coordinates as unknown as number[][][][];
  return polys.map((poly) => poly[0]!.slice(0, -1).map(([i, j]) => [x0 + i! * cellM, y0 + j! * cellM] as P2));
}

/** 📐️ World paper points of a view into the coordinates a shape of `bone` is authored in: for the elevation the point of the bone plane (u, v) at rest that projects onto the paper point (exact also for a bone that rests out of the sagittal plane), world rest for front and plan. */
export function toShapeFrame(asset: Json, bone: string, view: Viewpoint, ring: P2[]): P2[] {
  if (view !== "elevation") return ring;
  const f = rest(asset)[bone]!;
  const det = f.u[0]! * f.v[1]! - f.v[0]! * f.u[1]!;
  if (Math.abs(det) < 0.2) throw new Error(`bone ${bone} rests nearly edge-on to the side elevation (${det.toFixed(3)})`);
  return ring.map(([x, y]) => {
    const [dx, dy] = [x - f.o[0]!, y - f.o[1]!];
    return [(dx * f.v[1]! - dy * f.v[0]!) / det, (f.u[0]! * dy - f.u[1]! * dx) / det];
  });
}

/** ✏️ Filled poly shapes for the outline of solids, generalised at `tolMm` of paper. */
export function shapesFrom(asset: Json, scale: string, view: Viewpoint, ids: readonly string[], base: { id: string; bone: string; region: string; material: string; tone: number; z: number; role?: string }, tolMm = 0.03, minAreaM2 = 0): Json[] {
  const rings = outlineOf(asset, scale, new Set(ids), view)
    .map((r) => simplify(r as P2[], (tolMm * Number(scale)) / 1000))
    .filter((r) => r.length >= 3 && Math.abs(ringArea(r)) > minAreaM2)
    .sort((a, b) => Math.abs(ringArea(b)) - Math.abs(ringArea(a)));
  return rings.map((r, i) => ({ id: i === 0 ? base.id : `${base.id}-${i + 1}`, bone: base.bone, role: base.role ?? "body", z: base.z, filled: true, tone: base.tone, region: base.region, material: base.material, kind: "poly", points: toShapeFrame(asset, base.bone, view, r).map((p) => p.map(r5)), closed: true }));
}

export const ringArea = (r: P2[]): number => r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2;

/** 🩹️ Writes a debug text file into the generated folder. */
export function debug(name: string, text: string): void {
  mkdirSync(GENERATED, { recursive: true });
  writeFileSync(join(GENERATED, name), text);
}

/** 🌐️ The ring of an authored poly shape in world paper coordinates of its view at rest. */
export function worldRing(asset: Json, shape: Json, view: Viewpoint): P2[] {
  if (shape.kind !== "poly") throw new Error(`shape ${shape.id} is ${shape.kind}`);
  if (view !== "elevation") return shape.points;
  const f = rest(asset)[shape.bone]!;
  return shape.points.map(([a, b]: P2) => [f.o[0]! + a * f.u[0]! + b * f.v[0]!, f.o[1]! + a * f.u[1]! + b * f.v[1]!]);
}

/** 📍️ Whether a point lies inside a ring (even-odd). */
export function inside(ring: P2[], [x, y]: P2): boolean {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a[1] > y !== b[1] > y && x < ((b[0] - a[0]) * (y - a[1])) / (b[1] - a[1]) + a[0]) c = !c;
  }
  return c;
}

export const centroid = (r: P2[]): P2 => {
  const A = ringArea(r);
  let [cx, cy] = [0, 0];
  for (let i = 0; i < r.length; i++) {
    const [p, q] = [r[i]!, r[(i + 1) % r.length]!];
    const k = p[0] * q[1] - q[0] * p[1];
    (cx += (p[0] + q[0]) * k), (cy += (p[1] + q[1]) * k);
  }
  return [cx / (6 * A), cy / (6 * A)];
};

/** 🌲️ The number of joints between two bones of the rig. */
export function treeDistance(asset: Json, a: string, b: string): number {
  const up = (id: string) => {
    const chain: string[] = [];
    for (let x: string | undefined = id; x; x = asset.rig.bones.find((q: Json) => q.id === x)?.parent) chain.push(x);
    return chain;
  };
  const [p, q] = [up(a), up(b)];
  const common = p.find((x) => q.includes(x));
  return common === undefined ? Infinity : p.indexOf(common) + q.indexOf(common);
}

const solidBone = (asset: Json, scale: string, id: string): string | undefined => asset.volumes[scale].body.find((s: Json) => s.id === id)?.bone;

/** 🧭️ Which of the candidate solids each filled poly shape of one view stands for: a solid goes to the shape of its own bone when the view has one (the topmost that holds the centroid of its projected outline, else the topmost of that bone), otherwise to the shape that holds that centroid and whose bone is nearest in the rig. */
export function assignSolids(asset: Json, scale: string, view: Viewpoint, shapes: Json[], candidates: readonly string[]): Map<Json, string[]> {
  const out = new Map<Json, string[]>(shapes.map((s) => [s, []]));
  for (const id of candidates) {
    const rings = outlineOf(asset, scale, new Set([id]), view);
    if (rings.length === 0) continue;
    const big = rings.sort((a, b) => Math.abs(ringArea(b)) - Math.abs(ringArea(a)))[0]!;
    const c = centroid(big);
    const bone = solidBone(asset, scale, id);
    const own = shapes.filter((s) => s.bone === bone);
    const holding = shapes.filter((s) => inside(worldRing(asset, s, view), c));
    const near = holding.sort((p, q) => treeDistance(asset, p.bone, bone!) - treeDistance(asset, q.bone, bone!))[0];
    const hit = (own.length > 0 ? [...own].reverse().find((s) => inside(worldRing(asset, s, view), c)) ?? own[own.length - 1] : undefined) ?? near;
    if (hit) out.get(hit)!.push(id);
  }
  return out;
}

/** 🔁️ Replaces the points of a filled poly shape by the outline of the given solids (the largest ring; further rings become sibling shapes). */
export function retrace(asset: Json, scale: string, view: Viewpoint, shape: Json, ids: readonly string[], tolMm = 0.03): Json[] {
  const made = shapesFrom(asset, scale, view, ids, { id: shape.id, bone: shape.bone, region: shape.region, material: shape.material, tone: shape.tone, z: shape.z, role: shape.role }, tolMm);
  if (made.length === 0) return [];
  const { points: _p, ...meta } = shape;
  return made.map((m, i) => ({ ...meta, ...(i === 0 ? {} : { id: m.id }), points: m.points }));
}

/** 🎯️ A small inverse kinematics: coordinate search over the named offset fields (within the joint limits) that brings a point of a bone (bone-local u, v, w) to a target point given in the frame of a reference bone; returns the offsets and the residual distance in metres. */
export function reach(asset: Json, fixed: Record<string, Json>, vars: { bone: string; field: "angleDeg" | "abductionDeg" | "twistDeg"; min: number; max: number; start?: number }[], tip: { bone: string; at: number[] }, target: { bone: string; at: number[] }, extra?: (frames: Json) => number): { offsets: Record<string, Json>; error: number } {
  const { solveChain } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts");
  const x = vars.map((v) => v.start ?? 0);
  const build = (vals: number[]) => {
    const off: Record<string, Json> = clone(fixed);
    vars.forEach((v, i) => ((off[v.bone] ??= {}), (off[v.bone][v.field] = Math.round(vals[i]! * 100) / 100)));
    return off;
  };
  const point = (f: Json, at: number[]) => [0, 1, 2].map((k) => f.o[k] + at[0]! * f.u[k] + at[1]! * f.v[k] + at[2]! * f.w[k]);
  const cost = (vals: number[]) => {
    const frames = solveChain(asset.rig, build(vals), { x: 0, y: 0, angleDeg: 0 });
    const p = point(frames[tip.bone].frame, tip.at);
    const q = point(frames[target.bone].frame, target.at);
    return Math.hypot(p[0]! - q[0]!, p[1]! - q[1]!, p[2]! - q[2]!) + (extra ? extra(frames) : 0);
  };
  let best = cost(x);
  for (let step = 16; step > 0.05; step /= 2)
    for (let it = 0; it < 60; it++) {
      let moved = false;
      for (let i = 0; i < x.length; i++)
        for (const s of [step, -step]) {
          const y = [...x];
          y[i] = Math.max(vars[i]!.min, Math.min(vars[i]!.max, y[i]! + s));
          const c = cost(y);
          if (c < best - 1e-9) (best = c), (x[i] = y[i]!), (moved = true);
        }
      if (!moved) break;
    }
  return { offsets: build(x), error: best };
}

/** 🦴️ The world frames of a pose's bone offsets (root at the origin). */
export function framesOf(asset: Json, offsets: Record<string, Json>): Json {
  const { solveChain } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts");
  return solveChain(asset.rig, offsets, { x: 0, y: 0, angleDeg: 0 });
}

/** 🌡️ The outer area and the exposure of every thermal region as the volume rule measures them at one scale in the default pose (`info:outerAreaM2:<region>`, `info:exposureMeasured:<region>`). */
export function measuredRegions(asset: Json, scale = 50): Record<string, { area: number; exposure: number }> {
  const { measureVolumes } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
  const out: Record<string, { area: number; exposure: number }> = {};
  for (const m of measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] })) {
    const hit = /^info:(outerAreaM2|exposureMeasured):(.+)$/.exec(m.name);
    if (!hit) continue;
    out[hit[2]!] ??= { area: 0, exposure: 0 };
    if (hit[1] === "outerAreaM2") out[hit[2]!]!.area = m.value;
    else out[hit[2]!]!.exposure = m.value;
  }
  return out;
}

/** 🏷️ Renames a thermal region everywhere it is referenced. */
export function renameRegion(asset: Json, from: string, to: string): void {
  const reg = asset.thermal.regions.find((r: Json) => r.id === from);
  if (reg) reg.id = to;
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    if (r?.status === "authored") for (const v of VIEWS) for (const s of r.geometry[v]) {
      if (s.region === from) s.region = to;
      if (Array.isArray(s.regions)) s.regions = s.regions.map((x: string) => (x === from ? to : x));
    }
    const vol = asset.volumes[sc];
    if (vol?.status === "authored") for (const s of vol.thermal) if (s.region === from) s.region = to;
  }
  for (const sym of asset.symbols ?? []) for (const v of VIEWS) for (const s of sym.geometry?.[v] ?? []) if (s.region === from) s.region = to;
  for (const e of asset.thermal.emitters) if (e.region === from) e.region = to;
  for (const p of asset.poses) if (p.thermal.regions[from]) (p.thermal.regions[to] = p.thermal.regions[from]), delete p.thermal.regions[from];
  for (const c of asset.clips) for (const tr of c.tracks ?? []) if (tr.target.startsWith(`region:${from}:`)) tr.target = tr.target.replace(`region:${from}:`, `region:${to}:`);
}

/** 🎨️ Renames materials everywhere they are referenced (shapes of every scale and symbol, solids of every list, regions and their layers). */
export function remapMaterials(asset: Json, map: Record<string, string>): void {
  const fix = (o: Json) => {
    if (o && typeof o.material === "string" && map[o.material]) o.material = map[o.material];
  };
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    if (r?.status === "authored") for (const v of VIEWS) r.geometry[v].forEach(fix);
    const vol = asset.volumes[sc];
    if (vol?.status === "authored") for (const list of ["body", "thermal", "structure"]) vol[list].forEach(fix);
  }
  for (const sym of asset.symbols ?? []) for (const v of VIEWS) (sym.geometry?.[v] ?? []).forEach(fix);
  for (const r of asset.thermal.regions) (fix(r), r.layers.forEach(fix));
}

/** 📐️ Sets `dimensions` (width = forward x, height, depth = lateral z) to the body of the finest volume in the default pose, rounded to the millimetre. */
export function fitDimensions(asset: Json): number[] {
  const { measureAsset } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
  const m = measureAsset(asset, 50, undefined, undefined, 0.0075);
  const [w, h, d] = m.bbox.map((v: number) => Math.round(v * 1000) / 1000);
  Object.assign(asset.dimensions, { widthM: w, heightM: h, depthM: d });
  return [w, h, d];
}

/** ⚖️ The total mass of the rig. */
export const rigMass = (asset: Json): number => Math.round(asset.rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0) * 1e4) / 1e4;
