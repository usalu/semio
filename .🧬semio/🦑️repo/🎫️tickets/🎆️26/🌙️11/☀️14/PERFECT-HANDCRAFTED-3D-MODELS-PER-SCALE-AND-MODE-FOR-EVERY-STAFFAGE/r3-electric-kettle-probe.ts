/** 🧪 Probes the thermal behaviour that the catalog tests pin on the electric kettle (`🫕️phase-change`, `🚰️hydronic-coupling`) for an asset document: water temperature over the boil clip, evaporation before and at the boil and the wall gap at the plateau, so that an `areaM2` correction can be judged before it is written.
 *
 * Usage: `bun r3-electric-kettle-probe.ts <asset document json>`
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🫕️phase-change/🟦️.ts */
import { readFileSync } from "node:fs";
import environments from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import { solveThermalAt } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const room = environments.presets.find((p) => p.id === "office-neutral")!.environment;
const at = (t: number) => solveThermalAt(asset as never, { kind: "clip", clip: "boil", timeS: t } as never, room as never).regions;
for (const t of [100, 120, 170, 184, 185, 195]) {
  const r = at(t);
  console.log(`t=${t} water ${r["water"]!.surfaceTempC.toFixed(2)} C evap ${(r["water"]!.evaporationKgS * 1000).toFixed(4)} g/s phaseW ${r["water"]!.phaseChangeW.toFixed(0)} shell steady gap ${(100 - r["shell"]!.steadyTempC).toFixed(2)} K lid ${r["lid"]!.surfaceTempC.toFixed(1)} C`);
}
console.log("tests: water(120)<80, water(170)<97, 185..200 = 100, phaseW(184)>1500, evap(100)<0.05, 0.75<evap(195)<0.85, 0<gap<6");
