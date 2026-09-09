/** 🔍️ Runs the puzzle-owned `verify interactivity` self-tests in isolation.
 *
 * The `verify interactivity` CLI aborts at the framework-owned `interactivityLiveReconcileSelfTests`
 * (its `per-surface-credit-cap` mutation string went stale when a peer replaced the literal
 * `SURFACE_RECONCILE_SURFACE_BYTES = 8 * 1_024 * 1_024` with `ui_contract::UI_RESIDENT_SURFACE_BYTES`
 * in `de617a7c17`). That suite runs AFTER every puzzle suite, so a puzzle verdict is still observable —
 * this probe observes it directly instead of inferring it from the aborted run.
 */
import { interactivityPuzzleFillEnvelopeSelfTests } from "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts";
import { interactivityPuzzleFillP4eSelfTests } from "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts";
import { interactivityPuzzleFillPreviewJsonSelfTests } from "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts";

const repoRoot = process.argv[2] ?? process.cwd();
const suites: [string, (root: string) => void][] = [
  ["interactivityPuzzleFillEnvelopeSelfTests", interactivityPuzzleFillEnvelopeSelfTests],
  ["interactivityPuzzleFillP4eSelfTests", interactivityPuzzleFillP4eSelfTests],
  ["interactivityPuzzleFillPreviewJsonSelfTests", interactivityPuzzleFillPreviewJsonSelfTests],
];

let failed = 0;
for (const [name, suite] of suites) {
  try {
    suite(repoRoot);
    console.log(`PASS ${name}`);
  } catch (error) {
    failed += 1;
    console.log(`FAIL ${name}: ${error instanceof Error ? error.message : String(error)}`);
  }
}
process.exit(failed === 0 ? 0 : 1);
