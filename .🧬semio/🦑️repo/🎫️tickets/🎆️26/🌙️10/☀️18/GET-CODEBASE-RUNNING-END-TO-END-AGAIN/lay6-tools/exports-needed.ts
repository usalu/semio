import { readFileSync, writeFileSync } from "node:fs";
import { workspacePackages } from "./pkgs.ts";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const pkgs = workspacePackages();
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (rel: string): string => strip(rel).replace(/\.ts$/u, "").replace(/\/?\.json$/u, "").replace(/^\/|\/$/g, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/\/?$/u, "");
type Row = { targetFile: string; proposedSubpath: string; symbols: string[] | "*"; importers: string[] };
const out: Record<string, Record<string, Row>> = {};
for (const s of plan.skipped) {
  const names = (s.pkg as string).split(",");
  const pkg = pkgs.find((p) => names.includes(p.name))!;
  const base = pkg.sourceOwner === "" ? "🧰️framework/🛍️products/💻️os" : pkg.sourceOwner;
  const rel = s.to.startsWith(base + "/") ? s.to.slice(base.length + 1) : s.to;
  const key = names[0]!;
  const row = ((out[key] ??= {})[s.to] ??= { targetFile: s.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [] });
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...(s.names as string[] ?? [])])].sort();
  row.importers.push(s.from);
}
const summary = Object.entries(out).map(([pkg, rows]) => ({ package: pkg, newExportEntries: Object.keys(rows).length, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length, 0) }));
writeFileSync(process.argv[3]!, JSON.stringify({ summary, byPackage: out }, null, 1));
console.log(summary);
