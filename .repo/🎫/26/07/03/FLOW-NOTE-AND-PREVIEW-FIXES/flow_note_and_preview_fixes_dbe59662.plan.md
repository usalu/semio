---
name: Flow Note and Preview Fixes
overview: "Fix four Flow canvas issues: make Note's default size match a Slider, support native (non-DOM-overlay) double-click text editing on Notes, left-align Note text, and left-align/truncate the dictionary preview tree while keeping its fixed node width."
todos:
  - id: note-size
    content: Make note_widget_size match slider size (40x14) in dag rs; update stale height assertion in flow/core/rs/lib.rs
    status: completed
  - id: note-align
    content: Left-align Note text paint anchor in dag rs
    status: completed
  - id: preview-truncate
    content: Add left-aligned truncation-with-ellipsis helper for preview tree rows and note text, keeping fixed node width
    status: completed
  - id: note-edit-engine
    content: Add transient editing_note/caret state and begin/insert/backspace/delete/move/commit methods to DagHost, plus native caret painting
    status: completed
  - id: note-edit-flow
    content: Wrap DagHost note-edit methods in FlowHost with undo gesture grouping and sync_from_dag; expose via FlowSession wasm bindings
    status: completed
  - id: note-edit-react
    content: Wire double-click, keydown routing, click-away commit, and RAF caret blink in flow/react/index.tsx without any DOM overlay
    status: completed
  - id: tests
    content: Extend existing Rust and React test files to cover all of the above (no new test files)
    status: completed
isProject: false
---

# Flow Note and Preview Fixes

The Flow canvas is a WASM/Vello-painted graph (not React Flow). Node sizing/painting/hit-testing lives in [mathematical/graph/port/directed/dag/rs/lib.rs](mathematical/graph/port/directed/dag/rs/lib.rs) (`DagHost`, shared by all DAG-based apps: flow, procedural/2d, procedural/3d, draw, wires, nakagin, ...). Flow-specific widget/undo/eval logic lives in [flow/core/rs/lib.rs](flow/core/rs/lib.rs) (`FlowHost`/`FlowSession`). The React shell is [flow/react/index.tsx](flow/react/index.tsx) (`FlowCanvas`).

## 1. Note default size = one Slider

`note_widget_size` currently ignores its text argument and returns a preview-style height (`DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE) + DAG_PREVIEW_PAD*2` = 28), double the slider's height:

```829:832:mathematical/graph/port/directed/dag/rs/lib.rs
pub fn note_widget_size(_text: &str) -> (f64, f64) {
    (DAG_COMPONENT_WIDTH, DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE) + DAG_PREVIEW_PAD * 2.0)
}
```

Change it to match `slider_widget_width`/`slider_widget_height` exactly: `(DAG_COMPONENT_WIDTH, DAG_CHANNEL_ROW_HEIGHT)` (40 x 14, same as a Slider). This is read by `fit_node_size`'s `DagNodeKind::Note` arm (line 1015-1019) and every note creation path, so nothing else needs to change.

Update the now-stale assertion in [flow/core/rs/lib.rs](flow/core/rs/lib.rs):

```4634:4635:flow/core/rs/lib.rs
        assert!(node.width >= 40.0);
        assert!(node.height > 20.0);
```
to assert the note height equals `dag::DAG_CHANNEL_ROW_HEIGHT` (matching slider height) instead of `> 20.0`.

## 2. Left-align Note text

The Note paint branch centers text on the node's midpoint:

```4248:4254:mathematical/graph/port/directed/dag/rs/lib.rs
DagNodeKind::Note { text, .. } => {
    if lod.shows_detail_text() || lod.shows_controls() {
        let (x0, y0, x1, y1) = preview_content_bounds(node);
        let display = if text.is_empty() { "…" } else { text.as_str() };
        let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
    }
}
```

Change the anchor x to `x0 + DAG_PREVIEW_PAD` (left edge + padding, same padding used by preview tree rows at line 3910-3923), keeping y at `(y0 + y1) * 0.5`. `append_label` already renders left-to-right from the given point (confirmed by how preview tree rows compute a left `text_x` and pass it directly), so this alone left-aligns the text.

## 3. Dictionary preview: left-aligned, fixed-width, truncated

Per your decision, keep the uniform fixed-width column (`DAG_COMPONENT_WIDTH`) for visual consistency with every other node kind, and add truncation instead of letting text overflow. Preview tree rows are already collapsed-by-default (`expanded: BTreeSet::new()` — see [flow/core/rs/lib.rs:1176](flow/core/rs/lib.rs) and [mathematical/graph/port/directed/dag/rs/lib.rs:6373](mathematical/graph/port/directed/dag/rs/lib.rs)) and already render `▸`/`▾` chevrons that toggle per-path expansion:

```3903:3924:mathematical/graph/port/directed/dag/rs/lib.rs
DagPreviewContent::Tree { json } => {
    ...
    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
    let line = if row.has_children && !row.expanded {
        format!("{}: {}", row.label, row.summary)
    } ...
    append_label(scene, &line, text_pos, paint_px * 0.9, label_fill, label_halo);
}
```

Rows are already left-aligned (`text_x` is a left edge). The missing piece is truncation: add a small helper (near `preview_tree_collapsed_summary`, line 405) that truncates `line` to fit the available width (`x1 - text_x`, using `port_label_text_width` to measure) with a trailing ellipsis, and apply it before calling `append_label` for every preview tree row and for the Note text in section 2 (reuse the same helper so long note text also truncates instead of overflowing the box, matching the "fixed size + truncate" language for both).

## 4. Native double-click text editing for Notes

No DOM overlay. Caret and text are painted directly in the Vello scene, keystrokes are captured on the existing canvas container (`tabIndex=0` div at [flow/react/index.tsx:4290](flow/react/index.tsx)), mirroring how [writer/rs/lib.rs](writer/rs/lib.rs) (`WriterHost`) does native caret rendering/typing — but scoped down (single content string, no multi-line gutter/LSP).

### DagHost: transient edit state (shared engine layer)

Add a transient (non-persisted) field to `DagHost` alongside `widget_drag`/`pending_cluster_explode` ([mathematical/graph/port/directed/dag/rs/lib.rs:1726-1737](mathematical/graph/port/directed/dag/rs/lib.rs)):

```rust
struct NoteEditState { node_id: String, caret: usize, anchor: usize }
// on DagHost:
editing_note: Option<NoteEditState>,
caret_visible: bool,
```

Add `DagHost` methods (near `try_widget_pointer_down`/`toggle_preview_tree_path`, line ~3091-3160):
- `begin_note_edit(node_id, world_x, world_y)` — hit-tests a byte offset from `world_x` against the note's text (small helper mirroring writer's `hit_byte_in_line`, using the shared `cavas`/`infinite_cavas` text-measuring helpers already imported here, e.g. `port_label_text_width`), sets `editing_note = Some(NoteEditState { node_id, caret: offset, anchor: offset })`.
- `note_insert_text(chunk)`, `note_backspace()`, `note_delete_forward()`, `note_move_caret(dir, extend)` — mutate the target `DagNodeKind::Note.text` in place and update `caret`/`anchor` (byte offsets).
- `note_commit_edit()` — clears `editing_note`.
- `set_note_caret_visible(visible)` — sets `caret_visible` (toggled every frame from JS, like `WriterHost::set_caret_visible`).

Paint: extend the Note branch (line 4248) so when `self.editing_note` matches the node's id, it (a) left-aligns as in section 2, (b) truncates as in section 3, and (c) paints a thin caret bar (same technique as `WriterHost::render_caret_bar`, [writer/rs/lib.rs:1136-1146](writer/rs/lib.rs)) at the x computed from the caret byte offset via the shared byte→x text helper, only when `caret_visible` is true.

### FlowHost/FlowSession: widget sync + undo + wasm bindings

`FlowHost` wraps these with its own widget model and undo gestures, mirroring the existing slider-drag gesture pattern (`history.pending` set in `pointer_down_screen`, committed via `commit_gesture_history`, [flow/core/rs/lib.rs:2234-2236](flow/core/rs/lib.rs) and [flow/core/rs/lib.rs:3115-3129](flow/core/rs/lib.rs)):
- `begin_note_edit(widget_id, world_x, world_y)`: `self.history.pending = Some(self.fixture.clone())`, then `self.dag.begin_note_edit(...)`.
- `note_insert_text` / `note_backspace` / `note_delete_forward` / `note_move_caret`: delegate to `self.dag.<method>`, then `self.sync_from_dag()` (already has the `(Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => *text = dag_text.clone()` arm at [flow/core/rs/lib.rs:2629-2631](flow/core/rs/lib.rs)) and `self.touch_channel_eval()`.
- `note_commit_edit`: `self.dag.note_commit_edit()`, `self.commit_gesture_history()`.
- `set_note_caret_visible`: delegate to `self.dag.set_note_caret_visible`.

Expose all of these as `#[wasm_bindgen(js_name = ...)]` on `FlowSession` next to `setNoteText`/`setImageSrc` ([flow/core/rs/lib.rs:3466-3474](flow/core/rs/lib.rs)).

### React: FlowCanvas wiring, no overlay component

In [flow/react/index.tsx](flow/react/index.tsx):
- Add `const [editingNoteId, setEditingNoteId] = useState<string | null>(null)`.
- In `onCanvasDoubleClick` ([flow/react/index.tsx:4035-4058](flow/react/index.tsx)), before falling through to the spotlight: if the hovered widget's `kind === "inputNote"`, call `session.beginNoteEdit(hoveredId, world.x, world.y)`, `setEditingNoteId(hoveredId)`, `containerRef.current?.focus()`, `renderFrame()`, and `return`.
- In the container `onKeyDown` handler ([flow/react/index.tsx:3532-3590](flow/react/index.tsx)), branch at the top: if `editingNoteId` is set, route keys to the new note-edit bindings instead of undo/redo/delete-selection (Escape/Enter → `noteCommitEdit` + clear state; Backspace → `noteBackspace`; Delete → `noteDeleteForward`; Arrow keys/Home/End → `noteMoveCaret`; single printable characters with no meta/ctrl → `noteInsertText`), each followed by `evaluate()`/`persistFixture()`/`renderFrame()` like every other mutation path in this file; `event.preventDefault()` and return early so shortcuts don't also fire.
- Commit-on-click-away: in the pointer-down capture handler ([flow/react/index.tsx:4293](flow/react/index.tsx), `onContainerPointerDownCapture`), if `editingNoteId` is set and the click isn't on that same note, call `noteCommitEdit` + clear `editingNoteId` before continuing.
- Caret blink: in the existing continuous RAF loop ([flow/react/index.tsx:3655-3659](flow/react/index.tsx)), when `editingNoteId` is set, call `session.setNoteCaretVisible(Math.floor(performance.now() / 530) % 2 === 0)` before `renderFrame()`, mirroring [writer/react/index.tsx:464-472](writer/react/index.tsx).

No new DOM elements, no `FlowNoteOverlay` component — editing state and rendering stay entirely in the WASM/Vello scene, consistent with how writer works.

## Test updates (extend existing files only, per repo convention)

- [mathematical/graph/port/directed/dag/rs/lib.rs](mathematical/graph/port/directed/dag/rs/lib.rs) tests: adjust any note-size expectations to match `DAG_CHANNEL_ROW_HEIGHT`; add coverage for `begin_note_edit`/`note_insert_text`/`note_backspace`/caret offset math, and for preview/note text truncation.
- [flow/core/rs/lib.rs](flow/core/rs/lib.rs) `add_note_widget_with_text` (line ~4621-4636) and `set_note_text_keeps_uniform_component_width` (line ~4638+): update height assertions; add tests for the new `begin_note_edit`/`note_*_edit` wasm-facing methods and for undo grouping a whole edit gesture into one entry.
- [flow/react/index.tsx](flow/react/index.tsx) existing test suite: add coverage for double-click entering note-edit mode, keystrokes routing to the note-edit path instead of shortcuts, and commit-on-click-away/blur.

This work should happen inside a ticket per repo convention (`ticket_open`/`ticket_reopen`), associated with the appropriate goal from `repo://goals`.
