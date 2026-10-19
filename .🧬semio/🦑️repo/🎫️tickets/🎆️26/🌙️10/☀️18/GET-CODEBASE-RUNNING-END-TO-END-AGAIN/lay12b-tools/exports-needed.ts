import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const root = process.cwd();
const [, , cruisePath, outPath] = process.argv;
type V = { from: string; to: string; rule: { name: string } };
const violations = (JSON.parse(readFileSync(cruisePath!, "utf8")).summary.violations as V[]).filter((v) => v.rule.name === "no-cross-package-relative-@semio-tech/framework");
const UI = "🧰️framework/🔨️modules/🖱️ui/";
const MODULES = "🧰️framework/🔨️modules/";
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (p: string): string => strip(p.startsWith(MODULES) ? p.slice(MODULES.length) : p).replace(/\.(tsx?|json)$/u, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/\/$/u, "");
const stmt = /(?:import|export)\s+(type\s+)?(?:([\w$]+)\s*,?\s*)?(?:\*\s+as\s+([\w$]+)|\{([^}]*)\})?\s*(?:from\s*)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/gu;
const symbolsOf = (from: string, to: string): string[] | "*" => {
  const src = readFileSync(resolve(root, from), "utf8");
  const names = new Set<string>();
  let star = false;
  for (const m of src.matchAll(stmt)) {
    const spec = m[5] ?? m[6];
    if (!spec || !spec.startsWith(".")) continue;
    const target = relative(root, resolve(dirname(resolve(root, from)), spec)).normalize("NFC");
    if (target !== to.normalize("NFC")) continue;
    if (m[3] || m[6]) star = true;
    if (m[2]) names.add("default");
    for (const part of (m[4] ?? "").split(",")) { const n = part.trim().replace(/^type\s+/u, "").replace(/\s+as\s+.*$/u, ""); if (n) names.add(n); }
  }
  return star ? "*" : [...names].sort();
};
type Row = { targetFile: string; proposedSubpath: string; subpathAlreadyExported: boolean; scope: "ui-internal" | "ui-seam-or-other"; symbols: string[] | "*"; importers: string[] };
const exported = new Set(Object.keys(JSON.parse(readFileSync(resolve(root, "🧰️framework/📦️packages/🟦️typescript/package.json"), "utf8")).exports ?? {}));
const rows = new Map<string, Row>();
const seen = new Set<string>();
for (const v of violations) {
  const key = v.from + "\0" + v.to;
  if (seen.has(key)) continue;
  seen.add(key);
  const row = rows.get(v.to) ?? { targetFile: v.to, proposedSubpath: "./" + slug(v.to), subpathAlreadyExported: exported.has("./" + slug(v.to)), scope: "ui-internal", symbols: [], importers: [] };
  if (!v.from.startsWith(UI)) row.scope = "ui-seam-or-other";
  const syms = symbolsOf(v.from, v.to);
  if (syms === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...row.symbols, ...syms])].sort();
  row.importers.push(v.from);
  rows.set(v.to, row);
}
const list = [...rows.values()].sort((a, b) => a.proposedSubpath.localeCompare(b.proposedSubpath));
const ui = list.filter((r) => r.scope === "ui-internal");
const summary = {
  package: "@semio-tech/framework",
  note: "Each row is one cross-package relative target the importing nested package (ui-react, ui-styling, ui locale, tests) reaches through a path. The rule fires once for the framework package and once for its -rs twin per edge. Alternatives per row: export the target from @semio-tech/framework (proposedSubpath), or relocate the importer's dependency into the importing package.",
  newExportEntries: list.length,
  importerEdges: seen.size,
  uiInternal: { newExportEntries: ui.length, importerEdges: ui.reduce((n, r) => n + r.importers.length, 0), violationsCountedByCruiser: ui.reduce((n, r) => n + r.importers.length, 0) * 2 },
  subpathsAlreadyExported: list.filter((r) => r.subpathAlreadyExported).length,
  uiSeamOrOther: { newExportEntries: list.length - ui.length, importerEdges: list.filter((r) => r.scope !== "ui-internal").reduce((n, r) => n + r.importers.length, 0) },
};
writeFileSync(outPath!, JSON.stringify({ summary, removeExports: [{ package: "@semio-tech/ui-styling", subpath: "./builder/python/inventory", reason: "the Python environment inventory moved to the repository library product (its only consumer); the stub 🏗️builder/🐍️python/📋️inventory was deleted" }], entries: list }, null, 1));
console.log(JSON.stringify(summary, null, 1));
