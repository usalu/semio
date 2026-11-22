/** 🧍️ Person-level helpers of HW-07: regions, tones, bindings, rest frames, dimensions from the measured body and the manifest row.
 * @see ./lib.ts */
import { join } from "node:path";
import { totalPowerW } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { TICKET, frameOf, r, restState, type Json, type T2 } from "./lib.ts";

/** 🎨️ Sets the tone of every shape (all scales, views) bound to a region and of every body solid that a thermal solid of that region references. */
export function retone(asset: Json, region: string, tone: number, scales: readonly string[] = ["50", "100", "200"]): void {
  for (const [k, rep] of Object.entries(asset.representations) as [string, Json][]) if (rep.status === "authored" && scales.includes(k)) for (const v of ["elevation", "front", "plan"]) for (const s of rep.geometry[v]) if (s.region === region) s.tone = tone;
  for (const [k, vol] of Object.entries(asset.volumes) as [string, Json][]) {
    if (vol.status !== "authored" || !scales.includes(k)) continue;
    const ids = new Set(vol.thermal.filter((t: Json) => t.region === region && t.of).map((t: Json) => t.of));
    for (const b of vol.body) if (ids.has(b.id)) b.tone = tone;
  }
}

/** 🔁️ Rebinds every shape and solid of region `from` to region `to` (material and tone optional) and removes region `from`; its area joins `to`. */
export function mergeRegion(asset: Json, from: string, to: string, material?: string, tone?: number, toneScales: readonly string[] = ["50", "100", "200"]): void {
  for (const [k, rep] of Object.entries(asset.representations) as [string, Json][]) if (rep.status === "authored") for (const v of ["elevation", "front", "plan"]) for (const s of rep.geometry[v]) {
    if (s.region === from) { s.region = to; if (material) s.material = material; if (tone !== undefined && toneScales.includes(k)) s.tone = tone; }
    if (s.regions) s.regions = [...new Set(s.regions.map((x: string) => (x === from ? to : x)))];
  }
  for (const [k, vol] of Object.entries(asset.volumes) as [string, Json][]) {
    if (vol.status !== "authored") continue;
    const ids = new Set(vol.thermal.filter((t: Json) => t.region === from && t.of).map((t: Json) => t.of));
    for (const t of vol.thermal) if (t.region === from) { t.region = to; delete t.layer; if (material && !t.of) t.material = material; }
    for (const b of vol.body) if (ids.has(b.id)) { if (material) b.material = material; if (tone !== undefined && toneScales.includes(k)) b.tone = tone; }
  }
  const regions = asset.thermal.regions;
  const a = regions.find((g: Json) => g.id === from), b = regions.find((g: Json) => g.id === to);
  b.areaM2 = r(b.areaM2 + a.areaM2, 4);
  if (a.body && b.body) { b.body.metabolicShare = r(b.body.metabolicShare + a.body.metabolicShare, 5); b.body.skinFraction = r(b.body.skinFraction + a.body.skinFraction, 5); }
  asset.thermal.regions = regions.filter((g: Json) => g.id !== from);
  for (const e of asset.thermal.emitters) if (e.region === from) e.region = to;
}

/** 🌬️ A pose's thermal binding: every region exposed and shaded unless overridden, the plume at its strength. */
export function binding(asset: Json, activityMet: number, breathing: string, plume: number, overrides: Record<string, Json> = {}): Json {
  const regions: Record<string, Json> = {};
  for (const g of asset.thermal.regions) regions[g.id] = { exposedFraction: 1, sunlitFraction: 0, ...(overrides[g.id] ?? {}) };
  const emitters: Record<string, number> = {};
  for (const e of asset.thermal.emitters) if (e.kind !== "breath") emitters[e.id] = plume;
  return { activityMet, breathing, regions, emitters };
}

/** 🧭️ World frame of a bone at rest (scale 50 state). */
export const restFrame = (asset: Json, bone: string) => frameOf(restState(asset, "50"), bone);

/** 📐️ Sets `dimensions` per axis to the middle of the values every check accepts: within one smallest feature of the measured body of every
 * authored volume scale (the catalog guard) and within max(2 %, 15 mm) of the drawn extent of every view of every scale in the default pose
 * (`extent-*` of `validateAsset`); the total mass is the sum of the bones. Throws when no value satisfies all of them. */
export function measureDimensions(asset: Json): { byScale: Record<string, number[]>; drawn: Record<string, number[]> } {
  const byScale: Record<string, number[]> = {}, drawn: Record<string, number[]> = {};
  const lo = [-Infinity, -Infinity, -Infinity], hi = [Infinity, Infinity, Infinity];
  for (const s of ["50", "100", "200", "500"]) {
    if (asset.representations[s]?.status !== "authored") continue;
    const st: Json = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(s) } } as never);
    const ext = (view: string, i: "x" | "y") => { const p = st.shapes[view].filter((e: Json) => e.shape.role !== "ground").flatMap((e: Json) => e.rings.flat()); const v = p.map((q: Json) => q[i]); return Math.max(...v) - Math.min(...v); };
    const views = [[ext("elevation", "x"), ext("plan", "x")], [ext("elevation", "y"), ext("front", "y")], [ext("front", "x"), ext("plan", "y")]];
    drawn[s] = views.flat();
    views.forEach((vals, k) => { for (const v of vals) { lo[k] = Math.max(lo[k]!, v - Math.max(0.02 * v, 0.015) * 0.995); hi[k] = Math.min(hi[k]!, v + Math.max(0.02 * v, 0.015) * 0.995); } });
    if (asset.volumes[s]?.status === "authored") {
      const f = featureOf(Number(s));
      byScale[s] = measureAsset(asset, Number(s), undefined, undefined, f).bbox;
      byScale[s]!.forEach((v, k) => { lo[k] = Math.max(lo[k]!, v - f * 0.995); hi[k] = Math.min(hi[k]!, v + f * 0.995); });
    }
  }
  const keys = ["widthM", "heightM", "depthM"] as const;
  keys.forEach((key, k) => {
    if (lo[k]! > hi[k]!) throw new Error(`dimensions: no ${key} satisfies every scale and view (${lo[k]!.toFixed(4)} > ${hi[k]!.toFixed(4)}) ${JSON.stringify({ byScale, drawn })}`);
    asset.dimensions[key] = r((lo[k]! + hi[k]!) / 2, 3);
  });
  asset.dimensions.massKg = r(asset.rig.bones.reduce((s: number, b: Json) => s + (b.mass?.kg ?? 0), 0), 3);
  return { byScale, drawn };
}

/** 📥️ Writes the manifest row of an asset to `📥️manifest-rows/<id>.json` (shape of the `entries` rows of `🗂️catalog/🔣️.json`). */
export async function writeManifestRow(asset: Json, distinction: T2): Promise<Json> {
  const row = {
    id: asset.id, category: asset.category, title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM,
    powerW: Math.round(totalPowerW(asset) * 10) / 10,
    poses: asset.poses.filter((p: Json) => !p.key).map((p: Json) => p.id), clips: asset.clips.map((c: Json) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, asset.representations[k].status])),
    symbols: asset.symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction, completion: asset.completion,
  };
  await Bun.write(join(TICKET, "📥️manifest-rows", `${asset.id}.json`), JSON.stringify(row, null, 1) + "\n");
  return row;
}

/** ✂️ Removes parts by id from every scale: shapes of that id, body solids of that id and the thermal and structure solids that reference them or are named after them (`th-<id>...`). */
export function removeParts(asset: Json, ids: readonly string[]): void {
  const hit = (id: string) => ids.some((x) => id === x || id.startsWith(`th-${x}`));
  for (const rep of Object.values(asset.representations) as Json[]) if (rep.status === "authored") for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: Json) => !ids.includes(s.id));
  for (const vol of Object.values(asset.volumes) as Json[]) {
    if (vol.status !== "authored") continue;
    vol.body = vol.body.filter((s: Json) => !ids.includes(s.id));
    vol.thermal = vol.thermal.filter((s: Json) => !hit(s.id) && !(s.of && ids.includes(s.of)));
    vol.structure = vol.structure.filter((s: Json) => !(s.of && ids.includes(s.of)) && !ids.includes(s.id));
  }
}

/** 🌡️ Sets the `areaM2` of every region whose declared area misses the measured outer surface of the 1:50 or 1:100 thermal volume by more than
 * 80 % of its limit (30 % for body regions, 15 % otherwise) to the geometric mean of those two measurements (`volumes-thermal-coverage`). */
export function calibrateAreas(asset: Json): string[] {
  const outer: Record<string, Record<number, number>> = {};
  for (const m of measureVolumes(asset, { rules: ["volumes-thermal-coverage"] } as never) as Json[]) {
    const hit = /^info:outerAreaM2:(.+)$/.exec(m.name);
    if (hit) (outer[hit[1]!] ??= {})[m.scale] = m.value;
  }
  const changed: string[] = [];
  for (const g of asset.thermal.regions) {
    const a50 = outer[g.id]?.[50], a100 = outer[g.id]?.[100];
    if (a50 === undefined || a100 === undefined || g.areaBasis === "exchange") continue;
    const limit = g.model === "body" ? 0.3 : 0.15;
    if (Math.max(Math.abs(a50 / g.areaM2 - 1), Math.abs(a100 / g.areaM2 - 1)) <= 0.8 * limit) continue;
    const next = r(Math.sqrt(a50 * a100), 4);
    changed.push(`${g.id} ${g.areaM2} -> ${next} (1:50 ${a50.toFixed(4)}, 1:100 ${a100.toFixed(4)})`);
    g.areaM2 = next;
  }
  return changed;
}
