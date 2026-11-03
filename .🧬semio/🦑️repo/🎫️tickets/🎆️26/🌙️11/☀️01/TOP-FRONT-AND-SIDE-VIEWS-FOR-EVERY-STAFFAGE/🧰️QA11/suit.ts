import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, patchJson] = process.argv.slice(2) as [string, string, string | undefined];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const rep = asset.representations[scale];
for (const p of JSON.parse(patchJson ?? "[]") as { id: string; view: string; set: Record<string, unknown> }[]) {
  for (const s of rep.geometry[p.view]) if (s.id === p.id) Object.assign(s, p.set);
}
const result = measureSuitability(asset, rep, Number(scale), DEFAULT_PRINT_PROFILE);
console.log("[DEBUG] suitable", result.suitable, "minFeature", result.minFeatureMm.toFixed(3), JSON.stringify(result.violations.map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`)));
