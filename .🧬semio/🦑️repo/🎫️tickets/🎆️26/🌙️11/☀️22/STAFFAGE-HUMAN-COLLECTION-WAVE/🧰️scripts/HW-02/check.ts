/** 🧪️ Full gate for the HW-02 assets: `validateAsset`, `checkVolumes` (every geometric volume rule) and the catalog-silhouette gate (bbox within one feature, IoU ≥ 0.92, Hausdorff ≤ 2 features per view and scale).
 *  bun check.ts <slug|file.json> [...]   (env SKIP=volumes|silhouette, SCALES=50,100) */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { validateAsset } = await import(join(S, "🔨️modules/🗂️catalog/🟦️.ts"));
const { checkVolumes } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const skip = new Set((process.env.SKIP ?? "").split(","));
const scales = (process.env.SCALES ?? "50,100,200,500").split(",").map(Number);
let bad = 0;
for (const arg of process.argv.slice(2)) {
  const file = arg.endsWith(".json") ? arg : join(S, "🗂️catalog/👤️humans", arg, "🔣️.json");
  const asset = (await Bun.file(file).json()).asset;
  const t0 = performance.now();
  const out: string[] = [];
  for (const i of validateAsset(asset)) out.push(`validate ${i.code}: ${i.message.en}`);
  if (!skip.has("volumes")) for (const f of checkVolumes(asset)) out.push(`volumes ${f.rule} 1:${f.scale ?? ""} ${f.list ?? ""}: ${f.message.en}`);
  if (!skip.has("silhouette")) {
    const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
    for (const scale of scales) {
      const feature = featureOf(scale);
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      m.bbox.forEach((b: number, k: number) => { if (Math.abs(b - dims[k]!) > feature) out.push(`silhouette 1:${scale} bbox axis ${k}: ${b.toFixed(3)} vs ${dims[k]}`); });
      const row = VIEWPOINTS.map((v: string) => `${v} ${m.views[v].iou.toFixed(3)}/${(m.views[v].hausdorffM * 1000).toFixed(0)}mm`).join("  ");
      console.log(`  [DEBUG] 1:${scale} ${row}`);
      for (const v of VIEWPOINTS) {
        if (m.views[v].iou < 0.92) out.push(`silhouette 1:${scale} ${v} iou ${m.views[v].iou.toFixed(3)}`);
        if (m.views[v].hausdorffM > 2 * feature) out.push(`silhouette 1:${scale} ${v} hausdorff ${(m.views[v].hausdorffM * 1000).toFixed(1)} mm > ${(2000 * feature).toFixed(1)}`);
      }
    }
  }
  console.log(`${arg}\t${out.length} issues\t${((performance.now() - t0) / 1000).toFixed(1)} s`);
  for (const line of out.slice(0, Number(process.env.LIMIT ?? 60))) console.log(`   ${line}`.slice(0, 600));
  if (out.length) bad++;
}
process.exit(bad ? 1 : 0);
