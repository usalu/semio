/** 🔁️ Reproducibility check of the repair ops of a person: applies them in memory to the backup taken before the repair, compares the result with the catalog file (sections that differ) and measures the guard of the given scale on both.
 *  bun opsmeasure.ts <slug> <scale> */
import { isDeepStrictEqual } from "node:util";
import { measureAsset, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { applyOps } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

const [slug, sc] = process.argv.slice(2) as [string, string];
const backup = (await Bun.file(`${import.meta.dir}/../../🗑️generated/silhouette/SH-2/backup/${slug}.json`).json()).asset;
const now = (await Bun.file(catalogFile(slug)).json()).asset;
applyOps(backup, JSON.parse(await Bun.file(opsFile(slug)).text()));
for (const scale of ["50", "100", "200", "500"]) for (const view of VIEWPOINTS) {
  const [a, b] = [backup.representations[scale]?.geometry?.[view] ?? [], now.representations[scale]?.geometry?.[view] ?? []];
  if (!isDeepStrictEqual(a, b)) console.log(`1:${scale} ${view}: ops give ${a.length} shapes [${a.map((s: any) => s.id).join(" ")}], file has ${b.length} [${b.map((s: any) => s.id).join(" ")}]; first difference at ${a.findIndex((s: any, i: number) => !isDeepStrictEqual(s, b[i]))}`.slice(0, 1500));
}
console.log(`whole asset equal: ${isDeepStrictEqual(backup, now)}`);
for (const [name, asset] of [["backup+ops", backup], ["file", now]] as const) {
  const m = measureAsset(asset, Number(sc), undefined, undefined, featureOf(Number(sc)));
  console.log(`${name} 1:${sc} ${VIEWPOINTS.map((v) => `${v} ${m.views[v].iou.toFixed(4)}/${(m.views[v].hausdorffM * 1000).toFixed(1)}mm`).join("  ")}`);
}
