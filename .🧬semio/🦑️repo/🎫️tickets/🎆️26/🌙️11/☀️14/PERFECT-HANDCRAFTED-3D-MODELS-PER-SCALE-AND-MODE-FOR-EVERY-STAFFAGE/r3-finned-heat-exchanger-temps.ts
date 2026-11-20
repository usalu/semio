/** 🌡️ Round 3 finned heat exchanger: prints the solved surface temperature of every region in every pose of an asset file under the asset's reference environment, so pose texts can state what the solver gives.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts
 */
import { readFileSync } from "node:fs";

import { solveThermalAt } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
for (const pose of asset.poses) {
  const field: any = (solveThermalAt as any)(asset, { kind: "pose", pose: pose.id }, undefined);
  console.log(pose.id, Object.values(field.regions).map((r: any) => `${r.region} ${r.surfaceTempC.toFixed(1)} C`).join(", "));
}
