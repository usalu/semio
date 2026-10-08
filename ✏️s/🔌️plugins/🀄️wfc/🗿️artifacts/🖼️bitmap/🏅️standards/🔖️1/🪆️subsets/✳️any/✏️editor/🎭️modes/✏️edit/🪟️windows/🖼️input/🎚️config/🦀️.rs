//! 🎚️ Persisted local configuration for one exact WFC Bitmap Input window — which palette colour the brush
//! paints and how far the canvas is zoomed. One instance PER PANE, so two input panes can hold two different
//! brushes over the same document. The stroke in flight is tool state and lives in the window TRANSIENT
//! (`🫧️transient`), never here: config is persisted and has its own ledger, a gesture is neither.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.wfc.bitmap.inputwindowconfig", extension = "wfcbitmapinputwindowcfg")]
pub struct BitmapInputWindowConfig {
    pub active_color: u32,
    pub zoom: f64,
}

impl Default for BitmapInputWindowConfig {
    fn default() -> Self {
        Self { active_color: 0, zoom: 12.0 }
    }
}

/// 🔺️ Field-sparse diff of [`BitmapInputWindowConfig`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct BitmapInputWindowConfigDiff {
    pub active_color: Option<u32>,
    pub zoom: Option<f64>,
}

impl protocol::DiffAlgebra<BitmapInputWindowConfig> for BitmapInputWindowConfigDiff {
    fn inverse(&self, base: &BitmapInputWindowConfig) -> Self {
        Self {
            active_color: self.active_color.as_ref().map(|_| base.active_color.clone()),
            zoom: self.zoom.as_ref().map(|_| base.zoom.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.active_color.is_none() && self.zoom.is_none()
    }
}

impl protocol::MutationDiff<BitmapInputWindowConfig> for BitmapInputWindowConfigDiff {
    fn apply(&self, base: &BitmapInputWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<BitmapInputWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.active_color {
            next.active_color.clone_from(value);
        }
        if let Some(value) = &self.zoom {
            next.zoom.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.active_color.is_some() {
            self.active_color = later.active_color;
        }
        if later.zoom.is_some() {
            self.zoom = later.zoom;
        }
    }
}

/// 🪟️ The bitmap input window configuration's mutation vocabulary: one absolute setter per field group.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum BitmapInputWindowConfigMutation {
    SetActiveColor { color: u32 },
    SetZoom { zoom: f64 },
}

impl protocol::Mutation<BitmapInputWindowConfig> for BitmapInputWindowConfigMutation {
    type Diff = BitmapInputWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config",
            semantic_kind: "set-active-color",
            display_name: "Set Active Color",
            emoji: "🪟️",
            aggregate_variant: "SetActiveColor",
            payload_schema: "wfcbitmap.inputwindowconfig",
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
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config",
            semantic_kind: "set-zoom",
            display_name: "Set Zoom",
            emoji: "🪟️",
            aggregate_variant: "SetZoom",
            payload_schema: "wfcbitmap.inputwindowconfig",
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
            Self::SetActiveColor { .. } => &Self::DESCRIPTORS[0],
            Self::SetZoom { .. } => &Self::DESCRIPTORS[1],
        }
    }
    fn diff(&self, base: &BitmapInputWindowConfig) -> protocol::MutationOutcome<BitmapInputWindowConfigDiff> {
        let diff = match self {
            Self::SetActiveColor { color } => BitmapInputWindowConfigDiff { active_color: (base.active_color != *color).then(|| color.clone()), ..Default::default() },
            Self::SetZoom { zoom } => BitmapInputWindowConfigDiff { zoom: (base.zoom != *zoom).then(|| zoom.clone()), ..Default::default() },
        };
        if protocol::DiffAlgebra::<BitmapInputWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds that value.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &BitmapInputWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetActiveColor { .. } => Self::SetActiveColor { color: base.active_color.clone() },
            Self::SetZoom { .. } => Self::SetZoom { zoom: base.zoom.clone() },
        }])
    }
}
impl store::ArtifactDsl for BitmapInputWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid bitmap input window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BitmapInputWindowConfig {
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

impl store::ConfigRecord for BitmapInputWindowConfig {}

impl protocol::OpText for BitmapInputWindowConfigMutation {
    fn print_op(&self) -> String {
        crate::standards::v1::subsets::any::io::text::bitmap_json_encode(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        crate::standards::v1::subsets::any::io::text::bitmap_json_decode(line).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for BitmapInputWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

pub struct BitmapInputWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for BitmapInputWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::WFC_BITMAP_WINDOW_INPUT;
    const SCHEMA: &'static str = "wfcbitmap.inputwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = BitmapInputWindowConfig;
    type Mutation = BitmapInputWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

/// 🎚️ This window instance's persisted configuration, or the default when it has none yet.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> BitmapInputWindowConfig {
    view.window::<BitmapInputWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 🎯️ Addresses one config write at the window instance being dispatched — refuses outright when
/// the active window is not an input window, rather than writing a brush into some other pane.
pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: BitmapInputWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.input.window-required"), "wfc.bitmap.input.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.window.stale"), "wfc.bitmap.window.stale"))?;
    if kind != super::WFC_BITMAP_WINDOW_INPUT {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.input.window-kind-required"), "wfc.bitmap.input.window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<BitmapInputWindowConfigOwner>(id, mutation))
}
