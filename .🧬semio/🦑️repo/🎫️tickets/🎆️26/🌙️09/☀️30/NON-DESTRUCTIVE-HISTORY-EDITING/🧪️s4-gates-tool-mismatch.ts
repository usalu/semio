/** 🔀️ S4-GATES F9 cross-check: every raw per-plugin `*-tool-mismatch` refusal (anonymous text or unlabelled code) a guest raises, with
 * owner, sites and whether the framework code `app.command.tool-mismatch` is already used beside it. */
import { faultNoticeReport } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/📋️orchestration/🟦️.ts";

const report = faultNoticeReport("/Users/ueli/Documents/semio", "", "all");
const owner = (path: string): string => (path.startsWith("🧰️framework/") ? "🔌️plugin" : path.split("/")[2]!);
const raw = new Map<string, { owner: string; paths: Set<string> }>();
for (const site of report.sites) {
  const named = site.code ?? site.text;
  if (named === null || !/tool-(?:mismatch|unmapped)|tool-mismatch/u.test(named) || site.code === "app.command.tool-mismatch" || site.defaulted) continue;
  const entry = raw.get(named) ?? { owner: owner(site.path), paths: new Set<string>() };
  entry.paths.add(site.path);
  raw.set(named, entry);
}
const adopted = new Set(report.sites.filter((site) => site.code === "app.command.tool-mismatch").map((site) => owner(site.path)));
const rows = [...raw].sort(([a, x], [b, y]) => x.owner.localeCompare(y.owner) || a.localeCompare(b));
for (const [code, entry] of rows) console.log(`${entry.owner}\t${code}\t${entry.paths.size}\t${adopted.has(entry.owner) ? "partial" : "none"}\t${[...entry.paths][0]}`);
console.log(`distinct ${rows.length}, sites ${rows.reduce((sum, [, entry]) => sum + entry.paths.size, 0)}, owners ${new Set(rows.map(([, entry]) => entry.owner)).size}; adopted owners ${[...adopted].sort().join(" ")}`);
