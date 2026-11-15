import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const dir = process.env.VALIDATOR_MODULE ?? "🧊️volumes";
const { measureAsset } = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/${dir}/🌓️silhouette/🟦️.ts`);
const [slug, scale, view] = [process.argv[2]!, Number(process.argv[3]), process.argv[4] ?? "elevation"];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === slug)!;
const { asset } = await readStreamedAsset(entry);
const feature = (0.15 * scale) / 1000;
for (const cells of (process.env.CELLS ?? "1000").split(",").map((x) => (x === "rule" ? undefined : Number(x)))) {
  const m = measureAsset(asset as never, scale as never, undefined, cells, cells === undefined ? feature : undefined).views[view as "elevation"];
  console.log(cells ?? "rule", "p95", +(m.p95M * 1000).toFixed(1), "hd", +(m.hausdorffM * 1000).toFixed(1), "iou", +m.iou.toFixed(4));
}
