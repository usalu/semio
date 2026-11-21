# Native DAG port-row picking

The native screen-pick query now includes the qualified channel when a graph pointer lands inside a visible port row at detail or micro LOD. It reuses the exact channel-row hit helper already used by native hover. Existing graph targets retain their rank and order. An engine connector hit prevents an added row target, preserving connector priority and preventing duplicate output for the same channel. Coarser levels do not add row hits.

The query uses the existing world projection, native hit geometry, and qualified handle encoding. No asynchronous TypeScript picker changes were needed in this slice.

## Verification

- The neutral DAG fixture covers both port sides, detail and micro row hits, normal/compact/minimap row exclusion, connector priority and title space. The independent Three.js rectangle and radius checks passed all eight fixture rows.
- The isolated Rust source harness passed the pick projector, existing channel-row helper and actual port-row enumeration under the neutral cases. It also verifies graph node/edge order, qualified handle rank and labels, and the single connector candidate. The harness supplies the viewport/hit inputs and inspects typed rows before JSON serialization.
- The linked `board_dag_host_dag_unit` regression drives `dag_pick_targets_at_screen_json` on the real demo graph. It reads native port and connector coordinates, exercises all eight shared cases, and compares the original node/edge list prefix against coarse-LOD picking. Linked execution is pending disk admission.
- Rust AST and scoped whitespace verification passed. Source closure measured 2,100 active graph lines and 1,535 active DagHost fixture lines.
