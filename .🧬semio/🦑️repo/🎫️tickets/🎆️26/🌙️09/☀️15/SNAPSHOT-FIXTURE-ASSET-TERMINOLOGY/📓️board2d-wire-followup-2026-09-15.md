# Board2d And Puzzle Board Wire — Follow-up 2026-09-15

## Problem

`Board2dScene.fixtureJson` / `BoardHost::parse_fixture_json` still use **fixture** for runtime persisted board graph JSON. That collides with the taxonomy: **fixture** = test-only under `🧫️fixtures/`.

DAG board WASM already exposes `loadHostSnapshotJson` / `hostSnapshotJson` (`DagHost::load_host_snapshot_json`). The React board lane and scene spine have not caught up.

## Scoped renames (not started)

| Surface | Current | Target |
| --- | --- | --- |
| `Board2dScene` Rust/TS | `fixture_json` / `fixtureJson` | `host_snapshot_json` / `hostSnapshotJson` |
| `scene/📇️catalog.json` Board2dScene fields | `fixtureJson` | `hostSnapshotJson` |
| `BoardHost` WASM | `parseFixtureJson` | `parseHostSnapshotJson` (body unchanged until schema tokens move) |
| `EngineSurfaceKindDetail::Board2d` | `fixture_json` | `host_snapshot_json` |
| MCP builder | `setFixtureJson` | `setHostSnapshotJson` |

## Schema token debt (separate pass)

Board `parse_fixture_json` accepts runtime schemas such as `puzzle.2d.fixture` and `reasoning.mindmap.fixture`. Those are persisted snapshots, not test fixtures; schema strings should become `*.host_snapshot` or `*.snapshot` with fixture JSON under `🧫️fixtures/` updated in the same change.

## Done this ticket

- Removed unused `DagHost::fixture_json()` alias (call sites already use `host_snapshot_json()`).
