# Wave Z — browser checklist verification (2026-09-10)

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Serve: `http://127.0.0.1:6014` (wasm #31, not killed).
Harness: `🔍️browser-probe.ts` (additive flags `--undo --clipboard --marquee --importexport --locked --brush --gumball --suggestions`).
Fill owned by another wave. Item 9 closed by coordinator 07:40 (presence accent, not a fault).

No guest rebuild. No host code fix landed this wave — defects below are browser-proven with a precise handoff.

## Verdict table

| # | Item | Verdict | Artifacts | Evidence |
| --- | --- | --- | --- | --- |
| 1 | Example-switch undo is ONE history step | **DEFECT-REPORTED** | `probe-2026-09-10T05-51-29.md` + `-history-open.png` + `-undo-once.png` | After a clean Forest→Nakagin switch (no prefix picks), History COMMANDS has **exactly two rows**: `Resize Window` and `delete-object id=seed-left-001`. The document edit is one row (not 150), but it is **not** labeled Set Active Example. `#framework.history.undo.run` is **absent** from the DOM. Dispatch-clicking `#framework.history.undo` + entry.2 + Meta+Z + Ctrl+Z + Meta+Z leaves the navbar on **Nakagin Capsule Tower** and the two rows unchanged. Native W-X law `set_active_example_lands_as_one_edit_and_republishes_the_world_scene` is green; the browser undo path is inert. |
| 2 | Clipboard copy/cut/paste | **DEFECT-REPORTED** | `probe-2026-09-10T05-45-07.md` + `-clipboard-copy-paste.png` | Forest boot. Cmd/Ctrl+C then Cmd/Ctrl+V surfaces `framework.window.puzzle3dMainPerspective.action.paste.execute` (paste **form**, not an apply). Body stays **`1 Objects · 0 Attractions`**. History gains no paste/copy row. Zero FAULT_RE hits. W-Y laws `copy_then_paste_clones_selection_as_one_mutation` / `cut_undoes_as_one_step` are native-only. |
| 3 | Marquee rectangle + click pick | **PARTIAL** | `probe-2026-09-10T05-45-07.md` + `-marquee-drag.png` + `-marquee-click.png` | Drag on the perspective canvas and a later click complete with **zero faults**. Chrome does not expose a selection-id dump; object count stays 1; Inspection was not proven to update. Click-to-pick is the path other waves already used successfully on #31. Marquee **coverage** is unverified (Command category overlay often covered the canvas on the Nakagin combined run). |
| 4 | Import/export round trip | **DEFECT-REPORTED** | `probe-2026-09-10T05-52-53.md` + `-suggestions-open.png`; earlier `05-35-23-suggestions-open.png` | **Affordance found:** workspace context menu rows `4 Export` and `5 Import` (also Duplicate mod+d). Locator `/^export$/i` misses the `4 Export` label so the download never fired. No file written under `🗑️generated/`. W-Y commands `exportFixture` / `openImportFixture` / `importFixture` exist in the guest catalog; the live chrome path is the context menu, not Artifact/Catalogue. Round trip not completed. |
| 5 | Locked-volume gumball refusal | **DEFECT-REPORTED** | `probe-2026-09-10T05-45-07.md` + `-locked-refusal.png` | Inspection tab + `[id*='locked']` scan returned **0 controls**. No refusal notice (only the unrelated `Agent disconnected` toast). Gumball drag on an unlocked Forest slab also produced no notice. Cannot prove W-Y `gumball_scale_on_locked_volume_refuses_without_edit` in the browser until a lock toggle is reachable (Artifact row-action lock or inspector `object.locked`). |
| 6 | Brush tool strokes | **DEFECT-REPORTED** | `probe-2026-09-10T05-45-07.md` + `-brush-stroke.png`; Nakagin `05-35-23-brush-stroke.png` | `brush buttons=0` after clicking Utilities. No Brush utility chip in the DOM. A canvas drag still completed with **zero faults** (no place/remove proof). On the Nakagin combined run the Command category list covered the viewport. |
| 7 | Gumball drag → one history entry | **DEFECT-REPORTED** | `probe-2026-09-10T05-45-07.md` + `-gumball-drag.png` | After select + axis-ish drag, History still shows the same two rows (or the Command ACTIONS tree, not a `translateSelection` entry). No piece-move proof. Transform utility was never confirmed active (`gumball_active` is utility-gated). |
| 8 | Suggestions window opens/populates | **DEFECT-REPORTED** | `probe-2026-09-10T05-52-53.md` + `-suggestions-open.png` | Right-click / Alt+right-click opens the **Workspace Menu** (Set Active Example, Add Object, Duplicate, Export, Import, History, Hand, Selection, More, Delete). **No** vortex-suggestions popup, **no** candidate rows, **no** `openVortexSuggestions` control. Native `open_vortex_suggestions_opens_the_suggestion_popup` is green. |
| 9 | Perspective tab red + warning triangle | **CLOSED** | coordinator 07:40 in `📓️2026-09-10-cursor-coordination.md`; prior `probe-2026-09-10T05-19-26-fill-wait-ready.png` | Not a fault. No `destructive` class; icon is the perspective glyph; red is the local-user focus/presence accent. Skipped per instruction. |

## Probe runs

| Stamp | Flags | Notes |
| --- | --- | --- |
| `05-31-06` | `--interact --undo --settle=30` | First undo attempt. History collapsed. `button:Undo` click timeout. Nakagin rendered. 0 faults. |
| `05-35-23` | all checklist flags + Nakagin switch | Combined. History expanded: 4 rows (prefix pollution + `delete-object`). Undo click timeout. Paste form appeared. Context menu showed Export/Import. Command overlay blocked later canvas steps. 0 faults. |
| `05-43-23` | `--undo` only, prefix skipped | Clean switch → 2 history rows. `#framework.history.undo.run` null. Click timeout on `#framework.history.undo`. |
| `05-45-07` | Forest: clipboard/marquee/importexport/locked/brush/gumball/suggestions | Paste form + still 1 object. Marquee no-fault. Export click 0. Lock ids []. Brush buttons 0. Gumball no history. Suggestions = workspace menu. |
| `05-47-52` | `--undo` (pre-dom-click patch, failed to apply) | Same as 05-43-23. |
| `05-49-54` | `--suggestions --importexport --clipboard` | Menu confirmed again. |
| `05-51-29` | `--undo` with DOM click + Meta/Ctrl+Z | **Undo inert.** Example stays Nakagin. |
| `05-52-53` | `--suggestions` | Workspace menu; `context export=0` because label is `4 Export`. |

## Root causes / handoff

### 1. History undo does not dispatch in the live shell

Guest `ui_history_panel` builds `framework.history.undo` as a **tree item whose child button** is `framework.history.undo.run` (`plugin` `action_item`). The live DOM has the tree item and **no** `.run` node. Synthetic clicks on the item/button and `mod+z` do not move history. Native reserved `FrameworkUndoJob` still works in-process.

**Handoff:** host Tree/Interpreter must either (a) render the child activate button with id `framework.history.undo.run`, or (b) treat a tree-item click as the child's `Trigger::Activate`. Also confirm `mod+z` keybinding reaches `handle_action("undo")` when a world canvas has focus. After that, re-probe `--undo`: one undo of `delete-object id=seed-left-001` must restore Concrete Forest. The extra `Resize Window` row is a second issue (view command leaking onto the document stack during `setActiveExample`).

### 2. Clipboard paste is a command form, not an apply

`Cmd+V` opens the perspective window's paste **Execute/Reset** chrome instead of applying `Puzzle3dClipboardJob` with a fragment. Object census stays 1. **Handoff:** host reserved clipboard must attach `args.fragment` on paste so the guest job can clone. Clicking Execute with an empty fragment is a no-op.

### 3. Import/Export live in the workspace context menu

Not in Artifact/Catalogue. Rows are numbered (`4 Export`, `5 Import`). A follow-up probe must click `text=/export/i` (not `^export$`), wait for `download`, save JSON, then click Import and feed the file chooser.

### 4. Suggestions / Brush / Gumball / Lock are chrome-activation gaps

- Suggestions: context menu is the **shell workspace menu**, not `openVortexSuggestions`. Need the plugin row (Alt+right-click on a **vortex marker**, or a Command-list `openVortexSuggestions` if it is published).
- Brush: Utilities unfold did not expose a Brush chip. Need the perspective window utility bar (`Transform / Brush / Volume Brush / Relocate`).
- Gumball: Transform utility + object selection required (`gumball_active`).
- Lock: inspector `puzzle3d-play-inspector.object.locked` / Artifact row-action `setSelectionFlag` never appeared because Inspection body did not show an object group after the pick.

## Files changed

- `🔍️browser-probe.ts` — clipboard permission grant; `--undo` (History expand, DOM undo, Meta/Ctrl+Z); `--clipboard`; `--marquee`; `--importexport`; `--locked`; `--brush`; `--gumball`; `--suggestions`; skip prefix/example-switch when those flags are set (bare `--interact` unchanged).
- this report

## Laws / suites

No new law this wave (browser-only disposition). Existing native laws cited above remain the fail-before/pass-after for W-X / W-Y; they do **not** cover the host undo/clipboard/menu gaps. No cargo/vitest/tsc run this wave (no production TS/Rust edits).

## Open items

1. Host: history Undo activate button + `mod+z` while canvas focused.
2. Host: paste fragment wiring (or auto-apply without the Execute form).
3. Probe follow-up: click `4 Export` / `5 Import` by substring; filechooser round trip.
4. Probe follow-up: unfold perspective Utilities → Brush / Transform; Inspection lock toggle; vortex-marker Alt+right-click.
5. Guest label: coalesced example switch should read Set Active Example, not `delete-object id=seed-left-001`.
