/** 🔬️ Runs the library silhouette measure over assets: `bun silhouette-run.ts <out.jsonl> [slug,slug] [--pose id]`. */
import { appendFileSync, writeFileSync } from "node:fs";
import { measureAsset, VIEWPOINTS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const [log, only, scaleArg] = process.argv.slice(2);
const scale = Number(scaleArg ?? 50);
writeFileSync(log!, "");
for (const e of catalogEntries()) {
  if (only && only !== "all" && !only.split(",").includes(e.slug)) continue;
  const asset = loadAsset(e.path);
  const t0 = Date.now();
  try {
    if (asset.representations[String(scale)]?.status !== "authored") continue;
    const m = measureAsset(asset, scale);
    const row = { asset: asset.id, pose: m.pose, bboxError: m.bboxError.map((x) => Number(x.toFixed(4))), views: Object.fromEntries(VIEWPOINTS.map((v) => [v, { iou: Number(m.views[v].iou.toFixed(3)), recall: Number(m.views[v].recall.toFixed(3)), precision: Number(m.views[v].precision.toFixed(3)), strokes: m.views[v].openStrokes }])), sections: m.sections, ms: Date.now() - t0 };
    appendFileSync(log!, JSON.stringify(row) + "\n");
    const v = row.views as any;
    console.error(`${asset.id} bbox ${row.bboxError.join("/")} iou ${v.elevation.iou}/${v.front.iou}/${v.plan.iou} ${row.ms}ms`);
  } catch (error) {
    appendFileSync(log!, JSON.stringify({ asset: asset.id, error: (error as Error).message }) + "\n");
    console.error(asset.id, "ERROR", (error as Error).message);
  }
}
