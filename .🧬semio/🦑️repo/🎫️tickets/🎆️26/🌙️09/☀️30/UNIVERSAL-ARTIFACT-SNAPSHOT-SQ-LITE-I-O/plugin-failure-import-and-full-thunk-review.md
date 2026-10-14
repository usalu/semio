# Plugin Failure Import And Full Thunk Review

Read-only current source, 2026-10-13. No builds or edits.

## Actual E0432 Frontier

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs:5` imports OriginalTurnFailure from semio_framework_job. Current repository Rust search finds no definition at all; only stale uses in this same module. `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/🦀️.rs` already pub-use exports component root, so this is not a missing package reexport to fix.

Canonical current job/actor APIs use `semio_framework_value::ValueError`: Plugin actual jobs ABI cancel/step (`⚛️reactor/💼️jobs/🦀️.rs:370,373`) return it, StepContext::consume_retained returns it and preserves actual child receipt. Legitimate migration is call module failure types/generic Into bounds/retained fields/ControlledRetirement fields to canonical ValueError, not compatibility alias or resurrection of missing old wrapper. Original value accessor at line65 currently dereferences `.primary`; ValueError has no primary field, so accessor should borrow actual retained ValueError directly.

Two stale `OriginalTurnFailure::debit_original` calls lines48–49 require real progress semantics: close_original_slot can return an actual ValueError carrying newly spent retained progress. Debit that original child receipt through StepContext's consume_retained/fuel policy before propagating the exact original cause. Do not merely drop the call or attach measured authority. If debit itself fails, preserve actual causes in original owner custody rather than introducing an old-type wrapper. Parent should implement/test this migration in the real module; no source mutation here.

## Landed Full Thunks

Current registered constructor adds Some specialized export_file/import_file, identical_to compares optional address/presence, and guest identity rejects host full pointers. Import thunk downcasts actual Option<P>, requires empty, prepays original Port item/copy/depth transitions, calls shared synchronous typed import, then only infallibly moves P and diagnostics. No Any box/Arc/boxed Future in that producer. This matches wrapper-free destination plan.

One ordering detail: import_snapshot_file_into checks destination then pays transfers **before** calling `import_sqlite_snapshot_receiving::<P>`, where typed_snapshot_codec verifies exact P SQL schema/TypeId. Global callback lookup checks native/provider TypeId but not P::SQLITE_SCHEMA. Thus a provider with matching TypeId but altered schema causes grant/work consumption before exact schema rejection. If contract requires wrong-provider schema refusal before all effects, call borrowed typed_snapshot_codec::<P>(dialect) before prepayment. Current early Option mismatch/Some refusal is already before payment. No rejected P exists in this schema-failure case, but policy receipts are affected. No runtime behavior claimed.
