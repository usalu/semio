//! 🖥️ Fem2d play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, view state in `🎚️config`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `Fem2dCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one passthrough per node (fem2d's mode/window declarations stay
//! scalar/inline — no `mode_def`/`window_kind_def` object is built anywhere in the pre-migration code).

use crate::app_surface::{DisplayMode, ResultDisplay};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::editor::fem2d::commands::{
    add_area_load, add_bar, add_beam, add_combination, add_load_case, add_material, add_member_udl, add_nodal_load, add_node, add_region, add_section, add_support, remove_selection, set_active_example, set_analysis_settings, set_camera, set_locale,
    set_result_display, set_self_weight,
};
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::editor::fem2d::modes::edit;
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use crate::model::{Dof, ElementResult};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{
    create_default_layout, ui_text, ActionArgDef, ActionArgOption, AppIo, ArtifactEditor, ArtifactView, ConfigSpec, ConfigView, DraftView, Editor, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType,
    NoDraft, NoDraftMutation, SurfaceKind, UiNode,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const FEM2D_APP_ID: &str = "fem2d-play";

/// 📦️ The `fem2d-play` "default" example — read directly by the `setActiveExample` handler
/// (`crate::editor::fem2d::commands::set_active_example`) and every test fixture (`EditorBuilder` has
/// no `.example(...)` registration — see the SDK-gap doc comment on `create_fem2d_app` below).
pub const FEM2D_EXAMPLE_DSL: &str = crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem2dPlayApp::Command` — the SOLE dispatch surface for fem2d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies; `"setActiveExample" as
    /// "active-example"` and `"setCamera" as "camera"` are two of the rows that prove it. **Row order is
    /// the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Fem2dCommand for Fem2dSnapshot, Fem2dMutation, Fem2dConfig, Fem2dConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addBeam" as "add-beam" => add_beam::AddBeam,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addRegion" as "add-region" => add_region::AddRegion,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️ExportImportHelpers
/// 👁️ B1: `cfg`-driven counterpart of the deleted `ResultDisplay` `RefCell` — converts the flat
/// `Fem2dConfig` result-display fields back into `crate::app_surface::ResultDisplay`/`DisplayMode` so
/// the results window's render pipeline (built around those shared types) needs no changes.
async fn config_result_display(cfg: &Fem2dConfig) -> ResultDisplay {
    let mode = match cfg.result_mode.as_str() {
        "modal" => DisplayMode::Modal(cfg.result_mode_index as usize),
        "buckling" => DisplayMode::Buckling(cfg.result_mode_index as usize),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}

/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (out of this ticket's
/// scope: `🫀️core` is a shared crate), so this hand-rolls the same shape `serde_json::to_string` would
/// have produced, using `Dof`'s existing `{:?}` formatting (already used for the reaction-label layers
/// in the results window's render).
async fn dof_json(dof: Dof) -> Value {
    json!(format!("{dof:?}"))
}

async fn element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            json!({ "kind": "beam", "stations": stations.iter().map(|s| json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            json!({ "kind": "plane", "gauss": gauss.iter().map(|g| json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            json!({ "kind": "plate", "gauss": gauss.iter().map(|g| json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

async fn static_result_json(result: &crate::model::StaticResult) -> Value {
    json!({
        "displacements": result.displacements.iter().map(|d| json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| json!({ "nodeId": r.node_id, "dof": dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| json!({ "id": id, "result": element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

async fn results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), static_result_json(result))).collect())
}
//#endregion 🔖️ExportImportHelpers

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.2d` × 2D-Vector) plus `geometry:in` (importing an externally authored 2D outline as a new
/// `FemRegion` — see `import_media` above) and `results:out` (every load case/combination's solved
/// `crate::model::StaticResult`, pinned to the `computation.fem2d` artifact kind declared in
/// `crate::artifacts::fem2d::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub async fn fem2d_io() -> AppIo {
    AppIo {
        document_schema: crate::artifacts::fem2d::FEM_2D_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![fem2d_geometry_in_port(), fem2d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.fem".into(), name: "FEM 2D".into(), dimension: "2d".into(), component_kind: "fem2d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored 2D polygon-with-holes outline, imported as a new
/// `FemRegion`.
pub async fn fem2d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem2d` artifact kind.
pub async fn fem2d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem2d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🔖️Fem2dPlayApp
/// 🧪️ B1: unit struct — every former `Fem2dPlayApp` `RefCell` field (`result_display`, `camera`) plus
/// the deleted `ViewModel::locale` now live in `crate::editor::fem2d::config::Fem2dConfig`, written
/// through `Fem2dConfigMutation`s. v0 design unchanged: results are never persisted or cached —
/// `fem2d_solve`/`fem2d_solve_all` run fresh inside `render()`/`export_media` whenever the results
/// window is drawn or the `"results:out"` port is read.
#[derive(Default)]
pub struct Fem2dPlayApp;

impl ArtifactEditor for Fem2dPlayApp {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Config = Fem2dConfig;
    type ConfigMutation = Fem2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::fem2d::presence::Fem2dPresence;
    type PresenceMutation = crate::editor::fem2d::presence::Fem2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Fem2dCommand;

    const DIALECT: Dialect = crate::artifacts::fem2d::FEM2D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem2d::FEM_2D_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::fem2d::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> Fem2dSnapshot {
        crate::artifacts::fem2d::schema::empty_fem2d_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(fem2d_io())
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one). `"results:out"`
    /// runs every load case/combination's analysis fresh and returns them as plain JSON text in a
    /// `Structured` payload — `MediaPayload::Structured.json` doesn't require a `pack`-encoded value. A
    /// document with no load cases, or a solve failure, is reported as `MediaError::Payload` rather than
    /// an empty/panicking export.
    async fn export_media(port: &str, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = Self::io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::fem2d_engine::fem2d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem2d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override on this impl — per `📓️taxonomy.md`, whole-document
    /// replace (`SetSnapshot`) is banned outright with NO replacement mutation, so this falls back to
    /// the trait's own default (`None`).
    ///
    /// 🎞️ `"document:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment) instead of routing through `whole_document_operation`.
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...]}`
    /// polygon-with-holes contract into a new `FemRegion` via `create-region`, defaulted to the
    /// document's first existing material if any, else an `"unassigned"` placeholder id.
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = serde_json::from_value(value.get("outline").cloned().unwrap_or(Value::Null)).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => serde_json::from_value(holes_value).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let material_id = doc.snapshot.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = crate::app_surface::next_id(doc.snapshot.regions.iter().map(|r| r.id.clone()), "r");
                let region = crate::artifacts::fem2d::FemRegion { id, name: "Imported Geometry".into(), outline, holes, thickness: 0.02, material_id, mesh_size: 0.25 };
                Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::mutation::CreateRegion { region })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &Fem2dCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &Fem2dCommand,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, Fem2dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎯️ Fem2d has no user-visible config defaults to expose (all of `addRegion`'s
    /// `thickness`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings) — declaring `ConfigSpec::empty()` explicitly keeps the typed channel surface
    /// consistent with the sibling apps' convention.
    async fn config_spec() -> ConfigSpec {
        ConfigSpec::empty()
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, Fem2dConfig>) -> UiNode {
        let camera = &cfg.snapshot.camera;
        match body_key {
            model_window::BODY_KEY => model_window::render(doc.snapshot, camera),
            results_window::BODY_KEY => results_window::render(doc.snapshot, &config_result_display(cfg.snapshot), camera),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Fem2dPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"document:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub async fn reset_document_effect(scene: &Fem2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("fem2d document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🚧️ SDK GAP (contract §2.4, matching the cad pilot's identical note): `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods — `.editor::<E>(def: AppDefinition)` only takes the bare
/// definition, so the former `.example("default", …, FEM2D_EXAMPLE_DSL, "file")` and
/// `.workflow("fem2d", "FEM 2D", "structure")` registrations are dropped here, not ported. Every
/// consumer of the bundled example DSL (`setActiveExample`'s handler, every test fixture) still reads
/// `FEM2D_EXAMPLE_DSL` directly, so no behavior beyond the manifest-level example/workflow listing is
/// lost.
pub async fn create_fem2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::fem2d::FEM2D_DIALECT)
            .document(["semio", "fem", "fem2d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see
            // `fem2d_io` above) — deliberately a different `media_type`
            // (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`.
            .artifact_kind(crate::artifacts::fem2d::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(model_window::WINDOW_KIND_ID, LocalizedLabel::native("Model", "Modell"), model_window::BODY_KEY, SurfaceKind::Canvas2d, "fem-model")
            .window_kind(results_window::WINDOW_KIND_ID, LocalizedLabel::native("Results", "Ergebnisse"), results_window::BODY_KEY, SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[model_window::WINDOW_KIND_ID.into(), results_window::WINDOW_KIND_ID.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .mutation("addBeam", LocalizedLabel::native("Add Beam", "Balken hinzufügen"))
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", LocalizedLabel::native("Region", "Bereich")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addRegion", LocalizedLabel::native("Add Region", "Bereich hinzufügen"))
            .action_args("addRegion", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::number("thickness", LocalizedLabel::native("Thickness", "Dicke")).default_value(0.02),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(0.25),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
            ])
            // 🎯️ `terms` is `Fem2dCommand::AddCombination`'s typed `Vec<FemCombinationTerm>` — no single
            // `ActionArgDef` control maps to that shape, so (mirroring the sibling apps' precedent for
            // commands with no matching staged form) this action simply has no `.action_args(...)`
            // declaration.
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Eigenformen")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Knickformen")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))])
                    .default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 🎯️ Typed channel surface — `config_spec()`/`fem2d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem2dPlayApp::config_spec())
            .io(fem2d_io())
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

    pub type Fem2dApp = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn fem2d_app() -> Fem2dApp {
        new_app::<EditorApp<Fem2dPlayApp>>()
    }

    /// 🧪️ Adapts `create_fem2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry` still expects (SDK gap — mirrors the cad
    /// pilot's identical `cad_app_manifest_for_testkit` wrapper; `🧰️framework/**` is outside this
    /// packet's lease).
    async fn fem2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_fem2d_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn fem2d_app_with_registry() -> Fem2dApp {
        new_app_with_registry::<EditorApp<Fem2dPlayApp>>(fem2d_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut Fem2dApp, command: Fem2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut Fem2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{fem2d_app, render};
    use semio_framework_plugin::{ArtifactEditor, EditorApp, PluginApp};
    use store::ArtifactDsl;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<Fem2dCommand> {
        vec![
            Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }),
            Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
            Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
            Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
            Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }),
            Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }),
            Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) }),
            Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }),
            Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
            Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
            Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
            Fem2dCommand::AddCombination(add_combination::AddCombination {
                name: "ULS".into(),
                terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }],
            }),
            Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
            Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
            Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
            Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }),
            Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
            Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 19, "every Fem2dCommand row must be covered by every_command()");
        // 🏷️ Unlike flow's setLocale/flowEvalTick, every one of fem2d's 19 commands (including
        // setLocale) has a real manifest action declaration — see `create_fem2d_app`'s `.view_action`
        // calls.
        let definition = create_fem2d_app();
        for id in ids {
            assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == id), "command_id {id} must be a declared action");
        }
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 📌️ LAW: the pre-migration command wire format, row for row, INCLUDING both `Option` shapes of
    /// every row whose `None`/`Some` cases encode differently. Every hex string was dumped from the old
    /// `📡️protocol` crate's `Fem2dCommand` before `app_commands!` rebuilt the enum (ticket
    /// `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-2d.txt`). Row order is the binary variant ordinal, so a reordering — which
    /// no round-trip law can catch — shows up here as a leading-byte mismatch.
    #[semio_framework_async_macros::async_test]
    async fn every_command_keeps_its_pre_migration_bytes() {
        use protocol::OpBinary;
        let rows: Vec<(&str, Fem2dCommand)> = vec![
            ("010000020005000000000000f03f01050000000000000040", Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 })),
            ("010104026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
            ("010204026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
            ("01030105737465656c020006000105000000da7c724842", Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "steel".into(), e: 210e9 })),
            ("01040106697065333030030006000105a4005130630a763f020509c577de9de7153f", Fem2dCommand::AddSection(add_section::AddSection { name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 })),
            ("010501026e31020006000116020002", Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] })),
            ("010602046c697665026e3104000601010a010205000000000088b3c0030600", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) })),
            ("010601026e3103000600010a010205000000000088b3c0", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: None })),
            ("010701026531030006000105000000000000000002050000000000407fc0", Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None })),
            ("0108020464656164027231030006010105000000000088b340020600", Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) })),
            (
                "01090105737465656c060005000000000000000001050000000000000000020500000000000010400305000000000000004004060005057b14ae47e17a943f",
                Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
            ),
            ("010a01044c697665020006000101", Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })),
            (
                "010b0303554c530464656164046c69766502000600010c020d0200060101059a9999999999f53f0d020006020105000000000000f83f",
                Fem2dCommand::AddCombination(add_combination::AddCombination {
                    name: "ULS".into(),
                    terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }],
                }),
            ),
            ("010c010464656164020006000102", Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true })),
            ("010d000200040502050000000000003e40", Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) })),
            ("010e02026531026e3101000c0206010600", Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] })),
            ("010f010764656661756c7401000600", Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })),
            ("011000030005000000000000f03f010500000000000000400205000000000000f83f", Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })),
            ("0111020464656164056d6f64616c03000600010601020400", Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 })),
            ("0112010564652d444501000600", Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })),
        ];
        for (expected, command) in rows {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_fem2d_app()).expect("app definition json");
        for id in [model_window::WINDOW_KIND_ID, results_window::WINDOW_KIND_ID] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
        assert!(json.contains("computation.fem2d"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_spec_declares_no_fields() {
        assert!(Fem2dPlayApp::config_spec().fields.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_declares_config_io_and_computation_artifact_kind() {
        let definition = create_fem2d_app();
        assert!(definition.config.fields.is_empty());
        assert_eq!(definition.io.document_schema, crate::artifacts::fem2d::FEM_2D_SCHEMA);
        let computation_kind = definition.artifact_kinds.iter().find(|kind| kind.id == "computation.fem2d").expect("computation.fem2d artifact kind declared");
        assert_eq!(computation_kind.media_type.class, MediaClass::Computation);
        assert_eq!(computation_kind.media_type.form, MediaForm::Value);
    }

    #[semio_framework_async_macros::async_test]
    async fn app_io_forwards_the_engine_declared_ports() {
        let io = Fem2dPlayApp::io().expect("io declared");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }

    /// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
    /// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
    /// enforcement is exercised at the run-crate level.
    #[semio_framework_async_macros::async_test]
    async fn fem2d_io_declares_geometry_in_and_results_out_ports() {
        let io = fem2d_io();
        assert_eq!(io.document_schema, crate::artifacts::fem2d::FEM_2D_SCHEMA);
        assert_eq!(io.document_media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(io.document_media_type.form, semio_framework_plugin::MediaForm::Vector);
        assert_eq!(io.artifact.id, "2d.fem");
        assert_eq!(io.artifact.component_kind, "fem2d");

        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.required, "geometry:in is a required input port");
        assert_eq!(geometry_in.media_type.class, semio_framework_plugin::MediaClass::TwoD);
        assert_eq!(geometry_in.media_type.form, semio_framework_plugin::MediaForm::Vector);
        assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

        let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
        assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert!(!results_out.required, "results:out is optional");
        assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem2d"));
        assert_eq!(results_out.media_type.class, semio_framework_plugin::MediaClass::Data);
        assert_eq!(results_out.media_type.form, semio_framework_plugin::MediaForm::Value);
    }

    /// 🗣️ B1: the manifest itself (not a runtime `cfg.locale`-driven overlay) now carries every
    /// locale's translation via `LocalizedLabel`.
    #[semio_framework_async_macros::async_test]
    async fn manifest_labels_resolve_german_locale_2d() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_fem2d_app();
        let window_model = definition.window_kinds.iter().find(|window| window.id == model_window::WINDOW_KIND_ID).expect("model window kind declared");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::En), "Model");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::De), "Modell");
        let add_node_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "addNode").expect("addNode action declared");
        assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::En), "Add Node");
        assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::De), "Knoten hinzufügen");
        let set_locale_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setLocale").expect("setLocale action declared");
        assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::En), "Set Locale");
        assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::De), "Sprache festlegen");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_restores_document_after_add_node() {
        let mut app = fem2d_app();
        let before = app.snapshot().expect("snapshot").nodes.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 1.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = fem2d_app();
        assert!(render(&mut app, "fem2d.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_on_disjoint_edits() {
        let (mut instance_a, mut instance_b) = semio_framework_plugin::testkit::paired_apps::<EditorApp<Fem2dPlayApp>>("mem://fem2d-convergence");

        instance_a.dispatch_typed(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }), &semio_framework_plugin::testkit::meta("actor-a")).expect("a adds a material");
        instance_b.dispatch_typed(Fem2dCommand::AddNode(add_node::AddNode { x: 5.0, y: 5.0 }), &semio_framework_plugin::testkit::meta("actor-b")).expect("b adds a node");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        instance_a.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("snapshot a");
        let projection_b = instance_b.snapshot().expect("snapshot b");
        assert!(projection_a.materials.iter().any(|m| m.name == "Steel"), "A keeps its material");
        assert!(projection_a.nodes.iter().any(|n| n.x == 5.0), "A absorbs B's node");
        assert_eq!(projection_a.nodes.len(), projection_b.nodes.len(), "both instances converge to the same node set");
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️ConfigIo
    // (see `🔖️ManifestSanity` above for config/io declaration checks)
    //#endregion 🔖️ConfigIo

    //#region 🔖️ExportImportMedia
    /// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `📓️taxonomy.md`'s forbidden vocabulary), so `import_media("document:in")` now surfaces as a
    /// `Effect::LoadDocument` carrying the replacement document's pack bytes, not an
    /// `artifact_mutations` entry — asserted directly on `Emit` rather than through `app.snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn export_media_document_out_round_trips_via_import_media_document_in() {
        let _app = Fem2dPlayApp;
        let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let media = semio_framework_plugin::resolve_ready(Fem2dPlayApp::export_media("document:out", &doc)).expect("document:out exports");
        assert_eq!(media.media_type.class, MediaClass::TwoD);
        assert_eq!(media.media_type.form, MediaForm::Vector);
        let empty_projection = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let empty_history = semio_framework_plugin::HistoryView::empty();
        let empty_doc = ArtifactView::new(&empty_projection, &empty_history);
        let emit = Fem2dPlayApp::import_media("document:in", &media, &empty_doc).expect("document:in imports");
        assert!(emit.artifact_mutations.is_empty(), "whole-document replace must not be an artifact_mutations entry");
        let semio_framework::kernel::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("document:in must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded, snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_returns_json_with_every_case_and_combination() {
        let _app = Fem2dPlayApp;
        let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let media = semio_framework_plugin::resolve_ready(Fem2dPlayApp::export_media("results:out", &doc)).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "computation.fem2d");
                let value: Value = serde_json::from_str(&json).expect("results:out payload is valid JSON");
                for case_id in ["dead", "live", "uls"] {
                    let result = value.get(case_id).unwrap_or_else(|| panic!("missing {case_id} in results:out payload: {value}"));
                    assert!(result.get("displacements").is_some());
                    assert!(result.get("reactions").is_some());
                    assert!(result.get("checks").is_some());
                }
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_errors_when_no_load_cases_are_defined() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let error = semio_framework_plugin::resolve_ready(Fem2dPlayApp::export_media("results:out", &doc)).expect_err("no load cases means no results to export");
        match error {
            MediaError::Payload(port, _) => assert_eq!(port, "results:out"),
            other => panic!("expected MediaError::Payload, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_unknown_port_is_not_implemented() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        assert!(matches!(semio_framework_plugin::resolve_ready(Fem2dPlayApp::export_media("bogus:out", &doc)), Err(MediaError::NotImplemented)));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
        let _app = Fem2dPlayApp;
        let mut snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        snapshot.materials.push(crate::artifacts::fem2d::FemMaterial { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let payload = json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        assert_eq!(emit.artifact_mutations.len(), 1);
        match &emit.artifact_mutations[0] {
            Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::mutation::CreateRegion { region }) => {
                assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
                assert!(region.holes.is_empty());
                assert_eq!(region.material_id, "steel");
            }
            _ => panic!("expected CreateRegion"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_falls_back_to_unassigned_material_when_none_exists() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let payload = json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        match &emit.artifact_mutations[0] {
            Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::mutation::CreateRegion { region }) => assert_eq!(region.material_id, "unassigned"),
            _ => panic!("expected CreateRegion"),
        }
    }
    //#endregion 🔖️ExportImportMedia
}
//#endregion 🧪️Tests
