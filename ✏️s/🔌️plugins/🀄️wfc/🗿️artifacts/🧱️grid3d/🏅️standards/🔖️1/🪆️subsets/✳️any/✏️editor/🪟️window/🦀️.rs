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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.wfc.grid3d.windowconfig", extension = "wfcgrid3dwindowcfg")]
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

/// 🔺️ Field-sparse diff of [`Grid3dWindowConfig`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Grid3dWindowConfigDiff {
    pub camera_x: Option<f64>,
    pub camera_y: Option<f64>,
    pub camera_z: Option<f64>,
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    pub target_z: Option<f64>,
    pub camera_zoom: Option<f64>,
    pub active_tile_id: Option<String>,
    pub show_masked: Option<bool>,
}

impl protocol::DiffAlgebra<Grid3dWindowConfig> for Grid3dWindowConfigDiff {
    fn inverse(&self, base: &Grid3dWindowConfig) -> Self {
        Self {
            camera_x: self.camera_x.as_ref().map(|_| base.camera_x.clone()),
            camera_y: self.camera_y.as_ref().map(|_| base.camera_y.clone()),
            camera_z: self.camera_z.as_ref().map(|_| base.camera_z.clone()),
            target_x: self.target_x.as_ref().map(|_| base.target_x.clone()),
            target_y: self.target_y.as_ref().map(|_| base.target_y.clone()),
            target_z: self.target_z.as_ref().map(|_| base.target_z.clone()),
            camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom.clone()),
            active_tile_id: self.active_tile_id.as_ref().map(|_| base.active_tile_id.clone()),
            show_masked: self.show_masked.as_ref().map(|_| base.show_masked.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_z.is_none() && self.target_x.is_none() && self.target_y.is_none() && self.target_z.is_none() && self.camera_zoom.is_none() && self.active_tile_id.is_none() && self.show_masked.is_none()
    }
}

impl protocol::MutationDiff<Grid3dWindowConfig> for Grid3dWindowConfigDiff {
    fn apply(&self, base: &Grid3dWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Grid3dWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x.clone_from(value);
        }
        if let Some(value) = &self.camera_y {
            next.camera_y.clone_from(value);
        }
        if let Some(value) = &self.camera_z {
            next.camera_z.clone_from(value);
        }
        if let Some(value) = &self.target_x {
            next.target_x.clone_from(value);
        }
        if let Some(value) = &self.target_y {
            next.target_y.clone_from(value);
        }
        if let Some(value) = &self.target_z {
            next.target_z.clone_from(value);
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom.clone_from(value);
        }
        if let Some(value) = &self.active_tile_id {
            next.active_tile_id.clone_from(value);
        }
        if let Some(value) = &self.show_masked {
            next.show_masked.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.camera_x.is_some() {
            self.camera_x = later.camera_x;
        }
        if later.camera_y.is_some() {
            self.camera_y = later.camera_y;
        }
        if later.camera_z.is_some() {
            self.camera_z = later.camera_z;
        }
        if later.target_x.is_some() {
            self.target_x = later.target_x;
        }
        if later.target_y.is_some() {
            self.target_y = later.target_y;
        }
        if later.target_z.is_some() {
            self.target_z = later.target_z;
        }
        if later.camera_zoom.is_some() {
            self.camera_zoom = later.camera_zoom;
        }
        if later.active_tile_id.is_some() {
            self.active_tile_id = later.active_tile_id;
        }
        if later.show_masked.is_some() {
            self.show_masked = later.show_masked;
        }
    }
}

/// 🪟️ The 3D grid window configuration's mutation vocabulary: one absolute setter per field group.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub enum Grid3dWindowConfigMutation {
    SetActiveTile { tile_id: String },
    SetCamera { camera_x: f64, camera_y: f64, camera_z: f64, target_x: f64, target_y: f64, target_z: f64, zoom: f64 },
    SetShowMasked { show_masked: bool },
}

impl protocol::Mutation<Grid3dWindowConfig> for Grid3dWindowConfigMutation {
    type Diff = Grid3dWindowConfigDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-active-tile",
            display_name: "Set Active Tile",
            emoji: "🪟️",
            aggregate_variant: "SetActiveTile",
            payload_schema: "wfc.grid3dwindowconfig",
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
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "🪟️",
            aggregate_variant: "SetCamera",
            payload_schema: "wfc.grid3dwindowconfig",
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
            owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window",
            semantic_kind: "set-show-masked",
            display_name: "Set Show Masked",
            emoji: "🪟️",
            aggregate_variant: "SetShowMasked",
            payload_schema: "wfc.grid3dwindowconfig",
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
            Self::SetActiveTile { .. } => &Self::DESCRIPTORS[0],
            Self::SetCamera { .. } => &Self::DESCRIPTORS[1],
            Self::SetShowMasked { .. } => &Self::DESCRIPTORS[2],
        }
    }
    fn diff(&self, base: &Grid3dWindowConfig) -> protocol::MutationOutcome<Grid3dWindowConfigDiff> {
        let diff = match self {
            Self::SetActiveTile { tile_id } => Grid3dWindowConfigDiff { active_tile_id: (base.active_tile_id != *tile_id).then(|| tile_id.clone()), ..Default::default() },
            Self::SetCamera { camera_x, camera_y, camera_z, target_x, target_y, target_z, zoom } => Grid3dWindowConfigDiff { camera_x: (base.camera_x != *camera_x).then(|| camera_x.clone()), camera_y: (base.camera_y != *camera_y).then(|| camera_y.clone()), camera_z: (base.camera_z != *camera_z).then(|| camera_z.clone()), target_x: (base.target_x != *target_x).then(|| target_x.clone()), target_y: (base.target_y != *target_y).then(|| target_y.clone()), target_z: (base.target_z != *target_z).then(|| target_z.clone()), camera_zoom: (base.camera_zoom != *zoom).then(|| zoom.clone()), ..Default::default() },
            Self::SetShowMasked { show_masked } => Grid3dWindowConfigDiff { show_masked: (base.show_masked != *show_masked).then(|| show_masked.clone()), ..Default::default() },
        };
        if protocol::DiffAlgebra::<Grid3dWindowConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The window configuration already holds that value.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Grid3dWindowConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetActiveTile { .. } => Self::SetActiveTile { tile_id: base.active_tile_id.clone() },
            Self::SetCamera { .. } => Self::SetCamera { camera_x: base.camera_x.clone(), camera_y: base.camera_y.clone(), camera_z: base.camera_z.clone(), target_x: base.target_x.clone(), target_y: base.target_y.clone(), target_z: base.target_z.clone(), zoom: base.camera_zoom.clone() },
            Self::SetShowMasked { .. } => Self::SetShowMasked { show_masked: base.show_masked.clone() },
        }])
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
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
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

impl store::ConfigRecord for Grid3dWindowConfig {}

impl protocol::OpText for Grid3dWindowConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for Grid3dWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
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
pub fn addressed_config(view: &semio_framework_plugin::ViewModel, mutation: Grid3dWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let (id, kind) = kind(view).ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.grid3d.window.required"), "wfc.grid3d.window.required"))?;
    match kind {
        grid::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid3dGridWindowConfigOwner>(id, mutation)),
        preview::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowConfigMutation::of::<Grid3dPreviewWindowConfigOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc.grid3d.window.kind-required"), "wfc.grid3d.window.kind-required")),
    }
}
//#endregion 🪟️Owners
