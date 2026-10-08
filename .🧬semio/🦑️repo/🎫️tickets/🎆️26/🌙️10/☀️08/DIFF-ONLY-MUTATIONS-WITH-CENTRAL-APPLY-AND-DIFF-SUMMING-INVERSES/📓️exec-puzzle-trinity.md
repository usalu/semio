# Exec puzzle-trinity (in progress)

Status: all source/fixture/schema/test edits WRITTEN; compile verification in progress (build slot contention, one OOM kill at 137 already). Plugins are standalone workspaces: run cargo from `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/📦️packages/🦀️rust` (not `-p` from `✏️s`).

## Design as implemented
- Sparse typed diffs: `Option<T>` per field, `Option<Option<T>>` (skip + `deserialize_double_option`) for optional entity fields, nested patches for 5d part_2d/part_3d and grip_2d/grip_3d.
- Keyed deltas `{added, removed, patched[{id, patch}], reordered}` via `keyed_delta!` with sound `absorb` (create∘delete cancels, delete∘create replaces, patch∘patch merges) and concrete `inverse`.
- Every diff type has a concrete `DiffAlgebra::inverse/between/is_empty`; `MutationDiff::apply(&self, base, ApplyCapability)`; io folds and editor `advance` use `protocol::apply_diff`.
- Selection kinds: per-artifact classifier + sparse diff + concrete inverse from payload + base (shared `puzzleNd_selection_inverse` deleted). 3d follow-solver returns poses.
- Configs: per-field `Set*` + sparse `Diff`; window configs: single `Set{patch}`; ephemeral transient/presence keep whole-record `Snapshot` mutation (framework ephemeral transfer contract) with sparse `Diff`.
- Trinity: `RewritingDiff` = sparse `JackDiff` working-graph sub-diff + LHS/Pattern/RHS patches + map deltas; jack diff sparse with `Option<Option<>>` for manifest_id/root_node_id.

## Deviations
- Ephemeral transient/presence: whole-record snapshot payload/inverse retained.
- edit-lhs/rhs/working-graph inverses stay absolute setters (sparse forward diff); RHS statement lists atomic per list.
- disconnect-kind-compatibility inverse appends at end (exact for last-row removals only).
- 3d replace-object-vortex bug fixed (was always no-op) with fixture/oracle changes.
- New id-mismatch invariant in replace-node-handle / replace-object-vortex / replace-part-grip.
- `puzzleNd_snapshot_mutations` / `document_delta_operations` (non-leaf whole-snapshot comparators) untouched.

## Test status
(to be filled after compile)
