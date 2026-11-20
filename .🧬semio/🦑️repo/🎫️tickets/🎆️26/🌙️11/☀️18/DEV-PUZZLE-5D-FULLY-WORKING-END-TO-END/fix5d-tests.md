# Fix 5d lib test failures (fix5d2)

## Root causes
1. `command_envelope_round_trip_holds_for_an_applied_operation` (binary, text): `close_puzzle5d_store` is byte-identical in shape to the 2D twin. The failure was the framework store close (snapshot read lease registry retirement), which another agent fixed concurrently. No 5D change was needed. Run fix5d2-2 passed both tests.
2. `sqlite_snapshot_puzzle5d_all_four_copy_phases_have_real_interior_cancellation`:
   - Test drift. Under the current framework, `ReceivingSettlement::observe` swallows native body progress until `finish_body`. So the `SqliteSnapshotControl` callback never sees interior EncodeNative/DecodeNative progress. Interior cancellation belongs to the caller-owned native control callback. The test attached it to the wrong control. Sibling tests (3d, energy, trinity, architect) still use the pre-`out` signature and are stale, so they are no reference.
   - Real framework bug found after fixing the test: cancelling native decoding mid-body panicked "original construction action retains an unsettled reply". In `native-decoding`, `admit` (and the callers' `?`) dropped an unsettled `NativeSnapshotConstruction` when the body failed or `work.admit` failed.

## Changes
- `🧰️framework/🔨️modules/🧪️test/🛬️original-snapshot-owners/🦀️.rs`: added `with_encoding_observed` / `with_decoding_observed` (caller-owned native progress callback); `with_encoding` / `with_decoding` delegate with an always-true callback.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: EncodeNative and DecodeNative arms cancel through the native callback (interior condition and assertion unchanged); the sqlite control is permissive there.
- `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs`: new `NativeSnapshotDecodeOwner::cancel_constructed`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs`: `admit` cancels its construction when `work.admit` fails; new `spanned` helper admits, runs the body, records the receipt on success and cancels the construction on failure; all three decode spans use it.

## Verification
- `repo cargo --store=shared --configuration=prototype --lang=en -- test -p semio-s-artifact-puzzle-5d --lib -- command_envelope_round_trip interior_cancellation` (log `fix5d2-4.log`): 3 passed, 0 failed, EXIT 0.
- Earlier: fix5d2-1 failed to build in unrelated `semio-plugin-window-config` (concurrent edit, transient); fix5d2-2 showed envelope tests passing and the cancellation panic; fix5d2-3 (RUST_BACKTRACE) located the decode drop.

## Hub `viewer_never_mutates` (coordinator extra evidence)
Root cause of "registry retirement has reached terminal custody" (retirement.rs Deref expect): `PresenceStoreRetirement` dereferenced a funded-closed `SnapshotReadRegistryAliasRetirement` (no registry left) via `reads.has_returned()` in `retirement_demands` (backtrace in `fix5d2-hub-2.log`) and via `SnapshotReadLeaseRegistry::terminal_is_empty(reads)` in `close_step`.

Changes:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🦀️.rs`: inherent `has_returned` (false when closed) and `registry_terminal_is_empty` (true when closed) on the alias, shadowing the Deref'd registry methods.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️.rs`: `close_step` uses `reads.registry_terminal_is_empty()`.

Result (`fix5d2-hub-4.log`, `test -p semio-hub-puzzle --lib viewer_never_mutates`): the Deref panic is gone, but all three tests (2d/3d/5d) still FAIL, now at `semio-plugin-testing-laws .../bounded-viewer-fixture-and-viewer-laws/🦀️.rs:327` `close_registered_fixture_app`: `PluginLifecycleFailure ... "original retained turn completed with retained owners"` (Fault code app.message). This is a separate plugin-lifecycle close problem (a retained owner survives the fixture app's original close turn), not yet investigated. Still BLOCKED on that.
