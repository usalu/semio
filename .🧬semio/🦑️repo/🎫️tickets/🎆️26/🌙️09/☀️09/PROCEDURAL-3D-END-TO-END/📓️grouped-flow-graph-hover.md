# Grouped Flow Graph Hover

World3d's authored coincident groups now reach every matching directional port on the Flow canvas. The scene carries a primary node and port and the companion port addresses. A companion change updates the canvas even if the primary and source window stay the same. Addresses use the existing separator and side encoding. An empty channel name remains distinct from an absent port in the native projection.

React resolves the primary and companions against the current scene nodes and uses one typed Flow hover operation. Native WGPU resolves the same incoming marks through its admitted Flow port index. The DAG retains sorted physical handle identities for paint and keeps the singular pointer target for graph edits and publication. When zoom hides handles, the same paint pass marks their owning nodes. A new local pointer takes ownership and successive scene echoes cannot restore an old companion group.

## Verification

- The neutral React ownership regression failed first because the initial group was absent. The green suite passed 17 mounted-hook and ledger tests, including a stationary primary with a replaced companion, exact input/output identity, clearing and scoped text collisions. The test drives the real scene-to-session adapter and observes both directions in one native call.
- The independent SQLite oracle checked ordered matching to authored port sides and companion-only replacement. The ABI Ajv oracle passed against all 109 operations and eight events. The compact Rust witness compiled the actual DAG port mutation and paint predicate against the cached Pack JSON library and passed five cases. See the [source audit](📓️grouped-flow-graph-hover-audit.md) for the witness boundary.
- The native scene round-trip and WGPU canvas laws are registered for complete-group paint and clearing without changing the primary. They await a linked runtime. The canonical source-closure measure passes for all eleven inspected owners. The app's React plan is waiting for the repository's shared checkout graph leader to finish inventory, so these results do not include a fresh canonical application mount.
