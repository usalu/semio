# Lane lang6 (10-27 23:5x): the language-law findings no batch owns -> 0

FOR coordinator: ONE batch `C/lang6-tools/staged/rest/` (8 files, 7 sources + 1 test), `base` on live at 23:59, file-disjoint from nolegacy `lang`/`nobaseline`, touches neither `🔏️closure/` nor `🔏️authorizers/`.
- apply AFTER nolegacy `lease` -> `nobaseline` -> `lang` (or the langbudget re-cut): `python3 -I C/lang6-tools/staged/rest/stage.py apply --root /Users/ueli/Documents/semio`; revert: same with `revert`.
- hot files (store, nx graph, host-capacity, scan are edited by peers every ~30 min): right before landing run `python3 -I C/lang6-tools/build.py` (re-reads the live bytes, one anchored unique literal per edit, exits non-zero when an anchor moved; re-records base/new hashes).
- verify: `sh C/lang6-tools/staged/rest/verify.sh <root> [before]` (stage check, the language law restricted to the batch paths = 0 findings, 11 touched-module test files, sorted pass/fail lines to `FAILURE_OUT`). Whole repo: `CP_ROOT=<root> bun C/compliance-tools/cp-lawfull.ts out.json` must say 0.

## Findings and fixes (live 23:5x: 28 findings; nolegacy lang+nobaseline take 21 away; 7 remain, not 6: `test-store-isolation` of the landed isolate fence is the 7th)
| file (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`) | before | fix | after |
|---|---|---|---|
| `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store` | 66 / allowed 64 | the 2 sites added by `withRepositoryCacheStoreOwnerFrameV1` -> tokens `exact-issued-cache-owner-required`, `original-cache-owner-frame-required` | 64 |
| `…/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan` | 4 / 3 | `exact-optional-graph-target-bytes-required` | 3 |
| `⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph` | 141 / 140 | the site added with the owner-frame wrapper (`openNxGraphChild…`) -> `actual-nx-partition-owner-required` | 140 |
| `🏗️build/📏️incremental/🏎️bench/🛂️acceptance/⌨️command` | 3 / 0 | the module's own `both(english, german)` pair: table refusal and the two `Path exceeds` refusals (German: Geschlossene Abnahmetabelle erforderlich / Pfad überschreitet N Zeichen) | 0 |
| `🏗️build/📏️incremental/🏎️bench/🛂️acceptance` | 1 / 0 | nonce refusal as `both(...)` | 0 |
| `🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity` | 1 / 0 | queue waiter refusal -> `bazel-action-left-before-start` | 0 |
| `⚖️laws/🧫️test-store-isolation` | 1 / 0 | `test-store-isolation-rules-invalid:<problems>`; its test's `toThrow` follows | 0 |
Choices: the three allowlisted files lose exactly their excess (the ratchet stays equal: no allowlist row edited, so no clash with `lang`'s allowlist rewrite and no `allowance-stale`); the two sibling `Actual nx partition owner required` sites that already sit in the allowance are left (converting them needs allowlist edits owned by `lang`). Internal invariants become machine tokens (kebab-case, no prose), the user-visible bench acceptance command keeps the module's existing EN+DE `both` pattern (English first). No test pinned any of the 7 old texts (git/rg over the repo; only the isolation test, converted).

## Laws (mirror `/private/tmp/m/lang6`, 20 chars, cloned from live 23:3x; dropped at the end)
- Base: whole-repo law 28 findings; + nolegacy `nobaseline` + `lang` -> 7 (list above; the nobaseline row for `🧬️dsl/✨️derive/🗂️schema-document-index.json` was missing in the mirror, copied from live, hash = base) ; + `rest` -> **0 findings** (2881 sources). `🗣️language-catalogs/🧪️tests/🗣️corpus` (TS law + independent Python checker): 6 pass / 0 fail.
- Parse (`gate-g-tools/scan.ts`): 8 files, bad 0. `compliance-tools/verify_batch.py` on the batch: 4 findings, all pre-existing peer code, none from the edits: scan file docstring emoji `🪢` twice (lines 157/198), the nx graph protocol line that PARSES another tool's `[DEBUG]` output (known, documented by compliance).
- Tests (11 files, same bun invocation before/after, base = batch reverted in the same mirror): the sorted pass/fail lines are IDENTICAL (`🗑️generated/lang6/[DEBUG] tests-base.txt` = `tests-after.txt`): 75 pass / 6 fail both ways. The 6 reds are pre-existing and unrelated: "no test file of the repository reaches the checkout's live store" (finds `nx/graph/🧪️tests/⏱️deadline` L20-25: isolate lane), the nightly-workflow acceptance test, 4 in the closure `scan` suite (neutral loaded observation / managed Graph cold entry / required cache consumer scope / unnamed). `test-store-isolation` 3/1, acceptance 22/1, host-capacity 4+3+3+5 all green apart from those.
- Stage law `C/xplat5-tools/tests/test_stage_mode.py` OK (the stage.py is the mode-keeping quota template).

## Notes
- In-flight peer batches that rewrite the same files (benchci `acceptance`, cachegc `quota`, isolate `fence`, nxdeadline `release`, obsopen `nestedfix`, pinpub `nxsparse`, quiet16 `hold`) are `applied` on live for these paths today; once `rest` lands their `revert` would report drift on these 8 files (expected, same as every follow-up edit).
- Tools: `C/lang6-tools/{build.py,lawpaths.ts,sites.ts,peers.ts}`; logs `C/🗑️generated/lang6/[DEBUG] *`.
