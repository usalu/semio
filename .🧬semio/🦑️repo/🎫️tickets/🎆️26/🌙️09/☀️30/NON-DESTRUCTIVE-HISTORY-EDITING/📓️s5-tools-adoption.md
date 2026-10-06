# 📓️ S5-TOOLS — tool adoption census (who still mutates outside a `ToolTransaction`)

Author: S5-TOOLS, 2026-10-05 09:50. Method and limits are stated per table; nothing here was compiled or run beyond `git grep`
over tracked files. Paths are below `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/…/✏️editor/` unless written out. The full
gesture census with every file:line is `📓️s4-tools-a-report.md` § S5.2; this file is the adoption view of it.

## Counts first

| What | Count |
|---|---|
| Artifact editors that drive a tool machine | 15 |
| Streamed gestures on the shared `drive_gesture` (class D) | 6: fem 2d, fem 3d, lowpoly paint, generation3d, raster, wfc bitmap |
| Streamed gestures with a hand-written driver beside it (class W) | 3: layout transform, note ink, puzzle 2d select |
| Long-lived runner in an app-owned session (class L) | 1: draw canvas tool |
| One-shot runners (`start` + one `send`, class O) | 9: lowpoly, generation3d mesh selection, shooting, puzzle 2d one-shot, puzzle 3d, puzzle 5d, raster fill, process3d, cad |
| Sites on the shared node-drag machine (class N) | 12 in 10 artifacts |
| Transactions stamped with no machine (class X) | 1: remodel streamed ingest (design §15, stays by decision) |
| **Streamed gestures on the framework window slot (`context.gesture()`)** | **0 of 10** (D 6 + W 3 + L 1) — the slot landed 01:26, nobody adopted yet |
| Per-editor host-fact → abort arms still on disk | 5: generation3d, layout, puzzle 2d, raster, wfc bitmap |
| Plugin-owned persisted-gesture wire types still on disk | 8: `FemGumballGesture`, `LowpolyPaintGesture`, `GumballGesture` (generation3d), `LayoutTransformToolState`, `NoteInkToolState`, `Puzzle2dSelectToolState`, `RasterStrokeToolState`, `BitmapBrushToolState` |
| Interactive editor files that emit plain mutations with no transaction marker in the same file (grep, see below) | 26 files in 9 artifacts; by name 7 are pointer-driven, 19 are one-shot actions |

## Pointer-driven entry points that publish a plain edit (grep-level candidates, NOT read)

Method: tracked `✏️editor/**/🦀️.rs` outside `🧪️tests` that (a) emit `Emit::mutations(` / `artifact_mutations:` and (b) mention a
pointer, drop, drag, gumball, paint or brush word and (c) name none of `commit_transaction`, `stream_transaction`,
`node_drag_emit`, `drive_gesture`, `ToolMachineRunner`, `gesture_emit`, `lowpoly_tool_emit`, `cad_transform_tool_emit`. File-level:
a file that routes through a helper in another file is a false positive, a plain edit behind an unlisted helper is missed.
Lists: `🗑️generated/s5-tools/census-{plain-emit,transaction,interactive,outside}.txt`.

| Owner WP | Artifact | File | Why it is a candidate |
|---|---|---|---|
| S5-TOOLS | layout | `🎮️commands/📥️canvas-drop/🦀️.rs` | a drop is a pointer gesture; publishes a plain edit |
| S5-TOOLS | draw | `🎮️commands/📥️drop-layer-kind/🦀️.rs` | same |
| S5-TOOLS | block 3d | `🎮️commands/📍️place-vortex/🦀️.rs` | world click placement |
| S5-TEXT-STDIO | stdio png | `🎭️modes/✏️edit/🎮️commands/🎨️paint-native-region/🦀️.rs` | paint |
| S5-TEXT-STDIO | stdio tiff | `🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs` | paint |
| S5-TEXT-STDIO | stdio bmp | `🎭️modes/✏️edit/🎮️commands/🎨️paint-region/🦀️.rs` | paint |
| S5-TEXT-STDIO | stdio pdf | `🖼️page/🦀️.rs` | page canvas |

The other 19 hits are named actions (draw ×14: delete-selection, edit-path, add-layer, set-selected-opacity, edit-selection,
edit-fill, toggle-layer-visible, duplicate-layer, engagement-submit, combine-boolean, delete-layer, move-layer, patch-layers,
patch-layer; lowpoly ×2: add-primitive, object; raster ×3: apply-filter, transform-image, fill-selection). Under the tool contract
(`ArtifactApp` § ToolContract) an action executes once and its edit is one history row already; they matched on a word in a
comment or a shared import. They are listed so nobody has to re-derive that.

## Order of adoption (mine) and what each needs

1. `ArtifactView::provisional()` (framework, one `landing` wave) — fem 3d renders the preview without the committed revision's
   cached visuals and solve, so a render must know its snapshot is provisional (§ S5.7 of the report).
2. fem 2d + 3d (11 files, steps in § S5.7), then lowpoly paint, generation3d (its `ids` go into `GestureChart::context`; its
   Frozen arm goes), layout (wrapper + arm + four schema twins), note (tick reads `context.gesture().open()`).
3. draw: needs a slot whose gesture may hold no open transaction (`GestureState.transaction: Option<…>`); design to `main` first.
4. The three drop / place candidates above: read them, then either route through a one-shot tool (`drive_chart_gesture`) or
   record why they are actions.
