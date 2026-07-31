# React/js Hoist Verify Log

## Rule refinement

Bundle directory name is either:

- **Language tag** (`js`, `rs`, …) — bundle root IS the tag; sources live at root
- **Framework tag** (`react`, `r3f`, `react-renderer`) — framework implies implementation; no nested `js/`
- **Role tag** (`core`, `engine`, `runtime`, …) — sources live under `<lang>/` (e.g. `core/js/`)

## Changes

- Hoisted 31 `*/react/js/` dirs back to `*/react/`
- Hoisted `r3f/js/` and `react-renderer/js/` (same framework rule)
- Restored `./js/` exports for 52 role bundles accidentally patched by global replace
- Fixed vite alias paths in `ui/styling/vite-elements-assets.ts` and playground dev vite config
- Updated `migrate-language-tags.ts` with `FRAMEWORK_TAGS` skip set

## Tests

- `@semio-tech/draw-core:test` — 4/4 pass
- `@semio-tech/draw-react:test` — 2/2 pass
- `@semio-tech/note-react:test` — 2/2 pass
