# Synapse Visibility Fix

## Cause

1. `build_dag_host_snapshot_v1` already resolves legacy `out`/`in` (and empty) synapse ports onto `IoPortSpec.id` via `resolve_synapse_port`.
2. Play windows still dropped wires: `dag_host_snapshot_to_workflow` emitted port records as `nodeId@portId` while edge `sourcePortId`/`targetPortId` stayed bare. `GraphHost::sync_from_payload` → `port_to_io` keyed handles on the full port id, so `create_edge` never found both handles.

## Change

- `port_handle_id` / `port_to_io` in `🗺️surface/🕸️node-graph` strip a `node@` prefix so handle keys match edge endpoints.
- Flow + generation2d `dag_host_snapshot_to_workflow` emit bare `port.id` on port records (same as edge port fields).
- Unit tests cover legacy `out`/`in` → one engine edge, and a missing port → zero engine edges.
