import { readFileSync, writeFileSync } from "node:fs";
import { workspacePackages } from "./pkgs.ts";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const pkgs = workspacePackages();
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (rel: string): string => strip(rel).replace(/\.tsx?$/u, "").replace(/\/?\.json$/u, "").replace(/^\/|\/$/g, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/\/?$/u, "");
type Row = { targetFile: string; proposedSubpath: string; symbols: string[] | "*"; importers: string[]; rules: string[] };
const merge = (row: Row, s: any) => {
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...(s.names as string[] ?? [])])].sort();
  if (!row.importers.includes(s.from)) row.importers.push(s.from);
  for (const r of [s.hostRule ? "renderer-hosts-only-ui" : "", s.rulePkgs.length ? "no-cross-package-relative" : ""]) if (r && !row.rules.includes(r)) row.rules.push(r);
};
const byPackage: Record<string, Record<string, Row>> = {}, uiContract: Record<string, Row> = {};
const FW = "🧰️framework/";
for (const s of plan.skipped) {
  if (s.hostRule) {
    const base = s.to.startsWith(FW + "🔨️modules/") ? FW + "🔨️modules/" : s.to.startsWith(FW + "🛍️products/") ? FW + "🛍️products/" : FW;
    const rel = s.to.slice(base.length);
    merge((uiContract[s.to] ??= { targetFile: s.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [], rules: [] }), s);
    continue;
  }
  const names = s.rulePkgs as string[];
  const pkg = pkgs.find((p) => names.includes(p.name));
  if (!pkg) continue;
  const base = pkg.sourceOwner === "" ? "🧰️framework/🛍️products/💻️os" : pkg.sourceOwner;
  const rel = s.to.startsWith(base + "/") ? s.to.slice(base.length + 1) : s.to;
  merge(((byPackage[names[0]!] ??= {})[s.to] ??= { targetFile: s.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [], rules: [] }), s);
}
const summary = Object.entries(byPackage).map(([pkg, rows]) => ({ package: pkg, newExportEntries: Object.keys(rows).length, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length, 0) })).sort((a, b) => b.importerEdges - a.importerEdges);
const uiSummary = { package: "@semio-tech/framework (glue under 🧰️framework/📦️packages/🟦️typescript, the only non-ui public contract renderer hosts may import)", newExportEntries: Object.keys(uiContract).length, importerEdges: Object.values(uiContract).reduce((n, r) => n + r.importers.length, 0) };
writeFileSync(process.argv[3]!, JSON.stringify({ summary, uiContractSummary: uiSummary, byPackage, uiContract }, null, 1));
console.log(summary, uiSummary);
