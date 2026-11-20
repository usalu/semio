/** 🪟 Outer area of the cab glazing with one pane at a time, to see which pane carries which area: `bun r3-wheel-loader-glass.ts`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const base = JSON.parse(readFileSync("🗑️generated/r3-wheel-loader/🔣️.json", "utf8")).asset;
const ids = base.volumes["50"].thermal.filter((t: { region: string }) => t.region === "cab-glazing").map((t: { id: string }) => t.id);
for (const only of ids) {
  const asset = structuredClone(base);
  asset.volumes["50"].thermal = asset.volumes["50"].thermal.filter((t: { region: string; id: string }) => t.region !== "cab-glazing" || t.id === only);
  const m = measureVolumes(asset, { scale: 50 as never, rules: ["volumes-thermal-coverage" as never] });
  console.log(`[DEBUG] ${only} ` + m.filter((x) => x.name === "info:outerAreaM2:cab-glazing").map((x) => x.value.toFixed(3)).join(" "));
}
