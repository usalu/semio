/** 🌡️ The body check of `🧪️tests/🌡️catalog-physics` for people or asset files: core temperature, mean skin temperature and the warmest skin region of the default pose.
 *  bun physics.ts <slug|file.json>… */
import { evaluate, solveThermal } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { fileOf } from "./people.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const field: any = solveThermal(evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }) as never);
  const skins = Object.entries(field.regions).filter(([, r]: any) => r.skinTempC !== null).map(([id, r]: any) => [id, r.skinTempC] as const).sort((a, b) => b[1] - a[1]);
  console.log(`${arg.split("/").slice(-1)[0]}: core ${field.body?.coreTempC.toFixed(3)} °C, mean skin ${field.body?.meanSkinTempC.toFixed(3)} °C, warmest ${skins.slice(0, 3).map(([id, t]) => `${id} ${t.toFixed(3)}`).join(", ")}`);
}
