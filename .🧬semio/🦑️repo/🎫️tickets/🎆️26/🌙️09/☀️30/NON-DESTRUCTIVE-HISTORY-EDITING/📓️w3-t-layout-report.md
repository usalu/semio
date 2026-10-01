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
