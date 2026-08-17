# Wave M — `note/note` mutations facet

## Facet
`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-note`.

## Status
`partial` — implementation complete (all 33 triads authored, dispatch enum + glue.rs rewritten, all
call sites fixed), but **gates were NOT run to a clean, confirmed finish** by this lane. Stopped on
explicit coordinator instruction (build-lock contention across ~10 concurrent lanes); coordinator is
running one consolidated `cargo check`/`cargo test` pass and will requeue failures. See "Known
outstanding / needs re-verification" below for the exact state this was left in.

## Vocabulary derived from `NoteSnapshot`

Root scalars: `title` (identity field), `blocks: Vec<NoteBlockNode>` (id-keyed, z-order-meaningful,
group-nestable tree of 6 kinds: Text/Image/Table/Math/Ink/Group), `grid_visible/grid_spacing/
grid_subdivisions/grid_opacity/snap_enabled/snap_grid_spacing/pencil_width/eraser_radius` (8
document-level tool settings), `assets: BTreeMap<String, NoteImageAsset>` (id-keyed image blobs).

## mutationsCreated (slug → verb → superseded old variant)

| slug | verb | superseded |
|---|---|---|
| `rename-note` | rename | none (new — `title` had no mutation before) |
| `change-grid-visible` | change | `SetGridVisible` |
| `change-grid-spacing` | change | `SetGridSpacing` |
| `change-grid-subdivisions` | change | `SetGridSubdivisions` |
| `change-grid-opacity` | change | `SetGridOpacity` |
| `change-snap-enabled` | change | `SetSnapEnabled` |
| `change-snap-grid-spacing` | change | `SetSnapGridSpacing` |
| `change-pencil-width` | change | `SetPencilWidth` |
| `change-eraser-radius` | change | `SetEraserRadius` |
| `create-asset` | create | `PutAsset` (new-key case) |
| `replace-asset-payload` | replace | `PutAsset` (existing-key case) |
| `delete-asset` | delete | `RemoveAsset` |
| `create-block` | create | `SetBlocks` (append case) |
| `delete-block` | delete | `SetBlocks` (single removal) |
| `delete-blocks` | delete | `SetBlocks` (multi-select removal) |
| `duplicate-block` | duplicate | `SetBlocks` (single clone) |
| `duplicate-blocks` | duplicate | `SetBlocks` (multi-select clone) |
| `move-block-to-container` | move | `SetBlocks` (reparent) |
| `drag-blocks` | drag | `SetBlocks` (nudge/multi-drag) |
| `rename-block` | rename | `SetBlocks` (name patch) |
| `change-block-visible` | change | `SetBlocks` (visible patch) |
| `change-block-locked` | change | `SetBlocks` (locked patch) |
| `move-block` | move | `SetBlocks` (x/y patch) |
| `resize-block` | resize | `SetBlocks` (width/height patch) |
| `change-block-font-size` | change | `SetBlocks` (textSize patch) |
| `edit-block-text` | edit | `SetBlocks` (textContent patch) |
| `edit-block-math` | edit | `SetBlocks` (mathTex patch) |
| `change-block-ink-width` | change | `SetBlocks` (inkWidth patch) |
| `edit-block-ink-stroke` | edit | `SetBlocks` (ink draw/live-drag patch) |
| `insert-table-row` | insert | `SetBlocks` (tableAddRow patch) |
| `remove-table-row` | remove | `SetBlocks` (tableRemoveRow patch) |
| `insert-table-column` | insert | `SetBlocks` (tableAddColumn patch) |
| `remove-table-column` | remove | `SetBlocks` (tableRemoveColumn patch) |

33 mutations total (was 12: `SetGridVisible SetGridSpacing SetGridSubdivisions SetGridOpacity
SetSnapEnabled SetSnapGridSpacing SetPencilWidth SetEraserRadius SetBlocks PutAsset RemoveAsset
SetSnapshot`).

## genericVariantsRemoved
All 12 old variants deleted from `NoteMutation`. `SetSnapshot` has NO replacement — `setActiveExample`/
`setFixtureJson` now build `crate::apps::note::reset_document_effect` (`HostEffect::LoadDocument`,
outside undo history) instead of an `artifact_mutations` entry.

## Vocabulary decisions for audit (per coordinator request)

- **`SetBlocks`** (the old whole-`Vec<NoteBlockNode>` collection setter) does **not** survive
  anywhere — no mutation payload in this facet carries a bare `Vec<NoteBlockNode>` as a "replace the
  whole collection" argument. It decomposed into 21 targeted block mutations: `create-block`/
  `delete-block`/`delete-blocks` (existence), `duplicate-block`/`duplicate-blocks` (copy),
  `move-block-to-container` (hierarchy reparent), `drag-blocks` (relative multi-select offset),
  `rename-block`/`change-block-visible`/`change-block-locked`/`move-block`/`resize-block` (per-field
  scalars, all 6 block kinds), `change-block-font-size`/`edit-block-text` (Text-only),
  `edit-block-math` (Math-only), `change-block-ink-width`/`edit-block-ink-stroke` (Ink-only, the
  latter bundling `points`+bbox as one atomic authored-stroke edit — the one deliberate multi-field
  bundle, see Deviations), `insert-table-row`/`remove-table-row`/`insert-table-column`/
  `remove-table-column` (Table-only). `DuplicateBlocks.blocks: Vec<NoteBlockNode>` is the only
  remaining `Vec<NoteBlockNode>` field anywhere in the facet, and it is NOT a whole-collection
  replace — it's an addressed multi-item **add** (paired 1:1 with `source_ids`, always placed right
  after each source via the diff, never swapped in wholesale for the document's `blocks` field).
- **`PutAsset`** (the old put-synonym upsert) is retired outright — `put` never appears as a verb
  anywhere in the new vocabulary. Assets (`BTreeMap<String, NoteImageAsset>`) are id-keyed entities
  (keyed by a real, addressable string id, not a set-membership flag), so it split along the
  `create`/`replace` line per whether the target key already exists in `base`: **`create-asset`**
  (verb `create`, brand-new key — inverse `delete-asset`) and **`replace-asset-payload`** (verb
  `replace`, existing key's whole image blob swapped — inverse is itself, restoring the prior blob;
  this matches taxonomy rule 2's "`replace-<singular>-<payload>` per large structured field", since
  `NoteImageAsset { mime, data, width, height }` is one inseparable blob, never edited field-by-
  field). `RemoveAsset` → **`delete-asset`** (verb `delete`, exact taxonomy pairing with `create`).
  Call sites (`🎮️commands/🖊️ink/🦀️component.rs`) choose `create-asset` vs `replace-asset-payload` by
  checking `document.assets.contains_key(key)` before dispatching.

## Diff-internals change
`NoteBlocksDelta.added` widened from `Vec<NoteBlockNode>` (root-append-only) to
`Vec<NoteAddedBlockEntry { parent_id, index, block }>` so `create-block`/`duplicate-block(s)`/
`move-block-to-container` can place a node at an exact (possibly nested, possibly non-append)
position — required for `delete-block`'s inverse to restore a deleted block's EXACT original
position (parent + sibling index), not just re-append it. `apply_blocks_delta` updated to route
`added` entries through `engine::insert_block(parent_id, index, block)`. Added
`engine::find_block_location`/`engine::block_locked` helpers. This is diff-internal shape, not a
mutation payload — no wire/text format is affected (`NoteDiff` is plain-serde JSON, not a
`dsl::DslRecord`/`OpText`/`OpBinary` type).

## filesTouched

**Created** (33 triads × {mutation,diff,inverse}.rs + mutation.ts = 132 files) under
`🧬️mutations/{🏷️rename-note,👁️change-grid-visible,📏️change-grid-spacing,🔢️change-grid-subdivisions,
🌫️change-grid-opacity,🧲️change-snap-enabled,📐️change-snap-grid-spacing,✏️change-pencil-width,
🧽️change-eraser-radius,🆕️create-asset,🔁️replace-asset-payload,🗑️delete-asset,➕️create-block,
❌️delete-block,🧺️delete-blocks,🎯️duplicate-block,👥️duplicate-blocks,🚚️move-block-to-container,
🤏️drag-blocks,🔖️rename-block,👀️change-block-visible,🔒️change-block-locked,📍️move-block,
↔️resize-block,🔤️change-block-font-size,📝️edit-block-text,🧮️edit-block-math,
🖊️change-block-ink-width,🎨️edit-block-ink-stroke,⬇️insert-table-row,⬆️remove-table-row,
➡️insert-table-column,⬅️remove-table-column}/`.

**Removed** (12 old dirs, 3 rs + 3 ts files each):
`🧬️mutations/{✏️set-pencil-width,🌫️set-grid-opacity,👁️set-grid-visible,📄set-snapshot,
📏set-grid-spacing,📐set-snap-grid-spacing,📥put-asset,🔢set-grid-subdivisions,🗑️remove-asset,
🧱set-blocks,🧲set-snap-enabled,🧽set-eraser-radius}/`.

**Updated**:
- `🧬️mutations/🦀️component.rs` — dispatch enum: 12 hand-written match-dispatch variants → 33-variant
  `#[derive(dsl::DslEnum, dsl::Mutations)]` list with `#[mutations(...)]`; `apply_note_mutation`
  reduced to `MutationDiff::apply(&mutation.diff(base), base)`; extended `#[cfg(test)]` with law
  tests (see lawTests).
- `⚙️engine/🦀️component.rs` — added `find_block_location` (parent_id+index lookup for exact
  delete/move restore), `block_locked` (symmetry with existing `block_visible`).
- `🧬️schema/🔺️diff/🦀️component.rs` — `NoteBlocksDelta.added` widened (see above); new
  `NoteAddedBlockEntry` struct (no `Default` derive: `NoteBlockNode` has no sensible default).
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_blocks_delta`'s `added` handling routed through
  `insert_block`; old `diff_set_*`/`diff_put_asset`/`diff_remove_asset`/`diff_set_snapshot`/
  `diff_from_snapshot` builders replaced with 5 shared leaf-reused helpers (`note_block_patch_diff`,
  `note_block_added_diff`, `note_block_removed_diff`, `note_asset_upsert_diff`,
  `note_asset_removed_diff`).
- `📸️snapshot/💾️binary/🦀️component.rs`, `🧬️mutations/💾️binary/🦀️component.rs` — test call sites
  (`SetGridVisible`/`SetGridSpacing` literals → `change_grid_visible`/`change_grid_spacing` builders).
- `📦️packages/🦀️rust/📦️glue.rs` — `mutations` block's 12 old `pub mod set_*` mounts replaced with 33
  new `pub mod <slug>` blocks.
- `🎛️apps/🗒️note/🦀️component.rs` — added `reset_document_effect` helper (fem2d-precedent
  `HostEffect::LoadDocument` pattern).
- `🎛️apps/🗒️note/🎮️commands/{🔲️grid,🧲️snap,✏️drawing}/🦀️component.rs` — 8 `Set*` mutation literals →
  `change_*` builder calls.
- `🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs` — `setActiveExample`/`setFixtureJson` rerouted
  to `reset_document_effect`; tests rewritten to drive `handle()` directly and assert on the
  `LoadDocument` effect (mirrors fem2d's own test pattern — `dispatch()` never applies effects).
- `🎛️apps/🗒️note/🎮️commands/🧱️block/🦀️component.rs` — `add_block`/`move_block`/`delete_block`/
  `delete_selection`/`duplicate_blocks` helper/`patch_blocks` all rewritten to construct the new
  granular mutations; `patch_blocks` now routes each (id, field) pair to the one owning mutation
  (batched into one `Emit` per multi-select patch).
- `🎛️apps/🗒️note/🎮️commands/💬️engagement/🦀️component.rs` — `engagement_submit` → `rename_block`.
- `🎛️apps/🗒️note/🎮️commands/🕹️nudge/🦀️component.rs` — `nudge()` → one `drag_blocks` mutation.
- `🎛️apps/🗒️note/🎮️commands/🖊️ink/🦀️component.rs` — `note_ops_from_canvas_events` rewritten:
  per-event `create-block`/`delete-block`, and a `block_update_mutations` field-diff helper that
  collapses each touched id's net before→after change into `move-block`/`resize-block`/
  `edit-block-ink-stroke`/`change-block-ink-width`/`change-block-visible`/`change-block-locked`/
  `rename-block` as applicable; `create-asset`/`replace-asset-payload` per changed asset key.

## sharedFileRequests
None — `📦️glue.rs` is owned by this lane for the whole plugin, edited directly.

## allowlistKeysToRemove
None found seeded for this facet — `note` was in the "untouched" bucket, never allowlisted.
Post-change scan: zero `mutation-migration/semantic-vocabulary` breaches under `✏️s/🔌️plugins/🗒️note`.

## Gates

**NOT RUN to completion by this lane** — stopped on explicit coordinator instruction due to a
shared cargo build-lock contended by ~10 concurrent lanes. Deferred to the coordinator's
consolidated `cargo check`/`cargo test` pass. Do not read anything below as a pass; it is a log of
what this lane observed before stopping, for requeue purposes:

1. `cargo check -p semio-s-plugin-note` — run 3 times over the session:
   - Run 1: blocked entirely by a **foreign** `semio-s-plugin-stdio` compile failure (16 errors,
     all `cannot find object/workflow in subsets` + one `JsonValue::Value` variant-not-found, none
     in files this lane touched) — confirmed unrelated churn from another session, retried per the
     brief's blocked-churn protocol; stdio compiled clean on retry (~6 min later).
   - Run 2: reached this crate's own code. **19 errors, all inside `🗒️note`** — see "Known
     outstanding / needs re-verification" below for the exact list and the fixes applied for each.
   - Run 3: launched to confirm the run-2 fixes, but the coordinator's stop-now instruction arrived
     while it was still compiling (competing with ~10 other lanes' simultaneous `cargo check`
     processes for the same machine's CPU/lock) and was killed before finishing. **Result unknown —
     this is the primary thing to requeue.**
2. `cargo test -p semio-s-plugin-note --lib` — never reached; `cargo check` never confirmed clean.
3. `bun ./📜️script.ts policy` — run twice successfully (this does not touch cargo, no lock
   contention): after the first pass, fixed 22 `taxonomy/emoji-prefix` breaches (new triad dirs
   missing the U+FE0F variation selector — see file list below) and re-ran to confirm 0 remaining
   under `✏️s/🔌️plugins/🗒️note` for that rule. Final state (both runs): **0**
   `mutation-migration/semantic-vocabulary` breaches under `✏️s/🔌️plugins/🗒️note` (repo-wide total
   dropped from 4 pre-existing elsewhere, none of which were ever in `note`). The
   `mutation-migration/triad-completeness`/`artifact-engine`/`artifact-schema/facet-completeness`
   breaches this policy run reports for `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note` are the documented
   wrong-depth policy bug (pre-existing for ~91 other facets too, including already-fully-migrated
   `writer`/`sequence` — not introduced by this lane, not fixed by this lane, out of scope).

## Known outstanding / needs re-verification

The 19 errors from `cargo check` run 2 (verbatim `file:line`), and the fix applied for each — **not
re-confirmed by a completed compile**:

1. `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs:257` —
   `error[E0422]: cannot find struct NoteBlockPatch in this scope`. Fix: added `NoteBlockPatch` to
   that file's `use crate::artifacts::note::schema::diff::{...}` import list.
2. `🧬️mutations/🆕️create-asset/↩️inverse/🦀️component.rs:8` — `cannot find DeleteAsset`. Fix: added
   `use crate::artifacts::note::schema::mutations::DeleteAsset;`.
3. `🧬️mutations/🗑️delete-asset/↩️inverse/🦀️component.rs:9` — `cannot find CreateAsset`. Fix: added
   the matching import.
4. `🧬️mutations/➕️create-block/↩️inverse/🦀️component.rs:8` — `cannot find DeleteBlock`. Fix: added
   import.
5. `🧬️mutations/❌️delete-block/↩️inverse/🦀️component.rs:9` — `cannot find CreateBlock`. Fix: added
   import.
6. `🧬️mutations/🧺️delete-blocks/↩️inverse/🦀️component.rs:14` — `cannot find CreateBlock`. Fix: added
   import.
7. `🧬️mutations/🎯️duplicate-block/↩️inverse/🦀️component.rs:8` — `cannot find DeleteBlock`. Fix: added
   import.
8. `🧬️mutations/👥️duplicate-blocks/↩️inverse/🦀️component.rs:8` — `cannot find DeleteBlocks`. Fix:
   added import.
9. `🧬️mutations/⬇️insert-table-row/↩️inverse/🦀️component.rs:9` — `cannot find RemoveTableRow`. Fix:
   added import.
10. `🧬️mutations/⬆️remove-table-row/↩️inverse/🦀️component.rs:9` — `cannot find InsertTableRow`. Fix:
    added import.
11. `🧬️mutations/➡️insert-table-column/↩️inverse/🦀️component.rs:9` — `cannot find RemoveTableColumn`.
    Fix: added import.
12. `🧬️mutations/⬅️remove-table-column/↩️inverse/🦀️component.rs:9` — `cannot find InsertTableColumn`.
    Fix: added import.
13–14. `🧬️mutations/➕️create-block/🦠️mutation/🦀️component.rs:15,10` — `error[E0277]: NoteBlockNode:
    DslField not satisfied` (`block: NoteBlockNode` field tagged `#[dsl(block)]`; `NoteBlockNode` is
    a `dsl::DslEnum`, which implements `DslVariants` not `DslField` — only `DslRecord`-derived
    struct types get `DslField`). Fix: changed the field to `pub block: Box<NoteBlockNode>` with
    `#[dsl(statements, block)]` (the derive's `RequiredStatements` field kind, which needs a `Box<T>`
    and routes through `DslVariants` instead), boxed inside the `create_block(...)` builder so
    external callers still pass a bare `NoteBlockNode`; `diff.rs`'s `payload.block.clone()` changed
    to `(*payload.block).clone()` to unbox for `note_block_added_diff`'s bare-`NoteBlockNode` param.
15–16. `🧬️mutations/🎯️duplicate-block/🦠️mutation/🦀️component.rs:16,10` — same `DslField` error, same
    fix (`block: Box<NoteBlockNode>`, `#[dsl(statements, block)]`, box in the `duplicate_block(...)`
    builder, unbox in `diff.rs`).
17. `🧬️schema/🔺️diff/🦀️component.rs:72` — `error[E0277]: NoteBlockNode: Default not satisfied` (the
    new `NoteAddedBlockEntry` struct derived `Default`, which requires every field type to impl
    `Default`; `NoteBlockNode` doesn't). Fix: dropped `Default` from `NoteAddedBlockEntry`'s derive
    list and dropped the container's `#[serde(default)]` (nothing constructs it via
    `Default::default()` — every call site fills all 3 fields).
18–19. Two more `DslField`/positions at the same two sites as 13–16 (rustc reports both the field
    declaration and the derive-generated body use-site as separate spans) — same root cause, same
    fix, no separate action needed.

**After applying all of the above, this lane did not get a second `cargo check` run to completion**
(coordinator's stop instruction landed first). A fresh `cargo check -p semio-s-plugin-note` is the
single next step needed to confirm these fixes are sufficient — there could be further knock-on
errors (e.g. other `DslField`-requiring attributes this lane didn't grep for) not yet discovered.
`cargo test -p semio-s-plugin-note --lib` has never been run at all this session — the 30+ new/
extended `#[cfg(test)]` cases (law tests in `🧬️mutations/🦀️component.rs`, rewritten tests in
`🎮️commands/{🧱️block,🖊️ink,🗃️fixture}/🦀️component.rs`) are unverified at runtime, only reviewed by
eye.

## lawTests
Extended `🧬️mutations/🦀️component.rs`'s `#[cfg(test)] mod tests` with `assert_mutation_inverse_law`
coverage for all 33 kinds (root scalars, asset create/replace/delete, block create/delete/delete-many/
duplicate/duplicate-many/reparent/drag, block field scalars, kind-specific content edits, table row/
column ops), plus `assert_mutation_diff_absorb_law` (grid-spacing sequential change), plus
`dispatch_registers_semantic_descriptors` (33 kinds, every verb in `APPROVED_VERBS`), plus explicit
non-last-index delete/undo position-fidelity regression test.
NOT implemented: `assert_diff_algebra_between_law`/`assert_diff_algebra_inverse_law` — `DiffAlgebra`
is a stdio-artifact-standard-only policy (`POLICY_DIFF_ALGEBRA_ALLOWLIST`, `note` is a plugin
artifact, not a stdio standard, so this doesn't apply here).

## Deviations (justified)
- **`create-block`/`duplicate-block(s)` payload carries the fully-formed `NoteBlockNode` (with id
  already assigned)**, never generates ids inside `diff()` — keeps diff/inverse pure functions of
  `(payload, base)`; id/offset/name-suffix generation stays at the command layer (matches
  `sequence`'s `CreateStep` reference pattern exactly).
- **`edit-block-ink-stroke` bundles `points`+`x`+`y`+`width`+`height`** (rule 1's "inseparable
  facet" exception) rather than 5 separate scalar mutations — an ink stroke's geometry is always
  recomputed together as the user draws; the taxonomy explicitly reserves `update`/bundling for
  exactly this "never meaningfully set one-field-at-a-time" case.
- **No `rotate-block`/`group`/`ungroup` mutations minted** despite `rotation` existing on every
  block variant and `Group` being a creatable kind: no command in the plugin currently exposes
  rotation editing or a group/ungroup gesture (confirmed via full `🎮️commands` grep); minting
  mutations with zero call sites would be speculative dead surface. Flagged for a follow-up ticket
  if/when those gestures are added.
- **Table row/column ops assume cells are always blank** (`insert-table-row`'s inverse is exactly
  `remove-table-row` and vice versa) — correct today because no command in the plugin can set
  non-empty cell content (confirmed via grep of `patch_block_field`); would need a captured-content
  inverse if cell editing is added later.
- Schema description files (`📖️component.grammar.semio`, `🔗️component.graphql`, `🔣️component.json`,
  `🛰️component.proto`, `📡️component.protocol.semio`) were **not** rewritten — left as pre-existing
  generic/stale content (the grammar literally describes an unrelated "scene" facet's ops, already
  stale before this change). Same deviation `writer`'s wave-2 report took, for the same reason: no
  round-trip test exists to validate a rewrite against, and getting 33 mutations' worth of grammar/
  protocol/schema descriptions right without one is high-risk for low return in this pass.
