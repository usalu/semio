# Summary

Repo PostgreSQL schema and migrations for repo persistence.

# 💯Requirements

## Schema Groups

- Core repo state stays in the existing developer, ticket, scope, event, goal, and artifact tables.
- Kit persistence is normalized into three layers:
  - `kits`, `kit_checkpoints`, `kit_alternatives`, `kit_sessions`, `kit_drafts`, `kit_transactions`, and `kit_releases` for version-control and session state.
  - `kit_snapshots` for durable point-in-time kit snapshots, including initial and materialized release snapshots.
  - `kit_snapshot_*` tables for normalized metabolism-style snapshot content such as families, kinds, layouts, files, folders, qualities, pieces, connections, and owner-scoped properties or attributes.

## Design Boundary

- Checkpoint and transaction command streams remain JSONB in `kit_checkpoint_changes` and `kit_transaction_changes` because they are polymorphic action logs.
- Durable kit content is normalized under `kit_snapshots` so the snapshot can be queried relationally.
- `kit_snapshots.source_json` preserves the original wire payload so fixture-only fields are not lost while the relational surface evolves.
- Session-scoped draft bases keep `before_snapshot_json` for exact stateful recovery even when a draft has not been promoted to a durable normalized snapshot.
