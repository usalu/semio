# L7 wiring hygiene

Changes (all under `✏️s/🔌️plugins/🧩️puzzle/`):

1. `🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` nativeSources: store-close glob now `.../🧰️owned/🧪️tests/🧫️fixtures/♻️store-close/**/*` (the corpus json moved there; the old `🧪️tests/♻️store-close/🟦️.ts` and `🦀️.rs` were deleted in commit 1f0dfd57f85).
   - FINDING, not fixed: targets `test-store-close-source` / `test-store-close-native` (and `StoreCloseScript` in `🗿️artifacts/◻️2d/📦️packages/🦀️rust/📜️script.ts`) still point at the deleted test file and the nonexistent rust law `standalone_history_close_resumes_past_the_original_ledger_page`. They will fail; the fixture json has no consumer. Needs a decision: delete the targets/class/fixture, or restore the tests. (My scripted removal was denied by the permission classifier, so I stopped at the glob.)
2. Same file: added target `test-browser-time-travel` (copy of 3d `test-browser-lifecycle` shape, scenario `🧪️tests/⏪️time-travel/🟦️.ts`, react, en,de) so time-travel has an owner. Live/non-cached like the lifecycle targets.
3. 2d and 3d rust `📋️project.json` nativeSources: added `🧪️tests/📏️inline-import/**/*`; consumers are the rust `include_str!` in 2d `🕹️gumball-region-set-verb-and-snapshot-laws/🦀️.rs` and 3d `📥️snapshot-exchange-laws/🦀️.rs`.
4. `🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json` default inputs: added `🧪️tests/🧪️compose5d-preparetopologymodel/**/*`. It already runs via in-source vitest in `🎯️targets/⚛️5d-react/🟦️.tsx` (included by the config `include: ["../../🟦️.tsx"]`), only the cache input was missing.
5. storybook-2d / storybook-spatial: already owned by the root `.storybook` browser runner (`.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts` imports and `🌐️browser-discovery` fixture lists them). No change; storybook-index is owned by the puzzle vitest config.

Verification: all four edited project.json files parse (python json) and referenced paths exist. `repo show project` not run (graph service skipped); compose5d vitest not run.
