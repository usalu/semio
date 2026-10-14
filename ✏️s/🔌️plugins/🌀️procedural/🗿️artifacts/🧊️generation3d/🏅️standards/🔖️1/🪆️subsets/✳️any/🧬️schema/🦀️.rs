//! 🧬️ Generation3d artifact schema — every field of the artifact with its state class.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;

#[path = "🧭️transforms/🦀️.rs"]
pub mod transforms;


use crate::snapshot::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;

use ::semio_framework_schema::ArtifactSchema;
use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_os_flow::{flow_host_with_session, FlowEvalSession, FlowHost};

//#region 🔖️Generation3dArtifact
/// 🧬️ Generation3dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.procedural.generation3d")]
pub struct Generation3dArtifact {
    #[state(artifact)]
    pub host_snapshot: FlowHostSnapshot,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation3dArtifact

//#region 🔖️PreviewCamera
/// 📷️ 3D preview viewport camera (schema twin of the app config record).
#[derive(Clone, Debug, PartialEq)]
pub struct Generation3dPreviewCamera {
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub fov: f64,
}

impl Default for Generation3dPreviewCamera {
    fn default() -> Self {
        Self { position_x: 4.0, position_y: -4.0, position_z: 3.0, target_x: 0.0, target_y: 0.0, target_z: 0.0, fov: 45.0 }
    }
}
//#endregion 🔖️PreviewCamera

impl Default for Generation3dArtifact {
    fn default() -> Self {
        Self { host_snapshot: FlowHostSnapshot::default(), generation: GenerationPlayState::default().into() }
    }
}

impl Generation3dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation3dSnapshot {
        Generation3dSnapshot { host_snapshot: self.host_snapshot.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation3dSnapshot) -> Self {
        Self { host_snapshot: snapshot.host_snapshot, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation3dSnapshot) {
        self.host_snapshot = snapshot.host_snapshot;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation3d` — twenty-two handcrafted mutation leaves.
pub fn generation3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.procedural.generation3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🕸️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🧬️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// pure helpers over document types (`FlowHostSnapshot`/`DagHostSnapshot`/`FlowHost`), not app-referencing (the
/// Config-referencing preview/mesh-export helpers that used to sit alongside these stayed in
/// `crate::editor::generation3d` instead — see that file's own `PreviewPipeline`/`MeshBridge` regions).
pub const PROCEDURAL_EXAMPLE_HEX_COLUMN: &str = "hexagonal-mushroom-column";
pub const PROCEDURAL_EXAMPLE_RECT_EXTRUDE: &str = "rectangle-extrude-volume";
pub const PROCEDURAL_EXAMPLE_SPHERE_TORUS: &str = "sphere-cut-with-torus";
pub const PROCEDURAL_EXAMPLE_BOX_FILLET: &str = "box-fillet-preview";
pub const PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE: &str = "sphere-box-fuse";
pub const PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE: &str = "face-sweep-extrude";
pub const PROCEDURAL_EXAMPLE_RECTANGLE_WIRE: &str = "rectangle-wire-preview";
pub const PROCEDURAL_EXAMPLE_MESH_WORKBENCH: &str = "mesh-workbench";
pub const PROCEDURAL_EXAMPLE_BOX_SHELL: &str = "box-shell-preview";



/// 📄️ The artifact's `Default` projection. NOT empty: `FlowHostSnapshot::default()`
/// (`🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`) is the three-widget
/// `slider → add → preview` demo graph, so this is the DEFAULT document, not the empty one — the
/// name it carried until ticket 26/09/09/PROCEDURAL-3D-END-TO-END said otherwise and made every
/// "empty document" law read against a populated graph.
pub fn default_generation3d_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::default()
}

/// 🕳️ The genuinely EMPTY projection: no widgets, no synapses, no positions, no generations — the
/// identity element every totality law is written against.
pub fn empty_generation3d_snapshot() -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.host_snapshot.widgets.clear();
    snapshot.host_snapshot.synapses.clear();
    snapshot
}

/// 🧾️ Whether `example_id` names a bundled procedural-3d example host_snapshot.
pub fn is_generation3d_example_id(example_id: &str) -> bool {
    matches!(
        example_id,
        PROCEDURAL_EXAMPLE_HEX_COLUMN
            | "demo"
            | PROCEDURAL_EXAMPLE_RECT_EXTRUDE
            | PROCEDURAL_EXAMPLE_SPHERE_TORUS
            | PROCEDURAL_EXAMPLE_BOX_FILLET
            | PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE
            | PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE
            | PROCEDURAL_EXAMPLE_RECTANGLE_WIRE
            | PROCEDURAL_EXAMPLE_MESH_WORKBENCH
            | PROCEDURAL_EXAMPLE_BOX_SHELL
    )
}







/// 🎯️ The roster entry `selected_id` names — the ONE lookup every generate-mode surface resolves its
/// "current generation" through.
///
/// 🐛️ Why not `playbook::selected_generation(state)`: that reads `GenerationPlayState.selected_generation_id`,
/// which the ARTIFACT only ever writes from `CreateGeneration`/`DeleteGeneration` replay — `selectGeneration`
/// emits no artifact mutation at all, only `Generation3dConfigMutation::SetSelectedGeneration`. The
/// evaluation path already resolved the selection off the CONFIG (`flow_eval_tick::evaluate`,
/// `generation_command_result`) while the three generate-mode window bodies still resolved it off the
/// artifact, so clicking a generation row moved the evaluation and moved nothing a user could see
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap #7). The config is the single authority now, and this
/// function is where it is read.
pub fn generation_by_id<'a>(generation: &'a GenerationPlayState, selected_id: Option<&str>) -> Option<&'a semio_framework_artifact_playbook_playbook::FormGeneration> {
    let selected_id = selected_id?;
    generation.generations.iter().find(|entry| entry.id == selected_id)
}

pub fn generation_host_snapshot_for(host_snapshot: &FlowHostSnapshot, generation: &GenerationPlayState, selected_id: Option<&str>) -> FlowHostSnapshot {
    let Some(selected) = generation_by_id(generation, selected_id) else {
        return host_snapshot.clone();
    };
    let mut patched = host_snapshot.clone();
    for widget in &mut patched.widgets {
        let Some(value) = selected.values.get(widget_id(widget)) else {
            continue;
        };
        match widget {
            Widget::InputSlider { value: current, .. } => {
                if let Some(number) = value.as_f64() {
                    *current = number;
                }
            }
            Widget::InputNote { text, .. } => {
                if let Some(next) = value.as_str() {
                    *text = next.to_string();
                }
            }
            Widget::InputImage { src, .. } => {
                if let Some(next) = value.as_str() {
                    *src = next.to_string();
                }
            }
            Widget::Variable { name, .. } => {
                if let Some(next) = value.as_str() {
                    *name = next.to_string();
                }
            }
            _ => {}
        }
    }
    patched
}

/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped. This scope is the ONLY way generation3d builds one —
/// there is no `host_from_host_snapshot(…) -> FlowHost` to leak.
pub fn with_host<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_host_snapshot(host_snapshot, |host| {
        host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ A host that shares `session`'s neural cache and converged evaluation baseline, installed under `grant`.
pub fn host_with_session(host_snapshot: &FlowHostSnapshot, session: &FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant) -> Result<FlowHost, semio_framework_value::ValueError> {
    match flow_host_with_session(FlowHost::from_host_snapshot(host_snapshot.clone()), session, grant) {
        Ok((host, _)) => Ok(host),
        Err((error, host)) => {
            host.retire_cold();
            Err(error)
        }
    }
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`), which a captured `&mut` could
/// not provide while the scope itself holds the session borrow.
pub fn with_host_session<R>(host_snapshot: &FlowHostSnapshot, session: &mut FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> Result<R, semio_framework_value::ValueError> {
    let mut host = host_with_session(host_snapshot, session, grant)?;
    let result = body(&mut host, session);
    host.retire_cold();
    Ok(result)
}







//#endregion 🔖️DocumentHelpers


pub const GENERATION_3D_SCHEMA: &str = "generation.3d";

/// 🎯️ This subset's canonical dialect (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §2.1/§7.4) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind` matches this artifact's own `s.generation3d.schema.artifact` capability descriptor
/// above (`b"s.procedural.generation3d"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.procedural.generation3d@1/*#editor` / `s.procedural.generation3d@1/*#viewer`.
pub const GENERATION3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Helpers
/// 🌡️ A flow widget's stable id, across every widget variant (mirrors flow's private accessor).
pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}
/// 👁️ Whether a widget contributes preview geometry at all. A `Neuron` carries its own author-set
/// `preview` toggle; an `OutputPreview` is a preview by construction; a `Cluster` has no toggle of
/// its own, so its contract output channels always preview, which is the only way a grouped
/// sub-graph's geometry reaches the 3D world at all.
pub fn widget_previews(widget: &Widget) -> bool {
    matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. } | Widget::Cluster { .. })
}

//#endregion 🔖️Helpers

//#region 🔖️FaultNotices
/// 🪆️ The refusal code of a loaded parent whose composed children cannot be projected.
pub const GENERATION3D_CHILD_PROJECTION: &str = "generation3d.child.projection";
/// 📤️ The refusal code of a document export that could not be written.
pub const GENERATION3D_IO_EXPORT: &str = "generation3d.io.export";
/// 📂️ The refusal code of an import whose accepted file types cannot be determined.
pub const GENERATION3D_IO_IMPORT_ACCEPT: &str = "generation3d.io.import-accept";
/// 🧩️ The refusal code of a node the editor could not add.
pub const GENERATION3D_WIDGET_ADD: &str = "generation3d.widget.add";

/// 📣️ The localized notices of the document-level refusals both surfaces raise (design §20.12).
pub fn generation3d_document_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 4]> = std::sync::LazyLock::new(|| {
        [
            (GENERATION3D_CHILD_PROJECTION, LocalizedLabel::native("The document's composed parts cannot be restored.", "Die zusammengesetzten Teile des Dokuments können nicht wiederhergestellt werden.")),
            (GENERATION3D_IO_EXPORT, LocalizedLabel::native("The document could not be exported.", "Das Dokument konnte nicht exportiert werden.")),
            (GENERATION3D_IO_IMPORT_ACCEPT, LocalizedLabel::native("The importable file types could not be determined.", "Die importierbaren Dateitypen konnten nicht bestimmt werden.")),
            (GENERATION3D_WIDGET_ADD, LocalizedLabel::native("The node could not be added.", "Der Knoten konnte nicht hinzugefügt werden.")),
        ]
    });
    &*NOTICES
}
//#endregion 🔖️FaultNotices

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::generation3d::create_generation3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.generation".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D Generation", "3D-Generierung"),
        source_format: "generation.3d".into(),
        component_kind: "generation3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        schema: "generation.3d".into(),
        export_formats: vec![],
        import_formats: vec![],
        // 🖼️ "stdio.json"/"stdio.png" stay out of exports (generation2d owns those EXPORT claims, D3)
        // but stay in imports below — see `🚪️io/🦀️.rs`'s `🚪️IoRegistry` region.
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.stl".into()],
    }
}
//#endregion 🔖️ArtifactKind

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowHostSnapshot;
pub use semio_framework_artifact_flow_flow::Widget;
pub use semio_framework_artifact_playbook_playbook::GenerationPlayState;
//#endregion 🔁️Re-exports

#[path = "."]
pub mod catalogue {
    #[path = "🗂️catalogue/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod snapshot {
    #[path = "📸️snapshot/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod diff {
    #[path = "🔺️diff/🦀️.rs"]
    mod component;
    pub use component::*;
}
