# Semio Rs Merkle Hashing

**Status:** Done

**Goal:** Implement Merkle-style entity and connection hashing per plan (sorted child hashes, all non-computed fields).

## Summary

- `crate::hash`: `merkle_node_str`, `merkle_collection` for nodes and Relay shells.
- `gql_relay`: async `from_*` where child `compute_hash` is async; sync `simple_conn_sync!` `from_rows` for DTO rows; `FamilyConnection` + `Family::compute_entity_hash`.
- Meta: `Quality` merkle over scalars + sorted benchmark/attribute digests; `Stat`/`Layer`/`Group` complex + `compute_entity_hash`; `File` `content_hash` with JSON key `hash` via `#[serde(rename = "hash", alias = "contentHash")]`.
- `Conflict::compute_hash` + full-field merkle; `Query::conflicts` awaits `ConflictConnection::from_conflicts`.
- Geometry `entity` nodes: `compute_hash` on all node kinds; iface `hash` resolvers delegate.
- `iface::OwnedEntityConnection::empty` uses `merkle_collection([])`.
- Tests: merkle invariants, File serde `hash` key, gql_relay guard (no legacy `hash_ids` in relay module).

## Files

- `semio/rs/lib.rs`
- This ticket folder (logs only)
