//! ✏️ BIM model editor: the `ArtifactEditor` implementation, the command table and the manifest stitch. The editor is a routing table: every command body lives in a `🎮️commands/*`
//! node, every window render in `🎭️modes/✏️edit/🪟️windows/*`, every panel in `📌️panels/*`, derived values in `🔮️inference`, what an entity kind is in `🧩️entities`. A command only
//! emits mutations (or window config, presence and effects); nothing here applies a diff.

use crate::editor::bim::commands::{arm_utility, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, create_entity, delete_selection, rename_entity, set_camera, set_field, set_view, world_pointer_down, world_pointer_move};
use crate::editor::bim::config::app_schema_descriptor;
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::modes::edit;
use crate::editor::bim::modes::edit::windows::{plan, schedule, section, world};
use crate::editor::bim::panels::{library as library_panel, outliner as outliner_panel, properties as properties_panel};
use crate::editor::bim::presence::{BimPresence, BimPresenceMutation};
use crate::editor::bim::terminology::{bim_labels, BimLabels};
use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
use crate::{ModelSnapshot, BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_artifact_reference::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::HashMap;

//#region 🔖️Constants
pub const BIM_EDITOR_APP_ID: &str = "s.bim.model@1/*#editor";
const BIM_RETAINED_PAYLOAD_SCHEMA: &str = "bim.model.tool-command.v1";
const BIM_BOUNDED_RAW_BYTES: usize = 65_536;
const BIM_BOUNDED_WORK_ITEMS: usize = 4_096;
//#endregion 🔖️Constants

//#region 🔖️Dispatch
/// 🕹️ Per-dispatch state that is neither document nor app config: the framework selection of both domains, the addressed window with its own config, the armed utility and the
/// local presence. The generated `dispatch` has no way to thread an `InteractionView`, so the editor builds this once per command and hands it down. Handlers push the presence
/// they want shared into `presence_out`.
#[derive(Default)]
pub struct BimDispatchCtx {
    pub selected: Vec<String>,
    pub library_selected: Vec<String>,
    pub view: Option<ViewModel>,
    pub window_kind: String,
    pub utility: String,
    pub plan: plan::config::BimPlanWindowConfig,
    pub world: world::config::BimWorldWindowConfig,
    pub section: section::config::BimSectionWindowConfig,
    pub presence: BimPresence,
    pub presence_out: Vec<BimPresenceMutation>,
    pub gestures: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub window_transient: crate::editor::bim::transient::BimWindowTransient,
    pub transient_out: Option<crate::editor::bim::transient::BimWindowTransient>,
}

impl BimDispatchCtx {
    /// 🕹️ The context of a command addressed at `view` with the config snapshot of its window.
    pub fn new(selected: Vec<String>, library_selected: Vec<String>, view: Option<&ViewModel>, window_config: Option<&semio_framework_plugin::WindowConfigSnapshot>, presence: Option<&BimPresence>) -> Self {
        Self {
            selected,
            library_selected,
            window_kind: view.and_then(crate::editor::bim::chrome::addressed_kind).unwrap_or_default().to_string(),
            utility: view.map_or(crate::editor::bim::utilities::DEFAULT_UTILITY, crate::editor::bim::utilities::active).to_string(),
            view: view.cloned(),
            plan: plan::config::from_snapshot(window_config),
            world: world::config::from_snapshot(window_config),
            section: section::config::from_snapshot(window_config),
            presence: presence.cloned().unwrap_or_default(),
            presence_out: Vec::new(),
            gestures: None,
            window_transient: Default::default(),
            transient_out: None,
        }
    }

    /// 🗣️ The labels of the addressed view; `None` when the command carries no view.
    pub fn labels(&self) -> Option<&'static BimLabels> {
        self.view.as_ref().map(bim_labels)
    }
}
//#endregion 🔖️Dispatch

//#region 🔖️CommandTable
/// 🧩️ THE command table: `manifest id as wire keyword => payload, [publication lanes]; kind, label (en, de), description (en, de)`. The command enum, the bounded tool roster, the
/// publication contracts, the bounded-first-step proofs and the manifest actions are all generated from these rows, so a new command is one row here, one payload node and one bridge arm.
macro_rules! bim_command_table {
    ($callback:ident) => {
        $callback! {
            "createEntity" as "create-entity" => create_entity::CreateEntity, [Artifact]; Mutation, ("Create Entity", "Bauteil anlegen"), ("Creates one entity of the given kind (site, building, storey, wall ...) in the given container, or in the selected or first container, and selects it.", "Legt ein Bauteil der angegebenen Art im angegebenen, im ausgewählten oder im ersten passenden Container an und wählt es aus.");
            "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection, [Artifact]; Mutation, ("Delete Selection", "Auswahl löschen"), ("Deletes the given entities, or the current selection, children before parents, in one undoable step.", "Löscht die angegebenen Bauteile oder die aktuelle Auswahl, Kinder vor Eltern, in einem rückgängig machbaren Schritt.");
            "renameEntity" as "rename-entity" => rename_entity::RenameEntity, [Artifact]; Mutation, ("Rename Entity", "Bauteil umbenennen"), ("Gives one entity a new name.", "Gibt einem Bauteil einen neuen Namen.");
            "setField" as "set-field" => set_field::SetField, [Artifact]; Mutation, ("Set Field", "Parameter festlegen"), ("Sets one authored parameter (name, level, height ...) of the given entities, or of the selection, to the given value.", "Setzt einen Parameter (Name, Ebene, Höhe ...) der angegebenen Bauteile oder der Auswahl auf den angegebenen Wert.");
            "setView" as "set-view" => set_view::SetView, [WindowConfig, Presence]; View, ("Set View", "Ansicht festlegen"), ("Sets one view parameter of the addressed window (storey, cut height, projection, isolated storey, section plane, section line, depth); the document does not change.", "Setzt einen Ansichtsparameter des adressierten Fensters (Geschoss, Schnitthöhe, Projektion, isoliertes Geschoss, Schnittebene, Schnittlinie, Tiefe); das Dokument ändert sich nicht.");
            "setCamera" as "camera" => set_camera::SetCamera, [WindowConfig, Presence]; View, ("Set Camera", "Kamera festlegen"), ("Stores the navigation pose of the addressed window.", "Speichert die Navigationspose des adressierten Fensters.");
            "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown, [Artifact, WindowTransient]; Mutation, ("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ("A press in a plan or section window: it belongs to the armed utility (select picks, wall sets a point, window places ...).", "Ein Druck in einem Grundriss- oder Schnittfenster: er gehört zum aktiven Werkzeug (Auswahl wählt, Wand setzt einen Punkt, Fenster platziert ...).");
            "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove, [Artifact, WindowTransient]; Mutation, ("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), ("The pointer moved over a plan or section window: the armed utility's gesture advances and shows its preview.", "Der Zeiger bewegte sich über einem Grundriss- oder Schnittfenster: die Geste des aktiven Werkzeugs schreitet fort und zeigt ihre Vorschau.");
            "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp, [Artifact, WindowTransient]; Mutation, ("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ("A release in a plan or section window: a drag commits (marquee, handle, sliding opening, rectangle).", "Ein Loslassen in einem Grundriss- oder Schnittfenster: ein Ziehen wird abgeschlossen (Auswahlrahmen, Griff, gleitende Öffnung, Rechteck).");
            "canvasDoubleClick" as "canvas-double-click" => canvas_double_click::CanvasDoubleClick, [Artifact, WindowTransient]; Mutation, ("Canvas Double Click", "Leinwand-Doppelklick"), ("A double click finishes the gesture in progress (wall chain, railing, slab or roof polygon).", "Ein Doppelklick beendet die laufende Geste (Wandzug, Geländer, Decken- oder Dachpolygon).");
            "canvasCommitDraft" as "canvas-commit-draft" => canvas_commit_draft::CanvasCommitDraft, [Artifact, WindowTransient]; Mutation, ("Finish Drawing", "Zeichnen beenden"), ("Finishes the gesture in progress: a wall chain ends, a railing, slab or roof polygon is written.", "Beendet die laufende Geste: ein Wandzug endet, ein Geländer-, Decken- oder Dachpolygon wird geschrieben.");
            "canvasEscape" as "canvas-escape" => canvas_escape::CanvasEscape, [Artifact, WindowTransient]; Mutation, ("Cancel Drawing", "Zeichnen abbrechen"), ("Cancels the gesture in progress without a trace; what was already written stays.", "Bricht die laufende Geste spurlos ab; bereits Geschriebenes bleibt.");
            "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown, [Artifact, WindowTransient]; Mutation, ("World Pointer Down", "Welt-Zeiger gedrückt"), ("A press in the 3D window while a drawing utility is armed: the ground point is the click of the click-click tools.", "Ein Druck im 3D-Fenster bei aktivem Zeichenwerkzeug: der Bodenpunkt ist der Klick der Klick-Klick-Werkzeuge.");
            "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove, [Artifact, WindowTransient]; Mutation, ("World Pointer Move", "Welt-Zeiger bewegt"), ("The ground point under the pointer in the 3D window while a drawing utility is armed.", "Der Bodenpunkt unter dem Zeiger im 3D-Fenster bei aktivem Zeichenwerkzeug.");
            "armSelect" as "arm-select" => arm_utility::ArmSelect, [HostOnly]; View, ("Select Tool", "Auswahlwerkzeug"), ("Arms the select utility in the addressed window (V).", "Aktiviert das Auswahlwerkzeug im adressierten Fenster (V).");
            "armWall" as "arm-wall" => arm_utility::ArmWall, [HostOnly]; View, ("Wall Tool", "Wandwerkzeug"), ("Arms the wall utility in the addressed window (W).", "Aktiviert das Wandwerkzeug im adressierten Fenster (W).");
            "armWallArc" as "arm-wall-arc" => arm_utility::ArmWallArc, [HostOnly]; View, ("Arc Wall Tool", "Bogenwand-Werkzeug"), ("Arms the arc wall utility in the addressed window (A).", "Aktiviert das Bogenwand-Werkzeug im adressierten Fenster (A).");
            "armCurtainWall" as "arm-curtain-wall" => arm_utility::ArmCurtainWall, [HostOnly]; View, ("Curtain Wall Tool", "Vorhangfassaden-Werkzeug"), ("Arms the curtain wall utility in the addressed window (U).", "Aktiviert das Vorhangfassaden-Werkzeug im adressierten Fenster (U).");
            "armColumn" as "arm-column" => arm_utility::ArmColumn, [HostOnly]; View, ("Column Tool", "Stützenwerkzeug"), ("Arms the column utility in the addressed window (C).", "Aktiviert das Stützenwerkzeug im adressierten Fenster (C).");
            "armBeam" as "arm-beam" => arm_utility::ArmBeam, [HostOnly]; View, ("Beam Tool", "Trägerwerkzeug"), ("Arms the beam utility in the addressed window (B).", "Aktiviert das Trägerwerkzeug im adressierten Fenster (B).");
            "armSlab" as "arm-slab" => arm_utility::ArmSlab, [HostOnly]; View, ("Slab Tool", "Deckenwerkzeug"), ("Arms the slab utility in the addressed window (S).", "Aktiviert das Deckenwerkzeug im adressierten Fenster (S).");
            "armRoof" as "arm-roof" => arm_utility::ArmRoof, [HostOnly]; View, ("Roof Tool", "Dachwerkzeug"), ("Arms the roof utility in the addressed window (R).", "Aktiviert das Dachwerkzeug im adressierten Fenster (R).");
            "armWindow" as "arm-window" => arm_utility::ArmWindow, [HostOnly]; View, ("Window Tool", "Fensterwerkzeug"), ("Arms the window utility in the addressed window (N).", "Aktiviert das Fensterwerkzeug im adressierten Fenster (N).");
            "armDoor" as "arm-door" => arm_utility::ArmDoor, [HostOnly]; View, ("Door Tool", "Türwerkzeug"), ("Arms the door utility in the addressed window (D).", "Aktiviert das Türwerkzeug im adressierten Fenster (D).");
            "armOpening" as "arm-opening" => arm_utility::ArmOpening, [HostOnly]; View, ("Opening Tool", "Öffnungswerkzeug"), ("Arms the opening utility in the addressed window (O).", "Aktiviert das Öffnungswerkzeug im adressierten Fenster (O).");
            "armStair" as "arm-stair" => arm_utility::ArmStair, [HostOnly]; View, ("Stair Tool", "Treppenwerkzeug"), ("Arms the stair utility in the addressed window (T).", "Aktiviert das Treppenwerkzeug im adressierten Fenster (T).");
            "armRailing" as "arm-railing" => arm_utility::ArmRailing, [HostOnly]; View, ("Railing Tool", "Geländerwerkzeug"), ("Arms the railing utility in the addressed window (L).", "Aktiviert das Geländerwerkzeug im adressierten Fenster (L).");
            "armSpace" as "arm-space" => arm_utility::ArmSpace, [HostOnly]; View, ("Space Tool", "Raumwerkzeug"), ("Arms the space utility in the addressed window (P).", "Aktiviert das Raumwerkzeug im adressierten Fenster (P).");
            "armGrid" as "arm-grid" => arm_utility::ArmGrid, [HostOnly]; View, ("Grid Tool", "Rasterwerkzeug"), ("Arms the grid line utility in the addressed window (G).", "Aktiviert das Rasterlinien-Werkzeug im adressierten Fenster (G).");
            "armMeasure" as "arm-measure" => arm_utility::ArmMeasure, [HostOnly]; View, ("Measure Tool", "Messwerkzeug"), ("Arms the measure utility in the addressed window (M).", "Aktiviert das Messwerkzeug im adressierten Fenster (M).");
        }
    };
}

macro_rules! command_enum {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, ($en:literal, $de:literal), ($describe_en:literal, $describe_de:literal);)+) => {
        semio_framework_plugin::app_commands! {
            /// 🎯️ `BimModelApp::Command`: the sole dispatch surface of the editor, generated from the command table.
            pub enum BimCommand for ModelSnapshot, ModelMutation, NoConfig, NoConfigMutation, ctx = BimDispatchCtx {
                $($id as $key => $module::$payload),+
            }
        }
    };
}

macro_rules! tool_roster {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, ($en:literal, $de:literal), ($describe_en:literal, $describe_de:literal);)+) => {
        /// 🧵️ Every bounded first-step tool of the editor, in command-table order.
        pub const BIM_TOOL_IDS: &[&str] = &[$($id),+];
        const BIM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[$(ArtifactToolPublicationContract { tool_id: $id, lanes: &[$(ArtifactToolPublicationLane::$lane),+] }),+];
    };
}

macro_rules! tool_proofs {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, ($en:literal, $de:literal), ($describe_en:literal, $describe_de:literal);)+) => {
        semio_framework_plugin::bounded_first_step_tool_proofs! {
            owner: EditorApp<BimModelApp>,
            owner_file: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
            controller: "s.bim.model@1/*#editor",
            artifact_schema: "s.bim.model@1",
            factory: "BimCommandJobFactory",
            factory_type: BimCommandJobFactory,
            contract: ToolExecutionContract::bounded_first_step(BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500),
            tools: [$($id),+]
        }
    };
}

macro_rules! manifest_actions {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, ($en:literal, $de:literal), ($describe_en:literal, $describe_de:literal);)+) => {
        /// 📇️ The manifest action of every command row.
        fn command_actions() -> Vec<ActionDefinition> {
            vec![$(ActionDefinition::new($id, LocalizedLabel::native($en, $de), ActionKind::$kind, "box").describe(LocalizedLabel::native($describe_en, $describe_de))),+]
        }
    };
}

bim_command_table!(command_enum);
bim_command_table!(tool_roster);
bim_command_table!(manifest_actions);
//#endregion 🔖️CommandTable

//#region 🔖️ActionBridge
/// 🌉️ Host-action bridge into the closed `BimCommand` enum: the shells speak `{action, args}` with camelCase keys, every payload derives `FromValue` over snake_case, so this boundary
/// folds the keys, applies the per-verb aliases and decodes.
mod args_bridge {
    use super::*;
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn snake(key: &str) -> String {
        key.chars().flat_map(|ch| if ch.is_ascii_uppercase() { vec!['_', ch.to_ascii_lowercase()] } else { vec![ch] }).collect()
    }

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    fn integral(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    fn stringify(value: DslValue) -> DslValue {
        match value {
            DslValue::String(_) => value,
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
        }
    }

    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        DslValue::Object(entries)
    }

    fn value_as_text(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
                slot.1 = stringify(slot.1.clone());
            }
        }
        folded
    }

    fn ids_as_list(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "ids") {
                if let DslValue::String(single) = &slot.1 {
                    slot.1 = DslValue::Array(vec![DslValue::String(single.clone())]);
                }
            }
        }
        folded
    }

    /// 🎥️ The host nests the pose under `camera` (or sends it flat); a pose with `position` is the world's orbit, one with `x` the canvas'.
    fn camera_pose(args: Option<&DslValue>) -> DslValue {
        let folded = fold(args, &[], &[]);
        let DslValue::Object(entries) = &folded else { return folded };
        let pose = match entries.iter().find(|(key, _)| key == "camera") {
            Some((_, DslValue::Object(inner))) => inner.clone(),
            _ => entries.clone(),
        };
        let slot = if pose.iter().any(|(key, _)| key == "position") { "camera3d" } else { "camera2d" };
        let kept: Vec<(String, DslValue)> = pose.into_iter().filter(|(key, _)| matches!(key.as_str(), "x" | "y" | "zoom" | "position" | "target" | "up")).collect();
        DslValue::Object(vec![(slot.to_string(), DslValue::Object(kept))])
    }

    fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("bim action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<BimCommand, Fault> {
        let text = |value: &str| DslValue::String(value.into());
        Ok(match action {
            "createEntity" => BimCommand::CreateEntity(decode(action, fold(args, &[("type", "kind"), ("value", "kind"), ("parent_id", "parent")], &[("parent", text("")), ("name", text(""))]))?),
            "deleteSelection" => BimCommand::DeleteSelection(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "renameEntity" => BimCommand::RenameEntity(decode(action, fold(args, &[("value", "name")], &[]))?),
            "setField" => BimCommand::SetField(decode(action, ids_as_list(value_as_text(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))]))))?),
            "setView" => BimCommand::SetView(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
            "setCamera" => BimCommand::SetCamera(decode(action, camera_pose(args))?),
            "canvasPointerDown" => BimCommand::CanvasPointerDown(decode(action, fold(args, &[], &[]))?),
            "canvasPointerMove" => BimCommand::CanvasPointerMove(decode(action, fold(args, &[], &[]))?),
            "canvasPointerUp" => BimCommand::CanvasPointerUp(decode(action, fold(args, &[], &[]))?),
            "canvasDoubleClick" => BimCommand::CanvasDoubleClick(decode(action, fold(args, &[], &[]))?),
            "canvasCommitDraft" => BimCommand::CanvasCommitDraft(decode(action, fold(args, &[], &[]))?),
            "canvasEscape" => BimCommand::CanvasEscape(decode(action, fold(args, &[], &[]))?),
            "worldPointerDown" => BimCommand::WorldPointerDown(decode(action, fold(args, &[], &[]))?),
            "worldPointerMove" => BimCommand::WorldPointerMove(decode(action, fold(args, &[], &[]))?),
            "armSelect" => BimCommand::ArmSelect(decode(action, fold(args, &[], &[]))?),
            "armWall" => BimCommand::ArmWall(decode(action, fold(args, &[], &[]))?),
            "armWallArc" => BimCommand::ArmWallArc(decode(action, fold(args, &[], &[]))?),
            "armCurtainWall" => BimCommand::ArmCurtainWall(decode(action, fold(args, &[], &[]))?),
            "armColumn" => BimCommand::ArmColumn(decode(action, fold(args, &[], &[]))?),
            "armBeam" => BimCommand::ArmBeam(decode(action, fold(args, &[], &[]))?),
            "armSlab" => BimCommand::ArmSlab(decode(action, fold(args, &[], &[]))?),
            "armRoof" => BimCommand::ArmRoof(decode(action, fold(args, &[], &[]))?),
            "armWindow" => BimCommand::ArmWindow(decode(action, fold(args, &[], &[]))?),
            "armDoor" => BimCommand::ArmDoor(decode(action, fold(args, &[], &[]))?),
            "armOpening" => BimCommand::ArmOpening(decode(action, fold(args, &[], &[]))?),
            "armStair" => BimCommand::ArmStair(decode(action, fold(args, &[], &[]))?),
            "armRailing" => BimCommand::ArmRailing(decode(action, fold(args, &[], &[]))?),
            "armSpace" => BimCommand::ArmSpace(decode(action, fold(args, &[], &[]))?),
            "armGrid" => BimCommand::ArmGrid(decode(action, fold(args, &[], &[]))?),
            "armMeasure" => BimCommand::ArmMeasure(decode(action, fold(args, &[], &[]))?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the bim editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface: the implicit document in/out ports only.
pub fn bim_io() -> AppIo {
    AppIo {
        artifact_schema: BIM_MODEL_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.bim-model".into(), name: "BIM Model".into(), dimension: "3d".into(), component_kind: "bim".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️RetainedCommands
fn bim_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

fn bim_bounded_extent(command: &BimCommand) -> Option<usize> {
    BIM_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

/// 🧵️ The single step of a bounded command: dispatches against the retained roots with the addressed window's config and the local presence, and publishes the presence the handler asked for.
struct BimCommandWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    completed: bool,
}

impl BimCommandWork {
    fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, completed: false }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for BimCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.completed && command.command_id() == self.tool_id).then(|| bim_bounded_extent(command)).flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("bim-command-work-terminal"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let window_config = input.context.and_then(|context| context.window_config.as_ref());
        let cfg = ConfigView { snapshot: input.config, window: window_config };
        let selection = |domain: &str| input.interaction.selection.get(domain).map(|selection| selection.ids.clone()).unwrap_or_default();
        let view = input.context.and_then(|context| context.view_state.as_ref());
        let presence = input.context.and_then(|context| context.presence_view()).map(|presence| presence.local.clone());
        let mut ctx = BimDispatchCtx::new(selection(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection(crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view, window_config, presence.as_ref());
        ctx.gestures = Some(self.owner.clone());
        ctx.window_transient = crate::editor::bim::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let emit = input.command.dispatch(&doc, &cfg, &mut ctx)?;
        self.completed = true;
        let window_transient = match (ctx.transient_out.take(), view) {
            (Some(transient), Some(view)) => vec![crate::editor::bim::transient::addressed(view, transient)?],
            _ => Vec::new(),
        };
        Ok(if ctx.presence_out.is_empty() && window_transient.is_empty() { ArtifactCommandWorkStep::Complete(emit) } else { ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: ctx.presence_out, transient: Vec::new(), window_transient } } })
    }
}

struct BimCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BimCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BIM_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for BimCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BimModelApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BimModelApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        BIM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        bim_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BIM_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Bim command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BimCommandJobFactory {
    type Owner = EditorApp<BimModelApp>;
    const TOOL_IDS: &'static [&'static str] = BIM_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BIM_PUBLICATION_CONTRACTS;
}
//#endregion 🔖️RetainedCommands

//#region 🔖️Faults
/// 📣️ The en/de notices of every `bim.*` refusal code a command can raise.
pub fn bim_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| {
        [
            ("bim.create.kind-unknown", "This kind of entity does not exist.", "Diese Bauteilart gibt es nicht."),
            ("bim.create.unsupported", "This kind of entity cannot be created yet.", "Diese Bauteilart kann noch nicht angelegt werden."),
            ("bim.create.site-missing", "Create a site first.", "Legen Sie zuerst ein Grundstück an."),
            ("bim.create.building-missing", "Create a building first.", "Legen Sie zuerst ein Gebäude an."),
            ("bim.create.storey-missing", "Create or select a storey first.", "Legen Sie zuerst ein Geschoss an oder wählen Sie eines aus."),
            ("bim.create.wall-type-missing", "Create a wall type first.", "Legen Sie zuerst einen Wandtyp an."),
            ("bim.delete.unsupported", "These entities cannot be deleted yet.", "Diese Bauteile können noch nicht gelöscht werden."),
            ("bim.rename.target-missing", "The entity to rename does not exist.", "Das umzubenennende Bauteil gibt es nicht."),
            ("bim.rename.unsupported", "This kind of entity cannot be renamed yet.", "Diese Bauteilart kann noch nicht umbenannt werden."),
            ("bim.rename.rejected", "The name is not valid.", "Der Name ist ungültig."),
            ("bim.set.target-missing", "The entity to change does not exist.", "Das zu ändernde Bauteil gibt es nicht."),
            ("bim.set.field-unknown", "This entity has no such parameter.", "Dieses Bauteil hat keinen solchen Parameter."),
            ("bim.set.read-only", "This parameter cannot be edited yet.", "Dieser Parameter kann noch nicht bearbeitet werden."),
            ("bim.set.value-invalid", "The value is not valid for this parameter.", "Der Wert ist für diesen Parameter ungültig."),
            ("bim.view.window-required", "This setting belongs to one open window.", "Diese Einstellung gehört zu einem geöffneten Fenster."),
            ("bim.view.window-unsupported", "This window has no view settings.", "Dieses Fenster hat keine Ansichtseinstellungen."),
            ("bim.view.field-unknown", "This window has no such view setting.", "Dieses Fenster hat keine solche Ansichtseinstellung."),
            ("bim.view.value-invalid", "The value is not valid for this view setting.", "Der Wert ist für diese Ansichtseinstellung ungültig."),
            ("bim.view.storey-missing", "The storey does not exist.", "Das Geschoss gibt es nicht."),
            ("bim.camera.window-required", "A camera belongs to one open window.", "Eine Kamera gehört zu einem geöffneten Fenster."),
            ("bim.camera.pose-mismatch", "This window takes a different kind of camera pose.", "Dieses Fenster nimmt eine andere Art von Kamerapose."),
            ("bim.camera.invalid", "The camera pose is not valid.", "Die Kamerapose ist ungültig."),
            ("bim.tool.storey-missing", "Create or select a storey to draw on first.", "Legen Sie zuerst ein Geschoss an oder wählen Sie eines aus, auf dem gezeichnet wird."),
            ("bim.tool.type-missing", "The library has no type for this tool yet. Add one first.", "Die Bibliothek enthält noch keinen Typ für dieses Werkzeug. Legen Sie zuerst einen an."),
            ("bim.tool.rejected", "The model does not accept this here: it overlaps another element or leaves its host.", "Das Modell nimmt das hier nicht an: es überschneidet ein anderes Bauteil oder verlässt sein Trägerbauteil."),
            ("bim.gesture.retained-route", "Drawing gestures run only inside an open editor window.", "Zeichengesten laufen nur in einem geöffneten Editorfenster."),
        ]
        .into_iter()
        .map(|(code, en, de)| (code, LocalizedLabel::native(en, de)))
        .collect()
    });
    NOTICES.as_slice()
}
//#endregion 🔖️Faults

//#region 🔖️BimModelApp
#[derive(Default)]
pub struct BimModelApp;

/// 🧬️ The window-kind to render-body table: which body key a window kind or panel renders.
fn selection_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.selection(domain).ids.clone()
}

fn hover_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.hover(domain, "pointer").ids.clone()
}

fn instance_of(doc: &ArtifactView<'_, ModelSnapshot>) -> Option<u32> {
    doc.render_operation().map(|operation| operation.app_instance_id)
}

#[allow(clippy::too_many_arguments)]
fn render_body(body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel, elements: &[String], library: &[String], hover: &[String], preview: &crate::editor::bim::gestures::session::Preview) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let snapshot = doc.snapshot;
    let labels = bim_labels(view_state);
    let utility = crate::editor::bim::utilities::active(view_state);
    let windows = || TreeWindows::for_body(view_state, body_key);
    let node = crate::editor::bim::inference::with_inference(instance_of(doc), snapshot, |inference| match body_key {
        plan::BODY_KEY => {
            let config = plan::config::current(cfg);
            let revision = crate::render::plan::framing_revision(&plan::active_storey(snapshot, &config).unwrap_or_default());
            let overlay = crate::editor::bim::gestures::overlay::plan_records(snapshot, elements, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            plan::render_over(snapshot, inference, &config, elements, hover, utility, revision, labels, &overlay)
        }
        world::BODY_KEY => {
            let config = world::config::current(cfg);
            world::render(snapshot, inference, &config, elements, hover, crate::render::plan::framing_revision(&format!("{}{}", config.isolated_storey, config.projection.kind)))
        }
        section::BODY_KEY => {
            let config = section::config::current(cfg);
            let overlay = crate::editor::bim::gestures::overlay::section_records(inference, [config.start_x, config.start_y], [config.end_x, config.end_y], utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            section::render_over(snapshot, inference, &config, utility, 1, &overlay)
        }
        schedule::BODY_KEY => schedule::render(snapshot, inference, labels),
        outliner_panel::BODY_KEY => outliner_panel::render(snapshot, inference, labels, &windows()),
        properties_panel::BODY_KEY => properties_panel::render(snapshot, inference, elements, library, labels),
        library_panel::BODY_KEY => library_panel::render(snapshot, labels, &windows()),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("{}: {body_key}", labels.unknown_body.as_str()))).map_err(|_| crate::editor::bim::kit::ui_capacity_error()),
    })?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

impl ArtifactEditor for BimModelApp {
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        bim_fault_notices()
    }

    type Snapshot = ModelSnapshot;
    type Mutation = ModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = BimPresence;
    type PresenceMutation = BimPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = BimCommand;

    const DIALECT: Dialect = BIM_MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;

    bim_command_table!(tool_proofs);

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(BimCommandJobFactory::new(&controller))
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::<crate::editor::bim::gestures::GestureOwner>::default()
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BIM_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || bim_bounded_extent(&request.command).is_none() {
            return Err(crate::editor::bim::kit::fault("app.command.tool-mismatch", "Bim command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BimCommandWork::new(tool_id, request.instance_operation_owner.clone()));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            BimCommand::command_id,
            BIM_BOUNDED_RAW_BYTES,
            BIM_BOUNDED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(app_schema_descriptor())
    }

    fn initial_snapshot() -> ModelSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(bim_io())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("bim-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("bim-config-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default bim presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        plan::config::register(registry)?;
        world::config::register(registry)?;
        section::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        crate::editor::bim::transient::register(registry)
    }

    fn mounted_job_maintenance_step(_instance_id: u32, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn mounted_job_close_step(instance_id: u32, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(crate::editor::bim::inference::close(instance_id))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::editor::bim::inference::terminal_is_empty(instance_id)
    }

    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![outliner_panel::BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    fn entity_label(snapshot: &ModelSnapshot, _kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        let name = kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id))?;
        Some(LocalizedLabel::native(&name, &name))
    }

    fn interaction_topology(doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
        Ok(crate::editor::bim::interaction::topology(doc.snapshot))
    }

    fn command_id(command: &BimCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &BimCommand,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ModelMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = BimDispatchCtx::new(selection_ids(interaction, crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection_ids(interaction, crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view_state, cfg.window, None);
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &[], &[], &[], &Default::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        use crate::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
        let preview = crate::editor::bim::gestures::session::Preview::from_text(&crate::editor::bim::transient::current(transient).preview);
        render_body(body_key, doc, cfg, view_state, &selection_ids(interaction, BIM_ELEMENT_DOMAIN), &selection_ids(interaction, BIM_LIBRARY_DOMAIN), &hover_ids(interaction, BIM_ELEMENT_DOMAIN), &preview)
    }

    fn window_engagements(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), 0)
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let selected = selection_ids(interaction, crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN).len();
        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), selected)
    }

    fn window_measures(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        crate::editor::bim::chrome::measures(doc.snapshot, cfg, view_state)
    }
}
//#endregion 🔖️BimModelApp

//#region 🔖️Manifest
fn bim_action_args(id: &str) -> Vec<ActionArgDef> {
    let text = |name: &'static str, en: &'static str, de: &'static str| ActionArgDef::text(name, LocalizedLabel::native(en, de));
    match id {
        "createEntity" => vec![text("kind", "Kind", "Art").required(), text("parent", "Container", "Container"), text("name", "Name", "Name")],
        "deleteSelection" => vec![ActionArgDef::text_list("ids", LocalizedLabel::native("Entities", "Bauteile"))],
        "renameEntity" => vec![text("id", "Entity", "Bauteil").required(), text("name", "Name", "Name").required()],
        "setField" => vec![ActionArgDef::text_list("ids", LocalizedLabel::native("Entities", "Bauteile")), text("field", "Parameter", "Parameter").required(), text("value", "Value", "Wert").required()],
        "setView" => vec![text("field", "Setting", "Einstellung").required(), text("value", "Value", "Wert")],
        _ => Vec::new(),
    }
}

/// 🧱️ The editor manifest: one definition per taxonomy node.
pub fn create_bim_app() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(BIM_MODEL_DIALECT)
        .document(["semio", "bim"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("building")
        .mode_def(edit::definition())
        .default_mode_id(edit::BIM_EDIT_MODE_EDIT)
        .window_kind_def(plan::definition())
        .window_kind_def(world::definition())
        .window_kind_def(section::definition())
        .window_kind_def(schedule::definition())
        .default_layout(edit::layout())
        .panel_tab_def(outliner_panel::definition())
        .panel_tab_def(properties_panel::definition())
        .panel_tab_def(library_panel::definition())
        .io(bim_io());
    for definition in crate::editor::bim::interaction::definitions() {
        builder = builder.interaction(definition);
    }
    for utility in crate::editor::bim::utilities::definitions() {
        builder = builder.utility(utility);
    }
    for action in command_actions() {
        let id = action.id.clone();
        let pointer = crate::editor::bim::gestures::POINTER_COMMAND_IDS.contains(&id.as_str());
        let described = ActionDefinition { in_palette: !pointer, ..action.with_args(bim_action_args(&id)) };
        builder = builder.action_with(described).action_interactive_job(&id, InteractiveJobClassification::Migrated);
        if pointer {
            builder = builder.action_audience(&id, semio_framework_plugin::CapabilityAudience::Input);
        }
    }
    builder = builder.action_destructive("deleteSelection").keybinding("mod+z", "undo").keybinding("mod+shift+z", "redo").keybinding("delete", "deleteSelection").keybinding("backspace", "deleteSelection");
    for (keys, action) in crate::editor::bim::utilities::keybindings().into_iter().chain(crate::editor::bim::gestures::GESTURE_KEYBINDINGS.iter().copied()) {
        builder = builder.keybinding(keys, action);
    }
    builder.build_definition()
}
//#endregion 🔖️Manifest

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
