import { appendFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { checkAssetDetailed } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { loadCatalogFromFiles } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";

const { values } = parseArgs({ options: { out: { type: "string" }, json: { type: "string" }, samples: { type: "string", default: "5" }, "no-physics": { type: "boolean", default: false }, only: { type: "string" } } });
const catalog = await loadCatalogFromFiles();
const known = new Set(catalog.assets.map((a) => a.id));
const only = values.only ? new Set(values.only.split(",")) : null;
const mm = (v: number | undefined): string => (v === undefined || !Number.isFinite(v) ? "–" : v.toFixed(2));
const header = `| asset | ✓/✗ | issue codes (count) | 1:500 | 1:200 | 1:100 | 1:50 |\n|---|---|---|---|---|---|---|\n`;
writeFileSync(values.out!, `# Check v2 results\n\nprofile office-laser (min stroke 0.10, gap 0.15, feature 0.20, hatch spacing 0.40 mm), samples ${values.samples}, physics ${!values["no-physics"]}. Cells: status, min stroke / min gap / min feature in paper mm (– = nothing to measure); ✗ marks violations (g = gap, f = feature, s = stroke, h = hatch spacing counts, worst case per shape pair over all named poses).\n\n${header}`);
const rows: unknown[] = [];
for (const asset of catalog.assets) {
  if (only && !only.has(asset.id)) continue;
  const started = Date.now();
  let row: string;
  try {
    const report = checkAssetDetailed(asset, { samples: Number(values.samples), assets: known, physics: !values["no-physics"] });
    const counts = new Map<string, number>();
    for (const p of report.problems) {
      const first = p.split(" ")[0]!;
      const code = /^[a-z][a-z-]+$/.test(first) ? first : "runtime";
      counts.set(code, (counts.get(code) ?? 0) + 1);
    }
    const cell = (d: number): string => {
      const s = report.scales.find((x) => x.denominator === d);
      if (!s) return "?";
      if (s.status === "unavailable") return "unavail.";
      if (!s.suitability) return `error`;
      const by = (c: string): number => s.suitability!.violations.filter((v) => v.check === c).length;
      return `${s.suitability.suitable ? "ok" : `✗ (g${by("gap")} f${by("feature")} s${by("stroke")} h${by("hatch")})`} ${mm(s.suitability.minStrokeMm)}/${mm(s.suitability.minGapMm)}/${mm(s.suitability.minFeatureMm)}`;
    };
    row = `| ${asset.id} | ${report.problems.length ? "✗" : "✓"} | ${[...counts].map(([c, n]) => `${c} (${n})`).join(", ") || "–"} | ${cell(500)} | ${cell(200)} | ${cell(100)} | ${cell(50)} |`;
    rows.push({ asset: asset.id, ok: report.problems.length === 0, counts: Object.fromEntries(counts), problems: report.problems.slice(0, 60), scales: report.scales.map((s) => ({ d: s.denominator, status: s.status, suitable: s.suitability?.suitable, stroke: s.suitability?.minStrokeMm, gap: s.suitability?.minGapMm, feature: s.suitability?.minFeatureMm, violations: s.suitability?.violations.map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}${v.at ? ` @${v.at.xMm.toFixed(1)},${v.at.yMm.toFixed(1)}` : ""}`) })), seconds: (Date.now() - started) / 1000 });
  } catch (error) {
    row = `| ${asset.id} | ✗ | crash: ${(error as Error).message.slice(0, 120)} | | | | |`;
    rows.push({ asset: asset.id, ok: false, crash: (error as Error).message });
  }
  appendFileSync(values.out!, row + "\n");
  console.log(`[DEBUG] ${asset.id} ${((Date.now() - started) / 1000).toFixed(1)}s`);
}
if (values.json) writeFileSync(values.json, JSON.stringify(rows, null, 1));
