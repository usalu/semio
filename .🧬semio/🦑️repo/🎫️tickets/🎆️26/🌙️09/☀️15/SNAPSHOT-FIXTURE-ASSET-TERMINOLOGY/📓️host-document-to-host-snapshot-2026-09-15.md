# Host Document → Host Snapshot — 2026-09-15

## Rationale

Interim rename `FlowFixture` → `FlowHostDocument` still used forbidden product vocabulary **document**. Authoritative terms remain **artifact**, **snapshot**, **diff**, **asset** (runtime static), **fixture** (test-only under `🧫️fixtures/`).

The framework graph shape held by `FlowHost` / node-graph WASM is a **host snapshot** (editable graph projection), distinct from:

- **`FlowSnapshot`** — persisted artifact snapshot (plugin, composed child)
- **`FlowArtifact`** — artifact model
- **`FlowDiff` / `FlowDelta`** — VCS deltas (`hostSnapshot` root replace)

## Mapping (representative)

| Before | After |
| --- | --- |
| `FlowHostDocument` | `FlowHostSnapshot` |
| `DagHostDocument` / `SequenceHostDocument` | `*HostSnapshot` |
| `flow.hostDocument` schema | `flow.hostSnapshot` |
| VCS `ReplaceFlowHostDocument` | `ReplaceFlowHostSnapshot` |
| `FlowOwner::HostDocument` | `FlowOwner::HostSnapshot` |
| `host_document_json` / `hostDocumentJson` | `host_snapshot_json` / `hostSnapshotJson` |
| `setHostDocument` (node-graph wire) | `setHostSnapshot` |
| `hostDocumentChanged` (gesture answer) | `hostSnapshotChanged` |
| `synchronizeDocumentJson` / `documentJson()` (flow WASM ABI) | `synchronizeSnapshotJson` / `snapshotJson()` |
| `to_host_document` / `from_host_document` | `to_host_snapshot` / `from_host_snapshot` |

## Scripts

- `🔧️rename-host-document-to-host-snapshot.py` — bulk (~482 files)
- `🔧️rename-host-document-followup.py` — leftover identifiers + import-document path skip fix

## Intentionally unchanged

- Pack / IO: `encode_document`, `parent_document_id`
- Retained UI: `UiDocument*`, `begin_document`
- Command modules still named `import-document` / `export-document` (directory ids; follow-up)
- `setFixtureJson` / board `fixtureJson` (separate tracks)
- UI ids like `sequence-play-document.*` (surface ids, not data model)
- **`DagSession` WASM** exports **`loadHostSnapshotJson` / `hostSnapshotJson`** (was `loadFixtureJson` / `fixtureJson`; Rust still calls `DagHost::load_fixture_json` internally — rename follow-up).

## 2026-09-15 follow-up (surface + renderer)

- `NodeGraphScenePayload.fixture_json` / JSON **`fixtureJson`** → **`host_snapshot_json` / `hostSnapshotJson`** (`framework/🗺️surface/🕸️node-graph`).
- `GraphSession` wasm-bindgen export **`hostSnapshotJson()`** (was `fixtureJson()`).
- React `NodeGraph` DAG path commits via **`session.hostSnapshotJson()`**; probe registry exposes **`hostSnapshotJson`**; DOM **`data-host-snapshot-json`**.
- Wgpu frame-worker generated scene catalog: **`NodeGraphScene.hostSnapshotJson`** field.
- Flow protocol-unit test asserts **`snapshotJson`: 2609** (ABI schema already migrated).

## Verification (partial)

- Flow `🔬️source-contract/🟦️.ts` — extended bans on `HostDocument`, `hostDocumentJson`, `host_document`
- Follow-up compiles: sequence `host_from_host_snapshot` split from artifact `host_from_snapshot`

## Remaining

- Scan `rg 'HostDocument|host_document|hostDocument'` outside intentional retain list
- Regenerate flow WASM ABI JS if schema op names changed
- Full `cargo test` / renderer vitest sweep
