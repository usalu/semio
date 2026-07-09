---
name: compose-rs-merkle-hashing
overview: "Replace the partial per-entity hash logic in `compose/rs/lib.rs` with a uniform Merkle scheme: every entity hashes all its non-computed scalar fields plus the sorted hashes of its child entities; every collection (`gql_relay::*Connection`) hashes the sorted hashes of its child nodes (not their ids)."
todos:
 - id: hash-helpers
   content: Add merkle + merkle_collection helpers in crate::hash
   status: completed
 - id: value-dtos
   content: "Convert value-DTO SimpleObjects to manual #[Object] impls with compute_hash; rename File.hash → File.contentHash"
   status: completed
 - id: arc-entities
   content: Rewrite compute_hash for every Arc-backed entity (kit/type/representation/connector/port/design/piece/connection/side/tag/concept/quality/vcs/etc.) to merkle-fold non-computed fields + sorted child hashes
   status: completed
 - id: gql-relay
   content: Make gql_relay::*Connection::from_* async and switch hash to sorted child compute_hash; await every call site
   status: completed
 - id: geom
   content: "Rewrite geometry #[Object] hash resolvers (Coordinate/Vector/Point/Plane/Position/Offset/Place) to use merkle"
   status: completed
 - id: tests
   content: Extend mod tests with five new merkle hashing tests + a no-hash_ids guard; run cargo test -p compose
   status: completed
 - id: ticket
   content: Open/reopen the ticket and close it on completion with the touched-files summary
   status: cancelled
isProject: false
---

# Implement Merkle Hashing For All Entities

## Scope

All work happens inside the single crate file [`compose/rs/lib.rs`](compose/rs/lib.rs) (plus the existing ticket folder for any temporary logs). Exit conditions: every entity exposes a `hash` GraphQL field that is a Merkle hash, every `XxxConnection` exposes a `hash` that combines sorted child hashes, and `cargo test -p compose` passes (extending the existing `mod tests` block, no new test files).

## Design

### `crate::hash` helper API (replace current `h`-only module)

```rust
pub fn h(parts: &[impl AsRef<[u8]>]) -> String { /* unchanged */ }

/// Stable Merkle node: own scalar bytes feed in first, then child hashes are
/// sorted lexicographically and folded in. Order of `parts` matters; order of
/// `children` does not.
pub fn merkle(parts: &[&[u8]], children: &[String]) -> String { /* sorts children, blake3 */ }

/// Convenience for collections: hash a sorted list of child hashes.
pub fn merkle_collection(children: &[String]) -> String { /* sorts, blake3 */ }
```

Sorting child hashes guarantees:

- Insertion-order independence for set-like children (tags, qualities, attributes, props, …).
- Stability across replays / hydrations.

### Per-entity contract

For every Arc-backed entity:

- Add or rewrite `pub async fn compute_hash(&self) -> String`.
- Hash `id` + every **non-computed scalar** RwLock field (resolve `Option` to `""`, normalize `f64` via `format!("{:.9}")`, ints via `to_string`, bools via `"0"`/`"1"`).
- Collect `compute_hash().await` from every owned child Arc/value entity, then call `merkle(&own_parts, &child_hashes)`.
- The GraphQL `hash` resolver simply calls `compute_hash().await` (already the convention).

For every value-DTO entity (`Attribute`, `File`, `Folder`, `Author`, `Benchmark`, `Prop`, `Stat`, `Layer`, `Group`):

- Add `pub fn compute_hash(&self) -> String` (sync; values have no RwLocks).
- Convert the type from `SimpleObject` derive to a manual `#[Object]` impl that exposes every existing field plus `pub async fn hash(&self) -> String { self.compute_hash() }`. Keep `Serialize`/`Deserialize`/`Clone`/`Default` derives so the JSON snapshot/hydration paths are untouched.
- Rename `File.hash` (currently file-blob hash) to `File.content_hash` (`#[graphql(name = "contentHash")]`) so the entity-level `hash` field has a single, consistent meaning across the schema.

### Children-per-entity (drives the Merkle wiring)

- `Kit` → designs, types, files, folders, authors, concepts, tags, qualities, props, attributes, stats.
- `Type` → connectors, ports, representations, tags, concepts, qualities, props, attributes, stats, authors.
- `Representation` → tags, qualities, attributes (+ optional `file`).
- `Connector` → qualities, attributes (+ optional `port` weak ref by id).
- `Port` → no children.
- `Design` → pieces, connections, layers, groups, authors, qualities, props, attributes, stats (+ optional `location`).
- `Piece` → props, attributes, optional `position`.
- `Connection` → optional sides (connected/connecting), attributes.
- `Side` → no owned entity children; hashes ids of referenced piece/port/connector/design_piece (Weak refs are content references, not Merkle children).
- `Tag` / `Concept` / `Quality` → attributes (Quality also includes benchmarks).
- `Position` → center + plane; `Plane` → origin + x_axis + y_axis; etc. (geometry already nearly correct; rewrite to use the new `merkle` helper for consistency).
- `Graph` → checkpoints, alternatives, releases, drafts (+ `parent_root_for_active_draft` kit hash).
- `Checkpoint` → `root` kit hash + parent checkpoint hash + authors.
- `Alternative` → start checkpoint + checkpoints + draft + transaction.
- `Draft` → finalized_transactions + open_transaction + redo_transactions.
- `Transaction` → changes.
- `Change` → forward + backward operation payloads (hashed as JSON bytes).
- `Conflict`, `ReadVersion`, `WriteVersion`, `Session` → keep flat hashes (no owned entity collections).

### Computed fields explicitly excluded

Documented as a region comment near each entity:

- `Piece`: `parent_piece`, `child_pieces`, `parent_connection`, `child_connections`, `depth`, `path`, `flat_position`.
- `Type`: `connector_weak_by_id`, `port_weak_by_id`, `representation_weak_by_id`.
- `Design`: `piece_weak_by_external_id`.
- `Kit`: `design_weak_by_id`, `type_weak_by_id`, `tag_by_id`, `concept_by_id`, `quality_by_id`, `touch_epoch`, `snapshot_external_kit_id`.
- `Graph`: `materialized_cache`, `self_weak`, `op_history`.
- All `Weak<Owner>` back-pointers (parent direction), all `RwLock<HashMap<Id, Weak<…>>>` indexes.

### Collection hashing (`gql_relay`)

Today every `XxxConnection::from_*` does `hash_ids(rows.iter().map(|r| r.id.as_str()))`. Replace with a single async helper:

```rust
async fn merkle_from_children<H: Future<Output = String>, F: Fn(&T) -> H, T>(rows: &[T], hash_one: F) -> String
```

and rewrite each `from_*` constructor as:

- `pub async fn from_pieces(rows: Vec<Arc<Piece>>) -> Self { let hash = merkle_collection(&join_all(rows.iter().map(|p| p.compute_hash())).await); … }`

Because `compute_hash` is async, every `from_*` constructor becomes async. Update all ~37 call sites (`PieceConnection::from_pieces(...).await`, etc.); they're already inside `async fn` GraphQL resolvers in [`compose/rs/lib.rs`](compose/rs/lib.rs) so adding `.await` is mechanical. The `simple_conn!` macro and `entity_relay!` macro both expand to async `from_rows` returning `Self`. SimpleObject value rows (`File`, `Layer`, …) call their new sync `compute_hash()`; Arc'd entity rows call `compute_hash().await`.

### Tests (extend existing `mod tests` only)

Add to the existing `#[cfg(all(test, not(target_arch = "wasm32")))] mod tests` block:

- `entity_hash_changes_when_field_changes`: rename a kit / piece / tag and assert `hash` differs.
- `entity_hash_stable_across_clone_round_trip`: deep-clone a kit and assert root-kit hash equals.
- `connection_hash_is_order_independent`: insert tags A,B then B,A on equivalent kits and assert `tags.hash` matches via the GraphQL `hash` field (collections sort children).
- `parent_hash_is_merkle_of_children`: change a leaf piece's name and assert the owning Design's `hash` and Kit's `hash` both change.
- Keep an explicit guard test that searches `lib.rs` for any literal `hash_ids(` after the refactor and asserts zero matches (prevents regression to id-only hashes).

## Order of execution

1. Rewrite `crate::hash` module with `h` + `merkle` + `merkle_collection`.
2. Convert value-DTO `SimpleObject` types to manual `#[Object]` + sync `compute_hash`. Rename `File.hash` to `File.content_hash`.
3. Rewrite every Arc-entity `compute_hash` to feed all non-computed fields and Merkle-fold sorted child hashes.
4. Convert `gql_relay::*Connection::from_*` to async + sorted child Merkle hashes; update all call sites with `.await` (every site is already in an `async fn` resolver).
5. Rewrite the geometry `#[Object]` `hash` resolvers (`CoordinateNode`, `VectorNode`, `PointNode`, `PlaneNode`, `PositionNode`, `OffsetNode`, `PlaceNode`) to use `compute_hash` + `merkle`, mirroring the new pattern.
6. Extend `mod tests` with the five new tests above; run `cargo test -p compose --lib` until green; update `kit_store_bundle_serialize_hydrate_round_trip_via_graphql` if it asserts on specific hash strings.
7. Verify `npx nx build compose/graphql` regenerates the SDL with the new `contentHash` field on `File` (no consumer ticket needed because the bundle is greenfield per workspace rules).
8. Close the existing ticket (or open a new one if reopening is impossible) with the touched-files summary.
