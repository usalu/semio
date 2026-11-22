/** 🧰️ Shared kit of the HW-18 generators (worker-with-pallet-jack, person-carrying-ladder, gardener-with-wheelbarrow): reads a base human of the catalog, strips its carried object, adds a handcrafted object from one per-scale part list (3D solids of the body, thermal and structure lists and the 2D shapes of every view as extruded `solid` profiles, so the three views are one definition), solves the arms onto grips with the library's own 3D chain and writes the finished asset.
 * @see ../../📋️brief.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { restFrames, solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { evaluate, qualityOf, viewExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";

export type Json = any;
export type P2 = [number, number];
export type P3 = [number, number, number];
export type View = "elevation" | "front" | "plan";
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export const SCALES = [50, 100, 200, 500] as const;
export const ROOT = resolve(import.meta.dir, "../../../../../../../../..");
export const STAFFAGE = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage");
export const HUMANS = join(STAFFAGE, "🗂️catalog/👤️humans");
export const TICKET = resolve(import.meta.dir, "../..");
export const GENERATED = join(TICKET, "🗑️generated/HW-18");

export const r4 = (v: number): number => Math.round(v * 1e4) / 1e4;
export const t = (en: string, de: string): { en: string; de: string } => ({ en, de });
export const clone = <T>(v: T): T => JSON.parse(JSON.stringify(v));

/** 📥️ The asset of a catalog human. */
export const readHuman = (slug: string): Json => JSON.parse(readFileSync(join(HUMANS, slug, "🔣️.json"), "utf8")).asset;

/** 📤️ Writes the asset document of a catalog human (pretty, like the rest of the catalog), or with `HW18_DRAFT=1` a draft to `🗑️generated/HW-18/<id>.json`. */
export function writeHuman(asset: Json): string {
  const draft = process.env.HW18_DRAFT === "1";
  const dir = draft ? join(HUMANS, "../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🗑️generated/HW-18") : join(HUMANS, asset.id);
  mkdirSync(dir, { recursive: true });
  const file = join(dir, draft ? `${asset.id}.json` : "🔣️.json");
  writeFileSync(file, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
  return file;
}

/** ✂️ Removes the given bones with all their descendants and everything bound to them: shapes of every view and scale, solids of every list (with the thermal and structure references to removed body solids), anchors, pose and clip offsets, contacts and loads on removed anchors, and the named thermal regions with their bindings and clip tracks. */
export function strip(asset: Json, bones: readonly string[], regions: readonly string[] = []): void {
  const drop = new Set(bones);
  for (let grew = true; grew; ) {
    grew = false;
    for (const b of asset.rig.bones) if (b.parent !== null && drop.has(b.parent) && !drop.has(b.id)) (drop.add(b.id), (grew = true));
  }
  const regionDrop = new Set(regions);
  asset.rig.bones = asset.rig.bones.filter((b: Json) => !drop.has(b.id));
  const anchorsGone = new Set(asset.anchors.filter((a: Json) => drop.has(a.bone)).map((a: Json) => a.id));
  asset.anchors = asset.anchors.filter((a: Json) => !anchorsGone.has(a.id));
  for (const rep of Object.values(asset.representations) as Json[]) {
    if (rep.status !== "authored") continue;
    for (const v of VIEWS) rep.geometry[v] = rep.geometry[v].filter((s: Json) => !drop.has(s.bone) && !(s.solid?.axisBone && drop.has(s.solid.axisBone)) && !(s.region && regionDrop.has(s.region)));
  }
  for (const vol of Object.values(asset.volumes) as Json[]) {
    if (vol.status !== "authored") continue;
    const goneBody = new Set(vol.body.filter((s: Json) => drop.has(s.bone)).map((s: Json) => s.id));
    vol.body = vol.body.filter((s: Json) => !goneBody.has(s.id));
    for (const list of ["thermal", "structure"]) {
      vol[list] = vol[list].filter((s: Json) => !(s.of && goneBody.has(s.of)) && !(s.bone && drop.has(s.bone)) && !(s.member && drop.has(s.member)) && !(s.region && regionDrop.has(s.region)));
      for (const s of vol[list]) if (s.members) s.members = s.members.filter((m: string) => !drop.has(m));
    }
  }
  const cleanBones = (o: Json): void => {
    if (!o) return;
    for (const k of Object.keys(o)) if (drop.has(k)) delete o[k];
  };
  for (const p of asset.poses) {
    cleanBones(p.bones);
    p.contacts = p.contacts.filter((c: string) => !anchorsGone.has(c));
    p.loads = p.loads.filter((l: Json) => !anchorsGone.has(l.anchor));
    for (const r of regionDrop) delete p.thermal.regions[r];
  }
  for (const c of asset.clips) {
    for (const k of c.keys) cleanBones(k.bones);
    c.contacts = c.contacts.filter((s: Json) => !anchorsGone.has(s.anchor));
    c.tracks = (c.tracks ?? []).filter((tr: Json) => ![...regionDrop].some((r) => tr.target.startsWith(`region:${r}:`)));
  }
  asset.thermal.regions = asset.thermal.regions.filter((r: Json) => !regionDrop.has(r.id));
  asset.thermal.emitters = asset.thermal.emitters.filter((e: Json) => !(e.anchor && anchorsGone.has(e.anchor)) && !(e.region && regionDrop.has(e.region)));
  asset.structure.supports = asset.structure.supports.filter((s: string) => !anchorsGone.has(s));
}

/** 🧹️ Drops materials that no region, layer, solid or shape uses any more. */
export function pruneMaterials(asset: Json): void {
  const used = new Set<string>();
  for (const r of asset.thermal.regions) {
    used.add(r.material);
    for (const l of r.layers) used.add(l.material);
  }
  for (const vol of Object.values(asset.volumes) as Json[]) for (const list of ["body", "thermal", "structure"]) for (const s of vol[list] ?? []) if (s.material) used.add(s.material);
  for (const rep of Object.values(asset.representations) as Json[]) for (const v of VIEWS) for (const s of rep.geometry?.[v] ?? []) if (s.material) used.add(s.material);
  asset.materials = asset.materials.filter((m: Json) => used.has(m.id));
}

/** 🦴️ The 3D chain of a pose (root at the world origin, as `evaluate` before grounding). */
export const chain = (asset: Json, offsets: Json): Json => solveChain(asset.rig, offsets, { x: 0, y: 0, angleDeg: 0 });

/** 📍️ World point of bone-local (u, v, w). */
export function worldOf(frames: Json, bone: string, p: readonly number[]): P3 {
  const f = frames[bone].frame;
  return [0, 1, 2].map((k) => f.o[k] + p[0]! * f.u[k] + p[1]! * f.v[k] + (p[2] ?? 0) * f.w[k]) as P3;
}

/** 📍️ World position of an anchor. */
export function anchorWorld(asset: Json, frames: Json, id: string): P3 {
  const a = asset.anchors.find((x: Json) => x.id === id);
  return worldOf(frames, a.bone, [a.x, a.y, a.z ?? 0]);
}

/** 🔻️ The lowest of the given contact anchors (the height `ground: auto` puts on y = 0). */
export const lowest = (asset: Json, frames: Json, ids: readonly string[]): number => Math.min(...ids.map((id) => anchorWorld(asset, frames, id)[1]));

/** 🔎️ Nelder–Mead minimiser of `f` from `x0` with initial steps `step`. */
export function minimise(f: (x: number[]) => number, x0: number[], step: number[], iterations = 1500): number[] {
  const n = x0.length;
  let simplex = [x0, ...x0.map((_, i) => x0.map((v, j) => (i === j ? v + step[i]! : v)))].map((x) => ({ x, f: f(x) }));
  for (let it = 0; it < iterations; it++) {
    simplex.sort((a, b) => a.f - b.f);
    if (Math.abs(simplex[n]!.f - simplex[0]!.f) < 1e-14 && it > 50) break;
    const c = x0.map((_, j) => simplex.slice(0, n).reduce((s, p) => s + p.x[j]!, 0) / n);
    const at = (k: number): number[] => c.map((v, j) => v + k * (simplex[n]!.x[j]! - v));
    const r = { x: at(-1), f: 0 };
    r.f = f(r.x);
    if (r.f < simplex[0]!.f) {
      const e = { x: at(-2), f: 0 };
      e.f = f(e.x);
      simplex[n] = e.f < r.f ? e : r;
    } else if (r.f < simplex[n - 1]!.f) simplex[n] = r;
    else {
      const k = { x: at(r.f < simplex[n]!.f ? -0.5 : 0.5), f: 0 };
      k.f = f(k.x);
      if (k.f < Math.min(r.f, simplex[n]!.f)) simplex[n] = k;
      else simplex = simplex.map((p, i) => (i === 0 ? p : { x: p.x.map((v, j) => simplex[0]!.x[j]! + 0.5 * (v - simplex[0]!.x[j]!)), f: 0 })).map((p, i) => (i === 0 ? p : { ...p, f: f(p.x) }));
    }
  }
  simplex.sort((a, b) => a.f - b.f);
  return simplex[0]!.x;
}

export type ArmNames = { arm: string; forearm: string; hand: string };

/** 🤲️ Solves the arm angle, spread, twist, elbow and wrist so that the grip anchor lies on `target` (world, before grounding) and the hand's long axis follows `handDir` when given; keeps the joints inside their limits and close to `prefer`. Returns the offsets of the three bones and the residual in metres. */
export function solveArm(asset: Json, offsets: Json, names: ArmNames, grip: string, target: P3, opts: { prefer?: number[]; handDir?: P3; handWeight?: number; elbowOut?: number } = {}): { bones: Json; residual: number } {
  const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
  const lim = [bone(names.arm).joint, bone(names.forearm).joint, bone(names.hand).joint];
  const prefer = opts.prefer ?? [0, 0, 0, 20, 5];
  const clampPen = (v: number, lo: number, hi: number): number => (v < lo ? (lo - v) ** 2 : v > hi ? (v - hi) ** 2 : 0);
  const make = (x: number[]): Json => ({ ...offsets, [names.arm]: { angleDeg: r4(x[0]!), abductionDeg: r4(x[1]!), twistDeg: r4(x[2]!) }, [names.forearm]: { angleDeg: r4(x[3]!) }, [names.hand]: { angleDeg: r4(x[4]!) } });
  const cost = (x: number[]): number => {
    const fr = chain(asset, make(x));
    const g = anchorWorld(asset, fr, grip);
    let c = 1e4 * ((g[0] - target[0]) ** 2 + (g[1] - target[1]) ** 2 + (g[2] - target[2]) ** 2);
    c += 1e-5 * x.reduce((s, v, i) => s + (v - prefer[i]!) ** 2, 0);
    c += 1e-2 * (clampPen(x[0]!, lim[0].minDeg, lim[0].maxDeg) + clampPen(x[3]!, lim[1].minDeg, lim[1].maxDeg) + clampPen(x[4]!, lim[2].minDeg, lim[2].maxDeg) + clampPen(x[2]!, lim[0].twistMinDeg ?? -60, lim[0].twistMaxDeg ?? 60) + clampPen(x[1]!, -60, 60));
    if (opts.handDir) {
      const u = fr[names.hand].frame.u;
      c += (opts.handWeight ?? 0.02) * (1 - (u[0] * opts.handDir[0] + u[1] * opts.handDir[1] + u[2] * opts.handDir[2]));
    }
    return c;
  };
  let best = minimise(cost, prefer, [10, 10, 10, 15, 10]);
  for (let k = 0; k < 3; k++) best = minimise(cost, best, [4, 4, 4, 6, 4]);
  const out = make(best);
  const g = anchorWorld(asset, chain(asset, out), grip);
  return { bones: { [names.arm]: out[names.arm], [names.forearm]: out[names.forearm], [names.hand]: out[names.hand] }, residual: Math.hypot(g[0] - target[0], g[1] - target[1], g[2] - target[2]) };
}

/** 🧱️ A part of a carried object, stated once for the scales that show it: an extruded profile in the bone's (u, v) plane, a tube, a lateral bar, a wheel or a box. Every part yields one body solid, its thermal and structure references and the extruded 2D shape of all three views. */
export type Part = {
  id: string; bone: string; region: string; material: string; tone: number; scales: readonly number[];
  geom: { kind: "profile"; pts: P2[]; w: [number, number] } | { kind: "tube"; a: P2; b: P2; r: number; w: number } | { kind: "bar"; c: P2; r: number; w: [number, number] } | { kind: "wheel"; c: P2; r: number; ri?: number; w: [number, number]; n?: number } | { kind: "box"; c: P3; size: P3 } | { kind: "tubeW"; a: P2; b: P2; r: number; v: number } | { kind: "tube3"; a: P3; b: P3; r: number } | { kind: "loft"; sections: Json[] } | { kind: "ellipsoid"; c: P3; radii: P3 };
  role2d?: "body" | "detail"; views?: readonly View[]; filled?: boolean; open?: boolean; regions?: string[]; alt?: { views: readonly View[]; pts: P2[]; w: [number, number]; sweep: "u" | "v" | "w" }[]; zTip?: [number, number]; z2d?: number; slender?: boolean; sheet?: boolean; draw2d?: P2[]; drawW?: [number, number]; no2d?: boolean; no3d?: boolean; member?: string; tone2d?: number;
};

const circlePts = (c: P2, r: number, n: number): P2[] => Array.from({ length: n }, (_, i) => [r4(c[0] + r * Math.cos((2 * Math.PI * i) / n)), r4(c[1] + r * Math.sin((2 * Math.PI * i) / n))] as P2);

/** 🖍️ The 2D profile and lateral span of a part (bone-local u, v and bone-local w). */
export function profileOf(p: Part, scale: number): { pts: P2[]; w: [number, number]; sweep: "w" | "v" } {
  const g = p.geom;
  if (g.kind === "tubeW") return { ...stadium(g.a, g.b, g.r), w: [r4(g.v - g.r), r4(g.v + g.r)], sweep: "v" };
  if (g.kind === "tube3") return { ...stadium([g.a[0], g.a[1]], [g.b[0], g.b[1]], g.r), w: [r4(g.a[2] - g.r), r4(g.a[2] + g.r)], sweep: "w" };
  if (g.kind === "loft" || g.kind === "ellipsoid") throw new Error(`part ${p.id} of kind ${g.kind} needs draw2d`);
  if (p.draw2d) return { sweep: "w", pts: p.draw2d, w: p.drawW ?? (g.kind === "profile" || g.kind === "bar" || g.kind === "wheel" ? g.w : g.kind === "tube" ? [g.w - g.r, g.w + g.r] : [g.c[2] - g.size[2] / 2, g.c[2] + g.size[2] / 2]) };
  if (g.kind === "profile") return { pts: g.pts, w: g.w, sweep: "w" };
  if (g.kind === "bar") return { pts: circlePts(g.c, g.r, scale <= 100 ? 12 : 8), w: [r4(g.w[0] - g.r), r4(g.w[1] + g.r)], sweep: "w" };
  if (g.kind === "wheel") return { pts: circlePts(g.c, g.r, g.n ?? (scale <= 100 ? 24 : 12)), w: g.w, sweep: "w" };
  if (g.kind === "box") {
    const [cu, cv] = g.c, [du, dv] = g.size;
    return { pts: [[r4(cu - du / 2), r4(cv - dv / 2)], [r4(cu + du / 2), r4(cv - dv / 2)], [r4(cu + du / 2), r4(cv + dv / 2)], [r4(cu - du / 2), r4(cv + dv / 2)]], w: [r4(g.c[2] - g.size[2] / 2), r4(g.c[2] + g.size[2] / 2)], sweep: "w" };
  }
  return { ...stadium(g.a, g.b, g.r), w: [r4(g.w - g.r), r4(g.w + g.r)], sweep: "w" };
}

/** 💊️ A stadium polygon around the segment a–b with radius r. */
export function stadium(a: P2, b: P2, r: number): { pts: P2[] } {
  const ang = Math.atan2(b[1] - a[1], b[0] - a[0]);
  const cap = (c: P2, from: number): P2[] => Array.from({ length: 5 }, (_, i) => [r4(c[0] + r * Math.cos(from + (Math.PI * i) / 4)), r4(c[1] + r * Math.sin(from + (Math.PI * i) / 4))] as P2);
  return { pts: [...cap(b, ang - Math.PI / 2), ...cap(a, ang + Math.PI / 2)] };
}

/** 🧊️ The body solid of a part. */
export function solidOf(p: Part): Json {
  const base = { id: p.id, bone: p.bone, material: p.material, role: "body", tone: p.tone, ...(p.slender ? { slender: true } : {}), ...(p.sheet ? { sheet: true } : {}) };
  const g = p.geom;
  if (g.kind === "profile") return { ...base, kind: "extrude", profile: g.pts, plane: "xy", from: r4(g.w[0]), to: r4(g.w[1]) };
  if (g.kind === "tube") return { ...base, kind: "capsule", p0: [g.a[0], g.a[1], g.w], p1: [g.b[0], g.b[1], g.w], r0: g.r, r1: g.r };
  if (g.kind === "tubeW") return { ...base, kind: "capsule", p0: [g.a[0], g.v, g.a[1]], p1: [g.b[0], g.v, g.b[1]], r0: g.r, r1: g.r };
  if (g.kind === "tube3") return { ...base, kind: "capsule", p0: g.a, p1: g.b, r0: g.r, r1: g.r };
  if (g.kind === "loft") return { ...base, kind: "loft", sections: g.sections, capStart: true, capEnd: true };
  if (g.kind === "ellipsoid") return { ...base, kind: "ellipsoid", radii: g.radii, transform: { translation: g.c } };
  if (g.kind === "bar") return { ...base, kind: "capsule", p0: [g.c[0], g.c[1], g.w[0]], p1: [g.c[0], g.c[1], g.w[1]], r0: g.r, r1: g.r };
  if (g.kind === "box") return { ...base, kind: "box", size: g.size, transform: { translation: g.c } };
  const h = (g.w[1] - g.w[0]) / 2;
  const profile = g.ri ? [[-h, g.ri], [-h, g.r], [h, g.r], [h, g.ri], [-h, g.ri]] : [[-h, 0], [-h, g.r], [h, g.r], [h, 0]];
  return { ...base, kind: "revolve", profile: profile.map(([a, b]) => [r4(a!), r4(b!)]), axis: [0, 0, 1], origin: [0, 0, 0], transform: { translation: [g.c[0], g.c[1], r4((g.w[0] + g.w[1]) / 2)] } };
}

/** 🏗️ Adds the parts of one scale: body solids, `{ of }` thermal and structure references and the 2D shapes of all three views (extruded `solid` profiles with lateral spans in rest-world z). */
export function addParts(asset: Json, scale: number, parts: readonly Part[], extraStructure: Json[] = []): void {
  const vol = asset.volumes[String(scale)];
  const rep = asset.representations[String(scale)];
  const rest = restFrames(asset.rig);
  for (const p of parts.filter((q) => q.scales.includes(scale))) {
    if (!p.no3d) {
      vol.body.push(solidOf(p));
      vol.thermal.push({ id: `th-${p.id}`, of: p.id, region: p.region });
      vol.structure.push({ id: `st-${p.id}`, of: p.id, member: p.member ?? p.bone, part: "member" });
    }
    if (p.no2d) continue;
    const main = profileOf(p, scale);
    for (const v of p.views ?? VIEWS) {
      const { pts, w, sweep } = p.alt?.find((x) => x.views.includes(v)) ?? main;
      const z0 = sweep === "w" ? rest[p.bone].originZ : 0;
      const tip = p.zTip && sweep === "w" ? { zTip: [r4(z0 + p.zTip[0]), r4(z0 + p.zTip[1])] } : {};
      rep.geometry[v].push({ id: p.id, bone: p.bone, role: p.role2d ?? "body", z: p.z2d ?? 2, filled: p.filled ?? (p.role2d ?? "body") === "body", tone: p.tone2d ?? p.tone, ...(p.regions ? { regions: p.regions } : { region: p.region }), kind: "poly", points: pts, closed: !p.open, solid: { kind: "extruded", sweep, z0: r4(z0 + w[0]), z1: r4(z0 + w[1]), ...tip } });
    }
  }
  vol.structure.push(...extraStructure);
}

/** 🌡️ A thermal binding of every region and every non-breath emitter, with overrides. */
export function binding(asset: Json, met: number, breathing: string, over: Json = {}, plume = 0.6): Json {
  const regions: Json = {};
  for (const r of asset.thermal.regions) regions[r.id] = { exposedFraction: 1, sunlitFraction: 0, ...(over[r.id] ?? {}) };
  const emitters: Json = {};
  for (const e of asset.thermal.emitters) if (e.kind !== "breath") emitters[e.id] = e.kind === "plume" ? plume : 1;
  return { activityMet: met, breathing, regions, emitters };
}

/** 📏️ Extents of the drawings of every authored scale in the default pose: elevation width and height, front width and plan depth. */
export function extents(asset: Json): Record<number, { w: number; h: number; frontW: number; frontH: number; planW: number; planD: number; minY: number }> {
  const out: Json = {};
  for (const s of SCALES) {
    if (asset.representations[String(s)].status !== "authored") continue;
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: s } } as never) as Json;
    const f = viewExtent(st, "front")!, p = viewExtent(st, "plan")!;
    out[s] = { w: r4(st.bounds.maxX - st.bounds.minX), h: r4(st.bounds.maxY - st.bounds.minY), frontW: r4(f.horizontal), frontH: r4(f.vertical), planW: r4(p.horizontal), planD: r4(p.vertical), minY: r4(st.bounds.minY) };
  }
  return out;
}

/** 📐️ Sets `dimensions` from the 1:50 drawings (height and length of the side elevation, lateral depth as the mean of front and plan). */
export function setDimensions(asset: Json, massKg: number): void {
  const e = extents(asset)[50]!;
  asset.dimensions = { heightM: Math.round(e.h * 1000) / 1000, widthM: Math.round(Math.max(e.w, e.planW) * 1000) / 1000, depthM: Math.round(((e.frontW + e.planD) / 2) * 1000) / 1000, massKg: Math.round(massKg * 100) / 100 };
}

/** ⚖️ Sum of the bone masses of the rig. */
export const rigMass = (asset: Json): number => asset.rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0);

const TESS = new Map<string, Float64Array>();

/** 🕸️ World vertices of the body solids of one scale that `pick` selects, carried by the given frames (tessellated by the model kernel at the product quality). */
export function bodyVertices(asset: Json, scale: number, frames: Json, pick: (s: Json) => boolean): P3[] {
  const q = qualityOf(asset, scale);
  const out: P3[] = [];
  for (const s of asset.volumes[String(scale)].body.filter(pick)) {
    const key = `${scale}:${s.id}`;
    if (!TESS.has(key)) TESS.set(key, Float64Array.from(tessellateSolid({ ...s, node: s.bone } as never, q as never).positions));
    const pos = TESS.get(key)!;
    for (let i = 0; i < pos.length; i += 3) out.push(worldOf(frames, s.bone, [pos[i]!, pos[i + 1]!, pos[i + 2]!]));
  }
  return out;
}

/** 🏷️ Renames a thermal region everywhere it is named: the region, the shapes of every view and scale (`region` and `regions`), the thermal solids, emitters, pose bindings and clip tracks. */
export function renameRegion(asset: Json, from: string, to: string): void {
  const swap = (id: string): string => (id === from ? to : id);
  for (const r of asset.thermal.regions) r.id = swap(r.id);
  for (const e of asset.thermal.emitters) if (e.region) e.region = swap(e.region);
  for (const rep of Object.values(asset.representations) as Json[]) for (const v of VIEWS) for (const s of rep.geometry?.[v] ?? []) {
    if (s.region) s.region = swap(s.region);
    if (s.regions) s.regions = s.regions.map(swap);
  }
  for (const vol of Object.values(asset.volumes) as Json[]) for (const list of ["thermal", "structure", "body"]) for (const s of vol[list] ?? []) if (s.region) s.region = swap(s.region);
  for (const p of asset.poses) if (p.thermal.regions[from]) (p.thermal.regions[to] = p.thermal.regions[from], delete p.thermal.regions[from]);
  for (const c of asset.clips) for (const tr of c.tracks ?? []) tr.target = tr.target.replace(`region:${from}:`, `region:${to}:`);
}
