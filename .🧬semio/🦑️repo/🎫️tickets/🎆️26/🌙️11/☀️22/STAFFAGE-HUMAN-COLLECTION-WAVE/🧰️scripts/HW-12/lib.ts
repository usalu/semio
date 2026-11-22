/** 🎼️ HW-12 generator library: loads a base human, drops its props, fits rest placements and poses through the library's own `evaluate`, and turns handcrafted prisms into the matching 2D solid shapes of the three views and the 3D solids of the volume.
 * @see ../../📋️brief.md — the binding brief
 * @see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/README.md — conventions, solids, volumes */
import { mkdirSync } from "node:fs";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { bodyDimensions, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

export const STAFFAGE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const HUMANS = `${STAFFAGE}/🗂️catalog/👤️humans`;
export const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE";
export const SCALES = ["50", "100", "200", "500"] as const;
export type Scale = (typeof SCALES)[number];
export type Text = { en: string; de: string };
export type V3 = [number, number, number];
export type P2 = [number, number];
// deno-lint-ignore no-explicit-any
export type Any = any;

export const T = (en: string, de: string): Text => ({ en, de });
export const rd = (x: number, d = 4): number => Math.round(x * 10 ** d) / 10 ** d;

/** 📥️ A deep copy of a catalog human document. */
export async function loadDoc(slug: string): Promise<Any> {
  return structuredClone(await Bun.file(`${HUMANS}/${slug}/🔣️.json`).json());
}

/** 💾️ Writes the asset document into its own folder of the catalog, or with `HW12_DRAFT=1` as a draft to `🗑️generated/HW-12/<id>.json`. */
export async function saveDoc(doc: Any): Promise<string> {
  const seen = new Set<string>();
  doc.asset.provenance.references = doc.asset.provenance.references.filter((r: Any) => !seen.has(r.id) && !!seen.add(r.id));
  const draft = process.env.HW12_DRAFT === "1";
  const dir = draft ? `${TICKET}/🗑️generated/HW-12` : `${HUMANS}/${doc.asset.id}`;
  mkdirSync(dir, { recursive: true });
  const file = draft ? `${dir}/${doc.asset.id}.json` : `${dir}/🔣️.json`;
  await Bun.write(file, JSON.stringify(doc, null, 2) + "\n");
  return file;
}

export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, s: number): V3 => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / len(a));
export const rad = (d: number): number => (d * Math.PI) / 180;
export const deg = (r: number): number => (r * 180) / Math.PI;
export const v3 = (p: { x: number; y: number; z: number }): V3 => [p.x, p.y, p.z];

export type Frame = { o: V3; u: V3; v: V3; w: V3 };

/** 🧭️ The world frame of a bone in an evaluated state. */
export function frameOf(state: Any, bone: string): Frame {
  const f = state.bones[bone].frame;
  return { o: v3(f.o), u: v3(f.u), v: v3(f.v), w: v3(f.w) };
}

/** 📍️ A bone-local point carried into the world. */
export const at = (f: Frame, a: number, b: number, c: number): V3 => add(f.o, add(mul(f.u, a), add(mul(f.v, b), mul(f.w, c))));

/** 🧮️ Evaluates the asset with one probe pose made of the given bone offsets on top of `basePose` (or of nothing). */
export function evalOffsets(asset: Any, bones: Record<string, Any>, basePose?: string): Any {
  const base = basePose ? asset.poses.find((p: Any) => p.id === basePose) : undefined;
  const probe = { ...(base ?? asset.poses[0]), id: "__probe", bones: { ...(base?.bones ?? {}), ...bones } };
  return evaluate({ ...asset, poses: [...asset.poses.filter((p: Any) => p.id !== "__probe"), probe] }, { kind: "pose", pose: "__probe" });
}

/** 🧘️ The rest state: no offsets at all. */
export function restState(asset: Any): Any {
  const probe = { ...asset.poses[0], id: "__rest", bones: {} };
  return evaluate({ ...asset, poses: [...asset.poses, probe] }, { kind: "pose", pose: "__rest" });
}

/** 🔻️ Nelder–Mead minimiser for the small fits of rest placements and poses. */
export function minimise(f: (x: number[]) => number, x0: number[], step: number[] | number, iterations = 600, tolerance = 1e-10): { x: number[]; f: number } {
  const n = x0.length;
  const steps = Array.isArray(step) ? step : x0.map(() => step);
  let simplex = [x0.slice(), ...x0.map((_, i) => x0.map((v, j) => (i === j ? v + steps[i]! : v)))];
  let values = simplex.map(f);
  for (let it = 0; it < iterations; it++) {
    const order = values.map((v, i) => [v, i] as const).sort((a, b) => a[0] - b[0]).map(([, i]) => i);
    simplex = order.map((i) => simplex[i]!);
    values = order.map((i) => values[i]!);
    if (Math.abs(values[n]! - values[0]!) < tolerance) break;
    const centroid = x0.map((_, j) => simplex.slice(0, n).reduce((s, p) => s + p[j]!, 0) / n);
    const along = (t: number) => centroid.map((c, j) => c + t * (simplex[n]![j]! - c));
    const xr = along(-1), fr = f(xr);
    if (fr < values[0]!) {
      const xe = along(-2), fe = f(xe);
      [simplex[n], values[n]] = fe < fr ? [xe, fe] : [xr, fr];
    } else if (fr < values[n - 1]!) {
      [simplex[n], values[n]] = [xr, fr];
    } else {
      const xc = along(fr < values[n]! ? -0.5 : 0.5), fc = f(xc);
      if (fc < Math.min(fr, values[n]!)) [simplex[n], values[n]] = [xc, fc];
      else {
        simplex = simplex.map((p, i) => (i === 0 ? p : p.map((v, j) => simplex[0]![j]! + 0.5 * (v - simplex[0]![j]!))));
        values = simplex.map((p, i) => (i === 0 ? values[0]! : f(p)));
      }
    }
  }
  const best = values.indexOf(Math.min(...values));
  return { x: simplex[best]!, f: values[best]! };
}

/** 📐️ Fits the rest placement of `boneId` (already in the rig) so that its rest frame is the world frame `target` (origin, u, v); returns the residual. */
export function placeBone(asset: Any, boneId: string, target: { o: V3; u: V3; v: V3 }): number {
  const bone = asset.rig.bones.find((b: Any) => b.id === boneId);
  const parent = frameOf(restState(asset), bone.parent);
  const d = sub(target.o, parent.o);
  [bone.x, bone.y, bone.z] = [rd(dot(d, parent.u), 5), rd(dot(d, parent.v), 5), rd(dot(d, parent.w), 5)];
  const cost = (x: number[]): number => {
    [bone.angleDeg, bone.abductionDeg, bone.twistDeg] = x;
    const f = frameOf(restState(asset), boneId);
    return len(sub(f.u, target.u)) ** 2 + len(sub(f.v, target.v)) ** 2;
  };
  let best = { x: [0, 0, 0], f: Infinity };
  for (const a of [-120, -60, 0, 60, 120, 180]) for (const b of [-60, 0, 60]) for (const c of [-120, 0, 120]) {
    const r = minimise(cost, [a, b, c], 20, 400);
    if (r.f < best.f) best = r;
  }
  best = minimise(cost, best.x, 2, 800, 1e-14);
  [bone.angleDeg, bone.abductionDeg, bone.twistDeg] = best.x.map((v) => rd(((v + 540) % 360) - 180, 4));
  return Math.sqrt(cost([bone.angleDeg, bone.abductionDeg, bone.twistDeg]));
}

/** ✂️ Removes bones with everything bound to them: shapes, solids (with their thermal references), anchors, pose and clip offsets. */
export function dropBones(asset: Any, ids: ReadonlySet<string>): void {
  asset.rig.bones = asset.rig.bones.filter((b: Any) => !ids.has(b.id));
  const anchors = new Set(asset.anchors.filter((a: Any) => ids.has(a.bone)).map((a: Any) => a.id));
  asset.anchors = asset.anchors.filter((a: Any) => !anchors.has(a.id));
  for (const rep of Object.values(asset.representations) as Any[]) {
    if (rep.status !== "authored") continue;
    for (const view of Object.keys(rep.geometry)) rep.geometry[view] = rep.geometry[view].filter((s: Any) => !ids.has(s.bone));
  }
  for (const vol of Object.values(asset.volumes) as Any[]) {
    if (vol.status !== "authored") continue;
    const gone = new Set(vol.body.filter((s: Any) => ids.has(s.bone)).map((s: Any) => s.id));
    vol.body = vol.body.filter((s: Any) => !gone.has(s.id));
    vol.thermal = vol.thermal.filter((s: Any) => !(s.of && gone.has(s.of)) && !ids.has(s.bone));
    vol.structure = vol.structure.filter((s: Any) => !(s.of && gone.has(s.of)) && !ids.has(s.bone) && !ids.has(s.member));
  }
  for (const p of asset.poses) {
    for (const id of ids) delete p.bones[id];
    p.contacts = p.contacts.filter((c: string) => !anchors.has(c));
    p.loads = p.loads.filter((l: Any) => !anchors.has(l.anchor) && !ids.has(l.bone));
  }
  for (const c of asset.clips) {
    for (const k of c.keys) if (k.bones) for (const id of ids) delete k.bones[id];
    c.contacts = c.contacts.filter((s: Any) => !anchors.has(s.anchor));
    c.events = c.events.filter((e: Any) => !anchors.has(e.target) && !ids.has(e.target));
  }
  asset.structure.supports = asset.structure.supports.filter((s: string) => !anchors.has(s));
}

/** 🧱️ A handcrafted prism of a prop: a profile in the bone-local plane across the sweep (`v`: (u, w); `w`: (u, v)) extruded from `from` to `to` along the sweep axis. */
export type Prism = {
  id: string; bone: string; sweep: "v" | "w"; profile: P2[]; from: number; to: number;
  material: string; region: string; role?: "body" | "detail"; tone: number; z: number;
  slender?: boolean; sheet?: boolean; hatch?: string; views?: readonly ("elevation" | "front" | "plan")[]; volume?: boolean;
};

/** ✏️ The 2D shape of a prism for one view: the profile with an `extruded` solid, so the engine draws its true projection. */
export function prismShape(p: Prism, restOz: number): Any {
  const solid = p.sweep === "w" ? { kind: "extruded", sweep: "w", z0: rd(restOz + p.from, 5), z1: rd(restOz + p.to, 5) } : { kind: "extruded", sweep: "v", z0: rd(p.from, 5), z1: rd(p.to, 5) };
  return { id: p.id, bone: p.bone, role: p.role ?? "body", z: p.z, filled: true, tone: p.tone, region: p.region, ...(p.hatch ? { hatch: p.hatch } : {}), kind: "poly", points: p.profile.map(([a, b]) => [rd(a, 5), rd(b, 5)]), closed: true, solid };
}

/** 🧊️ The 3D body solid of a prism: an extrusion in the same bone-local frame. */
export function prismSolid(p: Prism): Any {
  const plane = p.sweep === "w" ? "xy" : "zx";
  const profile = p.sweep === "w" ? p.profile.map(([a, b]) => [rd(a, 5), rd(b, 5)]) : p.profile.map(([a, b]) => [rd(b, 5), rd(a, 5)]);
  return { id: p.id, kind: "extrude", bone: p.bone, material: p.material, role: p.role ?? "body", tone: p.tone, profile, plane, from: rd(p.from, 5), to: rd(p.to, 5), ...(p.slender ? { slender: true } : {}), ...(p.sheet ? { sheet: true } : {}) };
}

/** ➕️ Adds prisms to one scale: shapes to the three views and solids with their thermal references to the volume. */
export function addPrisms(asset: Any, scale: Scale, prisms: readonly Prism[], rest: Any): void {
  const rep = asset.representations[scale];
  const vol = asset.volumes[scale];
  for (const p of prisms) {
    const oz = rest.bones[p.bone].frame.o.z;
    for (const view of p.views ?? ["elevation", "front", "plan"]) rep.geometry[view].push(prismShape(p, oz));
    if (p.volume === false) continue;
    vol.body.push(prismSolid(p));
    vol.thermal.push({ id: `th-${p.id}`, of: p.id, region: p.region });
  }
}

/** 🔁️ A closed profile from a half-width table along u, mirrored across the centre line `c`. */
export function mirrored(table: readonly P2[], c = 0): P2[] {
  const right = table.map(([u, h]) => [u, c - h] as P2);
  const left = table.slice().reverse().map(([u, h]) => [u, c + h] as P2);
  return dedupe([...right, ...left]);
}

/** 🧹️ Drops consecutive duplicate points of a ring. */
export function dedupe(ring: readonly P2[]): P2[] {
  const out: P2[] = [];
  for (const p of ring) if (!out.length || Math.hypot(p[0] - out[out.length - 1]![0], p[1] - out[out.length - 1]![1]) > 1e-6) out.push(p);
  if (out.length > 1 && Math.hypot(out[0]![0] - out[out.length - 1]![0], out[0]![1] - out[out.length - 1]![1]) < 1e-6) out.pop();
  return out;
}

/** 📏️ A rectangle profile. */
export const rect = (a0: number, b0: number, a1: number, b1: number): P2[] => [[a0, b0], [a1, b0], [a1, b1], [a0, b1]];

/** 🦾️ Solves bone offsets (a flat list of `[bone, field]` unknowns) for a cost on the evaluated state, starting from `start`. */
export function solvePose(asset: Any, base: Record<string, Any>, unknowns: readonly (readonly [string, string])[], start: number[], cost: (state: Any, x: number[]) => number, step = 8, iterations = 1500): { bones: Record<string, Any>; residual: number } {
  const compose = (x: number[]): Record<string, Any> => {
    const bones: Record<string, Any> = structuredClone(base);
    unknowns.forEach(([b, f], i) => { bones[b] = { ...(bones[b] ?? {}), [f]: x[i] }; });
    return bones;
  };
  const f = (x: number[]) => cost(evalOffsets(asset, compose(x)), x);
  let r = minimise(f, start, step, iterations);
  r = minimise(f, r.x, step / 4, iterations);
  r = minimise(f, r.x, step / 16, iterations, 1e-14);
  const bones = compose(r.x.map((v) => rd(v, 3)));
  return { bones, residual: r.f };
}

/** 🌀️ Redraws the limbs of `bones` at every authored scale in all three views as revolved solids about their bone axis, one per loft solid of that bone at that scale, the radius of every section the mean of its two half extents (the limbs of the body are round lofts about their bone axis): a raised or turned limb then reads right in every view, as the flat and prismatic limb shapes of a hanging arm do not. */
export function revolveLimbs(asset: Any, bones: ReadonlySet<string>): void {
  for (const scale of SCALES) {
    const rep = asset.representations[scale], vol = asset.volumes[scale];
    if (rep.status !== "authored" || vol.status !== "authored") continue;
    const regionOf = new Map(vol.thermal.filter((t: Any) => t.of).map((t: Any) => [t.of, t.region]));
    for (const view of ["elevation", "front", "plan"] as const) {
      const old = rep.geometry[view].filter((s: Any) => bones.has(s.bone));
      const keep = rep.geometry[view].filter((s: Any) => !bones.has(s.bone));
      const made = vol.body.filter((s: Any) => bones.has(s.bone) && s.kind === "loft").map((s: Any) => {
        const twin = old.find((o: Any) => o.id === s.id) ?? old.find((o: Any) => o.role === (s.role ?? "body")) ?? old[0];
        const profile: P2[] = s.sections.map((c: Any) => [rd(c.at[0], 5), rd((c.rx + c.ry) / 2 + Math.hypot(c.at[1], c.at[2]), 5)] as P2);
        const ring = dedupe([[profile[0]![0], 0], ...profile, [profile[profile.length - 1]![0], 0]]);
        const own = regionOf.get(s.id) ?? twin?.region;
        const all = [...new Set([own, ...old.flatMap((o: Any) => [o.region, ...(o.regions ?? [])])].filter((r) => r !== undefined))];
        const regionField = s === vol.body.find((b: Any) => bones.has(b.bone) && b.bone === s.bone && b.kind === "loft") && all.length > 1 ? { regions: all } : { region: own };
        return { id: s.id, bone: s.bone, role: s.role ?? "body", z: twin?.z ?? 4, filled: true, tone: twin?.tone ?? s.tone, ...regionField, ...(twin?.hatch ? { hatch: twin.hatch } : {}), kind: "poly", points: ring, closed: true, solid: { kind: "revolved", axisBone: s.bone, axis: "u", axisX: 0, axisZ: 0, mirror: true } };
      });
      rep.geometry[view] = [...keep, ...made];
    }
  }
}

/** 📦️ Sets `dimensions` (width, height, depth) to the bounding box of the 1:50 body volume in the default pose (the guard allows one smallest feature) and prints every scale's box. */
export function fitDimensions(asset: Any): void {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  const boxes = SCALES.filter((scale) => asset.volumes[scale].status === "authored").map((scale) => bodyDimensions(worldSolids(asset, state.bones, Number(scale)).filter((x: Any) => x.texture !== true)));
  const fine = boxes[0]!;
  const pick = (k: number): number => {
    const values = boxes.map((b) => b[k]!);
    const mid = (Math.min(...values) + Math.max(...values)) / 2;
    return rd(Math.min(fine[k]! + 0.007, Math.max(fine[k]! - 0.007, mid)), 3);
  };
  Object.assign(asset.dimensions, { widthM: pick(0), heightM: pick(1), depthM: pick(2) });
  console.log("[DEBUG] volume boxes", JSON.stringify(boxes.map((b) => b.map((x) => rd(x, 3)))), "dimensions", JSON.stringify(asset.dimensions));
}

/** ✊️ Closes the open coarse-scale hands of the base figure into gripping fists ending at `maxU` along the hand bone, in the volume and in all three drawings of the given scales. */
export function fistHands(asset: Any, scales: readonly Scale[], maxU: number, bones: readonly string[] = ["hand-l", "hand-r"]): void {
  for (const scale of scales) {
    for (const s of asset.volumes[scale].body) {
      if (!bones.includes(s.bone) || s.kind !== "loft") continue;
      const kept = s.sections.filter((c: Any) => c.at[0] <= maxU - 0.012);
      const last = kept[kept.length - 1];
      s.sections = [...kept, { ...last, at: [maxU - 0.01, last.at[1], last.at[2]] }, { ...last, at: [maxU, last.at[1], last.at[2]], rx: rd(last.rx * 0.4, 5), ry: rd(last.ry * 0.4, 5) }];
    }
    for (const view of ["elevation", "front", "plan"]) for (const s of asset.representations[scale].geometry[view]) {
      if (!bones.includes(s.bone) || !s.points) continue;
      const xs = s.points.map((p: P2) => p[0]);
      const [lo, hi] = [Math.min(...xs), Math.max(...xs)];
      if (hi <= maxU) continue;
      s.points = s.points.map(([x, y]: P2) => [rd(lo + ((x - lo) * (maxU - lo)) / (hi - lo), 5), y]);
    }
  }
}

const closedOf = (s: Any): boolean => (s.kind === "poly" ? s.closed !== false : s.kind === "path" ? /z\s*$/i.test(s.d.trim()) : true);

/** 🧱️ Gives every flat shape of the `bones` (all bones when absent) a solid, so a pose that turns those bones out of their rest plane (a spread, a twist, a turn of the whole figure) draws them right: an elevation shape becomes its side profile extruded across the lateral extent of its own volume solid (same id) or of all solids of its bone at that scale; a flat front or plan shape is replaced by its solid elevation twin, else dropped; open strokes and details without a solid of their own are dropped. */
export function solidifyFlat(asset: Any, bones?: ReadonlySet<string>): void {
  const rest = restState(asset);
  for (const scale of SCALES) {
    const rep = asset.representations[scale], vol = asset.volumes[scale];
    if (rep.status !== "authored" || vol.status !== "authored") continue;
    const worlds = worldSolids(asset, rest.bones, Number(scale));
    const span = (pick: (w: Any) => boolean): [number, number] | undefined => {
      let lo = Infinity, hi = -Infinity;
      for (const w of worlds) if (pick(w)) for (let i = 2; i < w.positions.length; i += 3) { lo = Math.min(lo, w.positions[i]); hi = Math.max(hi, w.positions[i]); }
      return Number.isFinite(lo) && hi - lo > 1e-4 ? [rd(lo, 4), rd(hi, 4)] : undefined;
    };
    const inScope = (s: Any) => s.solid === undefined && s.role !== "ground" && (bones === undefined || bones.has(s.bone));
    const dropped: Any[] = [];
    rep.geometry.elevation = rep.geometry.elevation.flatMap((s: Any) => {
      if (!inScope(s)) return [s];
      if (!closedOf(s)) return [];
      const z = span((w) => w.id === s.id && w.bone === s.bone) ?? (s.role === "body" ? span((w) => w.bone === s.bone) : undefined);
      if (!z) dropped.push(s);
      return z ? [{ ...s, solid: { kind: "extruded", sweep: "w", z0: z[0], z1: z[1] } }] : [];
    });
    const drawn = new Set(rep.geometry.elevation.flatMap((s: Any) => [s.region, ...(s.regions ?? [])]));
    for (const s of dropped) {
      if ([s.region, ...(s.regions ?? [])].every((r: string) => r === undefined || drawn.has(r))) continue;
      const z = span((w) => w.bone === s.bone);
      if (z) { rep.geometry.elevation.push({ ...s, solid: { kind: "extruded", sweep: "w", z0: z[0], z1: z[1] } }); for (const r of [s.region, ...(s.regions ?? [])]) drawn.add(r); }
    }
    for (const view of ["front", "plan"] as const)
      rep.geometry[view] = rep.geometry[view].flatMap((s: Any) => {
        if (!inScope(s)) return [s];
        if (rep.geometry[view].some((o: Any) => o.bone === s.bone && o.solid !== undefined)) return [];
        const twin = rep.geometry.elevation.find((e: Any) => e.id === s.id && e.bone === s.bone && e.solid !== undefined);
        return twin ? [{ ...structuredClone(twin), z: s.z }] : [];
      });
  }
}

/** 🩹️ Sets the masses sum and returns it. */
export function massOf(asset: Any): number {
  return rd(asset.rig.bones.reduce((s: number, b: Any) => s + (b.mass?.kg ?? 0), 0), 2);
}
