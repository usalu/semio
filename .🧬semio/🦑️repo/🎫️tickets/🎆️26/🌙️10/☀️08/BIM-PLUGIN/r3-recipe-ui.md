# R3 Recipe: BIM Editor and Viewer UI (mirror of the Shooting subset)

Scope: implementation recipe for the BIM model editor (`✏️editor`) and viewer (`👁️viewer`), derived from r2-design §6 and the compliant shooting template. Read-only research: no tests were run and no generators were executed.

Paths below are relative to:
- `S` = `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `HUB` = `🌎️hub/🧩️compositions/🎥️shooting`

Code blocks are verbatim copies. `…` marks an elision.

---

## 0. Template map (shooting subset)

| Concern | File (relative to S unless noted) |
|---|---|
| Plugin root (apps, editor/viewer registration) | `HUB/🦀️.rs` |
| Artifact root (dialect, kind, declaration, mount tree) | `🗿️artifacts` root `🦀️.rs` (`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs`) |
| Editor app, command enum, job factory, manifest | `✏️editor/🦀️.rs` |
| Edit mode | `✏️editor/🎭️modes/✏️edit/🦀️.rs` |
| Scene window (World3d) | `✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️.rs` |
| Icon window (IconRender) | `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️.rs` |
| Window measures | `…/🪟️windows/🎥️scene/☑️options/*/🦀️.rs` |
| Command payloads | `✏️editor/🎮️commands/*/🦀️.rs` |
| Config (session state) + schema leaves | `✏️editor/🎚️config/🦀️.rs`, `…/🎚️config/🧬️schema/` |
| Presence + schema leaves | `✏️editor/👥️presence/🦀️.rs`, `…/👥️presence/🧬️schema/` |
| Terminology | `✏️editor/🗣️terminology/🦀️.rs` |
| Panels | `✏️editor/📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue}/🦀️.rs` |
| Viewer app | `👁️viewer/🦀️.rs` |
| Viewer mode and scene window | `👁️viewer/🎭️modes/👁️view/🦀️.rs`, `…/🪟️windows/🎥️scene/🦀️.rs` |
| Editor/viewer tests | `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `…/🧪️tests/🔬️window-action-contract/🦀️.rs`, `👁️viewer/🧪️tests/🔬️unit/🦀️.rs`, `HUB/🧪️tests/🔬️surface/🦀️.rs` |
| Fixtures | `🧫️fixtures/🔣️window-actions.json`, `🧫️fixtures/🧫️retained-command-limits/🔣️.json` |

The shooting `✏️editor/🦀️.rs` (about 1170 lines) is a routing table. Each taxonomy node contributes its own `definition()` or `render()`.

Framework references (paths relative to repo root):
- Plugin SDK: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- Operation progress: `…/🔌️plugin/⏳️operation-progress/🦀️.rs`
- Scene types (`World3dScene`, `Canvas2dScene`, `TableScene`): `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
- Scene TS twin: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
- Mesh data: `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs` (`pub struct MeshData`)
- Inference: `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs`
- React World3d host: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- React Canvas2d host: `…/🧱️elements/📐️Canvas2dHost/🟦️.tsx`, paint types `…/📐️Canvas2dHost/🎨️paint/🟦️.ts`
- Schema generator: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts`

---

## 1. Plugin wiring

### 1.1 Plugin root: `HUB/🦀️.rs`

```rust
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the shooting editor and viewer surfaces.
    pub enum ShootingApps: PluginApp {
        ShootingEditor(VcsArtifactApp<EditorApp<crate::editor::shooting::ShootingPlayApp>>),
        ShootingViewer(VcsArtifactApp<ViewerApp<crate::viewer::shooting::ShootingViewer>>),
    }
}

pub fn plugin() -> Result<Plugin<ShootingApps>, PluginAssemblyError> {
    Plugin::<ShootingApps>::builder("shooting")
        .label("Shooting")
        .version("0.1.0")
        .package_id("semio:shooting")
        .artifact(crate::artifacts::shooting::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<crate::editor::shooting::ShootingPlayApp>(crate::editor::shooting::create_shooting_app())
        .editor_mutation_roster::<crate::editor::shooting::ShootingPlayApp>()
        .viewer::<crate::viewer::shooting::ShootingViewer>(crate::viewer::shooting::create_shooting_viewer())
        .viewer_mutation_roster::<crate::viewer::shooting::ShootingViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::shooting::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist shooting edits to the open document".into(), optional: false })
        .try_build()
}
```

Recipe: the BIM hub composition copies this file with `bim` names. `.editor::<…>(create_*_app())` and `.viewer::<…>(create_*_viewer())` are the only registration calls for the two surfaces. The `ShootingApps` enum must list both variants.

### 1.2 Artifact root mount tree (`🗿️artifacts/🎥️shooting/🦀️.rs`)

- `SHOOTING_DIALECT = Dialect { artifact_kind: "s.shooting.shooting", standard: StandardId("1"), subset: SubsetId::ANY }` (line ~31). The canonical surface ids are `s.shooting.shooting@1/*#editor` and `…#viewer`.
- `artifact_kind()` returns `ArtifactKindSpec` (`id: "2d.shooting"`, `schema: SHOOTING_DOCUMENT_SCHEMA`).
- `declaration()` (line ~168):

```rust
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::shooting_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::shooting_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<editor::shooting::ShootingPlayApp>>()
        .try_build()
}
```

- The editor is mounted with `#[path = "."] pub mod editor { pub mod shooting { … } }` (line ~1344). Each taxonomy node is a `#[path]` module. Example:

```rust
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        …
```

Recipe: BIM mounts `editor::bim`, `viewer::bim`, and one `#[path]` per node (`🎭️modes/✏️edit/🪟️windows/🗺️plan`, `🧊️world`, `📐️section`, `📌️panels/*`, `🎮️commands/*`, `🎚️config`, `👥️presence`, `🗣️terminology`).

### 1.3 Subset-level alternative (not used by shooting)

The sourcing subset registers via `SubsetDeclaration` and `editor_surface::<App, A>(…)` / `viewer_surface::<Viewer, A>(…)` (`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`). Shooting does not use this path. Use the shooting `Plugin::builder` path for BIM.

---

## 2. Editor app: struct, trait, manifest

### 2.1 App struct and dispatch context (`✏️editor/🦀️.rs`)

```rust
#[derive(Default)]
pub struct ShootingPlayApp;
```

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShootingDispatchCtx {
    pub selected_asset_ids: Vec<String>,
}
```

`ShootingDispatchCtx` carries per-dispatch state that is neither document nor config. The app reads the `"assets"` interaction-domain selection once in `handle` and passes it down.

### 2.2 `ArtifactEditor` impl (line ~630)

Associated items, verbatim:

```rust
impl ArtifactEditor for ShootingPlayApp {
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![
        crate::examples::demo::source(),
        crate::examples::hexagonal_cut_concrete_forest_left::source(),
    ]
    }
    type Snapshot = ShootingSnapshot;
    type Mutation = ShootingMutation;
    type Config = ShootingConfig;
    type ConfigMutation = ShootingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ShootingPresence;
    type PresenceMutation = ShootingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = ShootingCommand;
    const DIALECT: Dialect = crate::SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;
```

Methods used by shooting (signatures from the framework trait at `🔌️plugin/🦀️.rs` ~39088–39823):
- `fn initial_snapshot() -> Self::Snapshot`
- `fn handle(command, doc: &ArtifactView<Snapshot>, cfg: &ConfigView<Config>, interaction: &InteractionView, view_state: Option<&ViewModel>, draft, engines: &EngineHandles) -> Result<Emit<Mutation, ConfigMutation, DraftMutation>, Fault>`
- `fn render(body_key, doc, cfg, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree>`
- `fn window_engagements(doc, cfg, view_state) -> HashMap<String, WindowEngagement>`
- `fn window_measures(doc, cfg, view_state) -> HashMap<String, Vec<WindowMeasure>>`
- `fn command_id(&Command) -> &'static str`, `fn command_from_action(action, args: Option<&DslValue>) -> Result<Command, Fault>`
- `fn interaction_topology(doc, cfg) -> Result<InteractionTopology, ValueError>`
- `fn config_spec() -> ConfigSpec`, `fn io() -> Option<AppIo>`, `fn app_schema() -> Option<AppSchemaDescriptor>`
- `fn export_media(port, doc) -> Result<Media, MediaError>`, `fn import_media(port, media, doc) -> ArtifactMutationOutcome<…>`
- `fn register_tool_job_factories(registry)`, `fn build_tool_job(request)`, `fn bounded_first_step_tool_proofs()` (macro-generated, see §4)
- Store owners and disposers: `build_document_store_owners`, `build_config_store_owners`, `build_draft_store_owners`, `build_artifact_store_one_item_preparation_factory`, `build_config_store_one_item_preparation_factory`, `build_document_store_disposer`, `build_config_store_disposer`, `build_draft_store_disposer`, `build_transient_store_disposer`, `build_transient_local_root_retirement_factory`, `build_presence_local_root_retirement_factory`, `build_presence_peer_retirement_factory`, `build_presence_store_disposer`. Shooting uses the `bounded_*` framework helpers for all of these. Presence uses an explicit empty-terminal disposer:

```rust
fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
    Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default shooting presence is the exact empty terminal")))
}
```

Recipe: copy this block with `BimPresence::default()`. Use the `bounded_*` helpers for document, config and transient stores. Use `no_draft_store_owners()` and `no_draft_store_disposer()` if the editor has no draft.

### 2.3 `interaction_topology` (flat `assets` domain)

```rust
fn interaction_topology(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
    …
        let ordered = doc.snapshot.assets.iter().map(|asset| semio_framework_plugin::TopologyNode { id: asset.id.clone(), granularity: "asset".into(), parent: None }).collect();
        semio_framework_plugin::InteractionTopology { domains: std::collections::BTreeMap::from([(SHOOTING_INTERACTION_DOMAIN.into(), semio_framework_plugin::DomainTopology { ordered })]) }
```

Without this, the framework drops every pick (per the doc comment). BIM: one domain per selectable granularity, for example `elements` with granularities `wall`, `slab`, `column` and a hierarchy provider if needed.

### 2.4 `handle`

```rust
fn handle(
    command: &ShootingCommand,
    doc: &ArtifactView<'_, ShootingSnapshot>,
    cfg: &ConfigView<'_, ShootingConfig>,
    interaction: &InteractionView<'_>,
    _view_state: Option<&semio_framework_plugin::ViewModel>,
    _draft: &DraftView<'_, Self::Draft>,
    _engines: &EngineHandles,
) -> Result<Emit<ShootingMutation, ShootingConfigMutation, Self::DraftMutation>, Fault> {
    let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection(SHOOTING_INTERACTION_DOMAIN).ids.clone() };
    command.dispatch(doc, cfg, &mut ctx)
}
```

### 2.5 `render` and `window_engagements`

```rust
fn render(body_key: &str, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let snapshot = doc.snapshot;
    let labels = shooting_play_labels(view_state);
    match body_key {
        SHOOTING_PLAY_BODY_SCENE => scene_window::render(snapshot, cfg.snapshot, view_state.active_utility_id.as_deref().unwrap_or("move")),
        SHOOTING_PLAY_BODY_ICON => icon_window::render(snapshot, cfg.snapshot),
        SHOOTING_PLAY_BODY_ARTIFACT => document_panel::render(snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_ARTIFACT)),
        SHOOTING_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_CATALOGUE)),
        SHOOTING_PLAY_BODY_INSPECTION => inspection_panel::render(snapshot, cfg.snapshot, labels),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
    }
    .map(semio_framework_plugin::built_to_component_tree)
}

fn window_engagements(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
    let labels = shooting_play_labels(view_state);
    HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::engagement(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::engagement(doc.snapshot, labels))])
}
```

Body keys are constants next to their node (`SHOOTING_PLAY_BODY_SCENE = "shooting.play.scene"`). Shooting `render` never matches unknown keys; it returns a labelled text node.

### 2.6 Manifest: `create_shooting_app()` (line ~947)

The manifest is built by one `Editor::builder` chain. Verbatim head and representative rows:

```rust
pub fn create_shooting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::SHOOTING_DIALECT)
            .document(["semio", "shooting"])
            .artifact_kind(crate::artifact_kind())
            .mode_def(edit::definition())
            .default_mode_id(edit::SHOOTING_PLAY_MODE_EDIT)
            .window_kind_def(scene_window::definition())
            .window_kind_def(icon_window::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("setActiveShot", LocalizedLabel::native("Set Active Shot", "Aktive Aufnahme festlegen"))
            .view_action("setShotSelection", LocalizedLabel::native("Set Shot Selection", "Aufnahmeauswahl festlegen"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .interaction(InteractionDefinition {
                id: SHOOTING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Assets", "Objekte"),
                granularities: vec![GranularityDefinition { id: "asset".into(), label: LocalizedLabel::native("Asset", "Objekt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(SHOOTING_PLAY_WINDOW_SCENE, vec![InteractionRef::new(SHOOTING_INTERACTION_DOMAIN)])
            .shell_action("exportActiveShot", LocalizedLabel::native("Export Active Shot", "Aktive Aufnahme exportieren"))
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_destructive("resetSnapshot")
            .action_args("addShot", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")), ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG"))]).default_value(&"png"),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .window_kind_utilities(SHOOTING_PLAY_WINDOW_SCENE, vec!["move".into(), "rotate".into(), "scale".into()])
            .config(ShootingPlayApp::config_spec())
            .io(shooting_io())
            .action_describe("setActiveShot", LocalizedLabel::native("Makes the shot with the given id the active one that shot edits and Export Active Shot apply to.", "Macht die Aufnahme mit der angegebenen Id zur aktiven, auf die Aufnahmeänderungen und Aktive Aufnahme exportieren wirken."))
            .build_definition()
}
```

Builder methods available to both `EditorBuilder` and `ViewerBuilder` (from `surface_builder_forward!` at framework ~41337): `artifact_kind`, `config`, `io`, `command_grammar`, `media_input`, `media_output`, `terminology`, `introduction`, `tutorial`, `dialog`, `icon_id`, `mode`, `mode_commands`, `mode_command`, `mode_layout`, `mode_tools`, `default_mode_id`, `window_kind`, `window_kind_with_engagement`, `mode_def`, `window_kind_def`, `panel_tab_def`, `window_kind_measures`, `window_kind_actions`, `window_kind_action_refs`, `window_kind_utilities`, `window_kind_initial_utility`, `window_kind_interactions`, `named_layout`, `default_layout`, `panel_tab`, `panel_tab_tree`, `panel_tab_framework`, `keybinding`, `view_action`, `shell_action`, `action_with`, `action_args`, `action_describe`, `action_use_when`, `action_audience`, `action_destructive`, `action_interactive_job`, `interactive_jobs`, `command`, `app_command`, `command_args`, `utility`, `interaction`, `utility_simple`, `tool`, `tool_simple`. `Editor` alone adds `mutation(id, label)`.

Shooting declares no keybindings. Use the drawing editor for keybindings (§7.2).

---

## 3. Commands

### 3.1 The `app_commands!` table (`✏️editor/🦀️.rs` line ~251)

```rust
semio_framework_plugin::app_commands! {
    pub enum ShootingCommand for ShootingSnapshot, ShootingMutation, ShootingConfig, ShootingConfigMutation, ctx = ShootingDispatchCtx {
        "importSnapshotJson" as "import-snapshot-json" => import_snapshot_json::ImportSnapshotJson,
        "setActiveShot" as "active-shot" => set_active_shot::SetActiveShot,
        "setShotCamera" as "shot-camera" => set_shot_camera::SetShotCamera,
        "patchShots" as "patch-shots" => patch_shots::PatchShots,
        "addShot" as "add-shot" => add_shot::AddShot,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "setCamera" as "camera" => set_camera::SetCamera,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "exportActiveShot" as "export-active-shot" => export_active_shot::ExportActiveShot,
        …
    }
}
```

Each row gives the manifest action id (camelCase), the DSL keyword (kebab-case), and the payload type. Appending rows is safe. Reordering breaks the binary wire format.

Recipe for BIM: one row per verb (`createWall`, `createWallArc`, `createColumn`, `createBeam`, `createSlab`, `createRoof`, `createWindow`, `createDoor`, `createOpening`, `createStair`, `createRailing`, `createSpace`, `createGrid`, `moveElements`, `setWallAxis`, `setActiveStorey`, `setCamera`, `setPlanViewport`, `setSectionLine`, and so on).

### 3.2 Action bridge: host args to typed payloads

The host sends camelCase keys. The payloads use snake_case fields. `args_bridge::command_from_action` (line ~310) folds keys, applies aliases and defaults, and decodes:

```rust
fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue { … }

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<ShootingCommand, Fault> {
    const IDS: &[(&str, &str)] = &[("ids", "asset_ids"), ("asset_id", "asset_ids")];
    …
    Ok(match action {
        "setActiveShot" => ShootingCommand::SetActiveShot(decode(action, fold(args, &[("value", "shot_id"), ("id", "shot_id")], &[]))?),
        "translateSelection" => ShootingCommand::TranslateSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("dx", zero()), ("dy", zero()), ("dz", zero())]))?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the shooting editor has no command for action '{action}'"))),
    })
}
```

The bridge also converts whole-number floats to `UInt`/`Int` (`integral`) and stringifies host control values (`stringify`) for patch verbs.

### 3.3 Command handler signature and emission

Each `🎮️commands/*` module exposes `pub mod <verb> { pub struct …; pub fn handle(…) }`:

```rust
pub fn handle(payload: &PatchShots, _doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
```

Document edit (`📷️shot/🦀️.rs`):

```rust
Some(shot_id) => Ok(Emit::mutations(vec![ShootingMutation::RenameShot(RenameShot { id: shot_id, new_label: payload.value.clone() })])),
```

Config-only edit (`🎮️commands/☀️scene`/`🗂️selection`): `Ok(Emit::config(vec![ShootingConfigMutation::SetShotSelection(…)]))`. Viewport camera and selection are config. Document edits are `Emit::mutations`.

`Emit` fields (framework ~13455): `artifact_mutations`, `config_mutations`, `window_config_mutations`, `draft_mutations`, `transaction`, `transaction_phase`, plus effects. Constructors: `Emit::mutations`, `Emit::config`, `Emit::effect(Effect)`, `Emit::default()`.

Effects used by shooting: `Effect::LoadDocument { pack, spr }` (`reset_document_effect`, line ~936) for whole-document replace outside undo history.

Recipe: BIM uses `Emit::mutations` for each create/move. Config for plan viewport, storey and section line. Shell effects for export and import.

### 3.4 Command unit tests

```rust
#[semio_framework_async_macros::async_test]
async fn set_active_shot_updates_fixture() {
    let mut app = shooting_app().await;
    let second_id = app.snapshot().expect("snapshot").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
    dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) })).await;
    assert_eq!(app.snapshot().expect("snapshot").active_shot_id, second_id);
}

#[semio_framework_async_macros::async_test]
async fn set_shot_selection_is_config_only_and_selects_the_shot_in_the_inspector() {
    let mut app = shooting_app().await;
    let shot_id = app.snapshot().expect("snapshot").shots.first().expect("fixture shot").id.clone();
    let result = dispatch(&mut app, ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec![shot_id] })).await;
    assert!(!result.edited_document(), "shot selection is config-only");
    assert!(render(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await.contains("shooting-play-inspector.shot"), "…");
}
```

The test harness (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `mod context`):
- `ShootingApp` wraps `VcsArtifactApp<EditorApp<ShootingPlayApp>>` with `Deref`, and closes the fixture on `Drop`.
- `shooting_app()` uses `new_app_with_registry::<EditorApp<ShootingPlayApp>>(shooting_app_manifest_for_tests, …)`, then `bind_instance_id(SHOOTING_TEST_INSTANCE)`. The registry is required so bounded tool proofs are admitted.
- `dispatch(app, command) -> Dispatched` calls `dispatch_typed`, settles the retained operation, and applies any `LoadDocument`. `Dispatched::edited_document()` checks the artifact lane.
- `dispatch_rows` returns the applied history rows, for asserting exactly one row per gesture.
- `history_verb(app, "undo"|"redo")`, `render(app, body_key)`, `world_scene(app)`, `icon_scene(app)`, `view(locale)`, `scene_window_measures`.
- `semio_framework_plugin::history_edit_acceptance_law!("shooting", ShootingPlayApp, shooting_app_manifest_for_tests, "../..");` gives the history acceptance law.
- Laws: `artifact_app_laws::assert_undo_redo_round_trip(&mut app, command, |app| …, before, after)`, `assert_two_registered_instances_converge`, `assert_registered_ingest_idempotent`.
- Language-neutral fixtures: `include_str!("../../🧫️fixtures/🔣️window-actions.json")` (window action oracle), `include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json")` (retained catalog oracle).
- Round-trip: `every_command_round_trips_through_text_and_binary` calls `store::os_store::test_support::assert_op_text_binary_equivalence(&command)` for every row.
- Command-id uniqueness and bridge tests: `command_from_action_round_trips_every_command_id`, `command_from_action_bridges_host_control_contracts`.

Recipe: BIM needs one `every_command()` roster in `unit_tests`, the same text/binary round trip, and a `dispatch`-then-assert test for each verb. `dispatch_rows` counts history rows for the gesture law.

---

## 4. Retained interactive jobs (gesture factories)

### 4.1 Bounded first-step verbs (shooting, every verb)

```rust
const SHOOTING_BOUNDED_TOOL_IDS: &[&str] = &["importSnapshotJson", "setActiveExample", …];
const SHOOTING_RETAINED_PAYLOAD_SCHEMA: &str = "shooting.shooting.tool-command.v1";
const SHOOTING_BOUNDED_RAW_BYTES: usize = 65_536;
const SHOOTING_BOUNDED_WORK_ITEMS: usize = 1;

fn shooting_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SHOOTING_BOUNDED_RAW_BYTES, 64, SHOOTING_BOUNDED_WORK_ITEMS as u64, 262_144, 7_500)
}

struct ShootingCommandJobFactory { keys: Vec<ToolFactoryKey> }

impl semio_framework::ToolJobFactory for ShootingCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ShootingPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ShootingPlayApp>>;
    fn payload_schema_id(&self) -> &str { SHOOTING_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { shooting_bounded_contract() }
    fn create_job(&mut self, _operation, payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ShootingCommandJobFactory {
    type Owner = EditorApp<ShootingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = SHOOTING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActiveShot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        …
    ];
}
```

The reducer runs the same `ShootingCommand::dispatch` on the retained snapshot:

```rust
fn shooting_bounded_reduce(command, snapshot, config, history, interaction, _hover, _context, operation) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {
    …
    let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection.get(SHOOTING_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default() };
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None }, &mut ctx)
}
```

`build_tool_job` builds `BoundedArtifactCommandWork::new(tool_id, shooting_bounded_reduce, shooting_bounded_extent)` and wraps it in `ArtifactRetainedCommandPayload::try_new(…)`. `register_tool_job_factories` calls `registry.register(ShootingCommandJobFactory::new(&controller))`.

Proofs (from the `✏️editor` impl):

```rust
semio_framework_plugin::bounded_first_step_tool_proofs! {
    owner: EditorApp<ShootingPlayApp>,
    owner_file: "…/✏️editor/🦀️.rs",
    controller: "s.shooting.shooting@1/*#editor",
    artifact_schema: "shooting.shooting",
    factory: "ShootingCommandJobFactory",
    factory_type: ShootingCommandJobFactory,
    tools: { "importSnapshotJson" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500), … }
}
```

Every verb must appear in `tools:` and in the manifest's `action_interactive_job(…, Migrated)`. The UI refuses dispatch of anything not `Migrated`.

### 4.2 Resumable gesture route (drawing, for pointer gestures)

Drawing's pointer gestures use a resumable contract, which is the pattern BIM wall/slab drawing needs. The live file is under `✏️editor/🪆️1-any/…` (see §10, finding F3):

```rust
const DRAWING_GESTURE_TOOL_IDS: &[&str] = &["canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDoubleClick", "canvasCommitDraft", "canvasEscape"];

impl semio::ToolJobFactory for DrawingGestureOperationJobFactory {
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(DRAWING_GESTURE_RAW_BYTES, 32, 1, 16_384, 7_500, 1, 1)
    }
    fn create_job(&mut self, _operation, payload) -> Result<Self::Job, …> {
        Ok(DrawingGestureOperationJob { payload: Some(payload), … })
    }
}
```

The gesture session holds state between samples. Its fields include `base: Option<DrawingToolBase { document, operation }>`, `tool`, `window_config`, `window_transient`, and `active_utility_id`. The payload is `CanvasPointerDown { x, y, width, height, shift, alt, ctrl, meta, app_instance_id, operation_id, generation, base_revision, world_x, world_y, checkpoint_* }`.

Recipe: BIM wall/beam/slab drawing, and any drag that spans many samples, uses `resumable` with a `DrawingGestureOperationJob`-style session. Commands with no per-sample state use bounded first-step.

### 4.3 Gumball as a tool machine (shooting)

Gumball gestures run through `ToolMachineRunner` with one relative leaf per gesture:

```rust
pub const SHOOTING_GUMBALL_TOOL_KEY: &str = "assets:0";

fn yield_gesture(_context: &mut GumballToolContext, event: Option<&gumball_tool::Event>, sink: &mut Vec<Command<gumball_tool::GumballTool>>) {
    let Some(gumball_tool::Event::Gesture(request)) = event else { return };
    sink.push(Command::Effect(ToolYield::upsert(SHOOTING_GUMBALL_TOOL_KEY, request.leaf.clone())));
    sink.push(Command::Effect(ToolYield::Commit));
}
```

A cancelled drag dispatches nothing. A committed drag is one history row (`ToolTransaction` `<appId>#<verb>`). The React host sends the net delta on release, or streams it with `gumballLiveDispatch` (`phase: "stream" | "commit" | "abort"`).

### 4.4 Progress and cancellation

See §8.

---

## 5. Windows

### 5.1 Scene window definition (World3d)

```rust
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: SHOOTING_PLAY_WINDOW_SCENE.into(),        // "shooting-scene"
        label: LocalizedLabel::native("Scene", "Szene"),
        body_key: SHOOTING_PLAY_BODY_SCENE.into(),    // "shooting.play.scene"
        surface_kind: SurfaceKind::World3d,
        icon_id: "shooting-scene".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
```

Icon window: same shape with `surface_kind: SurfaceKind::IconRender`.

### 5.2 Layout

```rust
pub fn layout() -> WindowLayout {
    create_default_layout(&[scene::SHOOTING_PLAY_WINDOW_SCENE.into(), icon::SHOOTING_PLAY_WINDOW_ICON.into()], "row", Some(&[68.0, 32.0]), Some(&["Model".into(), "Icon".into()]))
}
```

Viewer layout is a single stack:

```rust
WindowLayout {
    root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size: None,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: scene::WINDOW_KIND_ID.into(), title: Some("Scene".into()), instance_id: None, template_id: None, corner: None }],
    }),
}
```

Mode: `ModeDefinition { id: "edit", label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }`.

### 5.3 Window measures (chrome)

Each option is one `WindowMeasure` variant. Shooting's `WindowMeasure::Slider` (`☑️options/🌫️ambient`):

```rust
WindowMeasure::Slider {
    id: "shooting.measure.ambient".into(),
    label: Some(labels.measure_ambient.into()),
    value: snapshot.scene.ambient.intensity,
    min: 0.0, max: 3.0, step: Some(0.05),
    ready: None, loading: None, waiting: None, disabled: None,
    on_change: crate::editor::shooting::shooting_window_action("setAmbientIntensity", None),
}
```

Toggle (`☑️options/☀️sun-enabled`): `WindowMeasure::Toggle { id, icon_id, label, pressed, text: None, on_change }`.

Variants (framework `🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` ~1067): `Select { id, label, value, items, on_change }`, `Slider { … }`, `Number { id, label, value, min, max, step, ready, loading, waiting, disabled, on_change }`, `Toggle { id, icon_id, label, pressed, text, on_change }`.

Measures are rebuilt per frame through `window_measures`. Measure ids must be stable.

### 5.4 Window engagement (camera label input and status)

```rust
WindowEngagement {
    session_active: Some(true),
    options: None,
    input: Some(WindowEngagementInput {
        id: Some("shooting.camera-label".into()),
        value: None,
        placeholder: Some(labels.camera_label_placeholder.into()),
        disabled: None,
        on_change: None,
        on_submit: Some(crate::editor::shooting::shooting_window_action("saveCamera", None)),
        on_repeat_last: None,
        on_abort: None,
    }),
    control: None, controls: None,
    status: Some(vec![WindowEngagementStatus { id: "shooting.status.model".into(), text: format!("{} assets · {} shots", snapshot.assets.len(), snapshot.shots.len()) }]),
    possible_engagements: Some(/* saved cameras → loadSavedCamera */),
}
```

Typing publishes nothing. Submit sends the command.

### 5.5 Window config owner (drawing canvas, the pattern to copy)

Shooting has no window config or transient owners. Its `🫧️transient` folders hold only `📌️.empty.md`. The config contract is in drawing's canvas window (`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config/🦀️.rs`):

```rust
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(layout = "lines")]
#[artifact(id = "s.draw.drawing.canvas-window.config", extension = "drawingcanvaswindowcfg")]
pub struct DrawingCanvasWindowConfig {
    #[dsl(block)]
    pub viewport: store::Viewport2d,
    pub framed: bool,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum DrawingCanvasWindowConfigMutation {
    Set { viewport: store::Viewport2d, framed: bool },
}

impl protocol::Mutation<DrawingCanvasWindowConfig> for DrawingCanvasWindowConfigMutation {
    type Diff = DrawingCanvasWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "…/🪟️windows/🖼️canvas/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Drawing Canvas Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Set",
        payload_schema: "drawing.canvas-window.config",
        …
    }];
    fn inverse(&self, base) -> Result<Vec<Self>, ValueError> { Ok(vec![Self::Set { viewport: base.viewport.clone(), framed: base.framed }]) }
}

pub struct DrawingCanvasWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for DrawingCanvasWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::DRAWING_PLAY_WINDOW_CANVAS;
    const SCHEMA: &'static str = "drawing.canvas-window.config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = DrawingCanvasWindowConfig;
    type Mutation = DrawingCanvasWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> … { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> … { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<DrawingCanvasWindowConfigOwner>()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> DrawingCanvasWindowConfig {
    view.window::<DrawingCanvasWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn addressed(view: &ViewModel, config: DrawingCanvasWindowConfig) -> Result<WindowConfigMutation, Fault> {
    let id = view.window_id.as_deref()…;
    Ok(semio_framework_plugin::WindowConfigMutation::of::<DrawingCanvasWindowConfigOwner>(id, DrawingCanvasWindowConfigMutation::Set { viewport: config.viewport, framed: config.framed }))
}
```

The editor registers it with:

```rust
fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
    canvas_window::config::register(registry)
}
fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
    canvas_window::transient::register(registry)
}
```

Window transient (drawing canvas):

```rust
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DrawingCanvasWindowTransient {
    pub engagement_input: String,
    pub trace_pointer_generation: u64,
    pub trace_pointer_completed_work: u64,
    pub trace_pointer_pending_work: u64,
}

semio_framework_value::artifact_retire_struct!(DrawingCanvasWindowTransient { engagement_input, … });

semio_framework_plugin::transient_root! {
    state: DrawingCanvasWindowTransient,
    mutation: DrawingCanvasWindowTransientMutation,
    owner: "…/🪟️windows/🖼️canvas/🫧️transient",
    kind: "set-window-transient",
    display_name: "Set Drawing Canvas Window Transient",
    payload_schema: "drawing.canvas-window.transient",
    envelope: "s.draw.drawing.canvas-window.transient",
    extension: "drawingcanvaswindowtransient",
}

semio_framework_plugin::window_transient_owners! {
    state: DrawingCanvasWindowTransient,
    mutation: DrawingCanvasWindowTransientMutation,
    windows: { DrawingCanvasWindowTransientOwner => super::DRAWING_PLAY_WINDOW_CANVAS, },
}
```

Recipe for BIM:
- `🗺️plan` window config: `BimPlanWindowConfig { viewport: Viewport2d, storey_id: String, cut_height: f64 }`, with `DRAWING`-style owner.
- `🧊️world` window config: orbit viewport, projection preset, section box, storey isolation.
- `📐️section` window config: section line id and viewport.
- Each window has its transient (engagement input, pointer trace state).
- The app config (`BimConfig`) keeps only session state shared across windows (active storey, active utility, selection).

### 5.6 Presence (shooting)

```rust
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "shooting.presence")]
#[dsl(layout = "lines")]
#[derive(Default)]
pub struct ShootingPresence {
    pub selected_shot_ids: Vec<String>,
    #[dsl(block)]
    pub camera: ShootingCamera,
}
```

Its `ArtifactPack` impl encodes with `wrap_binary`. An empty `bytes` input decodes to `Self::default()`. Recipe: `BimPresence { camera, storey, engagement_input }` (r2 §6) copies this block.

### 5.7 Schema facets (`surface-schema`)

Each lane has five leaves: `🔣️.json`, `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`. They are in `…/✏️editor/🎚️config/🧬️schema/` and `…/👥️presence/🧬️schema/`. The mount is `#[path = "…/🎚️config/🧬️schema/🦀️.rs"] pub mod schema;`, and it declares the facet set:

```rust
pub fn app_schema_descriptor() -> ::semio_framework_schema_registry::AppSchemaDescriptor {
    ::semio_framework_schema_registry::AppSchemaDescriptor {
        id: "s.shooting.shooting",
        config: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        …
    }
}
```

Generator: `bun ./📜️script.ts surface-schema` in the registry folder `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`. From the repo root, the Nx form is `bun nx run @semio-tech/plugin-registry:surface-schema`. Check mode is `bun nx run @semio-tech/plugin-registry:surface-schema-check`. The target definitions are in that folder's `📋️project.json`. The root `package.json` script `nx` wraps `nx`.

Contract (`🧬️surface-schema/🟦️.ts`): one Rust declaration per lane is the source of truth. Its `SURFACE_SCHEMA_BANNER` is `🤖️ Generated by \`bun ./📜️script.ts surface-schema\` (surface-schema-projection) — do not edit.` A leaf without the banner is authored content the projection never touches.

Consequence: the shooting config and presence leaves contain no banner, so the generator treats them as authored (finding F2). BIM facets must either carry the banner (generated) or be hand-maintained. Decide once. Run `surface-schema-check` after every change to the Rust config or presence.

---

## 6. Terminology, keybindings, panels

### 6.1 Terminology (`✏️editor/🗣️terminology/🦀️.rs`)

```rust
semio_framework_ui_locale::app_labels! {
    pub struct ShootingLabels {
        inspection_title: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        shots: native_en "Shots", native_de "Aufnahmen", reuse_en "Shots", reuse_de "Aufnahmen";
        add_shot: native_en "Add Shot", native_de "Aufnahme hinzufügen", reuse_en "Add Shot", reuse_de "Aufnahme hinzufügen";
        …
    }
}

pub fn shooting_play_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static ShootingLabels {
    semio_framework_plugin::resolve_labels::<ShootingLabels>(view_state)
}
```

Recipe: one `app_labels!` block for all of BIM (en first, de second, `native_*` and `reuse_*` pairs). Tests check `"Inspection"` and `"Inspektion"` from `Locale::En` and `Locale::De`.

### 6.2 Keybindings (drawing, `✏️editor/🦀️.rs` ~2260)

```rust
.keybinding("mod+z", "undo")
.keybinding("mod+shift+z", "redo")
.keybinding("escape", "canvasEscape")
.keybinding("enter", "canvasCommitDraft")
.keybinding("delete", "deleteSelection")
.keybinding("backspace", "deleteSelection")
.keybinding("arrowleft", "nudgeSelectionLeft")
.keybinding("shift+arrowleft", "nudgeSelectionLeftFast")
```

Recipe: BIM tool hotkeys (`w` wall, `d` door, `n` window, `c` column, `s` slab, `r` roof) are `.keybinding("w", "setActiveUtility?")` or the tool's own action id. Mirror drawing's `setActiveUtility` pattern.

### 6.3 Panels

Panel tab definition (`📌️panels/🔍️inspection/🦀️.rs`):

```rust
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SHOOTING_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
```

`PanelGroup::Workbench` is used for document and catalogue. `PanelGroup::Details` for inspector.

`PanelTreeBuilder` (framework ~6270):

```rust
pub struct PanelTreeBuilder { namespace: UiText, sections: UiFixedList<BuiltNode>, selected_ids, highlighted_ids, interaction_domain, interaction_select, drop_action }

impl PanelTreeBuilder {
    pub fn new(namespace: impl TryInto<UiText>) -> UiAssemblyResult<Self>
    pub fn item_id(&self, kind: &str, id: &str) -> UiAssemblyResult<UiText>      // "{namespace}.{kind}.{id}"
    pub fn section<I: AsRef<str>>(self, id: I, label: Option<Label>, default_open: bool, items: UiFixedList<BuiltNode>) -> UiAssemblyResult<Self>
    pub fn section_or_placeholder<…>(…) -> …
    pub fn window_section<T>(self, windows: &TreeWindows<'_>, id: &str, label: Option<Label>, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<Self>
    pub fn build(self) -> UiAssemblyResult<BuiltNode>
}
```

`section` is for small fixed lists. Use `window_section` for entity lists (the `TreeWindows` ledger budgets rows).

Outliner (shooting document panel, `📌️panels/🗿️artifact/🦀️.rs`), the tree for BIM's model → site → building → storey → elements:

```rust
PanelTreeBuilder::new("shooting-play-document")?
    .window_section(windows, "shooting-play-document.shots", Some(ui_label(labels.shots.as_str())?), true, &snapshot.shots, |shot| {
        let ids = ui_value_list([ui_value_text(&shot.id)?])?;
        let args = ui_value_map([("shotIds", ids)])?;
        tree_item_with_icon(format!("shooting-shot:{}", shot.id), Label::data(shot.label.clone()), "camera", shooting_action("setShotSelection", Some(args)))
    })?
    .window_section(windows, "shooting-play-document.assets", …, &snapshot.assets, |asset| {
        tree_item_with_icon(format!("shooting-asset:{}", asset.id), Label::data(asset.name.clone()), "box", asset_select_action(&asset.id))
    })?
    .build()
```

Selection from a tree row dispatches `interactionSelect` with `domainId`, `merge` and a JSON-string `targets` array of `InteractionTarget { granularity, id }`:

```rust
let targets = semio_framework_pack_json::to_json_string(&[semio_framework_plugin::InteractionTarget { granularity: "asset".into(), id: asset_id.into() }]);
let args = ui_value_map([
    ("domainId", ui_value_text(SHOOTING_INTERACTION_DOMAIN)?),
    ("merge", ui_value_text("replace")?),
    ("targets", ui_value_text(&targets)?),
])?;
shooting_action("interactionSelect", Some(args))
```

Helpers: `tree_item_with_icon(id, label, icon_id, action)` (shooting root ~line 165), `shooting_action(name, args)` (`ActionFactory::new(CONTROLLER).action(…)`).

Properties / inspector (`📌️panels/🔍️inspection/🦀️.rs`). Authored fields become inputs. Read-only fields become `tree_item_desc`. An edit dispatches a patch command with the target id and field name:

```rust
fn shot_field(shot: &ShootingShot, name: &'static str, label: &str, value: &str, kind: Option<InputKind>) -> UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.shot.{name}");
    if let Some(kind) = kind {
        let args = ui_value_map([("field", ui_value_text(name)?), ("shotIds", ui_value_list([ui_value_text(&shot.id)?])?)])?;
        let (action, args) = shooting_action("patchShots", Some(args))?;
        let builder = input(kind).value(ui_text(value)?);
        let builder = match args {
            Some(args) => builder.try_on_with(Trigger::Change, action, args),
            None => builder.try_on(Trigger::Change, action),
        }
        .map_err(|_| ui_capacity_error())?;
        let control = builder.try_id(format!("{id}.input"))…try_build()…;
        return control_row(&id, label, control);
    }
    tree_item_desc(&id, ui_label(label)?, Some(value.to_string()))
}
```

`InputKind` (framework `🧬️contract/🧩️component/🦀️.rs` line ~130): `Text`, `LongText`, `Number`, `Date`, `Color`, `File`.

Recipe for BIM properties: authored parameters (wall thickness, height, material, type, level) use `InputKind::Number`/`Text` with `patch` commands (`patchElements` with `field` and `ids`). Inferred values (quantities, area, volume) are `tree_item_desc` rows with no control.

Catalogue (`📌️panels/🛍️catalogue/🦀️.rs`): preset rows that dispatch an add command with args:

```rust
fn catalog_shot_item(preset: &ShotPreset) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("format", ui_value_text(preset.format)?), ("shape", ui_value_text(preset.shape)?)])?;
    tree_item_with_icon(format!("shooting-play-catalogue.{}", preset.id), preset.label, "camera", shooting_action("addShot", Some(args)))
}
```

Use this for BIM's utilities list and type library (types and materials).

### 6.4 Schedule (table)

Table surfaces use `SurfaceKind::Table` with a `TableScene`. Shooting has none. The table reference is sourcing's pool (`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs`):

```rust
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { id: SOURCING_CURATION_WINDOW_POOL.into(), label: LocalizedLabel::native("Pool", "Pool"), body_key: SOURCING_CURATION_BODY_POOL.into(), surface_kind: SurfaceKind::Table, icon_id: "library".into(), … }
}

let columns = [("name", labels.col_name.as_str(), true), ("availability", labels.col_availability.as_str(), true), ("curated", labels.col_curated.as_str(), false)];
let rows = pool_kinds(document, cfg).iter().map(|kind| pool_row(document, kind, labels)).collect();
let table = sourcing_table(SOURCING_CURATION_SURFACE_POOL, &columns, rows, "dropOnPool", cfg.filters.sort.as_ref())?;

fn pool_row(…) -> DslValue {
    sourcing_table_row(&kind.id, vec![
        ("name", TableCell::Text { value: kind.name.clone() }),
        ("availability", TableCell::Number { value: kind.availability as f64 }),
        ("curated", TableCell::Stepper { value: curated_count(document, &kind.id) as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_table_action("curationSetCount", Some(&kind.id)) }),
    ])
}
```

Sourcing's helper (`sourcing_table`) builds `TableScene::base(columns_json, rows_json)`, then sets `row_drag_mime`, `drop_action_json`, `sort_json`, `domain_id` and `domain_granularity_id`, and calls `scene_surface(surface_id, SurfaceKind::Table, &scene)`.

`TableCell` (framework ~3380): `Text { value }`, `Number { value }`, `EditableText { value, action }`, `Stepper { value, min, max, step, action }`, `Buttons { buttons }`.

`TableScene` fields (framework ~1958): `lanes`, `columns_json`, `rows_json`, `selection_json`, `row_drag_mime`, `drop_action_json`, `sort_json`, `domain_id`, `domain_granularity_id`. Columns and rows are split into lanes by `split_lanes`.

Recipe: BIM schedule is a `SurfaceKind::Table` window with `columns_json` (quantity fields) and `rows_json` (one row per type or storey). Row selection uses `domain_id` so the selection feeds the same `interactionSelect` path. Quantities are inferred (§9), so the schedule reads from inference, not from authored fields.

---

## 7. Canvas2d (plan and section windows)

### 7.1 Canvas2dScene (framework `🎬️scenes/🦀️.rs` ~170)

```rust
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
    pub framing: Option<crate::Canvas2dFraming>,
    pub snapshot: Option<crate::Canvas2dSnapshotLease>,
    pub tool_run_trace: Option<String>,
    pub lanes: Vec<SceneLaneRef>,
}

impl Canvas2dScene {
    pub fn base(camera_x: f64, camera_y: f64, zoom: f64, layers_json: String) -> Self { … }
}
```

Canvas2d TS twin (`🖱️ui/🎬️scene/🟦️.ts` line 14): `{ cameraX, cameraY, zoom, layersJson, framing?, toolRunTrace?, lanes? }`. `Canvas2dFraming { revision, bounds, padding }`.

Drawing builds it like this:

```rust
scene_surface(
    DRAWING_PLAY_SURFACE_ID,                       // "drawing.play.composite"
    semio_framework_ui_contract::SurfaceKind::Canvas2d,
    &Canvas2dScene {
        framing: (!config.framed && plan.is_some()).then(|| semio_framework_plugin::Canvas2dFraming { revision, bounds, padding: 48.0 }),
        camera_x: config.viewport.x,
        camera_y: config.viewport.y,
        zoom: config.viewport.zoom,
        layers_json: semio_framework_pack_json::to_json_string(&records),
        snapshot: None,
        tool_run_trace: None,
        lanes: Vec::new(),
    },
)
```

### 7.2 layers_json record format

`layers_json` is a JSON array of records. Each record is a `DslValue` object:
- `id: string`
- `role: "meta" | "overlay" | "node" | "handle" | "wire" …` (host and paint code read `role`; `meta` carries `utility`)
- `transform: [a,b,c,d,e,f]`
- `segments: PathSegment[]`
- `fill: null | { kind: "solid", color: [r,g,b,a] }` (also gradient kinds: `x1,y1,x2,y2,cx,cy,r,stops`)
- `stroke: { color: [r,g,b,a], width, cap: "round", join: "round", dash? }`
- `opacity`, `blendMode`, `visible`, `fillRule: "evenodd"|…`
- optional `text: { content, size }` and `image: { src, width, height }`

Overlay constructor (drawing `🖼️canvas/🦀️.rs`):

```rust
fn overlay_record<T: ToValue + ?Sized>(id: &str, transform: [f64; 6], segments: &T, fill: Option<[f64; 4]>, stroke_color: [f64; 4], stroke_width: f64) -> DslValue {
    DslValue::object([
        ("id", String(id)), ("role", "overlay"), ("transform", transform), ("segments", segments),
        ("fill", fill.map_or(Null, |color| object([("kind", "solid"), ("color", color)]))),
        ("stroke", object([("color", stroke_color), ("width", width), ("cap", "round"), ("join", "round")])),
        ("opacity", 1.0), ("blendMode", "normal"), ("visible", true), ("fillRule", "evenodd"),
    ])
}
```

Text label record (artboard dimensions): `("text", object([("content", label), ("size", 12.0)]))` with empty `segments`. Path segments are `PathSegment::{Move{to}, Line{to}, Quad{ctrl,to}, Cubic{ctrl1,ctrl2,to}, Arc{…}, Close}`.

Canvas paint types (`📐️Canvas2dHost/🎨️paint/🟦️.ts` line 8) give `CanvasSceneNode` with the same keys. There is no hatch record type in this format. Hatches must be encoded as a `fill` of kind `pattern` or as many segments. Confirm with the paint code before relying on hatches.

Rule: layers are emitted in paint order. Meta record first. Scene nodes next (via `scene_view::nodes`). Overlays (selection, handles, marquee, preview) last.

### 7.3 Pointer events (canvas-pointer-down/move/up)

Host (`📐️Canvas2dHost/🟦️.tsx` ~290–307) dispatches:

```ts
dispatch("canvasPointerDown", { x: sample[0], y: sample[1], worldX: sample[3], worldY: sample[4], button, shift, ctrl, meta, alt, width, height });
dispatch("canvasPointerMove", { x: last[0], y: last[1], worldX, worldY, ...modifiers, width, height, samples: [[x,y],…], worldSamples: [[wx,wy],…] });
dispatch("canvasPointerUp", { x, y, worldX, worldY, shift, ctrl, meta, alt, width, height, cancelled });
```

Payload `x,y` are CSS pixels inside the canvas. `width,height` are the canvas size. `worldX,worldY` are the host's `screenToWorldLogical`.

Rust converts with the same formula (drawing `canvas_pointer_down`):

```rust
pub(crate) fn canvas_point_to_world(viewport: &store::Viewport2d, x: f64, y: f64, viewport_w: f64, viewport_h: f64) -> (f64, f64) {
    let zoom = viewport.zoom.max(0.01);
    ((x - viewport_w * 0.5) / zoom + viewport.x, (y - viewport_h * 0.5) / zoom + viewport.y)
}
```

Host `screenToWorldLogical`: `x: (screenX - w/2) / zoom + camera.x`, identical. Use the Rust conversion as the single source of truth. The payload's `worldX` is advisory.

Pick tolerance: `DRAWING_PICK_TOLERANCE_PX = 8.0`, divided by zoom to get world units.

Batched moves: `samples` oldest first. A move with no samples uses `x,y`. Lasso consumes one sample per turn.

`cancelled: true` on `canvasPointerUp` ends the gesture without commit.

BIM plan window: `canvasPointerDown` with `activeUtility` `wall`, `column`, `window`, and so on. The command turns the world point into an `interactionSelect` against the element domain (`pick`) or starts a draft (`wall` with two clicks, arc with three).

### 7.4 Drawing canvas window render skeleton

Plan window render signature (drawing): `render(prepared, revision, document, config: &DrawingCanvasWindowConfig, preview: &DrawingGesturePreview, active_utility, selection, point_selection) -> UiAssemblyResult<BuiltNode>`. Recipe: BIM plan render takes the storey's prepared 2D plan (cut elements, footprints, dimensions) as `layers_json` records.

---

## 8. World3d (world window, scene window)

### 8.1 World3dScene (framework `🎬️scenes/🦀️.rs` ~390)

Required fields in the Rust struct:
- `camera_json: String`
- `meshes_json: String` (default `"[]"`)
- `instances_json: String`
- `selection_json: String` (default `{"method":"rectangle","mode":"replace","ids":[],"hoveredId":null}`)

Optional (`skip_serializing_if` none): `snapshot`, `instances_delta_json`, `vortices_json`, `attractions_json`, `target_volumes_json`, `references_json`, `brush_preview_json`, `interaction_json`, `engagement_preview_json`, `pick_targets_json`, `lod_json`, `presentation_json`, `chunking_json`, `environment_json`, `frame_json`, `fit_json`, `terrain_json`, `points_json`, `status_json`, `tool_run_trace`, `annotations`, `scalar_field`, `modelling_options`, `domain_id`, `domain_granularity_id`, `lanes`.

Constructors:
- `World3dScene::base(camera_json, meshes_json, instances_json, selection_json)`
- `world3d_scene(camera_json, meshes_json, instances_json, selection_json, sun: &WorldSunConfig)` sets `environment_json = Some(world3d_environment_json(sun))`.

Shooting render (scene window):

```rust
semio_framework_plugin::scene_surface(
    SHOOTING_PLAY_SURFACE_SCENE,
    semio_framework_ui_contract::SurfaceKind::World3d,
    &World3dScene {
        environment_json: Some(shooting_environment_json(snapshot)),
        frame_json: active_shot(snapshot).map(shooting_frame_json),
        fit_json: Some(shooting_fit_json(cfg)),
        ..world3d_scene(camera_json(&cfg.camera), world_meshes_json(snapshot), world_instances_json(snapshot), world_selection_json(snapshot, active_utility), &WorldSunConfig::default())
    },
)
```

`scene_surface<T: SceneDoc>(id, kind, scene) -> UiAssemblyResult<BuiltNode>` (plugin ~600).

### 8.2 camera_json

Shooting (`world3d` camera shape):

```rust
fn camera_json(camera: &crate::ShootingCamera) -> String {
    let mut value = json!({
        "position": vec3(camera.position),
        "target": vec3(camera.target),
        "fov": camera.fov,
        "zoom": camera.zoom,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (value.as_object_mut(), camera.up) { object.insert("up", vec3(up)); }
    value.to_string()
}
```

Framework helper: `world3d_camera_json(position, target, fov)` (lowpoly uses it).

### 8.3 meshes_json

Framework helpers (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` ~48554–48630):

```rust
pub fn world3d_mesh_id_from_url(url: &str) -> String {
    let slug = url.trim_start_matches('/').rsplit('/').next().unwrap_or(url).trim_end_matches(".glb").trim_end_matches(".gltf");
    format!("mesh:{slug}")
}

pub fn world3d_meshes_json_from_kinds_and_urls(kinds: &[String], urls: &[String]) -> String {
    let mut meshes = Vec::with_capacity(kinds.len());
    for kind in kinds.iter() { meshes.push(world3d_mesh_kind_entry(kind)); }   // { id: kind, kind: kind }
    for url in urls {
        let id = world3d_mesh_id_from_url(url);
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) { continue; }
        meshes.push(object([("id", id), ("url", url.clone())]));
    }
    to_string(&Value::Array(meshes))
}
```

Three encodings, one per mesh entry:
- `{ "id": "box", "kind": "box" }`: procedural kind. The host resolves geometry with `meshDataFromKind(kind)`. Scene payloads never include tessellation for kinds.
- `{ "id": "mesh:<slug>", "url": "…glb" }`: GLB asset, loaded by URL.
- `{ "id": "<objectId>", "data": MeshData }`: inline mesh. Used by lowpoly and CAD (`world_meshes_json`). `data` is the framework `MeshData` (`🏗️mesh-engine`): `positions: f32[]` (flat xyz), `normals: f32[]`, `colors: f32[]` (RGBA linear), `indices: u32[]` (triangles), optional `uvs`, `faceIds`, `vertexIds`, `edgePositions`, `edgeIds`, `edgeUvs`, `edgeIsSeam: u8[]`, `paintTextureBase64`.

Host parse: `parseMeshes(meshesJson)` = `JSON.parse` then `resolveMeshRecord` (`record.data || !record.kind ? record : { ...record, data: meshDataFromKind(record.kind) }`). Mesh identity is decided by wire text (`splitJsonArrayElements`).

Lowpoly inline encoding:

```rust
let meshes = items.iter().filter_map(|item| {
    let id = item.get("id")?.as_str()?;
    let tessellation = item.get("tessellation")?;
    Some(DslValue::object([("id", id), ("data", ToValue::to_value(&mesh_data_from_transfer(tessellation, texture_cache.get(id).cloned())))]))
}).collect();
```

CAD inline encoding: `DslValue::object([("id", object.id), ("data", mesh_data_to_dsl(&data))])`, with a fallback `mesh_from_kind(CAD_FALLBACK_MESH_KIND)` when there are no objects.

Recipe for BIM: building elements tessellate into inline `data` entries keyed by element id (walls, slabs, roof). Only the fallback and imported GLBs use `kind` or `url`.

### 8.4 instances_json

Shooting (`world_instances_json`):

```rust
json!({
    "id": asset.id.as_str(),
    "meshId": mesh_id,
    "position": vec3([x, y, z]),
    "rotation": vec4(asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
    "scale": vec3(shooting_asset_scale(asset)),
    "label": asset.name.as_str(),
    "color": if selected { "#9aa0ab" } else { "#6b7280" },
    "selected": selected,
    "hovered": hovered,
})
```

Host record `WorldInstanceRecord` (`🌐️World3dHost/🟦️.tsx` ~221): `id`, `meshId?`, `position?`, `rotation?` (quaternion xyzw), `scale?`, `selected?`, `hovered?`, `highlighted?`, `disabled?`, `provisional?`, `smoothShading?`, `objectKind?`, `interactionId?`, `componentSource?`, `interactionGranularityId?`.

`interactionId` is what a pick resolves to when it differs from `id`. Lowpoly sets `interactionId = document_object_row_id(id)` and `interactionGranularityId = MESH_GRANULARITY_OBJECT`. Use this to map element instance → domain target.

Instance order matters: the host's `worldPick` resolves a hit by array index.

### 8.5 selection_json and gumball

Lowpoly/shooting selection (`world_selection_json`):

```rust
let mut value: Value = parse(&world3d_selection_json("pick", &[], None), …)?;
object.insert("transformMode", json!(active_utility));
object.insert("activeObjectId", json!(snapshot.active_asset_id.as_str()));
object.insert("gumballActive", json!(false));
```

Host `WorldSelectionRecord` fields: `method`, `selectionMergeMode`, `ids`, `hoveredId`, `granularity`, `selectionMode`, `activeObjectId`, `transformMode`, `interactionMode`, `gumballTarget`, `gumballActive`, `gumballSelectionIds`, `gumballConfig`, `gumballLiveDispatch`, `hoveredComponent`.

Gumball handles from a mode:

```ts
export function gumballConfigForTransformMode(mode: string, plane?: GumballConfig["plane"]): GumballConfig {
  const groups =
    mode === "transform" ? { moveAxes: true, movePlanes: true, rotate: true, scaleAxes: false, scalePlanes: false, scaleUniform: false }
    : mode === "rotate" ? { moveAxes: false, movePlanes: false, rotate: true, … }
    : mode === "scale" ? { moveAxes: false, movePlanes: false, rotate: false, scaleAxes: true, scalePlanes: true, scaleUniform: true }
    : { moveAxes: true, movePlanes: true, rotate: false, … };
  return plane ? { ...groups, plane } : groups;
}
```

Gumball verbs dispatched by the host: `translateSelection {…base, dx, dy, dz}`, `rotateSelection {…base, ax, ay, az, angle}`, `scaleSelection {…base, sx, sy, sz}` (`gumballIdentityDelta`). The base carries `ids`/`assetIds` from the selection.

Recipe: BIM gumball handles for element move (translate), rotate (door/window swing), and height (`ids` and `dz`). The drag is one history row through the tool machine (§4.3).

### 8.6 selectionJson and the interaction domain (picking)

Host instance pointer-down (`handleInstancePointerDown`, host line ~7010):

```ts
if (interactionDomainId) {
  dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, [world3dInstanceInteractionTarget(instances, id, interactionGranularity)], merge));
  return;
}
if (selectionMode === "mesh" || selectionMode === "object") {
  dispatch("worldPick", { granularity: "mesh", id: index, merge });
  return;
}
```

`interactionSelect` args: `{ domainId, targets: JSON string of [{granularity, id}], merge, method: "pick" }` (`world3dSelectionActionArgs`, host ~5993). Hover args: `{ domainId, channel: "pointer", targets: JSON string }` (`world3dHoverActionArgs`, ~5903).

The domain is set on the scene: `scene.domain_id = Some(…)` and `scene.domain_granularity_id = Some(…)`. Lowpoly does this in its render. Shooting does not.

Finding F1: shooting's scene never sets `domain_id`, so the host uses `worldPick` (index-addressed), which shooting no longer handles (the row was deleted). This path is probably dead for 3D clicks. Verify at runtime with console logs before fixing. BIM must set `domain_id` and `domain_granularity_id` on the plan and world scenes, and declare the domain with `window_kind_interactions`.

### 8.7 environment, frame, fit

Shooting:

```rust
fn shooting_environment_json(snapshot) -> String {
    json!({
        "ambient": { "intensity": …, "color": … },
        "sun": { "enabled": …, "azimuth": …, "elevation": …, "intensity": …, "color": … },
        "shadow": { "enabled": …, "opacity": …, "softness": … },
        "material": { "color": …, "metalness": …, "roughness": …, "emissive": …, "emissiveIntensity": …, "stroke": … },
    }).to_string()
}
fn shooting_fit_json(cfg) -> String { json!({ "enabled": cfg.center_model, "revision": cfg.fit_revision, "padding": 1.25 }).to_string() }
fn shooting_frame_json(shot) -> String { json!({ "width": …, "height": …, "shape": …, "badge": true }).to_string() }
```

Recipe: `fit_json` with a revision counter gives "frame all" on demand, as in shooting's center-model toggle.

---

## 9. Progress and cancellation

### 9.1 Framework

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏳️operation-progress/🦀️.rs`:

```rust
pub const CANCEL_TYPED_OPERATION_ACTION_ID: &str = "cancelTypedOperation";

#[derive(Clone, Debug)]
pub struct ArtifactOperationProgress {
    pub operation_id: u64,
    pub generation: u64,
    pub label: LocalizedLabel,
    pub completed_units: u64,
    pub cancelling: bool,
}

pub fn operation_progress_controls(status: &[ArtifactOperationProgress], controller: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> { … }
```

`operation_progress_controls` renders, per operation, a status text (`Working · Completed units: N` or `Cancelling`), a progress bar, and a Cancel button. The button dispatches `cancelTypedOperation` with `{ operationId, generation }` as 16-hex strings. `cancellation_action_definition()` declares it as `resumable_framework` with `ActionKind::View` and `in_palette(false)`.

Host side: `VcsArtifactApp::operation_progress_snapshot()` reads `tool_operations` with progress. `take_operation_progress_scope()` returns the app's `operation_progress_scope()` only when something changed. `dispatch_operation_cancellation` checks the target's instance, actor and generation, then cancels its lease.

### 9.2 Editor usage (raster, the concrete copy)

Raster's editor (`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`):

```rust
/// ⏳️ The layers panel renders the live operations with their Cancel controls; nothing else shows them.
fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
    semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![crate::editor::raster::panels::document::RASTER_PLAY_BODY_LAYERS.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
}
```

Render, inserted above the body when operations exist:

```rust
let node = if body_key == RASTER_PLAY_BODY_LAYERS && !doc.operations().is_empty() {
    let progress = semio_framework_plugin::app::operation_progress::operation_progress_controls(doc.operations(), RASTER_PLAY_CONTROLLER_ID, view_state.locale)?;
    semio_framework_ui_contract::column().try_id("raster.layers.with-progress")…try_children([progress, node])…
} else { node };
```

Recipe for BIM: `operation_progress_scope()` returns a `Partial` scope over the outliner and the status bar panels. Render `operation_progress_controls` above the outliner or in the status panel. Use it for IFC import, full inference runs, quantity takeoff, and large boolean ops. All of these must be `resumable` jobs (§4.2) and report `completed_units`.

---

## 10. Inference (how an editor reads results)

### 10.1 Shooting

The shooting editor does not read inference at render time. Its inference schema is `ShootingInference`:

```rust
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[artifact_schema(id = "s.shooting.shooting.inference")]
pub struct ShootingInference {
    #[derived]
    pub topology: ShootingTopology,
}

impl protocol::Inference<ShootingSnapshot> for ShootingInference {
    fn infer(snapshot: &ShootingSnapshot) -> Result<Self, ValueError> {
        Ok(Self { topology: compute_shooting_topology(snapshot) })
    }
}

impl protocol::InferenceSpec<ShootingSnapshot> for ShootingInference {
    fn inference_schema_id() -> &'static str { "s.shooting.shooting.inference" }
    fn schema_version() -> u32 { 1 }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.shooting.shooting.inference.topology", reads: &["shots", "saved_cameras"] }]
    }
}
```

### 10.2 Framework `InferredField` (incremental, dependency-aware)

`🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs`:

```rust
pub trait InferredField<P>: Send + Sync + 'static {
    type Key: Clone + Eq + std::hash::Hash + Ord + Send + Sync + ToValue + FromValue + 'static;
    type Value: Clone + Send + Sync + 'static;
    type Dependency: ToValue;
    const FIELD_ID: &'static str;
    const SCHEMA_VERSION: u32;
    fn reads() -> &'static [&'static str];
    fn plan(snapshot: &P) -> Vec<InferenceStep<Self::Key>>;
    fn dep_input(snapshot: &P, key: &Self::Key, parents: &[Self::Key]) -> Self::Dependency;
    fn compute(snapshot: &P, key: &Self::Key, parents: &[Self::Value]) -> Self::Value;
    fn compute_step(…) -> Result<ComputeStep<Self::Value>, InferenceFault> { … }   // resumable, fuel-based
}

pub struct InferenceCacheConfig { pub enabled: bool, pub budget_bytes: usize, pub persistence: InferencePersistence, pub record_stats: bool }
// Default: enabled = false, budget 16 MiB, persistence None, record_stats false.

pub struct InferenceCache { … }
impl InferenceCache { pub async fn new(config: InferenceCacheConfig) -> Self; pub fn enabled(&self) -> bool; pub fn used_bytes(&self) -> usize; pub async fn clear(&mut self); … }

pub struct InferenceSession { … }   // per-artifact-instance tier-1 gate: root DepHash per field id

pub fn infer_field<P, F: InferredField<P>>(snapshot: &P, cache: Option<&mut InferenceCache>) -> BTreeMap<F::Key, F::Value>;
pub async fn infer_field_after_diff<P, F, D>(snapshot: &P, diff: &D, session: &mut InferenceSession, cache: &mut InferenceCache) -> BTreeMap<F::Key, F::Value>;
```

`infer_field_after_diff` returns the stored result when the diff does not touch `F::reads()`. Otherwise it walks the plan and refreshes the session.

Plugin-side entry (`🔌️plugin/🦀️.rs` ~1822): `pub async fn infer_artifact(artifact_kind, inference_schema, request: &ArtifactInferenceExecutionRequest) -> Result<ArtifactInferenceExecution, …>`. The request carries `policy`, `budgets`, `cancellation_id`, `previous_state`, `requested_cache_mode` (`WireArtifactInferenceCacheMode::Cold | Incremental`).

### 10.3 How an editor consumes results

Two patterns exist in the repo:

(a) Service effect (gismap). The command does not compute. It asks the host to run the inference service, which returns a reviewable proposal through a later server-stamped command:

```rust
pub fn handle(_payload: &ProposeBoundsRegion, _doc, _cfg) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestServiceOperation { owner: "gis".into(), service_id: "s.gis.gismap.inference".into(), action: "propose".into(), payload: DslValue::Object(Vec::new()) }))
}
```

(b) Retained geometry session (drawing). A thread-local registry per app instance holds the prepared plan and the cached paint, and the render reads it. Public surface (`🧵️geometry/🦀️.rs`, drawing `✏️editor`):

```rust
pub fn prepare(render: AppRenderOperationContext, document: &DrawingSnapshot) -> bool
pub fn prepare_query(source: SceneIdentity, document: &DrawingSnapshot) -> bool
pub fn reconcile(doc: &ArtifactView<'_, DrawingSnapshot>) -> Vec<Effect>
pub fn with_visual<R>(render: Option<AppRenderOperationContext>, build: impl FnOnce(Option<&PreparedScene>, u32, bool) -> R) -> R
pub fn with_query<R>(captured: SceneIdentity, query: impl FnOnce(SceneAdmissionStatus, Option<MountedSceneQuery<'_>>) -> R) -> R
pub fn maintenance(instance: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep
pub fn close(instance: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep
pub fn terminal_is_empty(instance: u32) -> bool
```

Its state is a `thread_local! { static MOUNTED: RefCell<Registry> }` keyed by `app_instance_id` with `INSTANCES = 64`. The render calls `geometry_session::with_visual(doc.render_operation(), |plan, revision, fresh| canvas_window::render(plan, …))`.

Recipe for BIM: keep `InferenceCache` and `InferenceSession` in a per-instance thread-local registry, mirroring `geometry_session`. `prepare` runs the inference (through `infer_field_after_diff`) when the document revision changes. `reconcile` returns effects. `with_visual` gives the render the derived quantities and the plan. Close the registry on instance teardown with `close` and `terminal_is_empty`, as the framework requires. Set the cache `enabled: true` explicitly, since the default is off.

Inference fields for BIM (from r1-explore-inferences): topology (storeys, host relations), quantities (area, volume, length), and schedule rows. Each is an `InferredField` with `reads()` naming the element fields it depends on.

Viewer: `ViewerApp` gets the same inference result from the document. It never mutates.

---

## 11. Viewer

`👁️viewer/🦀️.rs`:

```rust
#[derive(Default, Clone, Copy)]
pub struct ShootingViewer;

impl ArtifactViewer for ShootingViewer {
    type Snapshot = ShootingSnapshot;
    type Mutation = ShootingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ShootingViewCommand;   // Noop only

    const DIALECT: Dialect = SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    fn handle(…) -> Result<ViewEmit<Self::ConfigMutation>, Fault> { Ok(ViewEmit::default()) }

    fn render(body_key: &str, doc, _cfg, _view_state) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            scene::BODY_KEY => scene::render(doc.snapshot),
            _ => …
        }.map(built_to_component_tree)
    }
}

pub fn create_shooting_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SHOOTING_DIALECT).document(["semio", "shooting"]).icon_id("camera").mode_def(view::definition()).default_mode_id(view::SHOOTING_VIEW_MODE_VIEW).window_kind_def(scene::definition()).default_layout(view::layout()).build_definition()
}
```

Viewer rules:
- No config, no presence, no transient (`NoConfig`, `NoPresence`, `NoTransient`).
- Viewer command enum has one inert variant. `ShootingViewCommand` implements `OpBinary` with empty bytes.
- The viewer scene (`👁️viewer/…/🪟️windows/🎥️scene/🦀️.rs`) duplicates helper functions rather than importing the editor (`policyViewerPurityBreaches`). Its `BODY_KEY = "shooting.view.scene"`, `WINDOW_KIND_ID = "shooting-view-scene"`.
- Tests: `assert_viewer_never_mutates::<ShootingViewer>()` and `assert_editor_and_viewer_share_dialect::<ShootingPlayApp, ShootingViewer>()` in `HUB/🧪️tests/🔬️surface/🦀️.rs`.

BIM viewer: `BimModelViewer` with mode `view` and windows `🧊️world` and `🗺️plan` (read-only). Read-only window config is not required.

---

## 12. Tests required per feature

- Test-driven: write the test first and run it. Check `[DEBUG]` logs in temporary code.
- Language-agnostic: a JSON fixture under `🧫️fixtures/` read by `include_str!` for each oracle (window actions, command routes, scene vectors). Shooting pattern: `window-actions.json` oracle compared to `serde_json::to_value(measures)`.
- Third-party oracle: at least one independent library or the reference implementation for geometry (for example `three.js` for mesh bounds, IFC reference for quantities), compared to the same output. Shooting uses `bun:test` with three.js/ajv in the TS twins.
- Round trip: `every_command_round_trips_through_text_and_binary`.
- Undo/redo: `assert_undo_redo_round_trip`.
- Convergence: `assert_two_registered_instances_converge`.
- Locale: `Locale::En` and `Locale::De`.
- Viewer law: `assert_viewer_never_mutates`.

---

## 13. Proposed BIM mapping (r2 §6 to shooting structure)

| r2 item | Recipe node | Mirrors |
|---|---|---|
| Editor `BimModelApp: ArtifactEditor` | `✏️editor/🦀️.rs` | shooting `ShootingPlayApp` |
| Mode `✏️edit` | `✏️editor/🎭️modes/✏️edit/` | shooting `edit` mode, `default_layout` |
| `🗺️plan` (Canvas2d, storey) | `…/🪟️windows/🗺️plan/` with config owner (storey id, Viewport2d, cut height) and transient | drawing `🖼️canvas` config and transient |
| `🧊️world` (World3d) | `…/🪟️windows/🧊️world/` with config (Viewport3dOrbit, projection, section box, storey isolation) | shooting `🎥️scene` |
| `📐️section` (Canvas2d) | `…/🪟️windows/📐️section/` with config (section line) | drawing canvas |
| `🌳️outliner` | `…/📌️panels/🌳️outliner/` | shooting `🗿️artifact` panel, `PanelTreeBuilder` |
| `🔍️properties` | `…/📌️panels/🔍️properties/` | shooting `🔍️inspection`, `InputKind` rows |
| `🛍️library` | `…/📌️panels/🛍️library/` | shooting `🛍️catalogue` |
| `🧮️schedule` | `…/📌️panels/🧮️schedule/` (`SurfaceKind::Table`) | sourcing `🏊️pool` |
| Commands `create-*`, `move-elements`, `set-wall-axis` | `…/🎮️commands/*` | shooting `🎮️commands/*` |
| Gestures (wall, arc, slab, roof, window, door, stair, grid, measure) | bounded and resumable factories, `InteractiveJob` | §4.1 and §4.2 |
| Gumball/drag | tool machine | §4.3 |
| Presence `BimPresence` | `…/👥️presence/` | shooting `👥️presence` |
| Terminology `BimLabels` (en, de) | `…/🗣️terminology/` | shooting `app_labels!` |
| Keybindings | `.keybinding(…)` | drawing §6.2 |
| Viewer `BimModelViewer` | `👁️viewer/` (mode `view`, world + plan) | shooting viewer |
| Progress and cancel | `operation_progress_scope` | raster §9.2 |
| Inference (quantities, topology) | `InferredField` + thread-local session | §10.3 (b) |

Build order (follow AGENTS.md: test first, one feature at a time):
1. Plugin skeleton and declaration (§1), with one command and a test that dispatches it through `dispatch` and asserts `edited_document()`.
2. Editor manifest (§2.6) with the interaction domain (§2.3) and the viewer.
3. Scene windows: `World3dScene` for `🧊️world` with `domain_id` set (§8.6), then `Canvas2dScene` for `🗺️plan` (§7).
4. Window config and transient owners (§5.5), with a window-ownership test per owner.
5. Panels (§6.3) and the schedule table (§6.4).
6. Gesture jobs (§4), then gumball (§4.3).
7. Inference and progress (§9, §10).
8. Schema facets (§5.7): decide banner policy, then run `bun nx run @semio-tech/plugin-registry:surface-schema-check`.

---

## 14. Findings and risks

- F1 (likely bug, verify first). Shooting's `World3dScene` never sets `domain_id`. The host then dispatches index-addressed `worldPick` (`World3dHost` ~7010–7025), and shooting no longer has that action. Instance picks in the scene may do nothing. Confirm with `read_console_messages` in the running app. BIM must set `domain_id` and `domain_granularity_id` (§8.6).
- F2 (schema drift). Shooting presence leaves disagree on field `activeUtilityId`. It appears in `👥️presence/🧬️schema/🟦️.ts` and the `.proto`, but not in the Rust `ShootingPresence` (`👥️presence/🦀️.rs`) or the JSON leaf. The leaves are also not banner-marked, so `surface-schema` leaves them alone. Do not copy the drift. Keep Rust, JSON, TS, GraphQL and proto in sync.
- F3 (legacy mount in drawing). The drawing artifact root mounts `canvas-pointer-down` from `✏️editor/🪆️1-any/…` (artifact root `🦀️.rs` line ~1120), while the `🏅️standards/…/✏️editor` tree holds the other nodes and tests. Do not copy the `🪆️1-any` path. Use the `🏅️standards` layout.
- F4 (shooting has no window owners). `🎭️modes/…/🫧️transient/` and `🎚️config` hold only `.empty.md` placeholders. Window config and transient precedents must come from drawing (§5.5).
- F5 (surface-schema banner). Only generated leaves carry `🤖️ Generated by …`. Decide before running the generator, because a run regenerates only bannered leaves.
- F6 (shooting facets). Shooting leaves use `include_str!` through `app_schema_descriptor()`. Any BIM facet must be listed there or the registry will not see it.

---

## 15. Verification status

- Read: all files named above, plus `r1-explore-editor-viewer.md` and `r2-design.md` §6.
- Not run: no `cargo test`, no `bun nx run … surface-schema`, no runtime console check. Findings F1 and F2 are from reading code only.
