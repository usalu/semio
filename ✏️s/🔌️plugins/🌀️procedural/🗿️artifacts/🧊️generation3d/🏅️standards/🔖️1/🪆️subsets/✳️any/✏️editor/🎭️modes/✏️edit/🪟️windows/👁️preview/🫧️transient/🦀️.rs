//! 🫧️ Computed evaluation owned by one exact Generation3d edit-preview window.

use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "generation.3dpreviewwindowtransient")]
#[dsl(layout = "lines")]
pub struct Generation3dPreviewWindowTransient {
    pub preview_eval_text: Option<String>,
}

impl store::ArtifactDsl for Generation3dPreviewWindowTransient {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str { Self::__DSL_ENVELOPE_ID }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Generation3d preview window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Generation3dPreviewWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Generation3d preview window transient pack envelope mismatch"))); }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(Self::__dsl_spec()) }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Generation3dPreviewWindowTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval { eval_text: Option<String> },
}

impl protocol::OpText for Generation3dPreviewWindowTransientMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown Generation3d preview window mutation '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| (spec.ordinary)()).expect("preview mutation variant exists");
        semio_framework_dsl_record::print(&record, &spec, semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Generation3dPreviewWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

impl Mutation<Generation3dPreviewWindowTransient> for Generation3dPreviewWindowTransientMutation {
    type Diff = Generation3dPreviewWindowTransientDiff;
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
        protocol::MutationOutcome::new(Generation3dPreviewWindowTransientDiff { preview_eval_text: (base.preview_eval_text != *eval_text).then(|| Generation3dPreviewEvalText { value: eval_text.clone() }) })
    }
    fn inverse(&self, base: &Generation3dPreviewWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| { vec![Self::SetPreviewEval { eval_text: base.preview_eval_text.clone() }] 
    })())
}
}

/// 🧱️ Carries an optional value as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dPreviewEvalText {
    pub value: Option<String>,
}

/// 🔺️ Sparse field delta over [`Generation3dPreviewWindowTransient`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dPreviewWindowTransientDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub preview_eval_text: Option<Generation3dPreviewEvalText>,
}

impl protocol::MutationDiff<Generation3dPreviewWindowTransient> for Generation3dPreviewWindowTransientDiff {
    fn apply(&self, base: &Generation3dPreviewWindowTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Generation3dPreviewWindowTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.preview_eval_text {
            next.preview_eval_text = value.value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.preview_eval_text.is_some() {
            self.preview_eval_text = other.preview_eval_text;
        }
    }
}

impl protocol::DiffAlgebra<Generation3dPreviewWindowTransient> for Generation3dPreviewWindowTransientDiff {
    fn inverse(&self, base: &Generation3dPreviewWindowTransient) -> Self {
        Self {
            preview_eval_text: self.preview_eval_text.as_ref().map(|_| Generation3dPreviewEvalText { value: base.preview_eval_text.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.preview_eval_text.is_none()
    }
}

semio_framework_value::artifact_retire_struct!(Generation3dPreviewWindowTransient { preview_eval_text });

impl semio_framework_value::retirement::RetireOwned for Generation3dPreviewWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self::SetPreviewEval { eval_text } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(eval_text)])
    }
}

fn preflight(mutation: &Generation3dPreviewWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let Generation3dPreviewWindowTransientMutation::SetPreviewEval { eval_text } = mutation;
    let retained_bytes = size_of::<Generation3dPreviewWindowTransient>().checked_add(eval_text.as_ref().map_or(0, String::capacity)).ok_or_else(|| "Generation3d preview window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint::for_ephemeral_item(retained_bytes);
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
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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
