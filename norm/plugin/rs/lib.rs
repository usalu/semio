//! 📏 Norm plugin — one WASM DocumentApp per norm family with headless NormHost-backed compliance.

use norm_core::{CheckReport, NormFamily, NormHost, SetDocumentOp};
#[cfg(test)]
use semio_framework_plugin::testkit;
use semio_framework_plugin::{
    create_default_layout, ui_stack_vertical, ui_text, ActionEmit, App, DocumentApp, DocumentView, OsMediaCapability, PanelGroup, ResourceKindSpec, SurfaceKind, UiNode, ViewState, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::Serialize;
use serde_json::Value;

//#region 🔖Shared
fn render_report(report: &CheckReport) -> UiNode {
    if report.checks.is_empty() {
        return ui_text("No checks computed.");
    }
    let children = report.checks.iter().enumerate().map(|(index, check)| ui_text(format!("{}. {} — {:?} u={:.2} — {}", index + 1, check.clause, check.status, check.utilization, check.message))).collect();
    ui_stack_vertical(children)
}

fn render_document_json<D: Serialize>(document: &D) -> UiNode {
    let json = serde_json::to_string_pretty(document).unwrap_or_else(|_| "{}".into());
    ui_text(json)
}

fn render_summary<F: NormFamily>(host: &NormHost<F>) -> UiNode {
    let report = host.report();
    ui_text(format!("{} — {} checks, worst u={:.2}, all pass={}", F::family_id().label(), report.checks.len(), report.worst_utilization(), report.all_pass()))
}
//#endregion 🔖Shared

macro_rules! define_norm_family_app {
    ($module:ident, $app_struct:ident, $app_id:literal, $label:literal, $variant:literal, $family_mod:ident, $family_ty:ident) => {
        pub mod $module {
            use super::*;
            use $family_mod as family_crate;

            type Family = family_crate::$family_ty;
            type Document = family_crate::Document;
            type Op = family_crate::Op;

            const BODY_INPUTS: &str = concat!("norm.", $variant, ".play.inputs");
            const BODY_RESULTS: &str = concat!("norm.", $variant, ".play.results");
            const BODY_DOCUMENT: &str = concat!("norm.", $variant, ".play.document");
            const BODY_CATALOGUE: &str = concat!("norm.", $variant, ".play.catalogue");
            const BODY_INSPECTION: &str = concat!("norm.", $variant, ".play.inspection");
            const WINDOW_INPUTS: &str = concat!("norm-", $variant, "-inputs");
            const WINDOW_RESULTS: &str = concat!("norm-", $variant, "-results");

            #[derive(Default)]
            pub struct $app_struct;

            impl DocumentApp for $app_struct {
                type Projection = Document;
                type Op = Op;

                fn app_id(&self) -> &str {
                    $app_id
                }

                fn document_schema(&self) -> &str {
                    concat!("semio.norm.", $variant, "/v1")
                }

                fn initial_projection(&self) -> Self::Projection {
                    Document::default()
                }

                fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Self::Projection>, _view_state: &ViewState) -> ActionEmit<Self::Op> {
                    match action {
                        "setDocument" => {
                            if let Some(next) = args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<Document>(value.clone()).ok()) {
                                return ActionEmit::commit(vec![SetDocumentOp::SetDocument { document: next }], "setDocument");
                            }
                        }
                        "evaluate" => {
                            let _host = NormHost::<Family>::from_document(doc.projection.clone());
                        }
                        _ => {}
                    }
                    ActionEmit::default()
                }

                fn render(&self, body_key: &str, doc: &DocumentView<'_, Self::Projection>, _view_state: &ViewState) -> UiNode {
                    let host = NormHost::<Family>::from_document(doc.projection.clone());
                    match body_key {
                        BODY_INPUTS => render_document_json(&doc.projection),
                        BODY_RESULTS => render_report(host.report()),
                        BODY_DOCUMENT => render_summary::<Family>(&host),
                        BODY_CATALOGUE => ui_text(format!("{} catalogue", $label)),
                        BODY_INSPECTION => {
                            if let Some(check) = host.report().checks.first() {
                                ui_text(format!("{check:?}"))
                            } else {
                                ui_text("No checks")
                            }
                        }
                        _ => ui_text(format!("Unknown body: {body_key}")),
                    }
                }
            }

            pub fn create_app() -> App {
                App::from_builder(
                    App::builder($app_id, $label)
                        .document(["semio", "norm", $variant])
                        .resource_kind(ResourceKindSpec {
                            id: format!("computation.norm.{variant}", variant = $variant),
                            name: $label.into(),
                            source_format: format!("norm.{variant}.document", variant = $variant),
                            component_kind: "norm".into(),
                            dimension: "compliance".into(),
                            media_capability: OsMediaCapability::MeshOnly,
                        })
                        .mode("edit", "Edit")
                        .default_mode_id("edit")
                        .window_kind(WINDOW_INPUTS, "Inputs", BODY_INPUTS, SurfaceKind::Canvas2d)
                        .window_kind(WINDOW_RESULTS, "Results", BODY_RESULTS, SurfaceKind::Canvas2d)
                        .default_layout(create_default_layout(&[WINDOW_INPUTS.into(), WINDOW_RESULTS.into()], "row", Some(&[42.0, 58.0]), Some(&["Inputs".into(), "Results".into()])))
                        .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, BODY_DOCUMENT)
                        .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, BODY_CATALOGUE)
                        .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, BODY_INSPECTION)
                        .operation("setDocument", "Set Document")
                        .view_action("evaluate", "Evaluate")
                        .keybinding("mod+z", "undo")
                        .keybinding("mod+shift+z", "redo"),
                )
                .example("default", "Default", serde_json::to_string(&Document::default()).expect("default document serializes"))
                .program($variant, $label, "compliance")
            }
        }
    };
}

define_norm_family_app!(din4108, Din4108PlayApp, "norm-din-4108-play", "DIN 4108", "din4108", norm_din_4108, Din4108Family);
define_norm_family_app!(din16798, Din16798PlayApp, "norm-din-en-16798-play", "DIN EN 16798", "din16798", norm_din_en_16798, DinEn16798Family);
define_norm_family_app!(din18599, Din18599PlayApp, "norm-din-v-18599-play", "DIN V 18599", "din18599", norm_din_v_18599, DinV18599Family);
define_norm_family_app!(en1990, En1990PlayApp, "norm-en-1990-play", "EN 1990", "en1990", norm_en_1990, En1990Family);
define_norm_family_app!(en1991, En1991PlayApp, "norm-en-1991-play", "EN 1991", "en1991", norm_en_1991, En1991Family);
define_norm_family_app!(en1992, En1992PlayApp, "norm-en-1992-play", "EN 1992", "en1992", norm_en_1992, En1992Family);
define_norm_family_app!(en1993, En1993PlayApp, "norm-en-1993-play", "EN 1993", "en1993", norm_en_1993, En1993Family);
define_norm_family_app!(en1994, En1994PlayApp, "norm-en-1994-play", "EN 1994", "en1994", norm_en_1994, En1994Family);
define_norm_family_app!(en1995, En1995PlayApp, "norm-en-1995-play", "EN 1995", "en1995", norm_en_1995, En1995Family);
define_norm_family_app!(en1996, En1996PlayApp, "norm-en-1996-play", "EN 1996", "en1996", norm_en_1996, En1996Family);
define_norm_family_app!(en1997, En1997PlayApp, "norm-en-1997-play", "EN 1997", "en1997", norm_en_1997, En1997Family);
define_norm_family_app!(en1998, En1998PlayApp, "norm-en-1998-play", "EN 1998", "en1998", norm_en_1998, En1998Family);
define_norm_family_app!(en1999, En1999PlayApp, "norm-en-1999-play", "EN 1999", "en1999", norm_en_1999, En1999Family);

//#region 🔖Manifest
fn register_norm_exports() {}

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
    ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use norm_din_4108::Document as Din4108Document;
    use semio_framework_plugin::PluginApp;
    use semio_framework_plugin::ViewState;
    use serde_json::json;

    #[test]
    fn thirteen_family_apps_are_registered() {
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
        ];
        assert_eq!(ids.len(), 13);
        assert!(ids.iter().all(|id| id.starts_with("norm-")));
    }

    #[test]
    fn din4108_host_backed_report_after_set_document() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let document = Din4108Document { airtightness_n50: 10.0, ..Din4108Document::default() };
        app.handle_action("setDocument", Some(&json!({ "document": document })), &ViewState::default(), &testkit::meta("local")).expect("set document");
        let host = NormHost::<norm_din_4108::Din4108Family>::from_document(app.projection().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    #[test]
    fn din4108_undo_redo_round_trip() {
        let mut app = testkit::new_app::<din4108::Din4108PlayApp>();
        let document = Din4108Document { psi_times_l_sum: 0.5, ..Din4108Document::default() };
        testkit::assert_undo_redo_round_trip(&mut app, "setDocument", Some(&json!({ "document": document })), |app| app.projection().expect("projection").psi_times_l_sum, 0.02, 0.5);
    }
}
//#endregion 🧪Tests
