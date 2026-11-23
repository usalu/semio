/** 🌓️ Measures the named assets as the catalog-silhouette guard does (bounding box against `dimensions` within one feature, IoU and Hausdorff distance of every view at every authored scale, default pose), without the sharp oracle of the test, so that an asset can be compared before and after an edit.
 *
 * Usage: `bun silhouette-2.ts <id> [<id> …] [--file <asset document json>]`
 * Prints one line per scale and view `<id> 1:<scale> <view> iou <n> hausdorff <m> ok|FAIL`, one line per scale for the box, then `<id>: N failing`.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { SCALE_DENOMINATORS, type Asset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, bodyDimensions, measureAsset, worldSolids } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { fixture } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧰️.ts";
import { catalog } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

if (import.meta.main) {
  const gate = fixture<{ gate: { iou: number } }>("🧊️catalog-silhouette").gate;
  const args = process.argv.slice(2);
  const fileAt = args.indexOf("--file");
  const ids = args.filter((arg, index) => !arg.startsWith("--") && (fileAt < 0 || index !== fileAt + 1));
  const all = fileAt < 0 ? catalog() : [(JSON.parse(readFileSync(args[fileAt + 1]!, "utf8")) as { asset: Asset }).asset];
  for (const id of ids) {
    const asset = all.find((candidate) => candidate.id === id)!;
    const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
    let failing = 0;
    for (const scale of SCALE_DENOMINATORS.filter((d) => (asset.volumes as Record<string, { status: string }>)[String(d)]?.status === "authored")) {
      const feature = featureOf(scale);
      const m = asset.representations[`${scale}`].status === "authored" ? measureAsset(asset, scale, undefined, undefined, feature) : undefined;
      const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones, scale));
      const boxOk = bbox.every((b, k) => Math.abs(b - dims[k]!) <= feature);
      if (!boxOk) failing += 1;
      console.log(`${id} 1:${scale} box ${bbox.map((b) => b.toFixed(3)).join(" x ")} against ${dims.join(" x ")} (feature ${feature}) ${boxOk ? "ok" : "FAIL"}`);
      if (m === undefined) continue;
      for (const view of VIEWPOINTS) {
        const measured = m.views[view];
        const ok = measured.iou >= gate.iou && measured.hausdorffM <= 2 * feature;
        if (!ok) failing += 1;
        console.log(`${id} 1:${scale} ${view} iou ${measured.iou.toFixed(4)} hausdorff ${measured.hausdorffM.toFixed(4)} (gate ${gate.iou}, ${(2 * feature).toFixed(4)}) ${ok ? "ok" : "FAIL"}`);
      }
    }
    console.log(`${id}: ${failing} failing`);
  }
}
