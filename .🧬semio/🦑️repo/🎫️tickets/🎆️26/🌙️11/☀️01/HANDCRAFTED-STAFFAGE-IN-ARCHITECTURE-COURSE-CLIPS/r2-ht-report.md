# R2 test sweep (agent ht)

Scope: every Python test module under `🎓️teaching/🏛️architecture/⚡️energy` (35 modules: `🧪️tests` and `🧪️architecture` folders, incl. the authoring package and `🧍️staffage/🧪️tests`), run one at a time through `T/m9_tests.sh` with the dotted module path. Plus staffage library vitest and React vitest with the ticket-local alias configs. Runner: `T/🧰️ht/run_all.sh`, `T/🧰️ht/vitest_run.sh`. Module list: `T/🧰️ht/modules.txt`.

## Totals

- Python: 35 modules. 28 pass, 3 fail (4 failures/errors in total), 4 collect no tests (exit 5). 259 tests ran.
- Staffage library vitest (`🧍️staffage/📦️packages/🟦️typescript`): 51 of 51 files pass, 1018 tests pass, 11 skipped (1029). Note: the q2 report recorded 954 tests; the count has grown since.
- React vitest (`🎯️targets/⚛️react/📦️packages/🟦️typescript`): 16 of 17 files pass, 106 pass, 1 skipped, 1 failed (108).

## Per-module results (Python)

| # | Module (under `⚡️energy`) | rc | Ran | Result |
|---|---|---|---|---|
| 1 | `❄️cooling/🎬️clip/👥️internal-gains/🧪️tests/🐍️` | 0 | 3 | OK |
| 2 | `❄️cooling/🎬️clip/🧪️tests/🐍️` | 0 | 4 | OK |
| 3 | `🌬️ventilation/🎬️clip/📦️volume/🧪️tests/🐍️` | 0 | 2 | OK |
| 4 | `🌬️ventilation/🎬️clip/🧪️architecture/🐍️` | 0 | 5 | OK |
| 5 | `🏗️authoring/🎨️visuals/🧍️staffage/🧪️tests/🐍️` | 0 | 24 | OK |
| 6 | `🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️` | 0 | 9 | OK |
| 7 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️` | 0 | 7 | OK |
| 8 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️icons` | 0 | 8 | OK |
| 9 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration` | 0 | 9 | OK (586 s) |
| 10 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️scenes` | 0 | 11 | OK |
| 11 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️section_consumers` | 0 | 9 | OK |
| 12 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️sections` | 0 | 9 | OK |
| 13 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only` | 0 | 5 | OK |
| 14 | `🏗️authoring/🎨️visuals/🧪️tests/🐍️transforms` | 0 | 2 | OK |
| 15 | `🏗️authoring/🎨️visuals/🧪️tests/📟️readout/🐍️` | 0 | 5 | OK |
| 16 | `🏗️authoring/🎬️clip/👥️audience/🧪️tests/🐍️` | 1 | 7 | FAILED (errors=3) |
| 17 | `🏗️authoring/🎬️clip/📁️output/🧪️tests/🐍️` | 1 | 9 | FAILED (errors=1) |
| 18 | `🏗️authoring/🎬️clip/🧪️tests/🐍️` | 0 | 31 | OK (skipped=1) |
| 19 | `🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🐍️` | 0 | 9 | OK (671 s) |
| 20 | `…/🌐️browser/🧪️tests/💾️persisted/🐍️` | 0 | 4 | OK |
| 21 | `…/🌐️browser/🧪️tests/📚️closure/🐍️` | 0 | 1 | OK |
| 22 | `🏗️authoring/🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️` | 0 | 8 | OK |
| 23 | `🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️` | 0 | 21 | OK (773 s) |
| 24 | `…/🥞️hierarchy/🧪️tests/🐍️` | 0 | 8 | OK |
| 25 | `…/🥞️hierarchy/🧪️tests/🧩️chapters/🐍️` | 0 | 12 | OK (849 s) |
| 26 | `…/🧩️chapters/🐍️pointer` | 0 | 1 | OK |
| 27 | `…/🧩️chapters/🐍️process` | 5 | 0 | no tests (helper module, 0 TestCase) |
| 28 | `🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🐍️` | 0 | 15 | OK (700 s) |
| 29 | `…/🧪️tests/🖼️stream/🐍️` | 0 | 3 | OK |
| 30 | `🏗️authoring/🔤️fonts/🧪️tests/🐍️` | 0 | 13 | OK |
| 31 | `🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🐍️` | 1 | 3 | FAILED (failures=2) |
| 32 | `📊️demand/🎬️clip/⚖️regulations/🧪️tests/☀️sun/🐍️` | 0 | 2 | OK |
| 33 | `📊️demand/🎬️clip/🧪️practice/🏗️composition/🐍️` | 5 | 0 | no tests (0 TestCase) |
| 34 | `📊️demand/🎬️clip/🧪️practice/🐍️` | 5 | 0 | no tests (0 TestCase) |
| 35 | `📊️demand/🎬️clip/🧪️practice/🔊️speech/🐍️` | 5 | 0 | no tests (0 TestCase) |

Exit 5 with `Ran 0 tests` means unittest found nothing to run. Those four modules hold no test cases, so they are not failures. Modules 33 to 35 are clip-practice modules, not test modules, so they should not be on the test list. Dropping them from the runner is a coordinator call.

## Failures

### 16 `🎬️clip/👥️audience` (3 errors)
Traceback head (all three share one root):
```
File ".../⚙️runtime/🐍️.py", line 492, in render_segment
    narration = source_narration(segment, language=settings["language"])
KeyError: 'language'
```
Also `⚙️runtime/🐍️.py:427` in `segment_state` (`settings["language"]`), raised from test line 93.
Assessment: not staffage. The clip runtime now reads `settings["language"]` directly, but the audience test settings dict has no `language` key. Either the settings contract changed and the test fixture is stale, or the runtime should default. Owner: clip runtime / audience test fixtures.

### 17 `🎬️clip/📁️output` (1 error)
```
File ".../📁️output/🧪️tests/🐍️.py", line 35, in test_generated_narration_is_read_and_invalidates_render_cache
TypeError: source_narration() missing 1 required keyword-only argument: 'language'
```
Assessment: not staffage. The test calls `source_narration` without `language`, which the runtime now requires. Same contract mismatch as module 16.

### 31 `🔤️fonts/🧪️tests/📑️heading` (2 failures)
```
AssertionError: Lists differ: [(440, '_title')] != []   (script 🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py)
AssertionError: Lists differ: [(643, …), (763, '_title'), (804, '_title')] != []   (script 🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py)
```
Assessment: not the staffage migration itself. These clip scripts still use the removed `_title` heading API (the standalone-heading cleanup). Real leftovers to remove, so they need a dispatched fix. Locations: `📊️excel/🎬️clip/⚡️energy/🐍️.py:440`, `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py:643, 763, 804`.

## Vitest

- Staffage library: `Test Files 51 passed (51)`, `Tests 1018 passed | 11 skipped (1029)`. rc 0.
- React: `Test Files 1 failed | 16 passed (17)`, `Tests 1 failed | 106 passed | 1 skipped (108)`. rc 1.
  - FAIL `🧪️tests/📚️catalog-loading/🟦️.tsx` > "loads the authored archive as one ready row per manifest entry with an empty manifest check". Received three `manifest-mismatch` entries, the first at `entries[137].analytics`: `Entry "plate-heat-exchanger" and its asset disagree on analytics.`
  - Assessment: likely caused by the staffage migration. The catalog entry `plate-heat-exchanger` and its asset analytics disagree after the migration, so the manifest check is failing. Fix belongs to whoever owns the staffage catalog manifest or that asset's analytics.

## Still open
- The four no-test modules (27, 33, 34, 35) are in the list because the module path was given, not because they contain tests. Coordinator should decide whether to drop them from the sweep.
- Run-time tail: integration (586 s), browser (671 s), interaction (773 s), chapters (849 s). Rerun the slow modules only when the fixes land.
- Logs were deleted from `T/🗑️generated/ht/` per the brief. Traceback heads are in this report.
