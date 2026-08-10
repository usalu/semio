//! 🖥️ FEM 3D play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config`, shared compute in the artifact's
//! `⚙️engine`. This file is a routing table: `handle` → `Fem3dCommand::dispatch`, `render` → body-key →
//! window, and a `🔖️Manifest` region that calls one passthrough per node (scalar `.mode(..)`/
//! `.window_kind(..)` calls stay inline — fem3d builds neither a `ModeDefinition` nor a
//! `WindowKindDefinition` object anywhere, see `modes::edit`'s and the window nodes' own doc comments).

use crate::apps::fem3d::commands::{analysis, camera, example, loads, model, results, selection};
use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::apps::fem3d::modes::edit;
use crate::apps::fem3d::modes::edit::windows::{model as window_model, results as window_results};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::model::{Dof, ElementResult};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    create_default_layout, ActionArgDef, ActionArgOption, App, AppIo, ConfigSpec, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, SurfaceKind, UiNode,
};
use store::EngineHandles;
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const FEM3D_APP_ID: &str = "fem3d-play";

/// 📦️ The `fem3d-play` "default" example, embedded via `crate::artifacts::fem3d::dsl` — shared by the
/// manifest's `.example(...)` registration, the `setActiveExample` handler, and every test fixture.
const FEM3D_EXAMPLE_DSL: &str = crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem3dPlayApp::Command` — the SOLE dispatch surface for fem3d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies for 3 of these 18
    /// rows: `setActiveExample`/`active-example`, `setCamera`/`camera`, `setResultDisplay`/
    /// `result-display`. **Row order is the binary variant ordinal: appending is safe, reordering is a
    /// wire-format break.** Unlike fem2d, there is NO `setLocale`/`SetLocale` row — fem3d's pre-migration
    /// `Fem3dCommand` enum never had one (a pre-existing, intentional asymmetry between the two apps).
    pub enum Fem3dCommand for Fem3dSnapshot, Fem3dMutation, Fem3dConfig, Fem3dConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addSolid" as "add-solid" => add_solid::AddSolid,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use example::set_active_example;
use loads::{add_area_load, add_combination, add_load_case, add_member_udl, add_nodal_load, set_self_weight};
use model::{add_bar, add_frame, add_material, add_node, add_section, add_solid, add_support};
use analysis::set_analysis_settings;
use camera::set_camera;
use results::set_result_display;
use selection::remove_selection;
//#endregion 🔖️Commands

//#region 🔖️Fem3dResultsJson
/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (the `🫀️core` kernel is
/// a cross-artifact shared crate, out of scope to touch here), so this hand-rolls the same shape
/// `serde_json::to_string` would have produced, using `Dof`'s existing `{:?}` formatting. Single
/// consumer (`export_media`), so this lives here rather than in the artifact's `⚙️engine`.
fn fem3d_dof_json(dof: Dof) -> Value {
    json!(format!("{dof:?}"))
}

fn fem3d_element_result_json(result: &ElementResult) -> Value {
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

fn fem3d_static_result_json(result: &crate::model::StaticResult) -> Value {
    json!({
        "displacements": result.displacements.iter().map(|d| json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| json!({ "nodeId": r.node_id, "dof": fem3d_dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| json!({ "id": id, "result": fem3d_element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn fem3d_results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), fem3d_static_result_json(result))).collect())
}
//#endregion 🔖️Fem3dResultsJson

//#region 🔖️Fem3dPlayApp
/// 🧮️ v0 design: results are recomputed fresh inside `render()`, no cache, no `RunAnalysis` operation.
/// Unit struct — every former `RefCell` field lives in `Fem3dConfig`, written through
/// `Fem3dConfigMutation`s.
#[derive(Default)]
pub struct Fem3dPlayApp;

impl ArtifactApp for Fem3dPlayApp {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Config = Fem3dConfig;
    type ConfigMutation = Fem3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::apps::fem3d::presence::Fem3dPresence;
    type PresenceMutation = crate::apps::fem3d::presence::Fem3dPresenceMutation;

    type Command = Fem3dCommand;

    const APP_ID: &'static str = FEM3D_APP_ID;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem3d::FEM_3D_SCHEMA;

    fn initial_snapshot() -> Fem3dSnapshot {
        crate::artifacts::fem3d::engine::empty_fem3d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(crate::artifacts::fem3d::engine::fem3d_io())
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one). `"results:out"` runs every load case/combination's analysis fresh and returns them as plain
    /// JSON text in a `Structured` payload. A document with no load cases, or a solve failure, is
    /// reported as `MediaError::Payload` rather than an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = Self::io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = <Fem3dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::artifacts::fem3d::engine::fem3d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = fem3d_results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem3d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(snapshot: Fem3dSnapshot) -> Option<Fem3dMutation> {
        Some(Fem3dMutation::SetSnapshot { snapshot: snapshot })
    }

    /// 🎞️ `"document:in"` reproduces the trait's default whole-document-pack importer (overriding
    /// `import_media` shadows it for every port). `"geometry:in"` decodes a minimal, app-owned
    /// `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...], "baseZ"?: f64, "height"?: f64, "layers"?:
    /// usize}` extruded-footprint contract into a new `FemSolid`, defaulted to the document's first
    /// existing material if any, else an `"unassigned"` placeholder id — the solid simply won't solve
    /// until a real material is assigned.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match Self::whole_document_operation(snapshot) {
                    Some(operation) => Ok(Emit::mutations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
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
                let base_z = value.get("baseZ").and_then(Value::as_f64).unwrap_or(0.0);
                let height = value.get("height").and_then(Value::as_f64).unwrap_or(1.0);
                let layers = value.get("layers").and_then(Value::as_u64).map(|v| v as usize).unwrap_or(1);
                let material_id = doc.snapshot.materials.first().map(|material| material.id.clone()).unwrap_or_else(|| "unassigned".into());
                let id = crate::app_surface::next_id(doc.snapshot.solids.iter().map(|s| s.id.clone()), "sol");
                let index = doc.snapshot.solids.len();
                let solid = crate::artifacts::fem3d::FemSolid { id, name: "Imported Geometry".into(), outline, holes, base_z, height, layers, mesh_size: 0.5, material_id };
                Ok(Emit::mutations(vec![Fem3dMutation::SetSolid { index, solid }]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧮️ No sticky `ActionArgDef` defaults are mirrored here (all of `addSolid`'s
    /// `baseZ`/`layers`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings).
    fn config_spec() -> ConfigSpec {
        ConfigSpec::empty()
    }

    fn command_id(command: &Fem3dCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &Fem3dCommand, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, Fem3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, Fem3dConfig>) -> UiNode {
        let camera = &cfg.snapshot.camera;
        match body_key {
            window_model::FEM3D_BODY_MODEL => window_model::render(doc.snapshot, camera),
            window_results::FEM3D_BODY_RESULTS => window_results::render(doc.snapshot, cfg.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Fem3dPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node. fem3d's mode/windows are all scalar
/// (`.mode(..)`/`.window_kind(..)`) declarations — no `_def` passthrough exists for them since no
/// `ModeDefinition`/`WindowKindDefinition` object is built anywhere (see `modes::edit`'s doc comment).
pub fn create_fem3d_app() -> App {
    App::from_builder(
        App::builder(FEM3D_APP_ID, LocalizedLabel::data("FEM 3D"))
            .document(["semio", "fem", "fem3d"])
            .artifact_kind(crate::artifacts::fem3d::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(window_model::FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), window_model::FEM3D_BODY_MODEL, SurfaceKind::World3d, "fem-model")
            .window_kind(window_results::FEM3D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), window_results::FEM3D_BODY_RESULTS, SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[window_model::FEM3D_WINDOW_MODEL.into(), window_results::FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("z", LocalizedLabel::data("Z")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", LocalizedLabel::native("Solid", "Volumenkörper")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addSolid", LocalizedLabel::native("Add Solid", "Volumenkörper hinzufügen"))
            .action_args("addSolid", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("depth", LocalizedLabel::native("Depth", "Tiefe")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::data("Material")).required(),
                ActionArgDef::number("baseZ", LocalizedLabel::native("Base Z", "Basis Z")).default_value(0.0),
                ActionArgDef::number("layers", LocalizedLabel::native("Layers", "Schichten")).default_value(1),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(0.5),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(false),
            ])
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::text("terms", LocalizedLabel::native("Terms", "Terme")).required(),
            ])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Moden")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Beulmoden")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard"))]).default_value("default"),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            // 🎯️ Typed channel surface — `config_spec()`/`fem3d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem3dPlayApp::config_spec())
            .io(crate::artifacts::fem3d::engine::fem3d_io()),
    )
    .example("default", LocalizedLabel::native("Family House", "Einfamilienhaus"), FEM3D_EXAMPLE_DSL, "file")
    .workflow("fem3d", "FEM 3D", "structure")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{ArtifactApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Fem3dApp = VcsArtifactApp<Fem3dPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn fem3d_app() -> Fem3dApp {
        new_app::<Fem3dPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn fem3d_app_with_registry() -> Fem3dApp {
        new_app_with_registry::<Fem3dPlayApp>(create_fem3d_app)
    }

    pub fn dispatch(app: &mut Fem3dApp, command: Fem3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Fem3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{dispatch, fem3d_app, Fem3dApp};
    use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the exact
    /// fixture values the pre-migration `fem3d_protocol` crate's own `Fem3dCommand` test used.
    fn every_command() -> Vec<Fem3dCommand> {
        vec![
            Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }),
            Fem3dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
            Fem3dCommand::AddFrame(add_frame::AddFrame { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "hea200".into(), roll: 0.5 }),
            Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11, g: 8.077e10 }),
            Fem3dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369, iz: 0.0000133, j: 0.0000006 }),
            Fem3dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: crate::artifacts::fem3d::FemDof::ALL.to_vec() }),
            Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: Some("live".into()) }),
            Fem3dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: 0.0, wz: -500.0, case_id: None }),
            Fem3dCommand::AddAreaLoad(add_area_load::AddAreaLoad { solid_id: "sol1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
            Fem3dCommand::AddSolid(add_solid::AddSolid { x: 0.0, y: 0.0, width: 4.0, depth: 2.0, height: 0.5, material_id: "concrete".into(), base_z: Some(0.0), layers: Some(2), mesh_size: None }),
            Fem3dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
            Fem3dCommand::AddCombination(add_combination::AddCombination { name: "ULS".into(), terms: "[[\"dead\",1.35],[\"live\",1.5]]".into() }),
            Fem3dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
            Fem3dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
            Fem3dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
            Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            Fem3dCommand::SetCamera(set_camera::SetCamera { json: "{\"x\":1}".into() }),
            Fem3dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 18, "every Fem3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 📌️ LAW: the pre-migration command wire format, row for row — the hex list is positionally aligned
    /// to `every_command()`, which carries exactly the values the old `📡️protocol` crate's baseline dump
    /// used (ticket `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-3d.txt`). Row order is the binary variant ordinal, so a reordering — which
    /// no round-trip law can catch — shows up here as a leading-byte mismatch. `addNodalLoad`'s `None`
    /// case is pinned separately below because `every_command()` only carries its `Some` shape.
    #[test]
    fn every_command_keeps_its_pre_migration_bytes() {
        use protocol::OpBinary;
        let expected = [
            "010000030005000000000000f03f0105000000000000004002050000000000000840",
            "010104026e31026e3203726f6405737465656c04000600010601020603030602",
            "01020406686561323030026e31026e3205737465656c050006010106020206030306000405000000000000e03f",
            "01030105537465656c030006000105000000da7c72484202050000806444ce3242",
            "0104010648454132303005000600010545f5d6c05609763f020554fc8458a258033f0305210ec81462e4eb3e040576830df4f521a43e",
            "010501026e310200060001160600020406080a",
            "010602046c697665026e3104000601010a020205000000000088b3c0030600",
            "01070102653104000600010500000000000000000205000000000000000003050000000000407fc0",
            "010802046465616404736f6c31030006010105000000000088b340020600",
            "01090108636f6e637265746508000500000000000000000105000000000000000002050000000000001040030500000000000000400405000000000000e03f05060006050000000000000000070402",
            "010a01044c697665020006000101",
            "010b0203554c531c5b5b2264656164222c312e33355d2c5b226c697665222c312e355d5d02000600010601",
            "010c010464656164020006000102",
            "010d000200040502050000000000003e40",
            "010e02026531026e3101000c0206010600",
            "010f010764656661756c7401000600",
            "011001077b2278223a317d01000600",
            "0111020464656164056d6f64616c03000600010601020400",
        ];
        let commands = every_command();
        assert_eq!(commands.len(), expected.len(), "the baseline hex list must cover every command row");
        for (command, expected) in commands.iter().zip(expected) {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
        }
        let nodal_load_without_case = Fem3dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem3d::FemDof::Tz, value: -5000.0, case_id: None });
        assert_eq!(
            nodal_load_without_case.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
            "010601026e3103000600010a020205000000000088b3c0"
        );
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword. Three rows
    /// (`setActiveExample`/`setCamera`/`setResultDisplay`) prove the wire keyword is NOT simply the
    /// kebab-cased command id — this is exactly what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keys = [
            "add-node",
            "add-bar",
            "add-frame",
            "add-material",
            "add-section",
            "add-support",
            "add-nodal-load",
            "add-member-udl",
            "add-area-load",
            "add-solid",
            "add-load-case",
            "add-combination",
            "set-self-weight",
            "set-analysis-settings",
            "remove-selection",
            "active-example",
            "camera",
            "result-display",
        ];
        for (command, expected) in every_command().into_iter().zip(expected_keys) {
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {command:?}: {printed:?}");
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_fem3d_app().definition).expect("app definition json");
        for id in [window_model::FEM3D_WINDOW_MODEL, window_results::FEM3D_WINDOW_RESULTS] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
        assert!(json.contains("computation.fem3d"), "artifact kind missing from the manifest");
    }

    #[test]
    fn manifest_labels_resolve_german_3d() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_fem3d_app().definition;
        let window = definition.window_kinds.iter().find(|w| w.id == window_model::FEM3D_WINDOW_MODEL).expect("model window declared");
        assert_eq!(window.label.resolve(Terminology::Native, Locale::De), "Modell");
        let action = definition.actions.iter().find(|a| a.id == "addFrame").expect("addFrame declared");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), "Rahmen hinzufügen");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), "Add Frame");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn undo_restores_document_after_add_node() {
        let mut app = fem3d_app();
        let before = app.snapshot().expect("snapshot").nodes.len();
        assert_undo_redo_round_trip(&mut app, Fem3dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0, z: 3.0 }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1);
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::fem3d::testkit::render;
        let mut app = fem3d_app();
        assert!(render(&mut app, "fem3d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    /// 🎞️ `"results:out"` runs every load case fresh and returns a `Structured` JSON payload — build a
    /// doc with the bundled example (which has load cases), export, assert the JSON round-trips through
    /// `serde_json` and names a case id.
    #[test]
    fn export_media_results_out_returns_solved_json_for_every_case_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }));
        let snapshot = app.snapshot().expect("snapshot");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let media = Fem3dPlayApp::export_media("results:out", &doc).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a Structured payload") };
        assert_eq!(schema, "computation.fem3d");
        let value: Value = serde_json::from_str(&json).expect("results:out payload is valid JSON");
        assert!(value.get("dead").is_some(), "expected the example fixture's dead case in the results map: {json}");
        assert!(value["dead"].get("displacements").is_some(), "expected a displacements array: {json}");
    }

    /// 🎞️ `"results:out"` on a document with no load cases errors rather than panicking or returning an
    /// empty payload.
    #[test]
    fn export_media_results_out_errors_without_load_cases_3d() {
        let snapshot = crate::artifacts::fem3d::engine::empty_fem3d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let err = Fem3dPlayApp::export_media("results:out", &doc).expect_err("no load cases should error");
        assert!(matches!(err, MediaError::Payload(..)));
    }

    /// 🎞️ `"geometry:in"` decodes an extruded-footprint JSON contract into a new `FemSolid` operation.
    #[test]
    fn import_media_geometry_in_adds_a_new_solid_3d() {
        let mut app: Fem3dApp = fem3d_app();
        dispatch(&mut app, Fem3dCommand::AddMaterial(add_material::AddMaterial { name: "Concrete".into(), e: 30e9, g: 12.5e9 }));
        let snapshot = app.snapshot().expect("snapshot");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let json = serde_json::json!({
            "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]],
            "holes": [],
            "baseZ": 0.5,
            "height": 3.0,
            "layers": 2,
        })
        .to_string();
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any }, payload: MediaPayload::Structured { schema: "geometry".into(), json } };
        let emit = Fem3dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        assert_eq!(emit.artifact_mutations.len(), 1);
        match &emit.artifact_mutations[0] {
            Fem3dMutation::SetSolid { solid, .. } => {
                assert_eq!(solid.outline, vec![[0.0, 0.0], [2.0, 0.0], [2.0, 1.0], [0.0, 1.0]]);
                assert_eq!(solid.base_z, 0.5);
                assert_eq!(solid.height, 3.0);
                assert_eq!(solid.layers, 2);
                assert_eq!(solid.material_id, "m0");
            }
            _ => panic!("expected SetSolid"),
        }
    }

    #[test]
    fn fem3d_io_matches_declared_artifact_identity_3d() {
        let io = Fem3dPlayApp::io().expect("fem3d declares typed media I/O");
        assert_eq!(io.artifact.id, "3d.fem");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🧪️Tests
