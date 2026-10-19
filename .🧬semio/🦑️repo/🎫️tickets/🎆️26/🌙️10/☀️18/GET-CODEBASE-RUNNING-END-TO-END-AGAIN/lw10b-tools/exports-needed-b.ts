import { readFileSync, writeFileSync } from "node:fs";
import { workspacePackages } from "../lay6-tools/pkgs.ts";
const closureFiles = new Set(readFileSync(process.argv[4]!, "utf8").split("\n").filter(Boolean));
const plan = { skipped: process.argv[2]!.split(",").flatMap((f) => JSON.parse(readFileSync(f, "utf8")).skipped) };
const pkgs = workspacePackages();
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (rel: string): string => strip(rel).replace(/\.ts$/u, "").replace(/\/?\.json$/u, "").replace(/^\/|\/$/g, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing");
type Row = { targetFile: string; proposedSubpath: string; symbols: string[] | "*"; importers: string[]; closureImporters: string[] };
const out: Record<string, Record<string, Row>> = {};
for (const s of plan.skipped) {
  const names: string[] = s.pkgs;
  const cands = pkgs.filter((p) => names.includes(p.name));
  const pkg = cands.sort((a, b) => b.sourceOwner.length - a.sourceOwner.length)[0];
  if (!pkg) continue;
  const base = pkg.sourceOwner;
  const rel = s.to.startsWith(base + "/") ? s.to.slice(base.length + 1) : s.to;
  const key = pkg.name;
  const row = ((out[key] ??= {})[s.to] ??= { targetFile: s.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [], closureImporters: [] });
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...(s.names as string[] ?? [])])].sort();
  const bucket = closureFiles.has(s.from) ? row.closureImporters : row.importers;
  if (!bucket.includes(s.from)) bucket.push(s.from);
}
const summary = Object.entries(out).map(([pkg, rows]) => ({ package: pkg, newExportEntries: Object.keys(rows).length, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length + r.closureImporters.length, 0), closureImporterEdges: Object.values(rows).reduce((n, r) => n + r.closureImporters.length, 0) }));
writeFileSync(process.argv[3]!, JSON.stringify({ summary, byPackage: out }, null, 1));
console.log(summary);
