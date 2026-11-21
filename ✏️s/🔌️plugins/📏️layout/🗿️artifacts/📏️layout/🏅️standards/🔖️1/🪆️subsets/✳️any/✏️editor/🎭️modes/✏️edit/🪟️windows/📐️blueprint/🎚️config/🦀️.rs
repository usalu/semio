//! 🎚️ Persisted local configuration for one exact Layout Blueprint or Preview window.

use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use semio_s_artifact_layout_layout_model::LayoutCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::NativeReceive, Clone, Debug, PartialEq, ToValue, FromValue, semio_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.layout.layout.windowconfig", extension = "layoutwindowcfg")]
pub struct LayoutWindowConfig {
    pub active_page_id: String,
    /// 🧰️ Host utility bar id. `select` is the pointer; `transform` arms the gumball.
    pub active_utility: String,
    #[dsl(block)]
    pub camera: LayoutCamera,
}

const DEFAULT_PAGE_ID: &str = "page-1";
const DEFAULT_UTILITY: &str = "select";

impl Default for LayoutWindowConfig {
    fn default() -> Self {
        Self { active_page_id: DEFAULT_PAGE_ID.into(), active_utility: DEFAULT_UTILITY.into(), camera: LayoutCamera::default() }
    }
}

/// 🩹 Owned-field patch of [`LayoutWindowConfig`]: exactly the fields an update sets. It is both the update payload and the sparse diff.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
pub struct LayoutWindowConfigPatch {
    pub active_page_id: Option<String>,
    pub active_utility: Option<String>,
    pub camera: Option<LayoutCamera>,
}

impl LayoutWindowConfigPatch {
    /// 🩹 The patch that sets every field of `config`.
    pub fn replacing(config: &LayoutWindowConfig) -> Self {
        Self { active_page_id: Some(config.active_page_id.clone()), active_utility: Some(config.active_utility.clone()), camera: Some(config.camera.clone()) }
    }

    /// 🔎️ This patch reduced to the fields that differ from `base`.
    pub fn against(&self, base: &LayoutWindowConfig) -> Self {
        Self {
            active_page_id: self.active_page_id.clone().filter(|value| *value != base.active_page_id),
            active_utility: self.active_utility.clone().filter(|value| *value != base.active_utility),
            camera: self.camera.clone().filter(|value| *value != base.camera),
        }
    }
}

impl protocol::MutationDiff<LayoutWindowConfig> for LayoutWindowConfigPatch {}

impl protocol::ColdMutationDiff<LayoutWindowConfig> for LayoutWindowConfigPatch {
    fn absorb(&mut self, other: Self) {
        self.active_page_id = other.active_page_id.or_else(|| self.active_page_id.take());
        self.active_utility = other.active_utility.or_else(|| self.active_utility.take());
        self.camera = other.camera.or_else(|| self.camera.take());
    }

    fn apply(&self, unpublished: &mut LayoutWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        *unpublished = LayoutWindowConfig {
            active_page_id: self.active_page_id.clone().unwrap_or_else(|| base.active_page_id.clone()),
            active_utility: self.active_utility.clone().unwrap_or_else(|| base.active_utility.clone()),
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
        };
        Ok(())
    }
}

impl protocol::DiffAlgebra<LayoutWindowConfig> for LayoutWindowConfigPatch {
    fn is_empty(&self) -> bool {
        self.active_page_id.is_none() && self.active_utility.is_none() && self.camera.is_none()
    }
}

impl protocol::ColdDiffAlgebra<LayoutWindowConfig> for LayoutWindowConfigPatch {
    fn inverse(&self, base: &LayoutWindowConfig) -> Self {
        Self { active_page_id: self.active_page_id.as_ref().map(|_| base.active_page_id.clone()), active_utility: self.active_utility.as_ref().map(|_| base.active_utility.clone()), camera: self.camera.as_ref().map(|_| base.camera.clone()) }
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum LayoutWindowConfigMutation {
    Update { patch: LayoutWindowConfigPatch },
}

impl protocol::Mutation<LayoutWindowConfig> for LayoutWindowConfigMutation {
    type Diff = LayoutWindowConfigPatch;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Layout Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Update",
        payload_schema: "layout.windowconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
}

impl protocol::ColdMutation<LayoutWindowConfig> for LayoutWindowConfigMutation {
    type ReferenceDiff = LayoutWindowConfigPatch;
    fn diff(&self, base: &LayoutWindowConfig) -> protocol::MutationOutcome<Self::ReferenceDiff> {
        let Self::Update { patch } = self;
        protocol::MutationOutcome::new(patch.against(base))
    }
    fn inverse(&self, base: &LayoutWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        let Self::Update { patch } = self;
        let changed = patch.against(base);
        Ok(if changed == LayoutWindowConfigPatch::default() { Vec::new() } else { vec![Self::Update { patch: protocol::ColdDiffAlgebra::inverse(&changed, base) }] })
    }
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<LayoutWindowConfig> for LayoutWindowConfigMutation {
    fn exchange(self, post: &mut LayoutWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::Update { mut patch } = self;
        if let Some(value) = patch.active_page_id.as_mut() {
            std::mem::swap(value, &mut post.active_page_id);
        }
        if let Some(value) = patch.active_utility.as_mut() {
            std::mem::swap(value, &mut post.active_utility);
        }
        if let Some(value) = patch.camera.as_mut() {
            std::mem::swap(value, &mut post.camera);
        }
        Ok(Self::Update { patch })
    }
    fn payload_bytes(&self) -> usize {
        let Self::Update { patch } = self;
        patch.active_page_id.as_ref().map_or(0, String::len) + patch.active_utility.as_ref().map_or(0, String::len)
    }
}

/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl pack::snapshot::ArtifactDsl for LayoutWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Layout window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl pack::snapshot::ArtifactPack for LayoutWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for LayoutWindowConfig {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

impl store::ConfigRecord for LayoutWindowConfig {}

impl store::os_store::codec_registry::native_snapshot::OriginalArtifactPackReceiving for LayoutWindowConfig {
    type Receiving = store::os_store::codec_registry::native_snapshot::OriginalSnapshotRecordReceiving<Self>;

    fn original_pack_birth_demand() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Self::Receiving>(), depth: 1, ..Default::default() }
    }

    fn admit_original_pack(source: &[u8], limits: protocol::PackLimits, native: &semio_framework_value::NativeDecodeControl<'_>, grant: semio_framework_value::RetainedCloneGrant) -> Result<Option<(Self::Receiving, semio_framework_value::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        Self::Receiving::admit(source, <Self as pack::snapshot::ArtifactDsl>::envelope_id(), limits, native, grant)
    }
}

impl protocol::OpText for LayoutWindowConfigMutation {
    fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

semio_os_kernel::original_pack_value_operation_receiving!(LayoutWindowConfigMutation);

impl protocol::OpBinary for LayoutWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| protocol::ProtocolError::Malformed { what: "artifact operation", offset: 0, detail: error.to_string() })
    }
}

macro_rules! config_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_plugin::WindowConfigOwner for $owner {
            fn initial_state_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
                Ok(semio_framework_value::RetirementDemand {
                    copy_bytes: 2 * std::mem::size_of::<Self::State>() + DEFAULT_PAGE_ID.len() + DEFAULT_UTILITY.len(),
                    capacity_bytes: DEFAULT_PAGE_ID.len() + DEFAULT_UTILITY.len(),
                    depth: 1,
                    ..Default::default()
                })
            }

            fn initial_state(grant: semio_framework_value::RetainedCloneGrant) -> Result<(Self::State, semio_framework_value::RetainedCloneProgress), semio_framework_value::ValueError> {
                let demand = Self::initial_state_demands()?;
                if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
                    return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original window config birth is not funded"));
                }
                let state = Self::State::default();
                let capacity = state.active_page_id.capacity() + state.active_utility.capacity();
                Ok((state, semio_framework_value::RetainedCloneProgress {
                    copied_items: 1,
                    copied_bytes: demand.copy_bytes,
                    retained_capacity_bytes: capacity,
                    ..Default::default()
                }))
            }

    fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),semio_framework_value::ValueError>{Ok((state,<Self::State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "layout.windowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = LayoutWindowConfig;
            type Mutation = LayoutWindowConfigMutation;
            fn native_pair_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
                Ok(semio_plugin::window_config_native_pair_demands::<Self::State>())
            }

            fn admit_native_pair(pack: &[u8], spr: &[u8], pack_limits: protocol::PackLimits, history_limits: store::os_store::codec_registry::native_snapshot::OriginalArtifactHistoryReadLimits, native: &mut semio_framework_value::NativeDecodeControl<'_>, grant: semio_framework_value::RetainedCloneGrant, output: &mut Option<Box<dyn semio_plugin::WindowConfigNativePairReceiving<Self::State>>>) -> Result<semio_framework_value::RetainedCloneProgress, store::os_store::codec_registry::native_snapshot::OriginalArtifactPairFailure> {
                semio_plugin::admit_window_config_native_pair::<Self::State>(pack, spr, pack_limits, history_limits, native, grant, output)
            }

            fn native_operation_wire() -> store::os_store::ArtifactOperationWire<Self::Mutation> {
                store::os_store::ArtifactOperationWire::canonical().with_original_receiving()
            }

            type Edit = store::snapshot_clone_preparation::ConfigApplyEdit<LayoutWindowConfig, LayoutWindowConfigMutation>;
            const MAXIMUM_PREPARATION_DEPTH: usize = 64;
            fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
                std::sync::Arc::new(store::snapshot_clone_preparation::ConfigApplyEdit::new())
            }
            fn preparation_source_birth() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_plugin::bounded_window_config_preparation_source_birth::<Self>() }
    fn build_store_owners(grant: semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State, Self::Mutation>> { semio_plugin::bounded_window_config_store_owners::<Self>(grant) }
            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_plugin::bounded_window_config_preparation_factory::<Self>() }
            fn store_disposer_birth() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
                semio_plugin::bounded_window_config_store_disposer_birth::<Self>()
            }
            fn build_store_disposer() -> Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_plugin::bounded_window_config_store_disposer::<Self>() }
        }
    };
}

config_owner!(LayoutBlueprintWindowConfigOwner, super::LAYOUT_PLAY_WINDOW_BLUEPRINT);
config_owner!(LayoutPreviewWindowConfigOwner, crate::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW);

pub fn register(registry: &mut semio_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_plugin::Fault> {
    registry.register::<LayoutBlueprintWindowConfigOwner>()?;
    registry.register::<LayoutPreviewWindowConfigOwner>()
}

fn from_owner<O: semio_plugin::WindowConfigOwner<State = LayoutWindowConfig, Mutation = LayoutWindowConfigMutation>>(snapshot: &semio_plugin::WindowConfigSnapshot) -> Option<LayoutWindowConfig> {
    snapshot.get::<O>().cloned()
}

pub fn from_snapshot(snapshot: Option<&semio_plugin::WindowConfigSnapshot>) -> LayoutWindowConfig {
    let Some(snapshot) = snapshot else { return LayoutWindowConfig::default() };
    match snapshot.window_kind_id() {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => from_owner::<LayoutBlueprintWindowConfigOwner>(snapshot),
        crate::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => from_owner::<LayoutPreviewWindowConfigOwner>(snapshot),
        _ => None,
    }.unwrap_or_default()
}

pub fn current<C>(view: &semio_plugin::ConfigView<'_, C>) -> LayoutWindowConfig {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_plugin::ViewModel, config: LayoutWindowConfig) -> Result<semio_plugin::WindowConfigMutation, semio_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_plugin::Fault::from("layout-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_plugin::Fault::from("layout-window-stale"))?;
    let mutation = LayoutWindowConfigMutation::Update { patch: LayoutWindowConfigPatch::replacing(&config) };
    match kind {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => Ok(semio_plugin::WindowConfigMutation::of::<LayoutBlueprintWindowConfigOwner>(id, mutation)),
        crate::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => Ok(semio_plugin::WindowConfigMutation::of::<LayoutPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_plugin::Fault::from("layout-window-kind-required")),
    }
}

