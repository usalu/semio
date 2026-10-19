import { readFileSync, writeFileSync } from "node:fs";
import { workspacePackages } from "./pkgs.ts";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const pkg = workspacePackages().find((p) => p.name === "@semio-tech/process")!;
const existing = new Set(Object.keys(pkg.exports));
const SO = "🧰️framework/🔨️modules/🏃️process/";
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
type Row = { action: "new-entry" | "extend-existing-glue"; targetFile: string; proposedSubpath: string; proposedGlueFile: string; kind: "ts" | "json"; symbols: string[] | "*"; importers: string[] };
const rows = new Map<string, Row>();
const used = new Set(existing);
const existingBySuffix = new Map(Object.entries(pkg.exports).filter(([, v]) => v).map(([k, v]) => [String(v).replace(/^\.\//u, ""), k]));
for (const s of plan.skipped as any[]) {
  const rel = (s.to as string).slice(SO.length);
  let row = rows.get(s.to);
  if (!row) {
    const json = rel.endsWith(".json");
    let slug = strip(rel.replace(/\/(?:🟦️|🔣️)\.(?:ts|json)$/u, "").replace(/\.(?:ts|json)$/u, "")).replace(/(^|\/)tests(?=\/|$)/gu, "$1testing").replace(/(^|\/)testing\/testing(?=\/|$)/gu, "$1testing");
    let sub = "./" + slug + (json ? ".json" : "");
    const glue = json ? rel : "📦️packages/🟦️typescript/" + rel;
    const extend = !json ? existingBySuffix.get(rel) : undefined;
    if (extend) sub = extend;
    else for (let i = 2; used.has(sub); i++) sub = "./" + slug + (json ? ".json" : "") + "-" + i;
    used.add(sub);
    row = { action: extend ? "extend-existing-glue" : "new-entry", targetFile: s.to, proposedSubpath: sub, proposedGlueFile: json ? rel : "📦️packages/🟦️typescript/" + rel, kind: json ? "json" : "ts", symbols: [], importers: [] };
    rows.set(s.to, row);
  }
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...((s.names as string[]) ?? [])])].sort();
  row.importers.push(s.from);
}
const byPackage = { "@semio-tech/process": Object.fromEntries([...rows].sort((a, b) => a[0].localeCompare(b[0]))) };
const summary = [{ package: "@semio-tech/process", newExportEntries: [...rows.values()].filter((r) => r.action === "new-entry").length, extendedExistingEntries: [...rows.values()].filter((r) => r.action === "extend-existing-glue").length, importerEdges: [...rows.values()].reduce((n, r) => n + r.importers.length, 0) }];
writeFileSync(process.argv[3]!, JSON.stringify({ summary, byPackage }, null, 1));
console.log(summary, "json rows", [...rows.values()].filter((r) => r.kind === "json").length);
