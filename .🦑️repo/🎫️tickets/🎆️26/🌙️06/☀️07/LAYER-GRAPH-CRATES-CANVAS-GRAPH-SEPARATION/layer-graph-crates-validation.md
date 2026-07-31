# Layer Graph Crates Validation

## Cargo (2026-06-07)

```
cargo test -p infinite_cavas --lib          → 3 passed
cargo test -p mathematical_graph --lib      → 3 passed
cargo test -p mathematical_graph_port --lib → (alias tests via directed)
cargo test -p mathematical_graph_port_directed --lib → board_engine_alias + layouts
cargo test -p mathematical_graph_port_directed_normal --lib → board_host tests
cargo test -p puzzle_2d --lib               → 108 passed
cargo test -p mathematical_graph_port_directed_dag --lib → 4 passed
```

## Architecture

- `infinite_cavas`: canvas only (camera, lod, text, raster, gpu, icons, geom_sel)
- `mathematical_graph`: generic GraphEngine + geometry + generic NodeDescJson
- `mathematical_graph_port`: handles/ports layer
- `mathematical_graph_port_directed`: directed board base (scene descriptors, types, layouts)
- `mathematical_graph_port_directed_normal`: BoardHost + puzzle.2d.fixture
- `puzzle_2d`: depends only on `mathematical_graph_port_directed_normal`
