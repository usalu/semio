//! 🧱️ Lane stores: the framework `Presence` and `Transient` facets of every live norm editor, each a single opaque `state` line that is the canonical JSON of [`NormPresence`] / [`NormTransient`]. See [`super`].

use super::{NormPresence, NormTransient};
use semio_framework_value::{FromValue, ToValue};

#[doc(hidden)]
pub use pack as __pack;

/// 🔄️ The canonical text of a lane payload; ephemeral lanes never persist, so the text is the only wire shape.
fn encode<T: ToValue>(value: &T) -> String {
    semio_framework_pack_json::to_json_string(&value.to_value())
}

fn decode<T: FromValue + Default>(state: &str) -> T {
    semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(state, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| T::from_value(value).ok()).unwrap_or_default()
}

/// 🫧️ A framework `Transient` record that carries a [`NormTransient`].
pub trait LaneTransient: Sized {
    type Mutation;
    fn lane(&self) -> NormTransient;
    fn replace(lane: &NormTransient) -> Self::Mutation;
}

/// 👥️ A framework `Presence` record that carries a [`NormPresence`].
pub trait LanePresence: Sized {
    type Mutation;
    fn lane(&self) -> NormPresence;
    fn replace(lane: &NormPresence) -> Self::Mutation;
}

/// 🔄️ Canonical lane text of a transient.
pub fn transient_state(lane: &NormTransient) -> String {
    encode(lane)
}

/// 🔄️ Lane text read back; unreadable text is the empty transient (no preview).
pub fn transient_of_state(state: &str) -> NormTransient {
    decode(state)
}

/// 🔄️ Canonical lane text of a presence.
pub fn presence_state(lane: &NormPresence) -> String {
    encode(lane)
}

/// 🔄️ Lane text read back; unreadable text is the anonymous presence.
pub fn presence_of_state(state: &str) -> NormPresence {
    decode(state)
}

impl Default for NormPresence {
    fn default() -> Self {
        Self::new("")
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __norm_lane_store {
    ($record:ident, $diff:ident, $mutation:ident, $lane_trait:ident, $lane_ty:ty, $state:ident, $of_state:ident, $extension:literal, $owner:literal, $emoji:literal, $semantic:literal, $display:literal, $presence:tt) => {
        #[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value_derive::NativeReceive)]
        #[value(rename_all = "camelCase", default)]
        #[artifact(extension = $extension)]
        #[dsl(layout = "lines")]
        pub struct $record {
            pub state: String,
        }

        #[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
        #[value(rename_all = "camelCase", default)]
        pub struct $diff {
            #[value(skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }

        impl protocol::MutationDiff<$record> for $diff {}

        impl protocol::ColdMutationDiff<$record> for $diff {
            fn absorb(&mut self, other: Self) {
                if other.state.is_some() {
                    self.state = other.state;
                }
            }

            fn apply(&self, unpublished: &mut $record, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(state) = &self.state {
                    unpublished.state = state.clone();
                }
                Ok(())
            }
        }

        impl protocol::DiffAlgebra<$record> for $diff {
            fn is_empty(&self) -> bool {
                self.state.is_none()
            }
        }

        impl protocol::ColdDiffAlgebra<$record> for $diff {
            fn inverse(&self, base: &$record) -> Self {
                Self { state: self.state.as_ref().map(|_| base.state.clone()) }
            }
        }

        impl $crate::__pack::snapshot::ArtifactDsl for $record {
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
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as $crate::__pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
                store::semio_format::wrap_text(&envelope, &body)
            }
        }

        impl $crate::__pack::snapshot::ArtifactPack for $record {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as $crate::__pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
                Ok(store::semio_format::wrap_binary(&envelope, &inner))
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                if bytes.is_empty() {
                    return Ok(Self::default());
                }
                let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity(<Self as $crate::__pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
                    return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as $crate::__pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
                }
                let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
            }
            fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }

        impl store::os_store::serialization::ArtifactStoreSnapshot for $record {
            fn reference_snapshot_codec() -> Option<store::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>> {
                Some(store::os_store::serialization::reference_snapshot_codec::<Self>())
            }
        }

        #[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
        #[value(rename_all = "camelCase")]
        pub enum $mutation {
            #[dsl(key = "set")]
            Set { state: String },
        }

        impl protocol::Mutation<$record> for $mutation {
            type Diff = $diff;

            const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
                schema_version: 1,
                owner: $owner,
                semantic_kind: $semantic,
                display_name: $display,
                emoji: $emoji,
                aggregate_variant: "Set",
                payload_schema: "🧬️schema/🔣️.json",
                text_opcode: None,
                binary_tag: None,
                invertibility: protocol::MutationInvertibility::ExplicitMutation,
                diff_participation: protocol::MutationDiffParticipation::Detect,
                outcome_classes: &[protocol::MutationOutcomeClass::Applied],
                composition: protocol::MutationComposition::Atomic,
                required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
            }];

            fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
                &Self::DESCRIPTORS[0]
            }
        }

        impl protocol::ColdMutation<$record> for $mutation {
            type ReferenceDiff = $diff;
            fn diff(&self, base: &$record) -> protocol::MutationOutcome<$diff> {
                match self {
                    Self::Set { state } => protocol::MutationOutcome::new($diff { state: (&base.state != state).then(|| state.clone()) }),
                }
            }
            fn inverse(&self, base: &$record) -> Result<Vec<Self>, semio_framework_value::ValueError> {
                Ok(vec![Self::Set { state: base.state.clone() }])
            }
        }

        impl protocol::OpText for $mutation {
            fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                semio_framework_dsl_record::variants_text::parse_op(line)
            }
            fn print_op(&self) -> String {
                semio_framework_dsl_record::variants_text::print_op(self)
            }
        }

        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                dsl::variants_binary::encode_op(self)
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                dsl::variants_binary::decode_op(bytes)
            }
        }

        impl $crate::$lane_trait for $record {
            type Mutation = $mutation;
            fn lane(&self) -> $lane_ty {
                $crate::$of_state(&self.state)
            }
            fn replace(lane: &$lane_ty) -> $mutation {
                $mutation::Set { state: $crate::$state(lane) }
            }
        }

        $crate::__norm_lane_store!(@presence $presence, $record);
    };
    (@presence true, $record:ident) => {
        impl store::ArtifactPresenceSnapshot for $record {
            fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<Self, store::io::control::NativeSnapshotFailure> {
                store::decode_presence_json(bytes, owner)
            }
        }
    };
    (@presence false, $record:ident) => {};
}

/// 🧱️ Declares the lane store types of one live norm editor in the invoking module: `NormLanePresence`, `NormLaneTransient` and their `Set` mutations. `owner` is the editor taxonomy path the mutation descriptors name.
#[macro_export]
macro_rules! norm_lane_stores {
    (presence_extension = $presence_extension:literal, transient_extension = $transient_extension:literal, owner = $owner:literal) => {
        $crate::__norm_lane_store!(NormLanePresence, NormLanePresenceDiff, NormLanePresenceMutation, LanePresence, $crate::NormPresence, presence_state, presence_of_state, $presence_extension, $owner, "👥️", "set-lane-presence", "Set Lane Presence", true);
        $crate::__norm_lane_store!(NormLaneTransient, NormLaneTransientDiff, NormLaneTransientMutation, LaneTransient, $crate::NormTransient, transient_state, transient_of_state, $transient_extension, $owner, "🫧️", "set-lane-transient", "Set Lane Transient", false);
    };
}

/// 🧱️ Declares the lane store of one read-only norm viewer in the invoking module: only `NormLaneTransient` and its `Set` mutation (ephemeral local-only, never shared, never persisted), no presence. `owner` is the viewer taxonomy path the mutation descriptor names.
#[macro_export]
macro_rules! norm_viewer_lane_stores {
    (transient_extension = $transient_extension:literal, owner = $owner:literal) => {
        $crate::__norm_lane_store!(NormLaneTransient, NormLaneTransientDiff, NormLaneTransientMutation, LaneTransient, $crate::NormTransient, transient_state, transient_of_state, $transient_extension, $owner, "🫧️", "set-lane-transient", "Set Lane Transient", false);
    };
}
