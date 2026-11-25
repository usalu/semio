# 🧾 Open Items Audit, Norm Plugin, 2026-11-25

Read-only audit. Inputs: the 56 `*-2026-11-22..25.md` reports of this ticket, including `norm-final-verification-2026-11-22.md`. No builds, no git writes, no edits outside this file.

Status key: **still-open** = report or current source confirms it is open. **resolved** = a later report or a quick source check shows it closed. **unclear** = not decidable from the reports or a quick read, so it needs a run.

Family rows show the last state a report verified. "not reported" means no report gives that family a result for that test kind. Aggregate results are noted under the table.

## 1. Last verified state per family

| Family | Engine / compliance tests | Examples tests | Mutations tests | Scene tests | Editor lib tests | Editor integration tests | Play route |
| --- | --- | --- | --- | --- | --- | --- | --- |
| din4108 | not reported (aggregate only) | 4/4 pass (2026-11-23) | not reported | not reported | not reported | not reported | pass 2026-11-22 (23871 ms). 2026-11-24 05:00 probe exit 1 only on `[DEBUG] ui install` console errors, a line since removed from framework source; rerun not recorded |
| din16798 | root 52/0, mutations crate 10/0 (2026-11-24) | 3/3 pass (2026-11-24) | crate 10/0 with `test-support`; python mutation oracle 113 handlers, 0 failures (shim run) | not reported | `history_edit_inputs_resolve` pass (2026-11-23) | not reported | pass 2026-11-22 (23623 ms). 2026-11-24 05:00 same DEBUG-only exit 1 as din4108 |
| din18599 | not reported; `din18599-model` compile was broken 2026-11-23, later state not recorded | 15/15 pass (2026-11-23) | not reported | not reported | not run (editor not started; model crate did not compile) | not run | pass in sweep 6, 2026-11-24 01:00 (chain 6 round 3 clean 2026-11-23 19:09) |
| en1990 | inferences 6/0, io 5/0, io-text 2/0 (2026-11-24); model 0 tests | 14/14 pass (2026-11-22) | wire twins 42/0; committed-diff law passes after 2026-11-24 afternoon fix; `replay_preparation_physical` target still does not compile | lib 74/0 (2026-11-24) | 32/0, `latency_bench` ignored (2026-11-24) | 48 pass / 3 fail (undo, history, conflict; framework) | pass 2026-11-22 (24886 ms). 2026-11-24 viewport probe passes after probe fix (42 s); `norm-eurocode` guest not rebuilt after the Rust changes |
| en1991 | not reported separately | 3/3 pass (2026-11-23, after asset regeneration) | 304 diff fixtures regenerated; schema crate run killed by memory warden (2026-11-24) | not reported | `history_edit_inputs_resolve` pass (2026-11-23) | not reported | pass 2026-11-22 (25106 ms). Sweep 6 2026-11-24 fail on framework scalar close; 07:30 chain 9 refusal gone, only `[DEBUG] ui install` prints |
| en1992 | not reported separately | 7/7 pass (2026-11-22) | lib 16/0 (bare); integration `committed_vectors_hold` failed on a missing fixture (2026-11-24); that fixture path is present now, rerun not recorded | not reported | partial: 29-min run passed ordered-fold, 8 live tests; failed ownership and click-on-check-row; 4 laws unfinished | not run to completion | pass, clean, 2026-11-24 01:30 |
| en1993 | field-meta choices 286/0 (2026-11-24 fix, old labels gone); schema test not run | 4/4 pass (2026-11-23 10:00) | not run (test-support forwarding gaps; split in progress) | not run: crate over source boundary, 8 of 32 files, 396338 B | not run: crate over source boundary, 33 of 32 files (2026-11-23); 36 `.rs` files in editor dir now | not run | works 2026-11-24 01:30 with other agents' `[DEBUG]` prints; not re-probed after removal |
| en1994 | not reported | 3/3 pass (2026-11-23 13:10) | not reported | not reported | not reported; editor asset tests not run | not reported | works 2026-11-24 01:30 with `[DEBUG] rel-total` prints; not re-probed after removal |
| en1995 | root 17/0 (2026-11-24, sqlite); bridges crate 4/0 | 11/11 (2026-11-22 first pass; no rerun recorded) | `mutations-bridges` 4/0; mutations lib not rerun after split | not reported | `history_edit_inputs_resolve` pass (2026-11-23) | not reported | pass 2026-11-22 (29754 ms). Chain 11 probe failed on `[DEBUG] ladder` prints (en1995, 1996, 1999); 01:30 works |
| en1996 | not reported | 2/2 pass (2026-11-22) | not reported | not reported | `history_edit_inputs_resolve` pass (2026-11-23) | not reported | pass 2026-11-22 (30081 ms); sweep 6 pass; chain 11 strict probe failed on `[DEBUG] ladder` prints |
| en1997 | checks lib 24/0 (2026-11-24); schema family run 97 pass / 26 fail (2026-11-24 sweep, interim) | 3/3 pass (2026-11-22) | not reported | 60/60 pass (2026-11-24 06:30, incl. 3 python oracle tests with `SEMIO_PYTHON` exported by hand) | 30/0, 1 ignored (2026-11-24) | 48 pass / 3 fail (undo, history, conflict; framework) | pass 2026-11-22 (24945 ms). 2026-11-24 07:30 refusal gone, only `[DEBUG] ui install` prints |
| en1998 | not reported; schema twins regenerated (2026-11-22) | 10/10 pass (2026-11-23 15:20) | not reported; mutation crates compile after feature fixes (2026-11-24) | 74 pass / 2 fail (2026-11-24 round 2): python oracle (no interpreter then) and drag budget under load | 26 pass / 5 fail (2026-11-24 round 2; all five empty-collection `bridges` gap) | 47 pass / 5 fail (2026-11-24 round 1); round 2 rerun with `RUST_MIN_STACK` not recorded | pass 2026-11-22 (27467 ms); 2026-11-24 07:30 refusal gone, DEBUG-only |
| en1999 | checks_compliance 106/0; package 122/0 (2026-11-24) | 4/4 pass (2026-11-23/24) | mutations lib 12/0 (2026-11-23); committed-diff law passes after 220 regenerated diffs (2026-11-24 afternoon) | 86/0 (2026-11-24 09:00) | 30/0, 1 ignored (2026-11-24 load-grip report) | 38 pass / 2 fail (2026-11-23 evening; history, conflict) | pass 2026-11-22 (30604 ms, older purlin). Chain 11 probe failed on `[DEBUG] ladder` prints; redesigned purlin not probed since |
| iso16757 | not reported separately (compliance gate part of semio-hub-norm 15/15, 2026-11-22) | 4/4 pass (2026-11-23) | `definitions` test-support fixed (2026-11-22); not run | not reported | not reported | not reported | pass 2026-11-22 (25595 ms); sweep 6 pass 2026-11-24 |
| vdi3805 | not reported | 31/31 pass across 4 crates (2026-11-23) | not reported | not reported | not reported | not reported | pass 2026-11-22 (25547 ms); sweep 6 pass 2026-11-24 |

Aggregates, last state: `norm-js:test` 12058 pass, 0 fail (2026-11-24, din16798 report). `norm-plugin:test` 15 of 15 pass (2026-11-24 09:30, first green official run). `semio-hub-norm` 15 of 15 pass (2026-11-22 race report). `test-wire-twins` 11886 pass, 0 fail (2026-11-22 wire-twins report). `verify-norm-document-contract` pass (2026-11-22).

## 2. Open items register

Each row gives the report, a short quote, a status, and the check or evidence behind it.

### closure-inventory-torn-write-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "`closureLeafPresentV1` (closure-repair) is existence-only" | resolved | closure-repair report section 1 fixed it (not grepped) |
| "Inference optional leaves ... still use unsynced writes" | resolved | closure-repair report section 2 made them durable (not grepped) |

### closure-repair-and-storage-sqlite-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Remaining storage failures (separate cause, not fixed here)" | resolved | graph-maintenance-format-owner report fixed the format owner; storage 24/0 later |

### describe-fuel-norm-eurocode-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Not measured ... the new fuel needs a component rebuild" | unclear | `norm-eurocode` republished 2026-11-23 00:51; fuel not re-measured |
| "the git index (staged) ... holds my sampler ... archivist should re-stage" | unclear | not checked (needs read-only `git` run, not done) |

### descriptor-emitter-prerequisite-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "No real rebuild through the new path was run" | still-open | no later report |
| "first live run: any component build after the emitter sources change" | still-open | no later report |

### din16798-content-tests-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "NEEDS A HUMAN DOUBLE-KEY against EN 16798-1 Annex B." | still-open | `ndp.din16798-1.occupancy.*` rows still say "UNVERIFIED" in `⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json` |
| "I could not run the repo's BDD runner or pytest" | still-open | no later run |
| "The other families' bare builds are not run." | still-open | no later run |
| "committed vector file ... change-bar-layer-lap-share ... No such file or directory" | resolved | fixture present at `⭕️change-bar-layer-lap-share/⛔range/🔣️.before.json`; rerun not recorded |

### din18599-route-refusal-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Status: root cause not yet isolated; no fix made by me" | resolved | chain 6 round 3 clean (2026-11-23 19:09); sweep 6 passes din18599 (2026-11-24 01:00) |

### editor-history-framework-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Find what makes one edit in `history_edits_end_to_end` exceed 262144 bytes" | still-open | `admit_paged(..., 262144 ...)` still at `📤️edit/🦀️.rs:68` |
| "Not verified: en1997 and en1998 (not run), and the full editor suite after the owner catalog change" | still-open | no later run for en1998 history |
| "(x, y) != (3200, 0)" allocation ledger | resolved | en1997 report 2026-11-24 says counter gone |

### en1990-editor-en1993-fieldmeta-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "`unit_tests::undo_redo_round_trips_through_the_wrapper`: `WorkLimit ... stopped after 4098 turns`" | still-open | en1997 r5 shows the same undo failure |
| "`unit_tests::context::history_edits_end_to_end`: `original report requires its installed semantic and retirement issuers`" | still-open | en1997 and en1999 show the same framework failure |
| "`unit_tests::context::conflict_history_edits_end_to_end`: `the archive load did not settle`" | still-open | en1990 integration 48/3 (2026-11-24) |
| "`latency_bench` was never run (needs a release build)" | still-open | no release run recorded |
| "The `norm-eurocode` guest was not rebuilt after these Rust changes" | still-open | later rebuild at 07:30 predates the en1990 twins/viewport changes |
| "Other families with empty collections in their default document ... only en1990 was changed." | still-open | en1998 editor lib still reports the empty `bridges` gap |

### en1990-twins-viewport-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Rebuild of the `norm-eurocode` guest before the next publication" | still-open | no rebuild after these Rust changes recorded |
| "`replay_preparation_physical` still does not compile (20 errors ...)" | still-open | target moved to its own `[[test]]`, still unverified |
| "Rust vector law ... cannot confirm the rewritten bundles until the three duplicate-`G` `before.json`" | unclear | 2026-11-24 afternoon report says en1990 passes the committed-diff law; duplicates not rechecked |
| "The remaining failure is en1993 field-meta choices." | resolved | grep finds no old en1993 labels |

### en1991-subjects-test-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "`norm-eurocode` must be rebuilt" | resolved | chain 9 rebuilt and published `norm-eurocode` at 2026-11-24 07:30 |

### en1995-source-boundary-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Not re-run after the `pub use` change: `-p semio-s-artifact-norm-en1995-mutations`" | unclear | not reported since |

### en1995-sqlite-snapshot-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "en1998 and en1999 sqlite tests were not re-run" | still-open | en1999 TS twin 9/0 reported later; en1998 never reported |

### en1997-editor-test-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Remaining in the en1997 content: mutation schema labels ... live acceptance" | resolved | schema labels audit 0 findings; remedy law 40 to 0; `LAYER_SITE` present; editor lib 30/0 |
| "JSON parse work quote was not admitted" | resolved | run 8: no occurrence |
| "history driver close" | still-open | en1997 integration 48/3 still lists history failures |
| "`undo_redo_round_trips_through_the_wrapper`, `history_edits_end_to_end`, `conflict_history_edits_end_to_end`" | still-open | integration r5 3 failures (framework) |
| "Python oracle tests that wait for the Python environment sync" | unclear | environment provisioned 2026-11-24; oracle tests passed 06:30 with `SEMIO_PYTHON` exported by hand |
| "enum options without label in en1990 (`newImportanceClass` 4, `newGroup` 7)" and the other families listed in that section | resolved | schema-labels final audit (2026-11-24 08:50) reports 0 findings |
| "`"oedometer".into()` ... `From<&str>` is not implemented" | resolved | string no longer present in en1997 tree |
| "Latency bench: runs now, does not finish within 22 minutes" | resolved | `release_only` arm added |

### en1997-scene-incremental-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Three scene items are still open" | resolved | scene 60/60 (2026-11-24 06:30) |
| "en1993: the scene crate is over its source boundary at 396338 B" | still-open | load-grip report: 8 of 32 files, 396338 B; not rerun |
| "Both are being fixed, see `norm-scene-crates-2026-11-24.md`." | unclear | referenced report is not in this ticket folder |

### en1998-editor-grips-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Editor `live_acceptance` x2 ... `insert_edit` inserts `value: None`" | still-open | round 2: 5 lib failures, same empty-collection `bridges` gap |
| "din18599 editor: not run." | still-open | no report since |
| "en1992 editor binary was killed by the coordinator after 2 h 49 min" and lib failures "ownership" and "click_on_check_row" | still-open | last direct run failed both |
| "Whether `every_bound_diagram_marker_commits_...` is a real hang is not decided" | still-open | no run recorded |
| "`renders_the_family_headline` aborted with a stack overflow" | unclear | `RUST_MIN_STACK` set; rerun log not recorded |
| "`scene_matches_the_numpy_oracle_for_spectra_forces_and_modes` (no Python interpreter)" | unclear | python environment provisioned 2026-11-24; not rerun |
| "a drag budget ... p95 1402 ms at a load average of 11; 169 / 257 ms on a quiet minute" | still-open | drag budget (240 ms) is load-sensitive; quiet runs pass |
| "Golden verification ... Not run against the final golden yet" | resolved | `golden-verify-en1998.py`: 876 entities, 0 problems |

### en1998-example-decode-2026-11-22.md and en1998-examples-tests-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "NOT verified: two `repo test semio-s-artifact-norm-en1998-examples` runs failed" | resolved | en1998 examples 10/10 (2026-11-23 15:20) |
| "showcase_asset_decodes ... NOT yet compiled or run" | resolved | 10 passed (2026-11-23) |
| "Other EN/DIN family example tests were not run" | resolved | examples verification 2026-11-23 |

### en1999-compliant-purlin-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "editor crate ... could not be built" | resolved | `every_pack_and_json_twin_equals_the_dsl_document_of_its_example` passes (en1999 mutations report) |

### en1999-mutations-compile-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "`mutation_vectors::committed_vectors_hold`: every one of the 219 bundles ... Needs a look" | resolved | 220 diffs regenerated; committed-diff law passes (2026-11-24 afternoon) |
| "`history_edits_end_to_end`, `conflict_history_edits_end_to_end`" | still-open | editor integration 38/2 |
| "`latency_bench` effectively hangs in a debug build" | still-open | ignored in debug (`release_only`); no release run recorded |

### en1999-package-scene-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Making the example a verified pass is a data task: double-key the rows" | still-open | no `.ver()` in `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table/en1999/🦀️.rs` |
| "The scene drag budget ... is load-sensitive" | still-open | passed 5 of 6 runs |
| "The TS twin ... zero-omitted section fields ... Not changed here." | resolved | test-serde report removed the skip attributes |

### eurocode-boot-authority-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Both still exit 1 only because of an unrelated concurrent edit: `World3dHost` ... `CatalogueDropGhost`" | unclear | `catalogue-drop` module exports no `CatalogueDropGhost`; probe not rerun |
| "The other four components keep their builds" | resolved | rebuilt 2026-11-22 to 2026-11-24 |

### graph-maintenance-final, format-owner, remaining (2026-11-22)

| Quote | Status | Evidence |
| --- | --- | --- |
| "closure-maintenance flakiness ... on this machine it stays flaky" | still-open | closure maintenance 22 pass / 4 fail (host pressure) per last run |
| "Ledger test: distinct graph datasets ... skipped on this host" | still-open | skip needs about 14 GiB free host |
| "budget tests: 17 pass, 2 fail (unrelated)" | still-open | not investigated; not part of the script |
| "The suite still has unrelated failures ... not analysed" | unclear | not rerun since |

### handoff-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Delete `🗑️generated/`, keep the scripts and reports, and close the ticket." | still-open | `🗑️generated` still present; ticket open |
| "closure-maintenance scored 21–25 of 26 per run" | still-open | same as graph-maintenance |
| "Background chains still running ... may stop when the old session ends" | unclear | not checked |

### mutation-vectors-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Status: in progress. Sections marked TODO are not yet verified." | still-open | TODO: other families, TS oracle, wire twins, final test lines |

### norm-end-to-end-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "The other four components still carry their 2026-11-21 builds" | resolved | rebuilt 2026-11-22 to 2026-11-24 |

### norm-example-test-mounting-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Not run: en1994 editor asset tests, compliance-report tests and the `.ts` example tests" | still-open | no later run |
| "Fix the en1999 purlin example ... Remove every `[DEBUG]` dump test" | resolved | verdict assertions in place; no `[DEBUG]` in family examples |

### norm-example-verdict-audit-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "EN 1992 office frame DSL 2026-11-22 22:42, pack 2026-11-20 01:18" | still-open | mtimes confirmed: pack Nov 20 01:18, DSL Nov 22 22:42 |
| "EN 1999 purlin DSL 2026-11-23 02:43, pack 2026-11-20 01:08" | resolved | en1999 assets regenerated 2026-11-23 (not timestamp-checked) |

### norm-example-verification-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "en1997 editor: The `integration` target does not compile" | resolved | compiles; integration 48/51 later |
| "vdi3805 failing examples ... being fixed" | resolved | 31/31 (2026-11-23 10:43) |

### norm-example-schema-audit-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "No decoder, cargo or Play test was run" | still-open | static checker only |

### norm-final-verification-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "The core still contains 5 `[DEBUG]` strings from other agents' active diagnostics" | still-open | framework scope; 32 framework files still contain `[DEBUG]` (not all checked) |
| literal last line `RESULTS_PENDING` | still-open | placeholder left in the report |
| "Rebuilds needed before the final sweep: `norm-eurocode` ... `norm-eurocode-structural`" | still-open | no record of rebuilding after the 2026-11-24 afternoon edits |
| "The `interpreter` file ... holds a temporary sampler in the git index" | unclear | not checked |
| "`norm-plugin:test` ... 14 of 15 pass" (earlier state) | resolved | 15 of 15 (2026-11-24 09:30) |

### norm-js-suites-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "en1998 mutate adapters ... still list 288/294 kinds" | still-open | outside `norm-js:test`, not updated |

### norm-plugin-rust-tests-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Remaining gap: the repo captures a child's stderr as the last 64 KiB" | still-open | not checked in `📥️capture/🟦️.ts` |
| "That test was not run: `@semio-tech/cargo-workspaces` has no `test` target." | unclear | not rerun |
| "Norm's app-surface must be ported to `WindowLayoutNode::Window`" | resolved | 0 norm sources reference the old window-layout names |

### norm-rust-test-race-and-budget-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Browser rebuild: rebuild of `norm` already pending" | unclear | rebuild recorded 2026-11-24 05:00; not confirmed against the 2026-11-22 change |

### norm-scene-surface-capacity-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "The probe has to be rerun once that line is removed to get exit 0." | unclear | `[DEBUG] ui install` absent from framework now; probe rerun not recorded |

### norm-tests-2026-11-22.md (Norm plugin test targets)

| Quote | Status | Evidence |
| --- | --- | --- |
| "Inventory record content differs from its name" (recurring infrastructure failure) | unclear | framework inventory module; not checked |
| all targets "not run" | resolved | superseded by 2026-11-24 green runs |

### norm-wire-twins-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "a browser bundle that embeds the norm EN 1998 twin should be rebuilt" | unclear | not checked |

### playwright-cache-pressure-2026-11-22.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Test outcome: Not verified." | resolved | `repo:test-storage-ledger-oracle-processes` passes 7/7 (norm-end-to-end report) |

### python-env-provisioning-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Make the inventory settle wait of the graph read cap its total time" | still-open | listed as follow-up |
| "Not tested: the real `uv sync` through `NativeDependenciesScript`." | still-open | exercised once by hand only |
| "its result is not recorded here" (oracle-test2) | unclear | log not kept |

### scalar-close-source-work-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Symptom 1: `scalar close changed Source or Work`" | still-open | line 150 of `🌳️source/🦀️.rs` still carries the refusal |
| "en1997 editor integration ... not re-run after the move" | still-open | integration still 3 failures |
| "No unit test was added" | still-open | note, not done |

### schema-crates-sweep-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Status: interim; the sweep is incomplete" | still-open | din4108, din16798, din18599, iso16757, en1991, en1992, en1994, en1996, en1998, en1999 have no result |
| "en1997 ... 97 passed; 26 failed" | still-open | interim family run |
| "`schema-crates-retry.sh` ... `RETRYDONE`" | unclear | `RETRYDONE` status not found |

### schema-labels-all-families-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "The en1992 and din18599 schemas are audit-clean; rerun ... once those crates compile" | still-open | 2026-11-24 08:50: "en1992 and din18599 are not tested yet" |

### store-replay-receipt-regression-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Remaining: `scalar close changed Source or Work` ... not covered" | still-open | source line 150 |
| "`semio-os-store-operation-wire` test code does not compile" | unclear | not rerun |
| "`semio-framework-pack-json` is over its source boundary (268140 of 262144 bytes)" | unclear | not rerun |
| "en1997 editor integration ... `history_edits_end_to_end` ... still fail" | still-open | see en1997 integration |

### test-serde-schema-required-2026-11-24.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "Rerun en1995, en1997, en1993 after their blockers clear" | still-open | en1995 root 17/0 later; en1997 schema 26 failures; en1993 schema not rerun |
| "Other families' serde snapshot tests ... can adopt `assert_serde_snapshot_law`" | still-open | optional follow-up |

### vdi3805-source-boundary-2026-11-23.md

| Quote | Status | Evidence |
| --- | --- | --- |
| "blatt-5-fail ... now drop the mandatory identity record (family 100)" | unclear | DSL header still reads `record-count=0`; record 100 removal not confirmed by grep |

### schema-crates / other single items

| Quote | Status | Evidence |
| --- | --- | --- |
| "The `🗑️generated` folder is therefore not deleted yet." (schema-crates-sweep) | still-open | folder present |

## 3. `[DEBUG]` strings in the norm sources

Scope: `✏️s/🔌️plugins/📕️norm` and `🌎️hub/🧩️compositions/📕️norm`, grep for `DEBUG`, excluding `target`, `node_modules`, `.cache` and `🗑️generated`.

| File | Line | Text |
| --- | --- | --- |
| `🗿️artifacts/🏛️en1992/.../✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs` | 27 | `eprintln!("[DEBUG] doc {n}: evaluate {:.0} ms", ...)` |
| same file | 33 | `eprintln!("[DEBUG] doc {n}: unit {:?} {ms:.0} ms", ...)` |
| same file | 37 | `panic!("[DEBUG] timing only");` |

No `[DEBUG]` string is in `🌎️hub/🧩️compositions/📕️norm`. The en1992 timing test is the only one left in the norm sources. The panic at line 37 means the test always fails.

Outside norm scope: 32 files under `🧰️framework` contain `[DEBUG]`. The five strings named in `norm-final-verification-2026-11-22.md` (FEM cleanup failure, `poll_wit` authority refusal, natural-file worker, retained close, unresolved input receipt) were not individually checked.

## 4. Still-open list (summary)

1. en1992 editor incremental unit test has three `[DEBUG]` lines and an unconditional `panic!("[DEBUG] timing only")` at line 37.
2. en1992 editor lib: ownership and click-on-check-row failures; four laws never finished; din18599 editor never run.
3. History and undo framework failures in the en1990, en1997 and en1999 editor integration suites; 262144-byte operation payload limit at `📤️edit/🦀️.rs:68` unexplained.
4. en1998 editor lib: 5 failures from the empty-collection `bridges` default-document gap.
5. en1993 scene and editor crates over the source boundary; no en1993 scene or editor test has run.
6. Scalar close refusal `scalar close changed Source or Work` still in framework source (`🌳️source/🦀️.rs:150`).
7. Rebuild of `norm-eurocode`, `norm-eurocode-materials`, `norm` and `norm-eurocode-structural` after the 2026-11-24 afternoon edits; Play strict probes not rerun for the 15 routes since the DEBUG-only results.
8. en1999 purlin and all `ndp.en1999.*` rows still unverified; din16798 occupancy densities still UNVERIFIED (human double-key needed).
9. en1992 office frame compliant pack (2026-11-20) older than its DSL (2026-11-22).
10. Release-mode `latency_bench` never run (en1990, en1999); en1998 and en1999 sqlite oracle tests partly not rerun.
11. Schema-family coverage incomplete: en1997 schema 26 failures; din4108, din16798, din18599, iso16757, en1991, en1992, en1994, en1996, en1998, en1999 have no schema result; en1992 and din18599 labels not tested.
12. en1990 `replay_preparation_physical` target does not compile.
13. Storage closure maintenance flaky on this host (21 to 25 of 26); ledger distinct-graph test skipped on this host; budget tests 17/2 uninvestigated.
14. `norm-scene-crates-2026-11-24.md`, referenced by the en1997 scene report, is not in the ticket folder.
15. `RESULTS_PENDING` placeholder at the end of `norm-final-verification-2026-11-22.md`.
16. Handoff step 5 not done: `🗑️generated` still present, ticket not closed.
17. Mutation-vectors report still lists TODOs for other families, the TS oracle and wire twins.
18. en1998 mutate adapters still list 288 of 294 kinds; en1994 and en1995 editor asset and compliance-report tests not run.
19. en1997 grip convention: still relative; the absolute `rebased` option from the load-grip report not applied.
20. Descriptor emitter prerequisite: first live component build through the new path not run.
21. Python environment: the real `uv sync` path verified only by hand; inventory-settle cap in the graph read still a follow-up.
22. Scene drag budgets (en1998, en1999) pass only on a quiet host.

## 5. Method notes

- All 56 dated reports read in full (`norm-final-verification-2026-11-22.md` included, ending with the `RESULTS_PENDING` line).
- Quick source checks run with read-only `grep`, `find` and `stat`. Checks that found a result are cited in the register. Anything not checked is marked unclear.
- No builds, tests or git commands that modify state were run.
