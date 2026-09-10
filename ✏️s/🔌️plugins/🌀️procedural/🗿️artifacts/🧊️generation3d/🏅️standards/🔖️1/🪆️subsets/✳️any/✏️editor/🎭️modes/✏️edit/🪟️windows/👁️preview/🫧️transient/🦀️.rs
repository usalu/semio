//! 🫧️ Computed evaluation owned by one exact Generation3d edit-preview window.

use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "generation.3dpreviewwindowtransient")]
#[dsl(layout = "lines")]
pub struct Generation3dPreviewWindowTransient {
    pub preview_eval_text: Option<String>,
}

impl store::ArtifactDsl for Generation3dPreviewWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation3d preview window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation3dPreviewWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() { return Err(store::PackError::Schema("Generation3d preview window transient pack envelope mismatch".into())); }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps)]
pub enum Generation3dPreviewWindowTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval { eval_text: Option<String> },
}

impl protocol::OpText for Generation3dPreviewWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown Generation3d preview window mutation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec()).expect("preview mutation variant exists");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation3dPreviewWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

impl Mutation<Generation3dPreviewWindowTransient> for Generation3dPreviewWindowTransientMutation {
    type Diff = Generation3dPreviewWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient",
        semantic_kind: "set-preview-eval",
        display_name: "Set Preview Evaluation",
        emoji: "🫧️",
        aggregate_variant: "SetPreviewEval",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: Some("set-preview-eval"),
        binary_tag: Some(1),
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Text, protocol::MutationLanguageSurface::Binary, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, base: &Generation3dPreviewWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        let Self::SetPreviewEval { eval_text } = self;
        let mut next = base.clone();
        next.preview_eval_text.clone_from(eval_text);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &Generation3dPreviewWindowTransient) -> Vec<Self> { vec![Self::SetPreviewEval { eval_text: base.preview_eval_text.clone() }] }
}

impl protocol::MutationDiff<Generation3dPreviewWindowTransient> for Generation3dPreviewWindowTransient {
    fn apply(&self, _base: &Generation3dPreviewWindowTransient) -> protocol::MutationApplyResult<Generation3dPreviewWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

store::artifact_retire_struct!(Generation3dPreviewWindowTransient { preview_eval_text });

impl store::retirement::RetireOwned for Generation3dPreviewWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::SetPreviewEval { eval_text } = self;
        store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(eval_text)])
    }
}

fn preflight(mutation: &Generation3dPreviewWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text } = mutation;
    let retained_bytes = size_of::<Generation3dPreviewWindowTransient>().checked_add(eval_text.as_ref().map_or(0, String::capacity)).ok_or_else(|| "Generation3d preview window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Generation3d preview window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: Generation3dPreviewWindowTransientMutation) -> Generation3dPreviewWindowTransient {
    let Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text } = mutation;
    Generation3dPreviewWindowTransient { preview_eval_text: eval_text }
}

pub struct Generation3dPreviewWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for Generation3dPreviewWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::GENERATION_3D_PLAY_WINDOW_PREVIEW;
    type State = Generation3dPreviewWindowTransient;
    type Mutation = Generation3dPreviewWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn addressed(snapshot: &semio_framework_plugin::WindowTransientSnapshot, eval_text: Option<String>) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    if snapshot.window_kind_id() != super::GENERATION_3D_PLAY_WINDOW_PREVIEW || snapshot.get::<Generation3dPreviewWindowTransientOwner>().is_none() {
        return Err(semio_framework_plugin::Fault::from("generation3d-preview-window-transient-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<Generation3dPreviewWindowTransientOwner>(snapshot.window_id(), Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text }))
}

//#region 🧬️Schema
/// 🧬️ The schema-first declaration of this lane, mounted the way every other `🎚️config`/`👥️presence`
/// owner mounts its own: `🧬️schema/🦀️.rs` carries the `ArtifactSchema`-derived shape the five sibling
/// format leaves state, while the struct above carries the runtime `DslArtifact` codec.
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🧬️Schema

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
