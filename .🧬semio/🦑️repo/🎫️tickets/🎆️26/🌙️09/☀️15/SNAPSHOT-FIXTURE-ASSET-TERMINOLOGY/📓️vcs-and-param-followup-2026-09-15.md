# VCS And Param Follow-Up — 2026-09-15

## Flow VCS

- `FlowVcsDocument::fixture_mut` → **`host_snapshot_mut`** (all call sites in `🌊️flow/🌿️vcs/🦀️.rs`).
- Artifact DSL round-trip: `flow_host_snapshot_dsl_to_host_snapshot(dsl: …)` — param was misnamed `fixture` with a broken body after bulk rename.

## Flow snapshot keyboard helpers

- `tree_from_host_snapshot`, `keyboard_order`, `keyboard_step` — fixed mixed `fixture` / `host_snapshot` identifiers left by param script.

## Flow host locals

- `build_tree` / `rebuild_dag` — keep **`let fixture = self.build_dag_host_snapshot_v1()`** (ephemeral `DagHostSnapshot` value, not a test fixture); script must not rewrite `&fixture` in those blocks.

## Param script (second pass)

- `🔧️rename-flow-host-snapshot-params.py` extended for qualified `…::FlowHostSnapshot` types; **+14 files** (total **42** across both runs).
- Script **must not** apply body rewrites to `&fixture` when `fixture` is a local `DagHostSnapshot` binding — manual fix required after run.

## Dag board host

- `DagHost::load_fixture_json` → **`load_host_snapshot_json`**; WASM `DagSession` already exposes `loadHostSnapshotJson`.
- `DagHost::fixture_json()` retained as alias to `host_snapshot_json()` (internal Rust only).

## Generation3d viewer

- `evaluate_fixture` → **`evaluate_host_snapshot`** (evaluates a flow host snapshot graph, not a test fixture).

## Dag helper rename (2026-09-15)

Script `🔧️rename-dag-fixture-symbols.py` (**9 Rust files**):

| Old | New |
| --- | --- |
| `dag_fixture_to_wire_literal` | `dag_host_snapshot_to_wire_literal` |
| `dag_fixture_execution_rows` | `dag_host_snapshot_execution_rows` |
| `dag_fixture_from_document` | `dag_host_snapshot_from_document` |
| `validate_dag_fixture_node_kinds` | `validate_dag_host_snapshot_node_kinds` |
| `build_dag_fixture` | `build_dag_host_snapshot` |

Hand fixes: navigate-graph keyboard helpers (broken mixed `fixture`/`host_snapshot`), Process3d/Shooting snapshot params, Flow diff projection, wasm `.d.ts` `hostSnapshotJson` / `loadHostSnapshotJson`.

- Regenerate `flow_core.d.ts` / `framework_surface.d.ts` wasm-bindgen outputs for DagSession renames.
- `ShootingSnapshot`, `Puzzle3dFixture`, `Process3dSnapshot` params still named `fixture` where type is snapshot/fixture domain type.
- `dag_fixture_to_wire_literal` / `dag_fixture_execution_rows` function names (DagHostSnapshot params).
