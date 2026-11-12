/** 📊️ Aggregates the drafts of `volume-draft.ts --all` into the quality tables of `volume-drafting.md`.
 *
 * Usage: `repo tools exec bun -- volume-summary.ts [--drafts <dir>] [--out <file>]` (defaults: `🗑️generated/drafts`, `🗑️generated/tables.md`; also writes `summary.json` next to the output).
 * @see ./volume-drafting.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

import { GENERATED, median } from "./volume-lib.ts";

type Row = { category: string; slug: string; solids: number; err: number[]; iou: number[]; recall: number[]; precision: number[]; needs: number; low: number; unmatched: number; skipped: number; elevationShapes: number; worstShapes: string[]; worstSolids: string[] };

const args = process.argv.slice(2);
const flag = (name: string, fallback: string): string => {
  const i = args.indexOf(name);
  return i >= 0 ? resolve(args[i + 1]!) : fallback;
};
const drafts = flag("--drafts", join(GENERATED, "drafts"));
const out = flag("--out", join(GENERATED, "tables.md"));

const index = JSON.parse(readFileSync(join(drafts, "index.json"), "utf8")) as { assets: { category: string; slug: string; path?: string; error?: string }[] };
const rows: Row[] = [];
const failed: string[] = [];
for (const entry of index.assets) {
  if (entry.path === undefined) {
    failed.push(`${entry.category}/${entry.slug}: ${entry.error}`);
    continue;
  }
  const { report } = JSON.parse(readFileSync(entry.path, "utf8"));
  const s = report.silhouette;
  rows.push({
    category: entry.category,
    slug: entry.slug,
    solids: report.counts.solids,
    err: report.bboxM.relativeError,
    iou: [s.elevation.iou, s.front.iou, s.plan.iou],
    recall: [s.elevation.recall, s.front.recall, s.plan.recall],
    precision: [s.elevation.precision, s.front.precision, s.plan.precision],
    needs: report.counts.needsAuthor,
    low: report.counts.lowConfidence,
    unmatched: report.counts.unmatchedFront + report.counts.unmatchedPlan,
    skipped: report.counts.skipped,
    elevationShapes: report.counts.elevationShapes,
    worstShapes: ["elevation", "front", "plan"].map((v) => `${v}:${s[v].worstShapes[0]?.id ?? "-"}`),
    worstSolids: ["elevation", "front", "plan"].map((v) => `${v}:${s[v].worstSolids[0]?.id ?? "-"}`),
  });
}

const pct = (x: number): string => `${x >= 0 ? "+" : ""}${(x * 100).toFixed(0)}`;
const f2 = (x: number): string => x.toFixed(2);
const ok = (r: Row): boolean => r.err.every((e) => Math.abs(e) <= 0.08) && r.iou.every((i) => i >= 0.8);
const lines: string[] = [];
const categories = [...new Set(rows.map((r) => r.category))].sort();
const medIou = (list: readonly Row[], k: number): number => median(list.map((r) => r.iou[k]!));

lines.push("| category | assets | median IoU e/f/p | median abs bbox err % x/y/z | pass both gates | needs-author (assets) | low-confidence (assets) | unmatched shapes |", "|---|---|---|---|---|---|---|---|");
for (const c of categories) {
  const list = rows.filter((r) => r.category === c);
  lines.push(`| ${c} | ${list.length} | ${[0, 1, 2].map((k) => f2(medIou(list, k))).join(" / ")} | ${[0, 1, 2].map((k) => (median(list.map((r) => Math.abs(r.err[k]!))) * 100).toFixed(0)).join(" / ")} | ${list.filter(ok).length} | ${list.reduce((a, r) => a + r.needs, 0)} (${list.filter((r) => r.needs > 0).length}) | ${list.reduce((a, r) => a + r.low, 0)} (${list.filter((r) => r.low > 0).length}) | ${list.reduce((a, r) => a + r.unmatched, 0)} |`);
}
lines.push(`| **all** | ${rows.length} | ${[0, 1, 2].map((k) => f2(medIou(rows, k))).join(" / ")} | ${[0, 1, 2].map((k) => (median(rows.map((r) => Math.abs(r.err[k]!))) * 100).toFixed(0)).join(" / ")} | ${rows.filter(ok).length} | ${rows.reduce((a, r) => a + r.needs, 0)} (${rows.filter((r) => r.needs > 0).length}) | ${rows.reduce((a, r) => a + r.low, 0)} (${rows.filter((r) => r.low > 0).length}) | ${rows.reduce((a, r) => a + r.unmatched, 0)} |`, "");

for (const c of categories) {
  lines.push(`### ${c}`, "", "| asset | solids | bbox err % x/y/z | IoU e/f/p | recall e/f/p | precision e/f/p | needs-author | low-conf | unmatched | skipped |", "|---|---|---|---|---|---|---|---|---|---|");
  for (const r of rows.filter((x) => x.category === c)) lines.push(`| ${r.slug} | ${r.solids} | ${r.err.map(pct).join(" / ")} | ${r.iou.map(f2).join(" / ")} | ${r.recall.map(f2).join(" / ")} | ${r.precision.map(f2).join(" / ")} | ${r.needs} | ${r.low} | ${r.unmatched} | ${r.skipped} |`);
  lines.push("");
}

if (failed.length > 0) lines.push("### Drafting errors", "", ...failed.map((f) => `- ${f}`), "");
writeFileSync(out, `${lines.join("\n")}\n`);
writeFileSync(out.replace(/\.md$/u, ".json"), `${JSON.stringify({ rows, failed }, null, 1)}\n`);
process.stdout.write(`${rows.length} assets summarised, ${failed.length} failed; wrote ${out}\n`);
