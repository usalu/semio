---
name: spatial repl history
overview: Turn `@spatial/js-renderer-r3f` into a full REPL by lifting the command palette + history controls out of `play/main.tsx` into the renderer package, and rebuild `DocumentHistory` + `CommandRuntime` around two diff‑based stacks where each modification stores the `CommandResponse` plus its backwards `TopologyDiff`. Readonly commands (empty diff) never enter the command stack; in‑command undo/redo walks per‑state snapshots, while out‑of‑command undo/redo applies the stored diffs.
todos:
 - id: history
   content: Rebuild DocumentHistory around Modification = { result, backwardsDiff } with undo/redo stacks; drop function-based recordCommand
   status: completed
 - id: runtime
   content: Add snapRedoStack + redo() to CommandRuntime; route undo/redo to in-command vs DocumentHistory based on active state; clear in-command stacks on commit/cancel; update canUndo/canRedo capabilities
   status: completed
 - id: commit
   content: "Replace recordCommand call in commit() with history.record({ result, backwardsDiff: inverse }); skip when diff empty"
   status: completed
 - id: repl
   content: Add CommandRepl + useDocumentHistory + useReplHistoryState in renderer-r3f/index.tsx; lift palette/input/shortcuts/history bar from play/main.tsx
   status: completed
 - id: play
   content: Shrink play/main.tsx to geometry+preset selection plus <CommandRepl> consumption
   status: completed
 - id: tests
   content: Extend core + renderer-r3f vitest suites for two-stack history, readonly skip, in-command redo, active/inactive routing, useReplHistoryState
   status: completed
isProject: false
---

# Spatial REPL with Two‑Stack Modification History

## 1. Modification model — [spatial/js/core/index.ts](spatial/js/core/index.ts)

Add a single record type the history persists:

```ts
interface Modification {
 readonly id: string;
 readonly commandId: string;
 readonly label: string;
 readonly result: CommandResponse; // includes forward diff + data
 readonly backwardsDiff: TopologyDiff; // inverse computed at commit time
}
```

Replace the function-based `DocumentCommand` / `DocumentHistory` (current `cmdStack` of `{do, undo}`) with a pure-diff history:

- `undoStack: Modification[]`, `redoStack: Modification[]`
- `record(mod)` — push to undo, clear redo. Skipped when `isEmptyTopologyDiff(mod.result.diff)` (readonly commands like `measure.distance/area/volume` never enter the stack).
- `undo(doc)` — pop undo, `applyTopologyDiff(doc.topology, mod.backwardsDiff)`, push popped onto redo. Returns `Modification | null`.
- `redo(doc)` — pop redo, `applyTopologyDiff(doc.topology, mod.result.diff)`, push popped onto undo.
- `peekUndo()` / `peekRedo()` for UI labels.

Drop `pushSnapshot` and the function-based `recordCommand` API entirely (no compat layer per repo rules).

## 2. CommandRuntime in‑command stacks — [spatial/js/core/index.ts](spatial/js/core/index.ts)

Currently there is one undo stack inside the runtime (`snapStack`) and `canRedo` is hard-wired to `false`. Add the symmetric redo:

```ts
private readonly snapUndoStack: { state: string; context: string }[] = [];
private readonly snapRedoStack: { state: string; context: string }[] = [];
```

- On `send(event)` that fires a non-transient transition not in `excludeEvents`: push the _before_ snapshot to `snapUndoStack` and **clear `snapRedoStack`** (new branch invalidates redo).
- `undo()` — pop `snapUndoStack`, push current onto `snapRedoStack`, restore.
- `redo()` — pop `snapRedoStack`, push current onto `snapUndoStack`, restore.
- `cancel()` clears both.
- After `commit()` succeeds, both in-command stacks are cleared (the active session is done).

Define an `inActiveCommand` helper: `state !== machine.initial && state !== "committed"`. When the runtime is **not** in an active command, the `undo()` / `redo()` methods route to `DocumentHistory.undo/redo(doc)` instead of touching the in-command stacks. When **active**, they touch only the in-command stacks.

Update `canUndo` / `canRedo` in `CommandSnapshot.capabilities` accordingly:

- Active: based on `snapUndoStack.length` / `snapRedoStack.length`.
- Inactive: based on `history.peekUndo() !== null` / `peekRedo() !== null`.

## 3. Commit records a Modification

In `CommandRuntime.commit`, after the existing `const inverse = applyTopologyDiff(topo, diff);` line replace the current `hist.recordCommand({ id, label, do, undo })` call with:

```ts
if (hist && !isEmptyTopologyDiff(diff)) {
 hist.record({
  id: `cmd-${this.spec.id}-${this.revision}`,
  commandId: this.spec.id,
  label: this.spec.label ?? this.spec.id,
  result: res, // built below, includes diff + data
  backwardsDiff: inverse,
 });
}
```

Readonly measure commands (`measure.distance/area/volume`) keep returning a `CommandResponse` with `EMPTY_TOPOLOGY_DIFF`, so `record` short-circuits — they simply do not enter the stack.

## 4. Renderer REPL — [spatial/js/renderer-r3f/index.tsx](spatial/js/renderer-r3f/index.tsx)

Lift the REPL UI out of `play/main.tsx` into the renderer package under a new `🪩Repl` region so the renderer is the full REPL surface. Public API:

```ts
export function useDocumentHistory(): DocumentHistory; // memoized per host
export function useReplHistoryState(rt, history): { canUndo; canRedo; undoLabel; redoLabel };
export interface CommandReplProps {
 /* presets, geometry, hooks */
}
export function CommandRepl(props: CommandReplProps): ReactNode; // canvas + palette + history controls
```

`CommandRepl` composes the existing `CommandCanvas` + `CommandSpatialView` with:

- The palette / `cmdLine` input (copied from `play/main.tsx` `PlaySession`, including `paletteRows`, `tryParseValueCommand`, `buildDispatchEvent`, value-style commands, capture-phase shortcut handler).
- A history bar showing `Undo: <label>` / `Redo: <label>` with disabled state from `useReplHistoryState`. Bound to `Ctrl+Z` / `Ctrl+Shift+Z` plus the existing `r` shortcut. The single `undo`/`redo` button calls `rt.undo()` / `rt.redo()` which routes to in-command vs document history based on active state.
- A "Last response" panel that prints `snapshot.lastResponse` (already on snapshot) and the diagnostics list.

`play/main.tsx` shrinks to: pick geometry asset + preset list, build `CommandRuntime` with a `DocumentHistory`, render `<CommandRepl>`.

## 5. Tests — extend the existing vitest suites

In `spatial/js/core/index.ts` `🧪Tests` region:

- `DocumentHistory` round-trip: record two modifications, `undo`/`redo` restores topology counts and revision via `applyTopologyDiff`.
- Readonly skip: `measure.distance` commit produces empty diff and `peekUndo()` stays `null`.
- In-command redo: `send` → `undo` → `redo` returns to the post-send snapshot; new `send` after `undo` clears `snapRedoStack`.
- Active vs inactive routing: while `state === "first_corner"`, `rt.undo()` walks `snapUndoStack`; when `state === machine.initial` (or after `commit`) it pops `DocumentHistory`.

In `spatial/js/renderer-r3f/index.tsx` `🧪Tests` region: smoke test that `useReplHistoryState` reports `canRedo=true` after an undo on a runtime with two committed modifications (using a stub kernel + bypassing canvas rendering, like the existing runtime test).

## 6. Out of scope

- Persisting the modification history across sessions.
- Cross-document branching / named history snapshots.
- Touching `coda` / `elements` / `compose` (per `AGENTS.md` no-tech-mixing rule).
