# Draft reconciliation, first edit and grid extent

## Root causes (all in the renderer input host, TS only)
1. `🧱️elements/🗣️Interpreter/🎯️commit-lane/🟦️.ts` `complete()`: after a native receipt it required the republished text to equal the typed text (`Input publication changed the requested value`). TRUE->true, 007->7 failed this, so the draft kept `data-draft-dirty`. Removed; the receipt revision match is the authority.
2. `✍️input-views/🟦️.tsx` `useCommitDraft.acknowledge`: acknowledged only when `pending.value === published`. Now a matching receipt revision settles the draft and shows the model's canonical text (draft reset to published when it still equals the committed raw text).
3. No-op commits (clearing a vacancy, unchanged value, `Emit::default`) return `applied` without a commit receipt. The lane threw `Input did not return a native commit receipt`, flagging the draft as conflicted. Now an applied outcome without receipt settles when target and revision are unchanged, and the draft returns to published.
4. Moved node id (cause of the "first edit after open fails" and E5 clear): the grid re-renders and a cell's UiNodeId changes (observed 33 -> other) while its `draftTarget` stays stable. `read()`/`send()` read the stale id, returned null, error `Input target changed`, `conflicted=true`, so every later commit (including the clear) was silently dropped. `resolve()` now re-finds the node by `draftTarget` (new `UiControlPort.find`, `UiDocumentStore.findNode`).

## Grid extent
`✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` `grid_extent` is now the whole worksheet (1,048,576 x 16,384), windowed; rows/columns beyond the used range are reachable vacancies, so C6 downward is editable. Fixture `🧪️tests/🧫️fixtures/📊️sheet-grid/🔣️.json` gained `expectedEditorRows/Columns`. Needs a wasm rebuild; unverified in browser until then.

## Tests
- TS (vitest, `claude-shell-leak-vitest.sh editable-controls`): 128 passed, exit 0; new: canonical reconcile TRUE/007/"" x stable/moved node, no-op commit.
- Browser probe `e2e-clear-probe4.ts` against dev server (TS hot reload, old wasm): temporary, undo/redo, TRUE->true, 007->7, 0.1, clear E5 all `dirty=false`, no notices.
- Rust grid fixture test `neutral_sparse...` (see result below).
