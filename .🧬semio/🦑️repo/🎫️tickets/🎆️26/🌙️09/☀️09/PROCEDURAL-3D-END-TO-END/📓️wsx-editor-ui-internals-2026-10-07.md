# WSX Editor UI Internals (generation3d)

As of 2026-10-08, read-only audit of the source. No builds, tests or edits were run.

## 0. Conventions

- `G3D` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d`. Its crate root file is `G3D/🦀️.rs`, which mounts every editor module by `#[path]` (`pub mod editor` at 609, `generation3d` at 611, `commands` at 657, `modes` at 735, `panels` at 770).
- `GEN` = `G3D/🏅️standards/🔖️1/🪆️subsets/✳️any` (the subset that owns the editor).
- `EDITOR` = `GEN/✏️editor`. Its root module `EDITOR/🦀️.rs` is the file the task calls `✏️editor/🦀.rs`. It holds the manifest builder and the `ArtifactEditor` impl.
- Where a section says "mount in `G3D/🦀️.rs`", the edit goes to the crate root file above, not to a file under `GEN`.
- Line numbers refer to the files as they are on disk today.
- `FW` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (the `Editor::builder` API, about 49k lines).
- `UI` = `🧰️framework/🔨️modules/🖱️ui` (framework UI contract).
- `SEMUI` = `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs` (procedural helpers over the UI contract).

## 1. Editor builder chain (`EDITOR/🦀️.rs`)

The file is 3584 lines. The app manifest is `create_generation3d_app()` at lines 2720-3132, and it is one fluent chain that ends in `.build_definition()` at 3132. Each entry below lists its line.

### 1.1 Identity, constants, helpers (1-117)

- 8-23: imports from `commands`, `modes`, `panels`, `terminology`, `transient`, `config`, `standards`.
- 86-93: sibling modules mounted by `#[path]`: `selection` (`🎯️selection`), `transform_commands` (`🎮️commands/🧭️transforms`), `edit_mesh_selection`, `knife_mesh_selection`.
- 95 `GENERATION_3D_PLAY_APP_ID = "procedural3d-play"` (controller id of every action).
- 98 `GENERATION3D_EDITOR_APP_ID = "s.procedural.generation3d@1/*#editor"`.
- 102 `generation3d_action(action, args) -> ActionDescriptor`. Every chrome builder uses this.
- 106 `categorized_action(id, label, kind, category)`.
- 111 `migrated_command(...)` marks a command `InteractiveJobClassification::Migrated`.

### 1.2 `app_commands!` block (118-165)

This is the only dispatch surface for generation3d behaviour. It has 42 rows (grep count). Each row is `"camelId" as "kebab-keyword" => module::Type`. The row order is the binary variant ordinal, so new rows must be appended. The rows are: setActiveExample, nodeGraphEdit, deleteSelection, removeWidget, addWidget, patchFlowWidgets, reorganize, translateSelection, rotateSelection, scaleSelection, addGeneration, removeGeneration, renameGeneration, updateGenerationValues, nodeGraphViewport, setLodMode, setShowMode, toggleSun, setSunAzimuth, setSunElevation, setSunIntensity, setCamera, selectGeneration, flowEvalTick, flowEvalResolve, flowTessellateResolve, flowEvalRelease, flowTessellateCancelResolve, setContributions, importDocumentRequest, importDocument, exportDocument, cycleShowMode, cycleLodMode, selectNextNode, selectPreviousNode, selectUpstreamNode, selectDownstreamNode, activateSelection, editMeshSelection, knifeMeshSelection, setWidgetInput.

The `handle` signature each module must export is `handle(payload, doc, cfg, session: &mut FlowEvalSession) -> Result<Emit<...>, Fault>`.

### 1.3 Manifest builder chain (2720-3132)

| Line | Member | What it registers |
|---|---|---|
| 2721 | `Editor::builder(GENERATION3D_DIALECT).document(["semio","procedural","3d"])` | dialect and document kind |
| 2722-2727 | `.command(migrated_command(...))` x6 | flowEvalTick, flowEvalResolve, flowTessellateResolve, flowEvalRelease, flowTessellateCancelResolve (all `in_palette: false`, runtime), setContributions (host) |
| 2735 | `.artifact_kind(artifact_kind())` | artifact kind spec |
| 2736 | `.icon_id("workflow")` | app icon |
| 2737-2738 | `.terminology("reuse")`, `.terminology_document("reuse", [...])` | terminology mode |
| 2739-2740 | `.mode_def(edit::definition())`, `.mode_def(generate::definition())` | modes |
| 2741 | `.default_mode_id("edit")` | default mode |
| 2742 | `.mode_layout("generate", "generation3d-generate")` | layout per mode |
| 2743-2747 | `.window_kind_def(...)` x5 | flow, edit preview, generations, form, generate preview |
| 2752 | `.tool(preview_eval_tool_definition())` | `previewEval` run; must be referenced by a mode or the build fails |
| 2753-2754 | `.default_layout(edit::layout())`, `.named_layout(generate::layout())` | layouts |
| 2755-2757 | `.panel_tab_def(...)` x3 | artifact, catalogue, inspection panels |
| 2759-2778 | `.action_with(...)` for about 20 mutations | see table 1.4 |
| 2762-2764, 2766, 2776 | `.mutation(...)`, `.action_destructive(...)` | deleteSelection, removeWidget, removeGeneration |
| 2782-2783 | `.shell_action(...)` | importDocumentRequest, exportDocument |
| 2786 | `.action_with(ActionDefinition { in_palette: false, .. })` | importDocument (not in palette) |
| 2791-2818 | `.action_with` / `.view_action` for view actions | camera, LOD, show mode, cycle modes, node traversal, sun, camera, selectGeneration |
| 2819-2861 | `.action_interactive_job(...)` for every action, `.action_destructive("setActiveExample")`, `.action_destructive("exportDocument")` | execution classification |
| 2862-2928 | `.action_args(...)` for editMeshSelection (13 args), knifeMeshSelection (start/end vectors, required), addWidget, setWidgetInput, exportDocument, setActiveExample | typed arguments that drive the staged form |
| 2941-2943 | `.utility(...)` x3: move, rotate, scale (`group: "transform"`) | gumball utilities |
| 2944, 2951 | `.window_kind_utilities(...)` on both preview windows | utility rail per window |
| 2971-2999 | `.window_kind_action_refs(...)` x5 | which window owns which action (see 1.5) |
| 3015-3032 | `.interaction(graph)` | framework `graph` domain |
| 3033-3045 | `.interaction(geometry)` (`selection::DOMAIN`) | component domain |
| 3046-3048 | `.window_kind_interactions(...)` x3 | domains per window |
| 3049-3092 | `.keybinding(chord, action)` x15 | see 1.6 |
| 3093 | `.config(Generation3dPlayApp::config_spec())` | config spec |
| 3094 | `.io(generation3d_io())` | io ports: `params:in`, `geometry:out` (3165) |
| 3095-3126 | `.action_describe(...)` x32 | agent-facing descriptions, EN/DE |
| 3127-3129 | `.action_audience(...)` x3 | nodeGraphEdit=Input, nodeGraphViewport and setCamera=Chrome |
| 3130-3131 | `.action_destructive("reorganize")`, `.action_destructive("importDocument")` | |
| 3132 | `.build_definition()` | validation happens here |

Related functions in the same file:

- `examples()` at 3145-3159: nine bundled examples. `demo-session` is deliberately excluded (comment at 3134-3144).
- `generation3d_io()` at 3165.

### 1.4 Actions (`ActionDefinition`)

Kinds: M = Mutation (VCS op with inverse), V = View (no history), S = Shell (host round-trip). "Pal" means the action is in the command palette (default true unless stated). The declaration line is in the builder, and the other columns are the post-hoc attachments (see finding F3 in section 7).

| Action | Kind, category, icon | Decl | Args | Destructive | Interactive job | Describe | Keybinding |
|---|---|---|---|---|---|---|---|
| setActiveExample | M, panel-left | 2759 | 2928 | 2820 | 2819 | 3106 | none |
| editMeshSelection | M, methods | 2760 | 2862 | no | 2821 | 3104 | mod+shift+m (3049) |
| knifeMeshSelection | M, methods | 2761 | 2895 | no | 2822 | 3105 | mod+shift+k (3050) |
| nodeGraphEdit | M (`.mutation`) | 2762 | none | no | 2823 | none | none |
| deleteSelection | M (`.mutation`) | 2763 | none | 2764 | 2824 | 3120 | delete,backspace (3073) |
| removeWidget | M, targets | 2765 | none | 2766 | 2825 | none | none |
| addWidget | M, create | 2767 | 2899 | no | 2826 | 3100 | none |
| patchFlowWidgets | M, methods | 2768 | none | no | 2827 | 3121 | none |
| setWidgetInput | M, methods | 2769 | 2910 | no | 2828 | none | none |
| reorganize | M, transform | 2770 | none | 3130 | 2829 | 3102 | mod+alt+l (3057) |
| translate/rotate/scaleSelection | M, transform | 2771-2773 | none | no | 2830-2832 | 3117-3119 | none |
| addGeneration | M, create | 2774 | none | no | 2833 | 3095 | mod+shift+g (3074) |
| removeGeneration | M, targets | 2775 | none | 2776 | 2834 | none | none |
| renameGeneration | M, methods | 2777 | none | no | 2835 | 3097 | none |
| updateGenerationValues | M, methods | 2778 | none | no | 2836 | 3099 | none |
| importDocumentRequest | S | 2782 | none | no | 2858 | 3122 | mod+o (3062) |
| exportDocument | S | 2783 | 2924 | 2861 | 2860 | 3123 | mod+shift+e (3063) |
| importDocument | M, not in palette | 2786 | none | 3131 | 2859 | 3124 | none |
| nodeGraphViewport | V, camera | 2791 | none | no | 2837 | none | none |
| setLodMode | V, layers | 2792 | none | no | 2838 | 3107 | none |
| setShowMode | V (`.view_action`) | 2793 | none | no | 2839 | 3103 | none |
| cycleShowMode | V, eye | 2800 | none | no | 2840 | 3125 | mod+alt+d (3075) |
| cycleLodMode | V, layers | 2801 | none | no | 2841 | 3126 | mod+alt+k (3076) |
| selectNext/Previous/Upstream/DownstreamNode | V, arrows | 2808-2811 | none | no | 2842-2845 | 3108-3111 | arrowdown/up/left/right (3089-3092) |
| activateSelection | V, circle | 2812 | none | no | 2846 | 3112 | Enter via surface binding (see 3.8) |
| toggleSun, setSunAzimuth, setSunElevation, setSunIntensity | V, sun | 2813-2816 | none | no | 2847-2850 | 3113-3116 | none |
| setCamera | V, camera | 2817 | none | no | 2851 | none | none |
| selectGeneration | V (`.view_action`) | 2818 | none | no | 2852 | none | none |

The `.command` rows (2722-2727) are not in the palette and carry the flow evaluation chain and setContributions.

### 1.5 Window ownership (`window_kind_action_refs`, 2971-2999)

- Flow main (`procedural-main`): nodeGraphEdit, nodeGraphViewport, setLodMode, the five node traversal verbs, activateSelection.
- Edit preview (`procedural-preview`): editMeshSelection, knifeMeshSelection, setCamera, setShowMode, toggleSun, setSunAzimuth, setSunElevation, setSunIntensity, translateSelection, rotateSelection, scaleSelection.
- Generations (`generation3d-generations`): addGeneration, selectGeneration, renameGeneration, removeGeneration.
- Form (`generation3d-generate-form`): updateGenerationValues.
- Generate preview (`generation3d-generate-preview`): setCamera, setShowMode, toggleSun, sun actions, translate/rotate/scale.

Actions not listed here are app-scoped. The comment at 2943-2970 says `build_definition` copies them onto every window (`setActiveExample`, `addWidget`, `patchFlowWidgets`, `reorganize`, `removeWidget`, `deleteSelection`, history/clipboard ids).

### 1.6 Keybindings (3049-3092)

Mod means the platform modifier.

- mod+shift+m: editMeshSelection (has required `operation`, so it opens the staged form)
- mod+shift+k: knifeMeshSelection (has required `start` and `end`, so it opens the staged form)
- mod+z, mod+shift+z: framework `undo`, `redo`
- mod+alt+l: reorganize (arg-free)
- mod+o: importDocumentRequest (arg-free)
- mod+shift+e: exportDocument (requires `format`, so staged)
- delete, backspace: deleteSelection
- mod+shift+g: addGeneration
- mod+alt+d: cycleShowMode
- mod+alt+k: cycleLodMode
- arrowdown, arrowup, arrowleft, arrowright: node traversal (bare, because the window kind scopes them)

The comments at 3053-3088 say a chord reaches an arg-free action directly, and a chord for an action with args opens a staged form. The chord key token is the lowercased DOM `event.key`.

### 1.7 Utilities, interactions, measures

- Utilities (2941-2943): `move`, `rotate`, `scale`, each `group: "transform"`, icon `move`, `rotate-cw`, `maximize-2`.
- Preview windows get all three. The Rust default for the active utility is `"move"` (`EDITOR/🦀️.rs:356`).
- `graph` interaction (3015-3032): granularities node, edge, handle. Hierarchy Topology. Hover transitive. Selection Multiple or Single, Pick or Rectangle, merges Replace, Additive, Subtractive, Invertive, Range.
- `geometry` interaction (3033-3045, `selection::DOMAIN = "geometry"`): granularities object, vertex, edge, face. Hierarchy Flat. Merges Replace, Additive, Subtractive, Invertive.
- Window measures, built in `EDITOR/🦀️.rs:2642-2651`:
  - Flow window: LOD select `generation3d-measure-lod` (`🕸️flow/🦀️.rs:68-92`).
  - Both preview windows: show mode select `generation3d-measure-show` plus sun group via `world3d_sun_measures("generation3d", ...)` (`👁️preview/🦀️.rs:63-85`).

### 1.8 `ArtifactEditor` hooks (`EDITOR/🦀️.rs`, 1924-2681)

- 1926 `examples()`
- 1941-1945 `app_catalogue_json()`: publishes the operator catalogue once on the reserved `framework.section.catalogue` surface.
- 1960-2031: store owners, disposers, preparation factories. Document store is `Generation3dSnapshot`, config is `Generation3dConfig`, draft is `NoDraft`, presence is `Generation3dPresence`, transient is `Generation3dTransient`.
- 2033-2040 `register_window_transient_owners`: `Generation3dPreviewWindowTransientOwner` and `Generation3dGeneratePreviewWindowTransientOwner`.
- 2041 `tool_intent_kinds`, 2053 `fault_notices`, 2069 `retained_window_transient_target`.
- 2088-2095 `register_tool_job_factories`: four factories. Bounded command, flow eval, contributions, document IO.
- 2096-2159 `build_tool_job`: routes a tool id to one of the four ID lists.
- 2246-2459 `command_from_action(action, args)`: maps each action id plus JSON args to a `Generation3dCommand`. This is the string wire used by React and the shell.
- 2460-2479 `host_event`: every host event that ends a gumball gesture (`WindowBlurred`, `PointerCaptureLost`, `UtilityChanged`, `Retiring`, `TimeTravelFrozen`, `BaseMoved`) becomes `translateSelection{phase:"abort"}`.
- 2485-2518 `handle(command, doc, cfg, interaction, view_state, ...)`: the real dispatch for selection-aware commands. It reads `interaction.selection("geometry")` and `interaction.selection("graph")` and calls `command.dispatch` for the rest.
- 2529-2575 `interaction_topology`: every widget port becomes a `handle` target, parented to its widget. Synapses become `edge` targets.
- 2576 `pending_effects`
- 2601-2603 `render`: calls `generation3d_render_body` with no request context.
- 2608-2635 `render_with_request_context` (the one the runtime calls): resolves the preview eval text from the window transient, builds `PreviewInteractionMarks::from_interaction`, applies the open gumball preview, runs the body inside the eval session owner.
- 2637-2640 `window_engagements_with_request_context`: returns a `WindowEngagement` for the edit preview only.
- 2642-2651 `window_measures`.
- 2656-2680 `context_menu` and `context_menu_with_request_context` both call `context_menu_body` (2690-2716).

Context menu contents (`context_menu_body`): reorganize; transform trio only if there is a selection; create group (addWidget, addGeneration); targets group (removeWidget, removeGeneration) only with a selection; methods group (renameGeneration, updateGenerationValues, patchFlowWidgets); transfer group (importDocumentRequest, exportDocument); delete selection last. Component edit actions (editMeshSelection, knife) are not in the menu.

## 2. The three panels

All three are `PanelTabDefinition`s with `kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_*_ID)`. The ids are framework constants (`framework.panel.artifact`, `framework.panel.catalogue`, `framework.panel.inspection`, from `📌️mode-panel-publication.json`). Panels are framework-injected tabs, so every mode publishes all three bodies.

Common render dispatch: `generation3d_render_body` (`EDITOR/🦀️.rs:345-387`) matches the body key. The panel bodies are:

- `procedural.play.artifact` at line 373
- `procedural.play.catalogue` at line 374
- `procedural.play.inspection` at line 375-383

Each body is then wrapped by `built_to_component_tree`.

### 2.1 Catalogue (`📌️panels/🛍️catalogue/🦀️.rs`, 124 lines)

- Definition (37-45): group `PanelGroup::Workbench`, label `FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL` (EN, framework-owned) and DE "Katalog", body `procedural.play.catalogue`.
- Data source: `semio_framework_os_flow::flow_palette_catalogue_sections()`, then `owning_exports` (113-118). That function replaces the "outputs" section items with `document_io::EXPORT_FORMATS` rows. The palette and the retained spotlight share the roster through `catalogue()` (106-110).
- Render (96-103): `PanelTreeBuilder::new("procedural-play-catalogue")`, one `window_section` (`procedural-play-catalogue.widgets`), one group row per section.
- Group row (90-94): a bindingless tree item with `tree_window_item`. It is lazy and windowed: a closed group stamps its total and materialises nothing. The first group opens by default.
- Catalogue row (63-81): `tree_item_with_action_draggable`.
  - Id: `identity::item_key(kind, neuronKind, format, action)`.
  - Action: `addWidget` with the descriptor map as args.
  - Drag payload (57-61): MIME `application/x-flow-widget`, value is the descriptor JSON.
  - Icon: `emoji:` icons fall back to `box`.
  - Name: `generation3d_catalogue_name` with a format suffix for export rows.
- Identity (`🪪️identity`, 4-15): key per kind and variant, for example `procedural-play-catalogue.neuron.<kind>`.
- Inputs dispatch: a click or Enter on a row sends `addWidget`. A drag sends the same descriptor to the canvas.

### 2.2 Inspection (`📌️panels/🔍️inspection/🦀️.rs`, 512 lines)

- Definition (25-33): group `PanelGroup::Details`, EN "Inspection" (framework), DE "Inspektion", body `procedural.play.inspection`.
- Data sources (render signature, 37): `FlowHostSnapshot` (document), selected node ids (`marks.graph_selection_ids()`), labels, tree windows, locale and terminology, meshes (export readiness), `eval_json`, `status_json`. The caller at `EDITOR/🦀️.rs:375-383` takes `eval_json` from the preview eval text or the session, and `meshes` from `export_meshes_from_session` (3537), only when the selected widget is an export.
- Transient: none. It reads the document and the session.
- Structure: one section `procedural-play-inspector.widget` containing a `UiFixedList` (`fields`). Empty states: no selection, or an unknown selection.
- Per widget type:
  - InputSlider (60-96): a number `input` with commit on blur, bound to `patchFlowWidgets{field:"value", widgetIds:[id]}`. The row is a tree item whose child is the control (76-91). The range is a read-only tree item.
  - InputNote (97-102): mesh source editor, or a long text input (`InputKind::LongText`), bound to `setWidgetInput{facet, widgetId, channel}`.
  - Neuron (103-178): the neuron kind row, then one row per input port from `flow_neuron_kind_info_map()`. Connected ports show the source. Collections go to `collection_input` (retained tree windows). Point and vector ports become three number inputs. Number, text and boolean ports become a typed input, or a toggle for boolean (`Trigger::Change`). Then one reflected row per output port, from `eval_json`, with error and stale states.
  - Variable (179-189): name and schema inputs with `facet` set to `variableName` and `variableSchema`. Read-only value row.
  - OutputAction (190-194): read-only action row.
  - OutputExport (195-204): format `select` (`Trigger::Change` to `setWidgetInput{facet:"exportFormat"}`), a `button` (`Trigger::Activate` to `exportDocument`, disabled until export is ready), and loss diagnostics per format.
- Reflection helpers: `reflected_value` (268-276) and `reflected_output` (279-292, recursive, windowed).
- Mesh source editor (295-305 and onward): primitives bind their exact schema path with `setWidgetInput{facet:"meshSource", path:[...]}`.

### 2.3 Artifact (`📌️panels/🗿️artifact/🦀️.rs`, 53 lines)

- Definition (24-32): group Workbench, EN "Artifact" (framework), DE "Artefakt", body `procedural.play.artifact`.
- Render (43-48): calls `graph_outline` in `🕸️flow/🦀️.rs:192-201`. So the panel is the same flow outline the Flow window uses, as a semantic tree. Nodes are windowed rows with status description and a `HoverPreview` binding. Wires are rows labelled `src@port → dst@port`. Both sections are `window_section_or_placeholder`.
- Data: `document` (host snapshot), `config`, `session` for status JSON.

### 2.4 Inputs and how they dispatch

Every control is built with `ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action(name, args)`, which produces an `ActionDescriptor` (`EDITOR/🦀️.rs:102`). The shell sends that to `command_from_action`.

- Text and number input: `input(kind).commit("blur")`, bound with `Trigger::Commit`.
- Toggle (boolean): `toggle(...)` with `Checkbox` appearance, bound with `Trigger::Change`.
- Select: `select(...).try_item(...)`, bound with `Trigger::Change`.
- Button: `button(...)`, bound with `Trigger::Activate`.
- The value comes to the action as the scalar `value` argument (see the comment at `SEMUI` 95-101).
- Inspector edits all go through `setWidgetInput` (`editable_field`, `INSPECTION:231-242`), with `widgetId`, `channel`, optional `component`, `index` and `operation`, and `facet` where needed. Its command module is `🎮️commands/🎚️set-widget-input/🦀️.rs` (385 lines). Each committed field is one absolute `change-widget-input` leaf. An unchanged field produces no edit.

### 2.5 EN and DE strings

- The labels are in `EDITOR/🗣️terminology/🦀️.rs` (710 lines). The struct `Generation3dLabels` is declared by `semio_framework_ui_locale::app_labels!` at line 7. Each field has four spellings: `native_en`, `native_de`, `reuse_en`, `reuse_de`. The macro makes a missing locale a compile error.
- Lookup: `generation3d_labels(view_state)` (700-702) calls `resolve_labels::<Generation3dLabels>(view_state)`. The view state carries locale and terminology. Builder labels use `LocalizedLabel::native(en, de)`.
- Helpers: `generation3d_catalogue_name` (402) and `generation3d_input_name` (564) map operator and port ids to labels, falling back to the raw name.
- Shared fixture: `GEN/🧫️fixtures/🗣️terminology.json` (69 KB). It is the roster of every label, one row each, with the expected EN and DE text.
- Tests: `🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` checks that every label matches the fixture exactly in every locale (Rust). `🗣️terminology/🧪️tests/🔬️unit/🟦️.ts` is the TS twin, checking that each row is bilingual and that DE differs from EN unless declared identical. `🗣️terminology/🧪️tests/🔬️snapshot-fixture-asset/🟦️.ts` guards the naming of snapshot and fixture symbols.

## 3. Edit mode and generate mode windows

### 3.1 Modes and layouts

- Edit mode (`🎭️modes/✏️edit/🦀️.rs`): label "Edit" (EN) and "Bearbeiten" (DE), icon `pencil`, tools `[previewEval]`, no layout id. Default layout (30-32): `create_default_layout([flow MAIN, preview], "row", [68, 32], ["Flow", "Preview"])`.
- Generate mode (`🎭️modes/🧬️generate/🦀️.rs`): label "Generate" (EN) and "Generieren" (DE), icon `sparkles`, tools `[previewEval]`, layout id `generation3d-generate`. Named layout (28-48): row of `[generations 22, form 43, generate preview 35]`.

### 3.2 Window kinds

| Window id | Body key | Surface | Label EN/DE | Icon | Location |
|---|---|---|---|---|---|
| `procedural-main` | `procedural.play.main` | NodeGraph | Flow / Workflow | flow-graph | `✏️edit/🪟️windows/🕸️flow/🦀️.rs:47-65` |
| `procedural-preview` | `procedural.play.preview` | World3d | Preview / Vorschau | preview | `✏️edit/🪟️windows/👁️preview/🦀️.rs:30-48` |
| `generation3d-generations` | `procedural.play.generations` | Canvas2d | Generations / Generationen | sparkles | `🧬️generate/🪟️windows/🗂️generations/` |
| `generation3d-generate-form` | `procedural.play.generate-form` | Canvas2d | Form / Formular | clipboard-list | `🧬️generate/🪟️windows/📝️form/🦀️.rs:21-39` |
| `generation3d-generate-preview` | `procedural.play.generate-preview` | World3d | Preview / Vorschau | preview | `🧬️generate/🪟️windows/👁️preview/` |

All `🫧️transient`, `☑️options`, `🎚️config`, `🎬️actions`, `👥️presence` and `🪛️utilities` facet directories under the windows and modes hold only a `📌️.empty.md`, except for the two preview `🫧️transient` folders (see section 4).

### 3.3 Flow window render pipeline (`🕸️flow/🦀️.rs`, 355 lines)

- `render` (304-349):
  1. `with_host` gives the workflow nodes and wires (`dag_host_snapshot_to_workflow`) and the operator records (`document_operator_records`, 240-294).
  2. `flow_backed_node_graph_extras(...)` gives capabilities, LOD, host snapshot, eval, and status JSON.
  3. Hover from `marks.hovered_graph_target()`, selection from `marks.graph_selection_ids()`, highlight from `graph_highlight_ids()`.
  4. `accessible_scene_surface("procedural.play", NodeGraph, NodeGraphScene{...}, graph_canvas label, graph_canvas_hint, Liveness::Off)`.
  5. `activatable_scene_surface(surface, GENERATION_3D_PLAY_APP_ID, "activateSelection", "Enter")`.
  6. Wrapped in a grow column with id `procedural-play-main.body`.
- The outline (node and wire rows) is not in this window body. It is in the Artifact panel.

### 3.4 Edit preview window render (`👁️preview/🦀️.rs`, 114 lines)

- `render` (89-113):
  1. `preview_payload(eval_json, host_snapshot, config, session, marks)` builds `meshes_json` and `instances_json` (`EDITOR/🦀️.rs:3432`).
  2. `preview_selection_json(config, active_utility, payload)` builds the selection JSON (3364-3389).
  3. `preview_status_json` and `preview_window_status_json` (from `preview_eval`).
  4. `preview_fit_json`.
  5. `accessible_scene_surface("procedural.play.preview", World3d, world3d_scene(...) with domain, Liveness::Polite)`. The interaction domain is `geometry` when the component granularity is active, otherwise `graph` (lines 105-106).
- The eval text comes from `Generation3dPreviewWindowTransient` (section 4).

### 3.5 Generate windows

- Form (`📝️form`): `render(host_snapshot, generation, selected_id, labels)` (43-49) calls `flow_host_snapshot_to_form_spec` and `crate::generation_form(spec, values, app, "updateGenerationValues", id)` (`SEMUI` 204). Every input dispatches `updateGenerationValues`.
- Generations (`🗂️generations`): `generation_tree` (`SEMUI` 127) with a rename field (`generation_rename_field`, 102) that dispatches `renameGeneration{id}`.
- Generate preview (`👁️preview`): same World3d pipeline as the edit preview, over the same `graph` domain and `preview_selection_json`. It shares `preview_window_measures`.

### 3.6 Utilities and gumball

- Utilities are declared in `EDITOR/🦀️.rs:2941-2943` and attached to the two preview windows (2944, 2951).
- The active utility is in the framework ViewModel as `active_utility_id`. The framework injects `setActiveUtility` and `setActiveTool` (`FW` 5784-5785 and 8241-8242). The Rust default is `"move"` (`EDITOR/🦀️.rs:356`).
- `preview_selection_json` sets `transformMode = active_utility` and `gumballActive = selection non-empty AND active_utility non-empty` (3374-3375). Live dispatch is on (`gumballLiveDispatch: true`, 3376). For a component selection, `gumballTarget` is the pivot (3382-3385).
- Gumball gesture commands are `translateSelection`, `rotateSelection` and `scaleSelection`. They are retained tool routes with `phase` (`begin`, `stream`, `commit`, `abort`). `transform_commands::gumball_once` (`🧭️transforms`, line 572) runs one step. The host event abort reasons are in `EDITOR/🦀️.rs:2460-2479`.
- Front-end twin: `World3dHost` maps a `transformMode` to a gizmo kind with `gumballKindForTransformMode` (`🌐️World3dHost/🟦️.tsx:2785-2792`).

### 3.7 Selection model

Two interaction domains, both framework-owned (no app state for selection):

1. `graph` (`GENERATION_3D_INTERACTION_DOMAIN = "graph"`, `EDITOR/🦀️.rs:3238`). Ids are widget ids (node), synapse ids (edge), or port handles `{widget}@{channel}` (handle). Hover channel `pointer` (3242). Default granularity for world picks `handle` (3247).
2. `geometry` (`selection::DOMAIN = "geometry"`, `🎯️selection/🦀️.rs:5`). Granularities object, vertex, edge, face. The active granularity comes from `interaction.active_granularity(DOMAIN)`.

Component ids (`🎯️selection/🦀️.rs`):

- Format: `{widget}@{channel}#{index}.{granularity}.{component}`, optionally followed by `~{handle}~{label}~{revision}` for analytic B-Rep components (`interaction_id` 37-40; `parse` 42-59).
- `ComponentTarget::parse` accepts only granularities vertex, edge and face (line 54).
- `component_group` (62-75) requires one instance and one granularity per group.
- `validate_cached_components` (143-180) reads only current cached geometry. A pending evaluation returns "still being evaluated".
- `ComponentSelection::from_interaction` (206-212) reads the active granularity and selection of `geometry`, and the hover of the `pointer` channel.
- `ComponentSelection::project` (214-238) writes into the preview JSON: `selectionMode`, `granularity`, `gumballActive: false`, `showEdges: true`, `activeObjectId`, `ids` (instances), `componentIds`, `gumballSelectionIds`, `targets` (hard-coded modes mesh, vertex, edge, face), and `hoveredComponent`.
- `PreviewInteractionMarks` (`EDITOR/🦀️.rs:3264-3353`) is the per-render view of both domains: `hovers`, `selects`, `graph_selection_ids`, `graph_selection_domains`, `hovered_graph_target`.

### 3.8 Pick path, from pointer to command

1. World3dHost mesh handlers (`🌐️World3dHost/🟦️.tsx`, onClick around 3395-3520; face at 3404, edge at 3467, vertex at 3515) call `onWorldPick({granularity, id, merge, objectId})`.
2. `handleWorldPick` (7196-7206): if the scene has a domain, it resolves the target with `world3dComponentInteractionTarget` and dispatches `interactionSelect` with `{domainId, targets: JSON, merge, method}` (`world3dSelectionTargetsActionArgs`, 6010-6012). Otherwise it dispatches `worldPick`.
3. Framework `interactionSelect` writes the selection of the domain. Instance picks use `graph` (6955).
4. The next render reads the selection: `render_with_request_context` (`EDITOR/🦀️.rs:2608-2635`) builds `PreviewInteractionMarks` and the preview JSON.
5. A command (`editMeshSelection`, `knifeMeshSelection`, `deleteSelection`, a gumball gesture, or a traversal verb) arrives as an action. `handle` (2485-2518) reads `interaction.selection("geometry")` or `("graph")`, validates cached components (2494-2498), and then calls the module's `apply_selected`.
6. `editMeshSelection` writes one tool transaction (`mesh_edit_emit`), and knife also writes `InteractionWrite::replace(DOMAIN, "face", [])` and `replace("graph", "node", [id])` (`🥽️edit-mesh-selection` and `🔪️knife-mesh-selection` 38-41).
7. Traversal verbs (`🧭️navigate-graph`, `step_emit` at 61) write only the graph selection and never enter history.

### 3.9 Quick-action rails and entry points

- Preview window `WindowEngagement` (`selection::engagement`, `🎯️selection/🦀️.rs:241-276`). This is the only per-selection action rail:
  - `options`: four granularity toggles (object, vertex, edge, face) that dispatch `setInteractionGranularity{domainId:"geometry", granularityId}`.
  - `possible_engagements`: buttons for the active granularity. Face: Extrude, Inset, Subdivide, Flip, Delete Faces. Edge: Cut Loop, Bevel, Dissolve Edges. Vertex: Move Vertices, Dissolve Vertices, Merge at Center, Move Proportionally, Snap to Grid. Analytic (B-Rep): Fillet and Chamfer on edges, Shell on faces. Each dispatches `editMeshSelection` with fixed default args.
  - `status`: a line with the selection count and a hard-coded shortcut text.
- `editMeshSelection`: palette (category methods), keybinding mod+shift+m, and the rail. The `operation` select and the other args are rendered by the shell's staged form from `action_args`.
- `knifeMeshSelection`: palette (category methods), keybinding mod+shift+k, and the staged form for `start` and `end` (vector controls). It has no rail button. The README describes it as "Knife Cut Selected Face".
- Context menu: no component edit actions (see 1.8).

### 3.10 Accessibility hooks

- `accessible_scene_surface` (`SEMUI` 60-82) sets label, description (clipped display copy) and liveness on every canvas. Graph canvas: `Liveness::Off`. Preview canvas: `Liveness::Polite`.
- `activatable_scene_surface` (`SEMUI` 84-94) adds an `Activate` binding for Enter and `aria-keyshortcuts`. This is needed for the `role="application"` contract.
- Tree rows (`🕸️flow/🦀️.rs:159-173`): each node row sets `granularity` and a status `description`, and binds `HoverPreview`. Windowed sections state their total so the scrollbar spans the whole list.
- Keyboard: the arrow chords are bare and scoped to the flow window kind (3089-3092, comment at 3077-3088).
- Labels: `graph_canvas`, `graph_canvas_hint`, `preview_canvas`, `preview_canvas_hint` in the terminology struct.
- Fixtures: `GEN/🧫️fixtures/⌨️keyboard-reachability.json` and `🧭️graph-keyboard-navigation.json` (subset level), `📌️mode-panel-publication.json`.

## 4. Transient, config, presence, and terminology schemas

Each store kind is an `ArtifactDsl` plus `ArtifactPack` pair with a `🧬️schema/🔣️.json`. The `x-semio-state` tag in the JSON names the state class.

| Store | Rust type and location | Fields | Mutations | x-semio-state | Class (my reading of AGENTS.md) |
|---|---|---|---|---|---|
| Document | `Generation3dSnapshot` (`🧬️schema/`) | host snapshot, generation play state | `Generation3dMutation` | n/a | persisted shared (Artifact store) |
| Config | `Generation3dConfig`, `🎚️config/🦀️.rs:97-113` | lodMode, showMode, camera{x,y,zoom}, previewCamera{position,target,fov}, sunJson, selectedGenerationId | 7 leaves (`🎚️config/🧬️schema/🧬️mutations/🦀️.rs:37-54`): snapshot, lod-mode, show-mode, camera, preview-camera, sun, selected-generation | `config` (6 fields) | persisted, local to the viewer |
| Presence | `Generation3dPresence`, `👥️presence/🦀️.rs:20-29` | camera, previewCamera, showMode | `snapshot` (95-101) | `presence` (3 fields) | ephemeral, shared |
| App transient | `Generation3dTransient`, `🫧️transient/🦀️.rs:9-11` | generationPreviewText | framework-owned `set` | `ephemeral-local-app` | ephemeral, local |
| Window transient (edit and generate preview) | `Generation3dPreviewWindowTransient`, `🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs:11-13` | previewEvalText | `set-preview-eval` (45-49) | `ephemeral-local-window` | ephemeral, local, per window |

Notes:

- Selection and hover are not in any of these stores. They are the framework's `graph` and `geometry` domains (section 3.7).
- Config mutations are absolute snapshots or one-field leaves. Every lane edit is undoable through the Config history.
- Config defaults (`🎚️config/🦀️.rs:159-170`): showMode "shaded", camera (0,0,1), preview camera position (4,-4,3), target (0,0,0), fov 45.
- Ladders (`🎚️config/🦀️.rs:50-73`): show modes `shaded`, `shaded+edges`, `wireframe`, `points`. LOD modes `coarse`, `medium`, `fine`. `cycleShowMode` and `cycleLodMode` walk them, and the pickers build their rows from the same list.
- Sun is stored as raw JSON (`sunJson`) and parsed with `Generation3dConfig::sun()` (174-176).
- The editor's `🫧️transient/🧬️schema/🔣️.json` and the window transient schema are the only `ephemeral-*` schemas in the editor.
- Presence is a shareable subset: cameras and show mode. Selection and hover broadcast through the framework's `PresenceInteraction`, which the presence file comments reference.

## 5. Minimal examples

These are patterns that follow the existing code. They are not compiled, and the claims marked "to verify" were not checked.

### 5.1 (a) New panel

1. Create `EDITOR/📌️panels/🧭️<name>/🦀️.rs`:

```rust
pub const GENERATION_3D_PLAY_BODY_X: &str = "procedural.play.x";

pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App("procedural.panel.x".into()),
        label: LocalizedLabel::native("X", "X"),
        group: PanelGroup::Details,
        body_key: Some(GENERATION_3D_PLAY_BODY_X.into()),
        children: Vec::new(),
    }
}

pub fn render(labels: &Generation3dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("procedural-play-x")?.section("procedural-play-x.main", Some(crate::ui_label(labels.x.as_str())?), true, ui_node_list([])?)?.build()
}
```

2. Mount it in `G3D/🦀️.rs` inside `pub mod panels` (770-781), with `#[path]`.
3. Import it at the top of `EDITOR/🦀️.rs` (line 18) and add `.panel_tab_def(x_panel::definition())` after line 2757.
4. Add a dispatch arm in `generation3d_render_body` (`EDITOR/🦀️.rs`, before `_ =>` near line 384), passing `TreeWindows::for_body(view_state, GENERATION_3D_PLAY_BODY_X)`.
5. Add the label fields to `Generation3dLabels` (EN and DE) and to `GEN/🧫️fixtures/🗣️terminology.json`.
6. Add a row to the mode-panel fixture (`GEN/🧫️fixtures/📌️mode-panel-publication.json`) and to its TS test.
7. Taxonomy: the panel leaf name must be in `members-of-panels` (`🔣️taxonomy.json` around line 12580-12602), and the emoji must be unique among siblings.

To verify: whether the shell renders a `PanelTabKind::App` id that the framework does not define. Every existing panel uses a framework id, so this has not been exercised for an app-owned id.

### 5.2 (b) New window utility

Utilities are not declared per window. They are declared on the app and attached to windows.

```rust
// EDITOR/🦀️.rs, next to the existing .utility(...) calls at lines 2941-2943
.utility(UtilityDefinition { group: Some("analysis".into()), ..UtilityDefinition::new("measure", LocalizedLabel::native("Measure", "Messen"), "ruler") })
// replaces the whole list for the window, so restate move, rotate and scale
.window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into(), "measure".into()])
```

Also restate the same full list at line 2951 for the generate preview.

Taxonomy: a utility has no folder. Its code belongs in the window's `🪛️utilities/` facet folder, which already exists with `📌️.empty.md` in both previews.

Important: `measure` will be treated as a transform by the gumball (finding F1). The selection JSON must not set `gumballActive` for a non-transform utility.

### 5.3 (c) New action with keybinding

```rust
// EDITOR/🦀️.rs, after the action_with calls in the manifest
.action_with(ActionDefinition::new("measureSelection", LocalizedLabel::native("Measure Selection", "Auswahl messen"), ActionKind::View, "ruler"))
.action_interactive_job("measureSelection", InteractiveJobClassification::Migrated)
.action_describe("measureSelection", LocalizedLabel::native("Measures the selected components and reports the result.", "Misst die ausgewählten Komponenten und zeigt das Ergebnis."))
.keybinding("mod+alt+m", "measureSelection") // not bound elsewhere in this file as of the audit
.window_kind_action_refs(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![/* the existing 11 refs, plus */ "measureSelection".into()])
```

Order matters: `action_with` must come before `action_interactive_job`, `action_describe` and `action_destructive`, because those calls silently do nothing for an undeclared id (finding F3).

If the action carries arguments, declare them with `.action_args(...)` after `action_with`. An arg-free chord fires straight through. A chord for an action with required args opens the staged form.

### 5.4 (d) New command

1. Module `EDITOR/🎮️commands/📏️measure-selection/🦀️.rs` (the emoji must be unique among the commands folder):

```rust
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "measure-selection")]
pub struct MeasureSelection { pub mode: String }

pub fn handle(payload: &MeasureSelection, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let _ = (payload, doc);
    Ok(Emit::default())
}
```

`Emit` has a `Default` impl (`FW` 13503). `Emit::config(vec![...])` is the pattern for a config-only view action (`🧭️set-sun-azimuth/🦀️.rs:19`).

2. Mount it in `G3D/🦀️.rs` inside `pub mod commands` (657-734).
3. Add a row to `app_commands!` (`EDITOR/🦀️.rs:118-165`), appended at the end. This sets the variant ordinal.
4. Add an arm to `command_from_action` (`EDITOR/🦀️.rs:2246`).
5. Add the tool id to `GENERATION3D_RETAINED_TOOL_IDS` (`395-431`) and a row to the bounded factory's `PUBLICATION_CONTRACTS` (`1050-1097`), with lanes. A view action that edits nothing uses only `Interaction`.
6. Add the action and its classification, description and keybinding (section 5.3).
7. If it reads selection, add a branch in `handle` (`EDITOR/🦀️.rs:2485-2518`).
8. Taxonomy: the leaf name must be in `members-of-commands` (`🔣️taxonomy.json` around line 9759). As of this audit, 14 of 40 command folder names do not appear verbatim in that file (see F8).

### 5.5 Taxonomy requirements at a glance

- Surface and facet names: a window needs `🪟️windows/<window>/` with its facet folders (`☑️options`, `🎚️config`, `🎬️actions`, `👥️presence`, `🪛️utilities`, `🫧️transient`). Empty facets carry `📌️.empty.md` (rule `artifact-empty-facet-primary-markdown-v1`, around line 16688). Facet names are at lines 16610-16660, and owner path patterns at 16665-16671.
- Surface child folders: `🎭️modes`, `🎮️commands`, `📌️panels`, `🎚️config`, `👥️presence`, `🫧️transient`, `🗣️terminology`, `🌉️wasm`, `📚️examples` (`🔣️taxonomy.json` around 28636-28647). Required children are modes, commands, config, presence and transient (28647-28655).
- Registries (`source: registry`): `members-of-commands` (9759), `members-of-windows` (14379), `members-of-modes` (12354), `members-of-options` (9736), `members-of-actions` (9500), `members-of-panels` (12580-12602). A leaf name must be in its registry.
- Schema first: any new config, presence or transient field needs a `🧬️schema/🔣️.json` entry with an `x-semio-state` tag, and the matching mutation leaf.

## 6. How TS and React render this

- The UI is guest-described. The Rust side builds a `BuiltNode` tree per body, and `built_to_component_tree` turns it into the framework `Component` union. That enum is at `UI/🧬️contract/🧩️component/🦀️.rs:1391-1412` and has 21 variants: Container, Text, Button, Separator, Input, Select, Toggle, KeyValueList, Slider, NumberStepper, Ring, IconSelect, Progress, Tree, TreeSection, TreeItem, Image, Surface, Extension, Table, TableRow.
- The React interpreter is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`. `renderComponent` (3219-3263) switches on the variant, and `case "surface"` (3258) delegates to `SurfaceView`.
- Tree items: `collectTreeItems` (1825) and `collectTreeItemControls` (1839). Controls that are children of a tree row are mounted inline (see the comment at `INSPECTION:76-84`).
- Surfaces: `renderComponentSceneHost` (603) and `resolveComponentSceneHost` (404-412) map `canvas-2d` to `Canvas2dHost`, `world-3d` to `World3dHost`, and `node-graph` to `NodeGraphHost`.
- There are no `.tsx` files under the procedural plugin (checked: zero). The procedural TS files (`GEN/**/🟦️.ts`) are typed view-model twins, pure validation twins (for example knife and component parsing in `🎯️selection/🟦️.ts` and `🎮️commands/🔪️knife-mesh-selection/🟦️.ts`), and tests. None of them render.
- The procedural package (`📦️packages/🟦️typescript/package.json`) is a test harness with no UI.
- Semantic helpers for the Rust side are in `SEMUI`.

## 7. Findings for lanes U1 to U4

These are verified in the source. Each one names the line.

- F1. Gumball appears for any selection, and it appears for any utility that is not a transform. `render` defaults the utility to "move" (`EDITOR/🦀️.rs:356`). `preview_selection_json` sets `gumballActive` from "selection non-empty and utility non-empty" (3375). World3dHost maps any unknown `transformMode` to "translate" (`gumballKindForTransformMode`, 2785-2792). A U3 measure utility will therefore show translate handles unless `preview_selection_json` gates `gumballActive` on the three transform ids.
- F2. `window_kind_utilities`, `window_kind_action_refs` and `window_kind_interactions` replace the whole list for the window (for example `FW` 5352-5357 sets `window.utilities = utility_ids`). Adding one entry means restating the full list.
- F3. The post-hoc builder calls silently do nothing when the id is not yet declared: `action_args` (`FW` 5435), `action_describe` (5447), `action_destructive` (5511), `action_interactive_job` (5537). The doc comment of `action_args` says the args are dropped. The others only mutate existing entries. Declare with `action_with` first.
- F4. The preview context menu has no component edit actions (`EDITOR/🦀️.rs:2690-2716`). The only per-selection rail is the preview `WindowEngagement` (`🎯️selection/🦀️.rs:241-276`). `WindowEngagement` also has `control` and `controls` fields, and the code leaves them `None`.
- F5. The engagement status text hard-codes the shortcut as literal text (`🎯️selection/🦀️.rs:272`, "Ctrl/⌘+Umschalt+M" and "Ctrl/⌘+Shift+M"). If the keybinding is customised, this text goes stale.
- F6. Edit defaults are duplicated in three places: the builder `action_args` defaults (`EDITOR/🦀️.rs:2862-2893`), the engagement JSON (`🎯️selection/🦀️.rs:266`), and the Rust `Default` of `EditMeshSelection` (`🥽️edit-mesh-selection/🦀️.rs:36`). A U2 inspector must keep all three in step.
- F7. The doc comment on `GENERATION3D_RETAINED_TOOL_IDS` says "34 rows" (line 392). The `app_commands!` block has 42 rows. The comment is stale.
- F8. 14 of 40 command folder names are not present verbatim in `🔣️taxonomy.json`. They include `🥽️edit-mesh-selection`, `🔪️knife-mesh-selection`, `🎚️set-widget-input`, `🎨️set-active-example`, `📤️export-document`, `📥️import-document`, `🧭️transforms`, `🧬️generation`, `🧯️flow-tessellate-cancel-resolve`. This is a string search only. Whether the taxonomy gate rejects them was not checked.
- F9. Component picks are limited to granularities vertex, edge and face in Rust (`🎯️selection/🦀️.rs:54`), in the TS twin, and in the World3dHost `targets` map (`🎯️selection/🦀️.rs:229` hard-codes mesh, vertex, edge, face). A new analysis target type needs changes in all three.
- F10. The preview chooses its interaction domain by granularity (`👁️preview/🦀️.rs:105-106`). So any U4 contextual action must work under `geometry` (components) or `graph` (instances).
- F11. The keybinding comment (3053-3088) says a chord for an action with arguments opens a staged form. That applies to `editMeshSelection` (mod+shift+m) and to `knifeMeshSelection` (mod+shift+k). Not verified in the shell code.
- F12. `ComponentSelection::project` hard-codes `gumballActive: false` for components (`🎯️selection/🦀️.rs:222`) and `preview_selection_json` sets it back to `!active_utility.is_empty()` when a pivot exists (3383-3384). Gumball for components therefore depends on the utility id, the same as F1.

## 8. Not checked

- The shell's staged form behaviour and `ShellHost` keybinding loop (cited only from comments).
- Whether the framework accepts an app-owned `PanelTabKind::App` id at runtime.
- Whether the taxonomy gate enforces F8.
- Contents of the two plan documents named in the task (`brep-mesh-widget-set-2026-10-07.md`, `wsx-ui-viewport-2026-10-07.md`). They were not read.
- The viewer surface (`👁️viewer`) and the generation3d `🌉️wasm` glue, which have no effect on the editor UI.
- No tests were run.
