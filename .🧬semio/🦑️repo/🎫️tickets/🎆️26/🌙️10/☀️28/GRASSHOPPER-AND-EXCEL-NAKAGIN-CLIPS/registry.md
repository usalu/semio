# Registry Report (REGISTRY)

Both series are wired like the BIM series. Scripts (re-runnable, idempotent) are in `T/registry/`: `series.py` (ids, scenes, folders, chapter and output names), `entries.py` (entry and playlist scripts), `check_scenes.py` (import gate), `apply.py` (manifest, titles, presentation, portable cases, series registry cases; reads each scene's `TITLE`), `write-transcripts.py` (transcripts from each scene's `NARRATION`). Re-run `write-transcripts.py` and `apply.py` after any scene or title change.

## Done

- Entries `A/🦗️grasshopper/<topic>/🎬️clip/🐍️.py` (5) and `A/📊️excel/<topic>/🎬️clip/🐍️.py` (4), playlists `A/🦗️grasshopper/🎬️clip/🐍️.py` and `A/📊️excel/🎬️clip/🐍️.py`.
- Chapter scene names: `Grasshopper_01_Anatomy` .. `Grasshopper_05_Trees`, `Excel_01_Anatomy` .. `Excel_04_Energy`. Outputs `Grasshopper_anatomy` .. `Excel_energy` (like `BIM_information`).
- Deviation from the brief order: chapter titles, presentation topic entries and the manifest were registered together in step 2 (after all nine scenes imported), not in step 1, because `collection.enrich` raises at runtime if the chapter map, the presentation topics and the manifest disagree. Nothing shared pointed at a missing script at any time. Only the `symbol` enum (`grasshopper`, `excel` added to `🪟️presentation/🧬️schema.json`), `📜️script.ts` and the new entries/playlists were done before.
- Manifest `A/⚡️energy/🎬️clip/🔣️.json`: nine entries inserted after `bim_consistency` and before `optimization_*` (the hierarchy orders chapters by the presentation topic order energy, parametric, bim, grasshopper, excel, optimization; appending at the end failed the boundary tests).
- `🪟️presentation/📚️collection/🔣️.json`: nine chapter titles. `🪟️presentation/🔣️.json`: topics `grasshopper` and `excel` filled (icon, symbol = topic id, cells 3 per row). Titles are taken from each scene's `TITLE`.
- Portable cases: `🧪️tests/🔣️.cases.json` (curriculum ids, chapter and segment counts, 134 render segments, 47 logical clips, first-segment ordinals, unique sources 126), `🪟️presentation/🧪️tests/🔣️.cases.json`, `🥞️hierarchy/🧪️tests/🔣️.cases.json`, `🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json` (counts: 26 clips, 47 chapters, 126 canonical sources, 134 segment occurrences; nine clip entries; 48 boundaries).
- New series registry tests (the brief's `test_curriculum_registers_*`, entry routing, playlist routing, titles equal in scene/collection/presentation, narration bilingual with unique keys, transcripts mention every video): `A/🦗️grasshopper/🎬️clip/🧪️tests/📇️registry/🐍️.py` and `A/📊️excel/🎬️clip/🧪️tests/📇️registry/🐍️.py` with generated `🔣️.cases.json` beside each. They sit in their own module so the engine agents' `🧪️tests/🐍️.py` stay untouched.
- `📜️script.ts`: python test list gets `🏢️nakagin.🧪️tests`, the G and X `🧪️tests`, `🧪️tests.🎭️presentation` and `🧪️tests.📇️registry`; the `.cjs` loop covers `🏗️bim`, `🎯️optimization`, `🦗️grasshopper`, `📊️excel` plus `🏢️nakagin/🧪️tests/🟨️.cjs`. Trap: this file has corrupted CR runs in HEAD (`\r\r\r\r\r\r\n`); the Edit tool rewrites them, so line endings were restored with `T/registry/eolnorm.py`; the diff is 3 lines. Use a byte-level edit for it.
- READMEs (en + de, structure like BIM, primary references from the research files) for both series; `A/README.md` rows for Grasshopper, Excel, Nakagin and the missing Optimization row. Transcripts `🗣️transcript.{en,de}.md` per series (G 110 clauses, X 90 clauses at last run).
- Taxonomy: not changed. The BIM and optimization precedent registers no `🎬️clip` kind (only `teaching-energy-clip` exists); the precedent is followed. If wanted: add `teaching-grasshopper-clip` and `teaching-excel-clip` modelled on `teaching-energy-clip`.
- `.gitignore`: nothing (runtime writes traces and speech to the cache).

## Tests (Windows, `SEMIO_REPO_BINARY` shim, `TMP`/`TEMP` and `SEMIO_TEACHING_OUTPUT_ROOT` inside the ticket `🗑️generated`; the temp dir must be inside the workspace or the runtime tests fail with `relative_to`)

| Module | Result |
| --- | --- |
| `⚡️energy.🏗️authoring.🎬️clip.🧪️tests` (curriculum contract, discovery with `SEMIO_TEST_MANIM=1`) | 31 tests OK (1 skipped) |
| `🦗️grasshopper...🧪️tests.📇️registry` + `📊️excel...🧪️tests.📇️registry` | 10 tests OK |
| `🪟️presentation.🧪️tests`, `🥞️hierarchy.🧪️tests`, `🥞️hierarchy.🧪️tests.🧩️chapters` | 35 tests, 33 OK; all hierarchy, boundary, collection and presentation-order tests pass; 2 errors in the browser-capture tests for `PhysicalFundamentals_FullSection` (`Boundary capture cancelled`; `repo tools exec` is not supported by the shim): environment, unrelated to the new series |
| `🏢️nakagin`, G `🧪️tests`, G and X `🎭️presentation`, X `🧪️tests` | owned by other agents; the run was still executing a long G test at report time (log `T/🗑️generated/registry/series.log`) |

## Open

- The `.cjs` tests of G, X and nakagin run in `script.ts test` only through the require loop; they were not executed here (`bun`/AJV through `repo`).
- Titles come from the scenes; if a scene agent changes a `TITLE`, re-run `apply.py` (updates the collection map, presentation topics, cases) and `write-transcripts.py`.

## Refresh after final scenes (23:06)

`apply.py` and `write-transcripts.py` re-run with all nine scenes (G 112 clauses, X 92). README rows for grasshopper_capsule/tower/casting/trees and excel_references/program/energy rewritten from the scene reports by `registry/readme_rows.py`. Registry tests: 10 OK. Titles unchanged. Both Excel reports (program 22:26, energy 22:18) were already final, so no second pass is pending; re-run the three scripts if any scene changes again.

## Refresh 2026-10-29

Re-ran apply, transcripts (G 115, X 95 clauses). Titles unchanged; README rows still match the scene reports. Registry tests plus energy contract and hierarchy modules: 49 OK; portable order, coverage and boundary tests: 3 OK. excel_program scene file last changed 05:36; re-run `registry/write-transcripts.py` after its pending update.
