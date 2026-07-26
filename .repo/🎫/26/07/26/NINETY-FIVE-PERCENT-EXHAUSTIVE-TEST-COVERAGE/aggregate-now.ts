#!/usr/bin/env bun
/** 📊Runs the same aggregation TestScript.enforceCoverageGate uses, against whatever .repo/coverage/ currently
 * holds, without re-running any tests — for inspecting a partial/interrupted Phase A baseline.
 * Ticket: 26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE. */
import { coverageDir, parseLcov, mergeLcov, summarizeCoverage, goProfileToLcov, type LcovFileRecord } from "../../../../../../repo/lib/js/index.ts";
import { readFileSync, readdirSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
function walk(dir: string, matches: (n: string) => boolean, found: string[] = []): string[] {
  if (!existsSync(dir)) return found;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) walk(full, matches, found);
    else if (matches(entry.name)) found.push(full);
  }
  return found;
}

const recordSets: LcovFileRecord[][] = [
  ...walk(coverageDir(root, "rust"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
  ...walk(coverageDir(root, "js"), (n) => n === "lcov.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
  ...walk(coverageDir(root, "py"), (n) => n.endsWith(".lcov")).map((f) => parseLcov(readFileSync(f, "utf8"))),
  ...walk(coverageDir(root, "dotnet"), (n) => n === "coverage.info").map((f) => parseLcov(readFileSync(f, "utf8"))),
  ...walk(coverageDir(root, "go"), (n) => n.endsWith(".cover")).map((f) => goProfileToLcov(readFileSync(f, "utf8"))),
];
const merged = mergeLcov(recordSets);
const summary = summarizeCoverage(merged);
console.log(`repo-wide (partial baseline): ${summary.linesHit}/${summary.linesFound} lines = ${summary.pct.toFixed(2)}%`);
console.log(`files counted: ${summary.perFile.length}`);
console.log("worst 20:");
for (const f of summary.perFile.slice(0, 20)) console.log(`  ${f.pct.toFixed(1)}% (${f.linesHit}/${f.linesFound})  ${f.path}`);
console.log("best 10:");
for (const f of summary.perFile.slice(-10)) console.log(`  ${f.pct.toFixed(1)}% (${f.linesHit}/${f.linesFound})  ${f.path}`);
writeFileSync(join(".repo/🎫/26/07/26/NINETY-FIVE-PERCENT-EXHAUSTIVE-TEST-COVERAGE", "phase-a-partial-summary.json"), JSON.stringify(summary, null, 2));
