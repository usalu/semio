/** 🌓️ Measures one asset as the catalog case of the `🧊️catalog-silhouette` guard does (the same `measureAsset`, the same gate; the sharp oracle is left to the suite): `bun tv1-silhouette.ts <id>|--file <json>` prints, per authored scale, the box against the dimensions and IoU and Hausdorff distance of each view with `ok` or `RED`, then one line `silhouette <id>: N red`. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { GATE, VIEWPOINTS, measureAsset } = await import(join(staffage, "🧪️tests/🧰️support/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(staffage, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const args = process.argv.slice(2);
const asset = args[0] === "--file" ? JSON.parse(readFileSync(resolve(args[1]!), "utf8")).asset : catalog().find((candidate: { id: string }) => candidate.id === args[0]);
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let red = 0;
for (const scale of [50, 100, 200, 500]) {
  if (asset.volumes?.[String(scale)]?.status !== "authored" || asset.representations[String(scale)]?.status !== "authored") continue;
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale, undefined, undefined, feature);
  const box = m.bbox.map((b: number, k: number) => {
    const bad = Math.abs(b - dims[k]!) > feature;
    red += bad ? 1 : 0;
    return `${((b - dims[k]!) * 1000).toFixed(1)}${bad ? " RED" : ""}`;
  });
  const views = VIEWPOINTS.map((view: string) => {
    const v = m.views[view];
    const bad = v.iou < GATE.iou || v.hausdorffM > 2 * feature;
    red += bad ? 1 : 0;
    return `${view} IoU ${v.iou.toFixed(4)} H ${(v.hausdorffM * 1000).toFixed(1)} mm ${bad ? "RED" : "ok"}`;
  });
  console.log(`1:${scale} (feature ${(feature * 1000).toFixed(1)} mm) box mm ${box.join(" / ")} | ${views.join(" | ")}`);
}
console.log(`silhouette ${asset.id}: ${red} red`);
