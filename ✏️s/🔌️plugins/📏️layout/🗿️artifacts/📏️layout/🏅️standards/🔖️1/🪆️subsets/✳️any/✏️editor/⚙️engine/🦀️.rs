//! ⚙️ Layout app engine — the app's own typed media I/O surface. Relocated from the deleted
//! artifact-tree `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact
//! is a schema + io, never an engine. `layout_io()` returns `AppIo`, this app's typed media surface —
//! app-owned per the region → destination map's rule 4. The stateful rendering/scene/export engine
//! (`LayoutEngine` and everything built on it) lives in the `🎬️scene` sibling file, same split the old
//! engine used purely for size.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

/// 🧮️ The export oracles this crate's scene laws call drive an original fixture caller, whose paid receipts the heap witness proves.
#[cfg(test)]
#[global_allocator]
static LAYOUT_ENGINE_TEST_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

//#region 🔖️Io
/// 🔌️ Layout's typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `2d.layout` artifact kind `create_layout_app` already declares) plus the two
/// WORKFLOWS-END-TO-END-TYPED-PORTS ports: `fields:in` (a `form.dictionary` this layout binds as a new
/// named data source — see `crate::LayoutSnapshot::data_fields`) and `layout:out`
/// (the current layout re-exported as `2d.layout` vector/SVG for a downstream consumer).
pub fn layout_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: "layout.layout".into(),
        artifact_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::TwoD, form: semio_framework_plugin::MediaForm::Vector },
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
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.layout".into(), name: "2D Layout".into(), dimension: "2d".into(), component_kind: "layout".into() },
    }
}
//#endregion 🔖️Io

//#region 🪢️TaxonomyMounts
#[path = "🎬️scene/🦀️.rs"]
pub mod scene;
//#endregion 🪢️TaxonomyMounts
