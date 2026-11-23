# Layout TypeScript tests (opus1122, ts2)

Runner: bun 1.4.2 direct, one file per invocation (paths must be `./`-prefixed or bun treats them as filters).
Scope: 61 files (`*.test.ts`, `🧪️tests/🟦️.ts`) under `✏️s/🔌️plugins/📏️layout/**`. No separate layout playground test files exist outside this set.

Result: 61/61 files pass after one fix; 1 initial failure.

## Failure and fix
- File: `…/✏️editor/⚙️engine/📤️export/🧪️tests/🟦️.ts` test "Layout editor delegates exact mutable original slots to its owned paid export definition".
- Cause: `✏️editor/🎥️media-export-job-and-factories/🦀️.rs` had two `impl LayoutMediaExportJob` blocks; the source-adoption inspector requires exactly one physical owner of `admit_original`.
- Fix: merged the second `impl LayoutMediaExportJob {` block (holding `step_turn`) into the first (removed the closing brace and reopening header). No test change. Now 22 pass, 0 fail.

Logs: `🗑️generated/opus1122/ts2-*`.
