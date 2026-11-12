# Verify wave 3 (verify6-)

## BLOCKER
`/home` is 100% full (451G/465G, 0 avail; `~/.cache/semio/cas` 254G, `semio/.cache` 62G). `repo` fails with `ENOSPC ... mkdir ~/.cache/semio/host-capacity/lock`. All remaining runs need a cache/CAS prune (coordinator decision; I did not delete shared caches).

## Findings and fixes (all unverified by a green test run)
- Workspace layout: the 8 plugin crates (peer, scrub-host, document-media, command, host-configuration, retained-command, extension, runtime-ops) are excluded from the framework workspace; they build from the ROOT workspace (`/home/deck/semio`), not `🧰️framework`. value-allocation-return, drift A/B crates are in framework. Allocator batch D therefore needs two runs (framework: value-allocation-return; root: the 8 plugin crates).
- A: `check --tests` of all drift-A + drift-B + value-allocation-return crates (framework, verify6-check3.log) was clean except one error, fixed afterwards.
- A diagnostic E0432: lib root is mounted as `component`, so the tests import is now `super::super::fault_wire_cursor_tests::controlled_step` (message/tests).
- A rig `encoder-rig.rs:60`: another agent changed `NativeEncodeControl::new_recorded_source` back to `OriginalSourceTurnWitness`, so the pack agent's `identity` edit broke; restored `turn.witness()` (matches current API).
- A RetainedDiffSequence: added real `preflight_work_demand` and `close_work_demand` quotes (per-phase, from the owning sub-owners; `🪢️sequence/🦀️.rs`); the test's SequenceApply now forwards them instead of the ceiling/`1`.
- A OwnedFieldProjection count: restored through a test-only surface: feature `testkit` on `semio-framework-value-snapshot-projection` (`snapshot_projection::testkit::take_field_advances`, thread-local counter in `ProjectionField::advance_into`); `semio-framework-replication` dev-dep enables it; sequence test asserts 0 advances for single-edit sequences.
- A wire-core: `test` source admission failed deterministically ("exceeds file count ... 📐️.json, 0 files remain", 33 files > 32; check did not enforce it). Folded the `🎟️work` test + fixture into `retained/🧪️tests/🦀️.rs` and `🛑️byte.json` (key `workProbe`); removed the `work_tests` mount and the two files. Orphan `retained/🎟️work/🔣️.json` is unreferenced (left).
- D value-allocation-return: `original/🧪️tests/🦀️.rs` was stale vs effect-threaded API (11 errors); migrated to `EffectBank` helpers (`turned`, `admitted`, `duplicate`), using the `📐️.json` law totals as work quotes. Compiled clean in check3.
- B: `metadata --locked` rc=0 (framework). drift-b crates compiled in check3 (incl. io-sqlite-snapshot-tests, ui-host-tests). Their `test` run not done.

## Not run (disk full)
- Framework `test --no-fail-fast` over `verify6-fw-pkgs.txt` (A+B+value-allocation-return): last attempts hit admission (fixed) and then ENOSPC.
- Root-workspace test of the 8 plugin crates; C remodel check/test (workspace `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling`, `-p ...mutations -p ...mutations-leaves -p ...remodeling`, then `test -p ...mutations --lib retained_params`); block-3d `retirement_law`.
Logs: `🗑️generated/verify6-*`.

## Verification 3 (verify7-)
Disk had 34 GB free this time. Coordinator BUILD PAUSE arrived after step 1 was queued; nothing new was queued afterwards (steps 2-5 NOT RUN). Source analysis only after that.

### Step 1: framework `test --no-fail-fast` over verify6-fw-pkgs.txt (58 packages) — RED, run killed
Log `🗑️generated/verify7-fw-test.log`: `cargo:test exceeded its 3592867 ms time limit`, rc=124. The binaries ran in package order and reached only the first ~11 packages before the limit. Results seen:
- ok: io-binary-source (4), manifest-mutation-inputs (10), math-rank (5).
- FAILED semio-framework-async-tests: 79 pass / 11 fail. Causes: (a) test input `🤖️generated/⏳️async/🟦️.ts` missing (generated TS projection not present; `exports_typescript_bindings`); (b) `original_cancel_admission_tests` x3 and `worker_parking` x2 unwrap Err from native-birth admission ("shared allocation birth is not funded" / "native original allocation requires its quoted actual effect"); (c) `OrderedMap` Drop assert "ordered-map root must be explicitly retired before drop" (🌱️value/🗂️ordered/🦀️.rs:105) fires in tests that panic/unwrap earlier or never retire.
- FAILED semio-framework-io-sqlite-snapshot-tests: 23 pass / 41 fail. 27 of the unwraps are `ValueError UnsupportedOwner "forwarded native allocation requires an actual quoted effect"` raised by `NativeDecodeConstruction::charge` (🌱️value/🛬️decode/🦀️.rs:179) when the allocation binding is `Forwarded`; plus "SQLite backing requires the original receiving allocation port". The migrated tests construct decoders through a Forwarded binding that the production path refuses, so they need to be driven through an effect-owning (quoted) binding like the value-allocation-return tests (`EffectBank`).
- FAILED semio-framework-manifest-view-model: 11/11 (resolved-host-context tests: ordered-map retire assert + "original Work verdict must be inspected").
- FAILED semio-framework-mesh-component-references: 1/1 (`component_reference_table_...retires_exact_backings`, tests/🦀️.rs:18, assertion left==right).
- FAILED/aborted semio-framework-diagnostic: `controlled_diagnostic_all_owner_corpus_and_exact_admission` panics "diagnostic retirement respects independent grants" (🎛️controlled/🦀️.rs:102) and `owner.terminal_is_empty()`.
- FAILED semio-framework-io-control-testing: panic in destructor -> non-unwinding abort ("semantic_and_observer_panics_resume_the_same_payload_only_after_original_owner_closure").
- FAILED semio-framework-pack-intrinsic: `retained_intrinsic_body_neutral_vectors_...` panics at 📜️record-body-cursor:595 then aborts.
- Packages after pack-record-preflight in the list never ran (hit the 1 h limit; the hang/abort cases above and queue time ate it).
Net: step 1 red; the failures are genuine test-vs-API drift (tests not driven through quoted effects, forwarded decode binding, missing generated TS), not allocator or disk problems. Fixing needs several focused migration passes per crate; I did not edit them blind during the pause.

### Steps 2-5
NOT RUN (build pause). Static facts from earlier reports remain: heap-assertion crates link only the testing allocator (`semio-framework-value-testing-allocator`, retirement-law twins via test-typed-input-law); driftb2 script `scratchpad/driftb2-test.sh` shows that io-sqlite-snapshot-tests currently FAILS (41 tests), so step 3 will not pass until the sqlite migration above is done. Remodel split check and block-3d `retirement_law` still unproven.

### Resume plan (in order, one cargo run at a time)
1. Re-express sqlite-snapshot tests (Forwarded -> quoted-effect binding), view-model, component-references, diagnostic controlled, io-control-testing, pack-intrinsic tests; generate async TS projection (or make test read it via the schema owner).
2. Re-run step 1 in small batches (<=10 packages) with per-batch timeouts, not one 58-package run.
3. Then steps 2, 3 (metadata --locked), 4, 5.
