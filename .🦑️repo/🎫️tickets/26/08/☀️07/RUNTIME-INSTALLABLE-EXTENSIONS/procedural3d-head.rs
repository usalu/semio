//! ⚙️ Procedural3d artifact — headless compute (constitutional: engine).

use crate::apps::procedural3d::config::Procedural3dConfig;
use crate::artifacts::procedural3d::dsl::{
    PROCEDURAL3D_EXAMPLE_BOX_FILLET_TEXT, PROCEDURAL3D_EXAMPLE_BOX_SHELL_TEXT, PROCEDURAL3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT, PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT, PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT, PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT,
    PROCEDURAL3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT, PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT,
};
use crate::artifacts::procedural3d::{widget_id, Procedural3dDocument};
use flow_core::dag::DagFixture;
use flow_core::forms_bridge::apply_generation_values_to_fixture;
use flow_core::{flow_host_with_session, FlowEvalSession, FlowFixture, FlowHost, Widget};
use flow_extension_brep::tessellate_geometry;
use playbook::{selected_generation, GenerationPlayState};
use serde_json::{json, Value};
use store::DocumentDsl;

//#region 🔖️Constants
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";
//#endregion 🔖️Constants

//#region 🔖️ExtensionContributions
use semio_framework_core::Contribution;
use std::sync::Mutex;

/// 🧩️ One host-aggregated plugin contribution entry (`contributionsJson` wire shape).
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgramContributionEntry {
    plugin_id: String,
    contribution: Contribution,
}

/// 🔌️ Installs or refreshes contributed `flow.extension` manifests when the host pushes a new catalogue.
pub fn sync_flow_extension_contributions(contributions_json: &str) {
    static LAST: Mutex<String> = Mutex::new(String::new());
    let mut last = LAST.lock().expect("flow contributions lock");
    if *last == contributions_json {
        return;
    }
    for info in flow_core::installed_flow_extensions() {
        flow_core::uninstall_flow_extension(&info.id);
    }
    if let Ok(entries) = serde_json::from_str::<Vec<ProgramContributionEntry>>(contributions_json) {
        for entry in entries {
            if let Contribution::FlowExtension { manifest_json, .. } = entry.contribution {
                flow_core::install_flow_extension_manifest(&entry.plugin_id, &manifest_json);
            }
        }
    }
    *last = contributions_json.to_string();
}
//#endregion 🔖️ExtensionContributions

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_procedural3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub fn procedural3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        "procedural.3d",
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Flow },
        semio_framework_plugin::ArtifactPresentation { id: "3d.procedural".into(), name: "3D Procedural".into(), dimension: "3d".into(), component_kind: "procedural3d".into() },
    )
    .with_ports(vec![
        semio_framework_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework_core::PortMultiplicity::One,
        },