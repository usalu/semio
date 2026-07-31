---
name: commands return diffs, central apply
overview: 'Refactor [compose/rs/lib.rs](compose/rs/lib.rs) so every `Change{Entity}Command::apply` returns a scoped structural diff, and every entity store implements one central `apply_diff` that applies children in strict "remove, then update, then add" order — centralizing event emission, cache invalidation, and pointer rewiring. Changes continue to be tracked as command lists (`KitChange.forward/inverse: Vec<ChangeKitCommand>`); diffs are ephemeral. Old flat diff types and dto-replacement paths are deleted.'
todos:
 - id: diff_rewrite
   content: Rewrite `pub mod diff` + `pub mod kit_diff` with sparse per-entity XDiff + nested XsDiff{removed,updated,added}, `is_empty`, `merge`
   status: completed
 - id: apply_diff_on_stores
   content: Add `pub fn apply_diff(&mut self, &XDiff) -> Result<()>` to every entity store (attribute, author, benchmark, concept, connection, connector, design, file, folder, group, kit, layer, piece, port, prop, quality, representation, side, stat, tag, typ), centralizing event emission, cache invalidation, and pointer rewiring in strict remove->update->add order
   status: completed
 - id: commands_return_diff
   content: Change every Change*Command::apply to return (XDiff, Vec<Self> /*inverse*/), route all mutations through store.apply_diff, delete direct set_*/insert_*/remove_* usage and the FromKitDiff variant
   status: completed
 - id: apply_many_compact
   content: Rewrite ChangeKitCommand::apply_many to fold diffs via merge, add ChangeKitCommand::compact that drops empty diffs, collapses repeated scalar writes, cancels Add+Remove pairs, and merges nested ChangeXCommands
   status: completed
 - id: kit_change_trim
   content: "Trim `pub mod kit_change`: keep KitChange as command-list forward+inverse, delete from_dto_pair / apply_forward_dto / apply_backward_dto, rewrite apply_forward/apply_backward over apply_many"
   status: completed
 - id: delete_legacy
   content: Delete DesignChange, DesignStore::delete_change/flatten_change/invert_change/validate_change/diff_from, KitDiff::between/apply_to_dto/apply_metadata_to_kit_dto, apply_design_full_dto, DesignDiffPatch, apply_design_diff_rpc, apply_kit_diff_rpc, and the WASM apply_design_diff / apply_kit_diff shims
   status: completed
 - id: caller_rewire
   content: Rewire kit_transaction, kit_draft, kit_session, kit_store_command, kit_checkpoint, kit_alternative, io, wasm to the new commands-only model
   status: completed
 - id: tests
   content: Add tests for apply_diff order-of-operations, per-command diff correctness, compact preserves final state, forward+inverse round-trip; rewrite existing apply_design_diff tests as command-based
   status: completed
 - id: agents_md
   content: Update `Change flow` line in [compose/rs/AGENTS.md](compose/rs/AGENTS.md) to describe the new command->diff->central-apply pipeline
   status: completed
isProject: false
---

# commands return diffs, central apply

## 1. Target shape (mirror `Compose.cs`)

Diffs become sparse + recursive. For every entity `X` (Attribute, Author, File, Folder, Tag, Concept, Quality, Benchmark, Prop, Port, Connector, Representation, Type, Piece, Side, Connection, Layer, Group, Stat, Design, Kit):

- `XDiff` — sparse scalar fields (each `Option<T>`, where `None` means "unchanged"); child collections carried as `Option<XsDiff>`. Must implement `is_empty()`, `merge(&Self, &Self) -> Self`.
- `XsDiff { removed: Vec<XIdDto>, updated: Vec<XDiffUpdate>, added: Vec<XFullDto> }` (matches `Compose.cs` `AttributesDiff`, `PortsDiff`, `TypesDiff`, …).
- `XDiffUpdate { id: XIdDto, diff: XDiff }`.

Reference lines in `Compose.cs` (authoritative shape): `AttributeDiff` 2196, `AttributesDiff.Apply` 2247, `PortDiff` 2889, `PortsDiff` 2920, `TypeDiff` 3659, `TypesDiff` 3750, `PieceDiff` 4125, `PiecesDiff` 4106, `DesignDiff` 4777, `DesignsDiff` 4768, `KitDiff` 7266, `Kit.ApplyDiff` 7435.

`Kit.ApplyDiff` (Compose.cs 7435) is the template: it applies collection diffs by calling `ApplyTypesDiff`/`ApplyDesignsDiff`, each of which does `removed → updated → added`.

## 2. Central `apply_diff` on every store

On every `pub mod <entity>` in [compose/rs/lib.rs](compose/rs/lib.rs), add one method:

```rust
pub fn apply_diff(&mut self, diff: &XDiff) -> crate::error::Result<()>;
```

Contract:

1. Apply **scalar field updates** (each `Some` field overwrites, emits `FieldChanged`).
2. For **each child `XsDiff`** in declaration order, call a shared helper `apply_children_diff` that executes strictly:
   - a. `for id in removed`: emit `ChildRemoved`, drop `Arc`, let `Weak`s dangle (consumers already tolerate this).
   - b. `for u in updated`: look up child `Arc`, call `child.write()?.apply_diff(&u.diff)?` (recursion).
   - c. `for f in added`: construct child store from `XFullDto`, wire parent `Weak`, push `Arc`, emit `ChildAdded`.
3. After all children applied: rewire cross-pointers once (e.g. `DesignStore::rewire_piece_flatten_parents`, `Connector → Port`, `Piece → Type`, `Side → Piece/Port`).
4. Invalidate local caches once at the end (`invalidate_hash_local`, `invalidate_flatten`, `invalidate_validation`).
5. Parent stores bubble invalidation (kit-level hash/validation) after their recursive call — see existing `KitStore::apply_design_diff` pattern at [lib.rs:10128](compose/rs/lib.rs).

This replaces the scattered `emit_ev` + `invalidate_hash` + `wire_graph_bus` calls currently sprinkled through every arm of `ChangeKitCommand::apply` at [lib.rs:1645-1994](compose/rs/lib.rs).

## 3. Command return shape

Change every `Change{Entity}Command::apply` from returning `Result<(KitChangeKind, Vec<Self>)>` / `Result<Vec<Self>>` to:

```rust
pub fn apply(&self, scope…) -> Result<(XDiff, Vec<Self>)>;
// XDiff = forward structural delta (for caller materialization / event correlation)
// Vec<Self> = inverse command fragment (in forward order; caller reverses for nested)
```

Flow inside each arm:

1. Build a scoped `XDiff` that represents **only** this command's effect (sparse).
2. Capture inverse command(s) by reading pre-state.
3. Call the matching store's `apply_diff(&XDiff)` — no direct `set_*` / `insert_*` / `remove_*` calls in command code.
4. Return `(diff, inverse_cmds)`.

Consequence: the `FromKitDiff` variant at [lib.rs:492](compose/rs/lib.rs) becomes unnecessary because every command already produces a diff — delete it. `KitStore::replace_from_full_dto` stays only for initial kit load (not change flow).

## 4. `ChangeKitCommand::apply_many` and `compact`

```rust
impl ChangeKitCommand {
    pub fn apply_many(kit: &KitStoreRef, cmds: &[ChangeKitCommand])
        -> Result<(KitDiff, Vec<ChangeKitCommand> /* inverse, pre-reversed */)>;

    /// Collapse redundant commands: consecutive scalar writes to the same (entity, field)
    /// keep only the last; Add{id}+Remove{id} within the batch cancels; nested
    /// ChangeXCommands with the same scope merge their inner lists; no-op
    /// commands (diff.is_empty()) are dropped.
    pub fn compact(cmds: Vec<ChangeKitCommand>) -> Vec<ChangeKitCommand>;
}
```

`apply_many` folds per-command `XDiff`s via `merge` into a single kit-scoped `KitDiff` and concatenates inverses in undo order — same reverse-order convention as the current `apply_many` at [lib.rs:1999-2014](compose/rs/lib.rs).

## 5. `KitChange` and `KitChangeKind`

`KitChange` stays as it is today at [lib.rs:7537-7549](compose/rs/lib.rs):

```rust
pub struct KitChange {
    pub forward: Vec<ChangeKitCommand>,
    pub inverse: Vec<ChangeKitCommand>,
    pub kind: KitChangeKind,
    pub author: Option<String>,
    pub time: Option<String>,
}
```

Delete: `KitChange::from_dto_pair`, `apply_forward_dto`, `apply_backward_dto` at [lib.rs:7556-7604](compose/rs/lib.rs) — these lift snapshot pairs into `FromKitDiff` and are made obsolete by commands-as-source-of-truth. `apply_forward`/`apply_backward` are kept but rewritten to delegate to `ChangeKitCommand::apply_many`.

## 6. Deletions (old diff-based paths marked "bad" by user)

In [compose/rs/lib.rs](compose/rs/lib.rs):

- `pub mod diff` [lib.rs:6834](compose/rs/lib.rs): rewrite. Delete old flat `DesignDiff { added_pieces, removed_pieces, modified_pieces, added_connections, removed_connections, modified_connections }`, `DesignChange`, `DesignStore::delete_change`, `DesignStore::flatten_change`, `DesignStore::invert_change`, `DesignStore::validate_change`. Replace with new sparse `DesignDiff { name: Option<String>, …, pieces: Option<PiecesDiff>, connections: Option<ConnectionsDiff>, … }`.
- `pub mod kit_diff` [lib.rs:6993](compose/rs/lib.rs): rewrite. Delete `DesignDiffPatch`, old flat `KitDiff` with `added_types/removed_types/modified_types/…/patched_designs/replaced_full_designs`, `apply_design_full_dto`, `apply_metadata_to_kit_dto`, `apply_to_dto`, `KitDiff::between`, `KitDiff::apply`. Replace with new sparse `KitDiff { name: Option<String>, …, types: Option<TypesDiff>, designs: Option<DesignsDiff>, files: Option<FilesDiff>, … }` plus recursive `apply` via `KitStore::apply_diff`.
- `KitStore::apply_design_diff_rpc` [lib.rs:10520](compose/rs/lib.rs), `apply_kit_diff_rpc` [lib.rs:10530](compose/rs/lib.rs): delete — callers use `execute_change_kit_commands` instead.
- `ChangeKitCommand::FromKitDiff` variant [lib.rs:492](compose/rs/lib.rs): delete.
- `DesignStore::diff_from` [lib.rs:6510](compose/rs/lib.rs): delete (diffs are per-command products now).
- WASM shims `apply_design_diff` [lib.rs:17156], `apply_kit_diff` [lib.rs:17171]: delete (replaced by `execute_change_kit_commands`).

## 7. Ownership of the refactor inside lib.rs

Sections to touch, in order:

- `pub mod diff` [lib.rs:6834](compose/rs/lib.rs): new sparse `DesignDiff` + `PiecesDiff`/`ConnectionsDiff`/`LayersDiff`/`GroupsDiff`/`StatsDiff` + helpers (`is_empty`, `merge`).
- `pub mod kit_diff` [lib.rs:6993](compose/rs/lib.rs): new sparse `KitDiff` + `TypesDiff`/`DesignsDiff`/`FilesDiff`/`FoldersDiff`/`AuthorsDiff`/`ConceptsDiff`/`TagsDiff`/`QualitiesDiff`/`PropsDiff`/`AttributesDiff` + helpers.
- Every entity module (`attribute`, `author`, `benchmark`, `concept`, `connection`, `connector`, `design`, `file`, `folder`, `group`, `kit`, `layer`, `piece`, `port`, `prop`, `quality`, `representation`, `side`, `stat`, `tag`, `typ`): add `pub fn apply_diff(&mut self, &XDiff) -> Result<()>`. Migrate the existing dto-apply helpers (`apply_full_dto_fields`, `apply_metadata_fields`, `apply_metadata_dto` — see [lib.rs:4226, 4450, 4635, 4955, 5795, 12202, 12218, 13327, 13567, 14133, 14310, 14498](compose/rs/lib.rs)) to be internal to the new `apply_diff`.
- `pub mod change_command` [lib.rs:346](compose/rs/lib.rs): rewrite every `impl`: return `(XDiff, Vec<Self>)`; call only `apply_diff` on stores (not `set_*` directly).
- `pub mod kit_change` [lib.rs:7507](compose/rs/lib.rs): trim to the new `apply_forward`/`apply_backward` wrappers over `apply_many`.
- `pub mod kit_transaction` [lib.rs:3066], `pub mod kit_draft` [lib.rs:3166], `pub mod kit_session` [lib.rs:3296], `pub mod kit_store_command` [lib.rs:3356], `pub mod kit_checkpoint` [lib.rs:2877], `pub mod kit_alternative` [lib.rs:3013]: replace any remaining `KitDiff` / `DesignDiff` call sites with command execution.
- `pub mod io` [lib.rs:15446](compose/rs/lib.rs): update serde shape for `KitChange` persistence (command-only, no pre-state diff snapshots).
- `pub mod wasm` [lib.rs:16876](compose/rs/lib.rs): delete `apply_design_diff` / `apply_kit_diff` handles; keep only `execute_change_kit_commands` (already added in plan [kit_store_story_0a521ca8](.cursor/plans/kit_store_story_0a521ca8.plan.md)).

## 8. Test coverage in `mod tests` [lib.rs:17436](compose/rs/lib.rs)

- `apply_diff_order_is_remove_update_add` per entity (assert event order via `event_bus` subscribe).
- `command_returns_diff_matching_state_delta` per command family (Scalar / Add / Remove / nested ChangeXCommands).
- `compact_preserves_final_state` — random 10-20 commands, `apply_many(compact(cmds))` yields same `to_full_dto()` as `apply_many(cmds)`.
- `round_trip_forward_inverse` — `apply_many(forward)` then `apply_many(inverse)` returns initial DTO (already partially at [lib.rs:17552](compose/rs/lib.rs), extend).
- Retire `apply_design_diff_add_piece_emits_child_added_and_hashes` [lib.rs:18944] and rewrite as command-based equivalent.

## 9. [compose/rs/AGENTS.md](compose/rs/AGENTS.md)

Update the `Change flow` line to: "`change_command` produces sparse diffs; every store's `apply_diff` is the single write path (remove → update → add), centralizing events, cache invalidation, and pointer rewiring. VCS: [`KitChange`](lib.rs) stores `Vec<ChangeKitCommand>` forward + inverse; diffs are ephemeral per-apply products."

## Out of scope

- Replaying from an `initial kit` to materialize a checkpoint (covered by [kit_vcs_command_document_cdecfba5](.cursor/plans/kit_vcs_command_document_cdecfba5.plan.md)).
- JS/React/sketchpad consumers of the old `applyKitDiff` / `applyDesignDiff` WASM methods — handled by the follow-up `KitStore` storybook story [kit_store_story_0a521ca8](.cursor/plans/kit_store_story_0a521ca8.plan.md).
- SQL/JSON/ZIP on-disk format migration — breaking format change accepted; old kits re-import from `KitFullDto`.
