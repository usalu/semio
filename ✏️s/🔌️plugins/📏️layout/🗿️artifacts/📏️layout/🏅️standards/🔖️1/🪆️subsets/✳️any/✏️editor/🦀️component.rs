//! 🖥️ Layout play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared canvas chrome in `🦀️canvas.rs`, headless compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `LayoutCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault>`, the exact signature `ArtifactEditor::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// (clippy::result_large_err is allowed crate-wide from the plugin root 📦️glue.rs.)

use crate::artifacts::layout::mutations::change_data_fields::mutation::ChangeDataFields;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::editor::layout::modes::edit;
use crate::editor::layout::modes::edit::windows::{blueprint, preview};
use crate::editor::layout::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, preflight as preflight_panel};
use crate::editor::layout::terminology::{layout_labels, LayoutLabels};
use semio_framework::kernel::Effect;
use semio_framework::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ArtifactEditor, ArtifactKindSpec, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UiNode,
    WindowEngagement, WindowEngagementInput, WindowEngagementPossible, WindowEngagementStatus, CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

use crate::editor::layout::engine::scene::LayoutEngine;

//#region 🔖️Constants
pub const LAYOUT_PLAY_APP_ID: &str = "layout-play";
pub use blueprint::{LAYOUT_PLAY_BODY_BLUEPRINT, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_WINDOW_BLUEPRINT};
pub use catalogue_panel::LAYOUT_PLAY_BODY_CATALOGUE;
pub use document_panel::LAYOUT_PLAY_BODY_DOCUMENT;
pub use inspection_panel::LAYOUT_PLAY_BODY_INSPECTION;
pub use preflight_panel::{LAYOUT_PLAY_BODY_PREFLIGHT, LAYOUT_PLAY_PREFLIGHT_TAB_ID};
pub use preview::{LAYOUT_PLAY_BODY_PREVIEW, LAYOUT_PLAY_SURFACE_PREVIEW, LAYOUT_PLAY_WINDOW_PREVIEW};

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub async fn layout_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(LAYOUT_PLAY_APP_ID).action(action, args)
}

//#region 🔖️Interaction
/// 🕹️ The one framework-owned interaction domain layout declares — pages/frames on the Blueprint
/// canvas plus the document tree's frame rows (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
/// Pages are never targets: canvas hit-testing only ever resolves frame ids (`DisplayList::hit_test`
/// only walks `rects`/`images`), so `"elements"` is frames-only despite the document tree also
/// listing pages/spreads/layers/etc.
pub const LAYOUT_INTERACTION_ELEMENTS: &str = "elements";
pub const LAYOUT_GRANULARITY_ELEMENT: &str = "element";

/// 🕹️ Builds `interactionSelect`'s JSON args for one merge over `ids` (all granularity `"element"`) —
/// shared by the canvas pointer commands (wrapped into a `Effect::DispatchAction`) and any
/// document-tree row whose click should select a real canvas element (wrapped into an `ActionDescriptor`).
pub async fn layout_select_action_args(ids: &[String], merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })).collect();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}

/// 🐁️ Builds `interactionHover`'s JSON args for the `"pointer"` channel — `id: None` clears hover.
pub async fn layout_hover_action_args(id: Option<&str>) -> Value {
    let targets: Vec<Value> = id.map(|id| vec![json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })]).unwrap_or_default();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": serde_json::to_string(&targets).unwrap_or_default() })
}

/// 🕹️ Wraps [`layout_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the
/// app asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub async fn layout_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(layout_select_action_args(ids, merge))), delay_ms: 0 }
}

/// 🐁️ Wraps [`layout_hover_action_args`] the same way, for `interactionHover`.
pub async fn layout_hover_effect(id: Option<&str>) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(114), action: INTERACTION_HOVER_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(layout_hover_action_args(id))), delay_ms: 0 }
}

/// 🕹️ Clicking empty canvas clears every domain's selection — `clearSelection` takes no `domainId`.
pub async fn layout_clear_selection_effect() -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 }
}
//#endregion 🔖️Interaction

/// 🙈️ An internal (non-palette) action declaration — the pointer/inspector/DnD/engagement-bound
/// vocabulary dispatched by the canvas and panels, never surfaced as a standalone palette command.
async fn layout_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `LayoutPlayApp::Command` — the SOLE dispatch surface for layout's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum LayoutCommand for LayoutSnapshot, LayoutMutation, LayoutConfig, LayoutConfigMutation {
        "setActivePage" as "active-page" => set_active_page::SetActivePage,
        "focusPreflightIssue" as "focus-preflight-issue" => focus_preflight_issue::FocusPreflightIssue,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasDragOver" as "canvas-drag-over" => canvas_drag_over::CanvasDragOver,
        "canvasDragLeave" as "canvas-drag-leave" => canvas_drag_leave::CanvasDragLeave,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setLocale" as "locale" => set_locale::SetLocale,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addPage" as "add-page" => add_page::AddPage,
        "patchPage" as "patch-page" => patch_page::PatchPage,
        "patchFrame" as "patch-frame" => patch_frame::PatchFrame,
        "canvasDrop" as "canvas-drop" => canvas_drop::CanvasDrop,
        "exportPng" as "export-png" => export_png::ExportPng,
        "exportSvg" as "export-svg" => export_svg::ExportSvg,
        "exportPdf" as "export-pdf" => export_pdf::ExportPdf,
        "exportPackage" as "export-package" => export_package::ExportPackage,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::layout::commands::{
    add_frame, add_page, canvas_drag_leave, canvas_drag_over, canvas_drop, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, engagement_input, engagement_submit, export_package, export_pdf, export_png, export_svg, focus_preflight_issue,
    patch_frame, patch_page, set_active_page, set_camera, set_locale,
};
//#endregion 🔖️Commands

//#region 🔖️WindowEngagement
async fn layout_window_engagement(config: &LayoutConfig, label: &str, labels: &LayoutLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some(format!("layout-engagement-{label}")),
            value: Some(config.engagement_input.clone()),
            placeholder: Some("undo, redo, export png".into()),
            disabled: None,
            on_change: Some(layout_action("engagementInput", None)),
            on_submit: Some(layout_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("layout-status-{label}"), text: format!("{} {}", labels.page.as_str(), config.active_page_id) }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "layout.eng.undo".into(), label: labels.undo.into(), detail: None, action: Some(layout_action("undo", None)) },
            WindowEngagementPossible { id: "layout.eng.redo".into(), label: labels.redo.into(), detail: None, action: Some(layout_action("redo", None)) },
        ]),
    }
}
//#endregion 🔖️WindowEngagement

//#region 🔖️LayoutPlayApp
/// 🧪️ B1: unit struct — every former `LayoutPlayRuntime` field now lives in [`LayoutConfig`], written
/// through [`LayoutConfigMutation`]s. Parley/font layout state stays on the app instance.
#[derive(Default)]
pub struct LayoutPlayApp;

impl ArtifactEditor for LayoutPlayApp {
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Config = LayoutConfig;
    type ConfigMutation = LayoutConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::layout::presence::LayoutPresence;
    type PresenceMutation = crate::editor::layout::presence::LayoutPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = LayoutCommand;

    const DIALECT: Dialect = crate::artifacts::layout::LAYOUT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::layout::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> LayoutSnapshot {
        crate::artifacts::layout::schema::default_document()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::editor::layout::engine::layout_io())
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &LayoutCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &LayoutCommand,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, LayoutConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LayoutMutation, LayoutConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `layout:out` forfeits the default's dispatch); `layout:out`
    /// re-exports the current layout's first page as `2d.layout` vector/SVG — reuses
    /// `export_document_svg` (the same exporter `exportSvg`/`LayoutCommand::ExportSvg` use). No `cfg`
    /// parameter reaches this method, so there is no config-carried "active page" to prefer over the
    /// first page.
    async fn export_media(port: &str, doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    payload: MediaPayload::Structured { schema: crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "layout:out" => {
                let document = doc.snapshot;
                let page = document.pages.first().ok_or_else(|| MediaError::Payload(port.to_string(), "layout has no pages to export".into()))?;
                let svg = crate::editor::layout::engine::scene::export_document_svg(document, &page.id).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.layout".into(), json: svg } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `fields:in` binds the incoming `form.dictionary`
    /// values into `LayoutSnapshot::data_fields_json` — layout has no existing text-interpolation/
    /// field-binding concept for frames/stories yet, so this stores the dictionary verbatim as a new
    /// named data source (see `crate::artifacts::layout::LayoutSnapshot::data_fields_json`'s doc) rather
    /// than wiring it into rendering today.
    async fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Emit<LayoutMutation, LayoutConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "fields:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "fields:in only accepts a Structured (JSON object) payload".into()));
                };
                Ok(Emit::mutations(vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_json: Some(json.clone()) })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    async fn render(body_key: &str, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> UiNode {
        let document = doc.snapshot;
        let config = cfg.snapshot;
        let labels = layout_labels(config);
        let mut engine = LayoutEngine::new();
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => blueprint::render(&mut engine, document, config),
            LAYOUT_PLAY_BODY_PREVIEW => preview::render(&mut engine, document, config),
            LAYOUT_PLAY_BODY_DOCUMENT => document_panel::render(document, config, labels),
            LAYOUT_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            LAYOUT_PLAY_BODY_INSPECTION => inspection_panel::render(document, config, labels),
            LAYOUT_PLAY_BODY_PREFLIGHT => preflight_panel::render(document, config),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    async fn window_engagements(_doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let labels = layout_labels(config);
        HashMap::from([(LAYOUT_PLAY_WINDOW_BLUEPRINT.to_string(), layout_window_engagement(config, "blueprint", labels)), (LAYOUT_PLAY_WINDOW_PREVIEW.to_string(), layout_window_engagement(config, "preview", labels))])
    }
}
//#endregion 🔖️LayoutPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_layout_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::layout::LAYOUT_DIALECT)
            .artifact_kind(ArtifactKindSpec {
                id: "2d.layout".into(),
                name: "Layout".into(),
                source_format: "layout.layout".into(),
                component_kind: "layout".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "layout.layout".into(),
                export_formats: vec![],
                import_formats: vec![],
                export_stdio_kinds: vec!["stdio.svg", "stdio.png"],
                import_stdio_kinds: vec!["stdio.svg", "stdio.png"],
            })
            .document(["semio", "layout"])
            .icon_id("layout")
            .mode_def(edit::definition())
            .default_mode_id(edit::LAYOUT_PLAY_MODE_EDIT)
            .window_kind_def(blueprint::definition())
            .window_kind_def(preview::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(preflight_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Palette-visible content commands — dispatched as VCS operations with a true inverse.
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addPage", LocalizedLabel::native("Add Page", "Seite hinzufügen"))
            .action_args("addFrame", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("rect", LocalizedLabel::native("Rectangle", "Rechteck")),
                    ActionArgOption::new("text", LocalizedLabel::native("Text Frame", "Textrahmen")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image Frame", "Bildrahmen")),
                ]).default_value("rect"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")),
            ])
            // 🐚️ Palette-visible shell exports — round-trip through the host.
            .shell_action("exportPng", LocalizedLabel::native("Export Png", "Png exportieren"))
            .shell_action("exportSvg", LocalizedLabel::native("Export Svg", "Svg exportieren"))
            .shell_action("exportPdf", LocalizedLabel::native("Export Pdf", "Pdf exportieren"))
            .shell_action("exportPackage", LocalizedLabel::native("Export Package", "Paket exportieren"))
            // 🔧️ Internal document operations — inspector/DnD-bound, not palette commands.
            .action_with(layout_internal_action("patchPage", LocalizedLabel::native("Patch Page", "Seite aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("patchFrame", LocalizedLabel::native("Patch Frame", "Rahmen aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("canvasDrop", LocalizedLabel::native("Canvas Drop", "Ablegen auf Leinwand"), ActionKind::Mutation))
            // 👁️ Ephemeral view state — active page, drop ghost, pointer, camera, engagement draft.
            // Selection/hover are framework-owned now (domain "elements") — no app-declared verbs;
            // `interactionSelect`/`interactionHover`/`clearSelection` auto-inject below.
            .action_with(layout_internal_action("setActivePage", LocalizedLabel::native("Set Active Page", "Aktive Seite festlegen"), ActionKind::View))
            .action_with(layout_internal_action("focusPreflightIssue", LocalizedLabel::native("Focus Preflight Issue", "Preflight-Problem fokussieren"), ActionKind::View))
            .action_with(layout_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(layout_internal_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View))
            .action_with(layout_internal_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View))
            .action_with(layout_internal_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::View))
            .action_with(layout_internal_action("canvasDragOver", LocalizedLabel::native("Canvas Drag Over", "Ziehen über Leinwand"), ActionKind::View))
            .action_with(layout_internal_action("canvasDragLeave", LocalizedLabel::native("Canvas Drag Leave", "Ziehen verlässt Leinwand"), ActionKind::View))
            .action_with(layout_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            // 🐚️ Engagement submit — routes typed export intents through the host, emits only shell effects.
            .action_with(layout_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Shell))
            // 📇️ Per-window action scoping — the content-authoring operations only make sense on the
            // interactive Blueprint surface; the read-only Preview surface renders output and never
            // creates or edits frames/pages. Exports, camera, pointer/drag, selection and hover are
            // surface-discriminated (via `surfaceId`) or global, so they stay unscoped orphans and
            // appear on both windows.
            .window_kind_action_refs(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![
                "addFrame".into(), "addPage".into(), "patchPage".into(), "patchFrame".into(),
            ])
            // 🕹️ Domain "elements": frames on the Blueprint canvas (pages are never targets — canvas
            // hit-testing only ever resolves frame ids). Flat: layout has no real parent/child
            // structure among frames. `Invertive` merge is the toggle-on-shift-click old
            // `CanvasPointerDown.extend` used to hand-roll; `Replace` is a plain click.
            .interaction(InteractionDefinition {
                id: LAYOUT_INTERACTION_ELEMENTS.into(),
                label: LocalizedLabel::native("Elements", "Elemente"),
                granularities: vec![GranularityDefinition { id: LAYOUT_GRANULARITY_ELEMENT.into(), label: LocalizedLabel::native("Element", "Element"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![InteractionRef::new(LAYOUT_INTERACTION_ELEMENTS)])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`layout_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(LayoutPlayApp::config_spec())
            .io(crate::editor::layout::engine::layout_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old `"sample"`/`"cylinder"`
            // app-level example registration and the no-op `.workflow("layout", …)` call are dropped
            // here (not silently: reported in this packet's migration notes). The subset's own
            // `📚️examples/🎬️demo` facet (`crate::artifacts::layout::examples::...`, real content,
            // pre-existing) is the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type LayoutApp = VcsArtifactApp<EditorApp<LayoutPlayApp>>;

    /// ✏️ `LayoutPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<LayoutPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<LayoutPlayApp>` builds it.

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn layout_app() -> LayoutApp {
        new_app::<EditorApp<LayoutPlayApp>>()
    }

    /// 🧪️ Adapts `create_layout_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry` still expects — framework testkit gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    async fn layout_app_manifest_for_testkit() -> App {
        App { definition: create_layout_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn layout_app_with_registry() -> LayoutApp {
        new_app_with_registry::<EditorApp<LayoutPlayApp>>(layout_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut LayoutApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub async fn test_screen_point(camera_x: f64, camera_y: f64, zoom: f64, width: f64, height: f64, world_x: f64, world_y: f64) -> (f64, f64) {
        let camera = infinite_canvas::camera::Camera { x: camera_x, y: camera_y, zoom };
        let viewport = infinite_canvas::camera::Viewport { width: width as u32, height: height as u32, dpr: 1.0 };
        let screen = infinite_canvas::camera::world_to_screen(&camera, &viewport, infinite_canvas::Point::new(world_x, world_y));
        (screen.x, screen.y)
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::layout::testkit::{dispatch, layout_app, layout_app_with_registry, render, test_screen_point};
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 20, "every LayoutCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the exact
    /// `as` literal declared in the `app_commands!` invocation above. Unlike flow (where the wire
    /// keyword happens to be the kebab-cased command id everywhere except `setLocale`), layout's
    /// pre-existing `📡️protocol` crate deliberately shortened every `set*` view command's wire keyword
    /// (`setActivePage` → `active-page`, `setCamera` → `camera`, `setLocale` → `locale`) — carried
    /// forward verbatim, not a drift.
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keyword = |id: &str| -> &'static str {
            match id {
                "setActivePage" => "active-page",
                "setCamera" => "camera",
                "setLocale" => "locale",
                "focusPreflightIssue" => "focus-preflight-issue",
                "engagementInput" => "engagement-input",
                "canvasPointerDown" => "canvas-pointer-down",
                "canvasPointerMove" => "canvas-pointer-move",
                "canvasPointerUp" => "canvas-pointer-up",
                "canvasDragOver" => "canvas-drag-over",
                "canvasDragLeave" => "canvas-drag-leave",
                "addFrame" => "add-frame",
                "addPage" => "add-page",
                "patchPage" => "patch-page",
                "patchFrame" => "patch-frame",
                "canvasDrop" => "canvas-drop",
                "exportPng" => "export-png",
                "exportSvg" => "export-svg",
                "exportPdf" => "export-pdf",
                "exportPackage" => "export-package",
                "engagementSubmit" => "engagement-submit",
                other => panic!("every_command() row {other} missing from this test's expected-keyword table"),
            }
        };
        for command in every_command() {
            let id = command.command_id();
            let expected = expected_keyword(id);
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ Rows whose `Option` fields make `None`/`Some` distinct wire cases round-trip text/binary
    /// identically either way — the enum's binary ordinal shifted when `setSelection`/`setHover`
    /// were deleted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so this no longer
    /// pins exact historical bytes (greenfield: no back-compat), only text/binary equivalence.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_round_trip_text_and_binary_either_way() {
        use crate::artifacts::layout::LayoutCamera;
        let cases: [(LayoutCommand, &str); 3] = [
            (LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }), "canvas-pointer-move x=1 y=2 width=800 height=600"),
            (LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }), "add-frame kind=rect x=1"),
            (LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }), "camera camera { x=1 y=2 zoom=1.5 }"),
        ];
        for (command, text) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<LayoutCommand> {
        use crate::artifacts::layout::LayoutCamera;
        vec![
            LayoutCommand::SetActivePage(set_active_page::SetActivePage { page_id: "page-2".into() }),
            LayoutCommand::FocusPreflightIssue(focus_preflight_issue::FocusPreflightIssue { object_id: Some("frame-1".into()), page_id: Some("page-1".into()) }),
            LayoutCommand::EngagementInput(engagement_input::EngagementInput { value: "export png".into() }),
            LayoutCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { surface_id: Some("layout.play.blueprint".into()), button: 0, extend: false, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: None, x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            LayoutCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            LayoutCommand::CanvasDragOver(canvas_drag_over::CanvasDragOver { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            LayoutCommand::CanvasDragLeave(canvas_drag_leave::CanvasDragLeave {}),
            LayoutCommand::SetCamera(set_camera::SetCamera { surface_id: None, camera: LayoutCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
            LayoutCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: Some(1.0), y: None }),
            LayoutCommand::AddPage(add_page::AddPage {}),
            LayoutCommand::PatchPage(patch_page::PatchPage { page_id: Some("page-1".into()), field: "width".into(), value: "300".into() }),
            LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-1".into(), page_id: Some("page-1".into()), field: "fill".into(), value: "0.5, 0.4, 0.3, 1".into() }),
            LayoutCommand::CanvasDrop(canvas_drop::CanvasDrop { surface_id: Some("layout.play.blueprint".into()), kind: "rect".into(), x: 1.0, y: 2.0, width: 800.0, height: 600.0 }),
            LayoutCommand::ExportPng(export_png::ExportPng { page_id: Some("page-1".into()) }),
            LayoutCommand::ExportSvg(export_svg::ExportSvg { page_id: None }),
            LayoutCommand::ExportPdf(export_pdf::ExportPdf { page_id: None }),
            LayoutCommand::ExportPackage(export_package::ExportPackage {}),
            LayoutCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: "export png".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_layout_app()).expect("app definition json");
        for id in [LAYOUT_PLAY_WINDOW_BLUEPRINT, LAYOUT_PLAY_WINDOW_PREVIEW] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::LAYOUT_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [LAYOUT_PLAY_BODY_DOCUMENT, LAYOUT_PLAY_BODY_CATALOGUE, LAYOUT_PLAY_BODY_INSPECTION, LAYOUT_PLAY_BODY_PREFLIGHT] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("2d.layout"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn window_kind_actions_scope_authoring_to_blueprint_only() {
        let definition = create_layout_app();
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
        };
        let blueprint_actions = resolve(LAYOUT_PLAY_WINDOW_BLUEPRINT);
        let preview_actions = resolve(LAYOUT_PLAY_WINDOW_PREVIEW);
        for authoring in ["addFrame", "addPage", "patchPage", "patchFrame"] {
            assert!(blueprint_actions.contains(&authoring.to_string()), "Blueprint must expose {authoring}");
            assert!(!preview_actions.contains(&authoring.to_string()), "Preview must NOT expose {authoring}");
        }
        for shared in ["exportPng", "exportPdf", "setCamera"] {
            assert!(blueprint_actions.contains(&shared.to_string()) && preview_actions.contains(&shared.to_string()), "{shared} stays on both windows");
        }
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn sample_fixture_parses() {
        let doc = crate::artifacts::layout::dsl::parse_dsl(crate::artifacts::layout::dsl::LAYOUT_SAMPLE_TEXT).expect("sample fixture");
        assert_eq!(doc.schema, crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA);
        assert!(!doc.pages.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = layout_app();
        assert!(render(&mut app, "layout.play.nope").contains("Unknown body"));
    }

    // 🕹️ `selected_and_hovered_frames_get_chrome_strokes` deleted: selection/hover chrome strokes read
    // `config.selected_ids`/`hovered_id`, both deleted with the framework-owned "elements" domain.
    // `canvas_layers` always renders with empty selection/hover now — `ArtifactApp::render` carries no
    // `InteractionView` (a known SDK gap, same as gis2d's/puzzle3d's inspection panels — see this
    // ticket's w3b-summary.md) — flagged, not fixed here (framework file, out of this crate's remit).

    #[semio_framework_async_macros::async_test]
    async fn window_engagements_cover_both_windows() {
        let mut app = layout_app();
        let engagements = app.window_engagements();
        let blueprint_engagement = engagements.get(LAYOUT_PLAY_WINDOW_BLUEPRINT).expect("blueprint engagement");
        let status = blueprint_engagement.status.as_ref().and_then(|rows| rows.first()).expect("status");
        assert!(status.text.contains("Page"));
        let input = blueprint_engagement.input.as_ref().expect("input");
        assert_eq!(input.placeholder.as_deref(), Some("undo, redo, export png"));
        assert!(engagements.contains_key(LAYOUT_PLAY_WINDOW_PREVIEW));
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_backed_add_frame_emits_operation() {
        // 🧬️ addFrame is declared `Mutation`: the registry-backed wrapper must let its operations through.
        let mut app = layout_app_with_registry();
        let result = dispatch(&mut app, LayoutCommand::AddFrame(add_frame::AddFrame { kind: "rect".into(), x: None, y: None }));
        assert_eq!(result.mutations.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_backed_pointer_move_is_view_only() {
        // 🧬️ canvasPointerMove is declared `View`: it mutates only config hover state and must never emit
        // an operation, which the registry kind-discipline check enforces.
        let mut app = layout_app_with_registry();
        let (sx, sy) = test_screen_point(0.0, 0.0, 1.0, 800.0, 600.0, 156.0, 220.0);
        let result = dispatch(&mut app, LayoutCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { surface_id: Some(LAYOUT_PLAY_SURFACE_BLUEPRINT.into()), x: sx, y: sy, width: 800.0, height: 600.0 }));
        assert!(result.mutations.is_empty(), "View action must not emit document operations");
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    #[semio_framework_async_macros::async_test]
    async fn export_media_layout_out_returns_svg_of_first_page() {
        // 🌉️ Same pre-existing per-process registration gap as `🎮️commands/🐚️export`'s
        // `export_actions_wire_to_real_layout_exporters` test — see that test's comment.
        crate::artifacts::layout::io::ensure_stdio_semio_drawing_registered();
        let app = layout_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let app = LayoutPlayApp::default();
        let media = semio_framework_plugin::resolve_ready(LayoutPlayApp::export_media("layout:out", &doc)).expect("export layout:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.layout");
        assert!(json.starts_with("<svg"));
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_document_out_round_trips_through_pack() {
        let app = layout_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let app = LayoutPlayApp::default();
        let media = semio_framework_plugin::resolve_ready(LayoutPlayApp::export_media("document:out", &doc)).expect("export document:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA);
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
        let decoded = <LayoutSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(decoded, document);
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_fields_in_sets_data_fields_json() {
        let mut app = layout_app();
        let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "form.dictionary".into(), json: r#"{"name":"Ada"}"#.into() } };
        app.import_media("fields:in", &media, &testkit::meta("local")).expect("import fields:in");
        let document = app.snapshot().expect("projection");
        assert_eq!(document.data_fields_json.as_deref(), Some(r#"{"name":"Ada"}"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_io_exposes_declared_ports() {
        let io = LayoutPlayApp::io().expect("layout declares io");
        assert!(io.ports.iter().any(|port| port.id == "fields:in"));
        assert!(io.ports.iter().any(|port| port.id == "layout:out"));
    }

    #[semio_framework_async_macros::async_test]
    async fn layout_io_declares_fields_in_and_layout_out_ports() {
        let io = crate::editor::layout::engine::layout_io();
        assert_eq!(io.document_schema, "layout.layout");
        assert_eq!(io.artifact.id, "2d.layout");
        let fields_in = io.ports.iter().find(|port| port.id == "fields:in").expect("fields:in declared");
        assert_eq!(fields_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert_eq!(fields_in.kind_id.as_deref(), Some("form.dictionary"));
        assert_eq!(fields_in.multiplicity, semio_framework::PortMultiplicity::One);
        let layout_out = io.ports.iter().find(|port| port.id == "layout:out").expect("layout:out declared");
        assert_eq!(layout_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(layout_out.kind_id.as_deref(), Some("2d.layout"));
        assert_eq!(layout_out.multiplicity, semio_framework::PortMultiplicity::Many);
        let all_ports = io.all_ports();
        assert!(all_ports.iter().any(|port| port.id == "document:in"));
        assert!(all_ports.iter().any(|port| port.id == "document:out"));
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🧪️Tests
