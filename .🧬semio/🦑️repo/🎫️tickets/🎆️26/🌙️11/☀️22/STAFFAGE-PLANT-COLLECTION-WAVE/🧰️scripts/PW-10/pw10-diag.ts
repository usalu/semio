/** 🔎️ Silhouette diagnosis of a candidate document: per view and scale the IoU of the default-pose body against the drawing and the solids and shapes that disagree most (helpers of round 3, not the product rule).
 * Usage: `repo tools exec bun -- pw10-diag.ts <asset document json> [scale]` */
import { readFileSync } from "node:fs";
import { compareView, meshesAt, poseState } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const scales = process.argv[3] ? [Number(process.argv[3])] : [50, 100, 200, 500];
for (const s of scales) {
  const st = poseState(asset, s);
  const body = asset.volumes[String(s)].body.filter((x: { texture?: boolean }) => !x.texture);
  const meshes = meshesAt(asset, s, body);
  for (const view of ["elevation", "front", "plan"] as const) {
    const m = compareView(view, st[view], meshes);
    console.log(`[DEBUG] 1:${s} ${view} iou ${m.iou.toFixed(3)} recall ${m.recall.toFixed(3)} precision ${m.precision.toFixed(3)}`);
    console.log(`[DEBUG]   solids out: ${m.worstSolids.slice(0, 6).map((x) => `${x.id} ${x.outsideM2.toFixed(3)}`).join(", ")}`);
    console.log(`[DEBUG]   shapes missing: ${m.worstShapes.slice(0, 6).map((x) => `${x.id} ${x.missingM2.toFixed(3)}`).join(", ")}`);
  }
}
