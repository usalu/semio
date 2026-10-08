//! 👥️ Process 3d presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of process3d engagement and camera state.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "process3d.presence")]
#[dsl(layout = "lines")]
pub struct Process3dPresence {
    pub engagement_input: String,
    #[dsl(coord)]
    pub camera_position: [f64; 3],
    #[dsl(coord)]
    pub camera_target: [f64; 3],
    pub camera_fov: f64,
}

impl Default for Process3dPresence {
    fn default() -> Self {
        Self { engagement_input: String::new(), camera_position: [3.0, -3.0, 2.0], camera_target: [0.0, 0.0, 0.0], camera_fov: 45.0 }
    }
}

/// 🔺️ Sparse field delta over [`Process3dPresence`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_fov: Option<f64>,
}

impl protocol::MutationDiff<Process3dPresence> for Process3dPresenceDiff {
    fn apply(&self, base: &Process3dPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Process3dPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.camera_position {
            next.camera_position = value.clone();
        }
        if let Some(value) = &self.camera_target {
            next.camera_target = value.clone();
        }
        if let Some(value) = &self.camera_fov {
            next.camera_fov = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.camera_position.is_some() {
            self.camera_position = other.camera_position;
        }
        if other.camera_target.is_some() {
            self.camera_target = other.camera_target;
        }
        if other.camera_fov.is_some() {
            self.camera_fov = other.camera_fov;
        }
    }
}

impl protocol::DiffAlgebra<Process3dPresence> for Process3dPresenceDiff {
    fn inverse(&self, base: &Process3dPresence) -> Self {
        Self {
            engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()),
            camera_position: self.camera_position.as_ref().map(|_| base.camera_position.clone()),
            camera_target: self.camera_target.as_ref().map(|_| base.camera_target.clone()),
            camera_fov: self.camera_fov.as_ref().map(|_| base.camera_fov.clone()),
        }
    }
    fn between(base: &Process3dPresence, other: &Process3dPresence) -> Self {
        Self {
            engagement_input: (base.engagement_input != other.engagement_input).then(|| other.engagement_input.clone()),
            camera_position: (base.camera_position != other.camera_position).then(|| other.camera_position.clone()),
            camera_target: (base.camera_target != other.camera_target).then(|| other.camera_target.clone()),
            camera_fov: (base.camera_fov != other.camera_fov).then(|| other.camera_fov.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.camera_position.is_none() && self.camera_target.is_none() && self.camera_fov.is_none()
    }
}

impl store::ArtifactDsl for Process3dPresence {
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

impl ArtifactPack for Process3dPresence {
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
pub enum Process3dPresenceMutation {
    #[dsl(key = "engagement-input")]
    SetEngagementInput {
        value: String,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
}

impl Mutation<Process3dPresence> for Process3dPresenceMutation {
    type Diff = Process3dPresenceDiff;

    /// 🧾️ Leaf metadata for the single presence verb. ⚠️ PROVISIONAL: the `owner` path below names
    /// no directory on disk — this enum has no `👥️presence/<slug>` leaf triad of its own, so the
    /// entry is a metadata placeholder to satisfy `protocol::Mutation`, matching `🪵️sourcing`'s
    /// own presence precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-engagement-input",
        semantic_kind: "set-engagement-input",
        display_name: "Set Engagement Input",
        emoji: "📄",
        aggregate_variant: "SetEngagementInput",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }]
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-camera",
        semantic_kind: "set-camera",
        display_name: "Set Camera",
        emoji: "📄",
        aggregate_variant: "SetCamera",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }]

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetEngagementInput { .. } => &Self::DESCRIPTORS[0],
            Self::SetCamera { .. } => &Self::DESCRIPTORS[1],
        }
    }

    fn diff(&self, base: &Process3dPresence) -> protocol::MutationOutcome<Process3dPresenceDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetEngagementInput { value } => Process3dPresenceDiff { engagement_input: (base.engagement_input != *value).then(|| value.clone()), ..Default::default() },
            Self::SetCamera { position, target, fov } => Process3dPresenceDiff { camera_position: (base.camera_position != *position).then_some(*position), camera_target: (base.camera_target != *target).then_some(*target), camera_fov: (base.camera_fov != *fov).then_some(*fov), ..Default::default() },
        })
    }

    fn inverse(&self, base: &Process3dPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetEngagementInput { .. } => Self::SetEngagementInput { value: base.engagement_input.clone() },
            Self::SetCamera { .. } => Self::SetCamera { position: base.camera_position, target: base.camera_target, fov: base.camera_fov },
        }])
    }
}

impl protocol::OpText for Process3dPresenceMutation {
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
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
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

impl protocol::OpBinary for Process3dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧹️Retirement
/// 📏️ The one heap owner a `Process3dPresence` root can hold: `engagement_input`'s UTF-8 buffer.
/// Every other field is a plain `f64` array, so a root whose engagement input is empty owns nothing
/// beyond its own inline bytes — the terminal-empty predicate the close lane fences on.
pub fn process3d_presence_is_terminal_empty(presence: &Process3dPresence) -> bool {
    presence.engagement_input.is_empty()
}

/// 👥️ Exact local and peer root ownership for process3d presence: one bounded turn returns the
/// variable-length engagement input, a second returns the inline root. Without it (and the disposer
/// below) every close of a registry-backed app faulted `interactive-job.close-owned-disposer-missing`
/// and the whole unit-test binary aborted in the fixture's `Drop` (ticket 26/09/15/DEV-PROCESS-REACT-E2E).
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Process3dPresenceRetirementFactory;

impl store::SnapshotRetirementFactory<Process3dPresence> for Process3dPresenceRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<Process3dPresence>) -> usize { std::mem::size_of::<Process3dPresenceRetirement>() }

    fn retire(&self, root: std::sync::Arc<Process3dPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(Process3dPresenceRetirement { root: std::mem::ManuallyDrop::new(Some(root)), engagement_input: std::mem::ManuallyDrop::new(None) })
    }
}

struct Process3dPresenceRetirement {
    root: std::mem::ManuallyDrop<Option<std::sync::Arc<Process3dPresence>>>,
    engagement_input: std::mem::ManuallyDrop<Option<String>>,
}

impl store::ErasedSnapshotRetirement for Process3dPresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if let Some(root) = self.root.take() {
            let released = std::sync::Arc::into_inner(root).map(|value| value.engagement_input).filter(|input| !input.is_empty());
            let released_bytes = released.as_ref().map_or(0, String::len);
            if released_bytes > maximum_bytes {
                self.engagement_input = std::mem::ManuallyDrop::new(released);
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(released);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        if let Some(input) = self.engagement_input.take() {
            let released_bytes = input.len();
            if released_bytes > maximum_bytes {
                self.engagement_input = std::mem::ManuallyDrop::new(Some(input));
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            drop(input);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.root.is_none() && self.engagement_input.is_none()
    }
}

impl Drop for Process3dPresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.root.is_none() && self.engagement_input.is_none(), "process3d presence retirement requires its exact terminal-empty witness");
        }
    }
}

/// 🧹️ The close-lane disposer paired with [`Process3dPresenceRetirementFactory`].
pub fn process3d_presence_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Process3dPresence, Process3dPresenceMutation>>> {
    Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Process3dPresence::default()), process3d_presence_is_terminal_empty).expect("the default process3d presence root holds no engagement input"))
}
//#endregion 🧹️Retirement

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = Process3dPresence::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dPresenceMutation::SetEngagementInput { value: "typing".into() }, &base).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dPresenceMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 1.0, 0.0], fov: 30.0 }, &base).await;
    }
}
