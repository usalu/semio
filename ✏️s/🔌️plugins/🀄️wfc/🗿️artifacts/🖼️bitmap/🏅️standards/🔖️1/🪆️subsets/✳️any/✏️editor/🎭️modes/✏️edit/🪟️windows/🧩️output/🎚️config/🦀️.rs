//! 🎚️ Persisted local configuration for one exact WFC Bitmap Output window — whether the pinned
//! cells are marked over the inferred image and how far the canvas is zoomed. One instance PER
//! PANE, so a second output pane can hold its own zoom over the same solve.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.wfc.bitmap.outputwindowconfig", extension = "wfcbitmapoutputwindowcfg")]
pub struct BitmapOutputWindowConfig {
    pub show_pins: bool,
    pub zoom: f64,
}

impl Default for BitmapOutputWindowConfig {
    fn default() -> Self {
        Self { show_pins: true, zoom: 8.0 }
    }
}

/// 🔺️ Field-sparse diff of [`BitmapOutputWindowConfig`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct BitmapOutputWindowConfigDiff {
    pub show_pins: Option<bool>,
    pub zoom: Option<f64>,
}

impl protocol::DiffAlgebra<BitmapOutputWindowConfig> for BitmapOutputWindowConfigDiff {
    fn inverse(&self, base: &BitmapOutputWindowConfig) -> Self {
        Self {
            show_pins: self.show_pins.as_ref().map(|_| base.show_pins.clone()),
            zoom: self.zoom.as_ref().map(|_| base.zoom.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.show_pins.is_none() && self.zoom.is_none()
    }
}

impl protocol::MutationDiff<BitmapOutputWindowConfig> for BitmapOutputWindowConfigDiff {
    fn apply(&self, base: &BitmapOutputWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<BitmapOutputWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.show_pins {
            next.show_pins.clone_from(value);
        }
        if let Some(value) = &self.zoom {
            next.zoom.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.show_pins.is_some() {
            self.show_pins = later.show_pins;
        }
        if later.zoom.is_some() {
            self.zoom = later.zoom;
        }
    }
}

/// 🪟️ The bitmap output window configuration's mutation vocabulary: one absolute setter per field group.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum BitmapOutputWindowConfigMutation {
    SetShowPins { show_pins: bool },
    SetZoom { zoom: f64 },
}

impl protocol::Mutation<BitmapOutputWindowConfig> for BitmapOutputWindowConfigMutation {
    type Diff = BitmapOutputWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config",
            semantic_kind: "set-show-pins",
            display_name: "Set Show Pins",
            emoji: "🪟️",
            aggregate_variant: "SetShowPins",
            payload_schema: "wfcbitmap.outputwindowconfig",
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
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config",
            semantic_kind: "set-zoom",
            display_name: "Set Zoom",
            emoji: "🪟️",
            aggregate_variant: "SetZoom",
            payload_schema: "wfcbitmap.outputwindowconfig",
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
            Self::SetShowPins { .. } => &Self::DESCRIPTORS[0],
            Self::SetZoom { .. } => &Self::DESCRIPTORS[1],
        }
    }
    fn diff(&self, base: &BitmapOutputWindowConfig) -> protocol::MutationOutcome<BitmapOutputWindowConfigDiff> {
        let diff = match self {
            Self::SetShowPins { show_pins } => BitmapOutputWindowConfigDiff { show_pins: (base.show_pins != *show_pins).then(|| show_pins.clone()), ..Default::default() },
            Self::SetZoom { zoom } => BitmapOutputWindowConfigDiff { zoom: (base.zoom != *zoom).then(|| zoom.clone()), ..Default::default() },
        };
        if protocol::DiffAlgebra::<BitmapOutputWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds that value.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &BitmapOutputWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetShowPins { .. } => Self::SetShowPins { show_pins: base.show_pins.clone() },
            Self::SetZoom { .. } => Self::SetZoom { zoom: base.zoom.clone() },
        }])
    }
}
impl store::ArtifactDsl for BitmapOutputWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid bitmap output window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BitmapOutputWindowConfig {
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

impl store::ConfigRecord for BitmapOutputWindowConfig {}

impl protocol::OpText for BitmapOutputWindowConfigMutation {
    fn print_op(&self) -> String {
        crate::standards::v1::subsets::any::io::text::bitmap_json_encode(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        crate::standards::v1::subsets::any::io::text::bitmap_json_decode(line).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for BitmapOutputWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

impl semio_framework_plugin::WindowConfigApplyMutation<BitmapOutputWindowConfig> for BitmapOutputWindowConfigMutation {
    fn exchange(self, post: &mut BitmapOutputWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetShowPins { mut show_pins } => {
                std::mem::swap(&mut show_pins, &mut post.show_pins);
                Self::SetShowPins { show_pins }
            }
            Self::SetZoom { mut zoom } => {
                std::mem::swap(&mut zoom, &mut post.zoom);
                Self::SetZoom { zoom }
            }
        })
    }
}

pub struct BitmapOutputWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for BitmapOutputWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::WFC_BITMAP_WINDOW_OUTPUT;
    const SCHEMA: &'static str = "wfcbitmap.outputwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = BitmapOutputWindowConfig;
    type Mutation = BitmapOutputWindowConfigMutation;
    type Edit = semio_framework_plugin::WindowConfigApplyEdit<BitmapOutputWindowConfig, BitmapOutputWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
        std::sync::Arc::new(semio_framework_plugin::WindowConfigApplyEdit::new())
    }
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

/// 🎚️ This window instance's persisted configuration, or the default when it has none yet.
pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> BitmapOutputWindowConfig {
    view.window::<BitmapOutputWindowConfigOwner>().cloned().unwrap_or_default()
}

/// 🎯️ Addresses one config write at the window instance being dispatched — refuses outright when
/// the active window is not an input window, rather than writing a zoom into some other pane.
pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: BitmapOutputWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.output.window-required"), "wfc.bitmap.output.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.window.stale"), "wfc.bitmap.window.stale"))?;
    if kind != super::WFC_BITMAP_WINDOW_OUTPUT {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.bitmap.output.window-kind-required"), "wfc.bitmap.output.window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<BitmapOutputWindowConfigOwner>(id, mutation))
}
