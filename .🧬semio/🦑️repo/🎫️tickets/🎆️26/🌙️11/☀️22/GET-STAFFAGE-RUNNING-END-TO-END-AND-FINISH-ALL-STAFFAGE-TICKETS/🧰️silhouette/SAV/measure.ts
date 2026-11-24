/** 🌓️ Every check of the catalog silhouette guard for assets of batch SAV: the bounding box per axis and the IoU and largest boundary distance per authored scale and view, failing ones marked, plus the `validateAsset` issue count.
 *  bun measure.ts <id|file.json>… */
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, bodyDimensions, measureAsset, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const fails: string[] = [];
  const lines: string[] = [];
  for (const scale of [50, 100, 200, 500]) {
    if (asset.volumes[String(scale)]?.status !== "authored") continue;
    const feature = featureOf(scale);
    const m = asset.representations[String(scale)].status === "authored" ? measureAsset(asset, scale, undefined, undefined, feature) : undefined;
    const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones as never, scale));
    bbox.forEach((b: number, k: number) => { if (Math.abs(b - dims[k]) > feature) fails.push(`1:${scale} axis ${k} ${b.toFixed(4)}/${dims[k]} (${((b - dims[k]) * 1000).toFixed(1)} mm, limit ${feature * 1000})`); });
    let line = `  1:${scale} box ${bbox.map((b: number) => b.toFixed(3)).join(" ")}`;
    if (m) for (const view of VIEWPOINTS) {
      const v = m.views[view];
      line += ` | ${view} ${v.iou.toFixed(3)} ${(v.hausdorffM * 1000).toFixed(1)}`;
      if (v.iou < 0.92) fails.push(`1:${scale} ${view} IoU ${v.iou.toFixed(4)}`);
      if (v.hausdorffM > 2 * feature) fails.push(`1:${scale} ${view} ${(v.hausdorffM * 1000).toFixed(1)} mm (limit ${2 * feature * 1000})`);
    }
    lines.push(line);
  }
  const issues = validateAsset(asset) as { code: string }[];
  console.log(`${asset.id} dims ${dims.join(" ")} | validateAsset ${issues.length} [${[...new Set(issues.map((i) => i.code))].join(",")}] | ${fails.length} failing`);
  for (const l of lines) console.log(l);
  for (const f of fails) console.log(`  FAIL ${f}`);
}
