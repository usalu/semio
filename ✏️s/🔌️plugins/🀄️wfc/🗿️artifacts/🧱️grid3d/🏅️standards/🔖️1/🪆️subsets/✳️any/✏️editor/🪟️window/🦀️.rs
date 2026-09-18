//! 🪟️ Exact-instance persisted ownership for the two `s.wfc.grid3d` panes. The grid window and the
//! preview window each carry their OWN camera and their own armed tile, because one KIND can be laid
//! out as several INSTANCES and a pose is never shared between panes
//! (`Puzzle2dWindowConfig`'s precedent).

use crate::editor::grid3d::modes::edit::windows::{grid, preview};

//#region 🎚️Config
/// 🎚️ ONE exact pane's persisted-local options: its orbit pose, the tile its pin utility is armed
/// with, and whether masked cells stay drawn. `WindowConfigOwner::State` requires `dsl::DslField`,
/// which `#[derive(dsl::DslArtifact)]` emits alongside the `__dsl_*` helpers the record-backed
/// codecs below are written against.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(id = "s.wfc.grid3d.windowconfig", extension = "wfcgrid3dwindowcfg", layout = "lines")]
pub struct Grid3dWindowConfig {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub camera_zoom: f64,
    pub active_tile_id: String,
    pub show_masked: bool,
}

impl Default for Grid3dWindowConfig {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_z: 0.0, target_x: 0.0, target_y: 0.0, target_z: 0.0, camera_zoom: 1.0, active_tile_id: String::new(), show_masked: true }
    }
}

impl Grid3dWindowConfig {
    /// 📷️ Whether this pane has never been posed: an all-zero camera stands exactly on what it looks
    /// at, which is degenerate in any projection and therefore can never be a real user pose.
    pub fn camera_unset(&self) -> bool {
        (self.camera_x, self.camera_y, self.camera_z) == (self.target_x, self.target_y, self.target_z)
    }

    pub fn position(&self) -> [f64; 3] {
        [self.camera_x, self.camera_y, self.camera_z]
    }

    pub fn target(&self) -> [f64; 3] {
        [self.target_x, self.target_y, self.target_z]
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid3dWindowConfigMutation {
    Snapshot { config: Grid3dWindowConfig },
}

impl protocol::Mutation<Grid3dWindowConfig> for Grid3dWindowConfigMutation {
    type Diff = Grid3dWindowConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
        semantic_kind: "set-window-config",
        display_name: "Set 3D Grid Window Configuration",
        emoji: "🪟️",
        aggregate_variant: "Snapshot",
        payload_schema: "wfc.grid3dwindowconfig",
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
    fn diff(&self, _base: &Grid3dWindowConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.clone()),
        }
    }
    fn inverse(&self, base: &Grid3dWindowConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: base.clone() }]
    }
}
//#endregion 🎚️Config

//#region 🔖️HandcraftedCodecs
/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.
impl store::ArtifactDsl for Grid3dWindowConfig {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid wfc grid3d window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` would fail every retained load of this
/// window kind.
impl store::ArtifactPack for Grid3dWindowConfig {
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

store::impl_whole_record_config!(Grid3dWindowConfig);

impl protocol::OpText for Grid3dWindowConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for Grid3dWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️HandcraftedCodecs

//#region 🪟️Owners
macro_rules! config_owner {
    ($owner:ident, $kind:expr) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = "wfc.grid3dwindowconfig";
            const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
            type State = Grid3dWindowConfig;
            type Mutation = Grid3dWindowConfigMutation;
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
    };
}

config_owner!(Grid3dGridWindowConfigOwner, grid::WINDOW_KIND_ID);
config_owner!(Grid3dPreviewWindowConfigOwner, preview::WINDOW_KIND_ID);

pub fn register_config(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<Grid3dGridWindowConfigOwner>()?;
    registry.register::<Grid3dPreviewWindowConfigOwner>()
}

/// 🎚️ The config of the window being rendered or dispatched, falling back to the sibling pane's and
/// then to the default — the same "read the current window first" chain every multi-pane editor uses.
pub fn config_from_view(view: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> Grid3dWindowConfig {
    view.window::<Grid3dGridWindowConfigOwner>().or_else(|| view.window::<Grid3dPreviewWindowConfigOwner>()).cloned().unwrap_or_default()
}

/// 🪟️ The `(instance id, kind id)` of the window a dispatch is addressed to.
pub fn kind(view: &semio_framework_plugin::ViewModel) -> Option<(&str, &str)> {
    let id = view.window_id.as_deref().or(view.focused_window_id.as_deref())?;
    let kind = view.active_window_kind_id.as_deref()?;
    Some((id, kind))
}

pub fn kind_for_view(view: &semio_framework_plugin::ViewModel) -> Option<&str> {
    kind(view).map(|(_, kind)| kind)
}

/// 🎚️ Addresses a config write at the exact window instance it belongs to — a pane may never write
/// its sibling's pose.
pub fn addressed_config(view: &semio_framework_plugin::ViewModel, config: Grid3dWindowConfig) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let (id, kind) = kind(view).ok_or_else(|| semio_framework_plugin::Fault::from("wfc-grid3d-window-required"))?;
    let mutation = Grid3dWindowConfigMutation::Snapshot { config };
    match kind {
        grid::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid3dGridWindowConfigOwner>(id, mutation)),
        preview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid3dPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("wfc-grid3d-window-kind-required")),
    }
}
//#endregion 🪟️Owners
