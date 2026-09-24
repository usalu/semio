# WP-C5 — Durable Collaborative Undo/Redo

Ticket: `26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP` (continues `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`).
Closes audit **A3** without inventing a parallel undo ledger.

## Design

Document history stays event-sourced:

- Domain ops plus `HistoryTransition::{Revert,Reinstate}` folded by `fold_history` / `fold_envelope_history`.
- Per-author selective undo/redo: `edit_is_local` + `redo_position` (S11/S12 `history_row_applied_v1` rail; KN1 `MutationDag` cascade for causal apply).
- Durability = the same transition log surviving pack+`.spr` reload / hub WAL restart. Session rebinds `local_actor_id` before Redo (construct seeds from applied tail, which may be foreign after selective undo).

WP-C5 makes selective undo the **default** store path and proves the shared redo stack is durable.

## Landed

| Area | Change |
|---|---|
| Store | `ArtifactCommand::Undo` → `UndoPolicy::TransformAgainstConcurrent`; `UndoInLane` requires `edit_is_local` |
| Fixture | `replication/.../durable-collaborative-redo-v1` language-agnostic fold (reload + hub-restart steps) |
| Rust | `durable_collaborative_redo_fixture_survives_reload_and_hub_restart` |
| TS | `framework-replication` vitest `durable-collaborative-redo` |
| Store unit | `plain_undo_is_selective_and_durable_across_event_log_reload` (pack+`.spr` + session actor rebind + Redo) |
| Hub fixture | `two-author-shell-v1` claims `durable-collaborative-redo` / `redo-stack-survived-hub-restart`; `executionProtocol` → compiled AppChannel **17** |
| Hub process | GIS shell: ordinary `redo()` after hub restart; fence asserts absence of `no-durable-collaborative-redo` |
| Hub check script | two-author composition laws use `DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1` |

## Verification (this slice)

| Check | Result |
|---|---|
| `cargo test -p semio-framework-os-kernel plain_undo_is_selective_and_durable` | PASS |
| `cargo test -p semio-framework-os-kernel transform_against_concurrent_undo` | PASS |
| `cargo test -p semio-framework-os-kernel exact_base_only_undo` | PASS |
| `cargo test -p semio-framework-replication durable_collaborative_redo_fixture` | PASS |
| `bun ./script.ts test -- durable-collaborative-redo` (`@semio-tech/framework-replication`) | PASS (12) |
| `trusted-stdio-gis-bundle-check --two-author-source` via fleet-mutex `hub wp-c5` | PASS (laws=18) |

Full Playwright `gis-map-proposal-process` / `os-hub:test-quick` not re-run here; composition source laws require the mounted journey separately.

## Non-goals

- No second undo/redo ledger beside Revert/Reinstate.
- ExactBaseOnly remains available via `UndoWithPolicy` for callers that need foreign-tail refusal.
