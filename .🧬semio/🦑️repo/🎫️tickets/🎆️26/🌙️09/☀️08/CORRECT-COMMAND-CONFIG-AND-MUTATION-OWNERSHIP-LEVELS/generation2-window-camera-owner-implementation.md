# Generation2d Exact Window Camera Owner Implementation

## Result

Generation2d editor camera ownership is cut over from the application config to three exact persisted local window-config owners. The main graph, edit preview, and generate preview now keep independent `Viewport2d` values per concrete window instance. The former application `camera` field and `SetCamera` mutation are removed across Rust and every schema facet without aliases.

The authored/default camera values remain `x = 0`, `y = 0`, and `zoom = 1`. Existing Canvas pointer and wheel commands retain their explicit empty-payload, empty-emission behavior.

## Exact Owner Ledger

| Concrete window kind | State | Mutation | Schema and envelope identity | Publication bound | Renderer |
| --- | --- | --- | --- | ---: | --- |
| `generation2d-main` | `Generation2dMainWindowConfig` | `Generation2dMainWindowConfigMutation::Snapshot` | `procedural.generation2d.mainwindowconfig` | 4,096 bytes | `NodeGraph` viewport |
| `generation2d-preview` | `Generation2dEditPreviewWindowConfig` | `Generation2dEditPreviewWindowConfigMutation::Snapshot` | `procedural.generation2d.editpreviewwindowconfig` | 4,096 bytes | edit `Canvas2d` camera |
| `generation2d-generate-preview` | `Generation2dGeneratePreviewWindowConfig` | `Generation2dGeneratePreviewWindowConfigMutation::Snapshot` | `procedural.generation2d.generatepreviewwindowconfig` | 4,096 bytes | generate `Canvas2d` camera |

Each state has its own Rust owner and JSON Schema, TypeScript, GraphQL, Protobuf, and WIT facets. The three states share the canonical OS-kernel `Viewport2d` value. State and Snapshot mutation decoding reject unknown fields. Pack decoding verifies the exact artifact identity, `Pack` component, and version 1; it rejects missing, foreign-owner, wrong-component, and wrong-version envelopes.

## Command and Render Routing

`NodeGraphViewport` is classified as a `WindowConfig` publication. The raw handler emits nothing. The retained Generation2d reducer requires the trusted command context, resolves the captured exact `ViewModel`, reads the current `generation2d-main` snapshot, and emits one addressed `WindowConfigMutation`. It rejects missing view context, stale concrete IDs, and wrong window kinds.

All three render paths resolve their corresponding exact owner from `ConfigView.window`. Two same-kind instances per kind therefore render six independent viewport values. The edit preview continues to read application-owned `show_mode`; only its camera moved.

Canvas pointer-down, pointer-move, pointer-up, and wheel remain `HostOnly` empty-payload commands whose raw handlers and actual retained receipts have no mutation or event lanes. No camera interaction was invented for them.

## App Surface Removal

`Generation2dConfig` now contains only application-level show-mode and selection state. Its Rust mutation aggregate and JSON Schema, TypeScript, GraphQL, Protobuf, and WIT facets contain no application camera field or `SetCamera` variant. The preparation/publication bridge and former unit expectations were updated to match the closed application contract.

## Tests

The neutral fixture describes two `generation2d-main`, two `generation2d-preview`, two `generation2d-generate-preview`, and one foreign instance. Its language-agnostic oracle validates the fixture and each owner schema with Ajv 2020, applies mutations independently with `fast-json-patch`, checks same-kind and cross-kind isolation, exact reopen JSON, Canvas no-ops, stable document/application JSON, and application-camera rejection.

The native law covers:

- all three state and mutation diff, inverse, DSL, Pack, text-op, and binary-op codecs;
- hostile unknown state and mutation fields and missing/foreign/component/version Pack envelopes;
- raw NodeGraph and four raw Canvas handlers;
- six concrete runtime window instances with distinct values;
- the actual retained NodeGraph route and its exact `WindowConfig` lane, with no application `Config` lane;
- actual retained Canvas no-op receipts;
- main `NodeGraph` and both `Canvas2d` renderer families before and after reopen;
- unchanged document and application Pack and SPR bytes;
- missing, stale, and wrong-kind context rejection; and
- bounded shutdown inside an explicitly sized 2 MiB test thread.

The known generic inner `WindowConfigPack` concrete-identity admission counterexample remains a shared-loader issue outside this bounded editor cutover. The test does not weaken it or add rejected-candidate synchronization drops.

## Registered Routes

The root and ticket Nx projects expose `generation2d-window-camera-ownership-oracle` and `generation2d-window-camera-ownership-native`. Both root and seed VS Code launch files register the oracle at order `311.226` and native at `311.227`. The permanent commands live only in the applicable `📜️script.ts` files. The native route enables `component-app-assembly` and selects the three Generation2d ownership laws by their shared test-name prefix.

## Verification

- `bun nx run workspace:generation2d-window-camera-ownership-oracle`: PASS through the registered root Nx target. The durable 65-line, 3,063-byte run log records `[DEBUG] generation2d-window-camera-ownership owners=3 instances=6 canvasNoOps=4 appCamera=absent` and `NX Successfully ran target generation2d-window-camera-ownership-oracle for project workspace`.
- `bun ./📜️script.ts verify generation2d-window-camera-ownership native`: pending a clean rerun after the coordinated shared Pack API edit ends. The latest attempt reached the feature-enabled Generation2d editor, exposed only incorrect `Viewport2d` namespaces in this slice, and those references are corrected to `semio_framework_os_kernel::Viewport2d`. The immediate rerun then stopped before Generation2d compilation in the concurrently edited shared Pack module, so it is not recorded as acceptance.

## Source Ledger

- Three exact owner implementations and their six schema facets live under the `🎚️config` directories of the main graph, edit preview, and generate preview window kinds.
- Window module roots and the Generation2d editor root register and consume the owners.
- The Generation2d retained reducer, `NodeGraphViewport` handler, publication contract, renderer paths, and editor preparation logic implement the routing cutover.
- The Generation2d application config and its schema facets remove camera ownership.
- The neutral fixture, neutral schema, TypeScript oracle, and Rust native law live in `✏️editor/🧪️tests/🪟️generation2d-window-camera-ownership`.
- Root `📜️script.ts`, root `📋️project.json`, ticket `validation/📜️script.ts`, ticket `validation/project.json`, `.vscode/launch.json`, and `.vscode/🧩️launch.seed.jsonc` expose the repeatable routes.

No Generation3d production source or shared Store, Pack, or framework implementation was changed for this slice.
