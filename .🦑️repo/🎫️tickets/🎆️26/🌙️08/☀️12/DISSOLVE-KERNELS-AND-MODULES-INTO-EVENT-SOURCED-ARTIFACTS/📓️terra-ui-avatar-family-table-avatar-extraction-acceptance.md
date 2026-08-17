# Terra UI Avatar Family Table Avatar Extraction Acceptance

## Completed Source Closure

- Moved the sole qualifying `TableAvatar` contract and implementation to `🧱️elements/📻️TableAvatar/🟦️component.tsx`.
- Kept Radix root/image/fallback helpers file-private; the new module exports only `TableAvatar` and `TableAvatarProps`.
- Deleted the broad Avatar component and story, including `DraggableAvatar` and `DraggableAvatarProps`.
- Repointed the two direct production consumers: `HistoryTable` and `VirtualFileSystem`.
- Replaced the DragAndDrop story's removed draggable avatar with local decorative initials markup while retaining its reorder and `DragHandle` demonstration.
- Coordinator registered only `TableAvatar` and `TableAvatarProps` in the shared React barrel.

## Registrar Contract

- Final shared React barrel SHA-256: `fdd7e8ec24ea5288b386bab04f2627d81194712e2461860e8e2abcead71a4a23`.
- The registrar exposes `TableAvatar` and `TableAvatarProps` from `🧱️elements/📻️TableAvatar/🟦️component.tsx`.

## Source Validation

- The stale-path scan across framework UI and UI Storybook sources found no `👤️Avatar` component/story paths, `DraggableAvatar`, or `DraggableAvatarProps` references.
- The escaped-helper scan found no `AvatarPrimitive`, `AvatarImage`, or `AvatarFallback` references outside the new TableAvatar implementation.
- New-path scan confirmed imports in `HistoryTable` and `VirtualFileSystem`, the TableAvatar story, and the React barrel registrar.
- `git diff --check` on all tracked leased paths emitted no whitespace errors. `git diff --no-index --check` on both new files emitted no whitespace errors; its exit status `1` denotes an intentional file addition.

## Final SHA-256 Evidence

| Artifact | SHA-256 |
| --- | --- |
| `🧱️elements/📻️TableAvatar/🟦️component.tsx` | `0634969fa873976f81ba4d14e56d56c9bf3e17f828a6f3e3629f2fd5405925bc` |
| `🧱️elements/📻️TableAvatar/🧪️story.tsx` | `9b835a3f29ec5bb28e0f227906763c1d7c2bd2766eb2eda4908faa4c11e117b9` |
| `🧱️elements/📜️HistoryTable/🟦️component.tsx` | `e255e22a2a2ed5c4c7a0b14e641ffaf1a1333d3585a1ef90b27fa68461cba634` |
| `🧱️elements/📁️VirtualFileSystem/🟦️component.tsx` | `3c1ce5cfc96b49967d1f9a1050fea59f0d91385e7198bc7b9b1857aabd9c7540` |
| `.storybook/stories/ui/🖐️DragAndDrop.stories.tsx` | `4f3840c1ccaa05788f8db23da3ab7cc1a5cc7f18876ea41f5cdf8e1c5b04ab8b` |

## UI React Checks

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:lint` | Passed. Bun printed the ambient `NO_COLOR`/`FORCE_COLOR` warning only. |
| `bun nx run @semio-tech/ui-react:typecheck` | Failed with emitted cross-package and UI errors, including unresolved framework types in `🟦️glue.ts`, conflicts in the shared React barrel, and errors in other UI elements. No repair was made. |
| `bun nx run @semio-tech/ui-react:test-quick` | Failed: 510 passed, 10 failed, and 2 unhandled errors. Emitted failures include Scene, icon styling, shell, tree, and VirtualFileSystem paths. No repair was made. |
| `bun nx run @semio-tech/ui-react:build` | Failed because Storybook could not resolve `@semio-tech/coda-desktop/renderer` from `.storybook/stories/ui/✅ValidationTree.stories.tsx`. No repair was made. |
