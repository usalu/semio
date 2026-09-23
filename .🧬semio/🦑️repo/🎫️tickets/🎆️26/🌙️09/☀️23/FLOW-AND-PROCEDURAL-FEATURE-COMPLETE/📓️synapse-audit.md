# Synapse Visibility Audit

Source: synapse audit on 2026-09-23.

## Root cause

`DagHost::rebuild_engine_with_layout` in `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (~4747–4756) calls `create_edge` only when both handle lookups succeed. Handles are keyed by `IoPortSpec.id` (channel names such as `number`, `text`, or `""` for preview). `FlowHost::build_dag_host_snapshot_v1` in `🌊️flow/🖥️host/🦀️.rs` (~1965) copies `SynapseSpec.from_port` / `to_port` verbatim. Those fields are often legacy `out` / `in` or empty, so the lookup misses and the canvas paints nothing. Paint walks `engine.edges`, not the fixture edge list.

Example: generation3d flow outline fixture stores `fromPort: "out"` while the handle is `height@number`.

## Fix

In `build_dag_host_snapshot_v1`, resolve each synapse endpoint to the canonical `IoPortSpec.id` (same rules as `connect_ports` / `first_output_port` / `first_input_port`, including legacy `out` / `in`) before formatting `source` and `target`. Do not change the engine skip: a synapse whose ports do not exist must still produce no wire.

## Test

Extend `flow_fixture_with_synapses_builds_dag_edges_and_ports` in `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` (~1382): two widgets, one synapse with `out` / `in`, assert one workflow edge and `host.dag.engine.edges.len() == 1`.

## Applied

`FlowHost::build_dag_host_snapshot_v1` now resolves each endpoint through `resolve_synapse_port` before it formats `source` and `target`. Legacy `out` / `in` and an empty id select the first port on that side. An unknown port or a missing widget emits no edge. `cargo test -p semio-framework-os-flow --lib flow_fixture_with_synapses_builds_dag_edges_and_ports` passed. The two legacy-port tests passed in the same crate. `delete_selection_removes_edge_selected_by_synapse_id_domain` failed once because its selection JSON used `🐙️handles` instead of `handles`; that key is corrected and the test is being rerun.

## Second drop

`would_create_cycle` in the same engine loop (~4742) can skip every edge on a two-edge cycle because `existing` includes all snapshot edges before the loop. Leave that for a later pass. The port join is the missing-synapse bug.
