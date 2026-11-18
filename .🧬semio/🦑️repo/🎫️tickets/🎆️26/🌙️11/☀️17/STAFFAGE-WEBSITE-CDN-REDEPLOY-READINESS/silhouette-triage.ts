/** 🧪 Silhouette triage: the checks of `catalog-silhouette` for chosen assets, from the working tree or from `git show HEAD`, as JSON lines of failures only.
 *  bun silhouette-triage.ts <current|head> <out.jsonl> [asset ...]   (no asset: all) */
import { execFileSync } from "node:child_process";
import { appendFileSync, writeFileSync } from "node:fs";
import { relative } from "node:path";
import { SCALE_DENOMINATORS, type Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { GATE, VIEWPOINTS, measureAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [mode, out, ...only] = process.argv.slice(2) as ["current" | "head", string, ...string[]];
const ROOT = "/Users/ueli/Documents/semio";
writeFileSync(out, "");
const entries = (await assetPathsFromFiles()).filter((e) => only.length === 0 || only.includes(e.folder));
for (const entry of entries) {
  const t0 = Date.now();
  let asset: Asset;
  try {
    if (mode === "head") {
      const text = execFileSync("git", ["-C", ROOT, "show", `HEAD:${relative(ROOT, entry.path)}`], { maxBuffer: 1 << 28 }).toString("utf8");
      asset = JSON.parse(text).asset;
    } else asset = ((await readStreamedAsset(entry)) as { asset: Asset }).asset;
  } catch (error) {
    appendFileSync(out, JSON.stringify({ asset: entry.folder, mode, error: String(error).slice(0, 200) }) + "\n");
    continue;
  }
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const scales = SCALE_DENOMINATORS.filter((d) => (asset.volumes as Record<string, { status: string }>)[String(d)]?.status === "authored");
  const fails: unknown[] = [];
  try {
    for (const scale of scales) {
      const feature = featureOf(scale);
      const m = measureAsset(asset, scale, undefined, undefined, feature);
      m.bbox.forEach((b, k) => {
        if (Math.abs(b - dims[k]!) > feature) fails.push({ scale, kind: "bbox", axis: k, got: +b.toFixed(4), want: dims[k], feature });
      });
      for (const view of VIEWPOINTS) {
        const v = m.views[view];
        if (v.iou < GATE.iou) fails.push({ scale, kind: "iou", view, section: m.sections[view] !== undefined, iou: +v.iou.toFixed(3) });
        if (v.hausdorffM > 2 * feature) fails.push({ scale, kind: "hausdorff", view, h: +v.hausdorffM.toFixed(4), limit: 2 * feature });
      }
    }
  } catch (error) {
    fails.push({ kind: "throw", error: String(error).slice(0, 300) });
  }
  appendFileSync(out, JSON.stringify({ asset: entry.folder, mode, scales, seconds: +((Date.now() - t0) / 1000).toFixed(1), fails }) + "\n");
}
