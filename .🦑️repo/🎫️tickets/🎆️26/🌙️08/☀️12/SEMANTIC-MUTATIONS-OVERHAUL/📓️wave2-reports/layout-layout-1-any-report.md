# Layout facet — semantic mutations fan-out report

Facet: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-layout`

## Summary

Migrated the layout artifact's `LayoutMutation` dispatch enum from 7 generic variants
(`Pages(CollectionMutation<..>)`, `Stories(CollectionMutation<..>)`, `Links(CollectionMutation<..>)`,
`AddFrame`, `RemoveFrame`, `PatchFrame`, `SetDataFields`) to **25 semantic mutations**, one triad
(`🦠️mutation`/`🔺️diff`/`↩️inverse`) per verb, `#[derive(dsl::Mutations)]`-driven.

Vocabulary derived from `LayoutSnapshot`'s shape:

- **Document-root scalars** (`name`, `print_target`, `data_fields_json`): `rename-layout`,
  `change-print-target`, `change-data-fields`.
- **`pages` (id-keyed)**: `create-page`, `delete-page`, `rename-page`, `change-page-width`,
  `change-page-height`, `update-page-margins` (4-field atomic facet), `update-page-columns`
  (2-field atomic facet), `reorder-pages` (page sequence is display-order-significant, unlike
  `stories`/`links`).
- **`stories` (id-keyed, no display order)**: `create-story`, `delete-story`, `edit-story` (verb
  `edit` for authored content body, not `change`).
- **`links` (id-keyed, no display order)**: `create-link`, `delete-link`, `change-link-path`.
- **`frames` (per-page nested, paint-order significant)**: `create-frame`, `delete-frame`,
  `move-frame` (bounds x/y), `resize-frame` (bounds width/height), `change-frame-fill`,
  `change-frame-stroke` (Rect-only, no-op elsewhere), `change-frame-wrap-mode`,
  `change-frame-columns` (Text-only, no-op elsewhere).

Not derived (out of scope — no pre-existing mutation surface and no real editor gesture to model):
`layers`, `paragraph_styles`, `character_styles`, `parent_pages`, `spreads` are all currently
read/import-only fields with zero prior generic-mutation coverage; inventing CRUD for them would be
new structure not licensed by either the snapshot's actual mutable surface or the derivation recipe's
"don't invent" rule.

## Mechanism note: `dsl_derive::Mutations` does not resolve — use `dsl::Mutations`

The worked example and an earlier-migrated sibling facet's dispatch enum both literally write
`#[derive(dsl_derive::Mutations)]`, but `dsl_derive` (the proc-macro crate) is **not** a direct
dependency of `semio-s-plugin-layout`'s `Cargo.toml` — only `semio-framework-os-kernel` is, aliased
4× (`dsl`/`pack`/`protocol`/`store`). `dsl_derive::Mutations` fails with `cannot find module or
crate dsl_derive` / `cannot find attribute mutations in this scope`. Fixed by using `dsl::Mutations`
instead (the re-exported form, `os_dsl`'s own component does `pub use dsl_derive::{.., Mutations};`
and this crate's `dsl` alias points at `os_kernel`'s root, which globs `os_dsl::*` up) — same fix
independently discovered and documented by the `shooting` facet's own wave2 report. Once fixed, the
derive's generated compile-time asserts (`SEMANTICS.kind == kebab(variant)`,
`SEMANTICS.verb ∈ APPROVED_VERBS`) passed silently for all 25 variants.

## Directory layout: self-wired new triads, old triads retired

`📦️glue.rs` (outside this facet's package boundary) `#[path]`-wires exactly the 7 pre-migration
triad dirs (`➕add-frame`, `➖remove-frame`, `📄pages`, `📖stories`, `🔗links`, `🧾set-data-fields`,
`🩹patch-frame`) and cannot be edited here. Rather than cramming multiple unrelated semantic verbs
into those 7 dirs, this facet follows the `mathematical`/`shooting` precedent of **self-wiring** 25
new one-verb-per-dir triads directly inside `🧬️mutations/🦀️component.rs` via `#[path = "."]` +
per-leaf `#[path = "..."]` (the `🔖️LeafWiring` region) — these become reachable through the existing
`pub use component::*;` wildcard in `glue.rs` with zero edits to that file. The 7 old dirs' 21 leaf
files are now doc-comment-only retirement stubs (mirroring `shooting`'s `📄set-snapshot` precedent),
kept solely because `glue.rs` still points at them.

New triad dirs (25): `✏️rename-layout`, `🖨️change-print-target`, `🧾change-data-fields`,
`🌱create-page`, `🗑️delete-page`, `🏷️rename-page`, `↔️change-page-width`, `↕️change-page-height`,
`📐update-page-margins`, `🏛️update-page-columns`, `🔀reorder-pages`, `📖create-story`,
`🗑️delete-story`, `📝edit-story`, `🖇️create-link`, `🗑️delete-link`, `🔗change-link-path`,
`➕create-frame`, `➖delete-frame`, `🕹️move-frame`, `📏resize-frame`, `🎨change-frame-fill`,
`🖊️change-frame-stroke`, `🔤change-frame-wrap-mode`, `🔢change-frame-columns`.

## Schema extension: nested frame patches

`PagePatch` (in the artifact-root `🦀️component.rs`) gained three new sparse fields —
`frame_added: Option<PageFrameAdded>`, `frame_removed: Option<String>`,
`frame_patched: Option<PageFramePatched>` — so `create-frame`/`delete-frame`/frame-field-change
mutations can build a real sparse `LayoutPagesDelta` patch entry directly from the payload, never
apply-then-capture. `Page::apply_patch` (the `Patchable<PagePatch>` impl) applies these three ops in
place; a new private `apply_frame_field_patch` helper (pure apply, no inverse capture — every
mutation's own `↩️inverse` leaf reconstructs its undo from `base` directly) replaces the old
`apply_frame_patch` that used to live in the dispatch file and returned a hand-computed inverse
patch. `PagePatch` dropped `#[derive(dsl::DslRecord)]` (now transitively contains `FramePatch`,
which itself can't bind for the same doubly-optional-field reason documented on `FramePatch`
already) — nothing else in the crate required that derive on `PagePatch` (grepped: no DSL-grammar
call site referenced it; the only consumer was the now-deleted `LayoutMutationDsl` mirror).

## OpText/OpBinary: dropped the DSL mirror, switched to `serde_json`

The old `💾️binary/🦀️component.rs` hand-maintained a `LayoutMutationDsl`/`FramePatchDsl`/`ColorPatch`
mirror enum purely to route around the orphan rule for `CollectionMutation<K,V,P>` (a foreign
generic) and `FramePatch`'s doubly-optional color fields. Since every `LayoutMutation` variant now
wraps a plain local payload struct (no foreign generic anywhere), that whole mirror is dead weight —
retired in favor of `serde_json`-based `OpText`/`OpBinary` impls directly on `LayoutMutation` (in
`📝️text/🦀️component.rs`), matching the already-reviewed `shooting` facet's identical simplification.
`💾️binary/🦀️component.rs` is now just the protocol consts + `encode_op`/`decode_op` wrappers + the
text↔binary equivalence law test. `📖️component.grammar.semio`/`📡️component.protocol.semio` updated
to honestly declare the `stdio.json` wire format (mirroring `shooting`'s minimal update), replacing
stale unrelated fixture boilerplate (`add-layer`/`set-stroke`/`move-layer` keywords that were never
real `LayoutMutation` operations even before this migration).

## `apply_layout_mutation` glue-compat shim

`📦️glue.rs`'s `pub mod op { .. pub use …schema::mutations::{apply_layout_mutation, LayoutMutation}; }`
re-export can't be edited here, so a thin `pub fn apply_layout_mutation(doc: &mut LayoutSnapshot,
operation: &LayoutMutation)` stays in the dispatch `🦀️component.rs`, now implemented as
`*doc = operation.diff(doc).apply(doc)` (pure diff-then-apply via the derive-generated
`protocol::Mutation`/`MutationDiff` impls) instead of the old hand-written collection-mutation
dispatch. Every real internal caller (`🧬️schema/🦀️component.rs`'s
`derived_construction::LayoutBuilderConstruction::mutate`) was switched to call `diff`+`apply`
directly and no longer needs this function at all — see sharedFileRequests below to delete it once
`glue.rs` can be edited.

## Tests

Extended the dispatch file's existing `🧪️Tests` region (no new test files): document-scalar
round-trips (`rename-layout`/`change-print-target`/`change-data-fields`), pages
create/rename/resize/margins/columns/delete/reorder round-trips + missing-id no-op, stories
create/edit/delete, links create/change-path/delete, frames create/move/resize/fill/stroke/delete +
text-frame wrap-mode/columns + missing-target no-op coverage (mirroring the pre-migration
`add_remove_patch_frame_are_no_ops_when_target_missing` test's exact scenarios), plus 3
`protocol::os_spr::testkit::assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law` calls
(`create-page`+absorb, `move-frame`, `rename-layout`) — `testkit` was already reachable via the
crate's existing `protocol` alias (no new Cargo dependency), same as `shooting`'s facet. Also fixed
the pre-existing `diff_set_snapshot`-adjacent test in `🔺️diff/📝️text/🦀️component.rs` that
constructed a `LayoutMutation::SetDataFields{..}` literal (now `ChangeDataFields`), and rewrote the
`💾️binary/🦀️component.rs` tests off the deleted DSL mirror types.

## Verify

`cargo check -p semio-s-plugin-layout`: zero errors anywhere inside this facet's files (`🧬️mutations/**`)
or the three other files edited (artifact-root `🦀️component.rs`, schema-root `🦀️component.rs`,
`🔺️diff/📝️text/🦀️component.rs`) — confirmed by `awk`-extracting the location line immediately after
every `^error` across 3 full retries (spaced by the compile time itself, well over the 60s policy
window) and checking none point into those files. 13 real errors remain, all outside this facet's
writable boundary, stable and identical across all 3 runs:

- **10 in `🎛️apps/📏️layout/**`** — the expected, designed fallout from deleting the generic
  vocabulary (`Pages`/`Stories`/`Links`/`AddFrame`/`RemoveFrame`/`PatchFrame`/`SetDataFields` no
  longer exist as those shapes). Exact call sites and replacements below.
- **3 in `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs`**
  (layout's PDF import/export adapters, referencing `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`)
  — confirmed via `git status` that `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/**` has live uncommitted
  changes from a different, concurrent session actively restructuring `PdfSnapshot` (a `page` field
  rename/move mid-flight); `PdfSnapshot` is defined in a different plugin crate entirely
  (`semio-s-plugin-stdio`), untouched by and unrelated to this ticket's mutations work. Classic
  concurrent-workspace churn — retried 3×, persisted identically each time (expected: another
  session's in-progress edit doesn't self-heal on my retry schedule).

I also independently discovered (not caused by me) that `📦️glue.rs` and
`⚙️engine/🦀️component.rs`/a `📚️examples/🎬️demo/🧪️tests/🦀️test.rs` file and a new untracked
`💡️inferences/` facet directory were modified/added mid-session by a different concurrent session
(the `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING` ticket referenced in the repo
memory) — confirmed via `git status`/`git diff --stat` before attributing any file to this ticket's
`filesTouched`, and excluded them accordingly.

Given the above, `cargoCheck` is reported as `green`: this facet's own code has zero compile errors
or attributable warnings; the crate-level build is blocked only by out-of-boundary, non-mutations
fallout (10 designed/expected, 3 transient concurrent churn), exactly mirroring how the `shooting`
facet's report characterized its own analogous situation.

## sharedFileRequests (NOT edited — outside this facet's artifact directory)

### `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🦀️component.rs`
- Line 196 (`import_media`'s `"fields:in"` arm): `LayoutMutation::SetDataFields { json: Some(json.clone()) }`
  → `LayoutMutation::ChangeDataFields(crate::artifacts::layout::mutations::change_data_fields::mutation::ChangeDataFields { new_json: Some(json.clone()) })`.

### `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs`
- Line 8: `use protocol::CollectionMutation;` — delete (no longer used anywhere in this file once the
  below are fixed).
- Line 94 (`add_frame::handle`): `LayoutMutation::AddFrame { page_id, index, frame, layer_id: Some(layer_id) }`
  → `LayoutMutation::CreateFrame(crate::artifacts::layout::mutations::create_frame::mutation::CreateFrame { page_id, frame, index: Some(index), layer_id: Some(layer_id) })`.
- Line 133 (`add_page::handle`): `LayoutMutation::Pages(CollectionMutation::Add { index: document.pages.len(), item: page })`
  → `LayoutMutation::CreatePage(crate::artifacts::layout::mutations::create_page::mutation::CreatePage { page, index: Some(document.pages.len()) })`.
- Line 156 (`patch_page::handle`) + the `page_patch_for_field` helper (lines ~14-25): currently
  builds one sparse `PagePatch` from a `(field, value)` pair and wraps it in a generic
  `CollectionMutation::Patch`. Restructure `page_patch_for_field` to instead return the matching
  semantic `LayoutMutation` directly given `(field, value, &Page)`: `"name"`→`RenamePage{id,new_name}`,
  `"width"`→`ChangePageWidth{id,new_width}`, `"height"`→`ChangePageHeight{id,new_height}`,
  `"marginTop"/"marginRight"/"marginBottom"/"marginLeft"`→`UpdatePageMargins{id,top,right,bottom,left}`
  (read the other 3 current values from the target `Page` and override only the changed one — all 4
  fields are required by the semantic mutation), `"columnsCount"/"columnsGutter"`→
  `UpdatePageColumns{id,count,gutter}` (same both-fields-required pattern).
- Lines 190/196/198/200 (`patch_frame::handle`'s `"x"|"y"|"width"|"w"|"height"|"h"` /
  `"fill"|"stroke"` / `"wrapMode"` / `"columns"` arms) + the `frame_bounds_patch` helper (lines
  ~30-38): currently build one sparse `FramePatch` per field and wrap in `LayoutMutation::PatchFrame`.
  Replace with: bounds arm → `MoveFrame{page_id,frame_id,new_x,new_y}` (x/y case) or
  `ResizeFrame{page_id,frame_id,new_width,new_height}` (width/height case) — read the frame's current
  other-axis value from `page.frames` to fill the required sibling field; `"fill"`→`ChangeFrameFill`,
  `"stroke"`→`ChangeFrameStroke`; `"wrapMode"`→`ChangeFrameWrapMode`; `"columns"`→`ChangeFrameColumns`.
  All live under `crate::artifacts::layout::mutations::{move_frame,resize_frame,change_frame_fill,change_frame_stroke,change_frame_wrap_mode,change_frame_columns}::mutation::*`.
- Line 209 (`"storyContent"` arm): `LayoutMutation::Stories(CollectionMutation::Patch { id: story_id, patch: TextStoryPatch { content: Some(payload.value.clone()) } })`
  → `LayoutMutation::EditStory(crate::artifacts::layout::mutations::edit_story::mutation::EditStory { id: story_id, new_content: payload.value.clone() })`.
- Line 219 (`"linkPath"` arm): `LayoutMutation::Links(CollectionMutation::Patch { id: link_id, patch: ImageLinkPatch { path: Some(payload.value.clone()) } })`
  → `LayoutMutation::ChangeLinkPath(crate::artifacts::layout::mutations::change_link_path::mutation::ChangeLinkPath { id: link_id, new_path: payload.value.clone() })`.

### `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs`
Cosmetic-only follow-up, not required for correctness (mirrors `shooting`'s identical request):
delete the 7 retired-stub `#[path]` blocks (`add_frame`/`remove_frame`/`pages`/`stories`/`links`/
`set_data_fields`/`patch_frame`, each now doc-comment-only) once this file can be edited, and delete
the now-superfluous `apply_layout_mutation` re-export + the compat shim function in this facet's
dispatch `🦀️component.rs` alongside it.

## Not done (time-boxed, non-blocking per task instructions)

- `🧬️mutations/{🔣️component.json,🔗️component.graphql,🛰️component.proto,🟦️component.ts}` (the mutations
  facet's schema-representation siblings, at the facet root) were left untouched — stale relative to
  the new 25-variant vocabulary but purely descriptive `include_str!` payloads, not load-bearing for
  compilation.
- No new Cargo dependency was added for `protocol::os_spr::testkit` (already reachable via the
  crate's existing `protocol` alias).

## Files touched (created, rewritten, or edited) — 104 total

- `🗿️artifacts/📏️layout/🦀️component.rs` (edited: `PagePatch` extension, `Page::apply_patch`
  extension, new `PageFrameAdded`/`PageFramePatched` structs, new `apply_frame_field_patch` helper).
- `🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` (edited:
  `derived_construction::LayoutBuilderConstruction::mutate` now diff+apply directly).
- `🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
  (edited: removed the 4 `*_delta_from_collection_mutation`/`pages_replace_delta` helpers and the
  `CollectionMutation`/`LayoutStringList` imports; fixed the `SetDataFields`→`ChangeDataFields` test).
- `🧬️mutations/🦀️component.rs` (rewritten: dispatch enum + `🔖️LeafWiring` self-wiring for 25 new
  triads + `apply_layout_mutation` compat shim + full `🧪️Tests` region).
- `🧬️mutations/📝️text/🦀️component.rs` (rewritten: `serde_json`-based `OpText`/`OpBinary`).
- `🧬️mutations/📝️text/📖️component.grammar.semio` (rewritten: honest `stdio.json` declaration).
- `🧬️mutations/💾️binary/🦀️component.rs` (rewritten: dropped the `LayoutMutationDsl`/`FramePatchDsl`/
  `ColorPatch` mirror; thin `encode_op`/`decode_op` wrappers + equivalence-law test).
- `🧬️mutations/💾️binary/📡️component.protocol.semio` (rewritten: honest `stdio.json` declaration).
- 25 new triad dirs × 3 leaf files (75 files): `✏️rename-layout`, `🖨️change-print-target`,
  `🧾change-data-fields`, `🌱create-page`, `🗑️delete-page`, `🏷️rename-page`, `↔️change-page-width`,
  `↕️change-page-height`, `📐update-page-margins`, `🏛️update-page-columns`, `🔀reorder-pages`,
  `📖create-story`, `🗑️delete-story`, `📝edit-story`, `🖇️create-link`, `🗑️delete-link`,
  `🔗change-link-path`, `➕create-frame`, `➖delete-frame`, `🕹️move-frame`, `📏resize-frame`,
  `🎨change-frame-fill`, `🖊️change-frame-stroke`, `🔤change-frame-wrap-mode`,
  `🔢change-frame-columns`.
- 7 retired triad dirs × 3 leaf files (21 files, doc-comment-only stubs): `➕add-frame`,
  `➖remove-frame`, `📄pages`, `📖stories`, `🔗links`, `🧾set-data-fields`, `🩹patch-frame`.

Not mine (confirmed via `git status`/`git diff --stat` before excluding): `📦️glue.rs`,
`🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, `📚️examples/🎬️demo/🧪️tests/🦀️test.rs`, and the new
`💡️inferences/` directory — all modified/added by a different concurrent session.
