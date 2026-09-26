# WGPU TextEditor Receipt Outbox and Local Echo

## Scope

This continuation gives each resident WGPU TextEditor one bounded, receipt-aware delivery lane matching the mounted React editor's first-plus-latest dispatch behavior.

## Implementation

- Added one active and one latest delivery snapshot per fixed EngineCanvas surface slot. A snapshot owns the controller, surface, full text, and selection.
- Added a fair fixed-slot outbox driver. It publishes at most one ready editor delivery per step and treats temporary input item or byte pressure as retained work.
- Correlated `textEdit` and `textSelect` through `ActionQueueReceipt`. Edit failure aborts the matching selection in the renderer; settlement updates only the exact live slot generation and receipt token.
- Coalesced selection-only changes through the same lane. A delivery omits `textEdit` only when the newest pending text, or the acknowledged text when no edit is pending, proves that the guest already has it.
- Kept the optimistic host text and caret when an incoming scene is an acknowledged local echo. A distinct guest buffer remains authoritative and clears the pending echo ledger.
- Restored the cached guest scene after the last refused edit. `undeclared-action` and `viewer-read-only` make the exact host read-only; an undeclared selection action suppresses later selection delivery while preserving edits.
- Added bounded retirement for delivery snapshots and pending echoes. A retired source cannot be mutated by a late receipt because slot generation is part of the receipt owner identity. Actions already admitted to the renderer remain dispatchable.
- Exposed `has_pending_text_editor_outbox` so the resident host wake predicate keeps driving ready work even when no generic text-buffer work remains.

## Neutral laws and oracles

The native implementation consumes both shared TextEditor fixtures:

- `📮️delivery` covers first/latest coalescing, edit refusal suppressing its selection, and viewer refusal restoring guest text and enabling read-only mode.
- `🔁️local-echo` covers ordered repeated text, stale acknowledgements, external edits, unsent typing, refusal restoration, the bounded pending ledger, and read-only refusal.

The mounted React delivery oracle passed all four focused cases in the root-owned run. The existing TypeScript local-echo oracle remains the third-party comparison for the same neutral fixture.

## Validation

- Root-owned WGPU WASM cargo check passed after the public receipt API and renderer integration landed: exit 0 in 1 minute 45 seconds.
- The two native-shape diagnostics from the preceding run were repaired: the Ink scroll law now retains its seeded node id, and the VFS law uses the qualified WGPU `InputState` type.
- `git diff --check` reports no whitespace faults for the edited Rust sources.
- `rustfmt --check` parses the edited Rust sources and reports only formatting differences in these concurrently edited files.
- Both neutral fixture JSON files parse with Bun.

Native laws queued for the coordinated cargo run:

- `text_editor_receipts_match_the_neutral_first_latest_refusal_and_read_only_laws`
- `text_editor_scene_echoes_never_overwrite_newer_unsent_local_text`
- `text_editor_delivery_state_matches_the_neutral_local_echo_ledger`
- `text_editor_outbox_preserves_local_echo_during_temporary_action_credit_pressure`

Changed existing TextEditor key, clipboard, composition, accessibility, and production typing laws now drain and settle the outbox rather than assuming synchronous action publication.
