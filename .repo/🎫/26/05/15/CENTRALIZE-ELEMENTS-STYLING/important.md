## Summary

- Added `elements/client/lib/styling` as `@elements/styling`: `palette.css` (fonts + `@theme` scale), `elements.css` (Tailwind entry + Golden Layout chrome + tokens + utilities split from former `globals.css`), `tailwind.config.ts`, and `index.ts`.
- `@elements/ui` now composes `@import "../styling/elements.css"` + `globals-ui.css` (docs, tree, fullscreen, prose asides).
- `elements/board` no longer depends on `@elements/ui`; play uses a board-local `BoardPlayWorkspace` shell with the same token classes (`bg-window`, `border-element`, 20px tab strip) and imports `../../styling/elements.css` (relative so Vite resolves without extra aliases).
- Compose tailwind presets now import the shared preset from `elements/client/lib/styling/tailwind.config.ts` (replacing the broken `../react/ui` sketchpad import).

## Validation

- `nx run @elements/board:test` — pass.
- `nx run @elements/board:build` — pass.

## Note

- Root `bun install` failed here (`@compose/sketchpad` 404 from registry); relative CSS paths avoid requiring a linked `@elements/styling` package for Vite. Workspace `package.json` still declares `@elements/styling` for explicit dependency graph.
