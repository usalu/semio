# Exact WGPU hover browser proof

The existing WGPU structure dump now includes the typed NodeGraph hover for scene nodes that have one. The field preserves the node, side-specific port, companion port IDs and the source window. The structure dump is collected on request. The independent mesh dump already reports the current hovered instance ID.

The browser journey selects Box Shell Preview using its mounted picker row. It uses the WGPU graph’s actual geometry census to locate the output port and checks the corresponding preview instance. It clears the graph hover, then moves a physical pointer over the preview until the same instance is picked and the graph reports the exact port. It checks that both surfaces clear on leave. The journey saves its evidence and returns an error when a direction fails.

The shared fixture passes six assertions with Zod and reproduces the missing field against the previous native projection. Current Rust serialization matches all fixture rows and omits the field on cleared or non-graph nodes. The retained-tree test is registered and awaits a full managed native build. The canonical WGPU browser journey awaits the real Wasm build.

- [Native red witness](./🗑️generated/wgpu-graph-hover-red.log)
- [Native green witness](./🗑️generated/wgpu-graph-hover-green.log)
- [Zod wire oracle](./🗑️generated/wgpu-exact-graph-hover-oracle.log)
