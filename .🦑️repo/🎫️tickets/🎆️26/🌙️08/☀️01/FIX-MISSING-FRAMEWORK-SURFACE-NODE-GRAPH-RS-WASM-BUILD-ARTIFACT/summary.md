# Fix missing framework-surface-node-graph-rs WASM build artifact blocking Storybook

## Summary
- Identified crate `@semio-tech/framework-surface-node-graph-rs` at `🧰️framework/🔨️module/🗺️surface/🕸️node-graph/⚡️implementation/🦀️rust`.
- Executed WASM build target (`bun ./📜️script.ts wasm`) for `@semio-tech/framework-surface-node-graph-rs`.
- Confirmed generation of `pkg/framework_surface_node_graph.js`, `pkg/framework_surface_node_graph_bg.wasm`, and related type definitions.
- Ran `bun nx run @semio-tech/ui-react:build` and verified that the Rollup import error for `@semio-tech/framework-surface-node-graph-rs/pkg/framework_surface_node_graph.js` is completely resolved.
