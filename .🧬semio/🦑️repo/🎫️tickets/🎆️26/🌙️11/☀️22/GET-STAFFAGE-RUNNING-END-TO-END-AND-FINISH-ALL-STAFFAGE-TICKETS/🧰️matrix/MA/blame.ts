/** 🕵️ Which refitted shape moves the silhouette guard of a view: `bun blame.ts <before.json> <after.json> <scale> <view>` measures the view of the after document against the volume (IoU and largest boundary distance, the numbers of `🧊️catalog-silhouette`), then once more with each changed shape of that view (near and far twins together) put back to its state before, and prints the shapes whose return improves the distance, best first. Run it from `S/📦️packages/🟦️typescript`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureAsset } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const [beforeArg, afterArg, scale, view] = process.argv.slice(2) as [string, string, string, string];
const before = readDoc(beforeArg).doc.asset;
const after = readDoc(afterArg).doc.asset;
const feature = featureOf(Number(scale));
const measure = (): { iou: number; mm: number } => {
  const m = measureAsset(after, Number(scale), undefined, undefined, feature).views[view];
  return { iou: m.iou, mm: m.hausdorffM * 1000 };
};
const base = measure();
console.log(`${after.id} 1:${scale} ${view}: IoU ${base.iou.toFixed(4)}, ${base.mm.toFixed(1)} mm (gate ${(2 * feature * 1000).toFixed(0)} mm)`);
const list = after.representations[scale].geometry[view] as any[];
const old = before.representations[scale].geometry[view] as any[];
const rows: { id: string; iou: number; mm: number }[] = [];
const family = (id: string): string => id.split("-").filter((token) => !["n", "f", "near", "far", "l", "r"].includes(token)).map((token) => token.replace(/^(h|f)(n|f)$/u, "$1")).join("-");
const changed = list.map((shape, i) => ({ i, was: old.find((other) => other.id === shape.id && other.bone === shape.bone) })).filter((entry) => entry.was !== undefined && JSON.stringify(entry.was) !== JSON.stringify(list[entry.i]));
for (const name of new Set(changed.map((entry) => family(list[entry.i].id)))) {
  const group = changed.filter((entry) => family(list[entry.i].id) === name);
  const kept = group.map((entry) => list[entry.i]);
  for (const entry of group) list[entry.i] = entry.was;
  rows.push({ id: `${name} ×${group.length} (${kept.map((shape) => `${shape.solid?.kind}${shape.solid?.sweep ?? shape.solid?.axis ?? ""}`).join(",")})`, ...measure() });
  group.forEach((entry, k) => (list[entry.i] = kept[k]));
}
for (const row of rows.sort((x, y) => x.mm - y.mm || y.iou - x.iou).slice(0, Number(process.argv[6] ?? 8))) console.log(`  without ${row.id}: IoU ${row.iou.toFixed(4)}, ${row.mm.toFixed(1)} mm`);
