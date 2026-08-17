# CW7 — App fan-out: `CollectionOperation` shape migration

Scope, per CW3's report (deviation #3) and the campaign contract's "App fan-out" section: migrate
every app crate that constructs or pattern-matches `vcs::CollectionOperation`/calls
`vcs::apply_collection_operation`/`vcs::invert_collection_operation`/
`vcs::collection_diff_from_operation` onto the frozen `protocol::CollectionOperation` shape and the
`protocol::{apply_collection_operation, invert_collection_operation, collection_diff_from_operation}`
functions (now re-exported at the facade — confirmed and fixed in `protocol/rs/lib.rs` immediately
before this wave started; previously only reachable via `protocol_command` directly, a gap now
closed).

## The shape change (verified against both crates' current source)

```rust
// vcs's own (OLD — unchanged, still exists, still used by anything not yet migrated):
pub enum CollectionOperation<TId, TItem, TPatch> {
    Add { index: usize, item: TItem },      // NO id — position-only insert
    Remove { id: TId },
    Move { id: TId, to_index: usize },
    Patch { id: TId, patch: TPatch },
}

// protocol_command's (NEW — the frozen target shape):
pub enum CollectionOperation<TId, TItem, TPatch> {
    Add { id: TId, item: TItem, at: usize }, // id supplied by the CALLER at construction time
    Remove { id: TId },
    Move { id: TId, to: usize },
    Patch { id: TId, patch: TPatch },
}
```

This is NOT a pure rename. `Add` gains a mandatory `id: TId` field that vcs's version never had —
the collection used to assign identity implicitly (by position, or by a field inside `TItem`
inspected after insertion); the new shape requires the id to be known and supplied *before*
insertion. `Move.to_index` renames to `Move.to` (pure rename, no semantic change).

## Per-crate migration steps

1. Grep your crate's `lib.rs` for `CollectionOperation::` (construction and pattern-match sites)
   and for `vcs::apply_collection_operation`/`vcs::invert_collection_operation`/
   `vcs::collection_diff_from_operation` calls.
2. Determine YOUR crate's natural id source for the collection's items — look at how the SAME
   collection's `Remove`/`Move`/`Patch` operations already obtain a `TId` for existing items (they
   already carry `id: TId` in both the old and new shape, unchanged). The item type `TItem` almost
   certainly already has that same id available as a field (commonly `id: String` on a
   `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslEnum)]` struct) — use that. If the id is
   generated fresh at insertion time (e.g. a counter, a ULID, a slug derived from user input),
   preserve that exact generation logic, just supply the resulting value as `Add`'s new `id` field
   instead of only implicitly deriving it after the fact.
3. Rewrite every `Add { index, item }` construction to `Add { id, item, at: index }` (`index`'s
   value becomes `at` verbatim — only the field name changes; `id` is new, per step 2).
4. Rewrite every `Move { id, to_index }` to `Move { id, to: to_index }` (pure rename).
5. Update pattern matches (`CollectionOperation::Add { index, item }` -> `Add { id, item, at }`,
   etc.) analogously — bind `id` even if some match arms don't use it yet (prefix `_id` if truly
   unused, never `..` unless you also verify the ignored fields carry no logic this match arm
   silently used to depend on).
6. Switch every `vcs::apply_collection_operation`/`vcs::invert_collection_operation`/
   `vcs::collection_diff_from_operation` call to `protocol::apply_collection_operation`/
   `protocol::invert_collection_operation`/`protocol::collection_diff_from_operation` (same
   function names, now operating on the new-shape `CollectionOperation`). Also switch any
   `use vcs::{CollectionOperation, ...}` import to `use protocol::{CollectionOperation, ...}` (and
   `Identified`/`Patchable`/`CollectionDiff`/`ItemPatch` similarly, if imported from `vcs` — these
   moved to `protocol` too per the same shape freeze, confirm by checking whether your crate's
   `TItem: Identified<TId>`/`Patchable<TPatch>` impls reference `vcs::Identified`/`vcs::Patchable`
   or already use `protocol::`).
7. If your crate's `Operation`/`OperationDiff` impl (the outer command type wrapping this
   collection op) already derives from `vcs::Operation`/`vcs::OperationDiff` per CW3's earlier
   Cargo.toml sweep (it should already have a `protocol` dependency — check before adding one),
   this is a good moment to also switch THAT import from `vcs::{Operation, OperationDiff, OpText}`
   to `protocol::{Operation, OperationDiff, OpText}` if not already done (CW3 flipped the trait
   *definitions* vcs re-exports via a temporary shim, but explicitly left the import-PATH surgery
   at each app call site as "CW7's job" — this is that job, do it while you're already in the file,
   but ONLY for the `Operation`/`OperationDiff`/`OpText` trio; do not touch anything else).
8. Add `protocol` as a direct Cargo dependency ONLY if genuinely missing (most of these ~40 crates
   already got it in CW3's mechanical sweep — check `Cargo.toml` first).
9. Build + test your crate. Fix any ripple (a sibling crate in the SAME workspace depending on your
   crate's public types that also construct/match the old shape — grep repo-wide for your crate's
   own public API names before assuming you're done, but do NOT edit a crate outside your
   assignment; report a cross-crate ripple instead of silently fixing it in someone else's file).
10. Write a short report `cw7-<crate-slug>.txt` in this ticket folder: what changed, the id source
    you chose and why, any deviation, build/test status.

## Ownership discipline (per every prior wave's rule)

Touch ONLY your assigned crate's own files (`lib.rs`, `Cargo.toml` if a dep is genuinely missing).
Never touch: root `Cargo.toml`, `.vscode/launch.json`, `vcs/rs/lib.rs`, `protocol/*`,
`framework/core/rs/lib.rs`, `framework/plugin/rs/lib.rs`, `framework/sync/rs/lib.rs`,
`dsl/derive/rs/lib.rs`, any hub binary, or any other app crate. Re-read your assigned file
immediately before editing (this is a live, concurrently-edited repo). Do not run git commands.
Scratch/progress files only in this ticket folder, as `.txt`.

## Crate assignments (17 crates with confirmed real usage, grep-verified before this wave)

imperative/core/rs, imperative/plugin/rs, layout/plugin/rs, layout/rs, architect/program/rs,
architect/plugin/rs, sequence/core/rs, infinite/board/port/directed/dag/rs,
infinite/board/port/directed/dag/plugin/rs, gis/plugin/rs, shooting/rs, shooting/plugin/rs,
process/plugin/rs, process/3d/rs, flow/core/rs, animate/present/rs, animate/plugin/rs,
lowpoly/core/rs.
