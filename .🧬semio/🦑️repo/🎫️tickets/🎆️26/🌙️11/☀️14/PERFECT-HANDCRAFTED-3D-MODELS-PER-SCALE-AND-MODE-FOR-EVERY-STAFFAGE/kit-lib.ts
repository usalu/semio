/** 🧰️ Shared authoring kit: context, source I/O and the solid primitives every author needs (loft from outline, superellipse section, shell offset, lobe stack, lattice bar set, envelope prism, hull loft from a scale's own drawings).
 *
 * Builds on `volumes-lib.ts` (poses, meshes, drawn shapes) and the product `measureVolumes`; it never re-implements a measurement. Every function is pure unless it is named `open*` or `save*`.
 * Frames: coordinates are metres in the bone-local frame (u along the bone, v sagittal normal, w lateral). A loft whose sections carry no rotation runs along its path: for a path along u `rx` is lateral (w) and `ry` sagittal (v); along v `rx` is u and `ry` is w; along w `rx` is u and `ry` is v (`LOFT_AXES`).
 * @see ./kit.md
 * @see ./design.md */
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { GENERATED, VIEWS, entryOf, featureM, loadSource, merged, poseState, sourcePath, type Json, type P2, type PoseState, type Ring, type V3, type View } from "./volumes-lib.ts";
import { seed } from "./volumes-seed.ts";

export { VIEWS, featureM, type Json, type P2, type Ring, type V3, type View };

export const SCALES = [50, 100, 200, 500] as const;
export const KIT_OUT = join(GENERATED, "kit");
export const DRAWN_ROLES: ReadonlySet<string> = new Set(["body", "detail", "glazing"]);
export const LOFT_AXES = { u: { rx: "w", ry: "v" }, v: { rx: "u", ry: "w" }, w: { rx: "u", ry: "v" } } as const;
export type Axis = "u" | "v" | "w";
export type ListName = "body" | "thermal" | "structure";

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
/** 🔢️ Rounds every number of a value to 0.01 mm. */
export const rnd = <T>(v: T): T => (typeof v === "number" ? (r5(v) as T) : Array.isArray(v) ? (v.map(rnd) as T) : v !== null && typeof v === "object" ? (Object.fromEntries(Object.entries(v).map(([k, x]) => [k, rnd(x)])) as T) : v);
export const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v));

export const flag = (args: readonly string[], name: string, fallback?: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : fallback);
export const has = (args: readonly string[], name: string): boolean => args.includes(name);
export const positional = (args: readonly string[], valued: readonly string[]): string | undefined => args.find((a, i) => !a.startsWith("--") && !valued.includes(args[i - 1] ?? ""));

export type Ctx = { spec: string; raw: Json; source: Json; m: Json; scale: number; vol: Json; feature: number; state: PoseState; file: string; seeded: boolean };

/** 📂️ Opens one scale of an asset: the raw asset, the authoring source (seeded in memory when absent), the asset with the source merged over it (`m`, what every measurement takes) and the evaluated default pose.
 * @example const c = openCtx("humans/jogger", 100); c.feature // 0.015 */
export function openCtx(spec: string, scale: number, sourceFile?: string): Ctx {
  const { entry, asset } = entryOf(spec);
  const file = sourceFile ?? sourcePath(entry);
  let source: Json | null = sourceFile === undefined ? loadSource(entry) : JSON.parse(readFileSync(file, "utf8"));
  const seeded = source === null;
  if (source === null) source = seed(asset);
  const vol = source.volumes[String(scale)];
  if (vol === undefined || vol.status !== "authored") throw new Error(`1:${scale} of ${spec} is not authored (${vol?.status ?? "missing"})`);
  for (const l of ["body", "thermal", "structure", "decisions"] as const) vol[l] ??= [];
  const m = merged(asset, source);
  return { spec, raw: asset, source, m, scale, vol, feature: featureM(scale, asset), state: poseState(m, scale), file, seeded };
}

/** 💾️ Writes lists of one scale back: re-reads the source file right before so another author's edits of other scales survive, replaces only the given lists, writes atomically and compactly. */
export function saveLists(c: Ctx, lists: Partial<Record<ListName | "decisions", Json[]>>): void {
  const fresh: Json = existsSync(c.file) ? JSON.parse(readFileSync(c.file, "utf8")) : c.source;
  const v = fresh.volumes[String(c.scale)];
  for (const [k, list] of Object.entries(lists)) {
    v[k] = rnd(list);
    c.vol[k] = v[k];
  }
  mkdirSync(dirname(c.file), { recursive: true });
  const tmp = `${c.file}.kit-${process.pid}.tmp`;
  writeFileSync(tmp, JSON.stringify(fresh));
  renameSync(tmp, c.file);
}

/** 📤️ Writes a tool output under `🗑️generated/kit/<name>` and returns its path. */
export function writeOut(name: string, data: string | Uint8Array): string {
  const path = join(KIT_OUT, name);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, data);
  return path;
}

export const log = (msg: string): void => void process.stderr.write(`[kit] ${msg}\n`);

/** ✏️ The closed drawn shapes (roles body, detail, glazing) of one view in the default pose, optionally of one bone. */
export function drawn(c: Ctx, view: View, bone?: string): { id: string; bone: string; region?: string; role: string; rings: Ring[] }[] {
  return (c.state[view] as Json[])
    .filter((s) => s.closed && s.rings.length > 0 && DRAWN_ROLES.has(s.shape.role) && (bone === undefined || s.shape.bone === bone))
    .map((s) => ({ id: s.shape.id, bone: s.shape.bone, region: s.shape.region, role: s.shape.role, rings: s.rings }));
}

export const mean = (xs: readonly number[]): number => (xs.length === 0 ? 0 : xs.reduce((a, b) => a + b, 0) / xs.length);

/* ---------------------------------------------------------------- primitives */

export type Section = { at: V3; rx: number; ry: number; exponent?: number; rotation?: [number, number, number, number] };
export type Common = { id: string; bone: string; material: string; tone?: number; role?: string };
const base = (p: Common, kind: string): Json => ({ id: p.id, kind, bone: p.bone, material: p.material, role: p.role ?? "body", tone: p.tone ?? 0.5 });

/** 〰️ A superelliptic loft section: |x/rx|^e + |y/ry|^e = 1 (e 2 ellipse, 3-5 rounded rectangle, 1 diamond) centred at `at`.
 * @example superellipseSection([0.2, 0, 0], 0.09, 0.05, 2.6) */
export const superellipseSection = (at: V3, rx: number, ry: number, exponent = 2, rotation?: Section["rotation"]): Section => ({ at, rx, ry, exponent, ...(rotation === undefined ? {} : { rotation }) });

/** 🔵️ Points of a superellipse (counter-clockwise, `n` points) for outlines and `extrude` profiles.
 * @example superellipsePoints(0.1, 0.05, 4, 24) */
export function superellipsePoints(rx: number, ry: number, exponent = 2, n = 24): [number, number][] {
  return Array.from({ length: n }, (_, i) => {
    const t = (2 * Math.PI * i) / n;
    const c = Math.cos(t);
    const s = Math.sin(t);
    return [Math.sign(c) * Math.abs(c) ** (2 / exponent) * rx, Math.sign(s) * Math.abs(s) ** (2 / exponent) * ry] as [number, number];
  });
}

/** 🎢️ A loft through sections (centres give the path). Without `rotation` a section is perpendicular to the path (see `LOFT_AXES`).
 * @example loftOf({ id: "thigh", bone: "thigh-l", material: "skin" }, [superellipseSection([0, 0, 0], 0.07, 0.08), superellipseSection([0.44, 0, 0], 0.05, 0.05)]) */
export const loftOf = (p: Common, sections: readonly Section[], caps = true): Json => ({ ...base(p, "loft"), sections: sections.map((s) => ({ ...s, exponent: s.exponent ?? 2 })), capStart: caps, capEnd: caps });

/** 📏️ A loft along bone axis `axis` through stations `{ s, c1, c2, r1, r2, e }`: `s` the position along the axis, `c1`/`c2` the section centre on the other two axes (in alphabetical u,v,w order), `r1`/`r2` the half extents along `LOFT_AXES[axis].rx` and `.ry`.
 * @example stationLoft({ id: "hull", bone: "chassis", material: "steel" }, "u", [{ s: 0, c1: 0.9, c2: 0, r1: 0.9, r2: 0.7 }, { s: 5, c1: 0.9, c2: 0, r1: 0.9, r2: 0.7 }]) */
export type Station = { s: number; c1?: number; c2?: number; rx: number; ry: number; e?: number };
export function stationLoft(p: Common, axis: Axis, stations: readonly Station[]): Json {
  const at = (s: Station): V3 => {
    const c1 = s.c1 ?? 0;
    const c2 = s.c2 ?? 0;
    return axis === "u" ? [s.s, c1, c2] : axis === "v" ? [c1, s.s, c2] : [c1, c2, s.s];
  };
  return loftOf(p, stations.map((s) => superellipseSection(at(s), s.rx, s.ry, s.e ?? 2)));
}

/** 🧬️ A loft from an outline: `outline` is a closed polygon in the plane (axis, `across`) (the drawn silhouette of a part: for axis u the elevation outline (u, v)); stations along the axis take the outline's extent across it, with `depth` giving the half extent along the missing third axis. Concave outlines use their outer hull per station.
 * @example loftFromOutline({ id: "body", bone: "chassis", material: "steel" }, [{x:0,y:0},{x:4,y:0},{x:4,y:1},{x:0,y:1}], { axis: "u", across: "v", depth: () => 0.9, stations: 8 }) */
export function loftFromOutline(p: Common, outline: readonly P2[], o: { axis: "u"; across: "v"; depth: (s: number) => number; stations?: number; e?: number; tol?: number; depthCentre?: (s: number) => number }): Json {
  const xs = outline.map((q) => q.x);
  const lo = Math.min(...xs);
  const hi = Math.max(...xs);
  const n = o.stations ?? 24;
  const raw: Station[] = [];
  for (let i = 0; i < n; i++) {
    const s = lo + ((hi - lo) * i) / (n - 1);
    const iv = outlineSpan(outline, s);
    if (iv === null) continue;
    raw.push({ s, c1: (iv[0] + iv[1]) / 2, c2: o.depthCentre?.(s) ?? 0, rx: Math.max(o.depth(s), 1e-4), ry: Math.max((iv[1] - iv[0]) / 2, 1e-4), e: o.e ?? 2.6 });
  }
  return stationLoft(p, o.axis, o.tol === undefined ? raw : selectStations(raw, o.tol));
}

/** ↕️ Min and max of an outline at x = s (outer hull of the crossings). */
export function outlineSpan(outline: readonly P2[], s: number): [number, number] | null {
  const ys: number[] = [];
  for (let i = 0, k = outline.length - 1; i < outline.length; k = i++) {
    const a = outline[k]!;
    const b = outline[i]!;
    if (a.x <= s !== b.x <= s) ys.push(a.y + ((s - a.x) / (b.x - a.x)) * (b.y - a.y));
  }
  return ys.length < 2 ? null : [Math.min(...ys), Math.max(...ys)];
}

/** ✂️ Greedy station selection: keeps the station whose omission would err most until every omitted station is within `tol` (metres) of the interpolation of the kept ones (Douglas-Peucker on centre and radii).
 * @example selectStations(dense, 0.0075) */
export function selectStations(st: readonly Station[], tol: number, fmin = 0): Station[] {
  const n = st.length;
  if (n <= 2) return st.map((s) => ({ ...s }));
  const keep = new Set([0, n - 1]);
  const at = (k: readonly number[], s: number): Station => {
    let j = 0;
    while (j < k.length - 2 && st[k[j + 1]!]!.s < s) j++;
    const a = st[k[j]!]!;
    const b = st[k[j + 1]!]!;
    const t = b.s === a.s ? 0 : Math.min(1, Math.max(0, (s - a.s) / (b.s - a.s)));
    const l = (x = 0, y = 0) => x + (y - x) * t;
    return { s, c1: l(a.c1, b.c1), c2: l(a.c2, b.c2), rx: l(a.rx, b.rx), ry: l(a.ry, b.ry) };
  };
  for (;;) {
    const ks = [...keep].sort((p, q) => p - q);
    let worst = 0;
    let wi = -1;
    for (let i = 0; i < n; i++) {
      if (keep.has(i)) continue;
      const p = at(ks, st[i]!.s);
      const q = st[i]!;
      const err = Math.max(Math.abs((p.c1 ?? 0) - (q.c1 ?? 0)), Math.abs((p.c2 ?? 0) - (q.c2 ?? 0)), Math.abs(p.rx - q.rx), Math.abs(p.ry - q.ry));
      if (err > worst) [worst, wi] = [err, i];
    }
    if (wi < 0 || worst <= tol) break;
    keep.add(wi);
  }
  return [...keep].sort((p, q) => p - q).map((i) => ({ ...st[i]!, rx: Math.max(st[i]!.rx, fmin), ry: Math.max(st[i]!.ry, fmin) }));
}

/** 🧅️ The solid inset by `t` metres (thermal layer shell, structural core) as a new solid with id `id`; box, ellipsoid, capsule, revolve, extrude and loft; `null` when it would vanish.
 * @example shellOffset(solid, 0.004, "th-shirt-torso") */
export function shellOffset(solid: Json, t: number, id: string, floor = 1e-3): Json | null {
  const s = clone(solid);
  s.id = id;
  delete s.region;
  delete s.layer;
  const d = (x: number): number => x - t;
  switch (s.kind) {
    case "box":
      s.size = s.size.map((x: number) => x - 2 * t);
      if (s.size.some((x: number) => x < floor)) return null;
      if (s.cornerRadius !== undefined) s.cornerRadius = Math.max(0, Math.min(s.cornerRadius - t, 0.45 * Math.min(...s.size)));
      break;
    case "ellipsoid":
      s.radii = s.radii.map(d);
      if (s.radii.some((x: number) => x < floor)) return null;
      break;
    case "capsule":
      s.r0 = d(s.r0);
      s.r1 = d(s.r1);
      if (s.r0 < floor || s.r1 < floor) return null;
      break;
    case "revolve":
      s.profile = s.profile.map(([a, r]: number[]) => [a, Math.max(0, r - t)]);
      if (s.profile.every(([, r]: number[]) => r < floor)) return null;
      break;
    case "loft":
      s.sections = s.sections.map((q: Json) => ({ ...q, rx: d(q.rx), ry: d(q.ry) }));
      if (s.sections.some((q: Json) => q.rx < floor || q.ry < floor)) return null;
      break;
    case "extrude": {
      const pr: number[][] = s.profile;
      const cx = mean(pr.map((q) => q[0]!));
      const cy = mean(pr.map((q) => q[1]!));
      const ext = Math.max(...pr.map((q) => Math.abs(q[0]! - cx)), ...pr.map((q) => Math.abs(q[1]! - cy)), 1e-3);
      const f = Math.max(0.05, 1 - t / ext);
      s.profile = pr.map((q) => [cx + (q[0]! - cx) * f, cy + (q[1]! - cy) * f]);
      const mid = (s.from + s.to) / 2;
      const h = (s.to - s.from) / 2 - t;
      if (h < floor / 2) return null;
      s.from = mid - h;
      s.to = mid + h;
      break;
    }
    default:
      return null;
  }
  return s;
}

/** 🔬️ The solid scaled in its cross-section by `k` (thermal patch of the right surface area: area ~ k^0.8).
 * @example scaledSolid(solid, 0.85, "th-leg-skin") */
export function scaledSolid(solid: Json, k: number, id: string, minR = 1e-3): Json {
  const s = clone(solid);
  s.id = id;
  const lim = (x: number): number => Math.max(x * k, minR);
  if (s.kind === "loft") s.sections = s.sections.map((q: Json) => ({ ...q, rx: lim(q.rx), ry: lim(q.ry) }));
  else if (s.kind === "ellipsoid") s.radii = s.radii.map(lim);
  else if (s.kind === "capsule") [s.r0, s.r1] = [lim(s.r0), lim(s.r1)];
  else if (s.kind === "box") {
    s.size = s.size.map(lim);
    if (s.cornerRadius !== undefined) s.cornerRadius = Math.min(s.cornerRadius, 0.45 * Math.min(...s.size));
  } else if (s.kind === "revolve") s.profile = s.profile.map(([a, r]: number[]) => [a, r * k]);
  else if (s.kind === "extrude") {
    const pr: number[][] = s.profile;
    const cx = mean(pr.map((q) => q[0]!));
    const cy = mean(pr.map((q) => q[1]!));
    const g = (q: number[]): number[] => [cx + (q[0]! - cx) * k, cy + (q[1]! - cy) * k];
    s.profile = pr.map(g);
    if (s.holes) s.holes = s.holes.map((h: number[][]) => h.map(g));
    const mid = (s.from + s.to) / 2;
    const h = Math.max(((s.to - s.from) / 2) * k, minR / 2);
    s.from = mid - h;
    s.to = mid + h;
  }
  return s;
}

/** 🌳️ A stack of lobes (ellipsoids) along a path, for crowns and foliage masses: `lobes` are `{ at, radii }` in bone-local coordinates; ids are `<id>-1…n`.
 * @example lobeStack({ id: "crown", bone: "trunk", material: "needles" }, [{ at: [6, 0, 0], radii: [2, 1.5, 1.5] }, { at: [8, 0.3, 0], radii: [1.4, 1.1, 1.1] }]) */
export const lobeStack = (p: Common, lobes: readonly { at: V3; radii: V3; rotation?: [number, number, number, number] }[]): Json[] =>
  lobes.map((l, i) => ({ ...base({ ...p, id: `${p.id}-${i + 1}` }, "ellipsoid"), radii: l.radii, transform: { translation: l.at, ...(l.rotation === undefined ? {} : { rotation: l.rotation }) } }));

/** 🪢️ A lattice bar set: one capsule per edge `[from, to]` between named nodes, radius `r` (a truss, a frame, a trellis); `slender` is set when the bar is at least 20 times longer than wide (C2).
 * @example latticeBars({ id: "truss", bone: "jib", material: "steel" }, { a: [0, 0, 0], b: [1, 0, 0], c: [0.5, 0.8, 0] }, [["a", "b"], ["a", "c"], ["b", "c"]], 0.02) */
export function latticeBars(p: Common, nodes: Record<string, V3>, edges: readonly (readonly [string, string])[], r: number): Json[] {
  return edges.map(([a, b]) => {
    const [p0, p1] = [nodes[a]!, nodes[b]!];
    const L = Math.hypot(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
    return { ...base({ ...p, id: `${p.id}-${a}-${b}` }, "capsule"), p0, p1, r0: r, r1: r, ...(L >= 40 * r ? { slender: true } : {}) };
  });
}

/** 📦️ An envelope prism: a box (rounded by `corner`) around a bone-local extent, how a coarser scale replaces a lattice or an assembly (marks `envelope: true`, C7).
 * @example envelopePrism({ id: "jib-envelope", bone: "jib", material: "steel" }, [0, -0.4, -0.4], [12, 0.4, 0.4]) */
export function envelopePrism(p: Common, min: V3, max: V3, corner = 0): Json {
  const size = [0, 1, 2].map((k) => max[k]! - min[k]!);
  return { ...base(p, "box"), size, transform: { translation: [0, 1, 2].map((k) => (min[k]! + max[k]!) / 2) }, ...(corner > 0 ? { cornerRadius: Math.min(corner, 0.45 * Math.min(...size)) } : {}), envelope: true };
}

/** ⭕️ A revolve of a profile `[axial, radial]` about bone axis `axis` (wheel, tyre, column, bowl).
 * @example revolveOf({ id: "tyre", bone: "wheel", material: "rubber" }, [[-0.1, 0.25], [0.1, 0.25], [0.1, 0.4], [-0.1, 0.4]], "w") */
export function revolveOf(p: Common, profile: readonly (readonly [number, number])[], axis: Axis = "u", origin: V3 = [0, 0, 0]): Json {
  return { ...base(p, "revolve"), profile, axis: axis === "u" ? [1, 0, 0] : axis === "v" ? [0, 1, 0] : [0, 0, 1], origin };
}

/** 🥫️ An extrusion of a polygon on a coordinate plane (`xy` (u,v)→w, `yz` (v,w)→u, `zx` (w,u)→v) from `from` to `to`.
 * @example extrudeOf({ id: "plate", bone: "chassis", material: "steel" }, "xy", [[0,0],[2,0],[2,0.5],[0,0.5]], -0.9, 0.9) */
export const extrudeOf = (p: Common, plane: "xy" | "yz" | "zx", profile: readonly (readonly [number, number])[], from: number, to: number, taperTo?: [number, number]): Json => ({ ...base(p, "extrude"), plane, profile, from, to, ...(taperTo === undefined ? {} : { taperTo }) });

/** 🥚️ A capsule (limb, bar, pipe) between two bone-local points.
 * @example capsuleOf({ id: "shank", bone: "shank-l", material: "skin" }, [0, 0, 0], [0.43, 0, 0], 0.05, 0.035) */
export const capsuleOf = (p: Common, p0: V3, p1: V3, r0: number, r1 = r0, lateralScale?: number): Json => ({ ...base(p, "capsule"), p0, p1, r0, r1, ...(lateralScale === undefined ? {} : { lateralScale }) });

/** 🔘️ An ellipsoid at a bone-local point.
 * @example ellipsoidOf({ id: "head", bone: "head", material: "skin" }, [0.13, 0, 0], [0.12, 0.1, 0.09]) */
export const ellipsoidOf = (p: Common, at: V3, radii: V3): Json => ({ ...base(p, "ellipsoid"), radii, transform: { translation: at } });

/** 🧱️ A box (optionally rounded) centred at a bone-local point.
 * @example boxOf({ id: "step", bone: "chassis", material: "steel" }, [0.2, 0.05, 0.9], [0.4, 0.1, 0.2], 0.01) */
export const boxOf = (p: Common, at: V3, size: V3, corner = 0): Json => ({ ...base(p, "box"), size, transform: { translation: at }, ...(corner > 0 ? { cornerRadius: corner } : {}) });

/* ---------------------------------------------------------------- hull loft from the drawings */

export const AXIS_INDEX: Record<Axis, number> = { u: 0, v: 1, w: 2 };

/** 🧭️ For a bone whose frame is axis-aligned in the world (|dot| > 0.9): local axis → world axis index and sign; `null` when the bone is rotated. */
export function worldAxes(c: Ctx, bone: string): { idx: [number, number, number]; sign: [number, number, number] } | null {
  const f = c.state.frames[bone];
  if (f === undefined) return null;
  const idx: number[] = [];
  const sign: number[] = [];
  for (const L of [f.u, f.v, f.w]) {
    const k = [0, 1, 2].reduce((a, b) => (Math.abs(L[b]!) > Math.abs(L[a]!) ? b : a), 0);
    if (Math.abs(L[k]!) < 0.9) return null;
    idx.push(k);
    sign.push(Math.sign(L[k]!));
  }
  return new Set(idx).size === 3 ? { idx: idx as [number, number, number], sign: sign as [number, number, number] } : null;
}

const VIEW_AXES: Record<View, [number, number]> = { elevation: [0, 1], front: [2, 1], plan: [0, 2] };
const VIEW_SIGN: Record<View, [number, number]> = { elevation: [1, 1], front: [1, 1], plan: [1, 1] };

/** ✂️ Scanline intervals of the drawn shapes of `bone` (all closed drawn shapes when `bone` is undefined) in one view at paper coordinate `at` along paper axis `along` ("x" or "y"); returns the union span `[lo, hi]` of the other paper axis or null. Paper coordinates are the view's own (`projectWorld`). */
export type Pick = (s: { id: string; bone: string; region?: string; role: string }) => boolean;
export function drawnSpan(c: Ctx, view: View, bone: string | undefined, along: "x" | "y", at: number, pick?: Pick): [number, number] | null {
  let lo = Infinity;
  let hi = -Infinity;
  for (const s of drawn(c, view, bone)) {
    if (pick !== undefined && !pick(s)) continue;
    for (const ring of s.rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const a = ring[k]!;
      const b = ring[i]!;
      const [pa, pb, qa, qb] = along === "x" ? [a.x, b.x, a.y, b.y] : [a.y, b.y, a.x, b.x];
      if (pa <= at !== pb <= at) {
        const q = qa + ((at - pa) / (pb - pa)) * (qb - qa);
        lo = Math.min(lo, q);
        hi = Math.max(hi, q);
      }
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

/** 🏗️ A hull loft of one bone from THAT scale's own drawings: stations along local `axis`, each section spanning the drawn extent of the bone's shapes in the two views that contain the axis (elevation and plan for a horizontal axis, elevation and front for a vertical one), centres from the span midpoints, stations reduced by `selectStations` to `tol`. Works for bones that are axis-aligned in the pose (vehicles, furniture, machine bodies, trunks); returns null otherwise (use the drafter plus `kit-fit.ts`).
 * @example hullLoft(openCtx("vehicles/delivery-van", 100), "chassis", "u", { id: "body-shell", material: "steel" }) */
export function hullLoft(c: Ctx, bone: string, axis: Axis, p: { id: string; material: string; tone?: number; role?: string }, o: { stations?: number; e?: number; tol?: number; pick?: Pick } = {}): Json | null {
  const wa = worldAxes(c, bone);
  if (wa === null) return null;
  const f = c.state.frames[bone]!;
  const A = AXIS_INDEX[axis];
  const iA = wa.idx[A]!;
  const others = ([0, 1, 2] as const).filter((k) => k !== A);
  const views = VIEWS.filter((v) => VIEW_AXES[v].includes(iA));
  const spanOf = (target: number, station: number): [number, number] | null => {
    for (const v of views) {
      const ax = VIEW_AXES[v];
      if (!ax.includes(target)) continue;
      const pa = ax.indexOf(iA) === 0 ? "x" : "y";
      const sgn = VIEW_SIGN[v][ax.indexOf(iA)]!;
      const span = drawnSpan(c, v, bone, pa, sgn * station, o.pick);
      if (span === null) continue;
      const sgnT = VIEW_SIGN[v][ax.indexOf(target)]!;
      return sgnT < 0 ? [-span[1], -span[0]] : span;
    }
    return null;
  };
  const extent: [number, number] = [Infinity, -Infinity];
  for (const v of views) {
    const ax = VIEW_AXES[v];
    const pa = ax.indexOf(iA) === 0 ? "x" : "y";
    for (const s of drawn(c, v, bone)) if (o.pick === undefined || o.pick(s)) for (const ring of s.rings) for (const q of ring) {
      const val = (pa === "x" ? q.x : q.y) * VIEW_SIGN[v][ax.indexOf(iA)]!;
      extent[0] = Math.min(extent[0], val);
      extent[1] = Math.max(extent[1], val);
    }
  }
  if (!Number.isFinite(extent[0])) return null;
  const n = o.stations ?? 60;
  const raw: Station[] = [];
  for (let i = 0; i < n; i++) {
    const world = extent[0] + ((extent[1] - extent[0]) * (i + 0.5)) / n;
    const spans = others.map((k) => spanOf(wa.idx[k]!, world));
    if (spans.some((s) => s === null)) continue;
    const loc = (k: number, w: number): number => wa.sign[k]! * (w - f.o[wa.idx[k]!]!);
    const iv = spans.map((s, j) => {
      const k = others[j]!;
      const [a, b] = [loc(k, s![0]), loc(k, s![1])];
      return [Math.min(a, b), Math.max(a, b)] as [number, number];
    });
    const sLoc = wa.sign[A]! * (world - f.o[iA]!);
    const [i1, i2] = iv as [[number, number], [number, number]];
    const kx = LOFT_AXES[axis].rx;
    const ky = LOFT_AXES[axis].ry;
    const byAxis: Record<string, [number, number]> = {};
    const [k1, k2] = others;
    byAxis[(["u", "v", "w"] as const)[k1!]!] = i1;
    byAxis[(["u", "v", "w"] as const)[k2!]!] = i2;
    const sx = byAxis[kx]!;
    const sy = byAxis[ky]!;
    const centre = (key: string): number => ((byAxis[key]?.[0] ?? 0) + (byAxis[key]?.[1] ?? 0)) / 2;
    const [c1k, c2k] = others.map((k) => (["u", "v", "w"] as const)[k]!);
    raw.push({ s: sLoc, c1: centre(c1k!), c2: centre(c2k!), rx: Math.max((sx[1] - sx[0]) / 2, 1e-4), ry: Math.max((sy[1] - sy[0]) / 2, 1e-4), e: o.e ?? 2.6 });
  }
  raw.sort((a, b) => a.s - b.s);
  if (raw.length < 2) return null;
  const sel = selectStations(raw, o.tol ?? 0.5 * c.feature, 0.55 * c.feature);
  return stationLoft({ ...p, bone }, axis, sel);
}
