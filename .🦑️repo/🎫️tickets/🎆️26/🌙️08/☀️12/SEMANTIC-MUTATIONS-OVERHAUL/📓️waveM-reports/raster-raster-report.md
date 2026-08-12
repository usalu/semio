# Facet report — `🖨️raster` / `🖨️raster`

## facet

- **plugin**: `✏️s/🔌️plugins/🖨️raster`
- **artifact**: `🖨️raster` (`raster.raster`, document schema `raster.document`)
- **crate**: `semio-s-plugin-raster`
- **mutations dir**: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`

## status

**partial — code complete on disk, gates deferred.** Every code, wiring and schema-description
change is authored and in the working tree. The compile/test gates were NOT run to completion by
this lane (see `gates`); the coordinator's consolidated pass owns verification.

## mutationsCreated

12 triads replace the 5 old hand-written variants. Emoji are unique within the facet.

| slug | dir | verb | payload (address → new-value) | supersedes |
|---|---|---|---|---|
| `create-layer` | `🌱create-layer` | `create` | `parent_id: Option<String>`, `index: usize`, `layer: Box<RasterLayerNode>` | `AddLayer` |
| `delete-layer` | `🗑️delete-layer` | `delete` | `layer_id` | `RemoveLayer` |
| `reorder-layers` | `🔀reorder-layers` | `reorder` | `layer_id`, `parent_id: Option<String>`, `index` | `MoveLayer` (the old one) |
| `rename-layer` | `✏️rename-layer` | `rename` | `layer_id`, `new_name` | `PatchLayer.name` |
| `change-layer-visible` | `👁️change-layer-visible` | `change` | `layer_id`, `new_visible: bool` | `PatchLayer.visible` |
| `change-layer-opacity` | `🌫️change-layer-opacity` | `change` | `layer_id`, `new_opacity: f32` | `PatchLayer.opacity` |
| `change-layer-blend-mode` | `🎨change-layer-blend-mode` | `change` | `layer_id`, `new_blend_mode` | `PatchLayer.blend_mode` |
| `move-layer` | `↔️move-layer` | `move` | `layer_id`, `new_x: f64`, `new_y: f64` | `PatchLayer.transform_x` + `.transform_y` |
| `resize-layer` | `📐resize-layer` | `resize` | `layer_id`, `new_width: u32`, `new_height: u32` | `PatchLayer.width` + `.height` |
| `change-layer-adjustment-kind` | `🎚️change-layer-adjustment-kind` | `change` | `layer_id`, `new_adjustment_kind` | `PatchLayer.adjustment_kind` |
| `add-layer-asset` | `🖇️add-layer-asset` | `add` | `asset_id`, `asset: RasterImageAsset` | (new — see `deviations`) |
| `remove-layer-asset` | `🗂️remove-layer-asset` | `remove` | `asset_id` | (new — inverse partner of the above) |

Ten are the coordinator's mandated derivations; the last two are a justified addition documented
under `deviations`.

### `move` vs `reorder` — the distinction the coordinator asked to have recorded

**Confirmed, and implemented exactly as ruled.** These are two different mutations that never share
a payload, a diff path or an emoji:

- **`🔀reorder-layers` (verb `reorder`) = LIST order.** It carries the old `MoveLayer`'s exact
  addressing (`layer_id` / `parent_id` / `index`) and performs a tree remove-then-insert, moving a
  layer within its sibling list or across into another `Group`. Its inverse is the layer's pre-move
  tree address read from `base` (`locate_layer`). This is what the layer-tree **drag-and-drop
  gesture** now emits — `commands/🖼️layer`'s `move_layer::handle` (the panel command, wire keyword
  still `move-layer`, unchanged for the UI) emits `RasterMutation::ReorderLayers`. A comment in that
  handler states the reason so the mismatch between the command name and the mutation name is not
  read later as a bug.
- **`↔️move-layer` (verb `move`) = SPATIAL position.** Newly minted, addressed by `layer_id` with
  `new_x` / `new_y` writing `transform.x` / `transform.y`. Its inverse is the old transform from
  `base`. It is reachable from the inspector's `transformX` / `transformY` field writes only.

No collision: the two never coexisted, since the old `MoveLayer` variant is fully consumed by
`reorder-layers` and the `↔️move-layer` directory was recreated from scratch for the spatial verb.

## genericVariantsRemoved

- `AddLayer { parent_id, index, layer }` → `create-layer`.
- `RemoveLayer { layer_id }` → `delete-layer`.
- `PatchLayer { layer_id, patch: RasterLayerPatch }` → **the forbidden raw option-bag payload is
  gone**, split into the six real verbs above (`rename` / three `change` / `move` / `resize`).
  `RasterLayerPatch` itself survives ONLY as a diff-internal fragment type
  (`RasterLayerPatchEntry.patch`, `apply_layer_patch`, `patch_layer_in_tree`) — never again as a
  mutation's own payload, which is what the taxonomy's forbidden list permits.
- `MoveLayer { layer_id, parent_id, index }` → `reorder-layers`.
- `SetSnapshot { snapshot }` → **deleted with NO replacement.** Whole-document replace is no longer
  expressible in the enum; it goes through `store::ArtifactStore::reset`.
- The hand-written `impl Mutation<RasterSnapshot> for RasterMutation` and the `apply_raster_mutation`
  / `inverse_raster_mutation` match dispatch are gone; the derive generates both impls. The two
  free functions survive as thin wrappers (still called by `RasterBuilderConstruction::mutate` and
  the mutations text shim) that now forward to `Mutation::diff` + `MutationDiff::apply` and
  `Mutation::inverse` — no match arms left.

`NoMutation` and `CollectionMutation` never appeared in this facet.

### Diff reshaping (required, not optional)

`RasterLayersDelta` was too weak to express the new vocabulary sparsely, so it was reshaped:

- `added: Vec<RasterLayerNode>` → `added: Vec<RasterLayerInsertion { parent_id, index, layer }>`.
  The old shape could only append at the root, which is why the old `AddLayer` diff fell back to
  `diff_from_snapshot(apply(...))` — an apply-and-capture the brief forbids — whenever `parent_id`
  was `Some`. `create-layer`'s diff is now genuinely sparse for nested inserts.
- `reordered: Option<Vec<String>>` (a flat root-only id order) → `moved: Vec<RasterLayerMove { id,
  parent_id, index }>`. The old `diff_move_layer` cloned the snapshot, mutated it and re-diffed the
  whole document; the new one builds one `RasterLayerMove` straight from the payload.
- `MutationDiff::{apply, absorb}` and `apply_layers_delta` updated for both. Apply order inside a
  layers delta is now removed → patched → moved → added, so an insertion index is interpreted
  against the post-move tree.
- Added `diff_add_asset` / `diff_remove_asset` builders for the two asset mutations.
- `diff_set_snapshot` / `diff_from_snapshot` are retained: they are still the honest implementation
  of `RasterDiff.artifact` (whole-artifact replacement), which `ArtifactStore::reset` and the
  composer path use. They are no longer reachable from any mutation.

## filesTouched

### created (36)

Twelve triads × (`🦠️mutation/🦀️component.rs`, `🔺️diff/🦀️component.rs`, `↩️inverse/🦀️component.rs`),
all under `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`:
`🌱create-layer`, `🗑️delete-layer`, `🔀reorder-layers`, `✏️rename-layer`, `👁️change-layer-visible`,
`🌫️change-layer-opacity`, `🎨change-layer-blend-mode`, `↔️move-layer` (mutation + inverse overwritten
in place, diff overwritten in place — dir predates this lane), `📐resize-layer`,
`🎚️change-layer-adjustment-kind`, `🖇️add-layer-asset`, `🗂️remove-layer-asset`.

Plus, in the same mutations dir:
- `🌱create-layer/🦠️mutation/🟦️component.ts` and the same `🟦️component.ts` stub beside each of the
  other ten new triads' `🦠️mutation` leaf (repo convention: `export {};`).
- `📖️component.grammar.semio` (mutations-dir top level — did not previously exist).

### updated (11)

- `…/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum rewritten as
  `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]`
  `#[mutations(snapshot = RasterSnapshot, diff = RasterDiff, schema = "raster.raster")]`, twelve
  single-unnamed-field variants referencing sibling leaves via `use super::<slug>;`. `#[cfg(test)]`
  region extended (see `lawTests`).
- `…/🧬️schema/🧬️mutations/📝️text/🦀️component.rs` — rewritten to the din16798 local-DSL-mirror bridge:
  private `RasterMutationDsl` enum with `#[derive(dsl::DslEnum)]`, hand-written `OpText` +
  `OpBinary` on it, `raster_mutation_to_dsl` / `raster_mutation_from_dsl`, then `OpText`/`OpBinary`
  for `RasterMutation` forwarding through the bridge. (The old code called `dsl::DslVariants`
  directly on the enum, which `dsl::Mutations` does not provide.)
- `…/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — three test fixtures re-expressed as
  `RasterMutation::CreateLayer(...)`.
- `…/🧬️schema/🧬️mutations/{🔗️component.graphql, 🔣️component.json, 🛰️component.proto, 🟦️component.ts}`
  — all four rewritten from the stale `JsonMutation` scaffold to the real twelve-variant vocabulary.
  Proto tags 1..12 assigned in dispatch-enum variant order. `🟦️component.ts` is now a real exported
  union type, not `export {};`.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `RasterLayersDelta` reshaped; `RasterLayerInsertion` and
  `RasterLayerMove` added.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_layers_delta`, `absorb`, `diff_add_layer`,
  `diff_move_layer` rewritten; `diff_add_asset` / `diff_remove_asset` added; two stale unused
  imports dropped.
- `…/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — one test fixture re-expressed as `CreateLayer`.
- `…/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — added `layer_blend_mode` / `layer_transform`
  accessors (needed by the new inverse leaves); deleted `layer_patch_for_field` (the option-bag
  builder, now dead); `raster_append_image_layer` replaced by `raster_image_layer_and_asset`, which
  returns `(asset_id, asset, layer)` instead of a whole replacement snapshot; its test updated.
- `🎛️apps/🖨️raster/🦀️component.rs` — `setSnapshot` / `setActiveExample` rows dropped from
  `app_commands!`; `whole_document_operation` override removed (so the inherited `document:in`
  default correctly reports `NotImplemented` rather than routing through a dead variant);
  `import_media("image:in")` now emits `add-layer-asset` then `create-layer`; manifest
  `.mutation("setSnapshot"/"setActiveExample")` and `.action_args("setSnapshot", …)` removed; the
  three command-vocabulary tests updated (21 → 19 rows).
- `🎛️apps/🖨️raster/🎮️commands/🖼️layer/🦀️component.rs` — every handler re-pointed at the new verbs;
  `layer_patch_for_field` replaced by a local `raster_mutation_for_field` that maps the panel's
  existing wire field names (`name`/`visible`/`opacity`/`blendMode`/`transformX`/`transformY`/
  `width`/`height`/`adjustmentKind`) onto the one real mutation each now means, so no UI call site
  changed; `move_layer::handle` now emits `ReorderLayers`.
- `🎛️apps/🖨️raster/🎮️commands/🗂️document/🦀️component.rs` — both whole-document command payloads
  (`SetSnapshot`, `SetActiveExample`) deleted; file reduced to a docstring explaining that
  whole-document replace now goes through `ArtifactStore::reset` outside the mutation enum.
- `📦️packages/🦀️rust/📦️glue.rs` — the five old triad mount blocks replaced by twelve real ones at
  the same `#[path]`-prefix depth. No inline `#[path = "."]` self-wiring anywhere.

### removed (4 directories, 12 files)

`➕add-layer/`, `➖remove-layer/`, `🩹patch-layer/`, `🖼️set-snapshot/` — all four deleted whole
(each held three stub leaves that merely delegated to the old hand-written dispatch; the three
`🔺️diff` leaves literally read `//! stub per-mutation diff leaf`).

## sharedFileRequests

**None.** Every change is inside `✏️s/🔌️plugins/🖨️raster/`, including this plugin's own
`📦️glue.rs`, which this lane owns exclusively. No framework, `🛢️db`, or cross-plugin file touched.

## allowlistKeysToRemove

All four raster entries currently in `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` (`📜️script.ts`
lines 5791–5794) are now free of `SetSnapshot` / `NoMutation` / `CollectionMutation`, including in
comments and doc-comments — verified by
`grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)" ✏️s/🔌️plugins/🖨️raster --include="*.rs" --include="*.ts"`
returning **zero hits** across the whole plugin:

```
✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎮️commands/🗂️document/🦀️component.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs
```

The two `🖼️set-snapshot/…` keys are for files that **no longer exist** (the directory was deleted),
so they are stale entries regardless of the token scan. `📜️script.ts` was NOT edited by this lane.

## gates

**`cargo check -p semio-s-plugin-raster` — PARTIALLY OBSERVED, treat as NOT RUN.**
One run was started before the coordinator's stop instruction and did finish, reporting
`Finished \`dev\` profile [unoptimized] target(s) in 5m 30s` with
`semio-s-plugin-raster (lib) generated 11 warnings` and **zero errors**. Two caveats that mean this
must not be recorded as a clean gate for the final tree:

1. It predates the final edit to
   `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` (dropping the now-unused `RasterStringList` and
   `locate_layer` imports, which that very run had flagged as warnings). The exact current tree was
   never compiled.
2. All 11 warnings were pre-existing repo-style lints (unused `extern crate vcs`, unnecessary
   qualifications, dead `SEMIO_RASTER_EXAMPLE_TEXT`, etc.), none introduced by this lane — but that
   assessment is from the pre-final-edit run.

**`cargo test -p semio-s-plugin-raster --lib` — NOT RUN.** Started, never reached the test-run
phase, abandoned unfinished on the coordinator's instruction. **No test result of any kind was
observed by this lane.** No claim is made that any test in this facet passes.

**`bun ./📜️script.ts policy` — RUN, observed, clean for this facet.** Completed. The only raster
line in the output is
`artifact-io/sniff-reality  ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs  … declares fn sniff(...) with an underscore-prefixed (unused) parameter`
— a pre-existing, repo-wide finding that fires identically for ~37 other plugins (norm, draw,
stdio, puzzle, block, …), is unrelated to mutation vocabulary, and is untouched by this lane. **No
new high-priority breach kinds.**

**Both compile gates are deferred to the coordinator's consolidated pass** because ~10 lanes are
contending for one shared cargo build lock. Verification of this facet is owed and unpaid.

### blocked-churn

The `UndoGroup { member_edits }` foreign break named in the assignment did **not** manifest: the
`cargo check` run above compiled `semio-framework-plugin` and the whole raster crate without the
expected `error[E0063]`, so the concurrent session had evidently landed its downstream construction
sites before that run. Nothing to report as blocked-churn.

## lawTests

Authored in the dispatch file's existing `#[cfg(test)]` region (no new test file). **Authored, not
observed passing** — see `gates`.

- `every_variant_registers_an_approved_semantic_descriptor` — iterates all 12 via `every_mutation()`,
  asserts `protocol::is_approved_verb` for each and that `SemanticMutation::kinds().len()` equals the
  variant count. (All six verbs used — `create`, `delete`, `reorder`, `rename`, `change`, `move`,
  `resize`, `add`, `remove` — are in `APPROVED_VERBS`, checked by reading that const.)
- `every_variant_round_trips_via_inverse` — all 12 against a base seeded with a pixel layer, an
  adjustment layer and an asset, so no kind hits its missing-target path by accident.
- `assert_mutation_inverse_law` + `assert_mutation_diff_absorb_law` (from
  `protocol::os_spr::testkit`) on three structurally distinct kinds:
  `create_layer_satisfies_the_inverse_and_absorb_laws`,
  `change_layer_opacity_satisfies_the_inverse_and_absorb_laws`,
  `reorder_layers_satisfies_the_inverse_and_absorb_laws`.
- Adapted from the pre-migration tests: `add_remove_layer_round_trip` (was
  `add_remove_patch_layer_round_trip`), `rename_and_change_layer_visible_round_trip` (the patch half,
  now split across two real verbs), `reorder_layer_into_group_round_trip` (was
  `move_layer_into_group_round_trip`), `store_applies_layer_create` (was `store_applies_layer_add`),
  `raster_op_text_round_trips_every_variant` (now loops all 12 rather than two hand-picked cases).
- New: `resize_layer_is_a_graceful_no_op_on_a_group` — asserts `diff == RasterDiff::default()` and
  `inverse().is_empty()` when the addressed layer is not a `Pixel`.
- `set_snapshot_round_trip` deleted; nothing replaces it, per the taxonomy.

## deviations

1. **Two extra mutations beyond the mandated ten: `add-layer-asset` / `remove-layer-asset`.**
   Deleting `SetSnapshot` orphaned the `image:in` media-import path, which was the *only* writer of
   `RasterSnapshot.assets` and did its work by computing a whole replacement document. With no
   whole-document variant and no asset verb, PNG import would have had to become a non-undoable
   `reset` — wrong, since importing an image is a targeted edit, not a document load.
   `assets: BTreeMap<String, RasterImageAsset>` is itself an id-keyed root collection, so
   `📓️derivation-rules.md` rule 2 applies and `add`/`remove` is its taxonomy-correct verb pair.
   `import_media` now emits `add-layer-asset` then `create-layer` in dependency order in one `Emit`.
   Flagging this as a deviation because it exceeds the assigned scope; trivially revertible by
   deleting the two triads and routing `image:in` to `reset` instead.

2. **Optional scope — `mask`, `rotation`, `scale_x`/`scale_y`: DEFERRED, none implemented.**
   Decision recorded as instructed. `replace-layer-mask` (verb `replace`, for the 5-field
   `Option<RasterLayerMask>`), `rotate-layer` (verb `rotate`) and a `scale-layer` (verb `scale`) are
   all genuinely warranted by the snapshot shape and all three verbs are in `APPROVED_VERBS`.
   They were deliberately skipped because (a) the brief marks them explicitly optional and
   non-blocking, (b) none is reachable from any current UI surface — the inspector panel writes only
   the nine fields the old `RasterLayerPatch` covered, so all three would ship as vocabulary with no
   caller, and (c) the assigned ten plus the two asset verbs already left the compile gates unrun.
   Recommend a follow-up ticket that adds them together with the inspector fields that would drive
   them. **Consequence to be aware of: `mask`, `scale_x`, `scale_y` and `rotation` remain writable
   only via `create-layer`'s full-node payload (or `ArtifactStore::reset`) — there is no targeted
   verb for them.**

3. **The `moveLayer` app command keeps its name while emitting `reorder-layers`.** The command id
   (`moveLayer`), its wire keyword (`move-layer`) and its manifest label are all unchanged, so no UI
   or wire consumer breaks, but the mutation it emits is `ReorderLayers`. This deliberate
   name/verb mismatch is commented at the handler. Renaming the *command* to `reorderLayers` would
   be the cleaner end state but is an app-vocabulary change outside this facet's mutation scope.

4. **`RasterLayerPatch` deliberately kept alive as a diff-internal type.** It is no longer any
   mutation's payload (the forbidden pattern), but it remains the fragment type inside
   `RasterLayerPatchEntry` that the six field-level diff leaves construct and that
   `apply_layer_patch` consumes. This is what the taxonomy's forbidden-vocabulary list explicitly
   permits ("option-bags may survive only as diff-INTERNAL types"). Collapsing it into six separate
   delta entries would be a larger diff-shape change with no policy benefit.

5. **`📝️text` / `💾️binary` subdir description files left untouched**, per the assignment's
   out-of-scope note (they describe the generic stdio envelope, not this vocabulary). Only the
   mutations-dir top-level `📖️component.grammar.semio` / `🔗️component.graphql` / `🔣️component.json` /
   `🛰️component.proto` / `🟦️component.ts` were rewritten. The binary-set
   `📡️component.protocol.semio` was likewise not rewritten — the per-variant `record … tag N`
   assignment now lives in the top-level `🛰️component.proto`'s `oneof` (tags 1..12 in variant order,
   append-only); if the coordinator wants the `.protocol.semio` mirror updated too, that is a small
   follow-up.

6. **Per-triad `🟦️component.ts` leaves are `export {};` stubs.** The brief's step 5 allows this
   ("per-triad `.ts` leaves may stay minimal `export {};` stubs matching repo convention"); the
   real types live in the mutations-dir top-level `🟦️component.ts`. Note this contradicts the
   brief's step 6 bullet ("`🟦️component.ts` mirrors must export real types"); resolved in favour of
   the assignment's explicit per-triad allowance. Easy to upgrade if the stricter reading wins.

## incomplete / owed

- **Both compile gates.** `cargo check` was observed clean only on a tree one edit stale;
  `cargo test` produced no result at all. This is the single real gap in this report.
- The snapshot facet's `📸️snapshot/🟦️component.ts` is still the stale generic `JsonSnapshot`
  scaffold, so the mutations `🟦️component.ts` defines a small local `RasterLayerNode` mirror rather
  than importing it. Fixing the snapshot facet's TS is outside this facet's scope.
- Optional mask / rotate / scale verbs, per `deviations` item 2.
