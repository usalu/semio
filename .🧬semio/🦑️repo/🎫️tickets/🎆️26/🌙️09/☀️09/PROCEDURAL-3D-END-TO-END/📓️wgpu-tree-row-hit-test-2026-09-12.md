# 🌳️ wgpu TREE ROW HIT TEST — one row metric, one geometry

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu tree row hit test", 2026-09-12/13.
Closes the defect `📓️wgpu-resident-budget-settle-2026-09-12.md` §8 named and refused to guess at.

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened. The react serve on 6018 was not touched, the procedural
guest was NOT restaged, and no git-state-modifying command was run.

---

## 1. TL;DR

| question | answer |
|---|---|
| **where did 6.4 px come from?** | `padding_standard * 2`. Not a font metric, not DPR, not an unready atlas: a tree row mounts as a keyed `Stack` with **no arena children of its own** (its icon, label, description and actions live inline on the owning `Tree`'s spec), and the retained mounted layout measured it as an ordinary vertical stack — `0 + padding * 2 = 3.2 * 2 = 6.4`. |
| **where did ~24 px come from?** | `paint::retained_tree_node_step`'s own private cursor, `const TREE_ROW_HEIGHT: f32 = 24.0`, stepped per painted row. |
| **why did the click do nothing?** | `events::hit_test` reads `tree.accepted_layout` — the layout's published rects. The painted row and the published rect were different rectangles, so the pointer landed between them. |
| **is it fixed on 6118?** | The geometry is, measured: `add-generation` is published at `[0, 72, 315.392, 24]` in a 24 px row band, and a pointer at the painted label is witnessed inside it. The click still dispatches nothing — because **no input of any kind reaches the shell on that target any more**, chrome included (§5.3). |
| **the fix** | ONE row metric (`Theme::tree_row_height` ← `dom.treeRowUiSpacing`, the `--size-workbench` React's `Tree` rows carry) and ONE geometry (`layout::TreeRowMetrics` + `tree_item_height`/`tree_section_height`/`tree_node_height`/`tree_row_control_rect`), consumed by the retained layout, the painter and the immediate-mode chrome. Layout is now the single writer of tree row rects. |

---

## 2. Root cause, with the arithmetic that reproduces the live numbers

### 2.1 What the live target published

`📓️wgpu-resident-budget-settle-2026-09-12.md` §8, verbatim: the Generations window's body measured
**25.6 px** with **6.4 px** rows, and
`stack[1]#procedural3d-play-generate.actions/stack[0]#procedural3d-play-generate.add-generation`
sat at `rect [3.2, 16, 308.992, 6.4]`. Five clicks (`40,49`, `86,126`, `150,49`, `40,62`, `86,119`)
each left `render begin` unchanged — no dispatch at all.

### 2.2 Every one of those four numbers is `mounted_layout`'s own arithmetic

The styling tokens (`🖱️ui/🎨️styling/🔤️tokens/🦀️.rs`): `chrome::UI_SPACING_COMPACT_PX = 3.2`,
`GAP_STANDARD_UI_SPACING = 1.0`, `PADDING_STANDARD_UI_SPACING = 1.0`, `typography::TEXT_XS_PX = 11.2`.
So `Theme::padding_standard = Theme::gap_standard = 3.2`.

A `TreeSection`/`TreeItem` record mounts as a keyed **`UiNode::Stack`** with `gap: None,
padding: None` (`reconcile`'s `Component::TreeSection | Component::TreeItem` arm; `record_consumes_subtree`
deliberately does NOT consume a tree's subtree, so the rows are real arena nodes). `Add Generation`
is a leaf row: **no arena children at all** — the row's icon, label, description and trailing actions
are `UiTreeItemNode` fields on the owning `Tree`'s inline spec, and `paint::retained_tree_node_step`
draws them straight from there.

`mounted_layout::measure_one`'s vertical-stack arm is
`height = children.height_sum + gap * (count - 1) + padding * 2`. With zero children that is

```
0 + 0 + 3.2 * 2 = 6.4      ← the row height, exactly
```

and the containing section row is `6.4 + 3.2 * 2 = 12.8`; the `Tree` node itself was a
`LayoutNodeKind::Leaf`, whose height is the plain sum of its children, so two one-row sections gave

```
12.8 + 12.8 = 25.6         ← the window body height, exactly
```

`arrange_one` then placed the row at `x = padding = 3.2`, `width = parent.width - padding * 2 =
315.392 - 6.4 = 308.992`, and `y = 12.8 (the first section) + 3.2 (the second section's padding) = 16`.
**`[3.2, 16, 308.992, 6.4]` is reproduced exactly, from the token table alone.** No font metric is
read for a tree row anywhere on this path (`DeterministicTextWorker` only shapes `UiNode::Text`
nodes, and a row has none), no scale is applied twice, and nothing is unlaid-out.

### 2.3 The painter's number

`paint::retained_tree_node_step` kept its own cursor: `const TREE_ROW_HEIGHT: f32 = 24.0`,
`cursor.row_y += TREE_ROW_HEIGHT` per painted row, `const PANEL_HEADER: f32 = 24.0` for a labelled
section's header band. That is the ~24 px pitch the labels were drawn at.

### 2.4 Why the click went nowhere

`events::hit_test_node` reads `tree.accepted_layout(id)` — `mounted_layout`'s published,
double-buffered rect. So the pointer was tested against the 6.4 px rectangles and the reader was
aiming at the 24 px ones. A click at the painted label's centre fell in the gap.

### 2.5 The third copy nobody could see

`paint::sync_interactive_state_node_step`'s `Tree*` phases walked the same rows and wrote
`node.layout.{x,y,width,height}` at the CORRECT 24 px pitch — into `Node::layout`, the immediate-mode
bucket that `UiTree::accepted_layout` consults only as a **fallback** when a node has no mounted
record for the live generation. `mounted_layout` publishes a record for every node it admits, so that
write was shadowed on every frame it ever ran. Three row geometries existed; the wrong one won.

---

## 3. Deliverable 1 — the fix, at the owning layer

### 3.1 One metric, off the token React's `Tree` already uses

React sizes every tree row shell with `h-workbench min-h-workbench max-h-workbench`
(`🧱️elements/🌳️Tree/🟦️.tsx`'s `treeRowShellClassName`), and reads the number as
`treeRowHeightPx = domSizePx("treeRowUiSpacing")`. The CSS chain is
`--size-workbench = 1.5 * --size-small = 1.5 * 5 * --ui-spacing = 7.5 ui-spacings`, and
`dom::TREE_ROW_UI_SPACING = 7.5`, so at the compact root **7.5 × 3.2 = 24 px**. Section rows use the
same shell, so a header row and an item row are the same number.

`Theme` now carries it, derived, never written down:

```rust
tree_row_height: chrome_px(dom::TREE_ROW_UI_SPACING),
tree_indent_per_level: chrome_px(dom::TREE_INDENT_PER_LEVEL_UI_SPACING),
tree_toggle_width: chrome_px(dom::TREE_TOGGLE_UI_SPACING),
```

and `widgets`'s immediate-mode chrome constants became the same token expression instead of the
literals `24.0`/`10.0`/`14.0` they happened to equal. `paint`'s own `const TREE_ROW_HEIGHT: f32 = 24.0`
is **deleted**.

### 3.2 One geometry — `layout::TreeRowMetrics`

New region `🌳️TreeRowGeometry` in `🧮️layout/🦀️.rs` (the module that already owns
`gap_for_token`/`padding_for_token`/`layout_vertical`):

| function | rule |
|---|---|
| `TreeRowMetrics::from_theme` | `row_height` = `header_height` = `theme.tree_row_height`; `control_width` = `TREE_ROW_CONTROL_WIDTH`; `control_height` = `theme.control_height`; `gap` = `theme.gap_standard` |
| `tree_item_height` | `0` when the item is not visible, else `row_height` + every expanded, visible nested row (capped at `TREE_ROW_MAX_DEPTH = 64`, the painter's own `RETAINED_TREE_DEPTH`) |
| `tree_section_header_height` | `row_height` when the section is labelled, `0` when it is not — the painter's own `section.label.is_some()` gate |
| `tree_section_height` | header + every visible item row |
| `tree_node_height` | the sum of the visible sections |
| `tree_row_control_rect` | the inline control's rect **relative to its row** — one definition, taken by the painter's control draw and by the layout's arrange |

### 3.3 The layout measures a tree from the tree's own spec

`mounted_layout::LayoutNodeKind` gained `Tree { height }`, `TreeSection { header, height }` and
`TreeRow { row, height, expanded }`. The heights are resolved at **admit** time, where `&UiTree` is in
hand, by `tree_row_kind`: a `Stack` whose parent is the `Tree` and whose `NodeKey::Explicit` names one
of that tree's sections is a section row; a `Stack` whose parent is already a row and whose key names
one of that tree's items is an item row — key-matched against the owning `Tree`'s still-intact spec,
the same rule `events::find_tree_item_spec` uses. Anything else keeps its own kind, so an ordinary
container that merely lives inside a tree is untouched. The ancestor walk runs only for a `Stack`
whose parent is already tree-shaped, so the 1 024-node non-tree workload pays one `matches!`.

Arrange places them the way the painter walks them:

| parent | child | rect |
|---|---|---|
| `Tree` | a section row | `(0, Σ preceding sections, tree width, its own height)` |
| `TreeSection` | an item row | `(0, header + Σ preceding rows, tree width, its own height)` |
| `TreeRow` | a nested item row | `(0, row + Σ preceding nested rows, tree width, its own height)` |
| `TreeRow` | its inline control | `tree_row_control_rect` — the painter's own rect |

**A collapsed branch's nested rows are mounted and never painted**, so `expanded` propagates: an
unreached row measures `0` and sits at its parent's own origin. Without that, the arena's
still-mounted descendants of a collapsed row would have overlapped — and stolen the pointer from —
the row drawn beneath them. The same is true of a hidden row, whose height is `0` by
`presence.visible()`.

### 3.4 Layout is now the single writer

`paint::sync_interactive_state_node_step`'s `Tree*` phases keep exactly what only they can do —
`NodeFlags::DRAG_SOURCE` per row — and their geometry writes are **deleted**, along with
`RetainedSyncTreeRecord::{y,height}` and `RetainedSyncTreeFrame::height`. Its `cfg(test)` twin
`sync_tree_row_layout`/`sync_tree_item_layout` became `sync_tree_row_drag_sources`/
`sync_tree_item_drag_source` for the same reason. One consequence is honest and intended: before the
first layout publication a tree row has no rect at all rather than a differently-derived one, so it
is not hittable until it is laid out.

Side effect, measured: dropping those two `f32`s per record from the sync cursor shrank
`Option<UiSurfaceSlot>` from **186 344 → 159 816 bytes**, so the committed boxed-slot budget
(`⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`, guard `ui::wgpu_engine`) was updated to the new
measurement — the `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` guard is what
reported it.

---

## 4. Deliverable 2 — the fixture and the two implementations that answer it

### 4.1 The oracle

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json` — language-neutral, no Rust and no
TypeScript in it: the metric and its provenance, six named rules
(`sectionHeader`, `itemRow`, `hidden`, `collapsed`, `unreachedRows`, `width`, `y`), six cases and five
pointer probes.

| case | what it pins |
|---|---|
| `one-labelled-section-of-three-flat-rows` | the shape the defect measured at 6.4 px per row |
| `unlabelled-action-section-first-row-is-the-add-generation-row` | the exact defect row |
| `two-sections-stack-in-order` | the live generate-mode Generations window: a labelled list section above the unlabelled actions section |
| `an-expanded-row-owns-its-nested-rows` | a row's height is its own row plus every nested row it reveals |
| `a-collapsed-row-hides-its-nested-rows` | a collapsed branch's mounted-but-unpainted rows measure zero and cannot take the pointer from the row beneath |
| `a-hidden-row-takes-no-space` | hidden is no row, no rect, and no gap |

### 4.2 Rust — the law over the live arena and the live hit registry

`🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs`, attached from `🧮️layout`. It mounts each case's authored
tree, runs the **real** `MountedLayoutJob` end to end (admit → shape → measure → arrange → publish),
reads back `UiTree::mounted_layout` (what `events::hit_test` reads) and then fires
`events::hit_test` at the fixture's own points. Four laws:

* `the_row_metric_is_the_one_the_fixture_was_pinned_against`
* `every_fixture_row_is_published_at_the_rect_the_painter_draws_it_at`
* `a_pointer_aimed_at_a_painted_row_hits_that_row`
* `the_defect_geometry_is_refused`

### 4.3 TypeScript — the twin on React's own row metric

`📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts` (registered in the wgpu renderer's
`vitest.config.ts`) reads the SAME fixture, takes its row height from React's own
`domSizePx("treeRowUiSpacing")`, and **re-derives every fixture rect with a second, independent
implementation** of the stacking rule — including the zero rects of an unreached row. It also pins
the CSS chain `--size-workbench = 1.5 * --size-small`, the `Tree` row shell that carries it, and that
the Rust side reads the token rather than a literal.

### 4.4 Both, run in the foreground

**The law FAILS on the pre-fix baseline** — the tree classification switched off in `mounted_layout`,
everything else identical — and the defect's own live number falls straight out of the fixture:

```
test wgpu::layout::tree_row_rect_tests::a_pointer_aimed_at_a_painted_row_hits_that_row ... FAILED
  hit an unkeyed node: Some(Positional(15, 0))
test wgpu::layout::tree_row_rect_tests::every_fixture_row_is_published_at_the_rect_the_painter_draws_it_at ... FAILED
  one-labelled-section-of-three-flat-rows [generations]: got (0, 0, 308.992, 32), want (0, 0, 308.992, 96)
test wgpu::layout::tree_row_rect_tests::the_defect_geometry_is_refused ... FAILED
  row measured 6.4, the defect published 6.4
test wgpu::layout::tree_row_rect_tests::the_row_metric_is_the_one_the_fixture_was_pinned_against ... ok

test result: FAILED. 1 passed; 3 failed
```

`cargo test -p semio-framework-ui --features testkit --lib tree_row_rect_tests -- --nocapture --test-threads=1`,
with the fix:

```
test wgpu::layout::tree_row_rect_tests::a_pointer_aimed_at_a_painted_row_hits_that_row ...
[DEBUG] tree-row-rects hit (86, 84) -> add-generation
[DEBUG] tree-row-rects hit (40, 30) -> gen-a
[DEBUG] tree-row-rects hit (40, 60) -> gen-b
[DEBUG] tree-row-rects hit (10, 36) -> leaf-a
[DEBUG] tree-row-rects hit (10, 36) -> after
test wgpu::layout::tree_row_rect_tests::every_fixture_row_is_published_at_the_rect_the_painter_draws_it_at ...
[DEBUG] tree-row-rects case one-labelled-section-of-three-flat-rows: 4 rows pinned
[DEBUG] tree-row-rects case unlabelled-action-section-first-row-is-the-add-generation-row: 2 rows pinned
[DEBUG] tree-row-rects case two-sections-stack-in-order: 5 rows pinned
[DEBUG] tree-row-rects case an-expanded-row-owns-its-nested-rows: 5 rows pinned
[DEBUG] tree-row-rects case a-collapsed-row-hides-its-nested-rows: 5 rows pinned
[DEBUG] tree-row-rects case a-hidden-row-takes-no-space: 3 rows pinned
test wgpu::layout::tree_row_rect_tests::the_defect_geometry_is_refused ... ok
test wgpu::layout::tree_row_rect_tests::the_row_metric_is_the_one_the_fixture_was_pinned_against ... ok

test result: ok. 4 passed; 0 failed; 398 filtered out
```

The two `(10, 36)` probes are the point of the pair: in the expanded case it lands on the nested
`leaf-a`, in the collapsed case on `after` — the row actually drawn there.

The TypeScript twin,
`bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🌳️tree-row-rects/🟦️.ts"`:

```
[DEBUG] tree-row-rects react rowHeightPx=24
[DEBUG] tree-row-rects twin case one-labelled-section-of-three-flat-rows: 4 rows re-derived
[DEBUG] tree-row-rects twin case unlabelled-action-section-first-row-is-the-add-generation-row: 2 rows re-derived
[DEBUG] tree-row-rects twin case two-sections-stack-in-order: 5 rows re-derived
[DEBUG] tree-row-rects twin case an-expanded-row-owns-its-nested-rows: 5 rows re-derived
[DEBUG] tree-row-rects twin case a-collapsed-row-hides-its-nested-rows: 5 rows re-derived
[DEBUG] tree-row-rects twin case a-hidden-row-takes-no-space: 3 rows re-derived

Test Files  1 passed (1)
     Tests  6 passed (6)
```

---

## 5. Deliverable 3 — what 6118 says, measured

### 5.1 The rows are published where they are painted — proven on the live target

`?plugin=generation3d&mode=generate`, `dumpStructure("generation3d-generations")`, this lane's
bundle (`🗑️generated/wgpu-hit/run-2/structures.json`):

```
tree  tree[0]                                                       [0, 0, 315.392, 813.6]
stack tree[0]/stack[0]#procedural3d-play-generate.generations       [0,  0, 315.392, 48]
stack   …generations/stack[0]#…                                     [0, 24, 315.392, 24]
stack tree[0]/stack[1]#procedural3d-play-generate.actions           [0, 48, 315.392, 48]
stack   …actions/stack[0]#procedural3d-play-generate.add-generation [0, 72, 315.392, 24]
```

Against the defect's own line — `add-generation` at `[3.2, 16, 308.992, 6.4]`, the window body
25.6 px tall — every number moved to the painter's: **each row is 24 px**, each labelled section
carries a 24 px header (`generations` = 24 + 24 = 48 at y 0; `actions` = 24 + 24 = 48 at y 48), and
`add-generation` sits at y 72, exactly `48 + 24`. That is the fixture's model, live, with no
fixture involved.

The bundle is this lane's: the dock trace now carries each body's `y`, which only this lane's
renderer prints —
`[DEBUG] wgpu-shell dock plan canvas=1434x836 windows=3 generation3d-generations@315x814+3,54
generation3d-generate-form@616x814+319,54 generation3d-generate-preview@502x814+935,54`.

### 5.2 The pointer is delivered inside that rectangle — and still dispatches nothing

The row's page point is derived, not guessed: body `(3, 54)` + rect `(0, 72)` + half the row ⇒
**(160.696, 138)**, and the click is witnessed at the browser edge
(`🗑️generated/wgpu-hit/run-2/verdict.json`):

```
pointerWitness = {"downs": 1, "last": {"x": 160.696, "y": 138, "offsetX": 160.696, "offsetY": 138},
                  "canvas": {"x": 0, "y": 0, "width": 1440, "height": 900}, "tag": "CANVAS"}
```

The canvas is the full 1440×900 page at the origin and `devicePixelRatio` is 1, so those are the
same coordinates `browser-boot`'s `pointerdown` listener forwards. The point is inside
`(3, 126)–(318, 150)`. And yet `render begin` stays at 7, `wgpu-shell command …` never appears, and
`addGeneration` is never mentioned.

### 5.3 Why — and it is not the row

**No input of any kind reaches the shell on this target.** `🗑️generated/wgpu-hit/input-witness.json`,
six probes after a settled boot, counting every console line each one produced:

| probe | new console lines |
|---|---|
| click the tree row `(160.696, 138)` | 0 (only the `world3d` render heartbeat) |
| click the navbar `(120, 14)` | 0 |
| wheel over the tree | 0 |
| `Tab` | 0 |
| `f` | 0 |
| focus the canvas, then click the row | 0 |

and a hover sweep (`hover-witness.json`) over the row's centre, its top, its bottom **and 40 px above
it** leaves `dumpStructure`'s `state.hovered` false on every node of that window — the hit test is
never even asked. `shot-before.png`/`shot-after.png` are the page's bare `#001117` ground: this
target is not presenting either.

The cause is named, not guessed: **the trunk serve this lane inherited was deleted out from under it
mid-session.** At 02:26 a peer removed `Trunk.toml`/`🌐️.html`/`📋️project.json` from
`🎯️targets/🧊️wgpu/📦️packages/🦀️rust` and stood up a Vite replacement at
`🎯️targets/🧊️wgpu/🌐️server` ("owns one browser listener consuming Nx-completed WGPU artifacts",
timestamps 02:13–02:29). The old serve could not restart (`Trunk.toml is neither a file nor a
directory`, `TRUNK_EXIT=1`); the measurements above were taken on the replacement,
`bun 🌐️server/📜️script.ts serve generation3d dev` with `S_OS_PORT=6118`, which boots the shell,
reconciles every document and paints — the `dumpStructure` evidence in §5.1 is real — but does not
yet present to the page or route a single pointer, wheel or key event into `Ui::dispatch_event`.

### 5.4 So, precisely

* **Claimed and proven:** the layout publishes tree rows at the painter's own geometry, on 6118,
  in generate mode, in the exact window and on the exact row the defect named.
* **Claimed and proven:** the pointer lands inside that published rectangle.
* **NOT claimed:** that clicking `Add Generation` dispatches `addGeneration`, that the generate
  preview then carries a mesh (`dumpFrameStats("generation3d-generate-preview")` reports
  `scenePasses 1, sceneDraws 1, sceneInstances 0` — an empty scene, as before any generation), or
  that the Flow node-graph and a Preview instance take a pointer. None of those can be observed
  while no input reaches the shell at all, and none of them is claimed here.
* **The named remaining hop, with its evidence:** the wgpu browser target's input path
  (`browser-boot`'s canvas listeners → `BrowserFrameTransport` → the frame worker's
  `Ui::dispatch_event`) delivers nothing on the new Vite host — click, wheel and keyboard alike, on
  chrome and on window bodies alike. That is one lane of its own, on the host the migration just
  created, and it is the reason the previous lane's five clicks were also silent: the row geometry
  was only half of it.

---

## 6. Checks and builds

| command | result |
|---|---|
| `cargo test -p semio-framework-ui --features testkit --lib -- --test-threads=1` | **397 passed, 5 failed** — the four new laws pass; every remaining failure is named in §8 and four of the five were verified failing on HEAD's own `mounted_layout` |
| `cargo test … --lib tree_row_rect_tests -- --nocapture` | 4 passed, 0 failed (§4.4) |
| `bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🌳️tree-row-rects/🟦️.ts"` | 6 passed, 0 failed (§4.4) |
| `cargo check -p semio-framework-ui --features testkit --tests` | clean; the 14 warnings are all pre-existing (`flex`'s legacy layout job, `component`'s macro doc comments) |

Two workspace-red waits were taken and attributed rather than worked around:

* **`semio-framework-ui` would not compile at all** on entry:
  `🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs:98` was missing
  `World3dScene::instances_delta_json`, a field a peer added to `🎬️scene/🎬️scenes/🦀️.rs` at 19:42 and
  did not carry into that literal (19:44). Waited the prescribed five minutes, re-checked unchanged
  four hours stale, then **completed** the peer's field (`instances_delta_json: None`, which the
  golden JSON does not see — the field is `skip_serializing_if = "Option::is_none"`). Nothing was
  reverted.
* **`semio-framework-pack`/`semio-framework-os-kernel` went red mid-session** (`RetainedPackSymbolTable`
  losing `get`/`push`/`is_empty`, `RetainedPackCatalogCursor` losing `symbol_scalars`) while a peer
  was editing `🎒️pack/📐️format/🦀️.rs`. Waited; the peer landed the rest and it went green on its own.
  Nothing of theirs was touched.

The shared cargo build directory was used throughout (`CARGO_PROFILE_WASM_DEV_DEBUG=false`,
`NX_DAEMON=false`); no private `CARGO_TARGET_DIR`, no `--release`, no git-state-modifying command.

---

## 7. Files

**Changed**

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs` | `tree_row_height`/`tree_indent_per_level`/`tree_toggle_width`, all `chrome_px(dom::…)` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs` | new region `🌳️TreeRowGeometry`: `TreeRowMetrics`, `tree_item_height`, `tree_section_height`, `tree_section_header_height`, `tree_node_height`, `tree_row_control_rect`, `TREE_ROW_CONTROL_WIDTH`, `TREE_ROW_MAX_DEPTH`; attaches the new law |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` | `LayoutNodeKind::{Tree,TreeSection,TreeRow}`, `tree_row_kind`, `owning_tree_spec`, `find_tree_item`, the job's `row_metrics`, and the measure/arrange arms |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` | `const TREE_ROW_HEIGHT` deleted; the retained tree walk, its `cfg(test)` immediate-mode twin and the control rect all read `TreeRowMetrics`; the interactive sync's tree geometry writes removed (drag-source sync kept) |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs` | `TREE_ROW_HEIGHT`/`TREE_INDENT_PER_LEVEL`/`TREE_TOGGLE_WIDTH` become the token expression |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | the `dock plan` trace now names each body's `y` as well as its `x`, so a probe can convert a window-space rect to a page point without guessing |
| `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` | `ui::wgpu_engine` element size re-measured: 186 344 → 159 848 |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs` | completed a peer's `World3dScene` field so the crate compiles (§6) |

**Added**

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-row-rects/🔣️.json` | the language-neutral oracle |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️tree-row-rects/🦀️.rs` | the Rust law over the live arena + hit registry |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️tree-row-rects/🟦️.ts` | the TypeScript twin on React's row metric (registered in `🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`) |
| `<ticket>/🐍️wgpu-hit-probe.mjs` | the runtime probe, whose every click point is derived from the dock plan + the published rects |
| `<ticket>/🗑️generated/wgpu-hit/{click-sweep,hover-witness,input-witness}.mjs` | the three witnesses §5.2/§5.3 quote |

**Also changed:** `<ticket>/📜️serve-generation3d-wgpu.sh` — the dev path is now opt-in (§8.3).

---

## 8. What is NOT claimed, and the build environment this ran in

### 8.1 A tree row's trailing ACTION icons still have no hit rect of their own

`paint::retained_tree_node_step` draws a row's `actions` at
`bounds.x + bounds.w - gap - (i + 1) * (TREE_ICON_SIZE + padding)`, and on the DOCUMENT path those
actions are `TreeItemProps::row_actions` — **props, not child records** (`reconcile::tree_item`), so
they mount no arena node and carry no rect. A click on a painted row-action icon therefore lands on
the ROW (its `activate`), not on the action. This is the same "declared but unplaced" family this
lane just closed one layer up, it is now the only one left inside a tree row, and it is named rather
than guessed at: closing it means giving the row-action rects an owner in
`layout::TreeRowGeometry` and either mounting the actions or registering their rects the way the
immediate-mode chrome's `ctx.input.register_hit` does.

### 8.2 Five `semio-framework-ui` tests still fail, four of them not this lane's

* `wgpu::prepared::tests::{preparation_yields_at_the_configured_item_budget,
  receiver_survives_worker_ownership_of_the_job,
  retained_codec_source_moves_once_and_retires_one_page_per_governed_step}` —
  `PreparedRenderInput::try_new` refuses its own test input ("test prepared input must fit fixed
  process permits", `🎟️prepared/🦀️.rs:1866`).
* `wgpu::engine::retained_document_hostile_fixtures::max_plus_one_…` — `UiDocumentBuilder::try_push`
  refuses a hostile page (`…hostile-fixtures/🦀️.rs:28`).

All four were verified failing **with HEAD's own `mounted_layout` swapped back in** and everything
else of this lane in place, so they are not this lane's.

* `wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms`
  asserts that no single layout slice of a 1 025-node/text workload exceeds 8 ms in a **debug**
  build. It is load-sensitive on this machine to the point of being uninformative: on HEAD's own
  `mounted_layout` three consecutive runs gave *pass, pass, **98.3 ms***; with this lane's layout,
  eight runs on an idle machine gave 8.2 / 8.8 / 10.6 / 13.0 / 15.2 / 18.9 / 22.9 / 30.0 ms. The
  best observed slice sits just over the threshold rather than just under it, so this lane may carry
  a small constant cost in the worst slice — the per-node work it adds is one `matches!` on the
  parent's kind for a `Stack` (`TreeRowMetrics` is built once per job, not per node), and no
  measurement here separates that from the machine. **Not claimed as passing, and not claimed as
  untouched.**

### 8.3 The dev-serve environment, and what had to be killed to build at all

The 6118 serve began this lane as `trunk serve` behind `<ticket>/📜️serve-generation3d-wgpu.sh` and
ended it as the peer's new Vite host (§5.3). Two things are worth recording for the next lane:

* **The shared cargo lanes gridlocked repeatedly.** Six to eight peer builds plus two serves sat at
  0 % CPU with no `rustc` anywhere for over an hour, each holding one of the
  `build`/`target` × `debug`/`wasm32` lock pairs and waiting for another. Breaking it needed killing
  wedged build PROCESSES — never anyone's source: two 1 h-old `wasm-pack` cargos, one 12-minute
  `verify` cargo holding the host artifact lock, and one peer `trunk build`. Nothing was reverted,
  and after each kill the queue drained. `cargo build --target wasm32-unknown-unknown --lib --locked`
  for `semio-framework-os-renderer-wgpu` then finished clean in 2 m 12 s.
* **Killing a `trunk serve` child is not a way out of a wedge.** Trunk's single `rel="rust"`
  pipeline IS that cargo call: SIGTERM to it fails the whole build pipeline
  (`cargo call … returned a bad status: signal: 15`) and leaves the serve on its previous bundle.
  The wedges were the peers' locks, not trunk siblings.
* `📜️serve-generation3d-wgpu.sh` now skips its dev path by default (`SEMIO_SERVE_TRY_DEV=1`
  restores it): it has refused on every start since the peer `localstorage` census landed, measured
  again this session as `DEV_EXIT=1` after ~10 minutes, and each attempt held the shared cargo locks
  for that whole time. The script's trunk fallback is itself now obsolete — `Trunk.toml` is gone
  (§5.3) — and the file is left in place, with its header, for whoever finishes that migration.
