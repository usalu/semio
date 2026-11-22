/** 🔬️ PW-12 gate diagnosis: the elevation Hausdorff of one scale of an asset with groups of shapes and their solids removed (by id prefix), to find which part disagrees with its volume.
 *
 * `repo tools exec bun -- <this file> <asset id> <scale> <prefix,prefix|none>...` */
import { loadCatalogFromFiles } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const [id, scaleArg, ...variants] = process.argv.slice(2);
const scale = Number(scaleArg) as 50;
const asset = (await loadCatalogFromFiles(undefined, undefined, new Set([id!]))).assets.find((a) => a.id === id)!;
for (const variant of variants) {
  const drop = variant === "none" ? [] : variant.split(",");
  const keep = (x: { id: string }): boolean => !drop.some((p) => x.id.startsWith(p) || x.id.startsWith(`th-${p}`));
  const copy = structuredClone(asset) as typeof asset;
  const rep = (copy.representations as Record<string, { geometry: Record<string, { id: string }[]> }>)[String(scale)]!;
  for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v]!.filter(keep);
  const vol = (copy.volumes as Record<string, { body: { id: string }[]; thermal: { id: string }[] }>)[String(scale)]!;
  vol.body = vol.body.filter(keep);
  const m = measureAsset(copy, scale, undefined, undefined, featureOf(scale));
  console.log(`[DEBUG] without ${variant}: ${["elevation", "front", "plan"].map((v) => `${v} h ${m.views[v as "plan"].hausdorffM.toFixed(3)}`).join(" ")}`);
}
