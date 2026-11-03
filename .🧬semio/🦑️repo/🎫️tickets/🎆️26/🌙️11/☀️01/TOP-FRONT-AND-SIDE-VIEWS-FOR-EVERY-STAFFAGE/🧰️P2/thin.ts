import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [id, scale] = process.argv.slice(process.argv.findIndex((x) => x.endsWith("thin.ts")) + 1);
const a = JSON.parse(readFileSync(`C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/${id}/🔣️.json`, "utf8")).asset;
const rep = a.representations[scale];
const run = (g: any) => { const r = { ...rep, geometry: g }; const x: any = measureSuitability({ ...a, representations: { ...a.representations, [scale]: r } }, r, Number(scale), DEFAULT_PRINT_PROFILE); return x.violations.map((v: any) => `${v.check}:${v.shapes.join("+")}:${v.measuredMm}`).join(" | ") || "none"; };
console.log("all", run(rep.geometry));
for (const vp of ["elevation", "front", "plan"]) {
  const g: any = { elevation: [], front: [], plan: [] }; g[vp] = rep.geometry[vp];
  console.log(vp, run(g));
}
