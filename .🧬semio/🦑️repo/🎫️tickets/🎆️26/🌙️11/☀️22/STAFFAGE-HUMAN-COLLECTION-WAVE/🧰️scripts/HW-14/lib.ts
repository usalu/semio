/** 🧰️ Shared helpers of HW-14's generators: base snapshots, bilingual texts, rest and pose frames through the library's `evaluate`, shape and solid traversal, bone removal and the shipped document writer.
 * @see ../../📋️brief.md */
import { mkdirSync, writeFileSync, readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

export const ROOT = resolve(import.meta.dir, "../../../../../../../../..");
export const STAFFAGE = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage");
export const CATALOG = join(STAFFAGE, "🗂️catalog/👤️humans");
export const TICKET = resolve(import.meta.dir, "../..");
export const GENERATED = join(TICKET, "🗑️generated/HW-14");
export const SCALES = ["50", "100", "200", "500"] as const;
export const VIEWS = ["elevation", "front", "plan"] as const;
export const LISTS = ["body", "thermal", "structure"] as const;

export type Json = any;
export type V3 = { x: number; y: number; z: number };
export type Frame = { o: V3; u: V3; v: V3; w: V3 };

/** 🧷️ The base figure: the snapshot in the generated folder (taken once from the catalog so that the derivation is stable while others edit the base), else the live catalog file. */
export function loadBase(slug: string): Json {
  const snap = join(GENERATED, "snap", `${slug}.json`);
  const path = existsSync(snap) ? snap : join(CATALOG, slug, "🔣️.json");
  return JSON.parse(readFileSync(path, "utf8")).asset;
}

export const T = (en: string, de: string) => ({ en, de });

/** 🖊️ Writes the shipped asset document. */
export function writeAsset(asset: Json): string {
  const dir = join(CATALOG, asset.id);
  mkdirSync(dir, { recursive: true });
  const path = join(dir, "🔣️.json");
  writeFileSync(path, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
  return path;
}

const REST: Json = { id: "__rest", title: T("r", "r"), description: T("r", "r"), bones: {}, ground: "authored", contacts: [], thermal: { regions: {}, emitters: {} }, loads: [] };

/** 🦴️ World frames of every bone in a pose (an id of the asset, an inline pose object, or null for the rest placement with the root as authored). */
export function frames(asset: Json, pose: string | Json | null): Record<string, Frame & { tip: V3 }> {
  const p = pose === null ? REST : typeof pose === "string" ? null : { ...REST, ...pose, id: "__probe" };
  const a = p === null ? asset : { ...asset, poses: [...asset.poses, p] };
  const s = evaluate(a, { kind: "pose", pose: p === null ? (pose as string) : p.id });
  const out: Record<string, Frame & { tip: V3 }> = {};
  for (const [id, b] of Object.entries(s.bones) as [string, Json][]) out[id] = { ...b.frame, tip: b.tip };
  return out;
}

/** 📍️ The world position of a bone-local point (u, v, w) in a frame. */
export function at(f: Frame, u: number, v: number, w = 0): V3 {
  return { x: f.o.x + f.u.x * u + f.v.x * v + f.w.x * w, y: f.o.y + f.u.y * u + f.v.y * v + f.w.y * w, z: f.o.z + f.u.z * u + f.v.z * v + f.w.z * w };
}

/** 🔁️ Calls `fn` for every shape of every representation and view (symbols excluded). */
export function eachShapeList(asset: Json, fn: (list: Json[], scale: string, view: string) => Json[] | void): void {
  for (const k of SCALES) {
    const r = asset.representations[k];
    if (r?.status !== "authored") continue;
    for (const v of VIEWS) if (r.geometry[v]) { const out = fn(r.geometry[v], k, v); if (out) r.geometry[v] = out; }
  }
}

/** ✂️ Removes bones with every shape, solid (and the thermal or structural references to removed body solids), anchor, offset and load bound to them. */
export function removeBones(asset: Json, ids: string[]): void {
  const gone = new Set(ids);
  asset.rig.bones = asset.rig.bones.filter((b: Json) => !gone.has(b.id));
  eachShapeList(asset, (l) => l.filter((s) => !gone.has(s.bone)));
  for (const k of SCALES) {
    const vol = asset.volumes[k];
    if (vol?.status !== "authored") continue;
    const dropped = new Set(vol.body.filter((s: Json) => gone.has(s.bone)).map((s: Json) => s.id));
    for (const l of LISTS) vol[l] = vol[l].filter((s: Json) => !gone.has(s.bone) && !(s.of && dropped.has(s.of)));
  }
  const anchors = new Set(asset.anchors.filter((a: Json) => gone.has(a.bone)).map((a: Json) => a.id));
  asset.anchors = asset.anchors.filter((a: Json) => !anchors.has(a.id));
  for (const p of asset.poses) {
    for (const id of ids) delete p.bones[id];
    p.contacts = p.contacts.filter((c: string) => !anchors.has(c));
    p.loads = p.loads.filter((l: Json) => !anchors.has(l.anchor) && !gone.has(l.bone));
  }
  for (const c of asset.clips) for (const key of c.keys) if (key.bones) for (const id of ids) delete key.bones[id];
}

/** 🗑️ Removes shapes (by id) from every view and scale, and body solids (by id) with their references. */
export function removeShapes(asset: Json, ids: string[]): void {
  const gone = new Set(ids);
  eachShapeList(asset, (l) => l.filter((s) => !gone.has(s.id)));
}
export function removeSolids(asset: Json, scale: string, ids: string[]): void {
  const vol = asset.volumes[scale];
  const gone = new Set(ids);
  for (const l of LISTS) vol[l] = vol[l].filter((s: Json) => !gone.has(s.id) && !(s.of && gone.has(s.of)));
}

/** 🔤️ Every bilingual text of a value with its path. */
export function texts(x: Json, path = "", out: [string, Json][] = []): [string, Json][] {
  if (x && typeof x === "object" && !Array.isArray(x)) {
    const keys = Object.keys(x);
    if (keys.length === 2 && keys.includes("en") && keys.includes("de")) { out.push([path, x]); return out; }
    for (const k of keys) texts(x[k], `${path}/${k}`, out);
  } else if (Array.isArray(x)) x.forEach((v, i) => texts(v, `${path}[${i}]`, out));
  return out;
}

export const round = (x: number, d = 5) => Math.round(x * 10 ** d) / 10 ** d;

/** 🏷️ Renames a thermal region everywhere: its definition, shape `region`/`regions`, thermal solids, pose and clip-key bindings, emitters and region tracks. */
export function renameRegion(asset: Json, from: string, to: string): void {
  const walk = (x: Json): void => {
    if (Array.isArray(x)) { x.forEach(walk); return; }
    if (!x || typeof x !== "object") return;
    for (const k of Object.keys(x)) {
      if (k === "region" && x[k] === from) x[k] = to;
      else if (k === "regions" && Array.isArray(x[k])) x[k] = x[k].map((r: string) => (r === from ? to : r));
      else if (k === "regions" && x[k] && typeof x[k] === "object" && from in x[k]) { x[k][to] = x[k][from]; delete x[k][from]; walk(x[k]); }
      else if (k === "target" && typeof x[k] === "string" && x[k].startsWith(`region:${from}:`)) x[k] = x[k].replace(`region:${from}:`, `region:${to}:`);
      else walk(x[k]);
    }
  };
  for (const r of asset.thermal.regions) if (r.id === from) r.id = to;
  walk(asset);
}

/** 🏷️ Renames a material everywhere (definition and every `material` field). */
export function renameMaterial(asset: Json, from: string, to: string): void {
  const walk = (x: Json): void => {
    if (Array.isArray(x)) { x.forEach(walk); return; }
    if (!x || typeof x !== "object") return;
    for (const k of Object.keys(x)) { if (k === "material" && x[k] === from) x[k] = to; else walk(x[k]); }
  };
  for (const m of asset.materials) if (m.id === from) m.id = to;
  walk(asset);
}

/** 🏷️ Renames a bone everywhere: rig, shapes, solids (bone, axisBone, member, members, weights), anchors, loads, poses and clip keys, respiration motion. */
export function renameBone(asset: Json, from: string, to: string): void {
  const walk = (x: Json): void => {
    if (Array.isArray(x)) { x.forEach((v, i) => { if (v === from && false) x[i] = to; walk(v); }); return; }
    if (!x || typeof x !== "object") return;
    for (const k of Object.keys(x)) {
      if (["bone", "parent", "axisBone", "member"].includes(k) && x[k] === from) x[k] = to;
      else if (k === "members" && Array.isArray(x[k])) x[k] = x[k].map((m: string) => (m === from ? to : m));
      else if (k === "bones" && x[k] && !Array.isArray(x[k]) && typeof x[k] === "object" && from in x[k]) { x[k][to] = x[k][from]; delete x[k][from]; walk(x[k]); }
      else walk(x[k]);
    }
  };
  for (const b of asset.rig.bones) if (b.id === from) b.id = to;
  walk(asset);
}

/** 🏷️ Renames shape and solid ids (and `of` references) by a string replacement on ids that contain `from`. */
export function renameIds(asset: Json, from: string, to: string): void {
  eachShapeList(asset, (l) => { for (const s of l) if (s.id.includes(from)) s.id = s.id.replaceAll(from, to); });
  for (const k of SCALES) {
    const vol = asset.volumes[k];
    if (vol?.status !== "authored") continue;
    for (const l of LISTS) for (const s of vol[l]) { if (s.id.includes(from)) s.id = s.id.replaceAll(from, to); if (s.of?.includes(from)) s.of = s.of.replaceAll(from, to); }
  }
}

import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

/** 📦️ World extent (min, max per axis) of the body solids of the given bones at 1:`scale` in a pose (null: rest). */
export function solidExtent(asset: Json, scale: string, bones: string[], pose: string | Json | null = null): { min: V3; max: V3 } {
  const p = pose === null ? REST : typeof pose === "string" ? null : { ...REST, ...pose, id: "__probe" };
  const a = p === null ? asset : { ...asset, poses: [...asset.poses, p] };
  const s = evaluate(a, { kind: "pose", pose: p === null ? (pose as string) : p.id });
  const keep = new Set(bones);
  const ws = worldSolids(a, s.bones as never, Number(scale)).filter((w: Json) => keep.has(w.bone));
  const min = { x: Infinity, y: Infinity, z: Infinity }, max = { x: -Infinity, y: -Infinity, z: -Infinity };
  for (const w of ws) for (let i = 0; i < w.positions.length; i += 3) {
    const [x, y, z] = [w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!];
    min.x = Math.min(min.x, x); min.y = Math.min(min.y, y); min.z = Math.min(min.z, z);
    max.x = Math.max(max.x, x); max.y = Math.max(max.y, y); max.z = Math.max(max.z, z);
  }
  return { min, max };
}

/** 📏️ Sets `dimensions` (width along x and height from the side elevation, depth from the front elevation) from the 1:`scale` drawing of the default pose. */
export function measureDimensions(asset: Json, scale = 50): void {
  const s = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const ext = (view: "elevation" | "front") => {
    let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
    for (const e of s.shapes[view]) for (const r of e.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
    return [x1 - x0, y1 - y0];
  };
  const [w, h] = ext("elevation");
  const [d] = ext("front");
  asset.dimensions.widthM = round(w!, 3);
  asset.dimensions.heightM = round(h!, 3);
  asset.dimensions.depthM = round(d!, 3);
}

/** 〰️ Ramer–Douglas–Peucker simplification of a closed ring to `tol` metres. */
export function simplify(ring: { x: number; y: number }[], tol: number): { x: number; y: number }[] {
  const rdp = (pts: { x: number; y: number }[]): { x: number; y: number }[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const L = Math.hypot(b.x - a.x, b.y - a.y) || 1e-12;
    let [far, at] = [0, 0];
    for (let i = 1; i < pts.length - 1; i++) { const d = Math.abs((b.x - a.x) * (a.y - pts[i]!.y) - (a.x - pts[i]!.x) * (b.y - a.y)) / L; if (d > far) [far, at] = [d, i]; }
    return far <= tol ? [a, b] : [...rdp(pts.slice(0, at + 1)).slice(0, -1), ...rdp(pts.slice(at))];
  };
  let i0 = 0;
  for (let i = 0; i < ring.length; i++) if (ring[i]!.x < ring[i0]!.x) i0 = i;
  const r = [...ring.slice(i0), ...ring.slice(0, i0)];
  let j = 0;
  for (let i = 0; i < r.length; i++) if (Math.hypot(r[i]!.x - r[0]!.x, r[i]!.y - r[0]!.y) > Math.hypot(r[j]!.x - r[0]!.x, r[j]!.y - r[0]!.y)) j = i;
  return [...rdp(r.slice(0, j + 1)).slice(0, -1), ...rdp([...r.slice(j), r[0]!]).slice(0, -1)];
}

/** 🩻️ The projected outlines (outer boundaries of the connected parts, largest first) of the body solids that `keep` selects at 1:`scale` in a pose (default: the presentation pose), in view coordinates (front: z, y; plan: x, z; elevation: x, y), traced on a raster of `cell` metres and simplified to `tol`. */
export function traceOutline(asset: Json, scale: string, view: "elevation" | "front" | "plan", keep: (solid: Json) => boolean, tol = 0.0015, pose?: string, cell = 0.001): number[][][] {
  const s = evaluate(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose });
  const ids = new Set(asset.volumes[scale].body.filter(keep).map((x: Json) => x.id));
  const ws = worldSolids(asset, s.bones as never, Number(scale)).filter((w: Json) => ids.has(w.id));
  const pts: number[][] = [];
  const tris: number[][] = [];
  for (const w of ws) {
    const p = w.positions, base = pts.length;
    for (let i = 0; i < p.length; i += 3) pts.push(view === "front" ? [p[i + 2]!, p[i + 1]!] : view === "plan" ? [p[i]!, p[i + 2]!] : [p[i]!, p[i + 1]!]);
    for (let t = 0; t < w.indices.length; t += 3) tris.push([base + w.indices[t]!, base + w.indices[t + 1]!, base + w.indices[t + 2]!]);
  }
  if (!pts.length) return [];
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const [x, y] of pts) { x0 = Math.min(x0, x!); y0 = Math.min(y0, y!); x1 = Math.max(x1, x!); y1 = Math.max(y1, y!); }
  x0 -= 3 * cell; y0 -= 3 * cell;
  const W = Math.ceil((x1 - x0) / cell) + 4, H = Math.ceil((y1 - y0) / cell) + 4;
  const M = new Uint8Array(W * H);
  for (const [a, b, c] of tris) {
    const P = [pts[a]!, pts[b]!, pts[c]!].map(([x, y]) => [(x! - x0) / cell, (y! - y0) / cell]);
    const jy0 = Math.max(0, Math.floor(Math.min(P[0]![1]!, P[1]![1]!, P[2]![1]!))), jy1 = Math.min(H - 1, Math.ceil(Math.max(P[0]![1]!, P[1]![1]!, P[2]![1]!)));
    for (let j = jy0; j <= jy1; j++) {
      const yc = j + 0.5;
      const xs: number[] = [];
      for (let e = 0; e < 3; e++) { const [p, q] = [P[e]!, P[(e + 1) % 3]!]; if ((p[1]! <= yc) !== (q[1]! <= yc)) xs.push(p[0]! + ((yc - p[1]!) / (q[1]! - p[1]!)) * (q[0]! - p[0]!)); }
      if (xs.length < 2) continue;
      const [lo, hi] = [Math.min(...xs), Math.max(...xs)];
      for (let i = Math.max(0, Math.round(lo)); i < Math.min(W, Math.round(hi)); i++) M[j * W + i] = 1;
    }
  }
  const seen = new Uint8Array(W * H);
  const out: { ring: { x: number; y: number }[]; n: number }[] = [];
  for (let start = 0; start < M.length; start++) {
    if (!M[start] || seen[start]) continue;
    const comp: number[] = [start]; seen[start] = 1;
    for (let q = 0; q < comp.length; q++) { const n = comp[q]!; const [i, j] = [n % W, Math.floor(n / W)]; for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { const m = (j + dj!) * W + i + di!; if (M[m] && !seen[m]) { seen[m] = 1; comp.push(m); } } }
    if (comp.length < 9) continue;
    const top = Math.min(...comp);
    const N = [[-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1], [-1, -1]];
    const fg = (i: number, j: number) => i >= 0 && j >= 0 && i < W && j < H && M[j * W + i] === 1;
    const ring: { x: number; y: number }[] = [];
    const [si, sj] = [top % W, Math.floor(top / W)];
    let [ci, cj] = [si, sj];
    let [bi, bj] = [si - 1, sj];
    for (let guard = 0; guard < 8 * comp.length + 16; guard++) {
      ring.push({ x: x0 + (ci + 0.5) * cell, y: y0 + (cj + 0.5) * cell });
      let k0 = N.findIndex(([di, dj]) => ci + di! === bi && cj + dj! === bj);
      let found = false;
      for (let k = 1; k <= 8; k++) {
        const kk = (k0 + k) % 8;
        const [ni, nj] = [ci + N[kk]![0]!, cj + N[kk]![1]!];
        if (fg(ni, nj)) { const pk = (kk + 7) % 8; [bi, bj] = [ci + N[pk]![0]!, cj + N[pk]![1]!]; [ci, cj] = [ni, nj]; found = true; break; }
      }
      if (!found || (ci === si && cj === sj)) break;
      void k0;
    }
    out.push({ ring, n: comp.length });
  }
  return out.sort((a, b) => b.n - a.n).map(({ ring }) => simplify(ring, tol).map((p) => [round(p.x), round(p.y)]));
}
