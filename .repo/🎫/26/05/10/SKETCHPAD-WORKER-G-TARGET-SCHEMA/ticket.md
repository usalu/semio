# Sketchpad Worker G — Target schema surface

**Worker:** G (sketchpad `index.tsx` only)  
**Repo MCP:** unavailable from tooling; tracked manually per prior tickets.

## Done

- Renamed local XState batch machinery: `createKeyedChangeHandlers` / `createSingleKeyChangeHandlers`, `AppChangeState`, context key `change`, events `*.CHANGE.START_NEW_CHANGE` / `SAVE_CHANGE` / `DISCARD_CHANGE` / `UNDO` / `REDO` / `RUN_OPERATION`.
- Command strings: `semio.{design,kit,quality}App.{startNewChange,saveChange,discardChange}`; hooks `useKitAppChange`, `useTypeAppChange`, `useDesignAppChange`, `DesignAppChangeProvider`.
- Fixed missing `namespace: "TYPE"` on type-app keyed handler registration.
- Footer: unsaved count from `vcsState()` + legacy `openTransaction`/`draft`; optional `login`/`logout` on `kitClient` with `[DEBUG]` warnings when absent.

## Files

- `semio/sketchpad/index.tsx`
