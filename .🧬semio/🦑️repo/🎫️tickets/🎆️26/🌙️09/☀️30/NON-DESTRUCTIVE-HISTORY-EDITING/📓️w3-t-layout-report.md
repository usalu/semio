# 📓️ W3-T-LAYOUT Report: Layout Tool-Machine Conversion

Executor W3-T-LAYOUT (acde7cd8aaa4b17f0). Scope: the layout plugin (`✏️s/🔌️plugins/📏️layout`). Brief: `🧭️plan.md` "W3-T
brief"; contract: `📋️design.md` §5, §7, §10, §11, §13. Overlay protocol: W3-T-SPATIAL (aac13b60d073dd9a1), unchanged.

Aliases: `LA` = `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout`, `S1` = `LA/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`ED` = `S1/✏️editor`, `MU` = `S1/🧬️schema/🧬️mutations`, `FX` = `S1/🧫️fixtures/🧬️mutations`.

## 1. Census (before this work)

| # | Gesture / control | Entry | Commit today | Verdict |
|---|---|---|---|---|
| L1 | Gumball move (React `Canvas2dGumballOverlay`, Transform utility) | `translateSelection {ids, dx, dy}` incremental per pointer move | `Emit::amend(MoveFrame × N absolute, "gumball-translate")` per tick — STATIC coalesce key merges two consecutive drags | **G → tool machine** |
| L2 | Gumball rotate | `rotateSelection {ids, angle}` per tick | `Emit::amend(RotateFrame × N absolute, "gumball-rotate")`; every frame turned about its OWN centre | **G** |
| L3 | Gumball axis/uniform scale | `scaleSelection {ids, sx, sy}` per tick | `Emit::amend(MoveFrame + ResizeFrame × N absolute, "gumball-scale")`, extent clamped to ≥ 1 | **G** |
| L4 | Canvas pointer down / move / up | `canvasPointer*` | hit test → `interactionSelect` / `interactionHover` redispatch; no document edit | K (unchanged) |
| L5 | Catalogue drag & drop | `canvasDragOver/Leave/Drop` | drop preview in window transient; ONE `create-frame`/`create-page` on drop | O (unchanged) |
| L6 | Inspector fields (x, y, w, h, rotation, margins, …) | `patchFrame`/`patchPage`/`patchDocument`, number/text inputs with `commit("blur")` | one absolute leaf per committed field (`move-frame`, `resize-frame`, `rotate-frame`, …) | O (unchanged; absolute set is the intent) |
| L7 | Inspector colour fields (`fill`, `stroke`) | colour input, `Trigger::Change` | one `change-frame-fill/stroke` per change | C — owned by W3-T2-CONTROLS' framework `ScrubMachine` (no plugin code; handler already returns absolute leaves, see §6) |
| L8 | Keyboard nudges | — | layout declares none | — |
| L9 | wgpu host | wgpu `🎞️Scenes` canvas-2d sends `canvasPointer*` only | there is NO wgpu canvas-2d gumball (confirmed by W3-T-SPATIAL) | nothing to convert; gap noted in §6 |

No layout gesture was a state machine; none carried a transaction; the gumball wrote absolute per-tick amends.

## 2. Parametric leaves (schema-first)

Three new RELATIVE, selection-scoped leaves of `LayoutMutation` (appended, binary tags 46–48):

| Leaf | Payload | Semantics (read off the BASE bounds, replays on any base) | Label en / de |
|---|---|---|---|
| `drag-frames` ✋️ | `{pageId, targets[], dx, dy}` | origin += (dx, dy) | "Drag 2 frames by (16, -8)" / "2 Rahmen um (16; -8) ziehen" |
| `rotate-frames` 🔃️ | `{pageId, targets[], pivotX, pivotY, angle}` (rad) | centre orbits the pivot, rotation += angle, extent kept | "Rotate 1 frame by 90°" / "1 Rahmen um 90° drehen" |
| `scale-frames` 🗜️ | `{pageId, targets[], pivotX, pivotY, sx, sy}` | centre moves from the pivot by the factors, extent ×(sx, sy) | "Scale 1 frame by (2, 0.25)" / "1 Rahmen um (2; 0,25) skalieren" |

- Outcomes (9-code vocabulary only): Fatal `mutation.invariant` (empty/repeated targets, non-finite numbers, factor ≤ 0 —
  all expressed as schema hard bounds: `minItems: 1`, `uniqueItems`, `exclusiveMinimum: 0`; no `x-semio-invariant` needed),
  Error `mutation.target-missing` (page absent, or no target is an unlocked frame of it), Warning `mutation.partial` (one per
  reason: missing / locked), Warning `mutation.no-op` (identity). Inverse = absolute setters of the BASE origin/extent/rotation
  (`move-frame`, `resize-frame`, `rotate-frame`), never a negated parameter. Conflict `target()` = `[pageId]`.
- Shared region `🔖️FrameSelection` in `MU/🦀️.rs`: `layout_frame_selection_diff/inverse`, targets invariant, pivot (= centroid
  of the target frame centres, also used by the gumball meta layer), label helpers.
- **Diff shape change (needed for multi-frame leaves):** `PagePatch.frame_patched: Option<PageFramePatched>` →
  `frames_patched: Vec<PageFramePatched>` (every field-patched frame of the page, page order). All 9 single-frame leaves, 8
  leaf tests and the 15 committed diff fixtures rewritten (`🧪️w3-t-layout-frames-patched.py`). The diff JSON Schema, TS twin,
  proto and GraphQL `PagePatch`/`FramePatch`/`*Patch` definitions were brought to Rust parity at the same time
  (`🧪️w3-t-layout-diff-page-patch.py`; they had drifted since W2-S-E's fixture refresh — the TS document contract was red).
- Leaf schemas `MU/{✋️drag-frames,🔃️rotate-frames,🗜️scale-frames}/🧬️schema/🔣️.json`: full `x-semio-ui` en/de — `pageId`
  reference (kind page), `targets` reference (kind frame, domain `elements`, granularity `element`, many), offsets/pivot
  steppers in mm, angle dial (rad shown in deg, π/2 snaps), factors log sliders 0.1–10 with snaps 0.5/1/2.
- Surfaces: aggregate `🔣️.json` oneOf, `🔗️.graphql` types + union, `🟦️.ts` interfaces + union + `parseDragFrames`/
  `parseRotateFrames`/`parseScaleFrames`, `📝️text/{🔣️.json, 🔗️.graphql, 🛰️.proto (oneof 46–48), 📖️.grammar.semio, 🅰️.g4,
  🔤️.ebnf}`, `💾️binary/📡️.protocol.semio` records (also fixed the missing `field payload bytes` line of `set-drawing-text`),
  `KINDS`, oracle catalog (`🔮️oracles/🔣️.json`: vectors, kinds, manifests, coverage 29).
- Evidence: 17 fixture quintets under `FX/{✋️drag-frames,🔃️rotate-frames,🗜️scale-frames}/` (applied, partial, rejected
  target-missing, no-op, invariant negative witness per leaf) authored by an INDEPENDENT Python implementation
  (`🧪️w3-t-layout-author-vectors.py`), cross-checked by the third-party `numpy` (matrix transforms, 1e-9) and `jsonschema`
  (accepted vectors pass, invariant vectors fail), plus 17 generated Rust fixture test modules (canonical JSON, declared
  outcome, committed diff, diff completeness, inverse).
- Second implementation: `S1/🧪️tests/📐️mutate-layout-1/🐍️.py` gained the three kinds (multi-step inverse), the feature
  gained 6 rows, the Rust adapter 3 kinds.

## 3. Tool machine

- `ED/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🦀️.rs` — the Transform utility IS the transform tool:
  `machine::statechart! transform_tool` (`idle --Once[applies]/upsert+commit--> idle`, `idle --Stream[applies]/upsert-->
  streaming`, `streaming --Stream/upsert net leaf--> streaming`, `streaming --Finish/upsert|retract+commit--> idle`), effects
  `ToolYield<LayoutMutation>`, driven by `ToolMachineRunner`; one key `frames:0` so an open transaction holds ONE net leaf
  (offsets/angles add, factors multiply, only for the same page, targets and pivot).
- `LayoutTransformTool::{start, resume, send, abort, persist}`; state `LayoutTransformToolState {states, verb, authoringSeed,
  baseRevision, transaction, entries}` persisted in the Blueprint window transient (`LayoutWindowTransient.transform_tool`,
  boxed; Rust + JSON Schema + TS twin with `parseLayoutTransformToolState` + proto + GraphQL; retirement + preflight footprint).
- `layout_transform_dispatch` (one dispatch): `phase` absent = one-shot transaction; `stream` = tick into the open
  transaction (persisted, previewed, never history); `commit` = folds the tail and publishes ONE `Emit::commit_transaction`
  (no coalesce key, no description — the row label is the leaf's); `abort{reason}` = zero trace. A commit with no open gesture
  publishes nothing (a stray tail is never an edit). Another verb / a one-shot interrupts → `captureLost`; a document revision
  that moved under the gesture → `baseMoved`. A request whose every target is locked raises ONE localized notice
  (`frames_locked`, en/de).
- Host aborts: `ArtifactEditor::host_event` maps blur → `blur`, capture loss → `captureLost`, utility switch / closing window →
  `retired`, history edit → `frozen`, remote edit → `baseMoved`, dispatched as the window's typed `translateSelection{phase:
  "abort"}`.
- Verbs `translateSelection`/`rotateSelection`/`scaleSelection` (`ED/🎮️commands/🧭️gumball/🦀️.rs`) carry `ids` (pinned by the
  overlay at pointer-down; empty = current selection), the delta, `phase`, `reason`; publication contracts gained the
  `WindowTransient` lane. The pivot of a turn/scaling is the centroid of the target frame centres on the base, and the
  `meta:gumball` layer now draws exactly that pivot (`ED/🖼️canvas/🦀️.rs`), so the overlay's screen math and the leaf agree
  (multi-frame turns now orbit the common pivot instead of spinning each frame in place).
- Preview: `render_body` paints `layout_transform_tool_preview(document, state)` for the owning Blueprint window only.
- `ArtifactEditor::mutation_label` = `SemanticMutation::label` (rows read "Drag 1 frame by (30, -6)").
- Deleted: the three `Emit::amend(..., "gumball-*")` paths, the absolute MoveFrame/RotateFrame/ResizeFrame gumball emission,
  the `≥ 1` extent clamp. `move-frame`/`resize-frame`/`rotate-frame` stay — they are the inspector's absolute field leaves.

## 4. Verification

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/📏️layout` (cwd `🧰️…/🧪️test`) | **0 findings**, 146/146 inputs of 48 leaves |
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/📏️layout` | **0 findings**, 62/62 payloads, 48/48 leaves witnessed, 3 negative witnesses rejected |
| `.venv/bin/python 🧪️w3-t-layout-author-vectors.py` (idempotence + numpy + jsonschema) | 17 vectors checked, 0 files pending; negative control (x + 1e-6) caught by numpy |
| `.venv/bin/python 🧪️w3-t-layout-diff-page-patch.py` | 15/15 committed page patches valid against the new `PagePatch` (old definition: 2 errors on move-frame) |
| `.venv/bin/python 🧪️w2-s-e-layout-oracle.py` (second implementation standalone) | **58 passed, 0 failed** (29 kinds × mutate/inverse) |
| `SEMIO_TEST_OUTPUT_SCOPE=w3-t-layout/oracle bun ./📜️script.ts oracle exhaustive --case 📐️mutate-layout-1` | 58/59 passed; the 1 errored is `identity-round-trip`, subject-only by design ("adapter has no oracle registration") |
| `bun ./📜️script.ts verify layout-document-contract` (LA package; Ajv + TS twins + tsc) | exit 0 — 90 snapshots, 39 committed diffs (was red before) |
| `bun ./📜️script.ts verify layout-window-ownership` (Ajv + twins + tsc) | exit 0 (config schema `activeUtility` drift fixed; transient row with an open transform tool added) |
| `bun ./📜️script.ts verify layout-frame-selection` (new: bun test TS twin vs Ajv + tsc) | exit 0 — 4 pass, 65 expects |
| `cargo check -p semio-s-artifact-layout-layout --tests` | see §5 |

## 5. Rust compile and test status

PENDING — filled in below once peers' in-flight edits in `semio-framework-plugin-host` (mismatched delimiter,
`🔌️plugin/🖥️host/🦀️.rs:3513`), `semio-framework-plugin` (`dsl::Edit` has no field `verb`) and `semio-framework-os-kernel`
(`🏪️store/🧩️composition/🚪️open/🦀️.rs:352` macro token) stop breaking the dependency graph.

## 6. Open items and notes

1. **wgpu**: there is no canvas-2d gumball in the wgpu shell, so the layout Blueprint gesture exists in React only; wgpu
   reaches the same leaves through the one-shot verbs (palette/agent). A wgpu canvas-2d gumball is a renderer feature outside
   this scope.
2. **Colour fields (L7)** dispatch `patchFrame{field: fill|stroke}` per change; the handler already returns the absolute
   `change-frame-fill/stroke` leaf, so W3-T2-CONTROLS' runtime scrub glue covers it without plugin code (if the colour input
   carries `gesture`/`commit`; not verified here).
3. Inherited (parent-page) frames are not moved by the gumball (as before); the gumball is not drawn for a selection of only
   inherited frames (no pivot on the page).
4. `verify taxonomy report --scope …` is blocked repo-wide by "Nested Cargo catalog digest drift" (central catalog).
5. Regeneration owed by the coordinator: layout plugin descriptor (`describe`: rotate/scale descriptions, publication
   contracts), central `schema generate` (new leaf schemas, diff schema, transient/config schemas), launch rows for the new
   `verify layout-frame-selection` segment.

## Session 2 — 2026-10-01

Successor S2-LAYOUT (session `⚪552b484a…`). Scope: layout plugin tree + the shared `Canvas2dGumballOverlay` (React + wgpu).
Aliases as above; `CH` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost`,
`SC` = `…/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, `F2` = fem 2d `…/🌐️any/✏️editor`.

### S2.1 Repair (rule 21)

Files the predecessor touched after its 03:07 report: only `🧪️transform-tool-transactions`, the transform unit test and
`🩹️patch-frame` (07:45–07:46); the 08:04 build compiled them (588 passed / 2 failed). No half-finished edit found.
- **Red 1 `one_shot_turns_and_scalings_are_one_transaction_each` — root cause confirmed by reading the store contract**
  (`🏪️store/🦀️.rs` `ArtifactStoreOneItemFootprint`, `fold_batch_item`): `work_items` counts staged ROWS (forward + every
  inverse row); the layout artifact lane ran on `bounded_config_store_one_item_preparation_factory` = 2 rows, while
  `rotate-frames`/`scale-frames` invert to 2 absolute rows PER TARGET (`move-frame` + `rotate-frame` / `resize-frame`) and a
  multi-frame `drag-frames` to one per target (the one-frame drag fit by luck). Fix (leaf-side declaration, no framework
  helper — §17.7 is CLOSURE's): `mutations::layout_mutation_inverse_rows` (per-leaf bound, `🔖️FrameSelection` region) and
  `LayoutArtifactStorePreparationFactory` in `ED/🦀️.rs` (region `🧺️ArtifactPreparation`) wrapping the bounded factory and
  declaring `ArtifactStoreOneItemFootprint::for_one_item(bound, bytes)`. Laws: `every_committed_inverse_fits_its_declared_fold_footprint`
  (walks all 45 committed quintets + exact 2-frame drag/turn/scaling rows) and the app-level 2-frame one-shot turn/scaling +
  `a_streamed_drag_of_two_frames_is_one_transaction`.
- **Red 2 "a page row keeps its own action"** — stale test, not a product bug: since the 09-30 00:11 framework commit
  `tree_item_with_action` stores the row's action as its `RowTarget` (`activation`), not a binding. Test now asserts
  `target.scope == LAYOUT_PLAY_APP_ID && activation == "setActivePage"` and that pick rows carry no target.
- "Locked frame visibility toggle" (status 17:00 triage) = `patch_page_reorders_and_deletes_and_patch_frame_sets_flags`; it
  was red at 07:45 and green at 08:04 (predecessor's `🩹️patch-frame` edit).

### S2.2 Census (assignment list vs. reality)

| Gesture | Exists in layout? | Now |
|---|---|---|
| frame drag / resize / turn | only via the Transform-utility gumball (no canvas body drag) | transform tool → `drag-frames` / `scale-frames` / `rotate-frames`, ONE `ToolTransaction` (S1 §3) |
| marquee | no (canvas pointer = click-select + hover only) | – |
| keyboard nudges | no keybinding / verb | – (follow-up: nudge verbs would reuse `translateSelection` one-shot; needs new verbs + describe) |
| inspector fields x/y/w/h/rotation | absolute field commits on blur (one dispatch) | unchanged: absolute set leaf is the intent (§13.1); continuous colour inputs ride the CONTROLS scrub glue |
| page-row actions | document tree page row = `setActivePage` (view config, never history); inspector page buttons = discrete `patchPage` (one leaf each) | unchanged (discrete commands, not tools) |
No `Emit::amend`, `coalesce_key` or scratch remains in the layout tree (`git grep`, 0 hits).

### S2.3 Canvas2d gumball — one protocol, both hosts (decision 21:36)

- **Schema-first contract** `CH/🧬️schema/🔣️gumball-meta/🔣️.json` (the `meta:gumball` layer) and corpus schema
  `CH/🧬️schema/🔣️gumball-dispatch/🔣️.json`. Protocol changes (all producers converted at once, no compat):
  `liveDispatch` (true = stream/commit/abort, false/absent = local preview + ONE one-shot pose delta on release, cancel = no
  dispatch); domain-neutral `modelToLayer {scale, offset}` replaces the fem-specific `space: "fem2d" | "world"` constants;
  dead `pivotModel` and fem2d's dead `camera` deleted (hosts measure in their own live camera about `pivotLayer`).
- **Shared corpus** `CH/🧫️fixtures/🧫️gumball-dispatch/🔣️.json` (23 cases: live/non-live × move/turn/scale, zoom+pan,
  scaled+flipped model map, turn past ±π unwrapped (was a full-turn jump before), release-without-move, return-to-press,
  cancel/blur, hit priority knob-over-ring, ring band, disabled handles), authored by an INDEPENDENT numpy implementation
  `T/🧪️s2-layout-gumball-dispatch-corpus.py` and validated with Python `jsonschema` (+ 2 negative schema checks).
- **React** `CH/🟦️GumballOverlay.tsx`: pure algebra (`canvas2dGumballHandleAt/Begin/Total/Increment/Drag/Release/Cancel`),
  no runtime import of the host module (cut the heavy chain; host camera restated beside it), wider ring hit band (±4 px,
  was the 1.5 px stroke), non-live ghost preview; host `🟦️.tsx`: transform utility keeps middle-button pan (parity with wgpu).
  Test `CH/🧪️tests/🧪️gumball-dispatch/🟦️.ts` (Ajv strict + replay + `gl-matrix` net-motion oracle), replaces the old
  `🔬️gumball-transform-delta` suite (registered in the React vitest config).
- **wgpu twin (new)** `CH/🎯️targets/🧊️wgpu/🦀️.rs` mounted as `canvas2d_gumball` in `🧊️renderer/🦀️.rs`: same algebra,
  handle paint + non-live ghost, gesture slot with pointer/window/host/generation ownership; `SC` routes press/move/release,
  pointer cancel, Escape, window close, stale generation and retirement through it (Transform utility: a primary press
  grabs a handle or reaches nothing, as in React). Tests: corpus replay `CH/🧪️tests/🧪️wgpu-gumball-dispatch/🦀️.rs` and
  three seam laws in `🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs` (`🧭️Gumball` region).
- **Producers**: layout `ED/🖼️canvas/🦀️.rs` emits `liveDispatch: true` (validated against the schema in its unit test
  with `semio_framework_schema::OwnedJsonSchemaValidator`); fem 2d `F2/🕹️interaction/🧭️gumball/🦀️.rs` emits `liveDispatch`
  + `modelToLayer` from `SCALE_2D`/`ORIGIN_2D`, `camera` parameter removed (callers in `📊️results`, `🧱️model`, 2 unit tests),
  dead `fem2d_gumball_meta_for_render` deleted. **S2-SPATIAL: please re-read these four fem 2d files.**

### S2.4 Replay laws (c)

`ED/🧪️tests/🧪️transform-tool-transactions/🦀️.rs` region `⏪️TimeTravel`, through the real `historyEdit*` verbs:
`a_gumball_drag_and_its_downstream_turn_edited_in_time_travel_replay_exactly` (edit the gumball drag's `dx` → review never
touches the committed head → overwrite → head == fresh fold with the downstream turn about its recorded pivot; then edit the
turn's `pivotX` → exact; ids/rows never re-minted) and `a_drag_edited_onto_a_missing_frame_blocks_finalize_until_its_targets_are_fixed`
(targets → missing frame = Error `mutation.target-missing`, blocking; `historyEditUseSelection{path:"/targets"}` after an
`interactionSelect` clears it; Exit = zero trace).

### S2.5 Verification (so far)

| Command | Result |
|---|---|
| `.venv/bin/python T/🧪️s2-layout-gumball-dispatch-corpus.py --check` | corpus current: 23 cases, 20 pressed handles, 33 dispatches; jsonschema + 2 negatives OK |
| React pkg `bun ./📜️script.ts test long gumball-dispatch` | **36 passed** (`G/s2-layout/vitest-gumball-4.txt`, 16:48 rerun) |
| React pkg `bun ./📜️script.ts typecheck` | 13 errors, **0 in my files** (peer files: plugin registry generated, ui contract, store worker, Shell) |
| React pkg `bun ./📜️script.ts test long input-contract` | could not load: peer break in `🔌️plugin/📇️registry/🤖️generated/🧩️plugins` (`inventory.filter` of undefined) |
| `schema mutation-inputs --under ✏️s/🔌️plugins/📏️layout` | **0 findings**, 146/146 inputs, 48 leaves |
| `schema mutation-payloads --under ✏️s/🔌️plugins/📏️layout` | **0 findings**, 62/62 payloads, 48/48 witnessed, 3 negatives |
| LA pkg `bun ./📜️script.ts verify layout-frame-selection` | exit 0 — 4 pass, 65 expects |
| LA pkg `bun ./📜️script.ts verify layout-window-ownership` / `layout-document-contract` | exit 0 / exit 0 (90 snapshots, 39 diffs) — after completing the path-budget renames below |
| `.venv/bin/python T/🧪️w2-s-e-layout-oracle.py` | 58 passed, 0 failed |
| `.venv/bin/python T/🧪️w3-t-layout-author-vectors.py` | 17 vectors vs numpy + jsonschema, **0 files pending** (script re-aligned, see S2.6) |
| `verify taxonomy report --scope CH` | my new dirs clean; findings on `🟦️GumballOverlay.tsx` naming (pre-existing since 09-16, see S2.6) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-layout cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-layout-layout --lib` | **595 passed, 0 failed** (17:43, `G/s2-layout/test-9.txt`; was 588/2). On the way: peer os-kernel breaks (12:10, 17:04), a lock cycle (13:04, own cargo killed), the usage cut, a disk-guard prune of the build dir (16:53), the half-renames (S2.6) and one regression from the stdio sqlite rollout — `JsonSnapshot` lost its JSON-text `ArtifactDsl`, so the layout JSON export serializer now parses with `stdio.json`'s `parse_json_text` (`🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/…/🦀️.rs`) |
| `cargo check --manifest-path 🌎️hub/Cargo.toml --target wasm32-wasip2 -p semio-hub-layout` | **exit 0** (17:50, `G/s2-layout/wasm-1.txt`) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-layout cargo test -p semio-framework-os-renderer-wgpu --lib -- gumball` | **5 passed, 0 failed** (22:02, `G/s2-layout/wgpu-3.txt`): corpus replay, meta parse, live seam, cancel/non-live seam, paint |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-layout cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-fem-2d --features component-app-assembly --lib -- gumball` | compiles (my 4-file fem 2d edit type-checks); **1 passed / 12 failed, all on the demo fixture**: `Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL)` → `TextError "expected LBrace, found Ident 'case-id'"` at line 61 — the fem 2d example no longer parses under the peer DSL/sqlite rollout (not the gumball change; the meta assertions never ran). **S2-SPATIAL / DSL owner** (`G/s2-layout/fem2d-2.txt`, 23:10) |
| first run 22:30 | died on a full disk (283 MiB free, `No space left on device` writing rlibs; disk guard recovered to 10 GiB) |

### S2.6 Repairs of peer half-renames in the layout tree (path budget)

The repo path-budget renamer (finished per coordinator) rewrote references but left three directories at their old names,
so the layout test build failed on two `include_str!`: moved (plain `mv`) `🧬️schema/🧫️fixtures/🪪️document-contract` →
`🪪️document`, `🧬️schema/🧪️tests/🪪️document-contract` → `🪪️document` (the LA `📜️script.ts` already imported it there),
`📐️blueprint/🎚️config/🧫️fixtures/🔬️window-ownership` → `🔬️window`. The renamer also shortened the 13 frame-selection case
folders (`⚠️skips`, `🔁️refuses`, `🚫️rejects`, `🔃️orbits`, …) while keeping the descriptive module names and messages;
`🧪️w3-t-layout-author-vectors.py` gained a `📏️PathBudget` folder map so it regenerates exactly the committed tree.

Since the 17:43 green run I removed `unnecessary qualification` warnings from my new laws (transform-tool tests:
`LayoutMutation`/`LayoutSnapshot`, `layout_select_action_args`, `INTERACTION_SELECT_ACTION_ID` are in scope) — **re-run of the
layout lib tests pending** (fleet rule 26 cargo hold 02:45–03:00; 03:03 retry blocked by the peer dsl refactor: os-kernel E0308/E0618 `RecordSpecProducer` in `🗣️dsl` + `🏪️store`).

### S2.7 Open / coordinator

1. Taxonomy (S2-TAX): `CH/🟦️GumballOverlay.tsx` is an unregistered named leaf (`kind-only-basename`, `semantic-stem-unresolved`,
   normalization collision with `🟦️.tsx`; the two importers get `reference-edit-required`) — pre-existing since 09-16. Element
   sub-directories only admit registered member kinds (`members-of-members-of-elements` = local-catalog/local-folders), so the
   proper home `CH/🧭️gumball/🟦️.tsx` needs a registry entry first; not moved here.
2. wgpu Canvas2d does not paint layout's `segments` path layers (only `x/y/width/height`, polyline, text, image) — layout
   frames are invisible on wgpu while the gumball works; renderer-owner gap (`🎞️Scenes` `render_canvas_2d`).
3. Regeneration owed: layout descriptor (`describe`), central `schema generate` (two new Canvas2dHost schemas), launch rows
   for the new React suite + Python corpus script. Re-activation of layout (6079/6179) and fem 2d after the cargo proofs.

## Session 3 — 2026-10-02

Successor S3-LAYOUT (session `⚪b7db773a…`, started 10:57). Scope: layout tree, `Canvas2dGumballOverlay` (React + wgpu twin),
and the wgpu Canvas2d path-layer painting gap (S2.7 (2)), region-scoped in `🎞️Scenes` (the wgpu shell is S3-W2C's).
Scratch: `T/🗑️generated/s3-layout/` (`G3`). Aliases as in S1/S2.

### S3.1 Repair (rule 28)

Files in my trees newer than the S2 section (03:03): only a PEER's `EngineHandles` move (05:41–05:58, clean-architecture
layering): `ED/🦀️.rs` + `👁️viewer/🦀️.rs` now `use semio_framework_2d::compute::EngineHandles`, layout `Cargo.toml` gained
`semio-framework-2d`. Not mine; kept. No half-finished edit of this WP found.

### S3.2 Verification (in progress)

| Command | Result |
|---|---|
| `.venv/bin/python T/🧪️s2-layout-gumball-dispatch-corpus.py --check` | exit 0 — corpus current: 23 cases, 20 pressed handles, 33 dispatches (jsonschema + 2 negatives) |
| `.venv/bin/python T/🧪️w3-t-layout-author-vectors.py` (dry) | exit 0 — 17 vectors vs numpy + jsonschema, **0 files pending** |
| `.venv/bin/python T/🧪️w2-s-e-layout-oracle.py` | **58 passed, 0 failed** |
| `schema mutation-inputs --under ✏️s/🔌️plugins/📏️layout` (cwd `…/🧪️test`) | **0 findings**, 146/146 inputs, 48 leaves |
| `schema mutation-payloads --under ✏️s/🔌️plugins/📏️layout` | **0 findings**, 62/62 payloads, 48/48 witnessed, 3 negatives |
| LA pkg `bun ./📜️script.ts verify layout-frame-selection` / `layout-window-ownership` / `layout-document-contract` | exit 0 (4 pass, 65 expects) / exit 0 / exit 0 (90 snapshots, 39 diffs) |
| React pkg `bun ./📜️script.ts test long Canvas2dHost` | **7 files, 89 passed** (gumball-dispatch, gesture-sample-lane, path, paint, input-contract — unblocked —, peer-presence, tool-run-trace) |
| root `bun ./📜️script.ts verify taxonomy report --scope CH` | BLOCKED: peer `🏛️program/📦️packages/🦀️rust/📋️project.json:11` one-word workspaceCommand `["test-snapshot-sqlite"]` breaks root routing (reported to `main` 11:06) |

### S3.3 wgpu Canvas2d `segments` paint (S2.7 (2)) — source written 11:15–11:50

React paints every record with `segments`, `text` or `image.src` through `drawSceneNode` (`CH/🎨️paint/🟦️.ts`: `Path2D` from
`pathSegmentsToSvgD`, node `transform`, fill `evenodd` unless `nonzero`, stroke width/cap/join/dash in layer units); the
wgpu `render_canvas_2d` ignored `segments` and drew such a record as an 8×8 box at (0, 0) — every layout frame, page and guide.
- **Shared corpus, schema-first:** `CH/🧬️schema/🔣️path-paint/🔣️.json` + `CH/🧫️fixtures/🧫️path-paint/🔣️.json` (16 cases,
  68 probes: layout frame/turned frame/margin guide/inherited dash exactly as `ED/🖼️canvas` emits them, evenodd vs nonzero
  holes, self-intersecting star, arc pie (chord ≠ arc), rotated large arc, quad + cubic bulges, node transform, segment
  after `close`, open subpath fill, thick stroke over fill). Authored by an INDEPENDENT numpy implementation
  `T/🧪️s3-layout-path-paint-corpus.py` (dense sampling, exact winding, centreline distance, dash walk) that refuses any
  probe closer than 1 px to a fill/stroke/dash boundary or 4·half-width to a join/end; jsonschema + 2 negatives.
- **Rust twin** `CH/🎨️paint/🦀️.rs` (mounted `canvas2d_paint` in `🧊️renderer/🦀️.rs`): SVG path semantics (leading move,
  restart after close, skip unknown, stop on missing field), SVG F.6.5/F.6.6 arcs, Wang-bounded Bézier flattening (0.2 px),
  sweep tessellation into trapezoids (crossing-split bands, evenodd/nonzero, clipped to the canvas rows), convex stroke pieces
  (miter limit 10 → bevel, round/bevel joins, butt/square/round caps, canvas dash rules), trapezoid pieces for gradients.
- **Scenes** (`SC`, region `Canvas2d` only): `CanvasLayer` gains `segments`/`transform`/`fillRule`, `CanvasStrokeJson`
  `cap`/`join`; `render_canvas_2d` routes exactly React's scene-node records to `render_canvas_scene_node` (fill solid or
  gradient pieces, stroke ≥ 1 px, zoom-scaled text lines via `◻️2d` `drawing_text_lines`, `image.src` at the node origin),
  the image branch now needs `dataUrl` and a `kind: "text"`/`polyline` record without its data falls through to bounds (React
  parity). `CanvasPaintOrder` keeps record order across the draw list's buckets (a quad after vector paint opens a fresh
  scissored layer — glyphs no longer sink under later fills) and clips the canvas to its rect. Renderer `Cargo.toml` gains
  `semio-framework-2d` (workspace) for the text line law.
- **Tests:** Rust `CH/🧪️tests/🧪️path-paint/🦀️.rs` (corpus replay + leading-move/truncation, restart-after-close,
  flatness bound under a skewed map, dash/miter/cap rules, gradient piece tiling); React `CH/🧪️tests/🧪️path-paint/🟦️.ts`
  (registered in the React vitest config): Ajv strict, `drawSceneNode` against a recording canvas, every probe re-derived
  by Three.js `SVGLoader` (third-party SVG parser, arcs, Béziers, stroke builder).

| Command | Result |
|---|---|
| `.venv/bin/python T/🧪️s3-layout-path-paint-corpus.py --check` | exit 0 — 16 cases, 68 probes (12 stroke, 24 fill, 32 none) |
| React pkg `bun ./📜️script.ts test long path-paint` | **18 passed** (schema + 16 cases + coverage) |
| negative control: 4 probes flipped in the corpus, same run | **4 failed / 14 passed** as expected; corpus regenerated, `--check` current |

### S3.4 Author script emits the current slugs (S2.7 (1b))

`T/🧪️w3-t-layout-author-vectors.py`: the `📏️PathBudget` rename map (`FOLDERS` + `folder_of`) is gone — every `CASES` row now
names its committed case folder slug (`skips`, `refuses`, `orbits`, …) beside its descriptive test-module slug, so the
generator writes the §14 tree directly (no rename step, no codemod). Dry run: **17 vectors vs numpy + jsonschema, 0 files
pending** (byte-identical to the committed quintets, Rust case tests and crate mounts). S2-CODES' "79 files" note predates
S2-LAYOUT's S2.6 re-alignment.

### S3.5 Gumball overlay home (S2.7 (1a)) — registry does not allow it

`verify taxonomy report --scope CH` (11:59, `G3/tax-ch-2.txt`): 16 errors, **none on the S3 additions** (`🎨️paint/🦀️.rs`,
`🧪️tests/🧪️path-paint`, `🧫️fixtures/🧫️path-paint`, `🧬️schema/🔣️path-paint`). `🟦️GumballOverlay.tsx` keeps its 7
pre-existing findings (kind-only-basename, semantic-stem-unresolved, 5 normalization collisions with `🟦️.tsx`) plus 2
`reference-edit-required` at its importers (`CH/🟦️.tsx:19`, `CH/🧪️tests/🧪️gumball-dispatch/🟦️.ts:26`); the other 7
(`🧪️tests|🧫️fixtures/{👕️peer-presence,🖊️path,🖱️gesture-sample-lane,🖱️input-contract}` directory-kind-unresolved) are
not this WP's. `🧭️gumball` is registered only in `members-of-commands`; `members-of-members-of-elements` admits only
`🗂️local-catalog`/`📎️local-folders`. Not moved — TAX action: register `🧭️gumball` as an element member, then move
`CH/🟦️GumballOverlay.tsx` → `CH/🧭️gumball/🟦️.tsx` (and the wgpu twin `CH/🎯️targets/🧊️wgpu/🦀️.rs` →
`CH/🧭️gumball/🎯️targets/🧊️wgpu/🦀️.rs`) with the two importers in one edit.
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-layout cargo test -p semio-framework-os-renderer-wgpu --lib -- canvas2d` (12:26, `G3/wgpu-1.txt`) | **did not reach the tests**: the crate's ONLY error is a peer's — `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6485:124` E0596 (`end_every_text_editor_typing`, mtime 12:08, owner guess S3-TEXT; reported to `main`). 0 errors / 0 warnings in `CH/🎨️paint`, `CH/🧪️tests/🧪️path-paint`, the Scenes `Canvas2d` region |
| React pkg `bun ./📜️script.ts typecheck` (includes `🧱️elements/**`) | 4 errors, **0 in my files** (peers: `🏪️store/👷️worker/🟦️.ts:3842` + backbone-parity `line` field, `🐚️Shell/🟦️.tsx:1112` `idleInstalledServiceStatusV1`) |
| scratch standalone harness `G3/paint-proof` (own `[workspace]`, private build-dir + target-dir, mounts the REAL `CH/🎨️paint/🦀️.rs` + its tests) `cargo test --offline` | **6 passed, 0 failed** (corpus 68/68 probes, leading move/truncation, restart after close, flatness under a skewed map, dash/miter/cap/round rules, gradient tiling); 1st run caught a wrong expectation of mine (round join point count at a 1 px half width — flatness-bounded, test now checks the fan on the half-width circle at width 20) |
| `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-layout-layout --lib` (12:36, `G3/layout-test-2.txt`) | **blocked by a peer**: `semio-framework-schema-registry` 28 errors — `🧬️schema/📇️registry/🦀️.rs` (12:20) has duplicated `ArtifactSchemaRegistry`/`SchemaDescriptorRegistryError` blocks + missing `parse_json`; reported to `main` 12:37 (1st attempt 12:30 was SIGKILLed while waiting on a lock, cause unknown, no guard logged it) |

### S3.6 N3 hook: layout reference chips (coordinator 12:31)

`ED/🦀️.rs` region `🪧️EntityLabels`: `ArtifactEditor::entity_label` → `layout_entity_label` — a frame chip reads its kind word
(layout terminology, every locale × terminology) then content and page: `Text Frame “Hello layout” · Page 1` /
`Textrahmen „Hello layout“ · Page 1`, `Image Frame “missing.png” · Page 1`, `Rechteck · Page 1`, `Rectangle 1 · Page 1` /
`Rectangle 2 · Page 1` when several frames of one kind share a page, parent-page frames name their parent page. Pages (and
layers/styles) keep the generic walk (`name`: "Page 1"), unknown ids the generic `<Kind> <short id>`. Law
`reference_chips_name_frames_by_kind_content_and_page` (transform-tool tests, `⏪️TimeTravel`); "Use selection" filling the
`/targets` frame reference is the existing law `a_drag_edited_onto_a_missing_frame_blocks_finalize_until_its_targets_are_fixed`
(WRITTEN, run pending with the layout lib tests).

### S3.7 Peer breaks met while verifying (all reported to `main`)

| When | Break | Owner guess |
|---|---|---|
| 11:06 | root `📜️script.ts` routing: one-word `["test-snapshot-sqlite"]` workspaceCommand (`🏛️program/…/📋️project.json:11`) | fixed by S3-INFRA ~11:40 |
| 12:26 | renderer wgpu `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:6485` E0596 | S3-TEXT (fixed 12:27) |
| 12:36 | `🧬️schema/📇️registry/🦀️.rs` duplicated blocks (28 errors) | fixed 12:40 |
| 12:47 | `🗣️dsl/✨️derive/🦀️.rs:2202` emits `::semio_framework_schema_state::StateClass` (12:24) but deriving crates lack the dependency — `🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs:48` E0433 first (`G3/wgpu-3.txt`) | dsl-derive / schema-state author |
