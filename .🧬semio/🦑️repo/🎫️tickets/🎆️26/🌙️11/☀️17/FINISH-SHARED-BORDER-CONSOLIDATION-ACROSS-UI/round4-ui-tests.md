# Round 4 UI and product test run

All runs native (`node_modules/.bin/vitest run --config T/vitest-<name>.config.ts`), logs in `🗑️generated/r4-*.log`. Run 1 started all 10 configs in parallel; that load caused most timeouts, so failures were re-run in isolation.

## Counts

| Config | Run 1 (10 in parallel) | After fixes / isolated re-run |
|---|---|---|
| ui-all | 12 failed, 1574 passed | 9 failed, 1585 passed (all 9 environment, see below) |
| course | 5 failed, 1116 passed (timeouts) | load-failures 6/6, navigation 53/53, deputy-and-contract 27/27, shared-borders 17/17, task-keyboard + challenge-views + sorting-concordance 42/42 pass alone; `journey-and-tabs` hangs (see Remaining) |
| os-final | 158 passed | 158 passed |
| hub-admin | 1 failed, 27 passed | 28 passed |
| demonstrator | 29 passed | 29 passed |
| teaching (staffage) | 2 failed, 273 passed | 275 passed |
| interpreter | 195 passed | 195 passed |
| gis-panel | 2 passed | 2 passed |
| cad-renderer | 4 passed | 4 passed |
| extra | 1 file failed ("No test suite"), 21 passed | unchanged (aggregator child, environment) |

## Fixes

1. ContextMenu title chip / content cap: expectations still named `--size-medium`; the shared `--chip-row-height` token replaced it. Updated `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🧲️contextmenu-and-more/🟦️.tsx` (2 tests, green).
2. TUI pointer routing: chips of one window now abut at a shared seam column (`x == previous.right`), per the shared-border rule. Test relaxed to `toBeGreaterThanOrEqual` in `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🖱️pointer-routing/🟦️.ts` (green).
3. hub-admin `command-routing`: not a product failure; the test reads `process.cwd()/📜️script.ts`, so it needs cwd = the package dir. Running the ticket config from `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript` gives 28/28. Harness issue only.
4. `answerCurrentTask` (`🧰️framework/🛍️products/📚️course/🧪️tests/🧰️learner-journey-kit/🟦️.tsx`): unbounded `while` replaced by a loop that fails after 10 attempts, so a stuck sort cannot spin forever.

## Round 3 suspects
- `chrome-panel-safe-area` (296 vs 300): passes now (not in any failure list).
- Canvas `shared-border-dock`: passes now.
- Panel dock test in the owned-locale aggregator: passes now; the aggregator's only real failures were the 2 ContextMenu tests above.

## Staffage `catalog-loading`
Run 1 failed with a 120 s timeout (plus a `getContext` jsdom message and one playground-tabs assertion), all while 10 suites ran together. Re-run alone: whole teaching config 275/275 green in 82 s including `catalog-loading` (8 tests) and `playground-tabs`. Root cause is CPU starvation under parallel runs (the test builds the whole authored archive), not a code defect. Run the staffage config alone.

## Remaining
- Course `journey-and-tabs` "goes from the introduction through a submitted run...": reproducible hang (event loop busy in microtasks, so the 60 s test timeout never fires). Instrumented: it gets through the Climates and second task, then spins at the third task (Masses sort) right after `proctor.outage = true`. Not a `CardStack.useCardInterlock` measure loop (checked with a call counter, reverted). Suspects are the in-flight edits to `session/proctor-link`, `proctor`, `presence/socket` and run `locked` handling, all modified today by others. Needs the owner of those edits; not changed here.
- Diagram keyboard (`rings the focused node`) and force (`cursorizes 20,000`): fail only because ui-all's glob also collects the sub-suites directly (`keyboard-tests/🟦️.tsx`, `force-tests`), where the harness mock is imported after `Diagram`. Via the real aggregator `🧪️tests/🧩️component/🟦️.tsx` all 55 pass. Fix would be a narrower include in `vitest-ui-all.config.ts`.
- "No test suite found" / `bun:test` / Playwright-API files (about 170 ui-all files, 4 course kits, 1 extra): helper modules and non-vitest suites collected by the broad glob; environment, not failures.

## Skipped for missing Playwright browsers (not installed, needs approval)
1. `🔨️modules/📏️border-presentation/segments.test.ts` > browser computes incident seam priority on logical axes
2. `🧪️tests/📤️prepared-readback` (Chromium WebGPU readback)
3. `🧪️tests/🔤️text-advances` (Chromium DOM width corpus)
4. `🧱️elements/🌳️Tree/🧪️tests/🔤️text-flow` (3 tests)
5. `🧱️elements/📐️Layout/🧪️tests/🧩️component` > keeps the window canvas full width under every open panel
