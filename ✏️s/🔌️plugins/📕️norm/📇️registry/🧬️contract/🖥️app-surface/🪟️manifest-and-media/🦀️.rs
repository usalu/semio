//! 🪟️ Manifest and media of the norm app surface: the mode, window and panel definitions, the artifact kind spec and the media export and import. See [`super`].

use super::*;

//#region 🔖️Manifest
/// ✏️ The `edit` mode definition — identical for all fifteen apps.
pub fn edit_mode_definition() -> ModeDefinition {
    ModeDefinition { id: MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A norm window kind — both windows of every app are plain `Canvas2d` surfaces with no measures,
/// engagement, actions or utilities, so only id/label/body/icon vary.
pub fn window_definition(id: &str, label: LocalizedLabel, body_key: &str, icon_id: &str) -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
        interactions: Vec::new(),
    }
}

/// 📌️ A norm panel tab — every one is a framework-predefined leaf id bound to this app's body key.
/// Byte-identical to the `AppBuilder::panel_tab(id, label, group, body_key)` scalar call it replaces
/// (`PanelTabSpec::leaf` builds exactly this shape).
pub fn panel_definition(id: &str, label: LocalizedLabel, group: PanelGroup, body_key: &str) -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(id.into()), label, group, body_key: Some(body_key.into()), children: Vec::new() }
}

/// 🗿️ A norm artifact kind — Data × Value document per owner-table (IO coverage lattice). `artifact_schema` is the
/// family's DOCUMENT_SCHEMA — the ONE schema identity the hub's codec rows, open targets and genesis key on (the same
/// value [`norm_io`] declares); the former `norm.<variant>.document` media string stays declared as `source_format`.
pub fn artifact_kind_spec(variant: &str, label: &str, artifact_schema: &str) -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: artifact_kind_id(variant),
        label: LocalizedLabel::data(label),
        source_format: format!("norm.{variant}.document"),
        component_kind: "norm".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: artifact_schema.into(),
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
pub fn norm_io(variant: &str, artifact_schema: &str) -> AppIo {
    let artifact_kind_id = artifact_kind_id(variant);
    AppIo {
        artifact_schema: artifact_schema.into(),
        artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            MediaPortSpec {
                id: "model:in".into(),
                label: media_port_label(LocalizedLabel::native("Model", "Modell")),
                direction: MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                kind_id: None,
                required: false,
                multiplicity: PortMultiplicity::One,
            },
            MediaPortSpec {
                id: "report:out".into(),
                label: media_port_label(LocalizedLabel::native("Report", "Bericht")),
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
/// artifact kind; `"artifact:out"` replicates the SDK default (whole-document pack) since overriding
/// `export_media` shadows it entirely. Any other port is `NotImplemented`.
pub fn export_media<F>(port: &str, variant: &str, artifact_schema: &str, document: &F::Document) -> Result<Media, MediaError>
where
    F: NormFamily,
    F::Document: pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot,
{
    if port == "report:out" {
        let report = cached_report_for::<F>(document).unwrap_or_else(|| {
            let report = F::evaluate(document);
            store_cached_report_for::<F>(document, report.clone());
            report
        });
        let json = semio_framework_pack_json::to_json_string(&report);
        return Ok(Media { media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_kind_id(variant), json } });
    }
    if port != "artifact:out" {
        return Err(MediaError::NotImplemented);
    }
    let bytes = pack::snapshot::ArtifactPack::encode_pack(document);
    Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: artifact_schema.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
}

/// 🎞️ `"model:in"` is an honest generic pass-through: no family `Document` has a generic "raw model" field to receive one, so the
/// payload is accepted but inert. A whole document never arrives as a mutation: `"artifact:in"` is refused because importing a
/// whole document is the shell's document load path (genesis/load, not a history row).
pub fn import_media<M>(port: &str, _media: &Media) -> Result<Emit<M, NoConfigMutation>, MediaError> {
    match port {
        "model:in" => Ok(Emit::default()),
        "artifact:in" => Err(MediaError::Payload(port.to_string(), "a whole document is loaded by the shell's document load path, never imported as mutations".into())),
        _ => Err(MediaError::NotImplemented),
    }
}
//#endregion 🔖️MediaPorts
