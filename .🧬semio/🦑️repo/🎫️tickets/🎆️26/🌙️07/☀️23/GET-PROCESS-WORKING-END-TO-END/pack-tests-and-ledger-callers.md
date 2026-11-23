# Pack Record-Retained Tests and Ledger Callers

## Pack record-retained: 31 of 31 pass

Root causes (product unless noted):
- Symbol/scalar page admission: quotes carried no copy bytes and too little depth, so the real reservation was denied. `🗃️symbol-table` `source_reserve_demands` now adds `next_capacity_copy_byte_demand`; `🎟️storage` `record_storage_admission_demands` now quotes via `source_step_demands`.
- Record-body retirement stall: quote used depth 1; now copy/depth come from `source_close_demands` (`record_storage_retirement_demands`).
- Symbol storage retirement stall (`🏷️symbols/🎟️storage`): quote used the default body hint while the step used the grant copy bytes as body; both now use `maximum_body_bytes`.
- Source test: fixture grant copy 17 could not fund the 424 B page metadata; fixture and schema const now 1024 (test/fixture).
- Receiving test: retiring owner now needs 6368 copy bytes; fixture `turn.copyBytes` 4096 -> 16384 (test/fixture).
- Test helpers: `lend_observed` (storage tests), `#[track_caller]` on source `receipt`.

## Ledger callers (`close_retirement_recipient*`, `close_committing`)

36 crates reference the close API (list in `🗑️generated/ledger-manifests.txt`). `cargo check --tests` per crate (`ledger-check.sh`):
- Clean (lib and tests): plugin-abi, plugin, plugin-interpreter, native-decoding, io-control-testing, original-snapshot-owners, job-native-decode, flow, store replay-operation, member-open-store, history, and the stdio artifact crates 1,2,3,5,7,8,10.
- Failing, all unrelated to the two ledger changes (stale tests against an earlier API redesign): missing `NativeEncodeWorkIssuer`/`NativeEncodeConstructionIssuer`/`NativeConstructionRecipientLoan`/`NativeDecode*Recipient`, `NativeEncodeControl::close_retirement_recipient` removed, `RetainedWorkOutcome::unwrap_err`, changed arities. Crates: encode tests, io-control(+tests), io-schema, io-sqlite-snapshot, store codec, space-history, presence snapshot, spr archive, plugin host, plugin runtime, space collection/native-caller, stdio txt/pdf/binary-io tests. `(lib)` failures: pdf-page-editor-authoring (missing `refuse`/`cancelled`) and mcp-catalog (`SEMIO_BUILD_MODE` unset).
- No compile error mentions `close_committing`, the decode retirement charge, or ledger types introduced by the previous pass.

Runtime tests of the clean dependents: see the section below.

## Runtime tests of dependents

| Crate | Result |
| --- | --- |
| native-decoding | all pass |
| job-native-decode | 9 passed |
| original-snapshot-owners | no tests (compiles) |
| plugin-abi (`semio-plugin-abi`) | 4 pass, 1 fail: `actor_input_serde_accepts_only_the_closed_tagged_corpus` (serde corpus assertion, no ledger/receipt involvement) |
| plugin-interpreter | 23 pass, 1 fail: `original_structured_control_matches_independent_vm_and_paid_label_retirement` (16 vs 17 at `🧭️control/🧪️tests/🦀️.rs:268`, VM label count; not traced to the ledger changes, unverified) |
| plugin (`semio-plugin`) | does not compile: `semio-plugin-window-kits` lib, 3 arity errors (unrelated) |

Not ported: the 19 crates with stale test targets (see above); porting them to the new issuer/recipient API is a separate task.
