# 🗒️ Handcrafted mutation fixtures — `note` artifact (33/33)

Tree: `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Wiring: `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs` (33 `#[cfg(test)] mod tests_…` entries,
each inserted directly after that leaf's own `pub mod inverse;`).

Every case carries the hand-authored quartet
(`📸️snapshot/⬅️before`, `📸️snapshot/➡️after`, `🦠️mutation`, `🎯️outcome`) plus its own
`🦀️component.rs`. No `.dsl.semio`/`.pack.semio`/`.op.semio`/`.spr.semio`/`.patch.semio` was forged —
those stay derived (contract D12).

## 📸️ The base snapshot

One hand-designed document, reused as `⬅️before` by all 33 cases because every case needs a real
instance of the entity it addresses and the tree is small enough to hold them all at once.

`schema: "note.document"`, `id: "note-fixture"`, `title: "Field Notes"`; grid `visible/32/4/0.35`,
snap `off/8`, pencil `3`, eraser `12`; assets `{ "asset-logo": image/png "bG9nbw==" 64×64 }`.

Root blocks, in z-order:

| # | id | kind | notes |
|---|----|------|-------|
| 0 | `blk-text` | text | `fontSize 16`, `content.childId = note-text-eea42a3b80b1052b` |
| 1 | `blk-ink` | stroke | 2 points, `strokeWidth 2`, at `(20, 160)` |
| 2 | `blk-table` | table | columns `[A, B]`, rows `[[Alpha, ""], ["", ""]]` |
| 3 | `blk-math` | math | `tex "E = mc^2"`, `displayMode true` |
| 4 | `blk-image` | image | `imageKey "asset-logo"` |
| 5 | `blk-group` | group | children `[blk-nested]` (an image) |

Three deliberate design constraints, each forced by a diff/inverse I read:

- **The text block's `content` handle is the one minted for an EMPTY paragraph list.**
  `edit-block-text`'s inverse recovers the prior paragraphs from the thread-local working-scene
  cache (`note_block_text`), which a snapshot decoded from JSON has never seeded — an uncached read
  returns `vec![]`. Anchoring the base handle at `note_text_child_handle("blk-text", [])` is the only
  base for which that inverse is a true inverse. Both `child_id`s were computed with the repo's own
  `DefaultHasher` recipe under the pinned `nightly-2026-07-07` toolchain:
  `("blk-text", "[]") → note-text-eea42a3b80b1052b`,
  `("blk-text", "[{\"runs\":[{\"text\":\"Hello, note.\"}]}]") → note-text-938222b3522927c6`.
- **The table's trailing row and trailing column are blank.** `remove-table-row`'s inverse is
  `insert-table-row`, which re-appends a *blank* row; `remove-table-column`'s inverse re-appends a
  header lettered `'A' + (columns.len() % 26)` plus a blank cell per row. A non-blank trailing
  row/column would break the inverse law, not the diff.
- **The group holds exactly one child.** Needed so `move-block-to-container` at index 0 has an
  observable "existing child gets pushed right" effect and so `drag-blocks`' subtree recursion is
  visible.

## 🗂️ The 33 cases

| leaf | case name | what the diff does |
|------|-----------|--------------------|
| `🏷️rename-note` | `retitles-the-document` | `title` → `"Field Notes v2"` |
| `👁️change-grid-visible` | `hides-the-grid` | `grid_visible` `true→false` |
| `📏️change-grid-spacing` | `widens-grid-spacing` | `grid_spacing` `32→48` |
| `🔢️change-grid-subdivisions` | `doubles-grid-subdivisions` | `grid_subdivisions` `4→8` |
| `🌫️change-grid-opacity` | `raises-grid-opacity` | `grid_opacity` `0.35→0.75` |
| `🧲️change-snap-enabled` | `enables-snap` | `snap_enabled` `false→true` |
| `📐️change-snap-grid-spacing` | `halves-snap-grid-spacing` | `snap_grid_spacing` `8→4` |
| `✏️change-pencil-width` | `thickens-pencil` | `pencil_width` `3→5` |
| `🧽️change-eraser-radius` | `enlarges-eraser` | `eraser_radius` `12→24` |
| `🆕️create-asset` | `adds-a-second-image-asset` | asset upsert `asset-sketch` |
| `🔁️replace-asset-payload` | `swaps-logo-payload-for-svg` | whole-value swap of `asset-logo` (drops its 64×64) |
| `🗑️delete-asset` | `removes-the-logo-asset` | asset removal; map empties, so serde omits `assets` |
| `➕️create-block` | `inserts-a-photo-block-at-root-index-2` | one `added` entry at `(None, 2)` |
| `❌️delete-block` | `removes-the-math-block` | one `removed` id, from a non-last index |
| `🧺️delete-blocks` | `removes-the-ink-and-image-blocks` | one `removed` list, two non-adjacent ids |
| `🎯️duplicate-block` | `copies-the-math-block-right-after-its-source` | `added` at source index + 1 |
| `👥️duplicate-blocks` | `copies-ink-and-table-with-shifting-indices` | one `added` per pair, base indices + 1 |
| `🚚️move-block-to-container` | `reparents-ink-into-the-callout-group` | `removed` + `added` in one diff |
| `🤏️drag-blocks` | `nudges-ink-and-the-whole-group-subtree` | two `patched` entries via `offset_block_tree` |
| `🔖️rename-block` | `renames-the-table-block` | whole-block patch, only `name` |
| `👀️change-block-visible` | `hides-the-image-block` | whole-block patch, only `visible` |
| `🔒️change-block-locked` | `locks-the-callout-group` | whole-block patch, only `locked` |
| `📍️move-block` | `repositions-the-math-block` | whole-block patch, `x`/`y` |
| `↔️resize-block` | `enlarges-the-image-block` | whole-block patch, `width`/`height` |
| `🔤️change-block-font-size` | `enlarges-the-intro-font` | whole-block patch, `font_size` |
| `📝️edit-block-text` | `replaces-the-intro-paragraphs` | reminted `content` child handle |
| `🧮️edit-block-math` | `replaces-the-tex-with-pythagoras` | whole-block patch, `tex` |
| `🖊️change-block-ink-width` | `thickens-the-sketch-stroke` | whole-block patch, `stroke_width` |
| `🎨️edit-block-ink-stroke` | `redraws-the-sketch-polyline` | whole-block patch, `points` + whole box |
| `⬇️insert-table-row` | `appends-a-blank-third-row` | row sized from current column count |
| `⬆️remove-table-row` | `drops-the-trailing-blank-row` | pops the LAST row |
| `➡️insert-table-column` | `appends-the-lettered-column-c` | header `C` + one blank cell per row |
| `⬅️remove-table-column` | `drops-the-trailing-column-b` | pops the last header + one cell per row |

### 🔬️ Two non-obvious behaviours the fixtures pin down

- **`duplicate-blocks` index skew.** Each `added` entry is pinned to its source's index in the
  BASE, but the entries are inserted sequentially. Duplicating `blk-ink` (root 1) and `blk-table`
  (root 2) yields `[text, ink, ink-copy, table-copy, table, math, image, group]` — the second copy
  lands *ahead* of its own source. That is the committed `after`, and the case asserts it explicitly.
- **`delete-asset` has no block cascade.** `blk-image`/`blk-nested` keep `imageKey: "asset-logo"`
  after the asset is gone. The case asserts the blocks are byte-identical, so a future cascade
  would fail loudly rather than silently.

## 🚫️ Rejection / no-op codes found (read from each `🔺️diff/🦀️component.rs`)

None of the 33 fixtures exercises a rejection — every case is `{"status":"applied"}` — but each
case's `declared_outcome_holds` names, in its doc comment, the guard it steps past. Full inventory:

| leaf | Fatal | Error | Warning |
|------|-------|-------|---------|
| `rename-note` | — | — | `mutation.no-op` (title equal) |
| `change-grid-visible` | — | — | `mutation.no-op` |
| `change-grid-spacing` | `mutation.invariant` (non-finite or ≤ 0) | — | `mutation.no-op` |
| `change-grid-subdivisions` | `mutation.invariant` (non-finite or < 1) | — | `mutation.no-op` |
| `change-grid-opacity` | `mutation.invariant` (non-finite or outside `0..=1`) | — | `mutation.no-op` |
| `change-snap-enabled` | — | — | `mutation.no-op` |
| `change-snap-grid-spacing` | `mutation.invariant` (non-finite or ≤ 0) | — | `mutation.no-op` |
| `change-pencil-width` | `mutation.invariant` (non-finite or ≤ 0) | — | `mutation.no-op` |
| `change-eraser-radius` | `mutation.invariant` (non-finite or ≤ 0) | — | `mutation.no-op` |
| `create-asset` | `mutation.duplicate-id` | — | — |
| `replace-asset-payload` | — | `mutation.target-missing` | `mutation.no-op` (payload equal) |
| `delete-asset` | — | `mutation.target-missing` | — |
| `create-block` | `mutation.duplicate-id`; `mutation.invariant` (container absent / not a group) | — | — |
| `delete-block` | — | `mutation.target-missing` | — |
| `delete-blocks` | — | `mutation.target-missing` (none exist) | `mutation.partial` (some missing) |
| `duplicate-block` | `mutation.duplicate-id` (new id taken) | `mutation.target-missing` (source) | — |
| `duplicate-blocks` | `mutation.duplicate-id` | `mutation.target-missing` (no source exists) | `mutation.partial` |
| `move-block-to-container` | `mutation.invariant` (self-container, or container not a group) | `mutation.target-missing` (block or container) | — |
| `drag-blocks` | — | `mutation.target-missing` (none exist) | `mutation.partial` |
| `rename-block` | — | `mutation.target-missing` | `mutation.no-op` |
| `change-block-visible` | — | `mutation.target-missing` | `mutation.no-op` |
| `change-block-locked` | — | `mutation.target-missing` | `mutation.no-op` |
| `move-block` | `mutation.invariant` (non-finite x/y) | `mutation.target-missing` | `mutation.no-op` |
| `resize-block` | `mutation.invariant` (non-finite or ≤ 0 extent) | `mutation.target-missing` | `mutation.no-op` |
| `change-block-font-size` | — | `mutation.target-missing` (absent **or not a text block**) | `mutation.no-op` |
| `edit-block-text` | — | `mutation.target-missing` (absent **or not a text block**) | none — this leaf has no no-op guard |
| `edit-block-math` | — | `mutation.target-missing` (absent **or not a math block**) | `mutation.no-op` |
| `change-block-ink-width` | — | `mutation.target-missing` (absent **or not an ink block**) | `mutation.no-op` |
| `edit-block-ink-stroke` | — | `mutation.target-missing` (absent **or not an ink block**) | `mutation.no-op` (all five fields equal) |
| `insert-table-row` | — | `mutation.target-missing` (absent **or not a table**) | none |
| `remove-table-row` | — | `mutation.target-missing` (absent **or not a table**) | `mutation.no-op` (1-row floor) |
| `insert-table-column` | — | `mutation.target-missing` (absent **or not a table**) | none |
| `remove-table-column` | — | `mutation.target-missing` (absent **or not a table**) | `mutation.no-op` (1-column floor) |

A wrong-kind target is reported as `mutation.target-missing`, **not** `mutation.invariant` — worth
knowing before anyone writes a rejection fixture for these leaves.

Separately, `NoteDiff`'s own apply layer (`🧬️schema/🔺️diff/🦀️component.rs`) raises a distinct code
family that no mutation leaf produces: `mutation.apply.duplicate-target`,
`mutation.apply.missing-target`, `mutation.apply.conflicting-target`,
`mutation.apply.invalid-value`, `mutation.apply.invalid-target`, `mutation.apply.invalid-index`,
`mutation.apply.invalid-order`.

## ⚠️ Things worth flagging

1. **A rejected note mutation still returns `Ok`.** `MutationOutcome::fatal`/`error` force
   `diff = NoteDiff::default()`, and `apply_note_mutation` is
   `mutation.diff(snapshot).into_parts()` → `MutationDiff::apply`. An empty diff applies cleanly and
   returns the base unchanged, so puzzle5d's `apply(...).is_ok()` probe cannot distinguish
   applied from rejected here. `declared_outcome_holds` therefore inspects the diff outcome's own
   message severities (`Error`/`Fatal`) instead of the apply result. Any future `rejected` note
   fixture must do the same.
2. **`inverse` must be reversed before replay.** The framework law
   (`assert_mutation_inverse_law`) and `delete-blocks`' own inverse comment both require it; the
   puzzle5d reference test does not reverse (harmless there, all its inverses are single-step).
   Every note test reverses, which `delete-blocks` genuinely needs.
3. **`DefaultHasher` stability.** The two `content.child_id` values are only reproducible under the
   same std as the plugin build. They were computed with the repo-pinned `nightly-2026-07-07`
   `rustc` (`1.99.0-nightly (c4af71034 2026-07-06)`) using the exact recipe in
   `note_text_child_handle`. A toolchain bump that changes `SipHash-1-3`'s std wiring would need
   these two constants regenerated.
4. **Not verified by execution.** `cargo` is unusable (peer's in-flight de-async sweep), so no test
   was run. Validation was structural only: `fixtures lint --by-tree` shows the tree at 0 uncovered,
   every `include_str!` target exists, every glue `#[path]` resolves, and `rustfmt --edition 2021`
   parses all 33 test files plus `📦️glue.rs`.
5. **De-async style.** Tests use `#[semio_framework_async_macros::async_test] async fn` with no
   `.await` on any call, matching the committed example tests and the puzzle5d reference.

Authoring aids (temporary, kept per ticket policy):
`🧪️scratch/note-fixtures-base.ts`, `🧪️scratch/note-fixtures-cases.ts`,
`🧪️scratch/note-fixtures-emit.ts`.

---

# 🔺️ Addendum — the serialized diff (`🔺️diff/🔣️component.json`), all 33 cases

Added after the dev's ruling that the serialized diff is the highest-value file in a case. All 33
note cases now carry `🔺️diff/🔣️component.json`, and each of the 33 test files gained the three
required assertions (`produces_committed_diff`, `committed_diff_is_canonical`,
`committed_diff_applies_to_after`), each worded for its own mutation. No case is rejected, so no
`🔺️diff/🚫️component.absent` was needed anywhere in this tree.

## 🧬️ What `NoteDiff` actually serializes to

`NoteDiff` is `#[derive(Serialize, Deserialize)] #[serde(rename_all = "camelCase", default)]` and
**no field carries `skip_serializing_if`** — so every committed diff is a full 23-key object with
`null` in every slot the mutation does not touch. Declaration order (and therefore emitted order):

```
artifact, schema, id, title, blocks,
gridVisible, gridSpacing, gridSubdivisions, gridOpacity,
snapEnabled, snapGridSpacing, pencilWidth, eraserRadius,
assets, linkedArtifact,
selectedBlockIds, activeUtilityId, engagementInput,
cameraX, cameraY, cameraZoom, locale, hoveredBlockId
```

Only three of those 23 are ever written by a note mutation leaf: a document scalar
(`title`/`grid*`/`snap*`/`pencilWidth`/`eraserRadius`), `blocks`, or `assets`. `artifact` (the
whole-document escape hatch), `linkedArtifact`, and the six presence/config slots
(`selectedBlockIds`, `activeUtilityId`, `engagementInput`, `cameraX/Y/Zoom`, `locale`,
`hoveredBlockId`) are `null` in all 33 committed diffs — which is itself the assertion that no
semantic mutation reaches into presence or config state.

Nested shapes, all also `default`-ed with no skips:

- `blocks: NoteBlocksDelta` → `{"added": [], "removed": [], "patched": [], "reordered": null}` —
  all four keys always present.
- `added[i]: NoteAddedBlockEntry` → `{"parentId", "index", "block"}` where `block` is the **real
  nested `NoteBlockNode`**, not a string.
- `patched[i]: NoteBlockPatchEntry` → `{"id", "patch": {"blockJson": "<string>"}}` where
  `blockJson` is `serde_json::to_string(&updated_block)` — a **compact JSON document embedded as a
  JSON string**. Every whole-block leaf (`rename`/`change-*`/`move`/`resize`/`edit-*`/table
  row-column) goes through this one shape.
- `assets: NoteAssetsDelta` → `{"entries": {"<key>": <asset> | null}}`; `null` is a removal.
  `NoteImageAsset` *does* carry `skip_serializing_if` on `width`/`height`, so those are omitted when
  absent — the only skipped fields anywhere in a note diff.

## 🗂️ Per-case diff shape

| leaf | committed `NoteDiff` (non-null slots only) |
|------|--------------------------------------------|
| `rename-note` | `title: "Field Notes v2"` |
| `change-grid-visible` | `gridVisible: false` |
| `change-grid-spacing` | `gridSpacing: 48.0` |
| `change-grid-subdivisions` | `gridSubdivisions: 8.0` |
| `change-grid-opacity` | `gridOpacity: 0.75` |
| `change-snap-enabled` | `snapEnabled: true` |
| `change-snap-grid-spacing` | `snapGridSpacing: 4.0` |
| `change-pencil-width` | `pencilWidth: 5.0` |
| `change-eraser-radius` | `eraserRadius: 24.0` |
| `create-asset` | `assets.entries["asset-sketch"] = {mime, data}` |
| `replace-asset-payload` | `assets.entries["asset-logo"] = {mime, data}` (whole value) |
| `delete-asset` | `assets.entries["asset-logo"] = null` |
| `create-block` | `blocks.added[0] = (parentId null, index 2, blk-photo)` |
| `delete-block` | `blocks.removed = ["blk-math"]` |
| `delete-blocks` | `blocks.removed = ["blk-ink", "blk-image"]` (one list, payload order) |
| `duplicate-block` | `blocks.added[0] = (null, 4, blk-math-copy)` |
| `duplicate-blocks` | `blocks.added = [(null, 2, blk-ink-copy), (null, 3, blk-table-copy)]` |
| `move-block-to-container` | `blocks.removed = ["blk-ink"]` **and** `blocks.added[0] = ("blk-group", 0, blk-ink)` |
| `drag-blocks` | `blocks.patched = [blk-ink, blk-group]` (group's `blockJson` embeds the offset child) |
| `rename-block` | `blocks.patched = [blk-table]` |
| `change-block-visible` | `blocks.patched = [blk-image]` |
| `change-block-locked` | `blocks.patched = [blk-group]` (nested child still `"locked":false` inside) |
| `move-block` | `blocks.patched = [blk-math]` |
| `resize-block` | `blocks.patched = [blk-image]` |
| `change-block-font-size` | `blocks.patched = [blk-text]` (unchanged `content` handle inside) |
| `edit-block-text` | `blocks.patched = [blk-text]` (reminted `content.childId` inside) |
| `edit-block-math` | `blocks.patched = [blk-math]` |
| `change-block-ink-width` | `blocks.patched = [blk-ink]` |
| `edit-block-ink-stroke` | `blocks.patched = [blk-ink]` (points + box in one entry) |
| `insert-table-row` | `blocks.patched = [blk-table]` |
| `remove-table-row` | `blocks.patched = [blk-table]` |
| `insert-table-column` | `blocks.patched = [blk-table]` |
| `remove-table-column` | `blocks.patched = [blk-table]` |

## ⚠️ Further notes on note's diff type

6. **`blocks.patched` is whole-block, not per-field.** Note has no field-level block patch: every
   `change-block-*`/`rename-block`/`move`/`resize`/`edit-*`/table leaf serializes the ENTIRE updated
   node into `blockJson`. So `produces_committed_diff` for those 22 leaves is a byte-exact check on
   a complete block document, which is stronger than compose's `pieces.updated[{piece, diff}]`
   per-field shape but also means every unrelated field of the block is pinned by the committed
   string.
6b. Because `blockJson` is `serde_json::to_string`, the committed string depends on serde's exact
   float rendering (`0.0`, not `0`) and on internally-tagged-enum field order (`"kind"` first, then
   declaration order). Both were reproduced exactly; a change to either would show up as a
   `produces_committed_diff` failure, not a silent pass.
7. **`Option<Option<T>>` is lossy at the JSON boundary.** `title`/`grid*`/`snap*`/`pencilWidth`/
   `eraserRadius`/`hoveredBlockId`/`linkedArtifact` are all double-`Option`. `Some(None)` ("clear
   this field") and `None` ("do not touch this field") both serialize to `null` and both decode back
   to `None`. None of the 33 cases needs `Some(None)`, so every committed diff round-trips exactly —
   but a future "clear the title" fixture would NOT be expressible in this JSON encoding without a
   schema change. Worth flagging to whoever owns `NoteDiff`.
8. **`move-block-to-container` is the only leaf whose diff touches two sub-collections at once**
   (`removed` + `added` in one `NoteBlocksDelta`). Its committed diff is the fixture that proves the
   reparent is atomic rather than two ops.
9. **`create-asset`/`replace-asset-payload` are indistinguishable at the diff level** — both emit
   the same single-key `assets.entries` upsert. Only the `🦠️mutation` payload and the guards
   (`mutation.duplicate-id` vs `mutation.target-missing`/`mutation.no-op`) tell them apart, which is
   why each of those two cases asserts on a different observable: key COUNT growth for `create`,
   dropped `width`/`height` for `replace`.

## ✅️ Re-verification after the addendum

- `bun ./📜️script.ts fixtures lint --by-tree` — the note tree raises **zero** findings and does not
  appear in the uncovered list. Independently re-checked by replaying the script's own `lintCase`
  rules against the note tree alone: 33 leaves, 33 covered, 33 enum variants, **0 errors**.
- All 33 `🔺️diff/🔣️component.json` files parse as JSON; every `include_str!` target (now 5 per
  case, 165 total) exists.
- All 33 glue `#[path]` entries still resolve; `📦️glue.rs` is +99/−0.
- `rustfmt --edition 2021 --emit stdout` parses all 33 test files and `📦️glue.rs`.
- Still no `cargo`: no test was executed and none is claimed to pass.
