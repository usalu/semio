/** 🧰️ Shared authoring library of HW-07 (tennis-player, basketball-player, yoga-practitioner): derives a person from the closest catalog human by
 * anthropometric scaling, then adds the authored props, poses, clips and physics of the new person. Every number written is either design input of
 * the per-asset script or derived here from it.
 * @see ../../📋️brief.md — the binding brief */
import { join, resolve } from "node:path";
import { featureOf, smallestDimension } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

export const REPO = resolve(import.meta.dir, "../../../../../../../../..");
export const STAFFAGE = join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage");
export const HUMANS = join(STAFFAGE, "🗂️catalog/👤️humans");
export const TICKET = resolve(import.meta.dir, "../..");
export const GENERATED = join(TICKET, "🗑️generated/HW-07");

export type Json = any;
export type T2 = { en: string; de: string };
export const tx = (en: string, de: string): T2 => ({ en, de });
export const r = (x: number, d = 5): number => Math.round(x * 10 ** d) / 10 ** d;
export const clone = <X>(x: X): X => structuredClone(x);

/** 📥️ The asset document of a catalog human. */
export async function loadHuman(slug: string): Promise<Json> {
  return await Bun.file(join(HUMANS, slug, "🔣️.json")).json();
}

/** 💾️ Writes the asset document of a new human (the catalog folder of its id). */
export async function saveHuman(doc: Json): Promise<string> {
  const path = join(HUMANS, doc.asset.id, "🔣️.json");
  await Bun.write(path, JSON.stringify(doc, null, 1) + "\n");
  return path;
}

const scalePoint = (p: number[], s: number): number[] => p.map((v) => r(v * s, 6));

function scaleShape(shape: Json, s: number): void {
  if (shape.points) shape.points = shape.points.map((p: number[]) => scalePoint(p, s));
  for (const k of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof shape[k] === "number") shape[k] = r(shape[k] * s, 6);
  if (shape.kind === "path") throw new Error(`path shape ${shape.id}: scaling of paths is not needed by the HW-07 sources`);
  if (shape.kind === "member") for (const e of [shape.from, shape.to]) for (const k of ["x", "y", "z"]) e[k] = r(e[k] * s, 6);
  const solid = shape.solid;
  if (solid) {
    for (const k of ["axisX", "axisZ", "z0", "z1"]) if (typeof solid[k] === "number") solid[k] = r(solid[k] * s, 6);
    if (solid.zTip) solid.zTip = scalePoint(solid.zTip, s);
  }
}

function scaleSolid(solid: Json, s: number): void {
  for (const k of ["p0", "p1", "radii", "size", "origin"]) if (Array.isArray(solid[k])) solid[k] = scalePoint(solid[k], s);
  for (const k of ["r0", "r1", "cornerRadius", "from", "to"]) if (typeof solid[k] === "number") solid[k] = r(solid[k] * s, 6);
  if (solid.profile) solid.profile = solid.profile.map((p: number[]) => scalePoint(p, s));
  if (solid.holes) solid.holes = solid.holes.map((h: number[][]) => h.map((p) => scalePoint(p, s)));
  if (solid.vertices) solid.vertices = solid.vertices.map((p: number[]) => scalePoint(p, s));
  if (solid.sections) for (const sec of solid.sections) { sec.at = scalePoint(sec.at, s); sec.rx = r(sec.rx * s, 6); sec.ry = r(sec.ry * s, 6); }
  if (solid.transform?.translation) solid.transform.translation = scalePoint(solid.transform.translation, s);
  if (solid.repeat?.translation) solid.repeat.translation = scalePoint(solid.repeat.translation, s);
  if (solid.repeat?.origin) solid.repeat.origin = scalePoint(solid.repeat.origin, s);
}

/** 🪵️ Widens the cross-section of a solid by `g` about its own axis (scaling down must not push a solid that met the smallest feature of its scale below it). */
function thicken(solid: Json, g: number): void {
  if (solid.kind === "capsule") { solid.r0 = r(solid.r0 * g, 6); solid.r1 = r(solid.r1 * g, 6); }
  else if (solid.kind === "loft") {
    const path = solid.sections.slice(1).reduce((t: number, sec: Json, i: number) => t + Math.hypot(...sec.at.map((v: number, k: number) => v - solid.sections[i].at[k])), 0);
    const cross = 2 * Math.min(Math.max(...solid.sections.map((x: Json) => x.rx)), Math.max(...solid.sections.map((x: Json) => x.ry)));
    if (path < cross) { const m = [0, 1, 2].map((k) => solid.sections.reduce((t: number, x: Json) => t + x.at[k], 0) / solid.sections.length); for (const sec of solid.sections) sec.at = sec.at.map((v: number, k: number) => r(m[k]! + (v - m[k]!) * g, 6)); }
    else for (const sec of solid.sections) { sec.rx = r(sec.rx * g, 6); sec.ry = r(sec.ry * g, 6); }
  }
  else if (solid.kind === "ellipsoid") solid.radii = solid.radii.map((x: number) => r(x * g, 6));
  else if (solid.kind === "box") solid.size = solid.size.map((x: number) => r(x * g, 6));
  else if (solid.kind === "revolve") solid.profile = solid.profile.map((p: number[]) => [p[0], r(p[1]! * g, 6)]);
  else throw new Error(`thicken: ${solid.kind} ${solid.id}`);
}

/** 📏️ Scales every model length of an asset uniformly by `s` (bones, anchors, shapes, volumes, poses, clips, thermal lengths and areas); paper
 * quantities (pens, hatches, valid ranges, paper-unit symbols), angles, tones and draw order stay. Masses and physiology are set by the caller. */
export function scaleAsset(asset: Json, s: number): void {
  const before = new Map<string, number>();
  for (const [k, vol] of Object.entries(asset.volumes) as [string, Json][]) if (vol.status === "authored") for (const list of ["body", "thermal", "structure"]) for (const solid of vol[list]) if (solid.of === undefined) before.set(`${k}/${list}/${solid.id}`, smallestDimension(solid));
  for (const k of ["heightM", "widthM", "depthM"]) asset.dimensions[k] = r(asset.dimensions[k] * s, 4);
  for (const b of asset.rig.bones) {
    for (const k of ["x", "y", "z", "lengthM"]) if (typeof b[k] === "number") b[k] = r(b[k] * s, 6);
    if (b.mass) { b.mass.comX = r(b.mass.comX * s, 6); b.mass.comY = r(b.mass.comY * s, 6); }
    if (b.joint.type === "prismatic") for (const k of ["minM", "maxM"]) if (typeof b.joint[k] === "number") b.joint[k] = r(b.joint[k] * s, 6);
  }
  for (const a of asset.anchors) for (const k of ["x", "y", "z", "footprintLengthM", "footprintWidthM"]) if (typeof a[k] === "number") a[k] = r(a[k] * s, 6);
  for (const rep of Object.values(asset.representations) as Json[]) {
    if (rep.status !== "authored") continue;
    for (const view of ["elevation", "front", "plan"]) for (const shape of rep.geometry[view]) scaleShape(shape, s);
    for (const view of ["elevation", "front", "plan"]) if (rep.sections?.[view]) rep.sections[view].plane = r(rep.sections[view].plane * s, 6);
  }
  for (const sym of asset.symbols) if (sym.units === "model-m") for (const view of ["elevation", "front", "plan"]) for (const shape of sym.geometry[view]) scaleShape(shape, s);
  for (const [k, vol] of Object.entries(asset.volumes) as [string, Json][]) {
    if (vol.status !== "authored") continue;
    const feature = featureOf(Number(k));
    for (const list of ["body", "thermal", "structure"]) for (const solid of vol[list]) if (solid.of === undefined) {
      scaleSolid(solid, s);
      const was = before.get(`${k}/${list}/${solid.id}`)!, now = smallestDimension(solid);
      if (now < feature && was >= feature * 0.998) thicken(solid, (feature * 1.002) / now);
    }
  }
  const offsets = (bones: Json) => { for (const o of Object.values(bones ?? {}) as Json[]) for (const k of ["dx", "dy"]) if (typeof o[k] === "number") o[k] = r(o[k] * s, 6); };
  for (const p of asset.poses) {
    if (p.root) for (const k of ["x", "y"]) if (typeof p.root[k] === "number") p.root[k] = r(p.root[k] * s, 6);
    offsets(p.bones);
  }
  for (const c of asset.clips) {
    if (typeof c.advanceM === "number") c.advanceM = r(c.advanceM * s, 6);
    for (const k of c.keys) offsets(k.bones);
  }
  const t = asset.thermal;
  if (typeof t.minimumAirPassageM === "number") t.minimumAirPassageM = r(t.minimumAirPassageM * s, 6);
  for (const g of t.regions) {
    g.areaM2 = r(g.areaM2 * s * s, 5);
    g.characteristicLengthM = r(g.characteristicLengthM * s, 4);
    for (const l of g.layers) if (typeof l.thicknessM === "number") l.thicknessM = r(l.thicknessM * s, 5);
  }
  for (const e of t.emitters) e.diameterM = r(e.diameterM * s, 5);
  if (t.respiration) for (const m of t.respiration.motion) m.dy = r(m.dy * s, 5);
}

/** ⚖️ De Leva segment masses rescaled to a new total body mass (the fractions of the source stay). */
export function setBodyMass(asset: Json, bodyKg: number, propBones: readonly string[] = []): void {
  const body = asset.rig.bones.filter((b: Json) => b.mass && !propBones.includes(b.id));
  const total = body.reduce((s: number, b: Json) => s + b.mass.kg, 0);
  for (const b of body) b.mass.kg = r((b.mass.kg * bodyKg) / total, 4);
}

/** 🫀️ DuBois skin area (DuBois and DuBois 1916). */
export const duBois = (kg: number, m: number): number => 0.007184 * kg ** 0.425 * (m * 100) ** 0.725;
/** 🔥️ Basal metabolic rate in W from the Mifflin-St Jeor equation (kcal/d; 1 kcal/d = 0.04843 W). */
export const mifflin = (kg: number, m: number, age: number, sex: "male" | "female"): number => (10 * kg + 625 * m - 5 * age + (sex === "male" ? 5 : -161)) * 0.04843;

//#region 🌀️Solids for shapes on bones that leave their rest plane
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

type V3 = [number, number, number];
const qrot = (q: number[], p: V3): V3 => {
  const [x, y, z, w] = q as [number, number, number, number];
  const ix = w * p[0] + y * p[2] - z * p[1], iy = w * p[1] + z * p[0] - x * p[2], iz = w * p[2] + x * p[1] - y * p[0], iw = -x * p[0] - y * p[1] - z * p[2];
  return [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x];
};

/** 🎯️ Sample points of a volume solid in its bone-local frame (u, v, w), enough for a bounding box. */
export function solidPoints(s: Json): V3[] {
  const pts: V3[] = [];
  const n = 16;
  if (s.kind === "loft") for (const sec of s.sections) for (let i = 0; i < n; i++) {
    const t = (2 * Math.PI * i) / n, e = sec.exponent ?? 2;
    const c = Math.cos(t), si = Math.sin(t);
    const px = sec.rx * Math.sign(c) * Math.abs(c) ** (2 / e), py = sec.ry * Math.sign(si) * Math.abs(si) ** (2 / e);
    const d = sec.rotation ? qrot(sec.rotation, [px, py, 0]) : [0, px, py] as V3;
    pts.push([sec.at[0] + d[0], sec.at[1] + d[1], sec.at[2] + d[2]]);
  }
  else if (s.kind === "capsule") for (const [p, rr] of [[s.p0, s.r0], [s.p1, s.r1]] as [V3, number][]) for (const d of [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]) pts.push([p[0] + d[0]! * rr, p[1] + d[1]! * rr, p[2] + d[2]! * rr * (s.lateralScale ?? 1)]);
  else if (s.kind === "ellipsoid") for (const d of [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]) pts.push([d[0]! * s.radii[0], d[1]! * s.radii[1], d[2]! * s.radii[2]]);
  else if (s.kind === "box") for (const a of [-1, 1]) for (const b of [-1, 1]) for (const c of [-1, 1]) pts.push([(a * s.size[0]) / 2, (b * s.size[1]) / 2, (c * s.size[2]) / 2]);
  else if (s.kind === "extrude") {
    const map = (p: number[], c: number): V3 => (s.plane === "xy" ? [p[0]!, p[1]!, c] : s.plane === "yz" ? [c, p[0]!, p[1]!] : [p[1]!, c, p[0]!]);
    for (const p of s.profile) for (const c of [s.from, s.to]) pts.push(map(p, c));
  } else if (s.kind === "revolve") {
    const ax = s.axis as V3, o = s.origin as V3;
    for (const [a, rad] of s.profile) for (const d of [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]) pts.push([o[0] + ax[0] * a + d[0]! * rad, o[1] + ax[1] * a + d[1]! * rad, o[2] + ax[2] * a + d[2]! * rad]);
  } else if (s.kind === "polyhedron") pts.push(...s.vertices);
  const t = s.transform;
  return pts.map((p) => {
    let q: V3 = t?.scale ? [p[0] * t.scale[0], p[1] * t.scale[1], p[2] * t.scale[2]] : p;
    if (t?.rotation) q = qrot(t.rotation, q);
    if (t?.translation) q = [q[0] + t.translation[0], q[1] + t.translation[1], q[2] + t.translation[2]];
    return q;
  });
}

/** 📦️ Bone-local bounding box of the body solids of a bone at one scale. */
export function boneBox(asset: Json, scale: string, bone: string): { lo: V3; hi: V3 } | null {
  const vol = asset.volumes[scale];
  if (vol?.status !== "authored") return null;
  const pts = vol.body.filter((s: Json) => s.bone === bone && s.role !== "texture").flatMap(solidPoints);
  if (pts.length === 0) return null;
  const lo: V3 = [Infinity, Infinity, Infinity], hi: V3 = [-Infinity, -Infinity, -Infinity];
  for (const p of pts) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); }
  return { lo, hi };
}

const REST_POSE = "hw07-rest";
const vec = (p: { x: number; y: number; z: number }): V3 => [p.x, p.y, p.z];
/** 🧭️ The frame of a bone of an evaluated state as arrays. */
export const frameOf = (state: Json, bone: string): { o: V3; u: V3; v: V3; w: V3 } => { const f = state.bones[bone].frame; return { o: vec(f.o), u: vec(f.u), v: vec(f.v), w: vec(f.w) }; };
/** 🧍️ The rest state of an asset at one scale (all offsets zero, the rig root at its rest). */
export function restState(asset: Json, scale: string): Json {
  const pose = { id: REST_POSE, title: tx("Rest", "Ruhe"), description: tx("Rest", "Ruhe"), key: true, bones: {}, ground: "authored", contacts: [], thermal: asset.poses[0].thermal, loads: [] };
  return evaluate({ ...asset, poses: [...asset.poses, pose] }, { kind: "pose", pose: REST_POSE } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never);
}

const ribbon = (pts: number[][], half: number): number[][] => {
  const left: number[][] = [], right: number[][] = [];
  for (let i = 0; i < pts.length; i++) {
    const a = pts[Math.max(0, i - 1)]!, b = pts[Math.min(pts.length - 1, i + 1)]!;
    const dx = b[0]! - a[0]!, dy = b[1]! - a[1]!, l = Math.hypot(dx, dy) || 1;
    left.push([pts[i]![0]! - (dy / l) * half, pts[i]![1]! + (dx / l) * half]);
    right.push([pts[i]![0]! + (dy / l) * half, pts[i]![1]! - (dx / l) * half]);
  }
  return [...left, ...right.reverse()];
};

/** 🌀️ Gives every flat shape on `bones` (all authored scales and views) an extruded `solid`, swept along the bone axis closest to the view direction
 * over the bone's body solids of that scale, so a pose that spreads or twists the bone draws it right in all three views. The profile is the rest
 * outline of the shape in the two other bone axes; open strokes become closed 2 mm ribbons (unfilled). The sweep covers `spanFactor` of the
 * bone's body thickness along the sweep axis (0.5 in elevation and front: a plate whose projection matches an elliptic section within about 10 %
 * up to a 45 degree turn, where a full-depth prism would draw the box diagonal; 0.2 in plan, whose profile already is the outline seen from above
 * and only tilts). Returns the number of shapes changed. */
export function solidify(asset: Json, bones: readonly string[], ribbonM = 0.002, spanFactor: Record<string, number> = { elevation: 0.5, front: 0.5, plan: 0.2 }): number {
  let changed = 0;
  const dirs: Record<string, V3> = { elevation: [0, 0, 1], front: [1, 0, 0], plan: [0, 1, 0] };
  const paperToWorld: Record<string, [number, number]> = { elevation: [0, 1], front: [2, 1], plan: [0, 2] };
  for (const [scale, rep] of Object.entries(asset.representations) as [string, Json][]) {
    if (rep.status !== "authored") continue;
    const state = restState(asset, scale);
    for (const view of ["elevation", "front", "plan"]) {
      const evaluated = state.shapes[view] as Json[];
      rep.geometry[view] = rep.geometry[view].map((shape: Json, index: number) => {
        if (!bones.includes(shape.bone) || shape.solid || shape.role === "ground" || shape.kind === "member") return shape;
        const ev = evaluated[index];
        if (ev?.shape?.id !== shape.id) throw new Error(`evaluated order differs at ${scale} ${view} ${shape.id}`);
        const f = frameOf(state, shape.bone);
        const axes = { u: f.u as V3, v: f.v as V3, w: f.w as V3 };
        const d = dirs[view]!;
        const sweep = (["u", "v", "w"] as const).reduce((b, a) => (Math.abs(axes[a][0] * d[0] + axes[a][1] * d[1] + axes[a][2] * d[2]) > Math.abs(axes[b][0] * d[0] + axes[b][1] * d[1] + axes[b][2] * d[2]) ? a : b), "u");
        const [pa, pb] = sweep === "u" ? (["v", "w"] as const) : sweep === "v" ? (["u", "w"] as const) : (["u", "v"] as const);
        const k = { u: 0, v: 1, w: 2 }[sweep];
        const box = boneBox(asset, scale, shape.bone);
        const c = box ? (box.lo[k]! + box.hi[k]!) / 2 : 0, half = box ? ((box.hi[k]! - box.lo[k]!) / 2) * spanFactor[view]! : 0.01;
        const lo = c - half, hi = c + half;
        const [i, j] = paperToWorld[view]!;
        const A = axes[pa], B = axes[pb], C = axes[sweep];
        const det = A[i]! * B[j]! - B[i]! * A[j]!;
        const local = (p: { x: number; y: number }): number[] => {
          const ri = p.x - f.o[i] - c * C[i]!, rj = p.y - f.o[j] - c * C[j]!;
          return [r((ri * B[j]! - B[i]! * rj) / det, 5), r((A[i]! * rj - ri * A[j]!) / det, 5)];
        };
        let ring: number[][];
        if (view === "elevation" && sweep === "w") ring = shape.points;
        else ring = (ev.rings[0] as { x: number; y: number }[]).map(local);
        if (!shape.closed) ring = ribbon(ring, ribbonM / 2).map((p) => [r(p[0]!, 5), r(p[1]!, 5)]);
        const base = sweep === "w" ? f.o[2] : 0;
        const solid = { kind: "extruded", sweep, z0: r(lo + base, 5), z1: r(hi + base, 5) };
        changed++;
        return { ...shape, kind: "poly", points: ring, closed: true, solid };
      });
    }
  }
  return changed;
}
//#endregion 🌀️Solids for shapes on bones that leave their rest plane

//#region 🎯️Pose solving (damped least squares on the rig chain)
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

export type Offsets = Record<string, { angleDeg?: number; abductionDeg?: number; twistDeg?: number; dx?: number; dy?: number }>;
export type Root = { x: number; y: number; angleDeg: number };
/** 🎯️ A goal of the pose solver: a bone-local point at a world target, a bone axis along a world direction, or the horizontal centre of mass at a target (x, z). */
export type Goal =
  | { kind: "point"; bone: string; p: V3; target: V3; weight?: number; dims?: number[] }
  | { kind: "axis"; bone: string; axis: "u" | "v" | "w"; dir: V3; weight?: number }
  | { kind: "com"; target: [number, number]; weight?: number };
export type Variable = [bone: string, field: "angleDeg" | "abductionDeg" | "twistDeg" | "dx" | "dy", stiffness?: number];

type Frames = Record<string, { o: V3; u: V3; v: V3; w: V3 }>;
const arr = (f: Json) => ({ o: f.o as V3, u: f.u as V3, v: f.v as V3, w: f.w as V3 });
/** 🦴️ World frames of the rig for offsets and a root; with `ground` the root height is set so the lowest of these anchors touches y = `groundY`. */
export function framesOf(asset: Json, offsets: Offsets, root: Root, ground?: { anchors: string[]; groundY?: number }): { frames: Frames; root: Root } {
  let solved: Json = solveChain(asset.rig, offsets as never, root);
  let rt = root;
  if (ground && ground.anchors.length) {
    const ys = ground.anchors.map((id) => anchorWorld(asset, solved, id)[1]);
    rt = { ...root, y: root.y + (ground.groundY ?? 0) - Math.min(...ys) };
    solved = solveChain(asset.rig, offsets as never, rt);
  }
  const frames: Frames = {};
  for (const [id, f] of Object.entries(solved) as [string, Json][]) frames[id] = arr(f.frame);
  return { frames, root: rt };
}
export const at = (f: { o: V3; u: V3; v: V3; w: V3 }, p: V3): V3 => [f.o[0] + p[0] * f.u[0] + p[1] * f.v[0] + p[2] * f.w[0], f.o[1] + p[0] * f.u[1] + p[1] * f.v[1] + p[2] * f.w[1], f.o[2] + p[0] * f.u[2] + p[1] * f.v[2] + p[2] * f.w[2]];
export function anchorWorld(asset: Json, solved: Json, id: string): V3 {
  const a = asset.anchors.find((x: Json) => x.id === id);
  const f = solved[a.bone].frame ?? solved[a.bone];
  return at(arr(f), [a.x, a.y, a.z ?? 0]);
}
/** ⚖️ Centre of mass of all bone masses in world. */
export function comOf(asset: Json, frames: Frames): V3 {
  let m = 0; const c: V3 = [0, 0, 0];
  for (const b of asset.rig.bones) if (b.mass) { const p = at(frames[b.id]!, [b.mass.comX, b.mass.comY, 0]); m += b.mass.kg; for (let k = 0; k < 3; k++) c[k] += p[k]! * b.mass.kg; }
  return [c[0] / m, c[1] / m, c[2] / m];
}

function limits(asset: Json, bone: string, field: string): [number, number] {
  const b = asset.rig.bones.find((x: Json) => x.id === bone);
  const j = b.joint;
  if (field === "angleDeg") return [j.minDeg ?? -180, j.maxDeg ?? 180];
  if (field === "twistDeg") return [j.twistMinDeg ?? -180, j.twistMaxDeg ?? 180];
  if (field === "abductionDeg") return [-180, 180];
  return [j.minM ?? -10, j.maxM ?? 10];
}

/** 🎯️ Solves the variables of a pose for weighted goals (damped least squares, numeric Jacobian, joint limits as clamps, a pull towards the start). */
export function solvePose(asset: Json, start: Offsets, vars: Variable[], goals: Goal[], opts: { root: Root; ground?: { anchors: string[]; groundY?: number }; iterations?: number; reg?: number } = { root: { x: 0, y: 0, angleDeg: 0 } }): { offsets: Offsets; root: Root; error: number } {
  const offsets: Offsets = clone(start);
  const get = (v: Variable) => offsets[v[0]]?.[v[1]] ?? 0;
  const set = (v: Variable, x: number) => { const [lo, hi] = limits(asset, v[0], v[1]); (offsets[v[0]] ??= {})[v[1]] = Math.min(hi, Math.max(lo, x)); };
  const x0 = vars.map(get);
  const reg = opts.reg ?? 1e-4;
  const residual = (): number[] => {
    const { frames } = framesOf(asset, offsets, opts.root, opts.ground);
    const out: number[] = [];
    for (const g of goals) {
      const w = g.weight ?? 1;
      if (g.kind === "point") { const p = at(frames[g.bone]!, g.p); for (const k of g.dims ?? [0, 1, 2]) out.push(w * (p[k]! - g.target[k]!)); }
      else if (g.kind === "axis") { const a = frames[g.bone]![g.axis]; const n = Math.hypot(...g.dir); for (let k = 0; k < 3; k++) out.push(w * (a[k]! - g.dir[k]! / n)); }
      else { const c = comOf(asset, frames); out.push(w * (c[0] - g.target[0]), w * (c[2] - g.target[1])); }
    }
    vars.forEach((v, i) => out.push((Math.sqrt(reg) * (v[2] ?? 1) * (get(v) - x0[i]!)) / 10));
    return out;
  };
  let lambda = 1e-2;
  let res = residual();
  let err = res.reduce((s, x) => s + x * x, 0);
  for (let it = 0; it < (opts.iterations ?? 200); it++) {
    const J: number[][] = [];
    for (const v of vars) { const x = get(v); const [, hi] = limits(asset, v[0], v[1]); const step = v[1] === "dx" || v[1] === "dy" ? 1e-4 : 1e-2; const h = x + step > hi ? -step : step; set(v, x + h); const rp = residual(); set(v, x); J.push(rp.map((y, k) => (y - res[k]!) / h)); }
    const n = vars.length;
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => J[i]!.reduce((s, y, k) => s + y * J[j]![k]!, 0)));
    const g = Array.from({ length: n }, (_, i) => J[i]!.reduce((s, y, k) => s + y * res[k]!, 0));
    let improved = false;
    for (let tries = 0; tries < 8 && !improved; tries++) {
      const M = A.map((row, i) => row.map((y, j) => y + (i === j ? lambda * (1 + A[i]![i]!) : 0)));
      const step = gauss(M, g.map((y) => -y));
      const old = vars.map(get);
      vars.forEach((v, i) => set(v, old[i]! + step[i]!));
      const r2 = residual(); const e2 = r2.reduce((s, x) => s + x * x, 0);
      if (e2 < err) { res = r2; err = e2; lambda = Math.max(1e-7, lambda / 3); improved = true; } else { vars.forEach((v, i) => set(v, old[i]!)); lambda *= 4; }
    }
    if (!improved || err < 1e-14) break;
  }
  for (const v of vars) { const o = offsets[v[0]]!; o[v[1]] = r(o[v[1]]!, v[1] === "dx" || v[1] === "dy" ? 5 : 2); }
  const { root } = framesOf(asset, offsets, opts.root, opts.ground);
  return { offsets, root, error: Math.sqrt(err) };
}
function gauss(A: number[][], b: number[]): number[] {
  const n = b.length, M = A.map((row, i) => [...row, b[i]!]);
  for (let c = 0; c < n; c++) {
    let p = c; for (let i = c + 1; i < n; i++) if (Math.abs(M[i]![c]!) > Math.abs(M[p]![c]!)) p = i;
    [M[c], M[p]] = [M[p]!, M[c]!];
    const d = M[c]![c]! || 1e-12;
    for (let i = 0; i < n; i++) if (i !== c) { const f = M[i]![c]! / d; for (let k = c; k <= n; k++) M[i]![k]! -= f * M[c]![k]!; }
  }
  return M.map((row, i) => row[n]! / (row[i]! || 1e-12));
}
//#endregion 🎯️Pose solving
