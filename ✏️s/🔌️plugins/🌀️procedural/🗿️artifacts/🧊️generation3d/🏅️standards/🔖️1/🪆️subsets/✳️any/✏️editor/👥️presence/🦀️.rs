//! 👥️ Generation3dPresence — shareable live ephemeral state + mutations.
//!
//! Shareable live subset of the 3d procedural surface: cameras and show mode. Selection/hover
//! broadcast automatically via the framework's typed `PresenceInteraction` (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `create_generation3d_app`'s
//! `.interaction(...)` declaration.

use crate::editor::generation3d::config::Generation3dPreviewCamera;
use protocol::Mutation;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of procedural 3d camera and show-mode state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "generation3d.presence")]
#[dsl(layout = "lines")]
pub struct Generation3dPresence {
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Generation3dPreviewCamera,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
}

impl Default for Generation3dPresence {
    fn default() -> Self {
        Self { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, preview_camera: Generation3dPreviewCamera::default(), show_mode: "shaded".into() }
    }
}

/// 🩹 Owned-field diff of [`Generation3dPresence`]: exactly the fields a leaf sets.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dPresencePatch {
    pub camera: Option<CameraJson>,
    pub preview_camera: Option<Generation3dPreviewCamera>,
    pub show_mode: Option<String>,
}

impl protocol::MutationDiff<Generation3dPresence> for Generation3dPresencePatch {
    fn apply(&self, base: &Generation3dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Generation3dPresence> {
        Ok(Generation3dPresence {
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
            preview_camera: self.preview_camera.clone().unwrap_or_else(|| base.preview_camera.clone()),
            show_mode: self.show_mode.clone().unwrap_or_else(|| base.show_mode.clone()),
            ..base.clone()
        })
    }
    fn absorb(&mut self, other: Self) {
        self.camera = other.camera.or_else(|| self.camera.take());
        self.preview_camera = other.preview_camera.or_else(|| self.preview_camera.take());
        self.show_mode = other.show_mode.or_else(|| self.show_mode.take());
    }
}

impl protocol::DiffAlgebra<Generation3dPresence> for Generation3dPresencePatch {
    fn inverse(&self, base: &Generation3dPresence) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            preview_camera: self.preview_camera.as_ref().map(|_| base.preview_camera.clone()),
            show_mode: self.show_mode.as_ref().map(|_| base.show_mode.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.preview_camera.is_none() && self.show_mode.is_none()
    }
}

impl store::ArtifactDsl for Generation3dPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for Generation3dPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum Generation3dPresenceMutation {
    #[dsl(key = "set-camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "set-preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        preview_camera: Generation3dPreviewCamera,
    },
    #[dsl(key = "set-show-mode")]
    SetShowMode {
        show_mode: String,
    },
}

impl Mutation<Generation3dPresence> for Generation3dPresenceMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate —
    /// `diff`/`inverse` dispatch here is a plain `match`, not the derive's per-leaf `MutationKind`
    /// shape. One entry per variant, in declaration order. ⚠️ PROVISIONAL: no variant below has an
    /// authored leaf directory on disk yet, so every `owner` names a path that does not exist —
    /// the same precedent puzzle3d's own config/presence aggregates set.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/👥️set-camera",
        semantic_kind: "set-camera",
        display_name: "Set Camera",
        emoji: "👥️",
        aggregate_variant: "SetCamera",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/👥️set-preview-camera",
        semantic_kind: "set-preview-camera",
        display_name: "Set Preview Camera",
        emoji: "👥️",
        aggregate_variant: "SetPreviewCamera",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    },
    protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/👥️set-show-mode",
        semantic_kind: "set-show-mode",
        display_name: "Set Show Mode",
        emoji: "👥️",
        aggregate_variant: "SetShowMode",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Generation3dPresenceMutation::SetCamera { .. } => &Self::DESCRIPTORS[0],
            Generation3dPresenceMutation::SetPreviewCamera { .. } => &Self::DESCRIPTORS[1],
            Generation3dPresenceMutation::SetShowMode { .. } => &Self::DESCRIPTORS[2],
        }
    }

    type Diff = Generation3dPresencePatch;

    fn diff(&self, base: &Generation3dPresence) -> protocol::MutationOutcome<Generation3dPresencePatch> {
        let patch = match self {
            Self::SetCamera { camera } => (camera != &base.camera).then(|| Generation3dPresencePatch { camera: Some(camera.clone()), ..Default::default() }),
            Self::SetPreviewCamera { preview_camera } => (preview_camera != &base.preview_camera).then(|| Generation3dPresencePatch { preview_camera: Some(preview_camera.clone()), ..Default::default() }),
            Self::SetShowMode { show_mode } => (show_mode != &base.show_mode).then(|| Generation3dPresencePatch { show_mode: Some(show_mode.clone()), ..Default::default() }),
        };
        match patch {
            Some(patch) => protocol::MutationOutcome::new(patch),
            None => protocol::MutationOutcome::empty().warning("mutation.no-op", "The presence already holds that value."),
        }
    }

    fn inverse(&self, base: &Generation3dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetCamera { .. } => Self::SetCamera { camera: base.camera.clone() },
            Self::SetPreviewCamera { .. } => Self::SetPreviewCamera { preview_camera: base.preview_camera.clone() },
            Self::SetShowMode { .. } => Self::SetShowMode { show_mode: base.show_mode.clone() },
        }])
    }
}

impl protocol::OpText for Generation3dPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for Generation3dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
