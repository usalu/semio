# Gap 4 (b): a rejected archive hydration step now ends the load in a typed fault

Static work only. Nothing was built or run.

## Cause
- `advance_document_archive_load` (`🔌️plugin/🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`) turns a rejected hydration step (`PersistedDocumentHydrationStep::Rejected`) into `Err(document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, ..))`. The same holds for a rejected SPR decode and every other phase fault.
- Its only caller, `drive_document_archive_original` (`🔌️plugin/🪜️maintenance-ladder/🦀️.rs`), re-inserted the load unchanged and returned that `Err` to the maintenance turn. Only the merge route (`request_fault` in `archive-merge`) and the replacement lane (`active.fault = ..` in `AwaitReplacement`) ever set the `Fault` terminal.
- So the load kept `state == Running`, `terminal_target == None`, with `fold` at 483/484. The hydration owner, retained history and input pack stayed live, and `status()` kept reporting `Running`. The host saw only a failed maintenance turn.

## Fix
1. `drive_document_archive_original` now converts a plain `PluginLifecyclePrimary::Fault` from `advance_document_archive_load` into `active.request_fault(&fault)`. It does this only when the load has no terminal target yet and the failure carries no receipt, context or work refusal. The step returns `Progress(failure.effects)`, so the physical receipt of the failed turn is still debited.
   - The next turn runs `drive_document_archive_terminal`, which retires the hydration, history, member and input owners through the existing bounded closure. Then `state == Fault`, `fault` holds the encoded typed fault, `status()` is terminal, and cancellation and progress are unchanged.
   - Retirement, receiving and value failures (budget or invariant breaches) still propagate as `Err`. They are not load verdicts.
2. Removed the stray `eprintln!` in the hydration-rejected branch (`🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs`). The message is now carried by the typed fault.

## Test (written, not run)
File: `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🗂️recursive-archive-rejection-and-stale-authority/🦀️.rs`, included by the test target `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1/🦀️.rs`, so it runs with the `plugin-runtime-plugin-builder-contract-part-1` target of the plugin tests.

- New: `retained_window_input_recursive_document_archive_rejected_history_frame_ends_in_a_typed_fault_with_terminal_progress`. A History frame that fails decode is appended. It asserts `state == Fault`, a present typed `fault`, `completed <= total`, not runnable afterwards, acknowledgement empties `document_archive_loads`, and the live archive is unchanged.
- Related existing laws (same file) that depend on the same path: `retained_window_input_recursive_document_archive_round_trips_the_complete_owned_closure`, `retained_window_input_recursive_document_archive_cancel_retires_the_exact_input_before_acknowledgement`.

## Open
- Not run. `🔌️plugin/🧪️tests/📦️units/📋️preserved.json` lists tests with a sha256 of their source. It likely needs regeneration through the repo tooling because the test file changed.
- The existing malformed-history cases in the round-trip law already expect `Fault`. If they passed before, a conversion I did not find exists elsewhere. The first run shows whether this fix is redundant or the missing link.
