/**
 * 📋 Consolidates the skipped (no existing public entry) edges into one exports spec and checks it against the repoapi plan, the live manifests and the glue files on disk.
 *
 *   bun exports-spec.ts <plan.json> <out.json>
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { packageIndex } from "../repoapi-tools/packages.ts";

const [planPath, outPath] = process.argv.slice(2) as [string, string];
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(planPath, "utf8"));
const final = JSON.parse(readFileSync(resolve(import.meta.dir, "../repoapi-tools/exports-final.json"), "utf8"));
const index = packageIndex();
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (rel: string): string => strip(rel).replace(/\.(ts|tsx|mjs|js)$/u, "").replace(/\/?\.json$/u, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/(^|\/)modules(?=\/)/gu, "$1").replace(/^\/|\/$/gu, "");
type Row = { target: string; proposedSubpath: string; glueFile: string; symbols: string[] | "*"; importers: string[]; status: string; planned?: { subpath: string; glueExists: boolean; manifestHasKey: boolean; manifestTarget?: string } };
const byPackage: Record<string, Record<string, Row>> = {};
const dispositions: Record<string, number> = {};
const categories: Record<string, number> = {};
const other: { from: string; to: string; specifier: string; reason: string; pkg: string }[] = [];
for (const s of plan.skipped as any[]) {
  if (!/^(no existing entry exposes|namespace)/u.test(s.reason)) { other.push({ from: s.from, to: s.to, specifier: s.specifier, reason: s.reason, pkg: s.pkg }); categories[s.reason.replace(/:.*/u, "")] = (categories[s.reason.replace(/:.*/u, "")] ?? 0) + 1; continue; }
  const names = (s.pkg as string).split(",");
  const pkgs = names.map(n => index.byName.get(n)).filter(Boolean) as any[];
  if (!pkgs.length) { other.push({ from: s.from, to: s.to, specifier: s.specifier, reason: "package unknown", pkg: s.pkg }); continue; }
  const siblings = index.all.filter(p => pkgs.some(q => q.sourceOwner === p.sourceOwner));
  const primary = pkgs.find(p => !p.name.endsWith("-rs") && Object.keys(p.exports).length) ?? pkgs[0]!;
  let planned: Row["planned"];
  let row: Row | undefined;
  for (const p of siblings) {
    const entry = Object.values(final.packages[p.name]?.entries ?? {}).find((e: any) => e.targetFile === s.to) as any;
    if (entry) {
      const live = (p.exports as any)[entry.subpath];
      planned = { subpath: entry.subpath, glueExists: !!entry.glueFile && existsSync(resolve(ROOT, entry.glueFile)), manifestHasKey: live !== undefined, manifestTarget: typeof live === "string" ? live : live ? JSON.stringify(live) : undefined };
      const pkgRows = (byPackage[p.name] ??= {});
      row = pkgRows[s.to] ??= { target: s.to, proposedSubpath: entry.subpath, glueFile: entry.glueFile ?? "", symbols: [], importers: [], status: "", planned };
      break;
    }
  }
  if (!row) {
    const base = primary.sourceOwner;
    const rel = s.to.startsWith(base + "/") ? s.to.slice(base.length + 1) : s.to;
    const pkgRows = (byPackage[primary.name] ??= {});
    row = pkgRows[s.to] ??= { target: s.to, proposedSubpath: "./" + (slug(rel) || "index"), glueFile: "", symbols: [], importers: [], status: "unplanned" };
  }
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...(s.names as string[])])].sort();
  row.importers.push(s.from);
}
for (const rows of Object.values(byPackage)) for (const row of Object.values(rows)) {
  row.importers = [...new Set(row.importers)];
  if (row.planned) row.status = !row.planned.manifestHasKey ? (row.planned.glueExists ? "planned: glue exists, manifest key missing" : "planned: glue and manifest key missing") : !row.planned.glueExists ? "planned: manifest key present, glue missing" : "planned: key and glue present, glue lacks the imported names";
  dispositions[row.status] = (dispositions[row.status] ?? 0) + 1;
}
const summary = Object.entries(byPackage).map(([pkg, rows]) => ({ package: pkg, entries: Object.keys(rows).length, importerFiles: new Set(Object.values(rows).flatMap(r => r.importers)).size, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length, 0), unplanned: Object.values(rows).filter(r => r.status === "unplanned").length })).sort((a, b) => b.importerEdges - a.importerEdges);
writeFileSync(outPath, JSON.stringify({ generatedFrom: "dependency-cruiser area runs after lay13", dispositions, summary, byPackage, notExportable: other, categories }, null, 1));
console.log(JSON.stringify({ dispositions, categories, summary: summary.slice(0, 12), packages: summary.length, entries: summary.reduce((n, s) => n + s.entries, 0) }, null, 1));
