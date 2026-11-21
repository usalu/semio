/** 🛠️ Shared helpers of the round 3 tools (`r3-*.ts`): the author's source merged over the catalog asset, the evaluated rig of any pose or clip frame, solids carried to the world by those frames, and an owned deferred rasteriser (depth, normal and solid-id buffers) that shades smoothly and draws silhouette, seam and crease edges.
 *
 * @see ./r3-tools.md
 * @see ./volumes-lib.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { Canvas, type RGB } from "./volumes-raster.ts";
import { GENERATED, entryOf, loadSource, localGeometry, merged, qualityOfAsset, resolveList, type CatalogEntry, type Json, type ListName, type ScaleMeshes, type SolidMesh, type V3 } from "./volumes-lib.ts";

export type Loaded = { entry: CatalogEntry; asset: Json; catalog: Json; source: Json };

/** 📥️ The catalog asset with the author's `volumes` source merged over it, exactly as `volumes-render.ts` reads it; exits with 2 when the asset or its source is missing. */
export function load(spec: string | undefined, usage: string): Loaded {
  if (spec === undefined) {
    process.stderr.write(`usage: ${usage}\n`);
    process.exit(2);
  }
  const { entry, asset } = entryOf(spec);
  const source = loadSource(entry);
  if (source === null) {
    process.stderr.write(`no source volumes/${entry.category}/${entry.slug}.json for ${spec}\n`);
    process.exit(2);
  }
  return { entry, asset: merged(asset, source), catalog: asset, source };
}

/** 🎛️ Command line of the round 3 tools: the first free argument and the named flags with a value. */
export function cli(valued: readonly string[]): { spec: string | undefined; flag: (name: string) => string | undefined; has: (name: string) => boolean } {
  const args = process.argv.slice(2);
  const flag = (name: string) => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
  return { spec: args.find((a, i) => !a.startsWith("--") && !valued.includes(args[i - 1] ?? "")), flag, has: (name) => args.includes(name) };
}

export const r3Out = (tool: string, entry: CatalogEntry, name: string): string => join(GENERATED, tool, entry.category, name);

export function savePng(path: string, canvas: Canvas): string {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, canvas.png());
  return path;
}

export type Frame = { o: V3; u: V3; v: V3; w: V3 };
export type RigBone = { id: string; parent: string | null; o: V3; tip: V3 };
export type Query = { kind: "pose"; pose: string } | { kind: "clip"; clip: string; timeS: number };
export type Rig = { frames: Record<string, Frame>; bones: RigBone[]; state: Json; query: Query };

/** 🦴️ The evaluated rig of a pose or of a clip at a time: the 3D frame, origin and tip of every bone, plus the evaluated 2D shapes of the scale when its drawing evaluates. */
export function rigAt(asset: Json, scale: number, query: Query): Rig {
  let state: Json;
  try {
    state = evaluate(asset, query as never, { kind: "scale", scale: { denominator: scale } } as never);
  } catch {
    state = evaluate(asset, query as never);
  }
  const v = (p: Json): V3 => [p.x, p.y, p.z ?? 0];
  const frames: Record<string, Frame> = {};
  const bones: RigBone[] = [];
  for (const bone of asset.rig.bones as Json[]) {
    const b = state.bones[bone.id];
    frames[bone.id] = { o: v(b.frame.o), u: v(b.frame.u), v: v(b.frame.v), w: v(b.frame.w) };
    bones.push({ id: bone.id, parent: bone.parent ?? null, o: v(b.frame.o), tip: v(b.tip) });
  }
  return { frames, bones, state, query };
}

export const queryOf = (asset: Json, pose: string | undefined, clip: string | undefined, t: string | undefined): Query => (clip !== undefined ? { kind: "clip", clip, timeS: Number(t ?? 0) } : { kind: "pose", pose: pose ?? asset.presentation.defaultPose });

/** 🧊️ Solids tessellated through the shared cache and carried to the world by the given bone frames, at the product's quality of 1:`scale` (what the viewer and the 3D exports show at that scale) or, without a scale, at the reference quality of the catalog validator. */
export function meshesIn(asset: Json, frames: Record<string, Frame>, solids: readonly Json[], scale?: number): SolidMesh[] {
  const q = qualityOfAsset(asset, scale);
  return solids.map((solid) => {
    const f = frames[solid.bone];
    if (f === undefined) throw new Error(`solid "${solid.id}" is bound to unknown bone "${solid.bone}"`);
    const g = localGeometry(solid, q);
    const world = new Float64Array(g.positions.length);
    const blend = weightsOf(asset, frames, solid);
    for (let i = 0; i < g.positions.length; i += 3) {
      const [a, b, c] = [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!];
      if (blend === null) {
        for (let k = 0; k < 3; k++) world[i + k] = f.o[k]! + a * f.u[k]! + b * f.v[k]! + c * f.w[k]!;
        continue;
      }
      const r = blend.rest;
      const p: V3 = [r.o[0]! + a * r.u[0]! + b * r.v[0]! + c * r.w[0]!, r.o[1]! + a * r.u[1]! + b * r.v[1]! + c * r.w[1]!, r.o[2]! + a * r.u[2]! + b * r.v[2]! + c * r.w[2]!];
      for (const { rest, now, weight } of blend.joints) {
        const d = [p[0] - rest.o[0]!, p[1] - rest.o[1]!, p[2] - rest.o[2]!];
        const [x, y, z] = [d[0]! * rest.u[0]! + d[1]! * rest.u[1]! + d[2]! * rest.u[2]!, d[0]! * rest.v[0]! + d[1]! * rest.v[1]! + d[2]! * rest.v[2]!, d[0]! * rest.w[0]! + d[1]! * rest.w[1]! + d[2]! * rest.w[2]!];
        for (let k = 0; k < 3; k++) world[i + k] += weight * (now.o[k]! + x * now.u[k]! + y * now.v[k]! + z * now.w[k]!);
      }
    }
    return { id: solid.id, role: solid.role, world, indices: g.indices };
  });
}

const RESTS = new WeakMap<object, Record<string, Frame>>();

/** 🕸️ The linear blend of a solid with `weights`, as the product's mesh does it: the solid is placed in the rest frame of its bone, and every point goes to the sum over its weighted bones of that bone's posed frame applied to the point in the bone's rest frame (weights normalised, at most four); null for a rigid solid. */
function weightsOf(asset: Json, frames: Record<string, Frame>, solid: Json): { rest: Frame; joints: { rest: Frame; now: Frame; weight: number }[] } | null {
  const live = ((solid.weights ?? []) as { bone: string; weight: number }[]).filter((w) => w.weight > 0).slice(0, 4);
  if (live.length === 0) return null;
  let rest = RESTS.get(asset.rig);
  if (rest === undefined) {
    rest = Object.fromEntries(Object.entries(restFrames(asset.rig)).map(([id, solved]) => [id, solved.frame as Frame]));
    RESTS.set(asset.rig, rest);
  }
  const total = live.reduce((sum, w) => sum + w.weight, 0);
  return { rest: rest[solid.bone]!, joints: live.map((w) => ({ rest: rest![w.bone]!, now: frames[w.bone]!, weight: w.weight / total })) };
}

/** 📚️ The three lists of one authored scale as world meshes in the given frames; a list that does not tessellate is reported in `errors` and left empty. */
export function listsIn(asset: Json, scale: number, frames: Record<string, Frame>): ScaleMeshes & { errors: string[] } {
  const volume = asset.volumes[String(scale)];
  const unresolved: string[] = [];
  const errors: string[] = [];
  const solids = { body: resolveList(volume, "body"), thermal: resolveList(volume, "thermal", unresolved), structure: resolveList(volume, "structure", unresolved) } as Record<ListName, Json[]>;
  const build = (list: ListName): SolidMesh[] => {
    const out: SolidMesh[] = [];
    const kept: Json[] = [];
    for (const s of solids[list]) {
      try {
        out.push(meshesIn(asset, frames, [s], scale)[0]!);
        kept.push(s);
      } catch (error) {
        errors.push(`${list}/${s.id}: ${(error as Error).message}`);
      }
    }
    solids[list] = kept;
    return out;
  };
  const [body, thermal, structure] = [build("body"), build("thermal"), build("structure")];
  return { body, thermal, structure, solids, solidMeshes: { body, thermal, structure }, unresolved, errors };
}

export type Cam = { r: V3; u: V3; e: V3 };

/** 🎥️ An orthographic camera in the product's paper convention (x ahead, y up, z lateral; the elevation looks from -z with +x to the right of the paper): `turnDeg` 0 is the product's side elevation, 90 its front, 180 the other side, 270 the back; `tiltDeg` 90 is its plan. */
export function cameraAt(turnDeg: number, tiltDeg: number): Cam {
  const a = (turnDeg * Math.PI) / 180;
  const t = (tiltDeg * Math.PI) / 180;
  const h: V3 = [Math.sin(a), 0, -Math.cos(a)];
  return { r: [Math.cos(a), 0, Math.sin(a)], u: [-Math.sin(t) * h[0], Math.cos(t), -Math.sin(t) * h[2]], e: [Math.cos(t) * h[0], Math.sin(t), Math.cos(t) * h[2]] };
}

const dot3 = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;

export type Placement = { cam: Cam; scale: number; cx: number; cy: number; ox: number; oy: number };
export const toScreen = (p: Placement, w: readonly number[]): [number, number] => [p.ox + (dot3(w, p.cam.r) - p.cx) * p.scale, p.oy - (dot3(w, p.cam.u) - p.cy) * p.scale];

/** 📐️ The paper box of world points under a camera. */
export function paperBox(cam: Cam, points: Iterable<readonly number[]>): { minX: number; minY: number; maxX: number; maxY: number } {
  const box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const p of points) {
    const [x, y] = [dot3(p, cam.r), dot3(p, cam.u)];
    box.minX = Math.min(box.minX, x);
    box.maxX = Math.max(box.maxX, x);
    box.minY = Math.min(box.minY, y);
    box.maxY = Math.max(box.maxY, y);
  }
  return box;
}

export function* pointsOf(meshes: readonly SolidMesh[]): Generator<[number, number, number]> {
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) yield [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!];
}

const CREASE_COS = Math.cos((48 * Math.PI) / 180);

/** 🪡️ Corner normals of a mesh: vertices welded by position; each corner averages (area-weighted) only the faces at its vertex that meet its own face within the crease angle, so a flat cap stays flat beside a fillet and a crease stays sharp. */
export function cornerNormals(mesh: SolidMesh): Float32Array {
  const w = mesh.world;
  const tris = mesh.indices.length / 3;
  const face = new Float32Array(tris * 3);
  const raw = new Float32Array(tris * 3);
  const around = new Map<string, number[]>();
  const keys: string[] = new Array(w.length / 3);
  for (let i = 0; i < w.length / 3; i++) keys[i] = `${Math.round(w[i * 3]! * 2e4)},${Math.round(w[i * 3 + 1]! * 2e4)},${Math.round(w[i * 3 + 2]! * 2e4)}`;
  for (let t = 0; t < tris; t++) {
    const [a, b, c] = [mesh.indices[t * 3]! * 3, mesh.indices[t * 3 + 1]! * 3, mesh.indices[t * 3 + 2]! * 3];
    const [ux, uy, uz] = [w[b]! - w[a]!, w[b + 1]! - w[a + 1]!, w[b + 2]! - w[a + 2]!];
    const [vx, vy, vz] = [w[c]! - w[a]!, w[c + 1]! - w[a + 1]!, w[c + 2]! - w[a + 2]!];
    const n: [number, number, number] = [uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx];
    for (const corner of [a, b, c]) {
      const key = keys[corner / 3]!;
      (around.get(key) ?? around.set(key, []).get(key)!).push(t);
    }
    const l = Math.hypot(...n) || 1;
    raw[t * 3] = n[0];
    raw[t * 3 + 1] = n[1];
    raw[t * 3 + 2] = n[2];
    face[t * 3] = n[0] / l;
    face[t * 3 + 1] = n[1] / l;
    face[t * 3 + 2] = n[2] / l;
  }
  const out = new Float32Array(tris * 9);
  for (let t = 0; t < tris; t++) for (let k = 0; k < 3; k++) {
    const s = [0, 0, 0];
    for (const f of around.get(keys[mesh.indices[t * 3 + k]!]!)!) {
      if (face[f * 3]! * face[t * 3]! + face[f * 3 + 1]! * face[t * 3 + 1]! + face[f * 3 + 2]! * face[t * 3 + 2]! < CREASE_COS) continue;
      s[0] += raw[f * 3]!;
      s[1] += raw[f * 3 + 1]!;
      s[2] += raw[f * 3 + 2]!;
    }
    const l = Math.hypot(s[0]!, s[1]!, s[2]!);
    for (let q = 0; q < 3; q++) out[t * 9 + k * 3 + q] = l > 0 ? s[q]! / l : face[t * 3 + q]!;
  }
  return out;
}

export type Item = { mesh: SolidMesh; color: RGB };
/** 🎚️ Depth tie (m): a surface within this distance behind the stored one still wins, so coincident faces of neighbouring solids resolve to the later one instead of a float-noise moiré. */
const TIE_M = 1e-5;
const normalCache = new WeakMap<SolidMesh, Float32Array>();

/** 🖌️ A deferred orthographic panel: triangles go into depth, normal, albedo and solid-id buffers at `aa` times the size; `shade` lights them (two-sided, light from the upper left of the viewer) and draws outlines at depth steps, seams between solids and creases; `canvas` returns the downsampled picture. */
export class Panel {
  readonly W: number;
  readonly H: number;
  readonly depth: Float32Array;
  readonly normal: Float32Array;
  readonly albedo: Uint8ClampedArray;
  readonly solid: Int32Array;
  readonly image: Uint8ClampedArray;
  private next = 1;
  constructor(readonly width: number, readonly height: number, readonly background: RGB = [247, 247, 244], readonly aa = 2) {
    this.W = width * aa;
    this.H = height * aa;
    this.depth = new Float32Array(this.W * this.H).fill(-Infinity);
    this.normal = new Float32Array(this.W * this.H * 3);
    this.albedo = new Uint8ClampedArray(this.W * this.H * 3);
    this.solid = new Int32Array(this.W * this.H);
    this.image = new Uint8ClampedArray(this.W * this.H * 3);
    for (let i = 0; i < this.W * this.H; i++) for (let k = 0; k < 3; k++) this.image[i * 3 + k] = background[k]!;
  }

  place(p: Placement): Placement {
    return { ...p, scale: p.scale * this.aa, ox: p.ox * this.aa, oy: p.oy * this.aa };
  }

  draw(placement: Placement, items: readonly Item[], clipDepth = Infinity): void {
    const p = this.place(placement);
    const { r, u, e } = p.cam;
    for (const { mesh, color } of items) {
      const id = this.next++;
      const w = mesh.world;
      const n = w.length / 3;
      const sx = new Float32Array(n);
      const sy = new Float32Array(n);
      const sd = new Float32Array(n);
      for (let i = 0; i < n; i++) {
        const [x, y, z] = [w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!];
        sx[i] = p.ox + (x * r[0] + y * r[1] + z * r[2] - p.cx) * p.scale;
        sy[i] = p.oy - (x * u[0] + y * u[1] + z * u[2] - p.cy) * p.scale;
        sd[i] = x * e[0] + y * e[1] + z * e[2];
      }
      const normals = normalCache.get(mesh) ?? normalCache.set(mesh, cornerNormals(mesh)).get(mesh)!;
      for (let t = 0; t < mesh.indices.length; t += 3) {
        const [a, b, c] = [mesh.indices[t]!, mesh.indices[t + 1]!, mesh.indices[t + 2]!];
        const [ax, ay, bx, by, cx, cy] = [sx[a]!, sy[a]!, sx[b]!, sy[b]!, sx[c]!, sy[c]!];
        const den = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy);
        if (Math.abs(den) < 1e-9) continue;
        const minX = Math.max(0, Math.floor(Math.min(ax, bx, cx)));
        const maxX = Math.min(this.W - 1, Math.ceil(Math.max(ax, bx, cx)));
        const minY = Math.max(0, Math.floor(Math.min(ay, by, cy)));
        const maxY = Math.min(this.H - 1, Math.ceil(Math.max(ay, by, cy)));
        for (let y = minY; y <= maxY; y++) for (let x = minX; x <= maxX; x++) {
          const l0 = ((by - cy) * (x + 0.5 - cx) + (cx - bx) * (y + 0.5 - cy)) / den;
          const l1 = ((cy - ay) * (x + 0.5 - cx) + (ax - cx) * (y + 0.5 - cy)) / den;
          const l2 = 1 - l0 - l1;
          if (l0 < -1e-6 || l1 < -1e-6 || l2 < -1e-6) continue;
          const d = l0 * sd[a]! + l1 * sd[b]! + l2 * sd[c]!;
          const i = y * this.W + x;
          if (d > clipDepth || d < this.depth[i]! - TIE_M) continue;
          this.depth[i] = d;
          this.solid[i] = id;
          for (let q = 0; q < 3; q++) {
            this.normal[i * 3 + q] = l0 * normals[t * 3 + q]! + l1 * normals[t * 3 + 3 + q]! + l2 * normals[t * 3 + 6 + q]!;
            this.albedo[i * 3 + q] = color[q]!;
          }
        }
      }
    }
  }

  shade(placement: Placement): void {
    const { r, u, e } = placement.cam;
    const lv = [-0.5, 0.72, 0.62];
    const ll = Math.hypot(...lv);
    const light = [0, 1, 2].map((k) => (lv[0]! * r[k]! + lv[1]! * u[k]! + lv[2]! * e[k]!) / ll);
    const { W, H } = this;
    const n = new Float32Array(W * H * 3);
    for (let i = 0; i < W * H; i++) {
      if (this.solid[i] === 0) continue;
      let [x, y, z] = [this.normal[i * 3]!, this.normal[i * 3 + 1]!, this.normal[i * 3 + 2]!];
      const l = Math.hypot(x, y, z) || 1;
      const facing = (x * e[0] + y * e[1] + z * e[2]) / l;
      const s = facing < 0 ? -1 / l : 1 / l;
      x *= s;
      y *= s;
      z *= s;
      n[i * 3] = x;
      n[i * 3 + 1] = y;
      n[i * 3 + 2] = z;
      const k = (0.36 + 0.64 * Math.max(0, x * light[0]! + y * light[1]! + z * light[2]!)) * (0.8 + 0.2 * Math.abs(facing));
      for (let q = 0; q < 3; q++) this.image[i * 3 + q] = this.albedo[i * 3 + q]! * k;
    }
    const step = 7 / (placement.scale * this.aa);
    const edge = new Uint8Array(W * H);
    const creaseCos = Math.cos((32 * Math.PI) / 180);
    const pair = (i: number, j: number) => {
      const [a, b] = [this.solid[i]!, this.solid[j]!];
      if (a === 0 && b === 0) return;
      let kind = 0;
      if (a === 0 || b === 0 || Math.abs(this.depth[i]! - this.depth[j]!) > step) kind = 3;
      else if (a !== b) kind = 2;
      else if (n[i * 3]! * n[j * 3]! + n[i * 3 + 1]! * n[j * 3 + 1]! + n[i * 3 + 2]! * n[j * 3 + 2]! < creaseCos) kind = 1;
      if (kind === 0) return;
      const near = a === 0 ? j : b === 0 ? i : this.depth[i]! >= this.depth[j]! ? i : j;
      if (edge[near]! < kind) edge[near] = kind;
      if (kind === 3) {
        const far = near === i ? j : i;
        if (edge[far]! < 3) edge[far] = 3;
      }
    };
    for (let y = 0; y < H; y++) for (let x = 0; x < W; x++) {
      const i = y * W + x;
      if (x + 1 < W) pair(i, i + 1);
      if (y + 1 < H) pair(i, i + W);
    }
    const mix = [1, 0.62, 0.5, 0.12];
    for (let i = 0; i < W * H; i++) if (edge[i]! > 0) for (let q = 0; q < 3; q++) this.image[i * 3 + q] = (this.solid[i] === 0 ? 40 : this.image[i * 3 + q]!) * (this.solid[i] === 0 ? 1 : mix[edge[i]!]!);
  }

  /** 👻️ Fades what is shaded so far towards the background and forgets its geometry, so that the next draw lies over a ghost. */
  ghost(keep = 0.25): void {
    for (let i = 0; i < this.W * this.H; i++) {
      for (let q = 0; q < 3; q++) this.image[i * 3 + q] = this.background[q]! - (this.background[q]! - this.image[i * 3 + q]!) * keep;
    }
    this.depth.fill(-Infinity);
    this.solid.fill(0);
  }

  canvas(): Canvas {
    const big = new Canvas(this.W, this.H);
    big.data.set(this.image);
    return this.aa === 1 ? big : big.downsample(this.aa);
  }
}

/** ➖️ A line of the given thickness on a canvas. */
export function thickLine(c: Canvas, x0: number, y0: number, x1: number, y1: number, color: RGB, width = 2): void {
  const h = (width - 1) / 2;
  for (let dy = -Math.floor(h); dy <= Math.ceil(h); dy++) for (let dx = -Math.floor(h); dx <= Math.ceil(h); dx++) c.line(x0 + dx, y0 + dy, x1 + dx, y1 + dy, color);
}

export function dot(c: Canvas, x: number, y: number, radius: number, color: RGB, rim: RGB | null = null): void {
  for (let j = -radius - 1; j <= radius + 1; j++) for (let i = -radius - 1; i <= radius + 1; i++) {
    const d = Math.hypot(i, j);
    if (d <= radius) c.set(Math.round(x) + i, Math.round(y) + j, color);
    else if (rim !== null && d <= radius + 1) c.set(Math.round(x) + i, Math.round(y) + j, rim);
  }
}

type Rect = { x: number; y: number; w: number; h: number };
const overlaps = (a: Rect, b: Rect): boolean => a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;

/** 🏷️ Places small labels next to their points without overlap: each label tries eight positions around its point and is dropped when none is free; returns how many were dropped. */
export function placeLabels(c: Canvas, clip: Rect, labels: readonly { x: number; y: number; text: string }[], color: RGB, scale = 2, taken: Rect[] = []): number {
  let dropped = 0;
  for (const l of labels) {
    const w = l.text.length * 6 * scale + 3;
    const h = 7 * scale + 4;
    const tries: [number, number][] = [[6, -h / 2], [-w - 6, -h / 2], [-w / 2, -h - 6], [-w / 2, 6], [6, -h - 4], [6, 4], [-w - 6, -h - 4], [-w - 6, 4], [14, -h - 14], [14, 14], [-w - 14, -h - 14], [-w - 14, 14]];
    let done = false;
    for (const [dx, dy] of tries) {
      const box = { x: Math.round(l.x + dx), y: Math.round(l.y + dy), w, h };
      if (box.x < clip.x || box.y < clip.y || box.x + w > clip.x + clip.w || box.y + h > clip.y + clip.h || taken.some((t) => overlaps(t, box))) continue;
      c.rect(box.x, box.y, w, h, [255, 255, 255]);
      c.text(box.x + 2, box.y + 2, l.text, color, scale);
      taken.push(box);
      done = true;
      break;
    }
    if (!done) dropped++;
  }
  return dropped;
}
