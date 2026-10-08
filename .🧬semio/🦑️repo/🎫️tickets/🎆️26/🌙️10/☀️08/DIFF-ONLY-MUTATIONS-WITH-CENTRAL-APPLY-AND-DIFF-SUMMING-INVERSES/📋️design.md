# 📋️ Diff-Only Mutations — Target Design And Violation Taxonomy

## Laws

Notation: `m` is a mutation of artifact snapshot type `P` with diff type `D`, `base: &P`.

- **L1 — Declarative concrete diff.** `m.diff(base) -> MutationOutcome<D>` builds the diff *declaratively* from the
  payload plus reads of `base`. The diff names exactly the entities/fields it changes with their new values
  (sparse, artifact-specific, typed). It never carries a whole after-snapshot, never is computed by mutating a copy of
  `base` and differencing (`D::between(base, &mutated)`), and never is a generic value/JSON patch.
- **L2 — Concrete inverse.** `m.inverse(base) -> Vec<Op>` returns concrete mutations of the same artifact, built from
  the payload plus reads of `base` inside the leaf's own `↩️inverse/🦀️.rs`. It is never derived generically from the
  forward diff (`*_inverse(base, outcome)`, `inverse_from_diff`, walking `m.diff(base).diff()`), never a
  whole-snapshot restore mutation, never a generic "apply this diff" mutation, never `Vec::new()` for a change that
  actually alters state.
- **L3 — Inverse diffs sum to the negative diff.** With `after = m.diff(base).apply(base)`, `s₀ = after`,
  `dₖ = invₖ.diff(sₖ₋₁)`, `sₖ = dₖ.apply(sₖ₋₁)`, `Σ = d₁ ⊕ … ⊕ dₙ` (`MutationDiff::absorb`):
  `Σ.apply(after) == base` and, where the artifact implements `DiffAlgebra`, `Σ == m.diff(base).diff().inverse(base)`.
- **L4 — Central apply only.** Only the framework's central applier turns a diff into a snapshot. No mutation leaf,
  `MutationKind` impl, hand-written `impl Mutation<P>`, diff builder or inverse builder calls `MutationDiff::apply`,
  takes `&mut P`, or clones `base` to write fields into it.
- **L5 — Impossible by design.** `MutationDiff::apply` requires an `ApplyCapability` that only the central applier
  (`protocol::apply_diff` and the store/vcs folds built on it) can mint, so leaf code cannot call it. Policy gate rules
  in `verify mutation-outcome-law` reject the remaining textual loopholes (`&mut <Snapshot>` in leaves, base clones
  mutated in leaves, `between(` in leaves, generic inverse helpers).

## Violation classes (audit vocabulary)

| Code | Meaning |
|---|---|
| `V1-SNAPSHOT-DIFF` | diff computed by mutating a clone of base and differencing / full-replacement diff |
| `V1-GENERIC-DIFF` | diff is a generic value/JSON patch or whole-collection replace instead of a sparse typed diff |
| `V2-DIFF-DERIVED-INVERSE` | inverse built by walking the forward diff via a shared generic helper |
| `V2-RESTORE-INVERSE` | inverse restores a whole snapshot/collection or wraps a generic diff |
| `V2-EMPTY-INVERSE` | inverse returns nothing for a state-changing mutation |
| `V3-LEAF-APPLY` | leaf/kind/diff/inverse calls `.apply(` or takes `&mut P` / mutates base clone |
| `V3-HAND-MUTATION` | hand-written `impl Mutation<P>` that bypasses `#[derive(Mutations)]` and applies directly |
| `V4-LAW-UNTESTED` | no L3 law test for the leaf |

## Rulings

- **Minimality.** A diff carries exactly the fields the mutation semantically owns. A `replace-<entity>` kind may set its
  whole entity record; a `rename-`/`set-<field>`/`move-` kind that emits a full entity record, a whole list, or a whole
  sub-document is `V1-GENERIC-DIFF`. Collections are diffed per id/index (`added`/`removed`/`modified` keyed rows), never
  as a whole-`Vec` replacement.
- **Absorb soundness.** `absorb` must coalesce same-key entries (patch∘patch → one patch, create∘delete → nothing,
  delete∘create → replace), otherwise L3's sum is ill-defined. An unsound `absorb` is a violation of its own (`V5-ABSORB`).
- **Generic seams are deleted, not wrapped.** `diff_from_model`, `graph_edit_diff`, `commit_value_tree_edit`,
  `*_selection_inverse`, `Restore`/`SetSnapshot` inverse variants, `apply_in_place`, whole-snapshot config/window diffs and
  `MutationOutcome::apply_to(&mut P)` are removed; each kind gets its own concrete diff/inverse in its leaf directory.

## Framework API (fixed contract — plugin executors code against this)

In `protocol` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`):

```rust
/// 🔑️ Proof the caller is the central applier. Minted only by `apply_diff`.
#[derive(Clone, Copy, Debug)]
pub struct ApplyCapability { _sealed: () }

pub trait MutationDiff<P>: Clone + Default + PartialEq + ToValue + FromValue + DiffAlgebra<P> {
    fn apply(&self, base: &P, capability: ApplyCapability) -> MutationApplyResult<P>;
    fn absorb(&mut self, other: Self);            // must coalesce same-key entries (V5-ABSORB)
    fn retire_cold(self) where Self: Sized {}
    fn retire_projection(projection: P) { drop(projection); }
}

pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;          // the NEGATIVE diff: inverse(d, base).apply(d.apply(base)) == base
    fn between(base: &P, other: &P) -> Self;      // sync/import only — forbidden in mutation leaves (gate)
    fn is_empty(&self) -> bool;
}

/// 🎯️ THE central applier — the only mint point of `ApplyCapability`.
pub fn apply_diff<P, D: MutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P>;
```

- `MutationOutcome::apply_to(&mut P)` is DELETED. Every caller uses `protocol::apply_diff(outcome.diff(), &base)`.
- A composite diff (enum over sub-diffs) forwards the `capability` it received to its sub-diffs' `apply`; it never mints one.
- `DiffAlgebra` is a supertrait: every diff type implements a concrete `inverse` (negative diff).
- Law helper (in `📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`, exported next to `assert_mutation_inverse_law`):
  `assert_mutation_inverse_sum_law<P: Clone + PartialEq + Debug, M: Mutation<P>>(mutation: &M, base: &P)` asserts, for an
  applied outcome: (1) inverse non-empty when `after != base`; (2) sequential inverse replay restores `base`;
  (3) `Σ = absorb(d₁..dₙ)` satisfies `apply_diff(Σ, after) == base`; (4) `canon(Σ) == canon(d.inverse(base))` where
  `canon(x) = { let mut c = D::default(); c.absorb(x); c }`.
- Every mutation leaf's `🧪️tests` calls `assert_mutation_inverse_sum_law` on each of its applied fixtures.

## Composite mutations ruling

`fold_plan_diff`/`fold_plan_inverse` stay (a composite is by definition a sequence of concrete child mutations); they
build intermediate states only through `protocol::apply_diff`, and the composite inverse is the reversed concatenation of
the children's concrete inverses.

## Build rules for every executor

- Wrap EVERY cargo call: `"$T/🚦️gate.sh" <your-label> -- cargo check ...` (T = this ticket folder, absolute path).
- Foreground only; never `run_in_background`/`Monitor`; never set `CARGO_TARGET_DIR`; prefer
  `cargo check -p <crate> --target wasm32-wasip2 --message-format=short` for plugins, `cargo check -p <crate>` for framework.
- Errors only inside `🧰️framework/**/🖥️host/**` or other peers' crates are not yours — note them and move on.
- "WRITTEN BUT UNVERIFIED" with a precise list is an acceptable report.

## Rulings (wave 2)

- **Position-exact inverses.** The inverse of a delete/remove on an ordered collection restores the item at its original
  index (create/insert kinds carry an optional `index`/`before` anchor; absent = append). Every ordered-collection kind has
  a law fixture that deletes/moves a MIDDLE row, not only the last one.
- **Replace kinds.** A `replace-<entity>` kind may invert to the same kind carrying the base entity value — that is the
  entity's own concrete setter, not a restore inverse.
- **No whole-document mutations.** `set-snapshot`/`patch-snapshot`/`replace-document`-style kinds and every
  `from_snapshot(base, target)`/`replacement(base, target)` snapshot-differencing helper are deleted in every artifact.
  Whole-document import/load is the artifact's genesis/load path (not a mutation, not a history row); a user action that
  replaces many fields becomes the concrete kinds it consists of.
- **Shared keyed list delta.** Norm's `📇️registry/🧬️contract/🪡️list-delta` (keyed added/removed/modified, coalescing absorb,
  randomized sequence test) is the reference shape for keyed collection diffs.

## Rulings (wave 3)

- **Inverse row order.** `inverse(base)` returns rows in the framework's existing store order: replay applies them
  LAST-TO-FIRST (`.rev()`), exactly as `assert_mutation_inverse_law`/`assert_mutation_inverse_sum_law` and the store's
  undo fold do. Every adapter, validator (incl. stdio's shared `validate_snapshot_edit_publication`) and test replays
  inverse rows reversed; nothing replays them in listed order.
- **No feature loss.** Removing snapshot kinds must not remove features: natural-file import is the load/genesis path,
  a details-pane field edit dispatches the concrete kind for that field.
- **Build gate.** `🚦️gate.sh` holds at most `GATE_SLOTS` (default 1) concurrent cargo — 2 and 4 parallel wasm checks both
  deadlocked on fine-grain locking. Never kill another executor's cargo.
- **Foundation status.** The coordinator runs `🔁️foundation-watch.sh` (framework value/replication/os-kernel/plugin, native +
  wasm32-wasip2, every 10 min) → `🗑️generated/coord/foundation.status` (`GREEN <time>` or `RED <time>` + first errors).
  Before any plugin cargo call: `until head -1 "$T/🗑️generated/coord/foundation.status" | grep -q '^GREEN'; do sleep 60; done`
  (foreground, at most 60 min, then report "blocked on foundation"). Never fix a RED foundation yourself unless the errors
  are in files your executor changed.
- **Frozen outcome codes only.** Outcome messages use ONLY the 11 codes in
  `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code` (no per-plugin codes). An insert index past the
  end → `mutation.target-missing`; a duplicate id → `mutation.duplicate-id`; payload invariant → `mutation.invariant`.
  `MutationApplyError` codes match `^mutation\.apply\.[a-z0-9-]+$` (e.g. `mutation.apply.missing-target`,
  `mutation.apply.invalid-add-index`, `mutation.apply.order-mismatch`, `mutation.apply.duplicate-id`) — never `diff.*`.
- **Gate is the burn-down list.** `bun ./📜️script.ts verify mutation-outcome-law` (≈ 10 min) — latest run:
  `🗑️generated/coord/gate-run-2.log`, 999 breaches (from 4152). grep it for your plugin path; every executor ends at 0.
- **Ephemeral roots.** Presence (`👥️presence`) and window transient (`🫧️transient`) lanes are ephemeral and travel by
  whole-root transfer (`ArtifactEphemeralTransferPreparationFactory`). They may keep ONE whole-root setter kind whose diff
  is still sparse (only slots that differ from base) and whose inverse is the same setter carrying the base root — the
  replace-kind ruling. Persisted config/document lanes may not. Gate R13/R14 exempt only files under those two lanes.
- **Snapshot-to-kinds translators are deleted.** Helpers that derive concrete kinds by differencing two snapshots
  (`puzzleNd_snapshot_mutations`, `puzzleNd_document_delta_operations`, norm `replacement`, …) are snapshot differencing
  and are removed; their callers dispatch the concrete kinds of the user action or go through load/genesis.
- **Host-scene reconciliation is a translator.** A renderer/host that reports a whole scene which the plugin diffs against
  the document (puzzle 3d/5d, puzzle 2d `board_snapshot` editing) is snapshot differencing. Hosts report gestures as
  concrete kinds (or gesture events the tool machine turns into concrete kinds); no translator is kept "until migrated".
- **Re-verify before reporting.** The repo's auto-sync (fast-forward merges of `origin/🐙ueli/⛳wip`, `reset: moving to HEAD`)
  has reverted fw-os-leaves' RestoreN→AssignN rename once. Before a final report, re-check with rg that your key changes
  are still on disk and re-run `bun ./📜️script.ts verify mutation-outcome-law` for your paths.

## Rulings (wave 4 — translators, `🔍️audit-translators.md`)

- **Example switch / JSON load / import = load.** `set-active-example`, `load-document-json`, `import-document`, exchange
  import and dev injectors (`set-snapshot`, `set-snapshot-json`) become the artifact's load/genesis effect
  (`Effect::LoadDocument` / reset-document effect): no mutation rows, no history row, no before/after diffing. Delete
  every `replace_document_operations`, `*_document_replacement`, `config_replacement`, `flow_scene_replacement` used for it.
- **Edit gestures emit concrete kinds.** Text edits, details-pane edits (incl. stdio's JSON-pointer
  `setSnapshotValue`/`insertSnapshotValue`/`removeSnapshotValue`/`moveSnapshotValue`/`renameSnapshotKey`), host gestures
  and closure edits resolve the addressed field/entity to ONE concrete kind (or the concrete kinds of the gesture) via a
  per-artifact edit-rules table (norm's `EDIT_RULES`/`NormEditRules` is the reference shape). Deleted: the stdio
  snapshot-edit lane (`snapshot_edit_emit`, `generic_snapshot_edit_expected`, `apply_snapshot_edit`, `snapshot_edit_net(_exact)`,
  `validate_snapshot_edit_publication`), every `net_mutations(base, next)` / `*_net_mutations` / `binary_net_replacement`,
  `host_operations(mutate-then-diff)`, `generation2d_host_snapshot_operations`, `edited_collection_operations`,
  `playbook_edit_blocks_leaves`, `vcs_demo_projection_diff_operations`, docx `xml_replace_document` inside part diffs.
- **Framework.** `whole_document_operation` (plugin trait method) is deleted; flow VCS `begin_replace_document` is a
  load/checkout path, never a history version row.

## Rulings (wave 5 — `🔍️audit-adversarial.md`)

- **AMB-1 positional rows, not order lists.** A diff never carries a whole `order`/`reordered` id list built from the base
  list. Ordered collections use positional rows: `{id, index}` on insert, `{id, from, to}` on move, `{id, index}` on remove
  (the index the inverse reinserts at). Applies to norm `🪡️list-delta`, architect, and every other `order` field.
- **AMB-2 derived data is central.** Derived handles, caches and computed children (mesh handles, tool solids, notation/
  results, presentation minting) are never minted inside a leaf diff. The artifact's `MutationDiff::apply` (the central
  applier's per-artifact step) re-derives them from the applied primary state; diffs carry primary rows only.
- **AMB-3 negative diffs read base.** `DiffAlgebra::inverse` is built by reading base values row by row; it never applies or
  simulates the diff on a copy, never calls `negative`/`state_after`-style simulators.
- **D-01 enforcement.** Rust cannot hide a `pub fn` from crates that depend on it, so L5 is enforced by the gate: no
  `apply_diff`/`ApplyCapability`/`.apply(` on a diff anywhere under `🧬️schema/**` or `🧬️mutations/**` (incl. diff-type
  modules and helpers), in any fn regardless of name; editors/io/store may call `apply_diff`. The gate scans ALL fns and
  helper modules (no name filter, no diff-impl exclusion except the diff type's own `apply` forwarding a received
  capability) and treats renamed `*between*`/`*_replacing`/`negative`/`state_after`/`value_diff*` as R10/R11.
- **`DiffAlgebra::between` is deleted.** Import/example/load are load paths (no diffing) and the framework has no production
  caller, so snapshot differencing has no legitimate seat left. The trait keeps `inverse` + `is_empty`; every `fn between`
  impl and its private helpers (`*Delta::between`, `Patch::between`, `Rows::between`, …) are removed. Sync uses its own
  `FrontierDelta`. Any remaining caller is a translator and becomes concrete kinds or a load.
- **Buffer edits carry their splice.** Text/byte buffer editors (txt, md, binary, wav data, dxf/ply raw text, …) send the edit
  gesture as splices (`{offset, delete, insert}` in the buffer's own unit) taken from the editor's change set; the kind
  carries them verbatim and its inverse is the splice restoring the deleted base bytes. Diffing a whole draft against the
  base buffer is a translator. Loading/importing a new buffer is a load.
- **No compatibility re-exports.** Moving a function means updating every caller; re-exporting it at the old path/name
  (`mutations::apply_*`, `os_spr::fold_*`, old `apply_*_mutation` names) is a compatibility layer and is not allowed.
- **One positional list delta.** The positional keyed list delta (norm-a's `🪡️list-delta`, removed/inserted/moved/modified,
  coalescing absorb, base-reading inverse, randomized sequence test) is promoted ONCE into the framework protocol
  (`protocol::list_delta`); plugin copies (norm registry contract, puzzle 3d `🔨️modules/🪡️list-delta`, any other) are
  deleted and every plugin uses the framework module. API in `📓️exec-fw-spine.md` "List-delta API".
