//! 🎛️ Norm plugin — the app-surface machinery every one of the fifteen compliance apps shares.
//!
//! 📌️ The fifteen norm apps are structurally identical by construction (one `edit` mode, an
//! inputs/results window pair, the framework document/catalogue/inspection panel trio, the same
//! `model:in`/`report:out` media ports, the same three commands) and differ only in their per-standard
//! `Document` type, ids and labels. Everything that does NOT vary lives here, ONCE; every taxonomy node
//! under `🎛️apps/<app>/` states only what genuinely varies and calls into this module. That is the
//! "shared declarations belong at the shallowest common ancestor" rule taken to its conclusion — the
//! shallowest common ancestor of fifteen sibling apps is the plugin's own `🫀️core`.
//!
//! Nothing here depends on any app or artifact module: every entry point is either a plain constructor
//! or generic over the artifact's `Document`/`NormFamily`, so `🫀️core` stays a leaf of the dependency
//! graph exactly as the artifacts require.

use crate::document::{CheckReport, NormFamily, NormHost};
use semio_framework_plugin::{
    ui_stack_vertical, ui_text, AppIo, ArtifactKindSpec, ArtifactPresentation, ConfigView, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec,
    MediaType, ModeDefinition, OsMediaCapability, PanelGroup, PanelTabDefinition, PanelTabKind, PortMultiplicity, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

//#region 🔖️Ids
/// 🆔️ The single mode every norm app declares.
pub const MODE_EDIT: &str = "edit";
//#endregion 🔖️Ids

//#region 🔖️Render
/// 📑️ Renders a whole `CheckReport` as one line per computed check.
pub fn render_report(report: &CheckReport) -> UiNode {
    if report.checks.is_empty() {
        return ui_text(Label::data("No checks computed."));
    }
    let children = report.checks.iter().enumerate().map(|(index, check)| ui_text(Label::data(format!("{}. {} — {:?} u={:.2} — {}", index + 1, check.clause, check.status, check.utilization, check.message)))).collect();
    ui_stack_vertical(children)
}

/// 📄️ Renders a document as pretty-printed JSON — the inputs window's surface.
pub fn render_document_json<D: Serialize>(document: &D) -> UiNode {
    let json = serde_json::to_string_pretty(document).unwrap_or_else(|_| "{}".into());
    ui_text(Label::data(json))
}

/// 🧾️ Renders a one-line headline for a family's current session — the document panel's surface.
pub fn render_summary<F: NormFamily>(host: &NormHost<F>) -> UiNode {
    let report = host.report();
    ui_text(Label::data(format!("{} — {} checks, worst u={:.2}, all pass={}", F::family_id().label(), report.checks.len(), report.worst_utilization(), report.all_pass())))
}

/// 📚️ Renders the catalogue panel's placeholder headline for a family.
pub fn render_catalogue(label: &str) -> UiNode {
    ui_text(Label::data(format!("{label} catalogue")))
}

/// 🔍️ Renders the inspection panel — the `selected_check_index` row of the report, falling back to the
/// first check when the index is unset or out of range (and to a placeholder when there are no checks).
pub fn render_inspection(report: &CheckReport, selected_check_index: Option<u32>) -> UiNode {
    let checks = &report.checks;
    let index = selected_check_index.map(|value| value as usize).filter(|index| *index < checks.len()).unwrap_or(0);
    match checks.get(index) {
        Some(check) => ui_text(Label::data(format!("{check:?}"))),
        None => ui_text(Label::data("No checks")),
    }
}

/// ❓️ The unknown-body-key fallback every norm app's `render` ends with.
pub fn render_unknown_body(body_key: &str) -> UiNode {
    ui_text(Label::data(format!("Unknown body: {body_key}")))
}
//#endregion 🔖️Render

//#region 🔖️Manifest
/// ✏️ The `edit` mode definition — identical for all fifteen apps.
pub fn edit_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A norm window kind — both windows of every app are plain `Canvas2d` surfaces with no measures,
/// engagement, actions or utilities, so only id/label/body/icon vary.
pub fn window_definition(id: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        id: id.into(),
        label,
        body_key: body_key.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: icon_id.into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 📌️ A norm panel tab — every one is a framework-predefined leaf id bound to this app's body key.
/// Byte-identical to the `AppBuilder::panel_tab(id, label, group, body_key)` scalar call it replaces
/// (`PanelTabSpec::leaf` builds exactly this shape).
pub fn panel_definition(id: &str, label: LocalizedLabel, group: PanelGroup, body_key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: Some(body_key.into()), children: Vec::new() }
}

/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice).
pub fn artifact_kind_spec(variant: &str, label: &str) -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: artifact_kind_id(variant),
        name: label.into(),
        source_format: format!("norm.{variant}.document"),
        component_kind: "norm".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: format!("norm.{variant}.document"),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}

/// 🆔️ `computation.norm.<variant>` — the artifact kind id `report:out` pins itself to.
pub fn artifact_kind_id(variant: &str) -> String {
    format!("computation.norm.{variant}")
}

/// 🔌️ Every norm family's typed media I/O surface — the implicit `document:in`/`document:out` pair
/// (auto-injected by `AppIo::all_ports`) plus the two extra workflow ports every norm app gets:
/// `model:in` (a generic upstream-model input — an honest pass-through, no family `Document` shape has
/// a generic "raw model" field to receive one into yet) and `report:out` (the computed `CheckReport`,
/// pinned to this family's own already-declared `computation.norm.{variant}` artifact kind via
/// `kind_id`). One function serves both the builder's `.io(...)` declaration and each app's
/// `ArtifactApp::io` override, so the two never drift apart.
pub fn norm_io(variant: &str, document_schema: &str) -> AppIo {
    let artifact_kind_id = artifact_kind_id(variant);
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
                kind_id: Some(artifact_kind_id.clone()),
                required: false,
                multiplicity: PortMultiplicity::Many,
            },
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: ArtifactPresentation { id: artifact_kind_id, name: variant.into(), dimension: "data".into(), component_kind: "norm".into() },
    }
}
//#endregion 🔖️Manifest

//#region 🔖️MediaPorts
/// 🎞️ `"report:out"` dumps the currently computed `CheckReport`, pinned to this family's declared
/// artifact kind; `"document:out"` replicates the SDK default (whole-document pack) since overriding
/// `export_media` shadows it entirely. Any other port is `NotImplemented`.
pub fn export_media<F>(port: &str, variant: &str, document_schema: &str, document: &F::Document) -> Result<Media, MediaError>
where
    F: NormFamily,
    F::Document: store::ArtifactPack,
{
    if port == "report:out" {
        let host = NormHost::<F>::from_document(document.clone());
        let json = serde_json::to_string(host.report()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_kind_id(variant), json } });
    }
    if port != "document:out" {
        return Err(MediaError::NotImplemented);
    }
    let bytes = store::ArtifactPack::encode_pack(document);
    Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: document_schema.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
}

/// 🎞️ `"model:in"` is an honest generic pass-through: a payload that happens to decode as this family's
/// own `Document` shape becomes a bundle of targeted `change-<field>` mutations (one per persistent
/// field, via each migrated facet's `XMutation::from_snapshot`) rather than a single whole-document
/// replace mutation — the banned `SetSnapshot` escape hatch has no 1:1 replacement, so `wrap` now
/// decomposes the imported document into the closed semantic vocabulary instead. Bundling them into one
/// `Emit::mutations` call keeps the import atomic (one edit, one undo entry), matching the old
/// single-mutation commit's history shape. Anything that doesn't decode is accepted but inert (no norm
/// family document has a generic "raw model" field to stash a foreign shape into yet). `"document:in"`
/// replicates the SDK default (decodes the base64 pack).
pub fn import_media<D, M, F>(port: &str, media: &Media, wrap: F) -> Result<Emit<M, crate::config::NormConfigMutation>, MediaError>
where
    D: Clone + Default + PartialEq + Serialize + DeserializeOwned + store::ArtifactPack,
    F: Fn(D) -> Vec<M>,
{
    if port == "model:in" {
        if let MediaPayload::Structured { json, .. } = &media.payload {
            if let Ok(document) = serde_json::from_str::<D>(json) {
                return Ok(Emit::mutations(wrap(document)));
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
    let document = <D as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
    Ok(Emit::mutations(wrap(document)))
}
//#endregion 🔖️MediaPorts

//#region 🔖️Commands
/// 📤️ The whole-document replace every app's `set-document` and `evaluate` commands emit — `description`
/// is the manifest action id the command was declared under, which the command log labels the edit with.

/// 📤️ Commit a typed document mutation (kept for the norm sub-lane's not-yet-migrated sibling facets;
/// migrated facets use `commit_snapshot_fields` below instead, since the whole-document `SetSnapshot`
/// variant this helper used to construct is banned with no 1:1 replacement).
pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::commit(vec![mutation], description))
}

/// 📤️ Commit a bundle of targeted semantic mutations as one described edit — the migrated facets'
/// replacement for `commit_snapshot`'s old single whole-document `SetSnapshot` commit: a `set-snapshot`
/// command payload (or a re-evaluation re-commit) decomposes into one `change-<field>` mutation per
/// persistent field via `XMutation::from_snapshot`, bundled here into a single undo entry.
pub fn commit_snapshot_fields<M>(mutations: Vec<M>, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::commit(mutations, description))
}

pub fn commit_document<D>(document: D, description: &str) -> Result<Emit<crate::document::SetArtifactMutation<D>, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::commit(vec![crate::document::SetArtifactMutation::SetArtifact { document }], description))
}

/// ☑️ The one config-only edit every app's `selected-check` command emits.
pub fn commit_selected_check_index<M>(index: Option<u32>) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {
    Ok(Emit::config(vec![crate::config::NormConfigMutation::SetSelectedCheckIndex { index }]))
}

/// 🎯️ Builds the args-side of an app's `command_from_action` bridge for `selected-check` — the shells
/// still speak `{action,args}` for chrome actions.
pub fn selected_check_index_arg(args: Option<&serde_json::Value>) -> Option<u32> {
    args.and_then(|value| value.get("index")).and_then(serde_json::Value::as_u64).map(|value| value as u32)
}
//#endregion 🔖️Commands

//#region 🔖️Views
/// 👁️ Reads the config's selected check index out of a `ConfigView` — the one field norm apps read.
pub fn selected_check_index(cfg: &ConfigView<'_, crate::config::NormConfig>) -> Option<u32> {
    cfg.snapshot.selected_check_index
}

/// 📄️ Reads the document out of a `ArtifactView` — spelled once so every app's `render`/`handle` reads
/// it the same way.
pub fn snapshot<'a, D>(doc: &'a ArtifactView<'_, D>) -> &'a D {
    doc.snapshot
}
//#endregion 🔖️Views

//#region 🧪️Tests
#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn the_edit_mode_is_the_same_for_every_app() {
        let mode = edit_mode_definition();
        assert_eq!(mode.id, MODE_EDIT);
        assert!(mode.tools.is_empty() && mode.commands.is_empty() && mode.layout_id.is_none());
    }

    #[test]
    fn a_window_definition_is_a_plain_canvas2d_surface() {
        let window = window_definition("norm-x-inputs", LocalizedLabel::native("Inputs", "Eingaben"), "norm.x.play.inputs", "download");
        assert_eq!(window.body_key, "norm.x.play.inputs");
        assert!(matches!(window.surface_kind, SurfaceKind::Canvas2d));
        assert!(window.actions.is_empty() && window.utilities.is_empty() && window.options.measures.is_empty());
    }

    /// 📌️ Proves `panel_definition` reproduces the scalar `AppBuilder::panel_tab` shape exactly (an
    /// `App`-kind leaf carrying the body key) — the property that keeps the manifest byte-identical
    /// after the panel declarations moved into `📌️panels/*` nodes.
    #[test]
    fn a_panel_definition_is_an_app_kind_leaf_carrying_its_body_key() {
        let panel = panel_definition("document", LocalizedLabel::native("Document", "Dokument"), PanelGroup::Workbench, "norm.x.play.document");
        assert!(matches!(&panel.kind, PanelTabKind::App(id) if id == "document"));
        assert_eq!(panel.body_key.as_deref(), Some("norm.x.play.document"));
        assert!(panel.children.is_empty());
    }

    #[test]
    fn norm_io_declares_model_in_and_report_out_beside_the_implicit_document_ports() {
        let io = norm_io("din4108", "semio.norm.din4108/v1");
        assert!(io.ports.iter().any(|port| port.id == "model:in" && port.direction == MediaPortDirection::In));
        let report_out = io.ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.direction, MediaPortDirection::Out);
        assert_eq!(report_out.kind_id.as_deref(), Some("computation.norm.din4108"));
        assert_eq!(io.artifact.id, "computation.norm.din4108");
    }

    #[test]
    fn the_artifact_kind_spec_is_a_data_value_document() {
        let spec = artifact_kind_spec("en1990", "EN 1990");
        assert_eq!(spec.id, "computation.norm.en1990");
        assert_eq!(spec.source_format, "norm.en1990.document");
        assert_eq!(spec.dimension, "data");
        assert_eq!(spec.component_kind, "norm");
        assert_eq!(spec.media_type.class, MediaClass::Data);
        assert_eq!(spec.media_type.form, MediaForm::Value);
    }

    #[test]
    fn render_report_falls_back_to_a_placeholder_when_nothing_was_computed() {
        let json = serde_json::to_string(&render_report(&CheckReport::default())).expect("json");
        assert!(json.contains("No checks computed."), "{json}");
    }

    #[test]
    fn render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index() {
        let mut report = CheckReport::default();
        report.push(crate::document::CheckResult::from_utilization(
            crate::document::ClauseId::new("demo", "§1", "1.1"),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 0.5),
            crate::document::Quantity::new(crate::document::QuantityKind::Dimensionless, 1.0),
            "demo check",
            crate::document::AnnexChoice::De,
        ));
        let inside = serde_json::to_string(&render_inspection(&report, Some(0))).expect("json");
        let outside = serde_json::to_string(&render_inspection(&report, Some(99))).expect("json");
        assert_eq!(inside, outside, "an out-of-range index must fall back to the first check");
        assert!(serde_json::to_string(&render_inspection(&CheckReport::default(), None)).expect("json").contains("No checks"));
    }

    #[test]
    fn selected_check_index_arg_reads_the_shell_wire_shape() {
        assert_eq!(selected_check_index_arg(Some(&serde_json::json!({ "index": 3 }))), Some(3));
        assert_eq!(selected_check_index_arg(Some(&serde_json::json!({}))), None);
        assert_eq!(selected_check_index_arg(None), None);
    }
}
//#endregion 🧪️Tests
