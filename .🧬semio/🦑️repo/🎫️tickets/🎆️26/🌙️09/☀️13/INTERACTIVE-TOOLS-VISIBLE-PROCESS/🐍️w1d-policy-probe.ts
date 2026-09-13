/** 🔭️ W1-D probe: runs the tool-run and puzzle fill policy self-tests, then the live predicates, and prints every finding. */
import * as script from "/Users/ueli/Documents/semio/📜️script.ts";
import { interactivityToolRunPolicySelfTests } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts";
import { interactivityPuzzleFillRunJobSelfTests } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts";
import { interactivityPuzzleFillTraceSelfTests } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-trace/🟦️.ts";
import { interactivityPuzzleFillP4eSelfTests } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
console.log(`self-tests tool-run=${interactivityToolRunPolicySelfTests()} run-job=${interactivityPuzzleFillRunJobSelfTests()} trace=${interactivityPuzzleFillTraceSelfTests()} p4e=${interactivityPuzzleFillP4eSelfTests()}`);
if (process.argv.includes("--self-tests-only")) process.exit(0);
const fill = script.interactivityPuzzleFillSources(repoRoot);
for (const failure of script.interactivityPuzzleFillP4eFailures(fill.precompute, fill.fill, fill.geometry)) console.log(`p4e | ${failure}`);
for (const failure of script.interactivityPuzzleFillRunJobFailures(fill)) console.log(`run-job | ${failure}`);
for (const failure of script.interactivityPuzzleFillTraceFailures(fill)) console.log(`trace | ${failure}`);
const started = performance.now();
const sources = script.interactivityToolRunPolicySources(repoRoot, script.INTERACTIVITY_TOOL_RUN_REQUIREMENTS);
console.log(`sources=${sources.length} walk=${Math.round(performance.now() - started)}ms`);
const rows = script.INTERACTIVITY_TOOL_RUN_REQUIREMENTS;
for (const [name, findings] of [
  ["amend", script.interactivityToolRunAmendFailures(sources, rows)],
  ["local-lifecycle", script.interactivityToolRunLocalLifecycleFailures(sources, rows)],
  ["legacy-trace", script.interactivityToolRunLegacyTraceFailures(sources)],
  ["declaration", script.interactivityToolRunDeclarationFailures(sources, rows)],
  ["reserved-action", script.interactivityToolRunReservedActionFailures(sources)],
] as const) {
  console.log(`== ${name}: ${findings.length}`);
  for (const finding of findings) console.log(`${finding.file}:${finding.line} ${finding.text}`);
}
console.log(`total=${Math.round(performance.now() - started)}ms`);
