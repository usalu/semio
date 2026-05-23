# Worker F — react target-schema hooks

## Done

- `semio/js`: `CommandBuilder` + session/version/unsaved-change/kit-operation/alternative navigators; restored `KitWriteScope` triple fields on `KitStore` alongside `kitWriteChangeId` for mixed migration state.
- `semio/react`: `usePendingTriad` (replaces `useDraft`), `useEdit*` hooks + schema maps (`Edit`), change-lifecycle hooks (`useChange`, `useCommandBuilder`, `useStartNewChange`, `useSaveChange`, `useUnsavedChanges` stub, auth/alternative stubs), re-exports matching JS command types.

## Verification

- `npx tsc --noEmit` in `semio/js` and `semio/react`: pass.
- `npx vitest run --config vite.config.ts index.tsx` (embedded tests): 16 passed (skipped full `npm test` wasm prebuild because `semio/rs` wasm build fails in parallel refactor).
