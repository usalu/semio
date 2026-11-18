/** 🧪 Trial of an in-memory patch against the checks of `catalog-silhouette`. bun humans-try.ts <asset> [patch.mjs] [scale ...] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { GATE, VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, patch, ...sc] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (patch && patch !== "-") {
  if (patch.endsWith(".json")) {
    const find = (list: any[], id: string) => list.find((x) => x.id === id);
    for (const o of JSON.parse(await Bun.file(patch.startsWith("/") ? patch : process.cwd() + "/" + patch).text())) {
      const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
      const solids = (): any[] => (asset as any).volumes[o.scale].body;
      const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
      if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
      else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
      else list[list.indexOf(find(list, o.id))] = o[key];
    }
  } else (await import(patch.startsWith("/") ? patch : process.cwd() + "/" + patch)).default(asset);
}
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
for (const scale of (sc.length ? sc.map(Number) : [500, 200, 100, 50])) {
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale as never, undefined, undefined, feature);
  const out: string[] = [];
  m.bbox.forEach((b, k) => { if (Math.abs(b - dims[k]!) > feature) out.push(`bbox${k} ${b.toFixed(4)} vs ${dims[k]}`); });
  for (const v of VIEWPOINTS) { const x = m.views[v]; out.push(`${v[0]}:${x.iou.toFixed(3)}/${(x.hausdorffM * 1000).toFixed(1)}mm${x.iou < GATE.iou || x.hausdorffM > 2 * feature ? "!" : ""}`); }
  console.log(scale, `limit ${(2 * feature * 1000).toFixed(0)}mm`, out.join("  "));
}
