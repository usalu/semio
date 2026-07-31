#!/usr/bin/env bun
/** 📊Phase B work-list builder: walks the raw per-tool coverage output directly (not the flat merged
 * summary, which loses provenance and can't disambiguate same-named files like "index.ts" across
 * different bundles), resolves each lcov group's owning bundle via its `coverageSlug` directory/filename,
 * and writes a ranked `worklist.json` — the hand-off artifact test-writing waves (Phase C) consume.
 * Ticket: 26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE. */
import { coverageDir, coverageSlug, parseLcov, goProfileToLcov, isCoverageExcluded, type LcovFileRecord } from "../../../../../../repo/lib/js/index.ts";
import { readFileSync, readdirSync, existsSync, writeFileSync, statSync } from "node:fs";
import { join, dirname, relative } from "node:path";

const repoRoot = process.cwd();

//#region 🔎ScriptTsScan
type BundleInfo = { scriptCwd: string; absRoot: string; lang: "rust" | "js" | "go" | "py" | "dotnet"; cargoPackage?: string; vitestProject?: string; goModuleLabel?: string };

function findScriptTsFiles(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    if (name === "node_modules" || name === "target" || name === ".claude" || name.startsWith(".git")) continue;
    const full = join(dir, name);
    const st = statSync(full);
    if (st.isDirectory()) findScriptTsFiles(full, out);
    else if (name === "script.ts") out.push(full);
  }
  return out;
}

function parseBundleInfo(scriptPath: string): BundleInfo | null {
  const text = readFileSync(scriptPath, "utf8");
  const absRoot = dirname(scriptPath);
  const scriptCwd = relative(repoRoot, absRoot);
  const cargoMatch = /runCargoTestBudgeted\(\s*\[\s*"([^"]+)"/.exec(text);
  if (cargoMatch) return { scriptCwd, absRoot, lang: "rust", cargoPackage: cargoMatch[1] };
  if (/runVitest\(/.test(text)) {
    const configPath = existsSync(join(absRoot, "vitest.config.ts")) ? join(absRoot, "vitest.config.ts") : existsSync(join(absRoot, "js", "vitest.config.ts")) ? join(absRoot, "js", "vitest.config.ts") : null;
    const vitestProject = configPath ? /name:\s*"([^"]+)"/.exec(readFileSync(configPath, "utf8"))?.[1] : undefined;
    return { scriptCwd, absRoot, lang: "js", vitestProject };
  }
  if (/goLevelTestArgs|goCoverageArgs/.test(text)) return { scriptCwd, absRoot, lang: "go" };
  if (/pytestLevelArgs|pytestCoverageArgs/.test(text)) return { scriptCwd, absRoot, lang: "py" };
  if (/dotnetLevelArgs|dotnetCoverageArgs/.test(text)) return { scriptCwd, absRoot, lang: "dotnet" };
  return null;
}

const bundles: BundleInfo[] = findScriptTsFiles(repoRoot)
  .map(parseBundleInfo)
  .filter((b): b is BundleInfo => b !== null);

// Root script.ts also drives repo-cli/repo-mcp-* variants via `goCoverageArgs(this.root, "./repo/client/cli/go")`.
bundles.push({ scriptCwd: "repo/client/cli", absRoot: join(repoRoot, "repo/client/cli"), lang: "go", goModuleLabel: "./repo/client/cli/go" });

const slugToBundle = new Map<string, BundleInfo>();
for (const b of bundles) {
  // Rust lcov files are now slugged by cargo package name (see runCargoTestBudgeted) since many crates build
  // from `this.repoRoot`, not their own bundle dir — matching on absRoot would collide dozens of crates onto
  // one slug. JS/py still slug by absRoot (their coverage helpers always use the bundle's own cwd).
  slugToBundle.set(b.lang === "rust" && b.cargoPackage ? coverageSlug(b.cargoPackage) : coverageSlug(b.absRoot), b);
  if (b.goModuleLabel) slugToBundle.set(coverageSlug(b.goModuleLabel), b);
}
//#endregion 🔎ScriptTsScan

//#region 🗄️GroupedRecords
type Group = { bundle: BundleInfo | null; groupKey: string; records: LcovFileRecord[] };
const groups: Group[] = [];

function walk(dir: string, matches: (n: string) => boolean, found: string[] = []): string[] {
  if (!existsSync(dir)) return found;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (statSync(full).isDirectory()) walk(full, matches, found);
    else if (matches(entry.name)) found.push(full);
  }
  return found;
}

for (const file of walk(coverageDir(repoRoot, "rust"), (n) => n.endsWith(".lcov"))) {
  const groupKey = file.split("/").pop()!.replace(/\.lcov$/, "");
  groups.push({ bundle: slugToBundle.get(groupKey) ?? null, groupKey, records: parseLcov(readFileSync(file, "utf8")) });
}
for (const file of walk(coverageDir(repoRoot, "js"), (n) => n === "lcov.info")) {
  const groupKey = dirname(file).split("/").pop()!;
  groups.push({ bundle: slugToBundle.get(groupKey) ?? null, groupKey, records: parseLcov(readFileSync(file, "utf8")) });
}
for (const file of walk(coverageDir(repoRoot, "py"), (n) => n.endsWith(".lcov"))) {
  const groupKey = file.split("/").pop()!.replace(/\.lcov$/, "");
  groups.push({ bundle: slugToBundle.get(groupKey) ?? null, groupKey, records: parseLcov(readFileSync(file, "utf8")) });
}
for (const file of walk(coverageDir(repoRoot, "go"), (n) => n.endsWith(".cover"))) {
  const groupKey = file.split("/").pop()!.replace(/\.cover$/, "");
  groups.push({ bundle: slugToBundle.get(groupKey) ?? null, groupKey, records: goProfileToLcov(readFileSync(file, "utf8")) });
}
//#endregion 🗄️GroupedRecords

//#region 📄WorklistItems
type WorklistItem = {
  id: string;
  lang: string;
  sourceFile: string;
  groupKey: string;
  scriptCwd: string | null;
  cargoPackage?: string;
  vitestProject?: string;
  totalLines: number;
  coveredLines: number;
  pct: number;
  priority: number;
  status: "pending";
  attempts: 0;
};

const items: WorklistItem[] = [];
let unresolvedGroups = 0;
for (const group of groups) {
  if (!group.bundle) unresolvedGroups++;
  for (const record of group.records) {
    if (isCoverageExcluded(record.path)) continue;
    const found = record.lines.size;
    const hit = [...record.lines.values()].filter((c) => c > 0).length;
    if (found === 0) continue;
    const id = `${group.groupKey}__${record.path}`.replace(/[^a-zA-Z0-9_-]+/g, "_").replace(/^_+|_+$/g, "").slice(-100);
    items.push({
      id,
      lang: group.bundle?.lang ?? "unknown",
      sourceFile: record.path,
      groupKey: group.groupKey,
      scriptCwd: group.bundle?.scriptCwd ?? null,
      cargoPackage: group.bundle?.cargoPackage,
      vitestProject: group.bundle?.vitestProject,
      totalLines: found,
      coveredLines: hit,
      pct: (hit / found) * 100,
      priority: found - hit,
      status: "pending",
      attempts: 0,
    });
  }
}
items.sort((a, b) => b.priority - a.priority);

const linesFound = items.reduce((s, i) => s + i.totalLines, 0);
const linesHit = items.reduce((s, i) => s + i.coveredLines, 0);
//#endregion 📄WorklistItems

const worklist = {
  version: 2,
  generatedAt: "2026-07-26",
  repoWidePct: { baseline: linesFound ? (linesHit / linesFound) * 100 : 0, current: linesFound ? (linesHit / linesFound) * 100 : 0, target: 95.0 },
  items,
  exclusions: [] as { path: string; reason: string }[],
};

const outPath = join(repoRoot, ".repo/🎫/26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE/worklist.json");
writeFileSync(outPath, JSON.stringify(worklist, null, 2));
console.log(`worklist v2: ${groups.length} groups (${unresolvedGroups} unresolved), ${items.length} items, ${linesHit}/${linesFound} lines = ${worklist.repoWidePct.baseline.toFixed(2)}%`);
console.log(`top 20 by uncovered lines:`);
for (const it of items.slice(0, 20)) console.log(`  ${it.priority} uncovered (${it.pct.toFixed(1)}%)  ${it.sourceFile}  [${it.groupKey}] -> ${it.scriptCwd ?? "???"}`);
console.log(`unresolved groups:`);
for (const g of groups.filter((g) => !g.bundle)) console.log(`  ${g.groupKey} (${g.records.length} files)`);
