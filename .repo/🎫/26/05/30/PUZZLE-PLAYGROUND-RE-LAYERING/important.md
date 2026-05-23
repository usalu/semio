# Root cause

`puzzle/5d/play/index.ts` (and siblings) had DOM auto-boot blocks that ran whenever the module loaded—not only as the Vite entry. The renderer imports `@puzzle/5d/play` for the 5D host, so 2D/3D bundles executed `boot5dPlay` on load.

Vite aliases also mapped all `@framework/playground/renderer/react/puzzle/*` subpaths to the full `index.tsx`, so every playground pulled every dimension’s hosts.

# Fix

- Browser boot stays in each play `index.ts`, gated by `import.meta.env.PUZZLE_PLAY_ENTRY` (`2d` / `3d` / `5d`) set in that play's `vite.config.ts` via `playEntryKind`.
- `stripPlaygroundRendererForPuzzleKind` + extended shell entry plugin slice renderer per `puzzle/2d|3d|5d` import.
- Removed puzzle subpath aliases from `createPlaygroundPlayViteConfig` so the plugin handles resolution.
