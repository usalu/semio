/** 🧱️ PW-16 diagnostic: the silhouette Hausdorff of one view, whole raster against the banded tiles of the catalog guard, with the worst tile. `repo tools exec bun -- tiles.ts <asset.json> <scale> <view>` */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [file, scaleText, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const asset = (JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset;
const scale = Number(scaleText);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale as never } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const f = featureOf(scale);
const whole = measureView(asset, state, solids, view, scale, undefined, f, 1e9);
const banded = measureView(asset, state, solids, view, scale, undefined, f);
console.log(`[DEBUG] whole h ${(whole.hausdorffM * 1000).toFixed(0)} iou ${whole.iou.toFixed(3)} cell ${(whole.cell * 1000).toFixed(1)}mm; banded h ${(banded.hausdorffM * 1000).toFixed(0)} iou ${banded.iou.toFixed(3)}`);
