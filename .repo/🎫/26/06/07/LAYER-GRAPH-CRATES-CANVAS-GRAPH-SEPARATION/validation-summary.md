# Validation summary

## Cargo tests
- `mathematical_graph_port_directed_dag`: 4 passed
- `puzzle_2d`: 108 passed (unchanged)
- `infinite_cavas`: 3 passed
- `mathematical_graph`: 3 passed

## Vitest
- `@dag/play`: 2 passed
- `@dag/react`: 2 passed

## Runtime (Playwright)
- `bun nx run @dag/play:validate` on http://127.0.0.1:6017/
- `[DEBUG] dag play surface mount`
- `[DEBUG] dag canvas loaded fixture`
- Single `<canvas>` rendered, no Unsupported UiNode

## Dev
- `bun run dev:dag` → port 6017
- Launch: `🛠️dev🌳dag` in `.vscode/launch.json`
