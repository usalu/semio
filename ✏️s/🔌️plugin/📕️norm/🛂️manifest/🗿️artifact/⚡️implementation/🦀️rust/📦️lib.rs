//! 📏️ Norm plugin — one WASM DocumentApp per norm family with headless NormHost-backed compliance.

use norm_core::{CheckReport, NormFamily, NormHost, SetDocumentOperation};
#[cfg(test)]
use semio_framework_plugin::testkit;
use semio_framework_plugin::{
    create_default_layout, ui_stack_vertical, ui_text, App, AppIo, ArtifactPresentation, ConfigView, DocumentApp, DocumentView, Emit, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec,
    MediaType, OsMediaCapability, PanelGroup, ArtifactKindSpec, PortMultiplicity, SurfaceKind, UiNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Shared
fn render_report(report: &CheckReport) -> UiNode {
    if report.checks.is_empty() {
        return ui_text(Label::data("No checks computed."));
    }
    let children = report.checks.iter().enumerate().map(|(index, check)| ui_text(Label::data(format!("{}. {} — {:?} u={:.2} — {}", index + 1, check.clause, check.status, check.utilization, check.message)))).collect();
    ui_stack_vertical(children)
}

fn render_document_json<D: Serialize>(document: &D) -> UiNode {
    let json = serde_json::to_string_pretty(document).unwrap_or_else(|_| "{}".into());
    ui_text(Label::data(json))
}

fn render_summary<F: NormFamily>(host: &NormHost<F>) -> UiNode {
    let report = host.report();
    ui_text(Label::data(format!("{} — {} checks, worst u={:.2}, all pass={}", F::family_id().label(), report.checks.len(), report.worst_utilization(), report.all_pass())))
}
//#endregion 🔖️Shared

//#region 🔖️Config
/// 🧮️ B1: shared `DocumentApp::Config` for every norm family — all fifteen apps have the identical
/// shape (one field: which `CheckReport::checks` row `BODY_INSPECTION` currently points at, previously
/// hardcoded to `report.checks.first()`), so unlike `shooting`'s per-app `ShootingConfig` this is ONE
/// type reused by every `define_norm_family_app!` expansion rather than duplicated fifteen times.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "normcfg")]
#[dsl(layout = "lines")]
pub struct NormConfig {
    /// 👁️ Which `CheckReport::checks` row `BODY_INSPECTION` renders — `None` (the default) means "the
    /// first check", matching the pre-B1 hardcoded `report.checks.first()`.
    pub selected_check_index: Option<u32>,
}

impl store::ConfigRecord for NormConfig {}

/// @emoji 🧮️ Whole-record diff for `NormConfigOperation` — mirrors `shooting_engine::ShootingConfig`'s
/// own `OperationDiff` impl (`apply` ignores `base` entirely; `NormConfigOperation::Snapshot` already
/// carries the full post-op config).
impl OperationDiff<NormConfig> for NormConfig {
    fn apply(&self, _base: &NormConfig) -> NormConfig {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// @emoji 🧮️ `NormConfig`'s operation enum — `Snapshot` is the generic whole-config inverse every other
/// variant's `backwards()` returns (mirrors `shooting_op::ShootingConfigOperation`'s "restore the
/// whole-config snapshot from just before it" pattern, the simplest correct inverse for a config this
/// small); `SetSelectedCheckIndex` is the one real per-field edit every norm family app dispatches.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NormConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: NormConfig,
    },
    #[dsl(key = "selected-check")]
    SetSelectedCheckIndex { index: Option<u32> },
}

impl Operation<NormConfig> for NormConfigOperation {
    type Diff = NormConfig;

    fn diff(&self, base: &NormConfig) -> NormConfig {
        match self {
            NormConfigOperation::Snapshot { config } => config.clone(),
            NormConfigOperation::SetSelectedCheckIndex { index } => NormConfig { selected_check_index: *index, ..base.clone() },
        }
    }

    fn backwards(&self, base: &NormConfig) -> Vec<Self> {
        vec![NormConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️Config

//#region 🔖️Io
/// 🔌️ Every norm family's typed media I/O surface — the implicit `document:in`/`document:out` pair
/// (auto-injected by `AppIo::all_ports`) plus the two extra workflow ports every norm app family gets:
/// `model:in` (a generic upstream-model input — an honest pass-through, no family `Document` shape has a
/// generic "raw model" field to receive one into yet) and `report:out` (the computed `CheckReport`,
/// pinned to this family's own already-declared `computation.norm.{variant}` artifact kind via
/// `kind_id`). One function serves both the builder's `.io(...)` declaration (`create_app`) and each
/// generated `DocumentApp::io` override, so the two never drift apart.
fn norm_io(variant: &str, document_schema: &str, artifact_kind_id: &str) -> AppIo {
    AppIo {
        document_schema: document_schema.into(),
        document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            MediaPortSpec {
                id: "model:in".into(),
                label: "Model".into(),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: None,
                required: false,
                multiplicity: PortMultiplicity::One,
            },
            MediaPortSpec {
                id: "report:out".into(),
                label: "Report".into(),
                direction: MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                kind_id: Some(artifact_kind_id.into()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: ArtifactPresentation { id: artifact_kind_id.into(), name: variant.into(), dimension: "compliance".into(), component_kind: "norm".into() },
    }
}
//#endregion 🔖️Io

macro_rules! define_norm_family_app {
    ($module:ident, $app_struct:ident, $app_id:literal, $label:literal, $variant:literal, $doc_crate:ident, $op_crate:ident, $family_crate:ident, $family_ty:ident) => {
        pub mod $module {
            use super::*;
            use ::$doc_crate as doc_crate;
            use ::$op_crate as op_crate;
            use ::$family_crate as family_crate;

            type Family = family_crate::$family_ty;
            type Document = doc_crate::Document;
            type Operation = op_crate::Operation;

            const BODY_INPUTS: &str = concat!("norm.", $variant, ".play.inputs");
            const BODY_RESULTS: &str = concat!("norm.", $variant, ".play.results");
            const BODY_DOCUMENT: &str = concat!("norm.", $variant, ".play.document");
            const BODY_CATALOGUE: &str = concat!("norm.", $variant, ".play.catalogue");
            const BODY_INSPECTION: &str = concat!("norm.", $variant, ".play.inspection");
            const WINDOW_INPUTS: &str = concat!("norm-", $variant, "-inputs");
            const WINDOW_RESULTS: &str = concat!("norm-", $variant, "-results");
            const ARTIFACT_KIND_ID: &str = concat!("computation.norm.", $variant);

            //#region 🔖️Command
            /// 🎯️ B1: this family's `DocumentApp::Command` — the SOLE dispatch surface for its own
            /// behavior. `SetDocument` is a whole-document replace (mirrors the pre-B1 `"setDocument"`
            /// action), `Evaluate` recomputes in place by recommitting the current projection (mirrors
            /// `"evaluate"`), and `SetSelectedCheckIndex` is the one real config edit this app family has
            /// today (drives `BODY_INSPECTION`'s check pointer — see `NormConfig`).
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
            pub enum Command {
                #[dsl(key = "set-document")]
                SetDocument {
                    #[dsl(block)]
                    document: Document,
                },
                #[dsl(key = "evaluate")]
                Evaluate,
                #[dsl(key = "selected-check")]
                SetSelectedCheckIndex { index: Option<u32> },
            }
            //#endregion 🔖️Command

            #[derive(Default)]
            pub struct $app_struct;

            impl DocumentApp for $app_struct {
                type Projection = Document;
                type Operation = Operation;
                type Config = NormConfig;
                type ConfigOperation = NormConfigOperation;
                type Command = Command;

                fn app_id(&self) -> &str {
                    $app_id
                }

                fn document_schema(&self) -> &str {
                    concat!("semio.norm.", $variant, "/v1")
                }

                fn config_schema(&self) -> &str {
                    concat!("config.norm.", $variant)
                }

                fn initial_projection(&self) -> Self::Projection {
                    Document::default()
                }

                fn io(&self) -> Option<AppIo> {
                    Some(norm_io($variant, self.document_schema(), ARTIFACT_KIND_ID))
                }

                /// 🏷️ Maps each `Command` variant back to the action id it was declared under in
                /// `create_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
                /// View/Shell kind-discipline check.
                fn command_id(&self, command: &Command) -> &str {
                    match command {
                        Command::SetDocument { .. } => "setDocument",
                        Command::Evaluate => "evaluate",
                        Command::SetSelectedCheckIndex { .. } => "setSelectedCheckIndex",
                    }
                }

                fn handle(&self, command: &Command, doc: &DocumentView<'_, Document>, _cfg: &ConfigView<'_, NormConfig>) -> Emit<Operation, NormConfigOperation> {
                    match command {
                        Command::SetDocument { document } => Emit::commit(vec![SetDocumentOperation::SetDocument { document: document.clone() }], "setDocument"),
                        Command::Evaluate => Emit::commit(vec![SetDocumentOperation::SetDocument { document: doc.projection.clone() }], "evaluate"),
                        Command::SetSelectedCheckIndex { index } => Emit::config(vec![NormConfigOperation::SetSelectedCheckIndex { index: *index }]),
                    }
                }

                fn render(&self, body_key: &str, doc: &DocumentView<'_, Document>, cfg: &ConfigView<'_, NormConfig>) -> UiNode {
                    let host = NormHost::<Family>::from_document(doc.projection.clone());
                    match body_key {
                        BODY_INPUTS => render_document_json(&doc.projection),
                        BODY_RESULTS => render_report(host.report()),
                        BODY_DOCUMENT => render_summary::<Family>(&host),
                        BODY_CATALOGUE => ui_text(Label::data(format!("{} catalogue", $label))),
                        BODY_INSPECTION => {
                            let checks = &host.report().checks;
                            let index = cfg.projection.selected_check_index.map(|value| value as usize).filter(|index| *index < checks.len()).unwrap_or(0);
                            match checks.get(index) {
                                Some(check) => ui_text(Label::data(format!("{check:?}"))),
                                None => ui_text(Label::data("No checks")),
                            }
                        }
                        _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
                    }
                }

                //#region 🔖️MediaPorts
                /// 🎞️ `"report:out"` dumps the currently computed `CheckReport`, pinned to this family's
                /// already-declared `ARTIFACT_KIND_ID`; `"document:out"` replicates the SDK default
                /// (whole-document pack) since overriding `export_media` shadows it entirely.
                fn export_media(&self, port: &str, doc: &DocumentView<'_, Document>) -> Result<Media, MediaError> {
                    if port == "report:out" {
                        let host = NormHost::<Family>::from_document(doc.projection.clone());
                        let json = serde_json::to_string(host.report()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: ARTIFACT_KIND_ID.into(), json } });
                    }
                    if port != "document:out" {
                        return Err(MediaError::NotImplemented);
                    }
                    let bytes = store::DocumentPack::encode_pack(doc.projection);
                    Ok(Media {
                        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                        payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                    })
                }

                /// 🎞️ `"model:in"` is an honest generic pass-through: a payload that happens to decode as
                /// this family's own `Document` shape becomes a whole-document replace; anything else is
                /// accepted but inert (no norm family document has a generic "raw model" field to stash a
                /// foreign shape into yet — see the ticket's port-recipe notes). `"document:in"`
                /// replicates the SDK default (decodes the base64 pack).
                fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, Document>) -> Result<Emit<Operation, NormConfigOperation>, MediaError> {
                    if port == "model:in" {
                        if let MediaPayload::Structured { json, .. } = &media.payload {
                            if let Ok(document) = serde_json::from_str::<Document>(json) {
                                return Ok(Emit::operations(vec![SetDocumentOperation::SetDocument { document }]));
                            }
                        }
                        return Ok(Emit::default());
                    }
                    if port != "document:in" {
                        return Err(MediaError::NotImplemented);
                    }
                    let MediaPayload::Structured { json, .. } = &media.payload else {
                        return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                    };
                    let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                    let document = <Document as store::DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                    Ok(Emit::operations(vec![SetDocumentOperation::SetDocument { document }]))
                }
                //#endregion 🔖️MediaPorts
            }

            pub fn create_app() -> App {
                App::from_builder(
                    App::builder($app_id, LocalizedLabel::data($label))
                        .document(["semio", "norm", $variant])
                        .artifact_kind(ArtifactKindSpec {
                            id: ARTIFACT_KIND_ID.into(),
                            name: $label.into(),
                            source_format: format!("norm.{variant}.document", variant = $variant),
                            component_kind: "norm".into(),
                            dimension: "compliance".into(),
                            media_capability: OsMediaCapability::MeshOnly,
                            media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
                            schema: format!("norm.{variant}.document", variant = $variant),
                            export_formats: vec![],
                            import_formats: vec![],
                        })
                        .io(norm_io($variant, concat!("semio.norm.", $variant, "/v1"), ARTIFACT_KIND_ID))
                        .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
                        .default_mode_id("edit")
                        .window_kind(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, SurfaceKind::Canvas2d, "download")
                        .window_kind(WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), BODY_RESULTS, SurfaceKind::Canvas2d, "bar-chart-3")
                        .default_layout(create_default_layout(&[WINDOW_INPUTS.into(), WINDOW_RESULTS.into()], "row", Some(&[42.0, 58.0]), Some(&["Inputs".into(), "Results".into()])))
                        .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, BODY_DOCUMENT)
                        .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, BODY_CATALOGUE)
                        .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, BODY_INSPECTION)
                        .operation("setDocument", LocalizedLabel::native("Set Document", "Dokument setzen"))
                        .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
                        .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "Ausgewählte Prüfung setzen"))
                        .keybinding("mod+z", "undo")
                        .keybinding("mod+shift+z", "redo"),
                )
                .example("default", LocalizedLabel::native("Default", "Standard"), serde_json::to_string(&Document::default()).expect("default document serializes"), "file")
                .workflow($variant, $label, "compliance")
            }
        }
    };
}

define_norm_family_app!(din4108, Din4108PlayApp, "norm-din-4108-play", "DIN 4108", "din4108", din4108, din4108_op, din4108_engine, Din4108Family);
define_norm_family_app!(din16798, Din16798PlayApp, "norm-din-en-16798-play", "DIN EN 16798", "din16798", din16798, din16798_op, din16798_engine, DinEn16798Family);
define_norm_family_app!(din18599, Din18599PlayApp, "norm-din-v-18599-play", "DIN V 18599", "din18599", din18599, din18599_op, din18599_engine, DinV18599Family);
define_norm_family_app!(en1990, En1990PlayApp, "norm-en-1990-play", "EN 1990", "en1990", en1990, en1990_op, en1990_engine, En1990Family);
define_norm_family_app!(en1991, En1991PlayApp, "norm-en-1991-play", "EN 1991", "en1991", en1991, en1991_op, en1991_engine, En1991Family);
define_norm_family_app!(en1992, En1992PlayApp, "norm-en-1992-play", "EN 1992", "en1992", en1992, en1992_op, en1992_engine, En1992Family);
define_norm_family_app!(en1993, En1993PlayApp, "norm-en-1993-play", "EN 1993", "en1993", en1993, en1993_op, en1993_engine, En1993Family);
define_norm_family_app!(en1994, En1994PlayApp, "norm-en-1994-play", "EN 1994", "en1994", en1994, en1994_op, en1994_engine, En1994Family);
define_norm_family_app!(en1995, En1995PlayApp, "norm-en-1995-play", "EN 1995", "en1995", en1995, en1995_op, en1995_op, En1995Family);
define_norm_family_app!(en1996, En1996PlayApp, "norm-en-1996-play", "EN 1996", "en1996", en1996, en1996_op, en1996_op, En1996Family);
define_norm_family_app!(en1997, En1997PlayApp, "norm-en-1997-play", "EN 1997", "en1997", en1997, en1997_op, en1997_op, En1997Family);
define_norm_family_app!(en1998, En1998PlayApp, "norm-en-1998-play", "EN 1998", "en1998", en1998, en1998_op, en1998_op, En1998Family);
define_norm_family_app!(en1999, En1999PlayApp, "norm-en-1999-play", "EN 1999", "en1999", en1999, en1999_op, en1999_op, En1999Family);
define_norm_family_app!(iso16757, Iso16757PlayApp, "norm-iso-16757-play", "ISO 16757", "iso16757", iso16757, iso16757_op, iso16757_op, Iso16757Family);
define_norm_family_app!(vdi3805, Vdi3805PlayApp, "norm-vdi-3805-play", "VDI 3805", "vdi3805", vdi3805, vdi3805_op, vdi3805_op, Vdi3805Family);

//#region 🔖️Manifest
/// 🗂️ Sole native setup hook for the whole `norm` plugin bundle (`semio_plugin!`'s single
/// `setup: register_norm_exports`) — registers all fifteen family document kinds' pack↔dsl codecs
/// here since each `define_norm_family_app!`-generated module has no native registration fn of its own.
fn register_norm_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<din4108::Din4108PlayApp>(din4108::Din4108PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<din16798::Din16798PlayApp>(din16798::Din16798PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<din18599::Din18599PlayApp>(din18599::Din18599PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1990::En1990PlayApp>(en1990::En1990PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1991::En1991PlayApp>(en1991::En1991PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1992::En1992PlayApp>(en1992::En1992PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1993::En1993PlayApp>(en1993::En1993PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1994::En1994PlayApp>(en1994::En1994PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1995::En1995PlayApp>(en1995::En1995PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1996::En1996PlayApp>(en1996::En1996PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1997::En1997PlayApp>(en1997::En1997PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1998::En1998PlayApp>(en1998::En1998PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<en1999::En1999PlayApp>(en1999::En1999PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<iso16757::Iso16757PlayApp>(iso16757::Iso16757PlayApp.document_schema());
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<vdi3805::Vdi3805PlayApp>(vdi3805::Vdi3805PlayApp.document_schema());
}

semio_framework_plugin::semio_plugin! {
    id: "norm",
    label: "Norm",
    version: "0.1.0",
    setup: register_norm_exports,
    apps: [
        din4108::create_app => din4108::Din4108PlayApp,
        din16798::create_app => din16798::Din16798PlayApp,
        din18599::create_app => din18599::Din18599PlayApp,
        en1990::create_app => en1990::En1990PlayApp,
        en1991::create_app => en1991::En1991PlayApp,
        en1992::create_app => en1992::En1992PlayApp,
        en1993::create_app => en1993::En1993PlayApp,
        en1994::create_app => en1994::En1994PlayApp,
        en1995::create_app => en1995::En1995PlayApp,
        en1996::create_app => en1996::En1996PlayApp,
        en1997::create_app => en1997::En1997PlayApp,
        en1998::create_app => en1998::En1998PlayApp,
        en1999::create_app => en1999::En1999PlayApp,
        iso16757::create_app => iso16757::Iso16757PlayApp,
        vdi3805::create_app => vdi3805::Vdi3805PlayApp,
    ],
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use ::din4108::Document as Din4108Document;

    #[test]
    fn fifteen_family_apps_are_registered() {
        let ids = [
            din4108::create_app().definition.id,
            din16798::create_app().definition.id,
            din18599::create_app().definition.id,
            en1990::create_app().definition.id,
            en1991::create_app().definition.id,
            en1992::create_app().definition.id,
            en1993::create_app().definition.id,
            en1994::create_app().definition.id,
            en1995::create_app().definition.id,
            en1996::create_app().definition.id,
            en1997::create_app().definition.id,
            en1998::create_app().definition.id,
            en1999::create_app().definition.id,
            iso16757::create_app().definition.id,
            vdi3805::create_app().definition.id,
        ];
        assert_eq!(ids.len(), 15);
        assert!(ids.iter().all(|id| id.starts_with("norm-")));
    }

    #[test]
    fn din4108_host_backed_report_after_set_document() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let document = Din4108Document { airtightness_n50: 10.0, ..Din4108Document::default() };
        app.dispatch_typed(din4108::Command::SetDocument { document }, &testkit::meta("local")).expect("set document");
        let host = NormHost::<din4108_engine::Din4108Family>::from_document(app.projection().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    #[test]
    fn din4108_undo_redo_round_trip() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let document = Din4108Document { psi_times_l_sum: 0.5, ..Din4108Document::default() };
        testkit::assert_undo_redo_round_trip(&mut app, din4108::Command::SetDocument { document }, |app| app.projection().expect("projection").psi_times_l_sum, 0.02, 0.5);
    }

    /// 🧮️ `SetSelectedCheckIndex` is config-only — it must dispatch cleanly and never touch the document.
    #[test]
    fn din4108_selected_check_index_is_a_config_only_edit() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let before = app.projection().expect("projection");
        app.dispatch_typed(din4108::Command::SetSelectedCheckIndex { index: Some(2) }, &testkit::meta("local")).expect("select check");
        let after = app.projection().expect("projection");
        assert_eq!(before, after, "a config-only command must never mutate the document");
    }

    /// 🔌️ Port recipe: every norm family app declares `model:in`/`report:out` alongside the implicit
    /// document ports, and `report:out` is pinned to this family's already-declared artifact kind.
    #[test]
    fn din4108_declares_model_in_and_report_out_ports() {
        let ports = din4108::create_app().definition.io.ports;
        assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == MediaPortDirection::In));
        let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.direction, MediaPortDirection::Out);
        assert_eq!(report_out.kind_id.as_deref(), Some("computation.norm.din4108"));
    }

    /// 🎞️ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
    #[test]
    fn din4108_report_out_exports_the_computed_check_report() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let media = semio_framework_plugin::PluginApp::export_media(&mut app, "report:out").expect("export report:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
        assert_eq!(schema, "computation.norm.din4108");
        let report: CheckReport = serde_json::from_str(&json).expect("report json parses");
        assert!(!report.checks.is_empty());
    }

    #[test]
    fn norm_config_dsl_round_trips() {
        store::test_support::assert_dsl_round_trip(&NormConfig::default());
        store::test_support::assert_dsl_round_trip(&NormConfig { selected_check_index: Some(3) });
    }

    #[test]
    fn norm_config_dsl_pack_equivalence() {
        store::test_support::assert_dsl_pack_equivalence(&NormConfig::default());
        store::test_support::assert_dsl_pack_equivalence(&NormConfig { selected_check_index: Some(7) });
    }

    #[test]
    fn norm_config_operation_snapshot_is_a_real_inverse() {
        let base = NormConfig { selected_check_index: Some(1) };
        let op = NormConfigOperation::SetSelectedCheckIndex { index: Some(5) };
        let next = op.diff(&base);
        assert_eq!(next.selected_check_index, Some(5));
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![NormConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&next);
        assert_eq!(restored, base);
    }

    #[test]
    fn norm_config_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::SetSelectedCheckIndex { index: Some(2) });
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::SetSelectedCheckIndex { index: None });
        store::test_support::assert_op_line_round_trip(&NormConfigOperation::Snapshot { config: NormConfig { selected_check_index: Some(9) } });
    }
}
//#endregion 🧪️Tests
