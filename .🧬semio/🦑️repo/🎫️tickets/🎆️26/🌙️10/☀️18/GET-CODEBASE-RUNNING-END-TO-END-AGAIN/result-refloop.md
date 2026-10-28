# result-refloop (lane refloop, 10-27): the 2-owner refresh loop of every print-rs:test run

STATUS 09:15: DONE. Cause measured on live generation 616, fix staged and verified on mirror `rlloop` (dropped). Nothing live touched, no official run, no process of mine running.

FOR coordinator: batch ready `C/refloop-tools/staged` (3 edited + 2 new files, flat, house style; all 5 `base` on live at 09:11). Apply `python3 -I C/refloop-tools/staged/stage.py apply /Users/ueli/Documents/semio` · revert `… revert /Users/ueli/Documents/semio` · state `… check <root>` (exit 2 = drift; rebuild from `C/refloop-tools/work/` with `python3 -I C/refloop-tools/mk.py`) · laws `sh C/refloop-tools/staged/verify.sh <root>` (law + stamps corpus, 18 tests, ~5 s), `RED_ON_BASE=1 sh … verify.sh <mirror root>` (mirror only: swaps the 3 base sources in around the law run).
FOR coordinator: verdict = `@semio-tech/print-rs:test` TWICE after landing. Run 1: the 18 declared-import facts carry the old parser revision (the module's bytes changed) → the two owners are re-inferred ONE last time (plus whatever the library edit re-infers). Run 2 (and any later no-edit run): NO `graph: refreshing` line, no `18 stale facts (analysis 18)`; expect ~25-52 s less per print run (g28-print2 80.7 s → ~30-55 s).
FOR memo owner (latent, not a current fault): the graph memo's recheck (`🧠️memo/🚪️command/🟦️.ts:15`) calls `observationsHold` without any analysis preparation, so an `analysis` row ever landing in a recheck set would refuse every lookup. Today's sealed recheck sets (288 rows) hold, so nothing to do now.

## 1. Cause (file:line + evidence)
- The 18 stale facts are all `analysis` rows of provider `build:declared-source-imports:v1`: 17 files under `🧊️wgpu/` + `🔌️plugin/📇️registry/🟦️.ts`, held only by `@semio-tech/framework-renderer-wgpu` and `@semio-tech/plugin-registry`, the two owners with a declared source-input contract (`🧱️project-assembly/🟨️.mjs:295` → `loaded.relativeSourceInputsAsyncV1`).
- That provider registered LAZILY, on the first analysis: `📚️library/🕸️dependencies/🟦️typescript/🟨️.mjs:15-21` (`sourceImportProviderV1`). The graph child checks facts in `holdsByStat` (`🏗️build/🕸️graph/🟦️.ts:371-379`); its preparation `await libraryBootstrap; prepareSourceAnalysesV1()` registers every inference parser (`🧬️source-analyses/🟨️.mjs:177`) but not this one. `sourceAnalysisFactHoldsV1` (`🏗️build/👁️observation/🟨️.mjs:322-325`) returns false for an unregistered provider → printed as `the source analysis of … changed` → refresh → the refresh's inference registers the parser and records the identical fact → publish → the next process starts unregistered again. A loop by construction; the files never mattered.
- Evidence (read-only probes, `C/refloop-tools/`):
  - `providers.ts` on generation 616: provider declared-source-imports 18 rows, 18 stale before the parser registers, 0 stale after it registers in the same process; owners exactly the 2.
  - `verify-row.ts`: replays the graph child's check (import inference, `libraryBootstrap`, `prepareSourceAnalysesV1`, `observationsHold`) on the 17 wgpu rows: `false`.
  - `rows.ts`: the recorded rows (e.g. `🎬️renderer-boot/🟦️.ts`: source mtime 10-16, revision `e01601dc…`, nestedObservations []).
  - Logs: the same 2 owners refreshed in g27-print2, g28-print1, g28p-edit (gen 612), g28p-revert (613), g28-print2 (614), every time `18 stale facts (analysis 18)`.

## 2. Other products (scan of `C/gate-g-tools/run/*.log`, g23-g29, 45 runs)
No other owner is refreshed in consecutive no-edit runs. Dashboard refreshes (g27-dash: repo-dashboard-rs; g28-dash: + filesystem-workspace-rs; g29-dash: 4) and course refreshes (g25-g29) all name files that the preceding gate landing changed; g25-dash/dash2 and every hot run refresh nothing. Structurally: `registerSourceAnalysisV1` has exactly two callers, the inference's `register` (covered by `prepareSourceAnalysesV1` before every check) and this lazy one, so no other provider can loop this way.

## 3. Fix (batch `refloop`)
- `🕸️dependencies/🟦️typescript/🟨️.mjs`: `sourceImportProviderV1` is exported; its revision hashes the parser, schema and compiler bytes with plain reads (no rows recorded into whatever scope happens to be active; the revision itself guards those bytes, and `require.resolve("typescript")` names exactly the compiler `parseRuntimeImports` loads); every module instance of one revision registers the first instance's parser (process-wide map), so a second load of the same bytes (e.g. a captured image and a source path) cannot hit `Ambiguous source analysis revision registration` now that registration is eager.
- `🕸️dependencies/🟦️typescript/🟨️.d.mts`: declares it.
- `🏗️build/🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs`: when the bootstrap loads the source-input module it calls `source.sourceImportProviderV1()`, so `libraryBootstrap` settled ⇒ every parser whose facts inference records is registered (graph child check, builder, deriver alike).
- No stamp bump: the analysis output is unchanged for the same inputs (stamps corpus law 17/17 green).

## 4. Laws (mirror `rlloop`, sealed, guard clean on every run)
New `🕸️dependencies/🟦️typescript/🧪️tests/🔁️verifier/🟦️.ts` + language-agnostic fixture `🧪️tests/🧫️fixtures/🔁️verifier/🔣️.json`: the test records declared-import facts of a 2-file contract, then for each case starts a FRESH process that prepares exactly as the graph child does and checks the rows: unchanged → holds, body edit → holds (replay cutoff works cross-process), import edit → stale; the expected verdict is cross-checked by an independent oracle (TypeScript's own `preProcessFile` import list unchanged ⇔ holds); a plain load analysing after the bootstrap gives the TypeScript import list (no ambiguity); a text guard pins the graph child's preparation expression so a change there forces this law to be revisited.
| | base | batch |
|---|---|---|
| verifier law | RED (`unchanged: expected true, received false` = the loop) | GREEN 1/1, 9 expects |
| `🔖️stamps/🧪️tests` (output corpus vs stamps) | | GREEN 17/17 |
| `tsc --strict` law + d.mts (`refloop-tools/tsconfig.rl.json`) | | 0 errors; sensitivity shown (injected TS2322 caught) |
| stage selftest | | revert → `mirror diff` lists none of the 5 files → apply → second apply no-op |
Not runnable under plain `bun test` (need an issued Script invocation, red on base and batch alike): `🕸️dependencies/🟦️typescript/🧪️tests/🟦️.ts`, `⚙️configured/🧪️tests`; `👁️observation/🧪️tests` needs a non-`/var/folders` root (ancestry check). `📥️inputs/🧪️tests` 3/1 (the cargo sentinel case, unrelated). Not proven by a law: the process-wide parser map (under Bun a `?revision=` import yields the same instance, so the second-instance case cannot be produced without writing a second copy into the source tree).

## 5. Status (newest first)
- 09:15 done; mirror `rlloop` dropped. Tools kept: `C/refloop-tools/{rows,providers,verify-row,prep}.ts`, `mk.py`, `work/`, `tsconfig.rl.json`, `staged/`.
- 09:05 batch staged, mirror green, red on base shown.
- 08:59 cause confirmed (providers.ts, verify-row.ts).
