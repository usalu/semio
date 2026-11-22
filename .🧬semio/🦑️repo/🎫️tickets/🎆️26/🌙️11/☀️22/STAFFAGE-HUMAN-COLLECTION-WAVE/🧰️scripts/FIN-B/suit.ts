/** 🖨️ Print suitability per scale and per named pose with the violating shapes (which pose makes a shape too thin): bun suit.ts <slug|file.json> [scales=500,200,100,50]   (VIEWS=1 measures every view on its own) */
import { join, resolve } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [arg, scales] = process.argv.slice(2);
const asset = (await Bun.file(arg!.endsWith(".json") ? arg! : join(catalog, arg!, "🔣️.json")).json()).asset;
for (const d of (scales ?? "500,200,100,50").split(",").map(Number)) {
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    for (const view of process.env.VIEWS ? ["elevation", "front", "plan"] : ["all"]) {
    const rep = asset.representations[String(d)];
    const one = { ...asset, poses: [pose] };
    const s = measureSuitability(one, view === "all" ? rep : { ...rep, geometry: { elevation: [], front: [], plan: [], [view]: rep.geometry[view] } }, d as never, DEFAULT_PRINT_PROFILE);
    console.log(`1:${d} ${pose.id} ${view} suitable=${s.suitable} feature=${s.minFeatureMm.toFixed(3)} gap=${Number.isFinite(s.minGapMm) ? s.minGapMm.toFixed(3) : "–"} ${s.violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}${v.at ? `@${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`).join("; ")}`);
    }
  }
}
