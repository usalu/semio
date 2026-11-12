/** 🔬️ Silhouette agreement of every pose of the given assets at 1:50: `bun silhouette-poses.ts slug,slug`. */
import { measureAsset, VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
for (const e of catalogEntries()) {
  if (!process.argv[2]!.split(",").includes(e.slug)) continue;
  const asset = loadAsset(e.path);
  for (const p of asset.poses) {
    try {
      const m = measureAsset(asset, 50, p.id);
      console.log(asset.id.padEnd(20), p.id.padEnd(22), "bbox", m.bboxError.map((x) => (x * 100).toFixed(1)).join("/"), "iou", VIEWPOINTS.map((v) => m.views[v].iou.toFixed(2)).join("/"));
    } catch (error) { console.log(asset.id, p.id, "ERR", (error as Error).message.slice(0, 80)); }
  }
}
