/** 🧪️ Probe: the 1:50 silhouette agreement (IoU, p95, soft p95) of every named pose of an asset file, to choose a default pose. bun probe-poses.ts <file> [scale] */
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
const a = (await Bun.file(process.argv[2]!).json()).asset;
const sc = Number(process.argv[3] ?? 50);
for (const p of a.poses.filter((p: any) => !p.key)) {
  const m = measureAsset(a, sc, p.id, undefined, featureOf(sc));
  console.log(p.id.padEnd(26), (["elevation", "front", "plan"] as const).map((v) => { const x: any = m.views[v]; return `${v} ${x.iou.toFixed(3)} p95 ${(x.p95M * 1000).toFixed(1)} soft ${(x.p95SoftM * 1000).toFixed(1)}`; }).join(" | "));
}
