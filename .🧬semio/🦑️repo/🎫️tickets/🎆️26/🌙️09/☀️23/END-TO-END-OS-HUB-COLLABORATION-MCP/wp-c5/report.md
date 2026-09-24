# WP-C5 Durable Collaborative Undo/Redo — Report

## Design

Builds on existing history ledger (Revert/Reinstate + fold_history). Selective per-author undo/redo; durability via pack+.spr / hub restart; session rebinds `local_actor_id`.

## Landed

1. Default Undo → TransformAgainstConcurrent; UndoInLane filters edit_is_local.
2. Fixture durable-collaborative-redo-v1 + Rust fold + TS runner.
3. Store unit plain_undo_is_selective_and_durable_across_event_log_reload (pack reload + actor rebind).
4. two-author-shell-v1: durable-collaborative-redo claim; AppChannel 17; process redo after restart; no no-durable fence.
5. Composition source check green (fleet-mutex hub wp-c5).

## Verification

- os-kernel plain_undo / transform / exact_base: PASS
- replication durable fold: PASS
- TS durable-collaborative-redo: PASS (12)
- hub two-author-source: PASS

## Docs

See ticket-root `wp-c5.md`.
