/** 🔬️ Probe (author 42): product p95 of one view at several raster sizes: bun author-42-probe4.ts <asset> <scale> <view> */
import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const sil: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const ev: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts");
const [spec, sc, view] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const state = ev.evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const solids = sil.worldSolids(m, state.bones, Number(sc)).filter((w: any) => w.texture !== true);
console.log("TILE_CELLS", sil.TILE_CELLS);
for (const cells of [300, 420, 600, 800, 1000, 1400]) {
  const r = sil.measureView(m, state, solids, view, Number(sc), cells, undefined);
  console.log(spec, sc, view, "cells", cells, "iou", r.iou.toFixed(4), "p95M", (r.p95M * 1000).toFixed(1), "mm hausdorff", (r.hausdorffM * 1000).toFixed(1), "mm");
}
