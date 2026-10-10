//! 🪟️ One macro for every persisted-local window configuration of the BIM surfaces: given a `DslRecord` + `DslArtifact` state struct it
//! derives the text/pack codecs, the sparse per-field diff, the `Replace` mutation (a `replace-<entity>` kind: payload is the window's whole configuration,
//! diff is only the fields that differ from the base, inverse is the same kind carrying the base configuration), the op codecs, the window-config owner
//! and the three accessors (`current`, `from_snapshot`, `addressed`). Editor and viewer windows share it; each invocation owns its module.

/// 🪟️ Declares one window configuration's sparse diff (`store::sparse_record_diff!`), its `ConfigRecord` mark, codecs, `Replace` mutation, owner and accessors.
#[macro_export]
macro_rules! bim_window_config {
    (config: $config:ident, diff: $diff:ident, mutation: $mutation:ident, owner: $owner:ident, window: $window:expr, schema: $schema:literal, owner_path: $path:literal, display: $display:literal, bytes: $bytes:expr, fields: { $($field:ident : $ty:ty),+ $(,)? } $(,)?) => {
        impl store::ConfigRecord for $config {}

        store::sparse_record_diff! { record: $config, diff: $diff, fields: { $($field: $ty),+ } }

        impl store::ArtifactDsl for $config {
            const EXTENSION: &'static str = Self::__DSL_EXTENSION;
            fn envelope_id() -> &'static str {
                Self::__DSL_ENVELOPE_ID
            }
            fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
                Self::__dsl_from_record(&record)
            }
            fn print_dsl(&self) -> String {
                let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid BIM window configuration envelope");
                store::semio_format::wrap_text(&envelope, &body)
            }
        }

        impl store::ArtifactPack for $config {
            fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
                Ok(store::semio_format::wrap_binary(&envelope, &body))
            }
            fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
                    return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
                }
                let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
            }
            fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }

        #[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
        #[value(tag = "kind", rename_all = "kebab-case")]
        pub enum $mutation {
            Replace { config: $config },
        }

        impl protocol::Mutation<$config> for $mutation {
            type Diff = $diff;
            const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
                schema_version: 1,
                owner: $path,
                semantic_kind: "set-window-config",
                display_name: $display,
                emoji: "🎚️",
                aggregate_variant: "Replace",
                payload_schema: $schema,
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
            fn diff(&self, base: &$config) -> protocol::MutationOutcome<Self::Diff> {
                let Self::Replace { config } = self;
                protocol::MutationOutcome::new(<$diff>::changing(base, config))
            }
            fn inverse(&self, base: &$config) -> Result<Vec<Self>, semio_framework_value::ValueError> {
                Ok(vec![Self::Replace { config: base.clone() }])
            }
        }

        impl protocol::OpText for $mutation {
            fn print_op(&self) -> String {
                semio_framework_pack_json::to_json_string(self)
            }
            fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
                semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
            }
        }

        impl protocol::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                Ok(protocol::OpText::print_op(self).into_bytes())
            }
            fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
                semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
            }
        }

        pub struct $owner;

        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $window;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = $bytes;
            type State = $config;
            type Mutation = $mutation;
            fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
                semio_framework_plugin::bounded_window_config_store_owners::<Self>()
            }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
            }
            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }

        /// 🎚️ The addressed window's retained configuration, or the default while it has never been written.
        pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> $config {
            view.window::<$owner>().cloned().unwrap_or_default()
        }

        /// 🎚️ The retained configuration inside a tool job's window-config snapshot, or the default (also for a snapshot of another window kind).
        pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> $config {
            snapshot.filter(|snapshot| snapshot.window_kind_id() == $window).and_then(|snapshot| snapshot.get::<$owner>()).cloned().unwrap_or_default()
        }

        /// 🎚️ Binds `config` to the exact window instance the gesture came from; no window, a stale window or a foreign window kind is a refusal.
        pub fn addressed(view: &semio_framework_plugin::ViewModel, config: $config) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
            let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("bim-window-required"))?;
            let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("bim-window-stale"))?;
            if kind != $window {
                return Err(semio_framework_plugin::Fault::from("bim-window-kind-mismatch"));
            }
            Ok(semio_framework_plugin::WindowConfigMutation::of::<$owner>(id, $mutation::Replace { config }))
        }
    };
}

/// ⚖️ The laws every window configuration owes: the `Replace` mutation applies through its sparse diff, its concrete inverse restores the base, the inverse sums to
/// the negative diff, and the mutation and the state survive their text and binary codecs.
#[cfg(test)]
pub async fn assert_window_config_laws<S, D, M>(base: &S, mutation: &M)
where
    S: store::ArtifactDsl + store::ArtifactPack + Clone + std::fmt::Debug + PartialEq + 'static,
    D: protocol::MutationDiff<S> + 'static,
    M: protocol::Mutation<S, Diff = D> + protocol::OpText + protocol::OpBinary + Clone + std::fmt::Debug + PartialEq,
{
    use protocol::{Mutation, OpBinary, OpText};
    let after = protocol::apply_diff(mutation.diff(base).diff(), base).expect("the mutation applies");
    let restored = mutation.inverse(base).expect("the mutation has an inverse").into_iter().rev().fold(after.clone(), |state, inverse| protocol::apply_diff(inverse.diff(&state).diff(), &state).expect("the inverse applies"));
    assert_eq!(&restored, base, "the concrete inverse restores the base");
    assert_eq!(M::parse_op(&mutation.print_op()).expect("op text parses"), *mutation);
    assert_eq!(M::decode_op(&mutation.encode_op().expect("op encodes")).expect("op decodes"), *mutation);
    assert_eq!(S::parse_dsl(&after.print_dsl()).expect("dsl parses"), after);
    assert_eq!(S::decode_pack(&after.encode_pack()).expect("pack decodes"), after);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(mutation, base).await;
}
