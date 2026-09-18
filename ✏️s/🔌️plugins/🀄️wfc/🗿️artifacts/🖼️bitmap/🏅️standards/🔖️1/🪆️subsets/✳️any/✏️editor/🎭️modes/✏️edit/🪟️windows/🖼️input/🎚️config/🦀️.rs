//! 🎚️ Persisted local configuration for one exact WFC Bitmap Input window — which palette colour a
//! stroke paints, how far the canvas is zoomed, and the IN-FLIGHT stroke's accumulated bounds. One
//! instance PER PANE, so two input panes can hold two different brushes over the same document.
//!
//! The in-flight stroke lives HERE and not in the app transient for one structural reason:
//! `ArtifactEditor::handle` is handed a `ConfigView` but no `TransientView`, so the window config is
//! the only lane a command can both write on pointer-down and READ again on release. Every
//! begin/extend write carries a coalesce key, so a two-hundred-sample drag folds into ONE config
//! edit rather than two hundred — the ledger discipline a per-sample amend would break — and ZERO
//! document operations until the gesture settles.

use semio_framework_value_derive::{FromValue, ToValue};

/// 🖌️ The inclusive bounding box a gesture has covered so far, in input-bitmap cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct BitmapStroke {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

impl BitmapStroke {
    /// 🖌️ A gesture that has touched exactly one cell.
    pub fn at(x: u32, y: u32) -> Self {
        Self { min_x: x, min_y: y, max_x: x, max_y: y }
    }

    /// 🖌️ Grows the box to include one more sampled cell — the whole accumulation a drag performs.
    pub fn extended(self, x: u32, y: u32) -> Self {
        Self { min_x: self.min_x.min(x), min_y: self.min_y.min(y), max_x: self.max_x.max(x), max_y: self.max_y.max(y) }
    }

    pub fn width(self) -> u32 {
        self.max_x - self.min_x + 1
    }

    pub fn height(self) -> u32 {
        self.max_y - self.min_y + 1
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.wfc.bitmap.inputwindowconfig", extension = "wfcbitmapinputwindowcfg", layout = "lines")]
pub struct BitmapInputWindowConfig {
    pub active_color: u32,
    pub zoom: f64,
    #[dsl(block)]
    pub stroke: Option<BitmapStroke>,
}

impl Default for BitmapInputWindowConfig {
    fn default() -> Self {
        Self { active_color: 0, zoom: 12.0, stroke: None }
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum BitmapInputWindowConfigMutation {
    Snapshot { config: BitmapInputWindowConfig },
}

impl protocol::Mutation<BitmapInputWindowConfig> for BitmapInputWindowConfigMutation {
    type Diff = BitmapInputWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config",
        semantic_kind: "set-window-config",
        display_name: "Set Bitmap Input Window Configuration",
        emoji: "🎚️",
        aggregate_variant: "Snapshot",
        payload_schema: "wfcbitmap.inputwindowconfig",
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
    fn diff(&self, _base: &BitmapInputWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }
    fn inverse(&self, base: &BitmapInputWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}

impl store::ArtifactDsl for BitmapInputWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid bitmap input window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for BitmapInputWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(BitmapInputWindowConfig);

impl protocol::OpText for BitmapInputWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for BitmapInputWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
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
pub fn addressed(view: &semio_framework_plugin::ViewModel, config: BitmapInputWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("wfc-bitmap-input-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("wfc-bitmap-window-stale"))?;
    if kind != super::WFC_BITMAP_WINDOW_INPUT {
        return Err(semio_framework_plugin::Fault::from("wfc-bitmap-input-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<BitmapInputWindowConfigOwner>(id, BitmapInputWindowConfigMutation::Snapshot { config }))
}
