# Wave 1 — `draw` facet report

Facet: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-draw`

## Vocabulary derived (14 semantic mutations, 0 generic left)

| Old (generic/mixed) | New semantic mutation | Verb | Notes |
|---|---|---|---|
| `SetLayerVisible` | `SetLayerVisible` | `set` | already the taxonomy's own canonical example — kept as-is |
| `SetLayerLocked` | `SetLayerLocked` | `set` | kept |
| `SetLayerOpacity` | `SetLayerOpacity` | `set` | kept |
| `SetLayerBlendMode` | `SetLayerBlendMode` | `set` | kept |
| `SetLayerName` | `RenameLayer` | `rename` | identity field → `rename`, `new_name` field per taxonomy naming mechanics |
| `SetLayerTransform` | `UpdateLayerTransform` | `update` | one atomic facet (pos+scale+rotation are ONE schema field, never decomposed) — `update` exception |
| `SetFill` | `ReplaceLayerFill` | `replace` | structured tagged-union sub-payload |
| `SetStroke` | `ReplaceLayerStroke` | `replace` | structured sub-payload |
| `SetBooleanOperation` | `SetLayerBooleanOperation` | `set` | renamed for entity-consistent naming |
| `SetTraceParams` | `UpdateLayerTraceParams` | `update` | 2-field facet, always set together |
| `AddLayer` | `CreateLayer` | `create` | full payload + optional (parent, index) |
| `DuplicateLayer` | `DuplicateLayer` | `duplicate` | kept — already matched taxonomy |
| `RemoveLayer` | `DeleteLayer` | `delete` | id-keyed removal, captures cascade |
| `ReorderLayer` | `ReorderLayer` | `reorder` | kept |
| `SetSnapshot` | **deleted, no replacement mutation** | — | banned vocabulary; whole-document replace now goes through `HostEffect::LoadDocument` (see below) |

Every `SEMANTICS.kind` matches its triad-dir stem and its variant's kebab form (derive-enforced
compile-time asserts pass). Entity is `"layer"` for all 14 (verified by a test iterating
`DrawMutation::kinds()`).

## Real handcrafted diffs (no apply-then-capture)

The pre-migration dispatch fell back to `diff_from_snapshot(apply_draw_edit_mutation(...))` (a
whole-snapshot capture — exactly the forbidden pattern) for nested `AddLayer`, `DuplicateLayer`,
`ReorderLayer`, and `SetSnapshot`. Fixing this required extending the diff schema itself:

- `🔺️diff/🦀️component.rs`: `DrawLayersDelta.added` changed from `Vec<DrawLayerNode>` (root-append
  only) to `Vec<DrawLayerAddition>` (`{ parent_id, index, layer }`) — the old shape couldn't express
  a nested insert at all.
- `🔺️diff/📝️text/🦀️component.rs`: `apply_layers_delta` now calls the existing `insert_layer` helper
  (parent-aware) instead of a blind `next.push(...)`; added `diff_create_layer`/`diff_reorder_layer`
  builders (remove+insert-at-address, both real sparse deltas).
- `create-layer`'s diff resolves a `None` index against BASE's real target-list length.
- `duplicate-layer`'s diff/inverse both recompute the duplicate's deterministic (content-addressed)
  id from BASE independently — no captured id needed, matching `engine::clone_draw_layer_node`'s
  existing hash-from-name scheme.
- `reorder-layer`'s diff is a real remove(old id)+insert(new address) sparse delta built from BASE's
  current node.
- `delete-layer`'s inverse reconstructs a real `create-layer` at the exact captured BASE
  `(parent_id, index)`, carrying the full removed subtree.

All 14 leaves' `diff`/`inverse` bodies delegate to their sibling `🔺️diff`/`↩️inverse` files per the
recipe; `MutationKind::inverse` returns `Vec::new()` on a missing target everywhere applicable
(`SetLayerBooleanOperation`/`UpdateLayerTraceParams` also return `Vec::new()` when the layer isn't
the right kind).

## `SetSnapshot` removal — whole-document replace

Per taxonomy: `SetSnapshot` is forbidden with **no replacement mutation**; whole-document
load/replace must go through the store's non-history reset path. Wired this for real:

- `apps/🖍️draw/🎮️commands/📄️artifact/🦀️component.rs`: added `load_document_effect(snapshot)`,
  which builds a fresh `store::ArtifactEnvelope`/`ArtifactVcs` (empty history) and calls
  `store::print_document_pack` to get `{pack, spr}`, then returns
  `Emit { effects: vec![HostEffect::LoadDocument { pack, spr }], .. }` — the same host-owned
  whole-store-swap primitive `apps::space`'s `open_space` command already uses. All four
  `set_snapshot`/`commit_document`/`set_fixture_json`/`set_active_example` command handlers now call
  this instead of constructing a mutation.
- `apps/🖍️draw/🦀️component.rs`: `ArtifactApp::whole_document_operation` override deleted (falls back
  to the framework default `None`), since there is no more `Mutation` vehicle for it. This disables
  the generic `import_media("document:in")` port for draw (nothing in this package exercised that
  port — grepped, no call sites).
- `marquee_select_covers_contained_layer_only` (an existing test that used `SetSnapshot` purely as a
  test-setup convenience) was rewritten to build its two positioned shapes through real dispatched
  commands (`add-layer` + `patch-layer` transform fields) instead of a whole-document swap — same
  test coverage (marquee hit-testing), zero dependency on banned vocabulary.

## Files touched (all inside `✏️s/🔌️plugins/🖍️draw`)

- `🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — rewritten:
  tuple-variant enum + `#[derive(dsl::DslEnum, dsl::Mutations)]`, `draw_op_for_layer_field`/
  `patch_layer_field` kept (rewired to new constructors), old apply/inverse dispatch fns deleted, new
  `#[cfg(test)] mod tests` with 8 `assert_mutation_inverse_law` calls + 1
  `assert_mutation_diff_absorb_law` + 1 descriptor-registration test.
- New triad leaf dirs (`🦠️mutation`/`🔺️diff`/`↩️inverse`, `.rs` only): `✏️rename-layer`,
  `🔄️update-layer-transform`, `🔁replace-layer-fill`, `♻️replace-layer-stroke`,
  `🔀set-layer-boolean-operation`, `🔧update-layer-trace-params`, `🌱create-layer`, `🗑️delete-layer`.
- Rewritten in place (kept slug): `👁️set-layer-visible`, `🔒️set-layer-locked`,
  `🌫️set-layer-opacity`, `🖌️set-layer-blend-mode`, `🧬️duplicate-layer`, `🔃reorder-layer`.
- Deleted dirs: `🏷️set-layer-name`, `↔️set-layer-transform`, `🎨set-fill`, `✏️set-stroke`,
  `🔀set-boolean-operation`, `🖼️set-trace-params`, `➕️add-layer`, `➖️remove-layer`, `🖼️set-snapshot`.
- `🧬️schema/🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs` — `DrawLayerAddition` +
  parent-aware apply/builders (see above).
- `🧬️mutations/📝️text/🦀️component.rs` — re-export list fixed (dropped `apply_draw_edit_mutation`).
- `🧬️mutations/💾️binary/🦀️component.rs` — tests updated to new constructors.
- `📸️snapshot/💾️binary/🦀️component.rs` — test updated to new constructors.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs` — `mutate()` now applies the computed
  diff instead of calling the deleted apply function (strictly more correct — no risk of diff/apply
  divergence).
- `🎛️apps/🖍️draw/🎮️commands/{🗂️layer,👁️view,🖱️canvas,📄️artifact}/🦀️component.rs` — every emit call
  site rewired to the new builders; `📄️artifact` gained `load_document_effect` (see above).
- `🎛️apps/🖍️draw/🦀️component.rs` — `whole_document_operation` override removed;
  `marquee_select_covers_contained_layer_only` test rewritten (see above); one now-unused import
  dropped.
- `📦️packages/🦀️rust/📦️glue.rs` — mutations module wiring updated (renamed/added/removed `pub mod`
  blocks to match); `op`/`mutations` re-export lists fixed.

## Blocked-mechanism

None outstanding. The one genuine risk (whole-document replace needing a non-`Mutation` vehicle) was
resolved entirely with existing framework-public APIs (`HostEffect::LoadDocument`,
`store::print_document_pack`) — no framework file was touched.

## Deferred (not blocking, per the ticket)

- Grammar (`📝️text/📖️component.grammar.semio`) and binary protocol
  (`💾️binary/📡️component.protocol.semio`) docs for the mutations facet still describe the old
  vocabulary — not updated (step g, explicitly non-blocking). Same for the per-triad `.ts` mirror
  files under the new/renamed leaf dirs — none written (only `.rs`); the old leaves' `.ts` files were
  deleted along with their dirs. Nothing in the Rust build depends on these (verified: `cargo check`/
  `cargo test` are clean), but a follow-up pass should backfill them for the TS/grammar consumers.

## Verify

- `cargo check -p semio-s-plugin-draw` — clean (4 pre-existing warnings, none touched by this
  ticket: an unused import in `🎹️composer`, one elided-lifetime lint in the same file, one
  pre-existing unused glob re-export in `glue.rs`, one pre-existing dead field in `⚙️engine`).
- `cargo test -p semio-s-plugin-draw --lib` — **88/88 passed**, 0 failed, including the 9 new law
  tests and the rewritten `marquee_select_covers_contained_layer_only`.
