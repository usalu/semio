# 🧷️ setContributions stall (2026-11-23)

## What the browser log proves

The 14:05 wasm console prints `one-item publication blocked` ONCE (the arm printed up to three times), then
`stalled 'setContributions' ... 4096 units`. So `Blocked` is not the repeating cause: it fired a single time and the other
4095 units were silent non-advancing answers from a different exit of `publish_mounted_typed_operation_unit`.

## Every producer of `ArtifactStoreOneItemAdvance::Blocked` (Config lane = bundle store)

| Site | Condition |
| --- | --- |
| `store/⚙️engine/🧳️bundle/🦀️.rs` `advance_apply_batch` (about 20 sites) | grant items/depth/copy/capacity below the phase demand; digest/inverse/diagnostic work quote not admitted; zero-progress step |
| `bundle/📬️commit/🦀️.rs` (4) | commit metadata, lane insert, queue depth |
| `bundle/📤️message/🦀️.rs` (12) | edit message, seed owner, outbox reserve |
| `store-components/.../store-owners-batch-fold-and-history` `fold_batch_item` (24) | per-phase copy/capacity/depth; phase 12 needs `foreign_step_presence` (the earlier fix) |
| `Preparation::advance` of the hand-authored `SourcingCurationConfigPreparation` | `!permits_one`, `prepare_work(1)` denied, copy below 4096; surfaces through bundle `PreparationStep::Blocked` |

## What `setContributions` hits natively

New law `the_real_demonstrator_pack_settles_through_the_typed_set_contributions_operation` (curation editor unit tests)
dispatches the real demonstrator pack through `settle_registered_typed_operation`. It settles, also with the receiving
grant clamped to depth 64 as in the browser. Its single `Blocked` is bundle `PreflightingCommit` digest step
(`advance_digest` with work demand 0 returns default progress, the zero-progress ternary) and resolves on the next unit.
Same single Blocked as the browser. So the Blocked is a benign one-turn wait and the persistent stall is a later
silent exit (candidates: the receiving quote exit `fuel <= scope_work` / items / copy, or the receiving scope not
admitted; the other agent's `with_native_receiving_quoted` debug tags probe the latter). Not reproducible natively with the
fixture caller, so it depends on the production turn budget (wasm memory pressure: the console also shows the guest at 60% of
its linear memory).

## Fix: no silent forever

Every silent non-advancing exit of the publication unit now records its cause and the stall fault carries it:

- `retained-command/🔄️full-operation/🦀️.rs`: `note_publication_wait` / `last_publication_wait` (cause plus three figures, per operation).
- `typed-operation-unit/🦀️.rs`: records the receiving-quote exit, the not-admitted receiving scope, `Blocked`, `PreparationRefused`, `AwaitingAck`; the two earlier `[DEBUG]` prints of this arm are gone.
- `jobs/.../typed-operations/🦀️.rs`: the `interactive-job.publication-stalled` message now ends with `: waiting on <cause> [a, b, c]`; the `[DEBUG] stalled` print is gone.
- Law: `a_publication_wait_names_its_cause_for_exactly_its_operation` (retained-command unit tests).

After the wasm rebuild the browser error names the exact exit and its figures; that decides the last production fix.

## Wasm

A wasm rebuild is required (framework production code changed).

## Test counts

- curation lib (`--test-threads=1`): 97 passed, 1 failed (`history_edits_end_to_end`, peer-owned, panics inside the peer's `trace/memory` debug code, unrelated to Blocked).
- retained-command: the new wait law passes (1 of the 8 selected by the filter run).
- the one-item publication crate was not re-run: its source is unchanged, all temporary instrumentation in it was reverted.
