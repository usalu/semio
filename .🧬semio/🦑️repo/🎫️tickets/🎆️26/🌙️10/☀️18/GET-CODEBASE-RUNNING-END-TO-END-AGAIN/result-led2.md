# Result led2 (ledger items O-05, O-07, O-01, O-10)

Executor `led2`, 2026-10-18 02:00 to 03:10, foreground, no sub-agent, no modifying git command, no build. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `C` = this ticket folder. Ledger: `F/COMPILE-DEFECT-LEDGER.md`, new rows `led2-01` to `led2-04`, the four section B rows carry a `led2: CLOSED` cell, section C has a `led2` line.

## Verdict

All four items are closed. Three of the four were already largely done by bd9 to bd10 (the audit worked from stale section B rows); the work was to verify against the live tree, fix what was really left (process fixtures, the process law, the pack pins) and correct the ledger.

## O-05 close-budget and frontier anchors: closed, no edit needed

* `advance_original_document_archive_close` was dead (only test callers) and was already deleted by bd10-08 together with `original_document_archive_close_demands`, `original_document_archive_close_slot`, their Rust test, TS test and the `🗃️archive-turn` fixture. Re-checked: `git -c core.fsmonitor=false grep` over all tracked files (excluding `.🧬semio`) has 0 hits for the three names; a recursive grep over `🧰️framework` has 0 hits.
* The 43 "absent" assertions no longer exist as drift: the suite was re-anchored by bd10-07 and passes against current code. Names the audit listed as removed: `JobOutcomeSlot`, `close_step_outcome_slot`, `step_outcome_slot_retirement_demands` are alive in `🧵️job/♻️retirement/📄️payload/🦀️.rs` (job-owned tests); the plugin-family assertions on them are anchored on that live code.
* Run (bun test, see Verification): close-budget 30 pass / 1 skip (env-gated native child-root receipts) / 0 fail, 482 expect calls; `🧵️retained-command/🪟️mounted/♻️frontier` 7/7; `♻️cancellation/📦️lease` 4/4; mounted group `📦️owner` 1/1; mounted group 1/1; `⏳️completion/🎟️receiving` 1/1; `🔬️app-child-member-registry` 3/3; `🛂️manifest/🔖️action-args/♻️retirement` 2/2.
* Not run: `🧩️composition/📨️emission/🌱️genesis` (needs `web-tree-sitter` at the repo version; bd10 recorded it green).

## O-07 window fixtures and three readers: closed, verified (nothing to author)

* The architect fixture `…/📋️register/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/` and the equation fixture `…/🕸️graph/🎚️config/🧪️tests/🧫️fixtures/🔬️window-config-ownership/🔣️.json` exist, sit next to the config schema (`🧬️schema/🔣️.json`) and mutation schema, and are consumed schema-first: strict Ajv plus independent `fast-json-patch` oracle, and the Rust twins read the same JSON via `runtime_fixture_text`. Architect run prints `ajv=valid json-patch=valid four-owner-isolation=valid reload=valid report-identity=valid`; `testEquationGraphWindowConfigOracle()` prints OK.
* The three readers (completion receiving, manifest action-args retirement, group owner) read live paths and pass (counts above).

## O-01 pack producer-authority cause fixture: closed

Findings (all re-derived, not copied from the ledger):

* The "unmounted draft" premise is stale. `semio-framework-pack-error` (`🎒️pack/⚠️error/📦️packages/🦀️rust`, lib path `../../🦀️.rs`) is a member of `🧰️framework/Cargo.toml` and a dependency of 7 production pack crates (`source`, `io`, `http`, `value`, `value/encode`, `value/decode`, `value/schema`, `async`). The TS `PackError` (`⚠️error/🟦️.ts`) already has `refusalKind`, `causeKind`, `retry`, `fromPagedRefusal`, `fromPagedAllocation`. `🧭️producer-authority/🟦️.ts` passes 4/4 (228 expect calls) with Ajv, system zlib and SQLite oracles.
* The pins are 50 (not 30) `source.{path,sha256,predicate}` objects over 10 files in `🧭️cause/🔣️.json`. Nothing reads them: neither the TS test nor the Rust twin `🎒️pack/🧪️tests/🧭️producer-authority/🦀️.rs` touches `source`, and a `git grep` for `source.sha256`, `.predicate`, `sourcePredicate` finds no reader anywhere. The only hashing rule on record is raw-byte SHA-256 of `source.path` (CENSUS4-PKG-35 section 6). Under it none of the 10 current files matches its pin, and none of the 5 committed blobs of `⚠️error/🦀️.rs` matches the pin either (so the pins were not even from a committed state).
* Decision: no re-pin (it would forge an attestation that nothing checks and that per-predicate review of 50 claims over carved files could not honestly renew), and the FIXTURES-ARE-TESTING-EXAMPLES-ONLY rule says fixtures carry plain examples, no source-registration inventory. Removed the `source` object from all 50 rows with a text-level edit; verified by JSON parse that every other field of every row is unchanged and that no `sha256`/`"source":` remains.
* `⚠️error/🦀️.rs` line 1 said "Unmounted canonical Pack producer-authority draft"; replaced by an accurate owner line (same line count, so no ratchet change).

## O-10 fixture placement (value, pack, actor, process, ui): closed statically

Rule source: `📓️canonical-fixture-architecture-2026-10-06.md` of ticket 26/05/30 `FIXTURES-ARE-TESTING-EXAMPLES-ONLY` (fixtures are plain examples under the owner's `🧪️tests`; no executable owner), the five `fixture-placement` law files, and `fixtureFacadeTargets` in `🧪️test/📖️source-text/🟦️.ts` (a `🧫️fixtures/…/🟦️.ts` outside `🧪️tests` is admitted only as a pure `export … from` facade whose targets sit under `🧪️tests`).

I wrote a faithful replica of all five laws (`C/led2-tools/placement.ts`, bun, no dependencies: ancestry, facade clause, inventory, `include_str!`/`include_bytes!` resolution, process import resolution). Result before my edits: value 0, pack 0, actor 0 (peers had already moved e.g. `🧵️job/⏱️context/🧪️testing/🎟️caller/🧫️fixtures` at 00:05), ui 0, process 6. After: 0 everywhere (83 / 143 / 67 / 180 `include_*` targets resolve).

The 6 process findings:

* 4 misplaced data files, moved under their own module's `🧪️tests` (plain `mv`, no git):
  * `🏃️process/🧪️testing/🦀️cargo/🎯️exact/🛠️preparation/🧫️fixtures/🔣️.json` -> `…/🛠️preparation/🧪️tests/🧫️fixtures/🔣️.json`
  * `…/🦀️cargo/🎟️admission/🧫️fixtures/🔣️.json` -> `…/🎟️admission/🧪️tests/🧫️fixtures/🔣️.json`
  * `…/🦀️cargo/🏃️execution/🧫️fixtures/🔣️.json` -> `…/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json`
  * `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🏗️construction/🧫️fixtures/🔣️.json` -> `…/🏗️construction/🧪️tests/🧫️fixtures/🔣️.json`
  * Consumers re-pointed from `../🧫️fixtures/…` to `./🧫️fixtures/…`: the four `🧪️tests/🟦️.ts` files and `construction/🧪️tests/🦀️.rs` (`include_str!("🧫️fixtures/🔣️.json")`). Repo-wide `git grep` found no other reference to these paths.
* 2 public SDK facades (`🏃️process/📦️packages/🟦️typescript/🧪️testing/🧫️fixtures/🟦️.ts`, hub `📦️packages/…/🧪️testing/🧫️fixtures/📊️observability-v1/🟦️.ts`): pure re-exports into `🧪️tests`. The process law was the only one of the five without the facade clause. Added `placedFacade` plus the `fixtureFacadeTargets` import to `🏃️process/🧪️tests/🧭️fixture-placement/🟦️.ts`, same shape as the actor law.

## Files changed

* Moved: the 4 fixture JSON files above (old directories removed).
* Edited: `…/🎟️admission/🧪️tests/🟦️.ts`, `…/🏃️execution/🧪️tests/🟦️.ts`, `…/🛠️preparation/🧪️tests/🟦️.ts`, `…/🏗️construction/🧪️tests/🟦️.ts`, `…/🏗️construction/🧪️tests/🦀️.rs`, `🧰️framework/🔨️modules/🏃️process/🧪️tests/🧭️fixture-placement/🟦️.ts`, `🧰️framework/🔨️modules/🎒️pack/⚠️error/🧪️tests/🧫️fixtures/🧭️cause/🔣️.json`, `🧰️framework/🔨️modules/🎒️pack/⚠️error/🦀️.rs` (line 1), `F/COMPILE-DEFECT-LEDGER.md`.
* Created: this report, `C/led2-tools/placement.ts` (law replica), `C/led2-tools/mkpaths.ts` (builds a tsconfig `paths` map from the tracked `@semio-tech/*` package exports so bun tests run without `node_modules`).
* All scratch output (`🗑️generated/led2-*`, the copied ajv packages, test artifact dir) deleted. No `[DEBUG]` text added.

## Verification (exact)

`node_modules` is a dangling symlink in this tree (the deps executor is repairing it), so bun tests ran with `bun test --tsconfig-override=<tsconfig from mkpaths.ts> --timeout 300000 ./🟦️.ts` from each test directory, `SEMIO_REPO_ROOT` and `SEMIO_TEST_ARTIFACT_DIR` set, with ajv 8.20, fast-json-patch 3.1.1 copied from `~/.bun/install/cache` into a scratch directory (since removed). Results are the ones quoted above (all pass). Also run: `construction/🧪️tests` 1/1 after the move. `bun C/led2-tools/placement.ts` -> all zero (last run after all edits).

## Still open / not verified

* The five fixture-placement law files themselves were not run through bun (they need `fast-glob`, `minimatch`, `typescript`, not installed); only the replica ran. Re-run them once `node_modules` is restored: `bun test` on the five `fixture-placement` files.
* Moved-fixture consumers `🎟️admission/🧪️tests` and `🏃️execution/🧪️tests` load the fixture but fail on behaviour (`admission.control` undefined, `refused` mismatch). Cause is peer work in progress on the cargo admission API (`🏃️process/🧪️testing/🦀️cargo/🧪️tests/🗄️admission/🟦️.ts` is staged new, many `🏃️process` files staged `M`); not related to the move and not touched. `🛠️preparation/🧪️tests` could not run (`typescript` missing).
* Rust side of the construction test (`include_str!` path) and the pack-error crate header edit are uncompiled (no admitted compile).
* Tree is still touched by peers (process module staged edits); no git command was used.
