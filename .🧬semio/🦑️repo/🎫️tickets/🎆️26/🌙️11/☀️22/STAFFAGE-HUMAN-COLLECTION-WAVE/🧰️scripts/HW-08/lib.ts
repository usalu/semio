/** 🧰️ Shared helpers of the HW-08 generators: catalog paths, base loading, subtree pruning, rest frames, shape and solid builders.
 * @see ../../📋️brief.md */
import { join, resolve } from "node:path";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
export const REPO = resolve(import.meta.dir, "../../../../../../../../..");
export const S = join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage");
export const HUMANS = join(S, "🗂️catalog/👤️humans");
export const T = resolve(import.meta.dir, "../..");
export const OUT = join(T, "🗑️generated/HW-08");
export const SCALES = ["50", "100", "200", "500"] as const;
export const VIEWS = ["elevation", "front", "plan"] as const;
export { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
export { restFrames, solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
export { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";

export type J = any;
export const clone = <T>(x: T): T => structuredClone(x);
export const loadDoc = (slug: string): J => JSON.parse(readFileSync(join(HUMANS, slug, "🔣️.json"), "utf8"));
export function saveDoc(slug: string, doc: J): void {
  mkdirSync(join(HUMANS, slug), { recursive: true });
  writeFileSync(join(HUMANS, slug, "🔣️.json"), JSON.stringify(doc, null, 2) + "\n");
}
export const r = (x: number, d = 4): number => Math.round(x * 10 ** d) / 10 ** d;
export const t = (en: string, de: string) => ({ en, de });

/** ✂️ Removes the bones of the given subtrees and everything bound to them: anchors, shapes, solids (and references to them), loads, offsets, contacts. */
export function pruneBones(a: J, roots: readonly string[]): void {
  const gone = new Set<string>(roots);
  let grew = true;
  while (grew) { grew = false; for (const b of a.rig.bones) if (b.parent && gone.has(b.parent) && !gone.has(b.id)) { gone.add(b.id); grew = true; } }
  a.rig.bones = a.rig.bones.filter((b: J) => !gone.has(b.id));
  const anchors = new Set(a.anchors.filter((x: J) => gone.has(x.bone)).map((x: J) => x.id));
  a.anchors = a.anchors.filter((x: J) => !anchors.has(x.id));
  dropShapes(a, (s) => gone.has(s.bone));
  dropSolids(a, (s) => gone.has(s.bone) || gone.has(s.member) || (s.anchor && anchors.has(s.anchor)));
  for (const p of a.poses) { for (const id of gone) delete p.bones[id]; p.contacts = p.contacts.filter((c: string) => !anchors.has(c)); p.loads = p.loads.filter((l: J) => !anchors.has(l.anchor) && !gone.has(l.bone)); }
  for (const c of a.clips) { for (const k of c.keys) if (k.bones) for (const id of gone) delete k.bones[id]; c.contacts = c.contacts.filter((s: J) => !anchors.has(s.anchor)); c.events = c.events.filter((e: J) => !anchors.has(e.target)); }
  a.thermal.emitters = a.thermal.emitters.filter((e: J) => !anchors.has(e.anchor));
}
export function dropShapes(a: J, pred: (s: J) => boolean): void {
  for (const k of SCALES) { const rep = a.representations[k]; if (rep.status !== "authored") continue; for (const v of VIEWS) rep.geometry[v] = rep.geometry[v].filter((s: J) => !pred(s)); }
  for (const sym of a.symbols) for (const v of VIEWS) sym.geometry[v] = sym.geometry[v].filter((s: J) => !pred(s));
}
export function dropSolids(a: J, pred: (s: J) => boolean): void {
  for (const k of SCALES) {
    const vol = a.volumes[k]; if (vol.status !== "authored") continue;
    const removed = new Set(vol.body.filter(pred).map((s: J) => s.id));
    vol.body = vol.body.filter((s: J) => !removed.has(s.id));
    for (const l of ["thermal", "structure"]) vol[l] = vol[l].filter((s: J) => !(s.of && removed.has(s.of)) && !pred(s));
  }
}
/** 🧹️ Removes materials and thermal regions no longer referenced, and the pose bindings of removed regions. */
export function pruneUnused(a: J): void {
  const regions = new Set<string>();
  const mats = new Set<string>();
  const visit = (s: J) => { if (s.region) regions.add(s.region); for (const x of s.regions ?? []) regions.add(x); if (s.material) mats.add(s.material); };
  for (const k of SCALES) { const rep = a.representations[k]; if (rep.status === "authored") for (const v of VIEWS) rep.geometry[v].forEach(visit); const vol = a.volumes[k]; if (vol.status === "authored") for (const l of ["body", "thermal", "structure"]) vol[l].forEach(visit); }
  for (const sym of a.symbols) for (const v of VIEWS) sym.geometry[v].forEach(visit);
  a.thermal.regions = a.thermal.regions.filter((x: J) => regions.has(x.id));
  for (const x of a.thermal.regions) { mats.add(x.material); for (const l of x.layers) mats.add(l.material); }
  for (const b of a.rig.bones) if (b.section) mats.add(b.section.material);
  a.materials = a.materials.filter((m: J) => mats.has(m.id));
  for (const p of a.poses) for (const id of Object.keys(p.thermal.regions)) if (!regions.has(id)) delete p.thermal.regions[id];
}
export const bone = (a: J, id: string): J => a.rig.bones.find((b: J) => b.id === id);
export const pose = (a: J, id: string): J => a.poses.find((p: J) => p.id === id);
export const anchor = (a: J, id: string): J => a.anchors.find((x: J) => x.id === id);
export const region = (a: J, id: string): J => a.thermal.regions.find((x: J) => x.id === id);
export const material = (a: J, id: string): J => a.materials.find((x: J) => x.id === id);
import { worldSolids, extentOf, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf, checkVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { evaluate as ev } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
export { worldSolids, extentOf, measureAsset, featureOf, checkVolumes };
/** 📦️ World extents of the body solids of a scale in a pose, per solid id (filtered). */
export function solidBoxes(a: J, poseId: string, scale: number, pick: (id: string) => boolean = () => true): Record<string, { min: number[]; max: number[] }> {
  const state = ev(a, { kind: "pose", pose: poseId });
  const out: Record<string, any> = {};
  for (const w of worldSolids(a, state.bones as never, scale)) if (pick(w.id)) { const e = extentOf([w]); out[w.id] = { min: [...e.min].map((x: number) => r(x, 3)), max: [...e.max].map((x: number) => r(x, 3)) }; }
  return out;
}
/** 🌓️ The catalog-silhouette gate for one asset: bbox against dimensions, IoU and Hausdorff per scale and view. */
export function silhouette(a: J, scales: readonly number[] = [50, 100, 200, 500]): string[] {
  const lines: string[] = [];
  const dims = [a.dimensions.widthM, a.dimensions.heightM, a.dimensions.depthM];
  for (const scale of scales) {
    const feature = featureOf(scale as never);
    const m = measureAsset(a, scale, undefined, undefined, feature);
    const bad = m.bbox.map((b: number, k: number) => Math.abs(b - dims[k]!) > feature);
    lines.push(`1:${scale} bbox ${m.bbox.map((b: number) => b.toFixed(3)).join(" ")} vs ${dims.join(" ")}${bad.some(Boolean) ? " FAIL" : ""} | ` + (["elevation", "front", "plan"] as const).map((v) => `${v} ${m.views[v].iou.toFixed(3)}/${(m.views[v].hausdorffM * 1000).toFixed(0)}mm${m.views[v].iou < 0.92 || m.views[v].hausdorffM > 2 * feature ? " FAIL" : ""}`).join(" "));
  }
  return lines;
}

/** 🦯️ One axisymmetric part along a bone axis (u from `u0` to `u1`, radius `r0` to `r1`). */
export type Seg = { id: string; u0: number; u1: number; r0: number; r1?: number; region: string; material: string; tone: number; role?: string; penMm?: number };
/** ✏️ The 2D shape of a part: its (u, v) profile, revolved about the bone axis, the same in all three views (only the paint order differs). */
export function revolvedShape(bone: string, s: Seg, z: number): J {
  const r1 = s.r1 ?? s.r0;
  return { id: s.id, bone, role: s.role ?? "body", z, filled: true, tone: s.tone, region: s.region, material: s.material, ...(s.penMm ? { penMm: s.penMm } : {}), kind: "poly", points: [[s.u0, -s.r0], [s.u1, -r1], [s.u1, r1], [s.u0, s.r0]].map(([a, b]) => [r(a!, 5), r(b!, 5)]), closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, axisZ: 0 } };
}
/** 🧊️ The 3D solid of the same part: a revolve of the profile about u (slender when twenty times as long as thick). */
export function revolvedSolid(bone: string, s: Seg, slender?: boolean): J {
  const r1 = s.r1 ?? s.r0;
  const isSlender = slender ?? (s.u1 - s.u0) >= 20 * 2 * Math.max(s.r0, r1);
  return { id: s.id, kind: "revolve", bone, material: s.material, role: s.role ?? "body", tone: s.tone, profile: [[s.u0, 0], [s.u0, s.r0], [s.u1, r1], [s.u1, 0]].map(([a, b]) => [r(a!, 5), r(b!, 5)]), axis: [1, 0, 0], origin: [0, 0, 0], ...(isSlender ? { slender: true } : {}) };
}
/** 📐️ A prism part swept laterally: the (u, v) profile from z0 to z1 (rest world z), drawn in all three views with an extruded solid. */
export function extrudedShape(id: string, bone: string, pts: number[][], z0: number, z1: number, z: number, extra: J): J {
  return { id, bone, role: "body", z, filled: true, ...extra, kind: "poly", points: pts.map(([a, b]) => [r(a!, 5), r(b!, 5)]), closed: true, solid: { kind: "extruded", sweep: "w", z0, z1 } };
}
export function extrudedSolid(id: string, bone: string, pts: number[][], w0: number, w1: number, extra: J): J {
  return { id, kind: "extrude", bone, role: "body", ...extra, profile: pts.map(([a, b]) => [r(a!, 5), r(b!, 5)]), plane: "xy", from: w0, to: w1 };
}
/** 🟦️ A rounded (u, v) rectangle as a polygon with chamfered corners. */
export function roundedRect(u0: number, u1: number, v0: number, v1: number, c: number): number[][] {
  return [[u0 + c, v0], [u1 - c, v0], [u1, v0 + c], [u1, v1 - c], [u1 - c, v1], [u0 + c, v1], [u0, v1 - c], [u0, v0 + c]];
}
/** 📏️ Sets `dimensions` from the 1:50 volume in the default pose (the silhouette guard's bbox, within 7 mm) pulled towards the drawn extents of every scale, and the mass from the segment masses. */
export function fitDimensions(a: J): void {
  const m = measureAsset(a, 50, undefined, undefined, featureOf(50 as never));
  const drawn = [[Infinity, -Infinity], [Infinity, -Infinity], [Infinity, -Infinity]].map(() => [] as number[]);
  for (const k of SCALES) {
    const st = ev(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(k) } } as never);
    const ext = (view: string, i: 0 | 1) => { const v = (st.shapes as J)[view].filter((x: J) => x.shape.role !== "ground").flatMap((x: J) => x.rings.flat().map((p: J) => (i === 0 ? p.x : p.y))); return Math.max(...v) - Math.min(...v); };
    drawn[0]!.push(ext("elevation", 0), ext("plan", 0)); drawn[1]!.push(ext("elevation", 1), ext("front", 1)); drawn[2]!.push(ext("front", 0), ext("plan", 1));
  }
  const fit = (vol: number, d: number[]) => { const mid = (Math.min(...d) + Math.max(...d)) / 2; return r(Math.max(vol - 0.0072, Math.min(vol + 0.0072, mid)), 4); };
  const [w, h, d] = m.bbox;
  a.dimensions = { heightM: fit(h, drawn[1]!), widthM: fit(w, drawn[0]!), depthM: fit(d, drawn[2]!), massKg: r(a.rig.bones.reduce((s: number, b: J) => s + (b.mass?.kg ?? 0), 0), 3) };
}
import { copyMesh } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { restFrames as rf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
export { copyMesh, qualityOf };
/** 📐️ The bone-local extents (u, v, w) of the body solids of one bone at one scale, and the rest world z range of their vertices. */
export function boneExtents(a: J, scale: string, boneId: string): { u: number[]; v: number[]; w: number[]; z: number[] } | null {
  const vol = a.volumes[scale]; if (vol.status !== "authored") return null;
  const fr = rf(a.rig)[boneId]!.frame; const q = qualityOf(a);
  const e = { u: [Infinity, -Infinity], v: [Infinity, -Infinity], w: [Infinity, -Infinity], z: [Infinity, -Infinity] };
  let any = false;
  for (const s of vol.body) {
    if (s.bone !== boneId || s.repeat) continue;
    const { positions: p } = copyMesh(s, q);
    for (let i = 0; i < p.length; i += 3) {
      any = true;
      const [u, v, w] = [p[i]!, p[i + 1]!, p[i + 2]!];
      e.u = [Math.min(e.u[0]!, u), Math.max(e.u[1]!, u)]; e.v = [Math.min(e.v[0]!, v), Math.max(e.v[1]!, v)]; e.w = [Math.min(e.w[0]!, w), Math.max(e.w[1]!, w)];
      const z = fr.o[2] + u * fr.u[2] + v * fr.v[2] + w * fr.w[2];
      e.z = [Math.min(e.z[0]!, z), Math.max(e.z[1]!, z)];
    }
  }
  return any ? e : null;
}
import { flattenGeometry } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
/** 🧱️ Gives every flat shape (no `solid`) on the given bones a prism solid, so that its view stays exact when the bone is spread, twisted or bent sideways:
 * elevation shapes sweep along w over the rest world z range of the bone's body solids; front and plan shapes are re-expressed in the bone frame (the two bone axes that lie most in the paper) and swept along the third axis over the bone's solid range on it.
 * In the rest pose each prism projects to the original outline (up to the small tilt of the bone frame against the paper); curves are flattened to polygons. */
export function solidify(a: J, bones: ReadonlySet<string>, scales: readonly string[] = SCALES, views: readonly string[] = VIEWS, thick?: Record<string, number>): number {
  const rest = rf(a.rig);
  let n = 0;
  for (const k of scales) {
    const rep = a.representations[k]; if (rep.status !== "authored") continue;
    for (const view of views) {
      rep.geometry[view] = rep.geometry[view].map((s: J) => {
        if (s.solid || s.role === "ground" || !bones.has(s.bone) || s.kind === "member") return s;
        let bone = s.bone; let ext = boneExtents(a, k, bone);
        for (let p = a.rig.bones.find((b: J) => b.id === bone)?.parent; !ext && p; p = a.rig.bones.find((b: J) => b.id === p)?.parent) ext = boneExtents(a, k, p);
        if (!ext) return s;
        const flat = flattenGeometry(s);
        if (flat.rings.length !== 1) return s;
        const ring = flat.rings[0]!;
        const fr = rest[bone]!.frame;
        const base: J = { id: s.id, bone: s.bone, role: s.role, z: s.z, filled: s.filled, tone: s.tone };
        for (const key of ["region", "regions", "material", "hatch", "penMm"]) if (s[key] !== undefined) base[key] = s[key];
        n++;
        if (view === "elevation") return { ...base, kind: "poly", points: ring.map((p: J) => [r(p.x, 5), r(p.y, 5)]), closed: flat.closed, solid: { kind: "extruded", sweep: "w", z0: r(ext.z[0]!, 4), z1: r(ext.z[1]!, 4) } };
        const dir = view === "front" ? 0 : 1;
        const axes = (["u", "v", "w"] as const).map((ax) => ({ ax, d: Math.abs(fr[ax][dir]) }));
        const sweep = axes.reduce((p, q) => (q.d > p.d ? q : p)).ax;
        const [a1, a2] = (["u", "v", "w"] as const).filter((ax) => ax !== sweep);
        const mid = (ext[sweep][0]! + ext[sweep][1]!) / 2;
        const [ia, ib] = view === "front" ? [2, 1] : [0, 2];
        const A = [[fr[a1!][ia]!, fr[a2!][ia]!], [fr[a1!][ib]!, fr[a2!][ib]!]];
        const det = A[0]![0]! * A[1]![1]! - A[0]![1]! * A[1]![0]!;
        const pts = ring.map((p: J) => {
          const r0 = p.x - fr.o[ia]! - mid * fr[sweep][ia]!, r1 = p.y - fr.o[ib]! - mid * fr[sweep][ib]!;
          return [r((r0 * A[1]![1]! - A[0]![1]! * r1) / det, 5), r((A[0]![0]! * r1 - A[1]![0]! * r0) / det, 5)];
        });
        const half = (thick?.[bone] ?? 0.01) * (ext[sweep][1]! - ext[sweep][0]!) / 2;
        const span = sweep === "w" ? ext.z : [mid - Math.max(0.005, half), mid + Math.max(0.005, half)];
        return { ...base, kind: "poly", points: pts, closed: flat.closed, solid: { kind: "extruded", sweep, z0: r(span[0]!, 4), z1: r(span[1]!, 4) } };
      });
    }
  }
  return n;
}
