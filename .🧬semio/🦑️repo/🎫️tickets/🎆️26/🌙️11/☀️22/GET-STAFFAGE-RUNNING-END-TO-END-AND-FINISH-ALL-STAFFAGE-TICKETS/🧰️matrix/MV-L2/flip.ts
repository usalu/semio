/** 🕵️ Which strokes the silhouette guard objects to as lofts: `bun flip.ts <before.json> <after.json> <scale> <view>` measures the view in the default pose as `🧊️catalog-silhouette` does (`measureView`, largest boundary distance against twice the smallest feature) with every loft of the second document that stands where the first has a sheet `<id>-mvs`, with none of them (the sheets instead), then takes lofts back to sheets one failing half at a time until the view passes, and prints the lofts that had to stay sheets with the distance of every step. Run it from `S/📦️packages/🟦️typescript`.
 * @see ../MV-3/culprit.ts — the same search over depth plates
 * @see ./tube.ts — `--flat id,id` keeps the strokes this search names as sheets */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureView, worldSolids } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [first, second, key, view] = process.argv.slice(2);
const scale = Number(key);
const [before, after] = [readDoc(first!).doc.asset, readDoc(second!).doc.asset];
const feature: number = featureOf(scale);
const limit = 2 * feature;
const sheets = new Map<string, any>(before.representations[key!].geometry[view!].filter((shape: any) => String(shape.id).endsWith("-mvs")).map((shape: any) => [`${shape.bone}|${shape.id.slice(0, -4)}`, shape]));
const list: any[] = after.representations[key!].geometry[view!];
const lofts: string[] = list.filter((shape) => shape.kind === "loft" && sheets.has(`${shape.bone}|${shape.id}`)).map((shape) => `${shape.bone}|${shape.id}`);
const measure = (flat: ReadonlySet<string>): number => {
  const copy = structuredClone(after);
  copy.representations[key!].geometry[view!] = list.map((shape) => (flat.has(`${shape.bone}|${shape.id}`) ? sheets.get(`${shape.bone}|${shape.id}`) : shape));
  const state: any = evaluate(copy, { kind: "pose", pose: copy.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const solids = (worldSolids(copy, state.bones, scale) as any[]).filter((w) => w.texture !== true);
  return measureView(copy, state, solids, view, scale, undefined, feature).hausdorffM as number;
};
const mm = (m: number): string => (m * 1000).toFixed(1);
const [all, none] = [measure(new Set()), measure(new Set(lofts))];
console.log(`${after.id} 1:${key} ${view}: limit ${mm(limit)} mm; ${lofts.length} lofts ${mm(all)} mm, all of them as sheets ${mm(none)} mm`);
const flat = new Set<string>();
for (let round = 0; round < 12 && measure(flat) > limit; round++) {
  let pool = lofts.filter((id) => !flat.has(id));
  while (pool.length > 1) {
    const half = pool.slice(0, Math.ceil(pool.length / 2));
    const rest = pool.slice(half.length);
    const [a, b] = [measure(new Set([...flat, ...half])), measure(new Set([...flat, ...rest]))];
    pool = a <= b ? half : rest;
    console.log(`[DEBUG] ${pool.length} left (${mm(Math.min(a, b))} mm with them as sheets)`);
  }
  flat.add(pool[0]!);
  console.log(`as a sheet: ${pool[0]} -> ${mm(measure(flat))} mm`);
}
console.log(`${after.id} 1:${key} ${view}: ${flat.size} strokes stay sheets: ${[...flat].map((id) => id.split("|")[1]).join(",")}; ${mm(measure(flat))} mm`);
