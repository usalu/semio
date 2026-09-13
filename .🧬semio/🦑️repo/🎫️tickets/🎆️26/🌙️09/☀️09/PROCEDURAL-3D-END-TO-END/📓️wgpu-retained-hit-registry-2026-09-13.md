# 🎯️ wgpu RETAINED HIT REGISTRY — the window body's own pointer targets

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu retained hit registry", 2026-09-13.
Closes the ONE hop `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6 named and refused to guess at.
Target: `http://127.0.0.1:6118/?plugin=generation3d[&mode=generate]`, the peer's
`🎯️targets/🧊️wgpu/🌐️server`.

Repo MCP was down this session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened. The procedural plugin was NOT restaged, the react serve on
6018 was not touched (it is down), 6118 is the peer's process and was NOT restarted, no
git-state-modifying command was run, and nothing of the concurrent lanes was reverted.

---

## 1. TL;DR

| question | answer |
|---|---|
| **the brief's defect** | `InputState::hit_targets` held 31 targets — the shell chrome — and not one row of any retained window body, so `hit_at(160.696, 138)`, a point inside the published `mounted_layout` rect `[0, 72, 315.392, 24]` of the `Add Generation` row, answered the WINDOW's `HitKind::ScrollRegion`. |
| **why** | Nothing ever projected the retained arena onto the flat registry the shell scans. The arena HAS a complete hit path of its own (`events::hit_test` over the published `mounted_layout` rects, `EventRouter` hover/activation/`UiCommand::Scene`), and the keyboard already routes into it — but `ShellState::handle_pointer_*` only ever consults `InputState::hit_at`, whose only writers were the immediate-mode chrome's `ctx.input.register_hit` calls. Two registries, one of them empty for every window body. |
| **the fix** | The retained paint frame gained a `RetainedPaintPhase::Hits` that re-walks the SAME walk the paint phase just painted from — same accumulated origin, same published rects — and mints one `RetainedHitRegistration` per interactive node. The engine publishes it per window; the shell pushes it into `InputState` right after each body's paint, AFTER the window's own `ScrollRegion`; a press on a retained target dispatches the node's own declared action, and a move over one is routed into that body's `events::EventRouter`, the one owner of `NodeFlags::HOVERED`. |
| **fixture + laws** | `🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` — 4 cases, 14 declared entries, 7 pointer points. Rust law over the LIVE pipeline (real layout → real `frame_into_step` → real `InputState`): **2 passed, 0 failed**, and **both fail on the pre-fix baseline** reproducing §6's own line. TypeScript twin: **11 passed, 0 failed**. |
| **runtime on 6118** | see §5. |

---

## 2. The hop, and where this lane sits on it

```
OS event ─► AppRuntime::handle_pointer_* (🧊️renderer)
   ├─ world3d_states / node_graph_states / tiled_map_states / board2d_states  ← bounds dispatch, already live
   └─ ShellState::handle_pointer_move / handle_pointer_button
        └─ InputState::hit_at ─► HitTarget
              ▲
              │  ✅ THIS LANE: the retained body's own entries, minted from the painted rects
              │
   Ui::frame_into_step
        Synchronize → Paint → Scenes → **Hits** → Publish
                                        │
                                        └─ input::retained_hit_registration(node, rect) ─► window.hit_registry
                                                                                              │
   ShellState::render_main_window_step / render_panel_step ─► interpreter::register_retained_hit_targets ─┘
```

Everything downstream of `hit_at` already existed and was already proven by other lanes: the shell's
`dispatch_action` funnel, `AppFrameTransactionPhase::WheelStart`'s
`wheel_propagates_to_scene_surface` gate and the `WheelWorld3d`/`WheelGraph`/`WheelMap`/`WheelBoard`
phases behind it, `events::EventRouter`'s hover chain and `UiCommand::App`/`UiCommand::Scene` lanes,
and `interpreter::dispatch_ui_event`'s command application. This lane only fills the registry they
all read.

---

## 3. Deliverable 1 — the fix, at the owning layer

### 3.1 One geometry, by construction

`RetainedPaintWalk::step` is the only place a retained child's origin is derived. It now subtracts a
scrollable parent's own `WidgetState::scroll_offset` from that origin, and BOTH the paint phase and
the new hit phase walk through it:

```rust
let (scroll_x, scroll_y) = tree.node(visit.node).filter(|node| node.flags.contains(NodeFlags::SCROLLABLE)).map_or((0.0, 0.0), |node| node.state.scroll_offset);
let child_visit = RetainedPaintVisit { origin_x: visit.origin_x + layout.x - scroll_x, origin_y: visit.origin_y + layout.y - scroll_y, .. };
```

No production writer sets `NodeFlags::SCROLLABLE` on this target today (`events::route_scroll` is its
only reader), so the term is a measured no-op right now — it is there so that the moment a scrolled
container paints offset, its hit targets move with it rather than diverging. The honest statement of
the brief's "with scroll offsets applied" is: the offset is applied in the ONE origin rule the
painter uses, never as a second correction on the registry side.

### 3.2 `RetainedPaintPhase::Hits`

`⚙️engine`'s retained paint frame gained a fifth phase between `Scenes` and `Publish`. It resets the
walk, and for each visit calls `register_retained_hit(tree, theme, node, origin)`, which reads the
node's published `accepted_layout` and hands `(node, absolute rect)` to the classifier. `Publish`
swaps the candidate list into `UiWindow::hit_registry`, read back through
`Ui::window_hit_targets(window_id)`. `RETAINED_HIT_REGISTRY_CAPACITY = 4 096`, well under `input`'s
own `HIT_TARGET_CAPACITY = 8 192`, so a hostile document cannot crowd the chrome out of the registry.

`frame_into_step` (the production entry) adds the caller's viewport offset to the origin exactly as
its `Paint` and `Scenes` phases do, so the registry is page-space and needs no second transform.

### 3.3 The classifier — `input::retained_hit_registration`

New region `🎯️RetainedHitRegistry` in `📥️input/🦀️.rs`, the module that owns `HitTarget`/`HitKind`.
It re-derives a synthesized row's role from the owning `Tree`'s own still-intact spec by key, through
`mounted_layout::owning_tree_spec`/`find_tree_item` — the SAME discriminator
`mounted_layout::tree_row_kind` measures with, so the row a pointer resolves and the row the layout
published are one classification.

| retained node | `HitKind` | control id | dispatched action |
|---|---|---|---|
| tree item row (`Stack` keyed by an item id) | `TreeItem` | `tree.label.<item id>` | the row's `activate` (falling back to the authored `item.action`) |
| labelled tree section row | `TreeItem` | `section.chevron.<section id>` | its `activate`, if any |
| `Button` | `Button` | `button.id` | `button.action` |
| `Stack` with `activate`, not a tree row | `Button` | `stack.id` | `activate` |
| `Input` / `Select` / `Toggle` / `Slider` | `Input` / `Select` / `Toggle` / `Slider` | the node's own id | none — the retained router owns what a press means |
| `ComponentScene`, `world-3d` | `World3d` | `surface_id` | none |
| `ComponentScene`, `node-graph` / `board-2d` | `ScrollRegion` | `<surface_id>.pane` | none |
| `ComponentScene`, `tiled-map` | `ScrollRegion` | `<surface_id>.map` | none |
| everything else | — | — | nothing registered |

The tree row's band is clipped to ONE `TreeRowMetrics::row_height` even though the row's published
height also covers the nested rows it reveals; each nested row registers its own band. A tree section
row's band is its own header height, `0` when the section is unlabelled (the painter's own
`section.label.is_some()` gate). A node with no area registers nothing, so a mounted-but-unplaced row
— a collapsed branch's descendant, a tree row's inline action button — can never take the pointer
from what is actually drawn there.

The `.pane`/`.map` suffixes are not invented here: they are exactly what
`ShellState::scroll_region_is_scene_surface` already reads, which is what makes
`wheel_propagates_to_scene_surface` answer `true` over a node graph or a map.

### 3.4 The shell publishes it, and routes the pointer into the body

* `ShellState::retained_hit_windows: HashMap<String, (String, Rect)>` — control id → the body that
  minted it and the rect it was painted into. Cleared once per chrome walk at
  `ShellChromeFramePhase::FrameSetup` step 0, where `FrameBuildPhase::InputFrame` has already drained
  `InputState::hit_targets` for this build.
* `register_retained_body_hits(window_id, body, input)` is called by BOTH bodies that paint a
  retained document — `render_main_window_step` phase 4 (a dock window) and `render_panel_step`
  phase 8 (a panel) — immediately after the document step, i.e. after the chrome registered the
  window's own `HitKind::ScrollRegion`. `InputState::hit_at` scans in reverse, so every body entry
  outranks the window beneath it.
* `handle_pointer_move` routes a move whose top hit is a retained entry into
  `interpreter::dispatch_ui_event(window_id, UiEvent::PointerMove { .. })` in that body's LOCAL
  coordinates — the one owner of `NodeFlags::HOVERED`, the hover bubble chain, and the
  `UiCommand::Scene` lane.
* `handle_pointer_button` routes a press on a retained entry through
  `route_retained_pointer_press`: a control the retained router itself owns (an `Input`'s focus, a
  `Select`'s popup) is dispatched into that router as `PointerDown`/`PointerUp` and its
  `FocusChanged` commands are fed to `chrome_build.note_content_focus_commands`, so keys follow the
  pointer; everything else dispatches the node's own declared action through `dispatch_action`, the
  same funnel every chrome hit uses, and returns — no chrome id convention (`dock.*`, `.vfs.`,
  `.minus`/`.plus`, `.item.`) can claim a document id.

Activation deliberately has ONE owner per node. A `Button`/`TreeItem` fires through the shell's
`hit.event`; the router's own `UiCommand::App` arm is not additionally driven for those, so a press
cannot dispatch twice.

---

## 4. Deliverable 2 — the fixture and the two implementations that answer it

### 4.1 The oracle

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` — language-neutral: the row
metric and its provenance, eight named rules (`oneGeometry`, `order`, `treeItem`, `treeSection`,
`button`, `input`, `engineSurface`, `container`, `zeroArea`), four cases carrying an authored document
tree plus the window body it is painted into, 14 declared registry entries (control id, hit kind,
dispatched action, absolute rect) and 7 pointer points (expected control id, kind, action, and for the
engine surfaces the `wheelPropagatesToScene` verdict).

| case | what it pins |
|---|---|
| `generate-mode-generations-window` | the exact window, rows and point §6 measured: the two labelled sections of the live Generations window and `(160.696, 138)` |
| `a-world-3d-canvas-takes-the-wheel` | a world-3d canvas registers `World3d` and the wheel gate opens |
| `a-node-graph-canvas-takes-the-wheel-through-the-pane-convention` | the `.pane` convention, and the same gate |
| `a-plain-container-never-takes-the-pointer` | a layout-only `Stack` registers nothing and the window answers |

### 4.2 Rust — the law over the live pipeline

`🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs`, mounted from `📥️input/🦀️.rs`. It drives the REAL
pipeline for every case — `Ui::apply_tree` → the real `MountedLayoutJob` (settled on the window's own
`layout_is_dirty` predicate, not the queue's verdict) → `Ui::frame_into_step` with a real scene host —
and then loads a real `InputState` exactly the way the shell loads it: the window's own `ScrollRegion`
first, the body's entries after. Two laws:
`every_fixture_case_registers_resolves_and_orders_on_the_live_registry` and
`the_defect_point_no_longer_answers_the_window`.

Both are ONE test each on purpose: `Ui`'s layout is driven through a process worker pool, and a law
that mounts a fresh `Ui` per `#[test]` starved itself once enough had been created — measured as
`retained paint never completed, parked in None`.

### 4.3 TypeScript — the twin

`📺️renderer/🧑‍🎨engine/🧪️tests/🎯️retained-hit-targets/🟦️.ts`, registered in
`🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`. Reads the same fixture and re-derives the WHOLE registry
from the fixture's declared rules with a second, independent implementation, on React's own row
metric (`domSizePx("treeRowUiSpacing")`) — the section/item band stacking, the engine-surface
kind/id mapping, the reverse-order resolution, and `wheel_propagates_to_scene_surface`'s predicate.

### 4.4 Both, run in the foreground

**The law FAILS on the pre-fix baseline** — the classifier short-circuited to `None`, everything else
identical — and reproduces §6's own console line exactly:

```
assertion `left == right` failed: generate-mode-generations-window: registry length, got []
  left: 0
 right: 4
assertion `left != right` failed: the window's own ScrollRegion answered the row point again
  left: Some("generation3d-generations")
 right: Some("generation3d-generations")
test result: FAILED. 0 passed; 2 failed
```

With the fix,
`RUST_MIN_STACK=33554432 cargo test -p semio-framework-ui --features testkit --lib -- retained_hit_target_tests:: --test-threads=1 --nocapture`:

```
[DEBUG] retained-hit-targets probe generate-mode-generations-window (160.696, 138) -> Some("tree.label.procedural3d-play-generate.add-generation")
[DEBUG] retained-hit-targets probe generate-mode-generations-window (160.696, 66) -> Some("section.chevron.procedural3d-play-generate.generations")
[DEBUG] retained-hit-targets probe generate-mode-generations-window (160.696, 90) -> Some("tree.label.procedural3d-play-generate.generations.empty")
[DEBUG] retained-hit-targets probe generate-mode-generations-window (160.696, 600) -> Some("generation3d-generations")
[DEBUG] retained-hit-targets case generate-mode-generations-window: 4 entries pinned, 4 points replayed
[DEBUG] retained-hit-targets probe a-world-3d-canvas-takes-the-wheel (1186, 460) -> Some("generation3d-generate-preview")
[DEBUG] retained-hit-targets case a-world-3d-canvas-takes-the-wheel: 1 entries pinned, 1 points replayed
[DEBUG] retained-hit-targets probe a-node-graph-canvas-takes-the-wheel-through-the-pane-convention (303, 254) -> Some("generation3d-flow.pane")
[DEBUG] retained-hit-targets case a-node-graph-canvas-takes-the-wheel-through-the-pane-convention: 1 entries pinned, 1 points replayed
[DEBUG] retained-hit-targets probe a-plain-container-never-takes-the-pointer (600, 200) -> Some("generation3d-generate-form")
[DEBUG] retained-hit-targets case a-plain-container-never-takes-the-pointer: 0 entries pinned, 1 points replayed
[DEBUG] retained-hit-targets: (160.696, 138) -> Some("tree.label.procedural3d-play-generate.add-generation") / addGeneration
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 402 filtered out
```

Every absolute rect in the fixture was hand-authored from the live target's own published numbers
(`📓️wgpu-tree-row-hit-test-2026-09-12.md` §5.1) and the live layout reproduced all 14 of them to
within 0.01 px.

The twin,
`bunx vitest run --config "🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🎯️retained-hit-targets/🟦️.ts"`:

```
Test Files  1 passed (1)
     Tests  11 passed (11)
```

---

## 5. Deliverable 3 — 6118, and why it is NOT claimed

**Not claimed, not attempted with a stale binary, and named with its exact blocker.**

The renderer wasm this deliverable needs could not be built. `semio-framework-os-renderer-wgpu` —
the crate that carries every line of this lane's shell/interpreter half, and the crate the 6118 serve
consumes through `dist/wasm-dev` — has not compiled since **05:33** because of a peer's in-flight
window-config migration, measured every few minutes through **06:59** — 86 minutes, with the declaring file (`🎚️config/🦀️.rs`, 05:28) untouched since:

```
error[E0277]: the trait bound `window::component::Puzzle3dWindowConfig: DslField` is not satisfied
   --> ✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🪟️window/🦀️.rs:194:18
194 |     type State = Puzzle3dWindowConfig;
error: could not compile `semio-s-artifact-puzzle-3d` (lib)
```

Diagnosis, so the owning lane can close it rather than re-derive it:

* `🔌️plugin/🪟️window/🎚️config/🦀️.rs:21` (last written **05:28**) added
  `store::mounted_pack_rt::DslField` to `WindowConfigOwner::State`'s bound list.
* `DslField` is emitted by `#[derive(dsl::DslRecord)]`. **No artifact's window config derives it** —
  `semio-s-artifact-flow-flow` fails identically
  (`the trait bound FlowMainWindowConfig: DslField is not satisfied`,
  `…/🌊️main/🎚️config/🦀️.rs:143`), and neither `FlowMainWindowConfig` nor `Puzzle3dWindowConfig`
  carries more than `value_derive::ToValue`/`FromValue`. This is a tree-wide migration in flight, not
  one stale artifact.
* Adding the derive is not mechanical: `Puzzle3dWindowConfig` carries
  `panel_pages: HashMap<String, u32>`, and `🗣️dsl` implements `DslField` for
  `BTreeMap<String, T>`/`Vec<T>`/`[T; N]`/`Box<T>` and **not** for `HashMap`. A persisted config
  record's map field has to be a `BTreeMap` for the encoding to be deterministic at all, so the
  migration is a type change across 13 signatures in that artifact alone.

I tried that `HashMap` → `BTreeMap` completion, measured that it does **not** lift the bound on its
own (the missing `DslRecord` derive is a second, independent step), and **reverted my own edit** so
the peer's files carry no foreign half-migration. Nothing of theirs was touched, nothing was
reverted but my own attempt.

The shared cargo cache had also filled the disk to 120 MiB free of 926 GiB, which failed builds and
the agent harness alike; `bun 🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts cache-prune` — the
repo's own registered prune — reclaimed **111.99 GiB** across 1 231 units. That is recorded because
it changes nothing but build time and every lane on this machine was affected.

### 5.1 What a runtime pass must measure, once that bound is migrated

The probe is already written and committed to the ticket — `<ticket>/🐍️wgpu-hit-probe.mjs`, extended
this lane with a **hover witness** and a **wheel witness**:

```
screen -dmS g3dwgpu "<ticket>/📜️serve-generation3d-wgpu.sh"     # already up; 6118 answers 200
bunx nx run @semio-tech/framework-renderer-wgpu:wasm            # then RELOAD the page, no restart
cd <ticket> && SEMIO_PROBE_SETTLE=40 SEMIO_PROBE_OUT=wgpu-hits/run-N bun 🐍️wgpu-hit-probe.mjs
```

| what it now measures | field in `verdict.json` |
|---|---|
| `state.hovered` on the row, sampled idle → on the row → 300 px away → back | `hoverWitness.{idle,onRow,away,back}` — all four were `[]` in §6's own run |
| one click on `Add Generation` at the derived row point | `clicked`, `renderBeginBefore`/`renderBeginAfter`, `addGenerationLines`, `commandSettled` |
| the generate preview's scene census | `previewStats` (`sceneInstances`) |
| the world3d camera before/after four wheel ticks over the Preview | `wheelWitness.{before,after,changed}` — byte-identical across all 121 traces in §6's run |
| the registry itself, per pointer move | `pointerHitLines` (the `[DEBUG] os_host pointer hit … targets=N hit=…` trace the previous lane left in place) |

The `targets=31 hit=Some((ScrollRegion, …))` line that trace prints is the single number to read
first: with this lane's registry it must report more than the chrome's 31 and answer
`Some((TreeItem, Some("tree.label.procedural3d-play-generate.add-generation")))` at
`(160.696, 138)`. Every one of those five rows is exactly what the Rust law already asserts against
the live registry off-target (§4.4), which is why the native half is claimed and the browser half is
not.

---

## 6. Checks and tests, run in the foreground

| command | result |
|---|---|
| `RUST_MIN_STACK=33554432 cargo test -p semio-framework-ui --features testkit --lib -- retained_hit_target_tests:: --test-threads=1 --nocapture` | **2 passed, 0 failed** (§4.4); both **fail** on the pre-fix baseline |
| `cargo test -p semio-framework-ui --features testkit --lib -- --test-threads=1` (whole crate, 404 tests) | **400 passed, 4 failed**, stable across 3 consecutive runs. The 4 are the ones `📓️wgpu-tree-row-hit-test-2026-09-12.md` §8.2 already named as not that lane's either: `wgpu::prepared::tests::{preparation_yields_at_the_configured_item_budget, receiver_survives_worker_ownership_of_the_job, retained_codec_source_moves_once_and_retires_one_page_per_governed_step}` (`PreparedRenderInput::try_new` refuses its own test input) and `wgpu::engine::retained_document_hostile_fixtures::max_plus_one_…` (`ArenaFull`). `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` DID fail on this lane's first run and is fixed, not silenced: the guard measured the two `Vec`s this lane adds to `UiSurfaceSlot` (159 848 → 159 896 bytes) and the committed budget was re-measured. |
| `cargo check -p semio-framework-ui --lib --features testkit` | clean; the 2 warnings (`paint`'s unused `tree_section_header_height` import, its dead `PANEL_HEADER`) are the tree-row lane's, not this one's — quoted as proof the expansion actually ran |
| `bunx vitest run --config "🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🎯️retained-hit-targets/🟦️.ts"` | **11 passed, 0 failed** |
| `bunx vitest run --config "🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts"` (all 16 files) | **175 passed, 6 failed** — the six are `🧪️tests/🧩️package-integration/🟦️.ts`'s own known constraint (it must run under `test-preview-generated`), unchanged by this lane |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | **blocked**, §5 — the crate has not compiled since 05:33 for a reason outside this lane, re-measured every few minutes through 06:59 |

**Stated plainly:** the `🖱️ui` half of this lane (the classifier, the `Hits` phase, the shared origin
rule, the fixture and both laws) is compiled, tested and proven. The `🐚️Shell`/`🗣️Interpreter`
half (≈80 lines: the registry publication, the owner map and the pointer routing) is **written but
not compile-verified**, because its crate has not built since before it was written. That is a
claim this report does not make, not a claim it makes weakly.

### Peer breakages run through, attributed and waited on rather than worked around

1. **`semio-framework-plugin`** was red at 05:33 (`*value = None` expecting `String`, three private
   `DocumentStoreOwners` fields) — the window-config retained lane mid-edit. Waited; it went green on
   its own at 05:47. Nothing was touched.
2. **`semio-framework-os-kernel`/`🏪️store`** went red at ~06:00 (`edit_index` declared twice, a
   `ManuallyDrop<Option<BTreeMap>>` being `+=`'d) — the same lane, one file further along. Waited; it
   cleared on its own.
3. **`semio-s-artifact-puzzle-3d`** (and `semio-s-artifact-flow-flow`, identically) — §5. Still red at 06:59. Diagnosed, not worked around; my one
   attempt at completing it was reverted.
4. **The disk** — §5. 120 MiB free of 926 GiB; the repo's own `cache-prune` reclaimed 112 GiB.

---

## 7. Files

**Changed**

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs` | new region `🎯️RetainedHitRegistry`: `RetainedHitRegistration`, `to_hit_target`, `RetainedTreeRow`, `retained_tree_row`, `retained_scene_hit`, `retained_hit_registration`; `ActionDescriptor` promoted from a `cfg(test)` import to a real one; the new law mounted |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` | `RetainedPaintPhase::Hits` in both paint entries + its `phase_name`; `UiWindow::hit_registry` and `RetainedPaintFrame::hit_candidates`; `register_retained_hit` and `RETAINED_HIT_REGISTRY_CAPACITY`; `Ui::window_hit_targets`; `RetainedPaintWalk::step`'s child origin now subtracts a scrollable parent's own offset — the ONE origin rule paint and hits share |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` | `owning_tree_spec`/`find_tree_item` are `pub(crate)`, so the registry re-derives a row's role through the SAME lookup the layout measures it with |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` | re-exports `RetainedHitRegistration` |
| `🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` | `ui::wgpu_engine` element size re-measured, 159 848 → 159 896 |
| `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `register_retained_hit_targets(window_id, input) -> Vec<String>` |
| `…/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `retained_hit_windows` state, cleared at `ShellChromeFramePhase::FrameSetup`; region `🎯️RetainedPointerRouting` (`retained_hit_window`, `register_retained_body_hits`, `retained_pointer_button`, `route_retained_pointer_move`, `route_retained_pointer_press`); registration from `render_main_window_step` phase 4 and `render_panel_step` phase 8, once per body per walk; `handle_pointer_move` and both halves of `handle_pointer_button` route a retained hit into its own body |
| `…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` | registers the new twin |
| `<ticket>/🐍️wgpu-hit-probe.mjs` | the hover witness and the wheel witness §5.1 describes |

**Added**

| file | what |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` | the language-neutral oracle — 4 cases, 14 entries, 7 points |
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs` | the Rust law over the live pipeline |
| `…/🧑‍🎨engine/🧪️tests/🎯️retained-hit-targets/🟦️.ts` | the TypeScript twin |

**Generated (disposable, under `<ticket>/🗑️generated/wgpu-hits/`)**: `rust-law.txt`, `ts-twin.txt`.

---

## 8. What is NOT claimed

* **Nothing on 6118.** §5. No click, no hover, no wheel, no selection and no screenshot is claimed,
  because the binary that would carry this lane could not be built.
* **The shell/interpreter half is not compile-verified.** §6. It is the smaller half by far and every
  type it touches is exercised by the `🖱️ui` laws, but "it compiles" is not claimed.
* **A `Toggle` in a retained body registers its hit but dispatches nothing.** `events::EventRouter`
  has no `UiNode::Toggle` arm and the registry deliberately does not invent one: the immediate-mode
  path dispatches `{"pressed": !pressed}` off `widget_maps.toggle_metas`, which the document path
  does not fill. Named, not papered over.
* **Tabs.** The brief lists them; the retained document contract has no tab component — the shell's
  panel tabs are chrome and already register `HitKind::PanelTab` themselves. Nothing was invented for
  a component that does not exist.
* **A tree row's trailing ACTION icons** still have no rect of their own on the document path
  (`📓️wgpu-tree-row-hit-test-2026-09-12.md` §8.1 named this): they are `TreeItemProps::row_actions`,
  props rather than child records, so they mount no arena node. The registry skips zero-area nodes
  rather than registering a garbage rect for them, so a click on a painted row-action icon lands on
  the ROW. Closing it still means giving those rects an owner in `layout::TreeRowGeometry`.
