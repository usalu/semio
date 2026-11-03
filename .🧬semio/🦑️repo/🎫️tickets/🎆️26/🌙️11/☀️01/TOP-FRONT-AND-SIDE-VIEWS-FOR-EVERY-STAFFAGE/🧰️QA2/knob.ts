import { writeFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const asset = catalog.assets.find((a) => a.id === "gas-boiler")!;
const out: string[] = [];
const variants: [string, (g: any) => void][] = [
  ["noall", (g) => { for (const v of ["elevation", "front", "plan"]) g[v] = g[v].filter((s: any) => s.id !== "knob"); }],
  ["onlyE", (g) => { g.front = g.front.filter((s: any) => s.id !== "knob"); g.plan = g.plan.filter((s: any) => s.id !== "knob"); }],
  ["onlyF", (g) => { g.elevation = g.elevation.filter((s: any) => s.id !== "knob"); g.plan = g.plan.filter((s: any) => s.id !== "knob"); }],
  ["onlyP", (g) => { g.elevation = g.elevation.filter((s: any) => s.id !== "knob"); g.front = g.front.filter((s: any) => s.id !== "knob"); }],
];
for (const [label, mut] of variants) {
  for (const d of [50, 100]) {
    const rep: any = structuredClone((asset as any).representations[String(d)]);
    mut(rep.geometry);
    const r = measureSuitability(asset, rep, d, DEFAULT_PRINT_PROFILE);
    out.push(`${label} 1:${d} ${r.violations.map((v: any) => v.shapes.join("+") + " " + v.check + " " + v.measuredMm.toFixed(3)).join("; ") || "ok"}`);
  }
}
writeFileSync(process.env.QA2_LOG!, out.join("\n") + "\n");
