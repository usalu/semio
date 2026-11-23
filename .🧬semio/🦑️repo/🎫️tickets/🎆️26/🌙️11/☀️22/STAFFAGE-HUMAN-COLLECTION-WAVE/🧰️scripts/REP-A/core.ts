/** 🌡️ The core and mean skin temperature of asset files in their default pose, as 🧪️tests/🌡️catalog-physics reads them.  bun core.ts <asset.json>… */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
for (const path of process.argv.slice(2)) {
  const asset = (await Bun.file(path).json()).asset;
  const field: any = solveThermal(evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }));
  console.log(`${asset.id} (${path.split("/").slice(-2).join("/")}): core ${field.body.coreTempC.toFixed(3)} °C, mean skin ${field.body.meanSkinTempC.toFixed(3)} °C, met ${asset.poses.find((p: any) => p.id === asset.presentation.defaultPose)?.thermal?.met ?? "?"}`);
}
