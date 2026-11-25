/** 🥢️ Which `lateralScale` members of an asset cost the silhouette guard its limit: `bun lateral-drop.ts <id|file.json> <scale> [--drop a,b [--out file]]` measures the guard at the scale with every member read, with the members of each single shape id removed in the three views, and (when no single id brings the scale below its limits) with each pair removed; `--drop` removes the named ids at the scale and writes the copy. Run it from `S/📦️packages/🟦️typescript`.
 * @see ./guard.ts */
import { parseArgs } from "node:util";
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { values, positionals } = parseArgs({ allowPositionals: true, options: { drop: { type: "string" }, out: { type: "string" } } });
const [arg, key] = positionals as [string, string];
const read = readDoc(arg);
const scale = Number(key);
const feature = featureOf(scale);
const without = (asset: any, ids: readonly string[]): any => ({ ...asset, representations: { ...asset.representations, [key]: { ...asset.representations[key], geometry: Object.fromEntries(Object.entries(asset.representations[key].geometry).map(([view, shapes]) => [view, (shapes as any[]).map((shape) => (shape.solid?.lateralScale !== undefined && ids.includes(shape.id) ? { ...shape, solid: Object.fromEntries(Object.entries(shape.solid).filter(([member]) => member !== "lateralScale")) } : shape))])) } } });
if (values.drop !== undefined) {
  read.doc.asset = without(read.doc.asset, values.drop.split(","));
  writeDoc(read, values.out);
  console.log(`${read.doc.asset.id} 1:${key}: lateralScale removed from ${values.drop}${values.out === undefined ? " in the catalog document" : ""}`);
} else {
  const asset = read.doc.asset;
  const ids = [...new Set((Object.values(asset.representations[key].geometry) as any[][]).flat().filter((shape) => (shape.solid?.lateralScale ?? 1) !== 1).map((shape) => shape.id as string))];
  const judge = (label: string, dropped: readonly string[]): boolean => {
    const m = measureAsset(without(asset, dropped), scale, undefined, undefined, feature);
    const ok = VIEWPOINTS.every((view: string) => m.views[view].iou >= 0.92 && m.views[view].hausdorffM <= 2 * feature);
    console.log(`${asset.id} 1:${key} ${label}: ${VIEWPOINTS.map((view: string) => `${view} ${m.views[view].iou.toFixed(4)}/${(m.views[view].hausdorffM * 1000).toFixed(1)}mm`).join("  ")}${ok ? "  OK" : ""}`);
    return ok;
  };
  judge("all read", []);
  const singles = ids.filter((id) => judge(`without ${id}`, [id]));
  if (singles.length === 0) for (let i = 0; i < ids.length; i++) for (let j = i + 1; j < ids.length; j++) judge(`without ${ids[i]},${ids[j]}`, [ids[i]!, ids[j]!]);
}
