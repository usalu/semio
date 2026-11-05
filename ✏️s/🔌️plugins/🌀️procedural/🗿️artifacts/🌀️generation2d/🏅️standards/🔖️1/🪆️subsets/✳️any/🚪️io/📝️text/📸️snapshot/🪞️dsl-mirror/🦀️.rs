//! 🪞️ Generation2d snapshot DSL mirror — the `Generation2dSnapshotDsl` twin, its conversions and the `ArtifactDsl` codec, owned by the crate that owns `Generation2dSnapshot`.

use crate::snapshot::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_os_kernel as store;
use std::collections::BTreeMap;

pub use semio_s_procedural_core_generation_sqlite_native::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};

/// 🧾️ Local twin of `Generation2dSnapshot`, flattening `FlowHostSnapshot`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslRecord)]` grammar.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "procedural.generation2d", layout = "lines")]
pub struct Generation2dSnapshotDsl {
    pub schema: String,
    #[dsl(block)]
    pub camera: CameraJsonDsl,
    #[dsl(statements, block)]
    pub widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    pub synapses: Vec<SynapseSpecDsl>,
    pub layout: BTreeMap<String, WidgetLayoutDsl>,
    #[dsl(key = "selected-generation")]
    pub selected_generation_id: Option<String>,
    pub preview_text: Option<String>,
    #[dsl(table)]
    pub generations: Vec<FormGenerationDsl>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for Generation2dSnapshotDsl {
    const EXTENSION: &'static str = "generation2d";
    fn envelope_id() -> &'static str {
        "procedural.generation2d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}


//#endregion 🔖️HandcraftedArtifactCodecs

pub fn generation2d_document_to_dsl(document: &Generation2dSnapshot) -> Generation2dSnapshotDsl {
    let host_snapshot = &document.host_snapshot;
    let generation = &document.generation;
    Generation2dSnapshotDsl {
        schema: host_snapshot.schema.clone(),
        camera: camera_to_dsl(&host_snapshot.camera),
        widgets: host_snapshot.widgets.iter().map(widget_to_dsl).collect(),
        synapses: host_snapshot.synapses.iter().map(synapse_to_dsl).collect(),
        layout: host_snapshot.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect(),
    }
}

pub fn generation2d_document_from_dsl(parsed: Generation2dSnapshotDsl) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Generation2dSnapshot {
        host_snapshot: FlowHostSnapshot { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

/// 📜️ `.generation2d` textual document — derive-engine grammar via `Generation2dSnapshotDsl`.
impl pack::snapshot::ArtifactDsl for Generation2dSnapshot {
    const EXTENSION: &'static str = "generation2d";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation2dSnapshotDsl as pack::snapshot::ArtifactDsl>::parse_dsl(text)?;
        generation2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Generation2dSnapshotDsl as pack::snapshot::ArtifactDsl>::print_dsl(&generation2d_document_to_dsl(self))
    }
}

/// 📦️ `.generation2d` binary pack — same `Generation2dSnapshotDsl` mirror as `ArtifactDsl` above;
/// `dsl::DslArtifact`'s derive already gives `Generation2dSnapshotDsl` its own `ArtifactPack` impl.



/// 📖️ Parses `.generation2d` DSL text into a `Generation2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation2dSnapshot` back to `.generation2d` DSL text.
pub fn print_dsl(document: &Generation2dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}
