//! 🧬️ Generation2d artifact schema — every field of the artifact with its state class.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;

use crate::snapshot::Generation2dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::Widget;
use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation2dArtifact
/// 🧬️ Generation2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, Default)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dArtifact {
    #[state(artifact)]
    pub host_snapshot: FlowHostSnapshot,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation2dArtifact


impl Generation2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Generation2dSnapshot {
        Generation2dSnapshot { host_snapshot: self.host_snapshot.clone(), generation: self.generation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Generation2dSnapshot) -> Self {
        Self { host_snapshot: snapshot.host_snapshot, generation: snapshot.generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Generation2dSnapshot) {
        self.host_snapshot = snapshot.host_snapshot;
        std::mem::replace(&mut self.generation, snapshot.generation).retire_cold();
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.generation2d` — twenty handcrafted schema leaves.
pub fn generation2d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.procedural.generation2d",
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
/// pure helpers over document types (`FlowHostSnapshot`/`DagHostSnapshot`/eval `Value`), not app-referencing.
/// 🏠️ Runs `body` against a catalogue-seeded host built from `fixture`, then retires that host.
///
/// A `FlowHost` owns a cloned `FlowHostSnapshot`, whose `layout: OrderedMap<WidgetLayout>` rejects a bare
/// drop (`ordered-map root must be explicitly retired before drop`,
/// `🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), so a host is CLOSED through
/// [`FlowHost::retire_cold`], never dropped.
pub fn with_host<R>(host_snapshot: &FlowHostSnapshot, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    FlowHost::with_host_snapshot(host_snapshot, |host| {
        host.set_neuron_kind_info_map(flow_neuron_kind_info_map());
        body(host)
    })
}

/// 🏠️ [`with_host`]'s session-backed twin: the host shares `session`'s neural cache and converged
/// evaluation baseline, and is retired the same way. `body` receives the session back alongside the
/// host because every real caller needs it mutably (`sync`/`tick`).
pub fn host_with_session(host_snapshot: &FlowHostSnapshot, session: &FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant) -> Result<FlowHost, semio_framework_value::ValueError> {
    match flow_host_with_session(FlowHost::from_host_snapshot(host_snapshot.clone()).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY), session, grant) {
        Ok((host, _)) => Ok(host),
        Err((error, host)) => {
            host.retire_cold();
            Err(error)
        }
    }
}

pub fn with_host_session<R>(host_snapshot: &FlowHostSnapshot, session: &mut FlowEvalSession, grant: semio_framework_value::RetainedCloneGrant, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> Result<R, semio_framework_value::ValueError> {
    let mut host = host_with_session(host_snapshot, session, grant)?;
    let result = body(&mut host, session);
    host.retire_cold();
    Ok(result)
}























pub fn empty_generation2d_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

pub const GENERATION_2D_SCHEMA: &str = "generation.2d";

/// 🪪️ This artifact's canonical `s.procedural.generation2d@1/*` dialect — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling editor module (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
/// `artifact_kind` matches `definition()`'s own `s.generation2d.schema.artifact` capability descriptor
/// (`s.procedural.generation2d`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location.
pub const GENERATION2D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation2d", standard: StandardId("1"), subset: SubsetId::ANY };

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
//#endregion 🔖️Helpers

//#region 🔖️FaultNotices
/// 🪆️ The refusal code of a loaded parent whose composed children cannot be projected.
pub const GENERATION2D_CHILD_PROJECTION: &str = "generation2d.child.projection";

/// 📣️ The localized notices of the document-level refusals both surfaces raise (design §20.12).
pub fn generation2d_document_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 1]> = std::sync::LazyLock::new(|| [(GENERATION2D_CHILD_PROJECTION, LocalizedLabel::native("The document's composed parts cannot be restored.", "Die zusammengesetzten Teile des Dokuments können nicht wiederhergestellt werden."))]);
    &*NOTICES
}
//#endregion 🔖️FaultNotices

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::generation2d::create_generation2d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.generation".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Generation", "2D-Generierung"),
        source_format: "generation.2d".into(),
        component_kind: "generation2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        schema: "generation.2d".into(),
        export_formats: vec![],
        import_formats: vec![],
        // 🖊️ "stdio.dwg" stays out of exports (generation3d owns that EXPORT claim, D3) but stays in
        // imports below — see `🚪️io/🦀️.rs`'s `🚪️IoRegistry` region for the ownership rule.
        export_stdio_kinds: vec!["stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.dxf".into(), "stdio.json".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.svg".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Constants
/// 🏷️ Plain string tag (NOT a trait const — `ArtifactEditor::DIALECT`+`ROLE` derive the real surface
/// id now, contract §2.1) reused wherever a window/panel needs a stable controller/action-factory id.
pub const GENERATION2D_PLAY_APP_ID: &str = "procedural2d-play";

/// 🪪️ The editor surface id every tool transaction's `tool` is scoped by (`<appId>#<verb>`, design §2).
pub const GENERATION2D_EDITOR_APP_ID: &str = "s.procedural.generation2d@1/*#editor";

/// 🕹️ The framework interaction domain every generation2d window is bound to: the node graph and
/// both canvas previews read and write the same `graph` hover/selection.
pub const GENERATION2D_INTERACTION_DOMAIN: &str = "graph";

/// 🐁️ The channel a pointer hovers on.
pub const GENERATION2D_INTERACTION_CHANNEL: &str = "pointer";

/// 🎯️ Prefixes the node-graph canvas stamps onto pick targets — the twin of the flow plugin main
/// window's `NodeGraphInteractionDomain` prefixes.
pub const GENERATION2D_GRAPH_NODE_TARGET_PREFIX: &str = "generation2d-play-document.widget.";
pub const GENERATION2D_GRAPH_EDGE_TARGET_PREFIX: &str = "generation2d-play-document.synapse.";
pub const GENERATION2D_GRAPH_HANDLE_TARGET_PREFIX: &str = "generation2d-play-document.handle.";

//#region 🔖️Addressing
/// 🌡️ Resolves a widget's stable id to its BASE-state index in the fixture's widget list.
pub fn widget_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🌡️ Resolves a synapse's stable id to its BASE-state index in the fixture's synapse list.
pub fn synapse_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️Addressing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::FlowHostSnapshot;
//#endregion 🔁️Re-exports

#[path = "."]
pub mod snapshot {
    #[path = "📸️snapshot/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod inferences {
    #[path = "💡️inferences/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod topology {
        #[path = "💡️inferences/🧭topology/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod diff {
    #[path = "🔺️diff/🦀️.rs"]
    mod component;
    pub use component::*;
}
