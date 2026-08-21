# 🧪️ Handcrafted mutation fixtures — `📏️layout` (25) and `📐️cad` (20)

Both mutation roots existed at the paths given in the brief; no path correction was needed.

- Layout: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- Cad: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
- Wiring: `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📦️glue.rs` (+75 lines), `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs` (+60 lines) — insert-only, nothing removed.

Every case carries the hand-authored quartet (`📸️snapshot/⬅️before`, `📸️snapshot/➡️after`,
`🦠️mutation`, `🎯️outcome`) plus its own `🦀️component.rs`. No derived `.op/.spr/.dsl/.pack/.patch`
encodings were hand-forged (contract D12 — `fixtures generate` owns those).

Each test file carries four assertions worded for its own mutation:
1. apply → named field-level assertions + whole-snapshot equality against the committed `after`;
2. inverse → the inverse's exact variant AND restored value is asserted before the round trip;
3. canonicality → decode→encode fixed point for both snapshots and the mutation;
4. declared outcome → clean (`messages().is_empty()`) plus a probe of that mutation's OWN diff-field
   shape (e.g. `pages.patched[0].patch.margin_top`, `drawings.values`, `shape_model` inner arm).

---

## 🔠️ Serde shapes (verified from source, not assumed)

The two artifacts do **not** share a mutation wire shape:

| | enum attrs | payload attrs | mutation JSON |
|---|---|---|---|
| `LayoutMutation` | *none* → serde **externally tagged**, variant names verbatim | *none* → **snake_case** fields | `{"RenameLayout":{"new_name":"Renamed Fixture"}}` |
| `CadMutation` | `#[serde(tag = "mutation", rename_all = "camelCase")]` | `#[serde(rename_all = "camelCase")]` on every payload | `{"mutation":"createNode","node":{…}}` |

`dsl::Mutations` is a plain `#[proc_macro_derive]`, so it cannot inject serde attributes; there is no
hand-written `Serialize`/`Deserialize` for either enum. Layout therefore differs from the puzzle5d
reference (which *is* internally tagged + camelCase).

Snapshot-level notes that shaped the JSON:
- `LayoutSnapshot::print_target` has **no** `skip_serializing_if`, so it is emitted as `null`;
  `data_fields_json` / `background_drawing` / `referenced_model` **do**, so they are omitted when absent.
- `Page`, `Frame`, `ImageLink`, `TextStory` carry per-field `#[serde(rename = …)]` (no `rename_all`);
  every `Option` on them is emitted as `null`. `LayoutBounds`/`LayoutRect` rename `width`→`w`, `height`→`h`.
- `Frame` is `#[serde(tag = "kind")]` (`rect`/`text`/`image`).
- `CadSnapshot`'s four model slots are `skip_serializing_if = "Option::is_none"`, so a vacated slot
  key **disappears** from the `after` JSON. `drawings` is always emitted.
- All `f32` colour components use exactly-representable values (`0.0/0.25/0.5/0.75/1.0`) so the
  canonicality test's `f32 → f64 → JSON` round trip is a true fixed point.

---

## 📏️ Layout — base snapshot

One shared base (`schema: "layout.layout"`, `name: "Fixture Layout"`), small but complete enough that
every mutation's target exists in it:

- `grid` `{12.0, 0.0, true}`; `paragraphStyles`/`characterStyles`/`parentPages`/`spreads` empty.
- `stories`: `story-1` "Alpha body.", `story-2` "Spare body." (story-2 is the delete target — it is last,
  so `create-story`'s append-at-end inverse restores the original order exactly).
- `links`: `link-1` (`alpha.png`, `hash-alpha`, 800×600@300), `link-2` (delete target, last).
- `pages`: `page-1` "Cover" 200×300 with layer `layer-1` (`objectIds: [frame-rect, frame-text]`) holding
  `frame-rect` (Rect, bounds 20/30/60×40, fill white, stroke null) and `frame-text` (Text, storyId
  `story-1`, columns 1, wrapMode `box`); `page-2` "Spare" (empty, last → delete/reorder target).
- `printTarget: null`, `dataFieldsJson`/`backgroundDrawing`/`referencedModel` absent.

| mutation leaf | case | what the `after` proves |
|---|---|---|
| `✏️rename-layout` | `renames-the-document` | root `name` only |
| `🖨️change-print-target` | `sets-a-cmyk-print-target` | `null` → `"cmyk-coated"` |
| `🧾change-data-fields` | `attaches-a-data-fields-payload` | opaque blob stored verbatim, unparsed |
| `🌱create-page` | `appends-page-3` | whole `Page` (margins/columns/layers) appended |
| `🗑️delete-page` | `removes-page-2` | id removed; inverse recreates the FULL record at index 1 |
| `🏷️rename-page` | `renames-page-1` | `PagePatch.name` only, geometry untouched |
| `↔️change-page-width` | `widens-page-1` | `width` 200→240, `height` pinned |
| `↕️change-page-height` | `lengthens-page-1` | `height` 300→360, `width` pinned |
| `📐update-page-margins` | `sets-asymmetric-margins-on-page-1` | all four edges atomically (12/18/24/6) |
| `🏛️update-page-columns` | `splits-page-1-into-three-columns` | `count`+`gutter` atomically (3/12.0) |
| `🔀reorder-pages` | `moves-page-1-behind-page-2` | complete final id order emitted, no record edited |
| `📖create-story` | `appends-story-3` | `TextStory` + its (empty) style runs |
| `📕delete-story` | `removes-story-2` | no cascade into the text frame that threads a story |
| `📝edit-story` | `rewrites-story-1-body` | `content` replaced, `styleRuns` untouched |
| `🖇️create-link` | `appends-link-3` | whole `ImageLink` incl. hash/dpi |
| `✂️delete-link` | `removes-link-2` | no cascade into frames |
| `🔗change-link-path` | `relinks-link-1-to-a-new-file` | `path` only — hash/px/dpi stay **stale on purpose** |
| `➕create-frame` | `inserts-a-rect-frame-at-index-1` | inserts at payload index INSIDE the page but APPENDS the id to `layer.object_ids` |
| `➖delete-frame` | `removes-the-text-frame-and-its-layer-membership` | frame dropped AND unregistered from every layer |
| `🕹️move-frame` | `moves-the-rect-frame` | `bounds.x/y` only |
| `📏resize-frame` | `resizes-the-rect-frame` | `bounds.w/h` only, origin stays anchored |
| `🎨change-frame-fill` | `repaints-the-rect-frame-fill` | Rect-only `fill`, stroke untouched |
| `🖊️change-frame-stroke` | `adds-a-stroke-to-the-rect-frame` | Rect-only `stroke` null→black; inverse re-clears |
| `🔤change-frame-wrap-mode` | `switches-the-text-frame-to-column-wrap` | Text-only `wrapMode`, columns untouched |
| `🔢change-frame-columns` | `splits-the-text-frame-into-two-columns` | Text-only `columns`, wrapMode untouched |

### Layout rejection / no-op codes found in the `🔺️diff` builders

| code | severity | leaves that emit it |
|---|---|---|
| `mutation.no-op` (applied, empty diff, `warn`) | Warning | `rename-layout` (same name) · `change-print-target` · `change-data-fields` · `rename-page` · `change-page-width` · `change-page-height` · `update-page-margins` · `update-page-columns` · `reorder-pages` (already at position) · `edit-story` · `change-link-path` · `change-frame-fill` (Rect already that fill) · `change-frame-stroke` · `change-frame-wrap-mode` · `change-frame-columns` |
| `mutation.target-missing` | Error | `delete-page` · `rename-page` · `change-page-width` · `change-page-height` · `update-page-margins` · `update-page-columns` · `reorder-pages` · `delete-story` · `edit-story` · `delete-link` · `change-link-path` · `create-frame` (missing page) · `delete-frame` (missing page **or** missing frame) · `move-frame` · `resize-frame` · `change-frame-fill/stroke/wrap-mode/columns` (missing page or frame) |
| `mutation.duplicate-id` | Fatal | `create-page` · `create-story` · `create-link` · `create-frame` (frame id already on that page) |
| `mutation.invariant` | Fatal | `move-frame` (non-finite x/y) · `resize-frame` (non-finite **or** non-positive w/h) |

Variant-mismatch behaviour that is **not** a coded rejection: `change-frame-fill`/`-stroke` against a
non-Rect frame and `change-frame-wrap-mode`/`-columns` against a non-Text frame produce a diff that
`apply_frame_field_patch` silently ignores, and their `↩️inverse` returns `Vec::new()`. Ditto
`create-frame` with a `layer_id` that matches no layer: the frame lands, the layer list is not updated.
Apply-time (not diff-time) codes reachable from a malformed pages/stories/links delta:
`mutation.apply.missing-target`, `mutation.apply.duplicate-target`, `mutation.apply.incomplete-diff`.

---

## 📐️ Cad — base snapshot

One shared base (`schema: "cad.document"`, `id: "cad-fixture"`), with **all four fixed model slots
occupied** so that each `delete-*-model` has something to vacate and each `create-*-model` exercises
its real (documented) overwrite path:

- `shapeModel` `shape-model-1` → `cad-shape-1!s.stdio.semio@v1/model`; `buildingModel`
  `building-model-1`; `energyModel` `energy-model-1`; `structureClassicModel` `structure-classic-model-1`.
- `drawings`: one handle `drawing-1` → `cad-drawing-1!s.stdio.semio@v1/drawing`.
- `referencesByModelDefinitionId`: `{"spatial.shape": [ref-1]}` where `ref-1` is
  `plan.png` / `image` / origin `[0,0,0]` / orientation `null` / scale `1.5` / widthWorld `8.0` /
  hidden `false` / locked `true` / opacity `0.5`.
- `nodes`: `node-1` "Root" (group), `node-2` "Base Plate" (solid — the delete target, last).
- `activeModelDefinitionId`: `"spatial.shape"`.

| mutation leaf | case | what the `after` proves |
|---|---|---|
| `🧱create-shape-model` | `rehandles-the-occupied-shape-slot` | create **overwrites** an occupied fixed slot; inverse is another `create-…` carrying the DISPLACED handle |
| `🧨delete-shape-model` | `vacates-the-shape-slot` | slot key disappears; siblings untouched |
| `🏢create-building-model` | `rehandles-the-occupied-building-slot` | same, `building_model` |
| `💥delete-building-model` | `vacates-the-building-slot` | same |
| `⚡create-energy-model` | `rehandles-the-occupied-energy-slot` | same, `energy_model` |
| `🔌delete-energy-model` | `vacates-the-energy-slot` | same |
| `🏛create-structure-classic-model` | `rehandles-the-occupied-structure-classic-slot` | same, `structure_classic_model` |
| `💣delete-structure-classic-model` | `vacates-the-structure-classic-slot` | same |
| `📐️create-drawing` | `appends-drawing-2` | Vec composition grows; diff is the WHOLE post-state list |
| `🧹delete-drawing` | `removes-drawing-1` | whole-list diff, empty here; inverse carries the escrowed target URI |
| `➕create-node` | `appends-node-3` | whole `CadNode` (label + kind) appended |
| `🗑delete-node` | `removes-node-2` | id removed; inverse recreates label AND kind |
| `🏷rename-node` | `relabels-the-root-node` | `label` only — `CadNodePatch` has no `kind`, so type can never drift |
| `👁change-reference-hidden` | `hides-the-shape-reference` | `hidden` flips, `locked` pinned |
| `🔒change-reference-locked` | `unlocks-the-shape-reference` | `locked` flips, `hidden` pinned |
| `📏change-reference-width` | `widens-the-shape-reference-plane` | `widthWorld` 8→12, uniform `scale` pinned |
| `📍move-reference` | `moves-the-shape-reference-off-origin` | 3-component origin written as a unit |
| `🖇replace-reference-media` | `reattaches-the-shape-reference-to-a-new-plan` | url/kind/scale/opacity atomically; a **`null` `new_orientation` LEAVES the existing orientation, it does not clear it** |
| `📎replace-references` | `swaps-the-shape-reference-list` | wholesale bucket substitution — `ref-1` vanishes because the payload omits it |
| `🎯change-active-model-definition` | `switches-the-active-pane-to-the-building-model` | one root string; no bucket is migrated or conjured |

### Cad rejection / no-op codes found in the `🔺️diff` builders

| code | severity | leaves that emit it |
|---|---|---|
| `mutation.no-op` (applied, empty diff, `warn`) | Warning | `create-shape-model` / `create-building-model` / `create-energy-model` / `create-structure-classic-model` (slot already holds an IDENTICAL handle) · `delete-shape-model` / `delete-building-model` / `delete-energy-model` / `delete-structure-classic-model` (slot already empty) · `rename-node` · `change-reference-hidden` · `change-reference-locked` · `change-reference-width` · `move-reference` · `replace-reference-media` (all five media fields already equal) · `replace-references` (bucket already equal) · `change-active-model-definition` |
| `mutation.target-missing` | Error | `delete-drawing` · `delete-node` · `rename-node` · `change-reference-hidden` · `change-reference-locked` · `change-reference-width` · `move-reference` · `replace-reference-media` (all reference verbs report `[model_definition_id, reference_id]` as the target path) |
| `mutation.duplicate-id` | Fatal | `create-drawing` · `create-node` |
| `mutation.invariant` | Fatal | `change-reference-width` (non-finite or ≤ 0) · `move-reference` (any non-finite origin component) |

Note the ordering inside `change-reference-width`: the **target-missing check runs first**, then the
`mutation.invariant` finiteness check, then the no-op check. `move-reference` uses the same order.
`replace-references` has **no** target-missing arm at all — an unknown `model_definition_id` simply
creates that bucket. Apply-time codes reachable from a malformed nodes delta:
`mutation.apply.missing-target`, `mutation.apply.duplicate-target`, `mutation.apply.conflicting-target`,
`mutation.apply.invalid-order`.

---

## ✅️ Verification performed (no `cargo` — the workspace is broken by a peer's de-async sweep)

1. `bun ./📜️script.ts fixtures lint --by-tree` from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`:
   both trees are gone from the uncovered list; no error finding names layout or cad. (The one
   `Cad: enum variant has no mutation directory` error is in stdio's `✳️any` tree and predates this work.)
2. All 180 `include_str!` targets across the 45 new test files resolve to real files.
3. Every `#[path]` in both `📦️glue.rs` files resolves (272 layout / 263 cad entries, 0 missing);
   the diff is insert-only (135 added lines, 0 removed).
4. `rustfmt --edition 2021 --emit stdout` parses all 45 test files and both `📦️glue.rs` files.

**No test is claimed to pass** — the crates were never compiled.

## ❓️ Could not determine

- Whether the generated `.op.semio` / `.spr.semio` / `.patch.semio` encodings will round-trip, since
  `fixtures generate` needs a compiling workspace. The `--by-tree` run reports the expected
  derived-encoding warnings for every new case.
- Whether the de-async sweep will leave `#[semio_framework_async_macros::async_test] async fn` in
  place or reduce it to a plain `#[test] fn`. These fixtures copy the puzzle5d reference exactly
  (async fn, **no `.await` on any call**), which is also what every sibling test in both trees does
  today, so they move with the sweep either way.

---

# 🔺️ Follow-up: the committed `🔺️diff/🔣️component.json` (all 45 cases)

`🔺️diff/🔣️component.json` is now a required core file (`CORE_CASE_FILES` in the puzzle plugin's
`📜️script.ts`). All 45 cases gained one, transcribed by hand from that mutation's own
`🔺️diff/🦀️component.rs`, and each test file grew from four assertions to seven
(`produces_committed_diff`, `committed_diff_is_canonical`, `committed_diff_applies_to_after`), each
worded for its own mutation.

Every committed diff has **exactly one populated top-level field**; everything else is explicit
`null`. That sparseness is the point: it pins which collection each verb may touch, so a mutation
that reached the right end state by rewriting the whole snapshot would fail `produces_committed_diff`
even though `applies_to_committed_after` still passed.

## 🔠️ Diff-type serde shapes (verified from source)

Both `LayoutDiff` and `CadDiff` carry `#[serde(rename_all = "camelCase", default)]` with **no**
`skip_serializing_if` anywhere, so serde emits every field: **27** for `LayoutDiff`, **47** for
`CadDiff`. The `default` only affects deserialization.

**Layout's diff wire shape is mixed-case.** `LayoutDiff`, `LayoutPagesDelta`/`LayoutStoriesDelta`/
`LayoutLinksDelta` and the `*PatchEntry` records are `camelCase`, but the records nested one level
deeper carry **no `#[serde(rename_all)]` at all** and therefore stay **snake_case** on the wire:

| nested record | wire fields |
|---|---|
| `PagePatch` | `name`, `width`, `height`, `margin_top`, `margin_right`, `margin_bottom`, `margin_left`, `columns_count`, `columns_gutter`, `frame_added`, `frame_removed`, `frame_patched` |
| `FramePatch` | `x`, `y`, `width`, `height`, `fill`, `stroke`, `wrap_mode`, `columns` |
| `PageFrameAdded` | `frame`, `index`, `layer_id` |
| `PageFramePatched` | `frame_id`, `patch` |
| `TextStoryPatch` / `ImageLinkPatch` | `content` / `path` |

So a single layout diff document contains `"pages" → "patched" → "patch" → "frame_patched" →
"frame_id"`: three camelCase levels then two snake_case ones. These records also lack
`#[serde(default)]`, so **every** field is required on decode — the committed JSON spells all of them
out. Cad has no such split: `CadNodesDelta`, `CadNodePatchEntry`, `CadNodePatch` and
`CadDrawingChildList` are all `camelCase`.

## ⚠️ `Option<Option<T>>` does not survive JSON — cad's four `delete-*-model` cases

`CadDiff::{shape,building,energy,structure_classic}_model` are `Option<Option<CadModelChild>>` with a
plain serde derive. `Some(None)` ("vacate the slot") and `None` ("leave the slot alone") **both**
serialize to `null`, and JSON `null` deserializes back to the **outer** `None`. `CadDiff::apply` does
distinguish them (`if let Some(value) = &self.shape_model { next.shape_model = value.clone(); }`), so:

- the committed diff for `delete-shape-model` and its three siblings is **all-null — byte-identical to
  `CadDiff::default()`**;
- `produces_committed_diff` and `committed_diff_is_canonical` still hold (both sides render `null`);
- but the JSON-decoded diff is **inert**: it does not carry `before` to `after`.

These four are the first fixtures in the repo to reach this field shape — neither puzzle5d nor
shooting has a diff builder that emits `Some(None)` (`grep -rn ": Some(None)"` over both trees:
zero hits). Rather than assert something false, their `committed_diff_applies_to_after` pins the hole
explicitly: it asserts the **in-memory** diff carries `before`→`after`, that the decoded diff equals
`CadDiff::default()`, and that applying it is a no-op — with a docstring saying that fixing the wire
shape (a `double_option` helper, or `skip_serializing_if` so an untouched slot is *omitted* rather
than `null`) must flip the test back to the plain form the other 41 cases use.

Layout is unaffected: its double-optional fields (`print_target`, `data_fields_json`, `FramePatch::
fill`/`stroke`) are only ever set to `Some(Some(v))` by the fixtures, which renders unambiguously.

## ✅️ Verification (re-run; still no `cargo`)

1. `fixtures lint --by-tree`: `📏️layout` and `📐️cad` raise **zero** error findings and appear on no
   uncovered row. (The 41 errors in the run are all `stdio/✳️semio/✳️any` `enum variant has no
   mutation directory` — another session's tree.)
2. `rustfmt --edition 2021 --emit stdout`: parses all 45 test files (each now exactly 7 assertions)
   and both `📦️glue.rs`.
3. `include_str!`: 225/225 targets resolve (5 per file); all 45 `🔺️diff/🔣️component.json` present,
   with exactly 27 (layout) / 47 (cad) top-level fields and exactly one populated field each.
4. **Offline stand-in for `committed_diff_applies_to_after`** — `🧪️scratch/fixtures-layout-cad/simulate_apply.py`
   re-implements both `MutationDiff::apply` functions (transcribed from each plugin's
   `🔺️diff/📝️text/🦀️component.rs`, including `apply_identified_delta`'s remove→append→patch→reorder
   order, `PagePatch`'s frame add/remove/patch fragments and their layer cascade, and cad's per-key
   reference merge) and confirms **45/45** committed diffs carry `before` → `after`. The four cad
   vacate cases are checked through the in-memory `Some(None)` arm, per the hole above.

Still **no test is claimed to pass** — the crates were never compiled.
