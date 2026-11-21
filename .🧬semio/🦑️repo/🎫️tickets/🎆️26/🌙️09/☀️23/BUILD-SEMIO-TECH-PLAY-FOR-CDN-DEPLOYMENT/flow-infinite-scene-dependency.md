# Flow Infinite Scene Dependency

The Flow browser WASM job depends on `semio-os-infinite`, whose manifest has an unconditional dependency on `semio-os-infinite-world-bridge`. That bridge crate mounts `🌍️world/🌉️bridge/🎞️scene-bridge/🦀️.rs`, the file whose changing digest restarted the live job twice. Browser wasm32 is not WASI p2, so the Infinite facade's world cfg is enabled. The broad Cargo dependency itself is unconditional.

Flow's production references observed in this review use board DAG ports, input and output codecs, board schema and layout, and DAG graph/host functions. No Flow production reference to the world scene bridge was found. The `flow` facade explicitly reexports the entire `semio_os_infinite` crate, so downstream API uses must be audited before changing that facade. Tests also use its board paths. This is evidence of broad wiring, not proof that replacing a single edge is sufficient.

Relative to `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow`, manifests directly naming `semio-os-infinite` are:

- `📦️packages/🦀️rust/Cargo.toml`
- `🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🖥️host/🧱️flow-host-model/📦️packages/🦀️rust/Cargo.toml`
- `🖥️host/⚖️laws/📦️packages/🦀️rust/Cargo.toml`
- `🕸️wasm/📦️packages/🦀️rust/Cargo.toml`
- `🕸️wasm/🧩️domain-adapter/📦️packages/🦀️rust/Cargo.toml`
- `🕸️wasm/🎬️actions-a/📦️packages/🦀️rust/Cargo.toml`
- `🕸️wasm/🎞️actions-b/📦️packages/🦀️rust/Cargo.toml`

Existing narrow providers include `semio-os-infinite-board-schema` for DAG input, layout and palette codecs and types; `semio-os-infinite-board-dag-model`, `semio-os-infinite-board-dag-graph` and `semio-os-infinite-board-dag-host` for the directed DAG facade exports; and `semio-os-infinite-board-engine` and graph/canvas producers for other directed exports. In particular, `dag_draw_lod_label` and `dag_pick_targets_at_screen_json` come from DAG graph, while `dag_set_canvas_palette` comes from DAG host. The current directed DAG namespace combines multiple producers, so imports and public types need a coordinated mapping; browser-specific `DagSession` and test cfgs must be preserved if used.

Any manifest changes require reviewing every lock containing the changed packages, not only deleting a world bridge entry: `🧰️framework/Cargo.lock`, `✏️s/Cargo.lock`, `🌎️hub/Cargo.lock`, and independent consumer workspaces including Flow bridge, Flow extension and artifact locks, dev Flow and downstream plugin locks. World bridge may remain for other consumers even after Flow is narrowed. Exact lock updates depend on the final dependency graph; no graph rebuild or compile was run during this review.
