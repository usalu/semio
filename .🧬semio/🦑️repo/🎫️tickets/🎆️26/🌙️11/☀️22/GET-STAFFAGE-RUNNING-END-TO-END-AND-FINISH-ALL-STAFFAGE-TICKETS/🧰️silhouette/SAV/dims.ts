/** 📐️ The declared dimensions the bodies of every authored scale accept: per axis the body extents of the default pose, the interval of declared values within one smallest feature of every scale, and, where the declared value lies outside it, the extent of the finest body, moved into the middle three fifths of that interval where it lies nearer to an end (to the millimetre). `--write` puts one `dims` op at the head of 🗑️generated/silhouette/SAV/ops/<id>.json and the changed fields into `manifest-changes.json` of this folder; an empty interval is printed with the solids that reach furthest and nothing is written for that axis.
 *  bun dims.ts <id> [--file <asset.json>] [--write] */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { readOps, writeOps } from "./ops.ts";
import { catalogFile } from "./files.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { file: { type: "string" }, write: { type: "boolean" } } });
const id = positionals[0]!;
const asset = (await Bun.file(values.file ?? catalogFile(id)).json()).asset;
const AXES = ["widthM", "heightM", "depthM"] as const;
const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
const reach = new Map<number, { id: string; min: number[]; max: number[] }[]>();
for (const d of scales) {
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  reach.set(d, worldSolids(asset, state.bones as never, d).map((w) => { const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity]; for (let i = 0; i < w.positions.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, w.positions[i + k]!); max[k] = Math.max(max[k]!, w.positions[i + k]!); } return { id: w.id, min, max }; }));
}
const set: Record<string, number> = {};
AXES.forEach((axis, k) => {
  const declared = asset.dimensions[axis] as number;
  const ext = scales.map((d) => { const l = reach.get(d)!; const lo = Math.min(...l.map((s) => s.min[k]!)), hi = Math.max(...l.map((s) => s.max[k]!)); return { d, lo, hi, size: hi - lo }; });
  const low = Math.max(...ext.map((e) => e.size - featureOf(e.d))), high = Math.min(...ext.map((e) => e.size + featureOf(e.d)));
  const inside = declared >= low && declared <= high;
  let line = `${id} ${axis} declared ${declared} | bodies ${ext.map((e) => `1:${e.d} ${e.size.toFixed(4)} [${e.lo.toFixed(4)}, ${e.hi.toFixed(4)}]`).join(", ")} | accepted ${low.toFixed(4)}..${high.toFixed(4)}`;
  if (inside) line += " ok";
  if (!inside) {
    if (low > high) line += " EMPTY";
    for (const e of ext) { const l = reach.get(e.d)!; line += `\n    1:${e.d} lowest ${[...l].sort((a, b) => a.min[k]! - b.min[k]!).slice(0, 3).map((s) => `${s.id} ${s.min[k]!.toFixed(4)}`).join(", ")} | highest ${[...l].sort((a, b) => b.max[k]! - a.max[k]!).slice(0, 3).map((s) => `${s.id} ${s.max[k]!.toFixed(4)}`).join(", ")}`; }
  }
  if (!inside && low <= high) {
    const margin = 0.2 * (high - low);
    const next = Number(Math.min(Math.max(ext[0]!.size, low + margin), high - margin).toFixed(3));
    const fits = next >= low && next <= high ? next : Number(((low + high) / 2).toFixed(4));
    set[axis] = fits;
    line += ` -> ${fits}`;
  }
  console.log(line);
});
if (values.write && Object.keys(set).length > 0) {
  const opsFile = `${import.meta.dir}/../../🗑️generated/silhouette/SAV/ops/${id}.json`;
  const existing = (await readOps(opsFile)).length > 0 ? await readOps(opsFile) : await readOps(`${import.meta.dir}/repair/${id}.json`);
  await writeOps(opsFile, [{ op: "dims", set: { ...Object.assign({}, ...existing.filter((o) => o.op === "dims").map((o) => o.set)), ...set } }, ...existing.filter((o) => o.op !== "dims")]);
  const manifestFile = `${import.meta.dir}/manifest-changes.json`;
  const manifest = (await Bun.file(manifestFile).exists()) ? JSON.parse(await Bun.file(manifestFile).text()) : {};
  manifest[id] = { ...(manifest[id] ?? {}), ...set };
  await Bun.write(manifestFile, JSON.stringify(Object.fromEntries(Object.entries(manifest).sort(([a], [b]) => a.localeCompare(b))), null, 2) + "\n");
  console.log(`${id}: ${Object.keys(set).join(", ")} written`);
}
