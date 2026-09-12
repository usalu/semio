# FEM2D and FEM3D Window Migration Preparation

## Audited State

FEM2D and FEM3D each define Model and Results editor window kinds. Their four concrete `🎚️config` directories are placeholders. Each app instead persists a shared four-field config containing `resultSourceId`, `resultMode`, `resultModeIndex`, and `camera`. The Model renderer reads the shared camera. The Results renderer reads the same camera plus the three result-display fields. Consequently a camera change in either window crosses into every Model and Results instance, and a result-display change crosses into every Results instance.

Both registered retained routes declare `setCamera` and `setResultDisplay` on the app `Config` publication lane. Their reducers construct `ConfigView { window: None }`. The direct `ArtifactEditor::handle` path discards `view_state`. `setActiveExample` also emits app-config resets for camera and result display. The two app schemas repeat the four fields in Rust, TypeScript, JSON Schema, GraphQL, and protobuf.

The same defect exists independently in both dimensions:

| Artifact | Model kind | Results kind | Camera representation |
| --- | --- | --- | --- |
| FEM2D | `fem2d-model` | `fem2d-results` | typed `{ x, y, zoom }` |
| FEM3D | `fem3d-model` | `fem3d-results` | current opaque `FemCamera { json }` wrapper |

## Exact Ownership Design

Create separate concrete window records because the kinds own different state:

- Model: camera only.
- Results: camera, nullable result source id, result mode, and non-negative result mode index.

The four owners belong in the existing concrete window `🎚️config` directories. Each owner must declare its exact `WINDOW_KIND_ID`, strict schema identity, bounded store owners, one-item preparation factory, and disposer. Register all four with the corresponding `ArtifactEditor`.

After removing the four view fields, retain a structurally empty `Fem2dConfig`/`Fem3dConfig` and minimal app-config mutation type only if the generated command surface still requires a concrete app config. Prefer that over `NoConfig` for this bounded migration because every structural command currently names the concrete config type and changing all command signatures would mix unrelated work into this slice. App schema facets must become strict empty records. The old view mutations and their size-accounting/preparation branches must disappear.

`setCamera` must resolve the exact current record from `ConfigView.window`, then use `ViewModel.window_id` plus the matching `window_instances` row to select only the Model or Results owner. `setResultDisplay` must accept only an exact Results instance. A payload cannot choose owner identity. Missing view state, a stale id, and the wrong kind must fail closed.

The direct `ArtifactEditor::handle` path should intercept these two view commands with its supplied `view_state`. The retained reducer/job path must retain `ArtifactOwnedToolJobContext.view_state` through completion, following the Equation graph-window pattern, and publish `WindowConfig`. `setActiveExample` should only load the requested document; it must preserve each open window's local preferences. Update publication contracts so `setCamera` and `setResultDisplay` use `WindowConfig`, and remove the obsolete Config output from `setActiveExample`.

Render must use the exact supplied `WindowConfigSnapshot`:

- FEM2D Model resolves `Fem2dModelWindowConfig`; FEM2D Results resolves `Fem2dResultsWindowConfig`.
- FEM3D Model resolves `Fem3dModelWindowConfig`; FEM3D Results resolves `Fem3dResultsWindowConfig`.
- A missing exact snapshot renders the concrete owner's default.
- Results display conversion moves beside the Results owner instead of accepting the app config.

## Required Contract Evidence

Add schema-first Rust, TypeScript, JSON Schema, GraphQL, and protobuf facets for the two record shapes in each dimension. TypeScript production parsers must reject unknown fields and malformed camera/result values. Neutral fixtures must include defaults, nullable source id, patches for camera and result display, unknown-field and wrong-type vectors, and two same-kind instance ids. Compare production behavior independently with Ajv and `fast-json-patch` through the existing registered FEM document-contract route.

Native laws must instantiate two Model instances and two Results instances for each dimension. Drive camera changes against one Model and one Results instance and a result-display change against one Results instance. Verify each same-kind sibling, the other kind, the app pack, and the document pack remain unchanged. Then load the same document bytes, close/reopen with the exact window packs, and prove only the addressed records restore. Also require explicit rejection for stale ids, Model-targeted result display, and foreign window kinds.

## Source Scope

The implementation is bounded to these source groups in each artifact:

- `✏️editor/🎚️config`: remove the four persisted view fields, their mutations, preparation/accounting branches, tests, and five-facet schema declarations.
- `✏️editor/🎮️commands/🎥️set-camera`, `👁️set-result-display`, and `📚️set-active-example`: move view output to exact window ownership and stop resets on document replacement.
- `✏️editor/🦀️.rs`: retain view context in both direct and retained execution, register concrete owners, update lanes, and resolve exact render snapshots.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🎚️config` and `📊️results/🎚️config`: replace placeholders with schema-first state, mutation, owner, and ownership laws.
- Existing Model/Results render tests and editor retained-route tests: replace shared-app assertions with exact instance assertions.

No FEM source was changed during this preparation audit.
