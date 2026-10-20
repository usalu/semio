# Result tst8: bun census re-run (2026-10-20) - INTERIM (stopped by the 12:55 coordinator write pause)

Runner `C/tst8-tools/run.py` (tst7 runner + per-suite finite envelope minted by `envwire.ts`, short in-repo artifact dir `C/🗑️generated/t8/<8>`), 2 concurrent, `./<file>` only. Suite list `list.py` (git ls-files -co + `from "bun:test"`): 2,175 suites (tst7: 2,145; +43 new, -13 gone).

## Pass 1 (plain real install, 11:07-12:29): 1,395 / 2,175 green (tst7 baseline 1,764 / 2,145)
Delta vs tst7: 451 regressions, 69 fixed, 309 still red, 20 new red suites.
Root cause of ~440 of the 451 regressions: **stale install**. `node_modules` is generation 70825028305173c7f252 (05:37); manifest-batch2 + cycle-break created 191 workspace packages that are not linked (`@semio-tech/framework-{hash,transient,filesystem,manifest,trace,effect}`, `ui/*`, `plugin-registry-publication`, plugin `*-js`), while live code already imports them. Counts of first errors: framework-hash 144, framework-transient 126, framework-manifest/schema 70, framework-filesystem/observation/* 31. Needs `lock --all` + `sync --all` (coordinator window); no code fix possible.

## Pass 2 (reds only, `--preload C/tst8-tools/resolve-overlay.ts`): emulates the missing links via `build.module` virtual facades (map `pkgmap.py`); in-process imports only (child `bun` processes still fail, BUN_OPTIONS is ignored). Stopped at 477 / 780; 173 of those rows were invalid (first overlay version emitted `export *` + default, fixed afterwards) and are counted by their pass-1 result. Pending: re-run `🗑️generated/tst8/red1.txt` remainder + `bad2.txt` after the pause.

## Interim totals (pass 1 + valid pass-2 rows): see `🗑️generated/tst8-census.json`
green 1,531 / 2,175; classes of 644 reds: law-drift 242, missing-dependency 193, envelope-gated 168, real-defect 21, cargo/wasm/native 18, browser 2.

## Regressions fixed at the root cause (edited before the pause)
| suite | cause | fix |
| --- | --- | --- |
| `🧪️tests/🧱️checked-in-asset-contracts` | ps1 bootstrap (auto-commit fd14da5) reads the debugger policy from `💻️client\🛠️developer-tools\🧬️schema`; law forbade the whole `💻️client` fragment | ps1 row forbids only the retired client layout, requires the policy path |
| `framework/📦️packages/🟦️typescript/🧪️tests/📤️module-package-surface` + `🔌️sdk-surface` | exports-stage generated JSON facades `import document from "x.json" with {type:"json"}; export default document;` (also ~60 other packages); law accepted only named re-exports | both laws accept exactly that two-statement shape (+ `default` in the loaded-values check); fixtures gain an accept case and a refuse case (default export of a non-document import) |
| `value/🧪️tests/🧭️fixture-placement`, `actor/🧪️tests/🧭️fixture-placement` (and process/pack twins) | `fixtureFacadeTargets` (🧪️test/📖️source-text) rejected the same facade shape | resolver accepts import + `export default <that id>`; unit test cases added |
| `value/🧬️retained-clone/🚚️bulk-vec/🧪️tests` | law sliced the router class up to `PlainElementRetirementSourceScript`, now defined before it | slice ends at the next `\nclass ` |
| `value/♻️retirement/🧪️tests/🧺️heap` | test moved into `🧺️heap/` by the lift; fixture URL stayed relative to old dir | `../🧫️fixtures/🧺️heap.json` |
Not fixed (peer edited <15 min or install-gated): `process/⏱️budget` (policy now requires complete fields; file edited 11:22 by a peer), `script-boundary` (needs framework-manifest link), 3 suites hitting the 256-char lease path under the in-repo artifact dir (harness; re-run with a scratchpad artifact dir, as tst7 did).

## Leftovers noted (not verified after sync)
- "Original process birth already received" (cargo testing, native-build cleanup operation, others): tests call `receiveScriptProcessInvocation` several times; fix = `withScriptProcessEnvelope` per test (law-drift finding 3).
- `puzzle/…/✏️editor/🌉️wasm` law bans the string `ArtifactVcs`, the wasm source docstring mentions `Puzzle3dArtifactVcs`; `process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile` expects `SEMIO_BUILD_PROFILE_INPUTS` in the (now router) caching artifacts script; `process/🧪️testing/🦀️cargo/🧪️tests` expects class `TestScript` in the os rust script; plugin fixture-placement and bim testkit: fixtures moved by fem-fix/fixture-agents, includes not re-pointed.

## Verification
All counts from `python3 C/tst8-tools/{run.py,classify.py,final.py,diff.py}`. My edited suites were in the pass-2 red list but their pass-2 results were not all reached before the pause: NOT yet verified green. No test run after 12:55. Three APFS test images leaked by the killed runner were detached.

## Status at 14:30 and how to resume (after "PAUSE LIFTED")
Paused since 12:55; no test process of mine has run since. At 14:17 `node_modules` moved to generation 81b8b51e9cf4b7e091f6 (already links framework-hash/-transient; sync attempt 2 is still queued behind pid 8903, so this is not the finished sync). Plain pass 1 is therefore stale; re-run it as a whole once the sync is green:
1. `python3 C/tst8-tools/list.py C/🗑️generated/tst8/suites.txt` then `python3 C/chain-tools/detach.py C/🗑️generated/tst8/run3.log python3 C/tst8-tools/run.py C/🗑️generated/tst8/suites.txt C/🗑️generated/tst8/results3.jsonl 60000 300` (no overlay needed once the links exist; ~80 min at load 40-70).
2. `python3 C/tst8-tools/classify.py results3.jsonl && python3 C/tst8-tools/final.py && python3 C/tst8-tools/diff.py results3.jsonl` (diff vs tst7); copy `🗑️generated/tst8/tst8-census.json` to `🗑️generated/tst8-census.json`.
3. Overlay variant (stale install): `TST8_OVERLAY=1 python3 C/tst8-tools/pkgmap.py C/🗑️generated/tst8/pkgmap.json` first; red list `red1.txt`, invalid rows `bad2.txt`.
Files kept: `C/tst8-tools/{run.py,list.py,envwire.ts,classify.py,final.py,diff.py,pkgmap.py,resolve-overlay.ts}`. Generated (coordinator may delete): `C/🗑️generated/tst8/`, `C/🗑️generated/t8/`, `C/🗑️generated/tst8-census.json` (interim).
Repo edits (all test/law/fixture files, no frozen path): `🧪️tests/🧱️checked-in-asset-contracts/🟦️.ts`; `🧰️framework/📦️packages/🟦️typescript/🧪️tests/{📤️module-package-surface,🔌️sdk-surface}/{🟦️.ts,🧫️fixtures/🔣️.json}`; `🧰️framework/🔨️modules/🧪️test/📖️source-text/{🟦️.ts,🧪️tests/🟦️.ts}`; `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🚚️bulk-vec/🧪️tests/🟦️.ts`; `🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🧺️heap/🟦️.ts`.
