# WGPU Flow Directional Targets

The WGPU Flow canvas publishes hover and selected handles using the side-specific port IDs supplied by the accepted NodeGraphScene. The renderer resolves each raw channel by widget, authored channel and direction. External scene hover and selected handles resolve in reverse through the same records, including widget IDs containing an at sign. The physical Flow snapshot and wire data still use authored channel names.

The native selection snapshot now includes nodes, edges and handles. The bounded action path counts all three kinds and checks their target byte lengths. An active handle or edge selection stays on the screen pointer path so its next hover does not run through a node-only bounded plan. The renderer keeps the accepted scene port addresses in its sync cache and retires them in bounded string pages.

## Verification

- The changed Rust files and the registered WGPU regression parsed with the pinned Rust formatter. The EngineCanvas diff passed Git's whitespace check.
- The focused Rust harness passed direction-sensitive forward and reverse mapping, native selected-handle projection, string and node credit refusal, and port-cache retirement. It used the live mapping and credit functions with a minimal host fixture. The generated harness and executable were removed.
- The registered WGPU fixture adds a real Flow Variable whose input and output are both named `vector`. It covers the native scene restore and screen hover and selection actions in Detail and Normal. The full Cargo build and mounted WGPU test were left to the owning build lane.
