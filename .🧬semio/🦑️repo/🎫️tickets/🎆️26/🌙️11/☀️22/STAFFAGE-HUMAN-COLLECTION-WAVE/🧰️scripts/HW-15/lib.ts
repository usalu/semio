/** 🧰️ Shared generator helpers of worker HW-15: load and write catalog assets, prune bones, shapes, solids and regions, rename and offset ids, and forward kinematics through the library's own rig solver.
 * @see ../../📋️brief.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md
 */
import { join, resolve } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";

export const ROOT = resolve(import.meta.dir, "../../../../../../../../..");
export const S = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage");
export const CATALOG = join(S, "🗂️catalog");
export const T = resolve(import.meta.dir, "../..");
export const GEN = join(T, "🗑️generated/HW-15");
export const SCALES = ["500", "200", "100", "50"] as const;
export const VIEWS = ["elevation", "front", "plan"] as const;

const rig = await import(join(S, "🔨️modules/🦴️rig/🟦️.ts"));
export const solveChain: (rig: any, offsets: any, root: any) => Record<string, any> = rig.solveChain;
export const restFrames: (rig: any) => Record<string, any> = rig.restFrames;

/** 📥️ Reads an asset document of a category folder. */
export async function load(category: string, slug: string): Promise<any> {
  return await Bun.file(join(CATALOG, category, slug, "🔣️.json")).json();
}

/** 🖨️ Formats JSON with two-space indent and every array of plain numbers on one line, like the catalog files. */
export function format(value: unknown): string {
  const out = (v: any, ind: string): string => {
    if (Array.isArray(v)) {
      if (v.length === 0) return "[]";
      if (v.every((x) => typeof x === "number")) return `[${v.map((x) => num(x)).join(", ")}]`;
      return `[\n${v.map((x) => ind + "  " + out(x, ind + "  ")).join(",\n")}\n${ind}]`;
    }
    if (v !== null && typeof v === "object") {
      const keys = Object.keys(v).filter((k) => v[k] !== undefined);
      if (keys.length === 0) return "{}";
      return `{\n${keys.map((k) => `${ind}  ${JSON.stringify(k)}: ${out(v[k], ind + "  ")}`).join(",\n")}\n${ind}}`;
    }
    if (typeof v === "number") return num(v);
    return JSON.stringify(v);
  };
  return out(value, "") + "\n";
}

/** 🔢️ A number rounded to 6 decimals without float noise. */
export function num(x: number): string {
  const r = Math.round(x * 1e6) / 1e6;
  return String(Object.is(r, -0) ? 0 : r);
}

/** 💾️ Writes the asset document of a human (or to an explicit file). */
export function save(doc: any, file?: string): string {
  const path = file ?? join(CATALOG, "👤️humans", doc.asset.id, "🔣️.json");
  mkdirSync(resolve(path, ".."), { recursive: true });
  writeFileSync(path, format(doc));
  return path;
}

/** 🧬️ Deep clone. */
export const clone = <X>(x: X): X => structuredClone(x);

/** 🌳️ The ids of the given bones and all their descendants. */
export function subtree(asset: any, ids: Iterable<string>): Set<string> {
  const out = new Set(ids);
  let grew = true;
  while (grew) {
    grew = false;
    for (const b of asset.rig.bones) if (b.parent && out.has(b.parent) && !out.has(b.id)) (out.add(b.id), (grew = true));
  }
  return out;
}

/** ✏️ Every shape list of the asset (representations and symbols) with its scale and view. */
export function* shapeLists(asset: any): Generator<{ scale: string; view: string; list: any[]; set: (l: any[]) => void }> {
  for (const [scale, rep] of Object.entries<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    for (const view of VIEWS) {
      const g = rep.geometry;
      if (Array.isArray(g[view])) yield { scale, view, list: g[view], set: (l) => (g[view] = l) };
    }
  }
}

/** 🧊️ Every volume solid list (body, thermal, structure) of every authored scale. */
export function* solidLists(asset: any): Generator<{ scale: string; kind: string; list: any[]; set: (l: any[]) => void }> {
  for (const [scale, vol] of Object.entries<any>(asset.volumes)) {
    if (vol.status !== "authored") continue;
    for (const kind of ["body", "thermal", "structure"]) if (Array.isArray(vol[kind])) yield { scale, kind, list: vol[kind], set: (l) => (vol[kind] = l) };
  }
}

/** ✂️ Removes shapes matching a predicate everywhere. */
export function removeShapes(asset: any, drop: (s: any) => boolean): number {
  let n = 0;
  for (const l of shapeLists(asset)) {
    const kept = l.list.filter((s) => !drop(s));
    n += l.list.length - kept.length;
    l.set(kept);
  }
  return n;
}

/** ✂️ Removes solids matching a predicate and every `of` reference to them, and drops removed bones from `members` and `weights`. */
export function removeSolids(asset: any, drop: (s: any) => boolean, bones: Set<string> = new Set()): number {
  let n = 0;
  for (const [, vol] of Object.entries<any>(asset.volumes)) {
    if (vol.status !== "authored") continue;
    const gone = new Set<string>();
    for (const kind of ["body", "thermal", "structure"]) {
      const list: any[] = vol[kind] ?? [];
      const kept = list.filter((s) => {
        const d = drop(s) || (s.of !== undefined && gone.has(s.of)) || (s.bone !== undefined && bones.has(s.bone)) || (s.member !== undefined && bones.has(s.member));
        if (d) gone.add(s.id);
        return !d;
      });
      n += list.length - kept.length;
      for (const s of kept) {
        if (Array.isArray(s.members)) s.members = s.members.filter((m: string) => !bones.has(m));
        if (Array.isArray(s.members) && s.members.length === 0) delete s.members;
      }
      vol[kind] = kept;
    }
  }
  return n;
}

/** 🦴️ Removes bones with their subtree, anchors, shapes, solids, offsets, contacts, events and loads that reference them. */
export function removeBones(asset: any, ids: Iterable<string>): Set<string> {
  const gone = subtree(asset, ids);
  asset.rig.bones = asset.rig.bones.filter((b: any) => !gone.has(b.id));
  const anchors = new Set(asset.anchors.filter((a: any) => gone.has(a.bone)).map((a: any) => a.id));
  asset.anchors = asset.anchors.filter((a: any) => !gone.has(a.bone));
  removeShapes(asset, (s) => gone.has(s.bone) || (s.solid?.axisBone !== undefined && gone.has(s.solid.axisBone)));
  for (const sym of asset.symbols ?? []) for (const v of VIEWS) if (Array.isArray(sym.geometry?.[v])) sym.geometry[v] = sym.geometry[v].filter((s: any) => !gone.has(s.bone));
  removeSolids(asset, (s) => Array.isArray(s.weights) && s.weights.some((w: any) => gone.has(w.bone)), gone);
  const offsets = (bones: any) => { for (const id of gone) delete bones?.[id]; };
  for (const p of asset.poses) {
    offsets(p.bones);
    p.contacts = p.contacts.filter((c: string) => !anchors.has(c));
    p.loads = p.loads.filter((l: any) => !anchors.has(l.anchor) && !gone.has(l.bone) && !gone.has(l.distributed?.bone));
  }
  for (const c of asset.clips) {
    for (const k of c.keys) offsets(k.bones);
    c.contacts = c.contacts.filter((s: any) => !anchors.has(s.anchor));
    c.events = c.events.filter((e: any) => !anchors.has(e.target));
  }
  asset.structure.supports = asset.structure.supports.filter((s: string) => !anchors.has(s));
  asset.thermal.emitters = asset.thermal.emitters.filter((e: any) => !anchors.has(e.anchor));
  return gone;
}

/** 🌡️ Removes a thermal region with its bindings and tracks (shapes and solids that name it must be gone first). */
export function removeRegion(asset: any, id: string): void {
  asset.thermal.regions = asset.thermal.regions.filter((r: any) => r.id !== id);
  for (const p of asset.poses) delete p.thermal.regions[id];
  for (const c of asset.clips) {
    for (const k of c.keys) if (k.thermal?.regions) delete k.thermal.regions[id];
    c.tracks = c.tracks.filter((t: any) => !t.target.startsWith(`region:${id}:`));
  }
}

/** 🔍️ Every place still naming a string value (for leftovers). */
export function find(obj: any, needle: string, path = ""): string[] {
  if (obj === needle) return [path];
  if (Array.isArray(obj)) return obj.flatMap((x, i) => find(x, needle, `${path}[${i}]`));
  if (obj && typeof obj === "object") return Object.entries(obj).flatMap(([k, v]) => find(v, needle, `${path}.${k}`));
  return [];
}

/** 🧭️ The world frames of a pose offset set, the root bone at its rest placement plus `root` (ground ignored). */
export function frames(asset: any, offsets: Record<string, any>, root = { x: 0, y: 0, angleDeg: 0 }): Record<string, any> {
  return solveChain(asset.rig, offsets, root);
}

export type V3 = [number, number, number];
export const v3 = (p: any): V3 => (Array.isArray(p) ? [p[0], p[1], p[2]] : [p.x, p.y, p.z]);
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const norm = (a: V3): number => Math.hypot(...a);
export const unit = (a: V3): V3 => mul(a, 1 / norm(a));

/** 📍️ A bone-local point (u, v, w) in world coordinates of a frame. */
export function toWorld(f: any, p: V3): V3 {
  return add(v3(f.o), add(mul(v3(f.u), p[0]), add(mul(v3(f.v), p[1]), mul(v3(f.w), p[2]))));
}

/** 📍️ A world point in the bone-local (u, v, w) coordinates of a frame. */
export function toLocal(f: any, p: V3): V3 {
  const d = sub(p, v3(f.o));
  return [dot(d, v3(f.u)), dot(d, v3(f.v)), dot(d, v3(f.w))];
}

/** 🌐️ Bilingual text. */
export const t = (en: string, de: string) => ({ en, de });
