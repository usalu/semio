/** 🧩️ Prop builders of HW-14: separable bone-local rescaling of a carried object in all drawings and volumes, and flat or tubular props drawn once in the bone frame and placed in the three views by a `solid`.
 * @see ./lib.ts */
import { type Json, type Frame, SCALES, VIEWS, LISTS, round } from "./lib.ts";

type Map1 = (x: number) => number;

/** 📐️ Rescales the geometry of `bone` (axis-aligned rest frame `f`) by the separable maps `U`, `V`, `W` of its local axes; `which(id)` says per shape or solid id which axes apply ("uvw", "uw", "" to skip). Elevation shapes are bone-local, front and plan shapes world rest coordinates. */
export function rescaleBone(asset: Json, bone: string, f: Frame, U: Map1, V: Map1, W: Map1, which: (id: string) => string): void {
  const axes = (id: string) => which(id);
  const pick = (m: Map1, on: boolean) => (on ? m : (x: number) => x);
  const toLocal = (p: { x: number; y: number; z: number }) => {
    const d = { x: p.x - f.o.x, y: p.y - f.o.y, z: p.z - f.o.z };
    return [d.x * f.u.x + d.y * f.u.y + d.z * f.u.z, d.x * f.v.x + d.y * f.v.y + d.z * f.v.z, d.x * f.w.x + d.y * f.w.y + d.z * f.w.z];
  };
  const toWorld = (u: number, v: number, w: number) => ({ x: f.o.x + f.u.x * u + f.v.x * v + f.w.x * w, y: f.o.y + f.u.y * u + f.v.y * v + f.w.y * w, z: f.o.z + f.u.z * u + f.v.z * v + f.w.z * w });
  for (const k of SCALES) {
    const r = asset.representations[k];
    for (const view of VIEWS) for (const s of r.geometry[view] ?? []) {
      if (s.bone !== bone) continue;
      const a = axes(s.id);
      if (!a) continue;
      if (s.solid) throw new Error(`rescaleBone: ${s.id} has a solid`);
      const [mu, mv, mw] = [pick(U, a.includes("u")), pick(V, a.includes("v")), pick(W, a.includes("w"))];
      const map = (p: number[]): number[] => {
        if (view === "elevation") return [round(mu(p[0]!)), round(mv(p[1]!))];
        const world = view === "front" ? { x: f.o.x, y: p[1]!, z: p[0]! } : { x: p[0]!, y: f.o.y, z: p[1]! };
        const [u, v, w] = toLocal(world);
        const q = toWorld(mu(u!), mv(v!), mw(w!));
        return view === "front" ? [round(q.z), round(q.y)] : [round(q.x), round(q.z)];
      };
      if (s.kind === "poly") s.points = s.points.map(map);
      else throw new Error(`rescaleBone: shape kind ${s.kind}`);
    }
    const vol = asset.volumes[k];
    for (const l of LISTS) for (const s of vol[l]) {
      if (s.bone !== bone || s.of) continue;
      const a = axes(s.id);
      if (!a) continue;
      const [mu, mv, mw] = [pick(U, a.includes("u")), pick(V, a.includes("v")), pick(W, a.includes("w"))];
      const sc = (m: Map1, c: number, h: number) => m(c + h) - m(c - h);
      if (s.kind === "box" || s.kind === "ellipsoid") {
        const t = s.transform?.translation ?? [0, 0, 0];
        const ext = s.kind === "box" ? s.size.map((x: number) => x / 2) : s.radii;
        const n = [sc(mu, t[0], ext[0]) / 2, sc(mv, t[1], ext[1]) / 2, sc(mw, t[2], ext[2]) / 2];
        s.transform = { ...(s.transform ?? {}), translation: [round(mu(t[0])), round(mv(t[1])), round(mw(t[2]))] };
        if (s.kind === "box") s.size = n.map((x) => round(2 * x)); else s.radii = n.map((x) => round(x));
      } else if (s.kind === "extrude" && s.plane === "xy") {
        s.profile = s.profile.map((p: number[]) => [round(mu(p[0]!)), round(mv(p[1]!))]);
        [s.from, s.to] = [round(mw(s.from)), round(mw(s.to))];
      } else throw new Error(`rescaleBone: solid ${s.id} ${s.kind}`);
    }
  }
}

/** ✉️ A flat rectangular prop in the (u, v) plane of its bone: the same bone-local outline in the three views, extruded laterally about the rest position `zRest` by `thickness`. */
export function flatShape(id: string, bone: string, z: number, tone: number, region: string, material: string, points: number[][], zRest: number, thickness: number, role = "body"): Json {
  return { id, bone, role, z, filled: true, tone, region, material, kind: "poly", points: points.map((p) => p.map((x) => round(x))), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(zRest - thickness / 2, 5), z1: round(zRest + thickness / 2, 5) } };
}

/** 🥢️ A round prop (tube, rod) about the u axis of its bone: the half profile `[u, r]` drawn in all three views and revolved about the bone axis. */
export function revolvedShape(id: string, bone: string, z: number, tone: number, region: string, material: string, half: number[][], role = "body"): Json {
  const pts = [...half.map(([u, r]) => [round(u!), round(r!)])];
  return { id, bone, role, z, filled: true, tone, region, material, kind: "poly", points: [[pts[0]![0], 0], ...pts, [pts[pts.length - 1]![0], 0]], closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0, mirror: true } };
}

/** 📦️ Places a new shape in a view list before the first shape with z above it (painting order by z). */
export function addShape(list: Json[], shape: Json): void {
  const i = list.findIndex((s) => s.z > shape.z);
  if (i < 0) list.push(shape); else list.splice(i, 0, shape);
}

export { SCALES, VIEWS, LISTS };

/** 💪️ Makes the drawings of a limb pose-true in front and plan: every front and plan shape of the given limb bones is replaced by the elevation outline of the same id revolved about its bone axis, and every shape of the given hand bones by the elevation outline extruded laterally over the rest span `[z0, z1]` of its volume. */
export function poseTrueLimb(asset: Json, scales: readonly string[], limbBones: string[], handBones: Record<string, Record<string, [number, number]>>, axes: Record<string, Record<string, [number, number, number?]>> = {}, fromU: Record<string, number> = {}): void {
  for (const k of scales) {
    const g = asset.representations[k].geometry;
    const byId = new Map(g.elevation.map((s: Json) => [s.id, s]));
    for (const view of ["front", "plan"] as const) {
      g[view] = g[view].map((s: Json) => {
        const e = byId.get(s.id) as Json | undefined;
        if (!e || e.bone !== s.bone) return s;
        if (limbBones.includes(s.bone)) {
          const [ax, az, ratio] = axes[s.bone]?.[k] ?? [0, 0, 1];
          const c = JSON.parse(JSON.stringify(e));
          if (fromU[s.bone] !== undefined) c.points = clipHalf(c.points, 0, fromU[s.bone]!, "above");
          if (c.kind === "poly" && ratio !== undefined && ratio !== 1) c.points = c.points.map((p: number[]) => [p[0], round(ax + (p[1]! - ax) * ratio, 5)]);
          return { ...c, z: s.z, solid: { kind: "revolved", axisBone: s.bone, axis: "u", axisX: axes[s.bone]?.[k]?.[0] ?? 0, axisZ: axes[s.bone]?.[k]?.[1] ?? 0, mirror: true } };
        }
        const span = handBones[s.bone]?.[k];
        if (span) return { ...JSON.parse(JSON.stringify(e)), z: s.z, solid: { kind: "extruded", sweep: "w", z0: span[0], z1: span[1] } };
        return s;
      });
    }
  }
}

/** ✂️ Clips a polygon to the half-plane `y >= c` (keep "above") or `y <= c` (keep "below") along the given axis index (0 = first, 1 = second coordinate). */
export function clipHalf(points: number[][], axis: 0 | 1, c: number, keep: "above" | "below"): number[][] {
  const inside = (p: number[]) => (keep === "above" ? p[axis]! >= c : p[axis]! <= c);
  const out: number[][] = [];
  for (let i = 0; i < points.length; i++) {
    const [p, q] = [points[i]!, points[(i + 1) % points.length]!];
    const [ip, iq] = [inside(p), inside(q)];
    if (ip) out.push(p);
    if (ip !== iq) { const t = (c - p[axis]!) / (q[axis]! - p[axis]!); out.push([p[0]! + t * (q[0]! - p[0]!), p[1]! + t * (q[1]! - p[1]!)]); }
  }
  return out.map((p) => p.map((x) => round(x)));
}

/** 🪡️ Splits a garment outline drawn on one upright spine bone into pieces carried by the spine bones they cover: `cuts` are rest heights (world y) from the top down with the bone below each cut and its rest origin height; the pieces overlap by `lap` metres so that no slit opens between them. Elevation coordinates are bone-local (u up), front coordinates world rest (y up). */
export function splitUpright(list: Json[], id: string, view: "elevation" | "front", boneOriginY: Record<string, number>, cuts: { y: number; bone: string }[], lap = 0.003): void {
  const i = list.findIndex((s) => s.id === id);
  if (i < 0) return;
  const s = list[i]!;
  if (s.kind !== "poly" || s.solid) throw new Error(`splitUpright ${id}`);
  const y0 = view === "elevation" ? boneOriginY[s.bone]! : 0;
  const world = s.points.map((p: number[]) => [p[0]!, p[1]! + 0, p[0]!]).map((p: number[]) => (view === "elevation" ? [p[0]!, p[0]! + y0] : p));
  void world;
  const toY = (p: number[]) => (view === "elevation" ? p[0]! + y0 : p[1]!);
  const axis: 0 | 1 = view === "elevation" ? 0 : 1;
  const pieces: Json[] = [];
  let above = s.points as number[][];
  let upper = { bone: s.bone, oy: y0 };
  for (const cut of cuts) {
    const c = view === "elevation" ? cut.y - upper.oy : cut.y;
    const top = clipHalf(above, axis, c - lap, "above");
    const rest = clipHalf(above, axis, c + lap, "below");
    if (top.length >= 3) pieces.push({ ...s, bone: upper.bone, points: top });
    const noy = boneOriginY[cut.bone]!;
    above = view === "elevation" ? rest.map((p) => [round(p[0]! + upper.oy - noy), p[1]!]) : rest;
    upper = { bone: cut.bone, oy: noy };
  }
  if (above.length >= 3) pieces.push({ ...s, bone: upper.bone, points: above });
  void toY;
  pieces.forEach((p, k) => { p.id = k === 0 ? s.id : `${s.id}-${k === pieces.length - 1 ? "skirt" : "waist"}`; p.z = round(s.z + k * 0.001, 4); });
  list.splice(i, 1, ...pieces);
}

import { traceOutline } from "./lib.ts";
/** 🩻️ Redraws one shape of a view and scale as the projected outline of the body solids `keep` selects in the presentation pose (front and plan coordinates are rest-world coordinates; use for shapes whose bone stands at rest in that view in the presentation pose). */
export function retrace(asset: Json, scale: string, view: "elevation" | "front" | "plan", id: string, keep: (solid: Json) => boolean, tol = 0.0015): void {
  const s = asset.representations[scale].geometry[view].find((x: Json) => x.id === id);
  if (!s) throw new Error(`retrace: no ${id} in ${scale} ${view}`);
  const rings = traceOutline(asset, scale, view, keep, tol);
  if (!rings.length) throw new Error(`retrace: nothing for ${id}`);
  if (view === "elevation") {
    const f = frames(asset, asset.presentation.defaultPose)[s.bone]!;
    s.points = rings[0]!.map(([x, y]) => { const [dx, dy] = [x! - f.o.x, y! - f.o.y]; return [round(dx * f.u.x + dy * f.u.y), round(dx * f.v.x + dy * f.v.y)]; });
  } else s.points = rings[0];
  if (!s.filled || !s.closed) { s.filled = true; s.closed = true; delete s.penMm; if (asset.representations[scale].pens[s.role] == null) s.role = "body"; }
  delete s.solid;
}

import { frames } from "./lib.ts";
/** 🩹️ Adds a filled patch given in world view coordinates of the presentation pose (elevation: x, y converted into the bone frame of that pose; front and plan: rest-world coordinates as drawn) that closes a slit or notch the volume does not have; it copies tone, region and material of the shape `like` of the same view. */
export function patch(asset: Json, scale: string, view: "elevation" | "front" | "plan", id: string, bone: string, like: string, world: number[][]): void {
  const list = asset.representations[scale].geometry[view];
  const ref = list.find((s: Json) => s.id === like);
  if (!ref) throw new Error(`patch: no ${like}`);
  let pts = world;
  if (view === "elevation") {
    const f = frames(asset, asset.presentation.defaultPose)[bone]!;
    pts = world.map(([x, y]) => { const [dx, dy] = [x! - f.o.x, y! - f.o.y]; return [round(dx * f.u.x + dy * f.u.y), round(dx * f.v.x + dy * f.v.y)]; });
  }
  const shape: Json = { id, bone, role: "body", z: ref.z - 0.0005, filled: true, tone: ref.tone, kind: "poly", points: pts, closed: true };
  if (ref.region) shape.region = ref.region;
  if (ref.regions) shape.regions = ref.regions;
  if (ref.material) shape.material = ref.material;
  addShape(list, shape);
}
