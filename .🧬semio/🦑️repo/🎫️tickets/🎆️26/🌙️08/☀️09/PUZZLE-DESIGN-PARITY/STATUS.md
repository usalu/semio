# STATUS — Puzzle Design Parity

## E2E runtime — DONE
- `jsonschema` WASM: `default-features = false` in framework schema crate
- Puzzle wasip2 IO register bodies cfg-gated off host-only `semio_framework_os`
- Flatten module wired in puzzle `glue.rs`
- Dev servers: 6012 (2d), 6013 (3d), 6014 (5d) serve `semio_s_plugin_puzzle.js` as `text/javascript`
- Fixture/DSL/catalog/kit/SPR fixes from design-parity waves landed

## Final sync bug — FIXED
- Sparse fixture node writes (missing `anchor` / connection defaults) made
  `puzzle2d_document_delta_operations` fall back to `SetSnapshot`, which clobbered
  concurrent peer edits over the backbone (LWW).
- Fix: canonicalize nodes/edges through typed `Puzzle2dNode`/`Puzzle2dEdge` before
  diffing; `add_node_to_fixture` writes `"anchor": "fixed"`.
- Regression: `sparse_node_without_anchor_still_emits_set_node`
- `cargo test -p semio-s-plugin-puzzle --lib` → **414 passed / 0 failed**

## Apps
- `bun run dev:puzzle:2d` → :6012
- `bun run dev:puzzle:3d` → :6013
- `bun run dev:puzzle:5d` → :6014
