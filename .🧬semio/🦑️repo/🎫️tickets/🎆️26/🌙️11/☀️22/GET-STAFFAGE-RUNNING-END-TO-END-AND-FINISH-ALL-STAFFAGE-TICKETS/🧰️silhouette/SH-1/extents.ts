/** 📏️ The drawn extents of people per scale and view as `validateAsset` reads them (`viewExtent`: horizontal and vertical extent of the default pose), beside the declared dimensions.
 *  bun extents.ts <slug|file.json>… */
import { viewExtent } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./people.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  console.log(`${asset.id} declared ${JSON.stringify(asset.dimensions)}`);
  for (const d of [50, 100, 200, 500].filter((x) => asset.representations[String(x)]?.status === "authored")) {
    try {
    const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
    console.log(`  1:${d} ${(["elevation", "front", "plan"] as const).map((v) => { const e = viewExtent(state, v); return e ? `${v} ${e.horizontal.toFixed(3)} x ${e.vertical.toFixed(3)}` : `${v} none`; }).join("; ")}`);
    } catch (e) { console.log(`  1:${d} not evaluated: ${String(e).slice(0, 90)}`); }
  }
}
