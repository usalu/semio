# 🧊️ Generation3d preview chain: removal and rewire map

Read-only audit for plan `📓️brep-mesh-widget-set-2026-10-07.md` (AD2, AD5). Nothing in source was edited, built or deleted.

Abbreviations: `GEN` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`. `ART` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`. `PBK` = `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural`. `FLOW` = `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow`. Paths are relative to `/Users/ueli/Documents/semio`.

Classes: **DELETE** = pure extension or tick plumbing with no user-visible behaviour of its own. **REWIRE** = keep the user-visible behaviour, change the source to the inference (`ArtifactInferenceService` plus `geometry`/`analysis` fields). **KEEP** = unaffected, or still needed by the flow plugin.

## 0. Method and limits

- The `grep` tool is broken, so every search used `/usr/bin/grep` or `find` with quoted globs (zsh aborts on unquoted `--include=*.rs`). `timeout` does not exist on this macOS, so no command used it.
- A repo-wide symbol sweep (`grep -rl` over `.`) did not finish within the time budget and was stopped. The result file `.../PROCEDURAL-3D-END-TO-END/hits-files.txt` is a 0-byte leftover from that sweep, written into the ticket folder by this audit. It should be removed by the coordinator (it is not part of the plan).
- Coverage is therefore: every directory and file under `ART` (scanned by symbol with line numbers), the framework flow and tool-run modules, the playbook extension, the composition tests, `.vscode/launch.json`, and the `project.json` targets. The `ART` symbol count is about 2,260 lines of hits; this map groups them by file and gives representative line anchors, not every line.
- Items marked "unverified" were inferred from names or comments and not read in full.

## 1. The current chain, end to end

### 1.1 Evaluate (editor and viewer)

1. A widget edit enters as a mutation `change-widget-input` / `set-widget-input` (`GEN/✏️editor/🎮️commands/🎚️set-widget-input/🦀️.rs:378`, `GEN/🧬️schema/🧬️mutations/🎛️change-widget-input/🦀️.rs`). Its editor `Emit` carries the preview owe via `owe_attached_previews_carrying` (`GEN/✏️editor/🦀️.rs:984-990`, `1564-1567`, `1324-1328`).
2. The owe sets a window latch (`preview_eval::owe_attached_previews`, `GEN/🧵️preview-eval/🦀️.rs:221`; `…_for_mutations` at `252`, `…_carrying` at `272`).
3. The `previewEval` tool run (started by `pending_effects`, `GEN/✏️editor/🦀️.rs:2576-2587`; viewer `GEN/👁️viewer/🦀️.rs:1318-1335`) dispatches `flowEvalTick` hops. `tick_effect` / `release_effect` at `GEN/🧵️preview-eval/🦀️.rs:144-149`.
4. A tick is `evaluate_tick` (`GEN/🧵️preview-eval/🦀️.rs:946`). It emits `ExtensionInvocation::new(extension, "evaluate", request, "flowEvalResolve")` (`🦀️.rs:994`), or, once no `evaluate` is pending, `preview_tessellate_invocations` (`🦀️.rs:839`, invocations at `874`, response `"flowTessellateResolve"`).
5. The geometry extension is address `"brep"` (`GEN/🧵️preview-eval/🦀️.rs:330-337`, `geometry_extension_address`), resolved by `semio_framework_os_flow::flow_extension_invocation_address` (`FLOW/🗿️artifacts/🌊️flow/📔️registry/🦀️.rs:603`).
6. Answers fold back through window-addressed routes: `flowEvalResolve` → `resolve_eval` (`GEN/🧵️preview-eval/🦀️.rs:1087`), then `continue_inline` (editor `⏱️flow-eval-tick/🦀️.rs:106-119`) runs the next wave inside the same turn. `flowTessellateResolve` → `resolve_tessellate` (`🦀️.rs:1159`).
7. Publication goes into the window's transient and then to the preview body (`GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:89-119`, generate variant `…/🧬️generate/…/👁️preview/🦀️.rs:71-119`, viewer `GEN/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:393-425`).

### 1.2 Tessellate (view time, but still extension-driven)

- `preview_tessellate_invocations` drives budgeted chunks: `PREVIEW_TESSELLATE_STEP_BUDGET = 24` (`🦀️.rs:305`), wall clock from `semio_framework_os_flow::mesh::TESSELLATE_STEP_WALL_MICROS` (`FLOW/🎒️mesh/🦀️.rs:269`, value 6000) at `🦀️.rs:314`.
- Meshes are pulled through the session (`session_preview_mesh` `🦀️.rs:634`, `mesh_data_for_preview_handle` `🦀️.rs:642`, `mesh_data_for_session_preview_channel` `🦀️.rs:691`), with a residency digest (`🦀️.rs:745`) and retention handles (`🦀️.rs:785`, `819`).
- Per-window mesh table cache in the viewer: `GEN/👁️viewer/.../👁️preview/🦀️.rs:189-327` (`PREVIEW_MESH_TABLE`, `build_preview_mesh_table`, signature via `preview_mesh_signature` `226-252`).

### 1.3 Extension contribution (process-wide registry)

- `setContributions` pushes the shell's `flow.extension` closure in pages. Editor route `GEN/✏️editor/🦀️.rs:1493-1600` (`GENERATION3D_CONTRIBUTIONS_TOOL_IDS`, contract `1520-1522`, `Generation3dContributionsWork` `1528`, factory `1575`). Viewer route `GEN/👁️viewer/🦀️.rs:722-875`.
- The install is `sync_host_flow_extension_contributions_page` (`FLOW/🗿️artifacts/🌊️flow/📔️registry/🦀️.rs:452`), and an invalidation keyed by `flow_extension_registry_generation` (`…/📔️registry/🦀️.rs:561`) owes a fresh evaluation.
- Commands: `GEN/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs:39-55`, `GEN/👁️viewer/🎮️commands/🧩️set-contributions/🦀️.rs:39-53`.

### 1.4 Extension-side (not in this artifact, listed for completeness)

- `flow-extension-brep` and `flow-extension-math` run in their own actors. The composition harness serves them in-process: `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs:59-100` (`evaluate` and `tessellate` handlers) and `…/🔬️flow-operators/🦀️.rs` (contributed plus linked installers, `37-46` and the doc block `1-35`).
- Extension crates: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/` and the math sibling.

### 1.5 Cancel

- The UI shows `cancellable`/`cancelAction`/`cancelArgs` from the status JSON. The cancel dispatches `toolRunAbort` (fixture `GEN/🧫️fixtures/🛑️preview-cancel.json:5`).
- The framework run ledger aborts the job. `PreviewEvalRunJob::close_step` (`GEN/🧵️preview-eval/⏯️tool-run/🦀️.rs:606-640`) calls `session.cancel_preview_evaluation(...)` (`:627`) and dispatches `release_effect` if work is outstanding.
- `flowEvalRelease` (`GEN/✏️editor/🎮️commands/🔓️flow-eval-release/🦀️.rs:16`) → `release_invocations` (`GEN/🧵️preview-eval/🦀️.rs:1119-1131`) emits `evaluateCancel` and `tessellateCancel` invocations, answered by `flowTessellateCancelResolve` (`GEN/🧵️preview-eval/🦀️.rs:1136-1147`, editor command `🧯️flow-tessellate-cancel-resolve/🦀️.rs:19`).

### 1.6 Document-side dependency that stays

- `GEN/🧬️schema/💡️inferences/🦀️.rs:17-40`: `Generation3dInference { topology }` with `reads: ["hostSnapshot"]`. Its only computation is `GEN/🧬️schema/💡️inferences/🧭topology/🦀️.rs:40-81`, which needs no flow session.
- `ART/🦀️.rs:113-115` registers `s.procedural.generation3d.inference.artifact`; `ART/🦀️.rs:150` attaches `generation3d_artifact_inference_descriptor()`.
- There is no `ArtifactInferenceService`, `InferredField`, `DepHash`, or `inference_services` anywhere in `ART` (grep confirmed). The new service is therefore purely additive.

## 2. Command inventory (editor and viewer)

### 2.1 Editor app command table

Declared at `GEN/✏️editor/🦀️.rs:122-164` as `app_commands! { … ctx = FlowEvalSession }`. **The command context is `FlowEvalSession`**, so every handler receives it as `_session`. Removing the session changes the signature of every row below.

| Row (line) | Command | Class | Note |
|---|---|---|---|
| 146 | `flowEvalTick` / `flow-eval-tick` | DELETE | `⏱️flow-eval-tick/🦀️.rs:122-124`; `evaluate` `57-92`, `continue_inline` `106-120`, `chain_ui_scope` `31-38`, `may_rearm` `43-45`, `window_args` `48-50`. The UI-scope part (`chain_ui_scope`) is REWIRE: the same panels must refresh on inference progress. |
| 147 | `flowEvalResolve` / `flow-eval-resolve` | DELETE | `✅️flow-eval-resolve/🦀️.rs:23-38`. |
| 148 | `flowTessellateResolve` / `flow-tessellate-resolve` | DELETE | `🔺️flow-tessellate-resolve/🦀️.rs:23-38`. |
| 149 | `flowEvalRelease` / `flow-eval-release` | DELETE | `🔓️flow-eval-release/🦀️.rs:16-18`. |
| 150 | `flowTessellateCancelResolve` / `flow-tessellate-cancel-resolve` | DELETE | `🧯️flow-tessellate-cancel-resolve/🦀️.rs:19-21`. |
| 151 | `setContributions` / `set-contributions` | DELETE | `🧩️set-contributions/🦀️.rs` (whole file, 55 lines), plus route `GEN/✏️editor/🦀️.rs:1493-1600`. |
| 122 | enum `Generation3dCommand … ctx = FlowEvalSession` | REWIRE | ctx becomes the inference-service handle (or unit). Every row below changes with it. |
| 123-145, 152-164 | all other rows | REWIRE (ctx) | Each handler's `_session: &mut FlowEvalSession` parameter goes. Listed in 2.3. |
| 137, 138 | `nodeGraphViewport`, `setLodMode` | REWIRE | `setLodMode` changes the tessellation tolerance (`🔬️set-lod-mode/🦀️.rs`). Tolerance becomes a view-time parameter (AD5). |

### 2.2 Viewer command table

Declared at `GEN/👁️viewer/🦀️.rs:244-259`. Viewer rows `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`, `flowEvalRelease`, `flowTessellateCancelResolve`, `setContributions` (`252-257`) are all DELETE, with the matching `GEN/👁️viewer/🎮️commands/⏱️…`, `✅️…`, `🔺️…`, `🔓️…`, `🧯️…`, `🧩️set-contributions/` files (each 15-55 lines). Viewer rows `setShowMode`, `setLodMode`, `setCamera`, `toggleSun`, `setSun*`, `setActiveExample`, `exportDocument` are REWIRE (`export-document` reads the session at `GEN/👁️viewer/🎮️commands/📤️export-document/🦀️.rs:32`).

Viewer owner: `GEN/👁️viewer/🦀️.rs:141-200` (`Generation3dViewInstanceOperationOwner`, holds `FlowEvalSession` and `PreviewEvalRunLink`). `build_tool_run_job` `1340-1347` and `pending_effects` `1318-1335` are REWIRE. Contributions route `722-875` is DELETE. Factory registration `1283`, `1369`, `1420`, proof `861-872` are DELETE. Command definitions `1703-1720` (`setContributions` at `1708`) are DELETE. Parse `1453` is DELETE. `FlowEvalSession` parameter `1615` is REWIRE.

### 2.3 Editor and viewer command handlers that take `FlowEvalSession` only as a dead parameter

These files change only in signature and imports. Class REWIRE (ctx), no behaviour change.

`set-sun-azimuth` (`🧭️set-sun-azimuth/🦀️.rs:16`), `set-sun-elevation` (`🌄️…:16`), `set-sun-intensity` (`🔆️…:16`), `toggle-sun` (`🌞️…:14`), `node-graph-viewport` (`🔭️…:19`), `edit-mesh-selection` (`🥽️…:198`, also imports `FlowHost` at `:11`), `import-document-request` (`📂️…:61`), `scale-selection` (`📏️…:33`), `node-graph-edit` (`✏️…:132`), `rename-generation` (`🏷️…:33`), `set-widget-input` (`🎚️…:378`), `cycle-lod-mode` (`🔁️…:16`), `import-document` (`📥️…:121`), `remove-generation` (`🗑️…:32`), `delete-selection` (`❌️…:35,47`), `rotate-selection` (`🔄️…:35`), `remove-widget` (`➖️…:19`), `set-camera` (`📷️…:17`), `add-widget` (`🧩️…:78`), `reorganize` (`🗺️…:15`), `select-generation` (`🎯️…:17`), `set-active-example` (`🎨️…:73`), `patch-flow-widgets` (`🩹️…:36`), `add-generation` (`➕️…:16`), `navigate-graph` (`🧭️…:109,133,142`), `export-document` (`📤️…:62,72`), `set-show-mode` (`👁️…:16`), `set-lod-mode` (`🔬️…:16`), `knife-mesh-selection` (`🔪️…:43`), `cycle-show-mode` (`🔁️…:17`), `update-generation-values` (`🎚️update-…:21`), `translate-selection` (`↔️…:34`).

Note: `patch-flow-widgets` and `edit-mesh-selection` take `FlowHost` directly and run flow evaluation as part of the command. Classify those two as REWIRE (AD6: emit `Generation3dMutation` leaves directly, no scratch `FlowHost`). `GEN/✏️editor/🎮️commands/🧭️transforms/🦀️.rs:17-18` imports `FlowHost` too (REWIRE, `validate_component_gesture` at `:61` is KEEP).

### 2.4 Editor modules that read the eval session (not commands)

| File | Lines | Class | Note |
|---|---|---|---|
| `GEN/✏️editor/🦀️.rs:172-300` | `Generation3dInstanceOperationOwner` (`179`), `with_session` `193`, `with_session_waking` `208`, `owe_attached_previews_for_mutations` `217-220`, `owe_attached_previews_carrying` `226-229`, `preview_eval_parts` impl `234-238`, close policy `246`, `with_scratch_session` `286` | DELETE (owner) / REWIRE (owner shape) | Owner stays as the instance holder for the inference cache and service handle, minus `FlowEvalSession`, `PreviewEvalRunLink` and the owe functions. `with_scratch_session` is DELETE (fresh session per render). |
| `GEN/✏️editor/🦀️.rs:540-620` | `owe_attached_previews`, `self.session: Option<FlowEvalSession>` (`542`), `owe_attached_previews` calls at `603`, `610` | REWIRE | |
| `GEN/✏️editor/🦀️.rs:749`, `984-990`, `1317-1328`, `1564-1567` | reducer and `with_mut` sites that owe previews | REWIRE | Replace owe with "invalidate inference for changed widget ids". |
| `GEN/✏️editor/🦀️.rs:846` | fn taking `session: &mut FlowEvalSession` | REWIRE | |
| `GEN/✏️editor/🦀️.rs:940`, `1964-1966` | owner construction | REWIRE | |
| `GEN/✏️editor/🦀️.rs:2200` | `owner.with_mut` for a mesh read | REWIRE | Reads `session` mesh; becomes inference geometry read. |
| `GEN/✏️editor/🦀️.rs:2581-2597` | `pending_effects` and `build_tool_run_job` (`PreviewEvalRunJob::<…>::new`) | REWIRE | Becomes an `ArtifactInferenceService` run start (progress unit `widget-step`). |
| `GEN/✏️editor/🦀️.rs:2602`, `2624-2634` | `render` and `render_with_request_context` with `with_session` | REWIRE | Preview body reads inference outputs instead of session. `preview_eval_text` (`2617-2620`) moves to the inference status. |
| `GEN/✏️editor/🦀️.rs:3216` | `preview_tessellate_invocations` (editor copy) | DELETE | Duplicate of `GEN/🧵️preview-eval/🦀️.rs:839`. |
| `GEN/✏️editor/🦀️.rs:3364` | `preview_selection_json` | KEEP (shape) / REWIRE (source) | Selection payload shape is part of the UI contract. |
| `GEN/✏️editor/🦀️.rs:3403-3421` | `PreviewPayload` (`meshes_json`, `instances_json`, `components`), `preview_payload_from_eval` | REWIRE | `meshes_json` is JSON text today; becomes typed mesh values (AD3). |
| `GEN/✏️editor/🦀️.rs:3432-3530` | `preview_payload` (eval_json → meshes/instances) | REWIRE | |
| `GEN/✏️editor/🦀️.rs:3532-3542` | `export_mesh_from_session`, `export_meshes_from_session`, `export_meshes_from_evaluation` | REWIRE | Export reads inference geometry. |
| `GEN/✏️editor/🎯️selection/🦀️.rs:143` | `validate_cached_components(snapshot, session: &mut FlowEvalSession)` | REWIRE | Component picks validated against inferred topology/faces (AD4). |
| `GEN/✏️editor/📌️panels/🗿️artifact/🦀️.rs:8,43` | artifact panel `render(…, session: &FlowEvalSession, …)` | REWIRE | Shows the evaluation status. Source becomes inference status. |
| `GEN/✏️editor/📌️panels/🔍️inspection/🦀️.rs:7,458` | `neural::Value` for inspector values | REWIRE | Typed controls (AD7). |
| `GEN/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:97-116` | `flow_palette_catalogue_sections`, `flow_app_catalogue`, `CatalogueItem` | REWIRE | Palette comes from the generation3d catalogue (AD1). Not an eval item, but it reads flow catalogue data. |
| `GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:12,304` | node-graph `render(…, session: &FlowEvalSession, …)` | REWIRE | Node-graph UI stays; its live status source moves. `flow_backed_node_graph_extras` (import `:12`) is an FLOW helper and stays KEEP if the node graph still uses the flow UI. |
| `GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:6-119` | edit preview `render(document, config, preview_eval_text, session, …)` | REWIRE | Keep `world3d_scene`, `fit_json`, `status_json` (`97`), `selection_json` (`92`). Replace `session` with inference. |
| `GEN/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:8-119` | generate preview | REWIRE | Same as edit preview, plus the generation-fixture patch (`98`) which moves to the inference input. |
| `GEN/✏️editor/🎮️commands/📤️export-document/🦀️.rs:10,24,62,72` | export reads retained session | REWIRE | |

### 2.5 Viewer modules that read the eval session (not commands)

| File | Lines | Class |
|---|---|---|
| `GEN/👁️viewer/🦀️.rs:24` | import | REWIRE |
| `GEN/👁️viewer/🦀️.rs:375-433`, `496`, `501`, `588`, `640-649`, `783` | `eval_json` path, `FlowEvalTickOutcome` (`501`), owe (`433`, `783`) | DELETE / REWIRE |
| `GEN/👁️viewer/🦀️.rs:1289`, `1615` | retained session doc and render param | REWIRE |
| `GEN/👁️viewer/🫧️transient/🦀️.rs:22` | "`FlowEvalSession::eval_json()` for the current document" | REWIRE (transient holds inference output reference instead) |
| `GEN/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:155-421` | `ViewPreviewPayload` (`173-184`), `PreviewMeshTable` (`189-194`), `PREVIEW_MESH_TABLE` (`196-200`), `preview_mesh_signature` (`226-252`, keyed on session fingerprint), `build_preview_mesh_table` (`256-310`), `preview_payload` (`312-365`), `preview_selection_json` (`375-386`, KEEP shape), `render` (`393-425`) | REWIRE (table keyed on inference DepHash), selection shape KEEP. |
| `GEN/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:162-170` | `evaluate_host_snapshot` (`#[cfg(test)]`, `with_host`, `host.evaluate()`) | DELETE (test-only, flow evaluation) |

## 3. Preview-eval module (`GEN/🧵️preview-eval/🦀️.rs`, 1475 lines)

Mostly DELETE, with a few KEEP and REWIRE islands.

| Line(s) | Item | Class |
|---|---|---|
| `1-44` | module docs and imports (`FlowEvalSession`, `ExtensionInvocation`, `ToolRunView`, etc.) | DELETE (imports), REWIRE (docs) |
| `57-125` | `FlowEvalTick`, `FlowEvalResolve`, `FlowTessellateResolve`, `FlowTessellateCancelResolve`, `FlowEvalRelease` payload structs | DELETE |
| `132-141` | `EVALUATE_STEP_BUDGET` / `EVALUATE_STEP_WALL_MICROS` (mirror of flow SDK) | DELETE (replaced by inference work-unit budget) |
| `144-153` | `tick_effect`, `release_effect`, `hop_effect` | DELETE |
| `162-180` | `may_rearm` | DELETE |
| `182-203` | `window_args` | DELETE |
| `205-219` | `applied_document_edits_digest` | REWIRE (used as inference cache input) |
| `221-290` | `owe_attached_previews`, `_for_mutations`, `_carrying`, `preview_kind`, `attached_preview_windows` | DELETE, except `preview_kind` and `attached_preview_windows` (`290-303`): REWIRE (window roster stays). |
| `305-330` | step budget `24`, wall, `preview_tolerance` (`318`), `GENERATION_3D_GEOMETRY_EXTENSION_ID` `"brep"` (`330`) | DELETE (`"brep"` address), REWIRE (`preview_tolerance` becomes view-time). |
| `336-361` | `geometry_extension_address`, `is_brep_geometry_handle` | DELETE |
| `363-459` | `PreviewInlineGeometry`, `PreviewChannelItem`, channel item collection, `preview_widget_ids` | REWIRE (channel addressing per widget stays; source is inference output). |
| `463-505` | `mesh_has_preview_geometry`, `PREVIEW_MESH_ROLES`, `preview_mesh_role`, `preview_payload_bounds` | KEEP (role and bounds logic reused by camera fit). |
| `528-584` | `PREVIEW_FIT_PADDING`, `preview_fit_revision`, `preview_fit_json` (`576`) | KEEP (camera fit, AD5 "camera fit" behaviour). Source of `revision` changes. |
| `586-625` | point/vector markers, `apply_show_mode_mesh` | KEEP |
| `626-761` | `decode_preview_mesh_pack`, session mesh fetch, residency digest, pack fingerprint | DELETE (replaced by typed `HalfedgeMesh` values, view-time tessellation retained by `DepHash`) |
| `763-840` | `preview_eval_publication_for`, `preview_mesh_retention_handles`, `pending_preview_tessellate_handles`, `preview_tessellate_invocations` (`839`) | DELETE |
| `885-1028` | `FlowEvalTickOutcome`, `chain_ui_scope` (`909`), `tick_is_unfinished`, `evaluate_tick` (`946`), `node_census_digest`, `census_chrome_marks` | DELETE, except `census_chrome_marks` (`1042-1085`) and `chain_ui_scope` (`909`): REWIRE to inference progress (the UI still needs chrome marks for node status). |
| `1087-1160` | `resolve_eval`, `note_eval_answer_fault`, `release_invocations`, `resolve_tessellate_cancel`, `resolve_tessellate` | DELETE |
| `1184-1230` | `preview_status_json`, `merge_status_json` | REWIRE (status object, see §4) |
| `1231-1257` | `preview_scene_status_json` | REWIRE |
| `1244-1257` | `preview_eval_run_is_abortable` | KEEP (abort affordance rule) |
| `1258-1300` | `preview_progress_status_json` / `_for` (phase outranks rules, `1276-1282`, `1269` tessellation ledger comment) | REWIRE (phase now comes from the inference service's `ToolRunProgress` / `PreviewTessellatePhase`) |
| `1372-1420` | extension evaluate fault, diagnostic entries | DELETE (extension faults gone) |
| `1445-1470` | `PreviewStatusDebug` (`meshes_json`, `instances_json` lengths), `preview_window_status_json` | REWIRE |
| `1469-1475` | `mod tool_run; pub use tool_run::*; mod tests;` | REWIRE |

## 4. The `previewEval` tool-run contract (documented for replacement)

This is what the UI relies on today. A replacement must keep these observable facts, even if the source changes.

### 4.1 Declaration (source of record `GEN/🧵️preview-eval/🔣️.json`)

- `toolId: "previewEval"`, `iconId: "eye"`, label EN "Preview evaluation" / DE "Vorschauauswertung".
- `definition`: `mutating: false`, `rebase: "restart"`, `reconfigure: "restart"`, `unit: "nodes and meshes"` (DE "Knoten und Netze").
- Stages (index order): `evaluate` ("Evaluating nodes"), `tessellate` ("Tessellating meshes").
- Counters (index order): `evaluated`, `failed`, `blocked`, `tessellated` ("Meshes tessellated"), `hops`.
- Reasons (code, id, verdict): 0 `queued` testing; 1 `computing` testing; 2 `evaluated` success; 3 `blocked` warning; 4 `failed` danger; 5 `tessellating` testing; 6 `tessellated` success; 7 `meshDiagnostics` warning; 8 `meshMissing` warning; 9 `settled` success; 10 `settledWithFailures` warning; 11 `extensionFault` danger. Rust enum `GEN/🧵️preview-eval/⏯️tool-run/🦀️.rs:78-125` must match these codes (test asserts this per the record note).

### 4.2 Rust types and commands (`GEN/🧵️preview-eval/⏯️tool-run/🦀️.rs`)

- `PreviewEvalRunStage` `28-47`, `PreviewEvalRunCounter` `50-75`, `PreviewEvalRunReason` `78-125`.
- `preview_eval_tool_definition` `139-144`, `preview_eval_run_definition` `134-137`.
- Observation: `PreviewEvalObservation` `162-196` (`count`, `stage`, `progress() -> (completed, total)`), `observe_preview_eval` `208-240` (reads per-widget status JSON and eval JSON), `PreviewEvalHop` `254-262` (`Dispatch(index)`, `Wait`, `Settled`), `next_preview_eval_hop` `264-277`.
- Run link: `PreviewEvalRunLink` `279-309` (windows, port, `settled`, `job_run`).
- Job: `PreviewEvalRunJob<O>` `449-600`:
  - `new` `468-494`: owes every window one evaluation and attaches the port.
  - `progress(state)` `510-531` builds `ToolRunProgress { identity, sequence: 0, state, stage, completed, total: Some(total), counters, units_per_second: 0.0, conflicts: 0, steps }`.
  - `settle` `533-544` writes `Settled` (success, counts nodes and tessellated) or `SettledWithFailures` (warning) as a step.
  - `emit` `545-551` writes a `PreviewReady` payload.
  - `step` `553-600`: `Dispatch` increments hops and `consume_fuel(1)`, `Settled` completes, `Wait` yields or emits Running.
  - Trace: `record` `496-509` upserts one trace record per node whose reason changed, keyed by `preview_eval_node_entity` (FNV-1a 64 of the node id, `:146-151`).
  - Cancel: `begin_close` `602`, `close_step` `606-640` (see §1.5).
- Start: editor `GEN/✏️editor/🦀️.rs:2591-2597`, viewer `GEN/👁️viewer/🦀️.rs:1340-1347`, gated on `request.tool_id == "previewEval"` and `ToolRunJobPurpose::Run`.

### 4.3 Framework side (generic, KEEP)

- `ToolRunState` (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:115`): `starting, running, paused, complete, finalizing, finalized, aborting, aborted, faulted`.
- `ToolRunProgress`, `ToolRunCounter`, `ToolRunStepRing`, `ToolRunTickWriter`, `ToolRunJobPort` (`dispatch`/`wait`/`wake`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs:150-183`), `ToolRunView` (`:201`), `is_tool_run_action_id` (`:219`, seven reserved ids including the abort), `ToolRunLedger` (`:713`), `begin_close` (`:1151`), `close_step` (`:1164`).
- Start action: `toolRunStart` requires `toolId` (`🛍️products/…/⏯️tool-run/🦀️.rs:1604`).
- Abort action: `toolRunAbort` (cancel fixture `cancelAction`).
- The generic run machinery is not flow-specific, so it stays in framework. Only the `previewEval` job in generation3d is replaced.

### 4.4 Progress and status object seen by the UI

Producer: `preview_window_status_json` / `preview_progress_status_json_for` (`GEN/🧵️preview-eval/🦀️.rs:1266-1282` and `1453`). Consumer: the shell parser `world3dComputeStatusV1` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:393-501`).

Fields the parser reads (type `World3dComputeStatusV1`, `🟦️.ts:393-424`): `computing`, `phase` (wire tag, e.g. `idle`, `samplingEdges`, `meshingFaces`, `cancelled`, `faulted`), `phaseLabel` (EN/DE), `hint` (empty-surface text, ≤200 chars), `unitsDone`, `unitsTotal`, `facesDone`, `facesTotal`, `inFlight`, `ratio` (clamped 0-1), `cancellable` (honoured only with non-empty `cancelAction`), `cancelAction` (`"toolRunAbort"`), `cancelArgs` (string or number values, a tool run's `{runId, generation}`).

Phase labels (from `GEN/🧫️fixtures/🛑️preview-cancel.json:9-33`): `idle` Idle/Bereit, `samplingEdges` Sampling edges/Kanten werden abgetastet, `meshingFaces` Meshing faces/Flächen werden vernetzt, `cancelled` Cancelled/Abgebrochen, `faulted` Geometry extension unavailable/Geometrie-Erweiterung nicht verfügbar.

Visible behaviour: `WorldComputeStatusPane` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:4507-4561`, cancel button `data-slot="world-compute-cancel"` at `:4561`), and `declareSurfaceCancelAction(cancelActionId)` (`:6261-6269`).

Not re-verified: the exact key names inside `cancelArgs` as produced by `framework/🛍️…/⏯️tool-run` (the TS doc says `runId`/`generation`).

### 4.5 Cancel law

From `GEN/🧫️fixtures/🛑️preview-cancel.json` (`format: semio.generation3d.preview-cancel`): a cancel stamps `phase: "cancelled"`, freezes `unitsDone/unitsTotal/facesDone/facesTotal`, sets `cancellable: false`, and quiesces every window latch (`armed=false, inFlight=0, owed=false, unfinished=false`). `cancelling twice is idempotent`. The fixture's `cancelResponseAction: "flowTessellateCancelResolve"` and `cancelCapability: ["evaluateCancel", "tessellateCancel"]` are DELETE (extension protocol). The laws themselves are REWIRE: they become "an abort on the inference job freezes progress, stops in-flight work and leaves the status cancelled".

## 5. Tests, fixtures and oracles

### 5.1 Unit and law tests inside `GEN`

| File | Hits | Class |
|---|---|---|
| `GEN/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` | 9 laws (`:27` run effects, `:111` gesture rearm, `:209` history verb, `:276` supersession, `:305` geometry dependencies, `:320` bounded cancel). Uses `FlowEvalSession` at `:30,141,187,245,259,322` | DELETE the flow-tick laws (`:27`, `:111`, `:209`, `:276`); REWIRE `:305` (geometry dependencies become inference `reads`) and `:320` (bounded cancel law stays, against the inference job). |
| `GEN/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 41 hits, 3 use `Generation3dInstanceOperationOwner` (`:30`, `:69`, `:85`, `:589`), `FlowEvalSession::new` (`:94`) | REWIRE |
| `GEN/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs:47` | owner work-capacity | REWIRE |
| `GEN/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | 6 hits | REWIRE |
| `GEN/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs` | 5 hits | REWIRE (status contract stays, source changes) |
| `GEN/🧪️tests/🔬️status-contract/🟦️.ts` | 24 hits, imports `world3dComputeStatusV1` from `@semio-tech/framework`, rebuilds the status object from the fixture | KEEP (third-party-style twin; retargets to the inference fixture) |
| `GEN/🧪️tests/🩹️gesture-rearm/🟦️.ts` | 17 hits, Rust replay of `FlowEvalSession` latches | DELETE (latch semantics gone) |
| `GEN/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts` | 19 hits (TS twin of the tick chain) | DELETE |
| `GEN/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🟦️.ts` | budget twin, mentions `FlowEvalSession` at `:7,227` | DELETE |
| `GEN/✏️editor/🧪️tests/🔬️tick-addressing/🟦️.ts` | 4 hits, tick addressing | DELETE |
| `GEN/✏️editor/🧪️tests/💥️extension-evaluate-fault/🟦️.ts` | 4 hits | DELETE |
| `GEN/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts` | 3 hits | REWIRE |
| `GEN/✏️editor/🧪️tests/🔬️mode-panels/🟦️.ts`, `🕹️interaction-scope/🟦️.ts` | scope and panel laws | REWIRE (`chain_ui_scope` expectations) |
| `GEN/🏅️…/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` | 4 hits (topology inference) | KEEP, extend with `geometry`/`analysis` laws |
| `GEN/🔨️modules/🏠️host/🧪️tests/🔬️retained-authority-laws/🦀️.rs` | 13 hits; uses `Widget::Neuron`, `neural` types only | KEEP |
| `GEN/🧪️tests/📐️example-geometry-3d-1/🥒️.feature` (21) and `🐍️.py` (5) | example-geometry language-agnostic and third-party oracle | REWIRE (the same example expectations, now inference-driven); the oracle stays. |
| `GEN/📚️examples/🧪️tests/🧩️geometry/🟦️.ts` | TS twin of the example-geometry lane (13 hits) | REWIRE |
| `GEN/🧪️tests/🫧️mutate-…-any-viewer-transient/🥒️.feature` (3) | viewer transient mutation law | REWIRE |

### 5.2 Fixtures (`GEN/🧫️fixtures/`) with an eval-chain role

| Fixture | Role | Class |
|---|---|---|
| `🛑️preview-cancel.json` (50 hits) | cancel law, phase labels, `cancelResponseAction` | REWIRE (law kept against the inference job; extension capability names DELETE) |
| `⏯️preview-eval-run.json` (38 hits) | run vocabulary, verdict law, scheduling law over latches, kernel release | REWIRE. Keep vocabulary (stages, counters, reasons); DELETE latch and release parts. |
| `⏱️evaluate-budget.json` (9 hits) | budget cut of a long `evaluate` | REWIRE to the inference work-unit budget (`widget-step`). |
| `🔁️incremental-eval.json` (3 hits) | one slider edit's dirty-set cost (hex column) | REWIRE: dirty-set precision becomes `DepHash` / `DiffRegions` (AD2). Keep the law. |
| `🪟️tick-addressing.json` (21 hits) | `flowEvalTick` window addressing per surface | DELETE |
| `🎨️example-switch.json` (11 hits) | example switch triggers contributions and evaluation | REWIRE |
| `📐️geometry-dependencies.json` | dependency contract of geometry | REWIRE (becomes inference `reads`) |
| `🎚️slider-gesture.json`, `🎛️generate-mode-interactions.json`, `🕹️interaction-scope.json`, `📌️mode-panel-publication.json` | interaction and publication laws touching preview | REWIRE |
| `🧵️preview-eval/🔣️.json` | tool-run declaration (source of record) | KEEP (declaration) / REWIRE (job) |
| `✏️editor/🧫️fixtures/💥️extension-evaluate-fault.json`, `🚧️contribution-gated-arming.json` | extension fault, contribution gated arming | DELETE |
| `🚪️io/🧫️fixtures/🛡️authority/🔣️.json` (4 hits) | authority law fixture; `FlowEvalSession` only as literal source text inside sample strings | KEEP (text samples), check if an authority scan forbids the identifier (unverified). |
| `🪟️tick-addressing`/`🛑️preview-cancel` note reference `🔒️tick-latch.json` | file **does not exist** in `GEN/🧫️fixtures` (grep count 0) | stale reference; DELETE with the tick laws. |

### 5.3 Composition tests (`✏️s/🧑‍💻dev/🧩️composition/`)

Cargo declarations: `📦️packages/🦀️rust/Cargo.toml`.

| Item | Lines | Class |
|---|---|---|
| `[[test]] generation3d-example-geometry` → `🧪️tests/🧩️generation3d-example-geometry/🦀️.rs` (`install_flow_extension` `:356`, schema `:129`) | Cargo `:63-64` | REWIRE: the lane compares example geometry; it currently installs flow extensions. Must run against inference, keep the parry3d and fnv oracles. |
| `[[test]] generation3d-incremental-eval` → `🧪️tests/🔁️generation3d-incremental-eval/🦀️.rs` (`install_flow_extension` `:129`, reads `FIXTURE_JSON` `:32`) | Cargo `:67-68` | REWIRE |
| `[[test]] generation3d-app-laws` → `🧪️tests/🧊️generation3d/🦀️.rs` (835 lines; imports `flow_eval_*` commands at `:85`, `:101`; `preview_eval` modules at `:73-76`; `FlowEvalSession` usage) | Cargo `:70-71` | REWIRE (command module imports and extension-driven laws). |
| `🧪️tests/🔬️brep-extension/🦀️.rs` (224 lines) | in-process `evaluate`/`tessellate` extension host (`:59-100`) | DELETE (extension host) |
| `🧪️tests/🔬️flow-operators/🦀️.rs` (204) | installs contributed and linked brep/math operators | DELETE |
| `🧪️tests/🔬️serial/🦀️.rs` (96) and `🔬️publication-authority/🦀️.rs` (26) | one serial lock over flow registry, neural cache and mesh cache | REWIRE (drop flow-registry state; keep the publication lease lock) |
| `🧪️tests/🔒️serial-lock-discipline/🦀️.rs` | serial lock laws | REWIRE |
| `🧪️tests/🧩️generation3d-example-geometry/🦀️.rs` | see above | REWIRE |
| `🧪️tests/🔁️generation3d-incremental-eval/🦀️.rs` | see above | REWIRE |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/` symlinked mounts (`#[path]`) | composition mounts `GEN` preview-eval unit tests at `:76` and `:82-104` | REWIRE (remove the mount with the DELETE laws) |

### 5.4 Other crate-level test anchors

- `GEN/🚪️io/🧪️tests/🔁️round-trip/🦀️.rs` (`io-round-trip` `[[test]]`, `GEN` Cargo `:101-103`) and `🧪️tests/🗿️artifact-surface/🟦️.ts` are KEEP.
- The `GEN/📦️packages/🦀️rust/Cargo.toml` `dev-dependencies` `parry3d` (`:91`) and `fnv` (`:93`) are oracles. KEEP (`fnv` is the trace-key oracle for `preview_eval_node_entity`; `parry3d` may be retained as the example-geometry oracle, see §6).

## 6. Dependencies (Cargo)

### 6.1 Generation3d artifact (`ART/📦️packages/🦀️rust/Cargo.toml`)

| Line | Dependency | Class |
|---|---|---|
| `20` | feature `component-app-assembly` lists `dep:semio-framework-os-flow`, `dep:machine`, `dep:semio-framework-os-infinite`, `dep:semio-framework-tool-machine` | REWIRE: drop `dep:semio-framework-os-flow` from this feature (once no `semio_framework_os_flow` use remains in the artifact). Keep the others. |
| `36` | `semio-framework-artifact-flow-flow` (schema types) | KEEP (document stays `FlowHostSnapshot`, AD1) |
| `37` | `semio-framework-artifact-infinite-dag` | KEEP (unverified use) |
| `66` | `semio-framework-os-flow` (optional) | REMOVE |
| `86` | dev-dep `semio-framework-os-flow` (comment `:83-85`, "headless example-geometry harness") | REMOVE with the example-geometry lane |
| `91` | dev-dep `parry3d = "0.17"` | KEEP as oracle (or REMOVE if the example lane is dropped) |
| `93` | dev-dep `fnv = "1.0.7"` | KEEP (trace-key oracle) |
| `110` | dev target dep `semio-framework-plugin-host` (`cfg(not(wasm32))`) | REWIRE (only needed for in-process extension hosting, unverified) |
| `101-103` | `[[test]] io-round-trip` | KEEP |

Non-flow `[dependencies]` on `semio-s-artifact-stdio-*`, `semio-framework-2d`, `semio-framework-ui*`, etc. are unaffected.

### 6.2 Composition (`✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust/Cargo.toml`)

| Line | Dependency | Class |
|---|---|---|
| `35` | `semio-framework-os-flow` | KEEP for the flow composition laws, REWIRE if generation3d-only |
| `36` | `semio-s-plugin-flow-extension-draw` | KEEP (not generation3d) |
| `39` | `semio-framework-os-kernel-neural-engine` | REWIRE (only needed by generation3d flow evaluation, unverified for other laws) |
| `41-44` | imperative math/text/effect/control | KEEP (not generation3d) |
| `49` | `semio-s-plugin-flow-extension-brep` (`component-guest`) | DELETE for generation3d lanes; KEEP for flow composition tests |
| `50` | `semio-s-plugin-flow-extension-math` | same as `49` |
| `54` | `semio-framework-artifact-flow-flow` | KEEP |
| `59` | `parry3d` | KEEP |
| `63-71` | `[[test]]` `generation3d-example-geometry`, `generation3d-incremental-eval`, `generation3d-app-laws` | see §5.3 |

### 6.3 Playbook procedural (`PBK/📦️packages/🦀️rust/Cargo.toml`)

| Line | Dependency | Class |
|---|---|---|
| `38` | `semio-s-plugin-flow-extension-math` | DELETE |
| `39` | `semio-s-plugin-flow-extension-brep` | DELETE |
| `40` | `neural_engine` (`semio-framework-os-kernel-neural-engine`) | DELETE |
| `41` | `semio-framework-artifact-flow-flow` | KEEP (schema) |
| `45` | `flow` (`semio-framework-os-flow`) | DELETE (after rewiring its `FlowHost` use) |

Playbook procedural code: `PBK/🦀️.rs:1-10` imports `flow::{forms_bridge::flow_host_snapshot_to_form_spec, FlowHost}`, `neural_engine::{Registry, SharedRegistry, …}`, and `Session` from `semio_s_spatial_kernel_semio_session`. `:403-404` registers the math and brep extensions into a `Registry` built in `ModuleGeometryOwner::new`. `:425-427` builds a `FlowHost` with the operator registry and geometry port. `:475-520` is `evaluated_preview_payload` (`tessellate_geometry` at `:499`, `:519`). `:550-593` covers import and `ExportSolid`. This is a second, separate consumer of the same chain. It is not covered by the plan's generation3d AD list and needs its own decision (see §8).

## 7. Launch, nx and TS

### 7.1 `project.json` nx targets

`ART/📦️packages/🦀️rust/📋️project.json` (project `@semio-tech/procedural-generation3d-rs`):
- `build`, `check`, `test` (`bun ./📜️script.ts build|check|test`): KEEP.
- `verify-generation3d-widget-inputs` (line `43`): REWIRE (checks change-widget-input leaves, which become typed).
- `verify-document-restoration-oracle` (`53`): KEEP.
- `canonical-architecture` (`62`): KEEP (architecture check; may need allow-list changes for the new inference, unverified).
- `verify-generation3d-document-io` (`71`): KEEP.
- `verify-generation3d-preview-window-transient` (`88`): REWIRE (checks the window transient that holds eval output).
- `semantic-wire-check` (`105`): REWIRE (wire contract includes `flowEval*` action ids; unverified line-by-line).
- `test-snapshot-sqlite` / `-native` / `-source` (`128`, `146`, `164`): KEEP.

`ART/📦️packages/🟦️typescript/📋️project.json`: `build`, `check`, `test`: KEEP.

Script router: `ART/📦️packages/🦀️rust/📜️script.ts` imports the terminology, snapshot-fixture, keyboard, mesh-selection, widget-input and widget-creation TS self-tests and `testGeneration3dIoInputContracts` / `testGeneration3dIoAuthorityFixture`. It has verify routes `document-restoration-oracle` and others. Any route that loads `🛑️preview-cancel.json`, `⏯️preview-eval-run.json`, `🪟️tick-addressing.json` or the `tick-latch` reference needs to be checked (unverified; not read in full).

### 7.2 `.vscode/launch.json` (93 generation3d entries; seed `.vscode/🧩️launch.seed.jsonc` has 59)

Per the AGENTS rule, every executable command is registered there. Entries that touch the chain or will need rewiring:

| Entry (line) | Command target | Class |
|---|---|---|
| `⚖️gate🏢️semio-tech🎡️play🧊️generation3d-semantic` (3444) | `procedural-generation3d-rs:semantic-wire-check` | REWIRE |
| `⚖️gate…🧊️generation3d-io` (3462) | `verify-generation3d-document-io` | KEEP |
| `⚖️verify-generation3d-document-io🟦️` (17862) | same | KEEP |
| `⚖️verify-generation3d-preview-window-transient🟦️` (17873) | `verify-generation3d-preview-window-transient` | REWIRE |
| `⚖️verify-generation3d-widget-inputs🧊️generation3d🦀️` (27023) | `verify-generation3d-widget-inputs` | REWIRE |
| `🛠️dev🌀️procedural🧊️generation3d⚛️react` (1078), `…🧊️wgpu🌐️wasm` (1100), `…🧊️wgpu🖥️native` (6985), `🛠️serve🌀️procedural🧊️generation3d⚛️react` (6963), `🛠️dev♻️mit-bestand🌀️procedural🧊️generation3d⚛️react` (1452) | dev serve targets | REWIRE (the served app must boot the inference path; no launch names change) |
| `📦️generation3d-preview-window-transient` (58512), `…-native` (58523) | preview-window-transient checks | REWIRE |
| `▶️generation3d-document-io` (58490), `⚖️generation3d-document-io-native` (58501) | document io | KEEP |
| `⚖️test-generation3d-app-laws🧑‍💻dev🧩️composition🦀️` (65156) | composition `generation3d-app-laws` | REWIRE |
| `⚖️test-generation3d-incremental🧑‍💻dev🧩️composition🦀️` (65178) | composition `generation3d-incremental-eval` | REWIRE |
| `⚖️geometry-contract🧩️extensions🌀️procedural🦀️` (66003) | playbook procedural extension geometry contract | REWIRE / decision (§6.3, §8) |
| `⚖️test-generation3d-inspector-controls♿️react🟦️` (19548) | inspector controls a11y | REWIRE |
| `⚖️test-generation3d…` and `⚖️gate…test-snapshot-sqlite-native` runs (`--args='long --offline --lib -E test(...)'`, lines 19193, 19521, 19567, 20221, 20328, 20740, 20786, 20836) | filter names include `document_io…`, `document_restoration…`, `generation3d_original_diff…` | KEEP where the filter is not flow; REWIRE where it names flow laws (check each filter before editing). |
| `⚖️generation3d-…` composition runs (`--test generation3d-app-laws --law-filter widget_input_`, `…inspector_brep_controls_`, `…mesh_brep_live_scoped_selection_`, `…mesh_`) (19276, 19293, 19312, 19329, 19489) | law filters | REWIRE (brep control names are kept; the laws move onto inference). |
| `⚖️verify-generation3d…` / `⚖️generation3d-semantic-wire-*` (39851, 39862) | semantic wire source and native | REWIRE |

Not generation3d but touching the same flow extensions, KEEP: `🌊️flow` entries (`⚖️gate…🌊️flow-native` 3480, `🛠️dev🌊️flow…` 704/842/6842, flow composition checks 7650, `⚖️flow-window-ownership-*` 58116-58138), and playbook extension `⚖️geometry-contract🧩️extensions🌀️procedural🦀️` (66003, see above).

The launch entries are one-line nx commands. This audit did not read all 93 generation3d entries. Rule for implementers: any new inference gate needs a new launch entry in both `launch.json` and the seed, following the existing `⚖️`/`▶️`/`📦️` naming.

### 7.3 TypeScript consumers

| File | Use | Class |
|---|---|---|
| `GEN/🟦️.ts` (root) and `GEN/📦️packages/🟦️typescript/📜️script.ts` | TS package router | KEEP |
| `GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🟦️.ts:3` | "`&FlowEvalSession`" boundary doc | REWIRE |
| `GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🟦️.ts:3` | `(…, &FlowEvalSession, activeUtility)` boundary | REWIRE |
| `GEN/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🟦️.ts` | generate preview TS | REWIRE |
| `GEN/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🟦️.ts` | viewer preview TS | REWIRE |
| `GEN/✏️editor/🎮️commands/🧩️add-widget/🧪️tests/🔬️unit/🟦️.ts`, `🎚️set-widget-input/…`, `🧭️navigate-graph/…`, `🥽️edit-mesh-selection/…` | TS command twins; some name `FlowEvalSession` as doc | REWIRE |
| `GEN/✏️editor/🧪️tests/…` (see §5.1) | TS laws | see §5.1 |
| `GEN/🚪️io/…`, `GEN/🧬️schema/…` (`🧬️mutations/…` ~60 TS files) | mutation mirrors (TS) | KEEP (AD6 adds leaves; ~3 TS files per leaf) |
| `GEN/📚️examples/…` | example TS | REWIRE only the geometry twin |
| Framework renderer `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` | `buildContributionsJson` (`🎯️targets/⚛️react/🟦️.tsx:368`), `dispatchInvokeExtensionEffect` (`🏛️ShellHost/🟦️.tsx:2089`, caller `:7234`), `publicInvocationStringPages` (window-fault test `🩺️window-fault/🟦️.ts:181`), `flowEvalTick` deferral comments in `🛠️ShellHelpers/🟦️.tsx:1179,1304,6430,6469`, `🔌️PluginRuntime/🟦️.tsx:1838,2186,3675`, `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:838,978,1527`, `🕸️NodeGraph/🟦️.tsx:2488,2760`, `🌐️World3dHost/🟦️.tsx:6260` | KEEP (generic extension and contributions plumbing, still used by the flow plugin and the playbook extension). Comments naming `flowEvalTick` are REWIRE once the flow plugin no longer uses the generation3d variant. |
| Framework `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` | `World3dComputeStatusV1` | KEEP (shape is the UI contract) |
| Framework `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (≈11 k lines, many `flowEvalTick` fixtures) | renderer contract laws | KEEP (generic); the `flowEvalTick` example rows are REWIRE to a neutral action id when generation3d stops using it. |
| Framework `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📖️read-only-run/🟦️.ts`, `🛑️terminal-quiet/🟦️.ts`, and `🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/🟦️.ts` | tool-run laws | KEEP (generic) |

## 8. What the flow plugin (not procedural) still needs (must stay in framework/flow)

The flow plugin has its own editor commands for these, so none of them may be deleted from `FLOW` or from the flow plugin:

- Flow host and eval session: `FlowEvalSession` (definition `FLOW/🖥️host/🦀️.rs:3289`), `FlowEvalPublication` (`:3274`), `FlowEvalPublicationLedger` (`:3200`), `FlowEvalSessionState` (`:3079`), `flow_host_with_session` (`:4634`). The flow plugin's own commands are `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/` with `⏱️flow-eval-tick` (`🦀️.rs`), `🏁️flow-eval-resolve`, `🧮️evaluate`, `🧩️set-contributions` (with its `🧪️tests/🧩️registry` and `🔬️unit`). Keep all of them.
- Flow extension registry: `FLOW/🗿️artifacts/🌊️flow/📔️registry/🦀️.rs` (`register_linked_flow_extension_installer` `:113`, `install_flow_extension` `:322`, `sync_host_flow_extension_contributions_page` `:452`, `flow_extension_registry_generation` `:561`, `flow_extension_invocation_address` `:603`).
- Extension wasm evaluation: `FLOW/🧩️extensions/🕸️wasm/🦀️.rs` (`flow_inference_dependency_json` `:9`, `evaluate_invoke_json` `:85`, `EVALUATE_STEP_BUDGET` `:123`, `cancel_evaluation` `:491`, `cancel_all_evaluations` `:497`, `ExtensionEvaluationResources` `:528`).
- Mesh tessellation step: `FLOW/🎒️mesh/🦀️.rs:269` (`TESSELLATE_STEP_WALL_MICROS`).
- Flow catalogue and palette: `FLOW/🗂️catalogue/🦀️.rs` (used by generation3d's catalogue panel today; generation3d moves to its own catalogue, but the flow catalogue stays for the flow plugin).
- Flow extension plugins: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/` (`flow-extension-brep`, including `evaluate-budget` and `extension-guest-standalone` tests) and `…/🧮️math/` (flow-extension-math).
- Flow shell/renderer plumbing (see §7.3): `dispatchInvokeExtensionEffect`, `buildContributionsJson`, `publicInvocationStringPages`, the `flowEvalTick` deferral and hop-trace code, the World3d status pane and cancel affordance (KEEP; generic).
- Framework plugin mention `FLOW`-adjacent: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14220` (doc only, `Mutex<FlowEvalSession>` driver). Not generation3d. KEEP.
- Framework generic tool-run: `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` and the plugin tool-run module (§4.3). Keep.

Shared between flow and generation3d and therefore not to be deleted from `FLOW`: `flow_inference_dependency_json` (the flow plugin's own dependency contract), `ExtensionEvaluationResources`, mesh tessellation step budget.

## 9. Open decisions and risks

1. **Playbook procedural (`PBK`) is a second consumer.** It links `neural_engine`, `flow-extension-brep` and `flow-extension-math` directly, and tessellates through `tessellate_geometry`. The plan's AD list does not mention it. Decide: move it to the same generation3d inference (preferred, one geometry source) or keep it on the flow plugin path. If kept, its Cargo deps (§6.3) stay and only the generation3d chain is removed.
2. **`FlowEvalSession` as command context.** About 35 command handlers take it as `_session`. Changing `ctx` (`GEN/✏️editor/🦀️.rs:122`) changes the `app_commands!` macro expansion for the whole artifact in one edit. Do it in one wave.
3. **Node-graph window (`GEN/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`) keeps the flow graph UI.** The graph's live status (`render(…, session)` `:304`) and `flow_backed_node_graph_extras` (`:12`) are flow helpers. Confirm whether the node-graph UI itself stays flow-backed or moves to the generation3d graph.
4. **Stale fixture reference.** `🛑️preview-cancel.json` and `🪟️tick-addressing.json` reference `🔒️tick-latch.json`, which does not exist in `GEN/🧫️fixtures`. Remove the references with the tick laws.
5. **Repo-wide symbol sweep incomplete.** Outside `ART`, `PBK`, `FLOW`, the composition crate and the framework renderer/tool-run, there may be more `previewEval` / `flowEval*` references (e.g. the sibling `✏️s/🔌️plugins/🌊️flow` and other plugins). Re-run `/usr/bin/grep -rlI` with quoted patterns and `--exclude-dir` for `target`, `node_modules`, `dist`, `🗑️generated`, `.git`, before the wave starts.
6. **Scratch file left in the ticket folder.** `…/PROCEDURAL-3D-END-TO-END/hits-files.txt` (0 bytes) is from the stopped sweep. Remove it.
7. **Unverified details.** `cancelArgs` key names; whether `semantic-wire-check` and `canonical-architecture` enumerate flow action ids; the full list of the 93 generation3d launch entries; contents of `🧪️tests/📐️example-geometry-3d-1/🐍️.py`; whether the `🚪️io` authority scan forbids `FlowEvalSession` text.

## 10. Summary of counts

- Generation3d command rows that go: 6 editor and 6 viewer flow rows, plus `set-contributions` in both. Command files that go: 14 editor and 14 viewer flow directories (all `⏱️ ✅️ 🔺️ 🔓️ 🧯️ 🧩️ set-*` plus `flow-eval-*`).
- Command handlers that only change signature (`ctx`): about 35 (§2.3).
- Core files: `GEN/🧵️preview-eval/🦀️.rs` (≈1,475 lines, mostly DELETE), `GEN/🧵️preview-eval/⏯️tool-run/🦀️.rs` (≈680 lines, REWIRE the job), `GEN/✏️editor/🦀️.rs` (≈3,500 lines, REWIRE and DELETE islands), `GEN/👁️viewer/🦀️.rs` (≈1,700 lines).
- Tests and fixtures that go: about 8 TS and 2 Rust law files, plus `🩹️gesture-rearm`, `🪟️tick-addressing.json`, `💥️extension-evaluate-fault.json`, `🚧️contribution-gated-arming.json`, the brep-extension and flow-operators composition laws.
- Cargo: remove `semio-framework-os-flow` from `ART` (lines 20, 66, 86), `neural_engine` and both extension crates from the composition's generation3d lanes, and the three flow deps from the playbook procedural crate (lines 38-40, 45).
