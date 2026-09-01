#!/usr/bin/env bun
// 🔬️ Scratch gate-validation driver for the gif@87a/✳️any fixture corpus wave.
//
// Reads the projections `🦀️probe` produced with the committed oracle and judges them with the
// FRAMEWORK'S OWN `compareProjections`, under the real `semantic-raster-v1` profile resolved from
// the loaded oracle registry — never with a comparison written here. Three questions:
//
//   1. witnessability — does each of the 12 declared kinds move the compared surface at all?
//   2. the inverse law — does the oracle's own inverse restore the base projection?
//   3. the gate, BOTH ways — accept the fixture against itself, reject a genuinely wrong document.
//
//   bun 📜️script.ts <probe-report.json>

import { compareProjections, loadOracleRegistry, profileTable } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const PROFILE = "semantic-raster-v1";
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const report = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const profiles = profileTable(loadOracleRegistry(repoRoot));
if (!profiles.has(PROFILE)) throw new Error(`${PROFILE} is not in the loaded registry — ${[...profiles.keys()].join(", ")}`);

const judge = (left: unknown, right: unknown) => compareProjections(PROFILE, left, right, profiles);
const paths = (verdict: ReturnType<typeof judge>) => verdict.diffs.map((diff) => diff.path.replace(/^\$\./, "")).join(", ");

console.log(`[fixture] ${report.fixture} — ${report.fixtureBytes} bytes, magic ${JSON.stringify(report.magic)}`);
console.log(`[base] ${JSON.stringify(report.base)}`);

console.log(`\n[gate accept] fixture vs itself: equal=${judge(report.base, report.base).equal} diffs=${judge(report.base, report.base).diffs.length}`);
for (const wrong of report.wrongDocuments) {
  const verdict = judge(report.base, wrong.projection);
  console.log(`[gate reject] fixture vs ${wrong.label}: equal=${verdict.equal} diffs=${verdict.diffs.length} — ${paths(verdict)}`);
  for (const diff of verdict.diffs) console.log(`[gate reject]   ${diff.path}: oracle=${JSON.stringify(diff.oracle)} subject=${JSON.stringify(diff.subject)} (${diff.reason})`);
}

console.log("");
let unwitnessable = 0;
let inverseFailures = 0;
for (const row of report.cases) {
  const moved = judge(report.base, row.projection);
  const restored = judge(report.base, row.restoredProjection);
  const expectedStill = row.kind === "no-mutation";
  if (moved.equal !== expectedStill) unwitnessable += 1;
  if (!restored.equal) inverseFailures += 1;
  console.log(
    `[kind] ${String(row.kind).padEnd(26)} bytes=${String(row.mutatedBytes).padStart(4)} byteIdentical=${String(row.byteIdenticalToFixture).padEnd(5)} movedDiffs=${String(moved.diffs.length).padStart(2)} inverseRestoresProjection=${String(restored.equal).padEnd(5)} inverseRestoresBytes=${row.restoredBytesIdenticalToFixture}` +
      (moved.diffs.length > 0 ? `  moved: ${paths(moved)}` : ""),
  );
}
console.log(`\n[summary] ${report.cases.length} kind(s); ${unwitnessable} not behaving as declared; ${inverseFailures} inverse law failure(s)`);
