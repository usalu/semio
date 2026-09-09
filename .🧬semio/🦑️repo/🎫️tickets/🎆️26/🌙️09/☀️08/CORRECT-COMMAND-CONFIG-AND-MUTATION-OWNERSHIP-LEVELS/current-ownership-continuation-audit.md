# Current Ownership Continuation Audit

## Scope

Read-only reassessment on 2026-09-09 of the non-Puzzle, non-Wires,
non-Rewriting, non-shared-value config/mutation/command surface. This report
updates the inventory with the current working tree and distinguishes a source
owner that is actually wired to the framework from an empty per-window taxonomy
directory.

The required owner vocabulary is:

| Owner | Meaning |
| --- | --- |
| `WindowConfig` | persisted local preference keyed by an exact concrete window instance |
| `WindowTransient` | local ephemeral draft, gesture, selection, popup, or result keyed by that instance |
| `AppTransient` | app-local derived/read-model state that is intentionally shared across that app's local surfaces |
| `Operation` | retained work input, checkpoint, progress, cancellation, and completion receipt |
| `OS/session projection` | shell/device identity or directory read model, outside an artifact editor |

No code, generated output, Git state, or AGENTS file was changed. No Cargo/Nx
command was run by this audit.

## Current completion and evidence matrix

| Family | Current source state | Evidence status | Decision |
| --- | --- | --- |
| Writer | `WriterPlayApp` is `NoConfig`/`NoTransient` at `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1025-1032`; it registers `WriterMainWindowConfigOwner` and `WriterMainWindowTransientOwner` at `:1079-1084`. | `writer-equation-handoff.md` records green Ajv + independent `fast-json-patch` oracle and green 2/2 native runtime test. | **Complete.** Do not reopen absent a regression. |
| Equation | `EquationPlayApp` is `NoConfig` at `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1164-1171` and registers `EquationGraphWindowConfigOwner` at `:1226-1227`. | The handoff records green two-window Ajv/`fast-json-patch` oracle and green 2/2 native runtime test. | **Complete.** |
| Jack graph camera/LOD and text selection | graph `WindowConfigOwner` is real at `.../🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🦀️.rs:49-86`; editor selection has a real `WindowTransientOwner` at `.../📝️editor/🫧️transient/🦀️.rs:79-87`. | The ticket has green graph owner oracle/check reports, but the assembled editor/query runtime remains pending in `jack-query-output-ownership.md:48-78` and `jack-query-runtime-bounds.md:31`. | **Partial; retain the native-runtime gap.** |
| Block 3D brush preview | `Block3dWorldWindowTransient` and `SetBrushPreview` exist under `.../🧱️block/.../🪟️windows/🌐️world/🫧️transient`; render receives the addressed preview at `.../🌐️world/🦀️.rs:58-63`. | Ticket report records focused Puzzle dependency check; unit fixture tests exercise the transient mutation. No broad native ownership result is claimed. | **Preview move complete in source; remaining Block config is a separate batch.** |
| Procedural Generation 2D preview | `Generation2dTransient` owns `generation_preview_text`; command work emits `CompleteWithEphemeral` at `.../🌀️generation2d/.../🦀️.rs:194-213`, and renderer reads `transient.snapshot` at `:814-817`. | Source confirms the correct `AppTransient` lane; no new native completion evidence found. | **Preview move complete in source.** |

The rest of the matrix below still has a concrete app-config publisher and no
implemented exact window owner. A `📌️.empty.md` below a window taxonomy path is
not a config/transient implementation and is not validation evidence.

## P0 batch: complete Jack's remaining app-wide collision

`JackConfig` now contains only `jack_query` at
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:8-9`,
but the app still declares it as `ArtifactEditor::Config` at
`.../✏️editor/🦀️.rs:644-651`. The editor renderer reads it at
`🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs:10-12`; retained `runQuery` falls
back to it at `🎮️commands/▶️run-query/🧵️job/🦀️.rs:188-190`. This makes two
concrete query editors overwrite a durable app-wide source buffer.

`JackTransient` likewise stores one app-wide `query_execution_id`, `result`, and
`query_error` at `🫧️transient/🦀️.rs:10-14`. The retained job emits the root
transient mutation at `🎮️commands/▶️run-query/🧵️job/🦀️.rs:156-158`, and every
Results renderer reads that same root at `✏️editor/🦀️.rs:904-910`. A query from
one editor can therefore replace another editor's Results window. This conflicts
with the prior decision in `trinity-jack-rewriting-state-ownership-audit.md`:
query source is an Editor-window persisted preference and query result is a
Results-window ephemeral result.

Required implementation:

1. Add an exact Editor `WindowConfig` owner for `jack_query`; capture it in the
   retained query operation input and make text editing emit only an addressed
   window-config mutation.
2. Add a Results `WindowTransient` owner for execution id/result/error. Capture
   the originating concrete editor and explicit results target in the retained
   job context; publish completion only to that results window. Do not infer a
   target from a window kind.
3. Delete `JackConfig`, `JackConfigMutation`, its preparation factory, and
   `JACK_RETAINED_CONFIG_TOOL_IDS`; then make `TrinityJackPlayApp` `NoConfig`.
   Delete the root `JackTransient` result transport after its last consumer has
   moved. Keep document graph mutations and the already-migrated graph camera
   owner unchanged.

Required parity: update every Rust/TypeScript/GraphQL/JSON Schema/Proto config
and transient mirror plus job-publication contracts. Required runtime proof:
two query editor/result pairs execute concurrently, retain independent sources
and results, reload only each editor config, reset both result transients, and
leave graph document and app config unchanged. The existing query oracle is not
enough until the assembled app runtime completes successfully.

## P0 batch: make the existing display families real window owners

All of the following fields are local display or tool state. Their render and
command consumers are already concrete surfaces, so the narrow correct owner is
the named exact window. They must not become a `BTreeMap` inside app config.

| Family and current app config | Exact consumers | Required partition |
| --- | --- | --- |
| Layout `🎚️config/🦀️.rs:27-40`: `active_page_id`, `drop_preview`, `engagement_input`, `camera`, `preview_camera` | commands convert screen coordinates with the selected global camera at `🎮️commands/🛬️canvas-drag-over/🦀️.rs:18-21`; Preview renders `preview_camera` at `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:39-41`; canvas renders `drop_preview` at `🖼️canvas/🦀️.rs:160-161`. | Blueprint and Preview cameras plus active page are `WindowConfig` for their own concrete windows. Drop preview and engagement draft are `WindowTransient` for their originating canvas. Commands must read the addressed surface value before calculating document mutations. |
| Note `🎚️config/🦀️.rs:26-32`: `engagement_input`, `camera` | Composite renderer reads camera at `🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs:95-96`; submit consumes draft at `🎮️commands/📤️engagement-submit/🦀️.rs:18-20`. | Composite camera is `WindowConfig`; engagement input is `WindowTransient`. The Navigator must not receive another camera copy unless it has a separate viewport producer. |
| Sequence `🎚️config/🦀️.rs:29-41`: `last_run_json`, `orientation`, `camera` | Main/compiled/script surface taxonomy exists; app config unit test still asserts `last_run_json`/orientation at `🎚️config/🧪️tests/🔬️unit/🦀️.rs:5-8`. | Main viewer camera/orientation are `WindowConfig`; compiled execution output is `Operation` or `WindowTransient` for its result surface, never a persisted replayed `last_run_json`. |
| Remodeling `🎚️config/🦀️.rs:70-78`: `camera`, `layers`, `frame_cursor`, `report_table` | Model world uses camera/layers at `🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs:74-159`; Frames uses cursor at `🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs:95-96`; Report uses table selection at `🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs:120-121`. | Model camera/layer visibility, Frames cursor, and Report table selection are independent `WindowConfig` records. Do not preserve them as one cross-mode app configuration. |
| GIS Map `🎚️config/🦀️.rs:26-40`: layer visibility, camera, render/vector/LOD modes, layer stroke scale | Map option panel reads LOD and layer weights at `🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🔽️lod-mode/🦀️.rs:45-47` and `.../📏️layer-weights/🦀️.rs:18-22`. | All listed fields are `WindowConfig` for the concrete Map view. Layer maps must be stored within that window record, not in global editor config. |
| GIS Terrain `🎚️config/🦀️.rs:21-23`: `camera_json` | Terrain scene directly receives it at `🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️.rs:70-72`. | `WindowConfig` for each Terrain view. |
| Shooting `🎚️config/🦀️.rs:26-47`: defaults, selection, centering/fit, camera draft, camera | Save-camera uses global camera/draft at `🎮️commands/🎥️camera/🦀️.rs:48-52`; selection toggle creates a global fit revision at `🎮️commands/🗂️selection/🦀️.rs:77-81`; export reads camera at `🎮️commands/🖨️export/🦀️.rs:35-36`. | Scene camera/defaults/centering/fit and local selected-shot view are `WindowConfig` or framework interaction selection. Camera draft label is `WindowTransient`. An export/save operation must capture the exact camera/defaults input so a later viewport action cannot change the saved/exported outcome. |

These seven families have only old config unit tests asserting defaults or old
config mutations (for example Layout `🎚️config/🧪️tests/🔬️unit/🦀️.rs:5-9`, GIS
Map `...:7-21`, Remodeling `...:6-10`). The audit found no language-neutral
two-instance oracle or native owner-runtime evidence for them. The migration
batch must add both before declaring it complete.

## P1 batch: remaining visual/tool configuration

| Family | Current field(s) and source evidence | Required owner |
| --- | --- | --- |
| Block 3D | `active_representation_id`, `wanted_tags`, `windows`, brush kind/radius/flip, and camera remain in `🎚️config/🦀️.rs:22-45`. Brush controls are rendered from that global config at `🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🖌️brush/🦀️.rs:26-52`; world rendering still takes it at `🌐️world/🦀️.rs:58-63`. | Brush preferences/camera/representation filters are `WindowConfig` for the World instance. `windows` is host layout/view lifecycle and must be derived from the framework, not persisted inside app config. Keep the completed brush-preview `WindowTransient` independent. |
| Lowpoly | `LowpolyConfig` still mixes object focus, tool parameters, colour, camera, engagement draft, edges and sun at `🎚️config/🦀️.rs:23-54`. | Paint parameters/colour, focus, camera, edge display, sun are `WindowConfig` for the owning view; utility selection is host state; typed engagement is `WindowTransient`. Existing framework interaction owns selection/hover and must remain the sole owner. |
| Procedural 2D | preview output is correctly transient, but `camera`, `show_mode`, and `selected_generation_id` remain in global config at `🎚️config/🦀️.rs:23-31`. | Flow/preview camera and show mode are exact `WindowConfig`; selected generation is `WindowConfig` for the selection surface unless an explicit app-wide local chooser is specified. Keep output `AppTransient`. |
| Procedural 3D | config keeps LOD/show/cameras/sun/selection and `preview_eval_text` at `🎚️config/🦀️.rs:69-89`. Its own comment admits the computed evaluation is persisted because it is reconstructed per dispatch; `SetPreviewEval` still writes config at `:191-193,410-413`; Preview consumes the text at `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:49-65`. | `preview_eval_text` is remaining computed-preview misclassification: move it to `AppTransient` or, if independent preview surfaces are required, the exact Preview `WindowTransient`; retain resumable evaluator/checkpoint state as `Operation`. Move actual view settings to their exact `WindowConfig`. |
| Drawing | `engagement_input`, camera, and trace generation/completed/pending remain in `🎚️config/🦀️.rs:18-30`. | Camera `WindowConfig`; input `WindowTransient`; trace counts/progress `Operation` projection. This remains the P0 job-state follow-up from the earlier audit. |
| Raster | brush size/opacity, measured `composite_viewport`, and camera are still config at `🎚️config/🦀️.rs:20-32`. | Brush/camera `WindowConfig`; physical viewport is `WindowTransient` (a measurement), never restored as a preference. |

Block preview has a narrow typed fixture/mutation test but no reported full owner
runtime. Procedural 2D's preview move is source-correct. Neither fact validates
the remaining app-config fields in this batch.

## P1 batch: interaction, jobs, and host projection state

| Family | Current fields | Correct owner and requirement |
| --- | --- | --- |
| Forms | `current_step_index`, `try_values`, `contributions_json` in `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:410-417`; display merges attempt values at `:153-169`. | Step is `WindowConfig`; unfinished inputs are `WindowTransient`; contributions come from host program/view context. The existing document-contract check validates artifact field exclusion, not this local-state ownership. |
| Flow | view controls, catalogue/automation, contributions, generation JSON, and duplicate-widget progress are all in `.../🌊️flow/.../🎚️config/🦀️.rs:35-61`. | Camera/LOD/proximity/grid/preview visibility are `WindowConfig`; generation/progress are `Operation`/`AppTransient`; contributions are host-owned. Catalogue/automation may remain deliberate `AppConfig` only after a producer trace demonstrates an app-wide local workspace preference. |
| CAD | selection/hover, engagement draft/pane/session/preview/generation/finalization, multiple cameras, sun/dislocate, and contributions share `.../📐️cad/.../🎚️config/🦀️.rs:84-146`. | Framework interaction/presence owns selection/hover; drafts and pane state are `WindowTransient`; preview/session/finalization are `Operation`; per-world cameras/display controls are `WindowConfig`; contributions are host program data. This is a large mixed-owner config and needs a schema split before commands are migrated. |
| FEM 2D/3D | each global config retains `result_source_id`, result mode/index, and camera at `.../🏗️fem/.../🎚️config/🦀️.rs:18-34`. | Result selection and mode/camera are view-local `WindowConfig` for each result surface. Preserve document result data; do not turn it into config merely because a renderer selects a source. |
| Architect Program | search/query/history/result/analysis/filter/camera remain in `.../🏛️architect/.../🎚️config/🦀️.rs:24-42`. | Register/filter/camera are `WindowConfig`; query draft is `WindowTransient`; outputs are `AppTransient` or a retained search `Operation`; intentional user search history may remain `AppConfig`. |
| Home Space | active tab plus directory receipt/session/auth/client fields remain in `.../🪐️space/.../🏠️home/.../🎚️config/🦀️.rs:122-139`, and directory events are folded there at `:162-189`. | Tab is `WindowConfig`; directory page/read-model and receipt/binding/auth plus client identity are `OS/session projection`. This cannot be completed in a plugin-only window batch; it needs the existing directory/identity host owner. |

No native owner-lifecycle evidence was found for these six migrations. FEM's
broader native artifact gate is specifically blocked before the relevant runtime
in `native-validation-diagnostics.md:161`; it must not be used as a passing FEM
ownership result.

## Dependency-ordered execution batches

1. **Finish Jack only.** It already has concrete owner APIs and an independent
   query oracle. Move the remaining source/result channels, delete the app
   config/root transient, fix assembled runtime compilation, then run its
   two-editor/two-results lifecycle law.
2. **Display-family template batch:** Layout, Note, Sequence, Remodeling, GIS
   Map/Terrain, Shooting. Define one schema-first window config record per real
   surface, route typed action mutations by `window_id`, then change render and
   coordinate conversion to read the window projection. Split drafts/results
   before deleting their global config mutations.
3. **Visual/tool batch:** Block 3D residual settings, Lowpoly, Procedural 2D/3D,
   Drawing, Raster. Reuse the complete Block preview and Procedural 2D transient
   patterns. The Generation 3D `preview_eval_text` move is the highest-priority
   corrective slice inside this batch.
4. **Mixed-lifetime batch:** Forms, Flow, CAD, FEM, Architect Program, Home.
   First separate schemas by owner, especially operation snapshots and
   OS/session projections. Then move producers and consumers. Do not recreate
   framework selection/presence or directory/identity storage in a plugin.
5. **Evidence gate for every batch.** Add a language-neutral two-concrete-window
   trace, validate it with the existing independent TypeScript/Ajv plus
   `fast-json-patch` route where applicable, and add native runtime coverage for
   reload isolation, transient reset, operation cancel/resume, unchanged document
   bytes, and terminal cleanup. Update Rust, TypeScript, GraphQL, JSON Schema,
   Proto, text codecs, mutation fixtures, and aggregate artifact/diff contracts
   in the same owner move.

The inventory's old default/config unit tests are not sufficient parity: they
mostly assert the state that must be removed. Only Writer and Equation satisfy
the complete source, independent oracle, and native runtime standard today.
