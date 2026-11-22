/** 🧰️ Shared helpers of the HW-11 generators: read a base human, probe poses, measure the default-pose extents, write the asset document.
 * @see ../../📋️brief.md */
import { join, resolve } from "node:path";
import { evaluate, viewExtent, type Asset, type BoneOffset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { bodyDimensions, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

export const HUMANS = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
export type Json = any;
export type Bones = Record<string, BoneOffset>;
export type V3 = { x: number; y: number; z: number };

/** 📥️ The asset of a catalog human, or of a JSON file path. */
export async function readAsset(slugOrFile: string): Promise<Json> {
  const path = slugOrFile.endsWith(".json") ? slugOrFile : join(HUMANS, slugOrFile, "🔣️.json");
  return structuredClone((await Bun.file(path).json()).asset);
}

/** 💾️ Writes the asset document of a human with the schema header. */
export async function writeAsset(asset: Json): Promise<string> {
  const path = join(HUMANS, asset.id, "🔣️.json");
  await Bun.write(path, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
  return path;
}

/** 🦴️ World frames of every bone in a probe pose made of the given offsets (feet grounded automatically when `contacts` are given). */
export function probe(asset: Json, bones: Bones, contacts: string[] = []): Record<string, { o: V3; tip: V3; u: V3; v: V3; w: V3 }> {
  const a = structuredClone(asset);
  a.poses = [...a.poses.filter((p: Json) => p.id !== "probe"), { ...structuredClone(a.poses[0]), id: "probe", bones, contacts, key: true, loads: [] }];
  const state = evaluate(a as Asset, { kind: "pose", pose: "probe" });
  return Object.fromEntries(Object.entries(state.bones).map(([id, b]) => [id, { o: b.frame.o, tip: b.tip, u: b.frame.u, v: b.frame.v, w: b.frame.w }]));
}

/** 📏️ Default-pose extents of every authored scale: height and forward width of the side elevation, lateral depth of the front, plan extents. */
export function extents(asset: Json): Record<string, { height: number; width: number; frontW: number; frontH: number; planX: number; planZ: number }> {
  const out: Record<string, any> = {};
  for (const d of [50, 100, 200, 500]) {
    if (asset.representations[`${d}`]?.status !== "authored") continue;
    const s = evaluate(asset as Asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d as 50 } });
    const f = viewExtent(s, "front")!, p = viewExtent(s, "plan")!;
    out[d] = { height: s.bounds.maxY - s.bounds.minY, width: s.bounds.maxX - s.bounds.minX, frontW: f.horizontal, frontH: f.vertical, planX: p.horizontal, planZ: p.vertical };
  }
  return out;
}

/** 📐️ Sets `dimensions` (forward width, height, lateral depth) to the middle of the interval every check accepts: the default-pose body of each authored volume within one smallest feature (7.5, 15, 30, 75 mm, the silhouette guard) and each drawn view within max(2 %, 15 mm) (the validator); rounded to millimetres, the mass to the sum of the bone masses. */
export function measureDimensions(asset: Json): Json {
  const kg = Math.round(asset.rig.bones.reduce((sum: number, b: Json) => sum + (b.mass?.kg ?? 0), 0) * 100) / 100;
  const state = evaluate(asset as Asset, { kind: "pose", pose: asset.presentation.defaultPose });
  const e = extents(asset);
  const feature: Record<string, number> = { "50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075 };
  const lo = [-Infinity, -Infinity, -Infinity], hi = [Infinity, Infinity, Infinity];
  const keep = (k: number, v: number, tol: number) => { lo[k] = Math.max(lo[k]!, v - tol); hi[k] = Math.min(hi[k]!, v + tol); };
  for (const d of Object.keys(e)) {
    if (asset.volumes[d]?.status === "authored") bodyDimensions(worldSolids(asset as Asset, state.bones as never, Number(d))).forEach((v, k) => keep(k, v, feature[d]! * 0.98));
    const x = e[d]!;
    for (const [k, v] of [[0, x.width], [0, x.planX], [1, x.height], [1, x.frontH], [2, x.frontW], [2, x.planZ]] as const) keep(k, v, Math.max(0.015, 0.02 * v) * 0.98);
  }
  const r = (v: number) => Math.round(v * 1000) / 1000;
  const mid = [0, 1, 2].map((k) => (lo[k]! <= hi[k]! ? (lo[k]! + hi[k]!) / 2 : NaN));
  if (mid.some(Number.isNaN)) console.log("[DEBUG] no common dimension", JSON.stringify({ lo, hi }));
  asset.dimensions = { ...asset.dimensions, massKg: kg, widthM: r(mid[0]!), heightM: r(mid[1]!), depthM: r(mid[2]!) };
  return e;
}

/** 🔤️ A bilingual text. */
export const t = (en: string, de: string) => ({ en, de });

/** 🧹️ Removes every shape, solid, anchor, region binding and mass of the named bones (and of shapes/solids whose id matches `idPattern`). */
export function dropParts(asset: Json, bones: string[], idPattern?: RegExp, regions: string[] = [], materials: string[] = []): void {
  const gone = (o: Json) => bones.includes(o.bone) || (idPattern !== undefined && idPattern.test(o.id ?? "")) || regions.includes(o.region);
  asset.rig.bones = asset.rig.bones.filter((b: Json) => !bones.includes(b.id));
  asset.anchors = asset.anchors.filter((a: Json) => !bones.includes(a.bone));
  for (const rep of Object.values(asset.representations) as Json[]) {
    if (rep.status !== "authored") continue;
    for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = (rep.geometry[v] ?? []).filter((s: Json) => !gone(s));
  }
  for (const vol of Object.values(asset.volumes) as Json[]) {
    if (vol.status !== "authored") continue;
    for (const list of ["body", "thermal", "structure"]) vol[list] = vol[list].filter((s: Json) => !gone(s));
    const ids = new Set(vol.body.map((s: Json) => s.id));
    for (const list of ["thermal", "structure"]) vol[list] = vol[list].filter((s: Json) => s.of === undefined || ids.has(s.of));
  }
  asset.thermal.regions = asset.thermal.regions.filter((r: Json) => !regions.includes(r.id));
  asset.materials = asset.materials.filter((m: Json) => !materials.includes(m.id));
}

const r4 = (v: number) => Math.round(v * 1e5) / 1e5;
/** ▢️ A closed rounded rectangle `[x0, x1] × [y0, y1]` with corner radius `r` (`n` segments per corner), counter-clockwise. */
export function roundRect(x0: number, x1: number, y0: number, y1: number, r: number, n = 4): number[][] {
  const pts: number[][] = [];
  const corner = (cx: number, cy: number, a0: number) => { for (let i = 0; i <= n; i++) { const a = a0 + (Math.PI / 2) * (i / n); pts.push([r4(cx + r * Math.cos(a)), r4(cy + r * Math.sin(a))]); } };
  corner(x1 - r, y0 + r, -Math.PI / 2); corner(x1 - r, y1 - r, 0); corner(x0 + r, y1 - r, Math.PI / 2); corner(x0 + r, y0 + r, Math.PI);
  return pts;
}
/** ⬭️ A closed ellipse polygon about (cx, cy) with semi-axes (rx, ry), `n` points. */
export function ellipse(cx: number, cy: number, rx: number, ry: number, n = 24): number[][] {
  return Array.from({ length: n }, (_, i) => [r4(cx + rx * Math.cos((2 * Math.PI * i) / n)), r4(cy + ry * Math.sin((2 * Math.PI * i) / n))]);
}
/** ➕️ Adds shapes to a view of an authored scale, before the shape with id `before` when given (painting order). */
export function addShapes(asset: Json, scale: number, view: string, shapes: Json[], before?: string): void {
  const list = asset.representations[`${scale}`].geometry[view];
  const at = before === undefined ? -1 : list.findIndex((s: Json) => s.id === before);
  if (at < 0) list.push(...shapes); else list.splice(at, 0, ...shapes);
}

/** 📝️ Adds a bilingual sentence to a decision field of a representation (`before` puts it first). */
export function note(asset: Json, scale: number, field: string, en: string, de: string, before = false): void {
  const d = asset.representations[`${scale}`].decisions;
  const old = d[field] ?? { en: "", de: "" };
  d[field] = before ? { en: `${en} ${old.en}`.trim(), de: `${de} ${old.de}`.trim() } : { en: `${old.en} ${en}`.trim(), de: `${old.de} ${de}`.trim() };
}
/** 🔁️ Replaces text in every decision field of every representation and volume (both languages, one pair per call). */
export function reword(asset: Json, en: [string, string], de: [string, string]): void {
  const swap = (x: Json) => { if (x?.en !== undefined) { x.en = x.en.split(en[0]).join(en[1]); x.de = x.de.split(de[0]).join(de[1]); } };
  for (const rep of Object.values(asset.representations) as Json[]) if (rep.decisions) for (const v of Object.values(rep.decisions)) swap(v);
  for (const vol of Object.values(asset.volumes) as Json[]) for (const v of vol.decisions ?? []) swap(v);
}

const KEY_DE: Record<string, RegExp> = {};
/** ✂️ Drops the sentences that mention removed parts from every decision and assumption text (English and German sentence by sentence where both have the same number of sentences, otherwise inside clauses separated by ";" or ","); returns the texts it could not clean. */
export function scrub(asset: Json, en: RegExp, de: RegExp): string[] {
  void KEY_DE;
  const left: string[] = [];
  const split = (s: string) => s.split(/(?<=\.)\s+/);
  const clean = (x: Json, path: string) => {
    if (!en.test(x.en)) return;
    const e = split(x.en), d = split(x.de);
    if (e.length === d.length) {
      const keep = e.map((s, i) => !en.test(s) && !de.test(d[i]!));
      x.en = e.filter((_, i) => keep[i]).join(" ");
      x.de = d.filter((_, i) => keep[i]).join(" ");
    }
    if (en.test(x.en)) left.push(`${path}: ${x.en}`);
  };
  for (const [k, rep] of Object.entries(asset.representations) as [string, Json][]) for (const [f, v] of Object.entries(rep.decisions ?? {})) clean(v, `representations.${k}.decisions.${f}`);
  for (const [k, vol] of Object.entries(asset.volumes) as [string, Json][]) (vol.decisions ?? []).forEach((v: Json, i: number) => clean(v, `volumes.${k}.decisions[${i}]`));
  asset.thermal.assumptions.forEach((v: Json, i: number) => clean(v, `thermal.assumptions[${i}]`));
  asset.structure.assumptions.forEach((v: Json, i: number) => clean(v, `structure.assumptions[${i}]`));
  return left;
}

/** 🩹️ Fills every empty decision text of a representation field or a volume entry with the given scale-specific text. */
export function fillEmpty(asset: Json, rep: Record<string, Record<string, [string, string]>>, vol: Record<string, [string, string][]> = {}): string[] {
  const left: string[] = [];
  for (const [k, r] of Object.entries(asset.representations) as [string, Json][]) for (const [f, v] of Object.entries(r.decisions) as [string, Json][]) if (v.en.trim() === "" || v.de.trim() === "") { const x = rep[k]?.[f]; if (x) { v.en = x[0]; v.de = x[1]; } else left.push(`${k}.${f}`); }
  for (const [k, r] of Object.entries(asset.volumes) as [string, Json][]) (r.decisions ?? []).forEach((v: Json, i: number) => { if (v.en.trim() === "" || v.de.trim() === "") { const x = vol[k]?.[i]; if (x) { v.en = x[0]; v.de = x[1]; } else left.push(`volumes.${k}[${i}]`); } });
  return left;
}
