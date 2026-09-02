//! ⚙️ Layout app engine — the app's own typed media I/O surface. Relocated from the deleted
//! artifact-tree `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact
//! is a schema + io, never an engine. `layout_io()` returns `AppIo`, this app's typed media surface —
//! app-owned per the region → destination map's rule 4. The stateful rendering/scene/export engine
//! (`LayoutEngine` and everything built on it) lives in the `🎬️scene` sibling file, same split the old
//! engine used purely for size.

//#region 🔖️Io
/// 🔌️ Layout's typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `2d.layout` artifact kind `create_layout_app` already declares) plus the two
/// WORKFLOWS-END-TO-END-TYPED-PORTS ports: `fields:in` (a `form.dictionary` this layout binds as a new
/// named data source — see `crate::artifacts::layout::LayoutSnapshot::data_fields_json`) and `layout:out`
/// (the current layout re-exported as `2d.layout` vector/SVG for a downstream consumer).
pub async fn layout_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "layout.layout".into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "fields:in".into(),
                label: "Fields".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
                kind_id: Some("form.dictionary".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::One,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "layout:out".into(),
                label: "Layout".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
                kind_id: Some("2d.layout".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::Many,
            },
        ],
        // 🗄️ `AppIo.export_formats`/`import_formats` stay enum-of-legacy-formats-typed in the framework
        // (no string-based sibling field exists here the way `ArtifactKindSpec::export_stdio_kinds`
        // does) and `negotiate_wire_format` never reads `AppIo`'s copies (only `ArtifactKindSpec`'s,
        // via `OsArtifactDescriptor`) — this is still-scaffolding per `AppIo`'s own doc comment
        // ("apps don't populate this yet"). Left empty rather than fabricating a legacy-enum value
        // for a field nothing consumes; see ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6 report.
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.layout".into(), name: "2D Layout".into(), dimension: "2d".into(), component_kind: "layout".into() },
    }
}
//#endregion 🔖️Io
