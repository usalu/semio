import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { resolve } from "node:path";
const R = "/Users/ueli/Documents/semio/";
type Pkg = { name: string; owner: string; sourceOwner: string };
const seen = new Map<string, Pkg>();
for (const base of ["", "✏️s/", "♻️mit-bestand/"]) {
  const j = JSON.parse(readFileSync(R + base + "package.json", "utf8"));
  for (const w of (j.workspaces ?? []) as string[]) {
    const owner = resolve(R + base, w).slice(R.length);
    const f = R + owner + "/package.json";
    if (!existsSync(f) || seen.has(owner)) continue;
    seen.set(owner, { name: JSON.parse(readFileSync(f, "utf8")).name, owner, sourceOwner: owner.includes("/📦️packages/") ? owner.split("/📦️packages/")[0]! : owner });
  }
}
const pkgs = [...seen.values()];
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const strip = (s: string): string => s.replace(/[^\x00-\x7f]/gu, "").replace(/\/+/gu, "/").replace(/^\/|\/$/gu, "");
const slug = (rel: string): string => strip(rel).replace(/\.ts$/u, "").replace(/\.tsx$/u, "").replace(/\.mjs$/u, "").replace(/\/?\.json$/u, "").replace(/^\/|\/$/g, "").replace(/(^|\/)tests(?=\/|$)/gu, "$1testing");
type Row = { targetFile: string; proposedSubpath: string; symbols: string[] | "*"; importers: string[] };
const out: Record<string, Record<string, Row>> = {};
const unowned: { from: string; specifier: string; reason: string }[] = [];
for (const s of plan.skipped) {
  if (!s.owner) { unowned.push({ from: s.from, specifier: s.specifier, reason: s.reason }); continue; }
  const group = pkgs.filter((p) => p.sourceOwner === s.owner);
  const named = (s.rules as string[]).map((r) => r.replace(/^no-cross-package-relative-/, "")).filter((r) => group.some((p) => p.name === r));
  const pkg = group.find((p) => named.includes(p.name)) ?? group.find((p) => p.owner.endsWith("/📦️packages/🟦️typescript")) ?? group[0]!;
  const rel = s.to.startsWith(s.owner + "/") ? s.to.slice(s.owner.length + 1) : s.to;
  const row = ((out[pkg.name] ??= {})[s.to] ??= { targetFile: s.to, proposedSubpath: "./" + (slug(rel) || "index"), symbols: [], importers: [] });
  if (s.names === "*" || row.symbols === "*") row.symbols = "*"; else row.symbols = [...new Set([...(row.symbols as string[]), ...((s.names as string[]) ?? [])])].sort();
  if (!row.importers.includes(s.from)) row.importers.push(s.from);
}
const collisions: Record<string, string[]> = {};
for (const [pkg, rows] of Object.entries(out)) {
  const by = new Map<string, string[]>();
  for (const r of Object.values(rows)) by.set(r.proposedSubpath, [...(by.get(r.proposedSubpath) ?? []), r.targetFile]);
  for (const [sub, files] of by) if (files.length > 1) collisions[pkg + " " + sub] = files;
}
const summary = Object.entries(out).map(([pkg, rows]) => ({ package: pkg, newExportEntries: Object.keys(rows).length, importerEdges: Object.values(rows).reduce((n, r) => n + r.importers.length, 0) }));
writeFileSync(process.argv[3]!, JSON.stringify({ summary, byPackage: out, unresolvedPackageEntries: unowned, subpathCollisions: collisions }, null, 1));
console.log(summary, "unowned", unowned.length, "collisions", Object.keys(collisions).length);
